//! Host-agnostic C ABI surface shared by the AE and OFX entry shims.
//!
//! The generated host entry (C/C++) calls these `osb_*` symbols; everything that
//! is not host-specific lives in [`HostInstance`], so the shims stay thin and the
//! logic is unit-testable without any host SDK.

use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::assets::TextureStore;
use crate::instance::EffectInstance;
use crate::params::{Control, ParameterDescriptor, ParameterValue};
use crate::render::Concurrency;
use crate::storyboarding::Effect;

/// Control kinds mirrored to the C shims (keep in sync with the entry sources).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiControl {
    Slider = 0,
    Checkbox = 1,
    Color = 2,
    Angle = 3,
    Popup = 4,
    Point = 5,
    Text = 6,
    Picker = 7,
}

/// Parameter metadata copied into the C shim at param setup.
#[repr(C)]
pub struct AbiParamInfo {
    pub kind: u32,
    pub min: f64,
    pub max: f64,
    pub default_number: f64,
    pub default_bool: i32,
}

/// Owns the built effect instance and resolves textures; served over the C ABI.
pub struct HostInstance {
    instance: EffectInstance,
    textures: TextureStore,
}

impl HostInstance {
    pub fn new(effect: Box<dyn Effect + Send + Sync>, width: f32, height: f32) -> Self {
        HostInstance {
            instance: EffectInstance::new(effect, width, height),
            textures: TextureStore::new(),
        }
    }

    pub fn instance_mut(&mut self) -> &mut EffectInstance {
        &mut self.instance
    }

    pub fn textures_mut(&mut self) -> &mut TextureStore {
        &mut self.textures
    }

    pub fn build(&mut self) {
        self.instance.build();
    }

    pub fn concurrency(&self) -> Concurrency {
        self.instance.concurrency()
    }

    pub fn set_concurrency(&mut self, concurrency: Concurrency) {
        self.instance.set_concurrency(concurrency);
    }

    /// Renders into a host buffer. `rowbytes` may be negative (bottom-up, as
    /// Premiere and some OFX hosts supply); the frame is copied with the correct
    /// orientation. When `swap_rb` is set the RGBA frame is written as BGRA.
    ///
    /// # Safety
    ///
    /// `dst` must point to a writable region of at least `stride * height` bytes,
    /// where `stride` is `rowbytes.unsigned_abs()`, and must stay valid for the
    /// duration of the call.
    pub unsafe fn render_into(
        &self,
        dst: *mut u8,
        rowbytes: i32,
        width: u32,
        height: u32,
        time: f64,
        swap_rb: bool,
    ) -> bool {
        if dst.is_null() || width == 0 || height == 0 {
            return false;
        }
        let mut frame = self
            .instance
            .render_frame(&self.textures, width, height, time);
        if swap_rb {
            for pixel in frame.as_chunks_mut::<4>().0 {
                pixel.swap(0, 2);
            }
        }
        let row_len = width as usize * 4;
        let stride = rowbytes.unsigned_abs() as usize;
        for row in 0..height as usize {
            let src = &frame[row * row_len..row * row_len + row_len];
            let target_row = if rowbytes < 0 {
                height as usize - 1 - row
            } else {
                row
            };
            let out = std::slice::from_raw_parts_mut(dst.add(target_row * stride), row_len);
            out.copy_from_slice(src);
        }
        true
    }

    pub fn param_count(&self) -> usize {
        self.instance.parameters().len()
    }

    pub fn param(&self, index: usize) -> Option<ParameterDescriptor> {
        self.instance.parameters().into_iter().nth(index)
    }

    /// Applies a host parameter value (by index) to the effect before build.
    pub fn apply_param(&mut self, index: usize, value: ParameterValue) {
        if let Some(descriptor) = self.param(index) {
            self.instance
                .effect_mut()
                .set_parameter(&descriptor.name, &value);
        }
    }

    /// Applies a resolved asset path (from a host picker) to the effect.
    pub fn set_asset_path(&mut self, index: usize, path: &str) {
        self.apply_param(index, ParameterValue::Text(path.to_string()));
    }
}

/// Sets a numeric parameter by index (called from the host entry at render time).
///
/// # Safety
/// `handle` must be a valid create pointer.
#[no_mangle]
pub unsafe extern "C" fn osb_set_param_number(handle: *mut c_void, index: u32, value: f64) {
    if let Some(host) = (handle as *mut HostInstance).as_mut() {
        host.apply_param(index as usize, ParameterValue::Number(value));
    }
}

/// Sets a boolean parameter by index.
///
/// # Safety
/// `handle` must be a valid create pointer.
#[no_mangle]
pub unsafe extern "C" fn osb_set_param_bool(handle: *mut c_void, index: u32, value: i32) {
    if let Some(host) = (handle as *mut HostInstance).as_mut() {
        host.apply_param(index as usize, ParameterValue::Bool(value != 0));
    }
}

/// Sets a text/asset-path parameter by index.
///
/// # Safety
/// `handle` must be valid; `ptr`/`len` describe a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn osb_set_param_text(
    handle: *mut c_void,
    index: u32,
    ptr: *const u8,
    len: u32,
) {
    let Some(host) = (handle as *mut HostInstance).as_mut() else {
        return;
    };
    if ptr.is_null() {
        return;
    }
    let bytes = std::slice::from_raw_parts(ptr, len as usize);
    let text = String::from_utf8_lossy(bytes).into_owned();
    host.apply_param(index as usize, ParameterValue::Text(text));
}

/// Sets an RGBA color parameter by index.
///
/// # Safety
/// `handle` must be a valid create pointer.
#[no_mangle]
pub unsafe extern "C" fn osb_set_param_color(
    handle: *mut c_void,
    index: u32,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
) {
    if let Some(host) = (handle as *mut HostInstance).as_mut() {
        host.apply_param(
            index as usize,
            ParameterValue::Color(crate::math::Color4::new(r, g, b, a)),
        );
    }
}

/// Maps a [`Control`] to the ABI control kind.
pub fn control_kind(control: &Control) -> AbiControl {
    match control {
        Control::Slider { .. } => AbiControl::Slider,
        Control::Checkbox => AbiControl::Checkbox,
        Control::Color => AbiControl::Color,
        Control::Angle => AbiControl::Angle,
        Control::Popup { .. } => AbiControl::Popup,
        Control::Point => AbiControl::Point,
        Control::Text => AbiControl::Text,
        Control::Picker { .. } => AbiControl::Picker,
    }
}

/// Builds the C-visible [`AbiParamInfo`] for a parameter.
pub fn abi_param_info(parameter: &ParameterDescriptor) -> AbiParamInfo {
    let (min, max) = match &parameter.control {
        Control::Slider { min, max } => (*min, *max),
        Control::Angle => (-360.0, 360.0),
        _ => (0.0, 0.0),
    };
    let (default_number, default_bool) = match parameter.default {
        ParameterValue::Number(value) => (value, 0),
        ParameterValue::Bool(value) => (0.0, value as i32),
        ParameterValue::Choice(value) => (value as f64, 0),
        _ => (0.0, 0),
    };
    AbiParamInfo {
        kind: control_kind(&parameter.control) as u32,
        min,
        max,
        default_number,
        default_bool,
    }
}

/// Copies a Rust string into a caller-provided NUL-terminated buffer.
///
/// # Safety
/// `buf` must be valid for at least `len` bytes.
pub unsafe fn copy_str(value: &str, buf: *mut u8, len: u32) -> i32 {
    if buf.is_null() || len == 0 {
        return 0;
    }
    let bytes = value.as_bytes();
    let count = bytes.len().min((len as usize).saturating_sub(1));
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, count);
    *buf.add(count) = 0;
    1
}

/// # Safety
/// `handle` must be a pointer returned by the generated create function.
pub unsafe fn abi_destroy(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    drop(Box::from_raw(handle as *mut HostInstance));
}

/// # Safety
/// `handle` must be a valid create pointer.
pub unsafe fn abi_build(handle: *mut c_void) {
    if let Some(host) = (handle as *mut HostInstance).as_mut() {
        host.build();
    }
}

/// # Safety
/// `handle` must be valid; `dst` must point to at least `rowbytes.abs() * height` bytes.
pub unsafe fn abi_render(
    handle: *const c_void,
    dst: *mut u8,
    rowbytes: i32,
    width: u32,
    height: u32,
    time: f64,
    swap_rb: bool,
) -> i32 {
    let Some(host) = (handle as *const HostInstance).as_ref() else {
        return 0;
    };
    let ok = catch_unwind(AssertUnwindSafe(|| {
        host.render_into(dst, rowbytes, width, height, time, swap_rb)
    }))
    .unwrap_or(false);
    ok as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::Texture;
    use crate::context::Context;
    use crate::storyboarding::{OsbLayer, OsbOrigin, Storyboard};

    struct Solid;

    impl Effect for Solid {
        fn generate(&mut self, _context: &mut Context) -> Storyboard {
            let mut storyboard = Storyboard::new();
            storyboard
                .layer(OsbLayer::Background)
                .create_sprite("sb/white.png", OsbOrigin::TopLeft)
                .fade(0.0, 1000.0, 1.0, 1.0, crate::animations::Easing::None);
            storyboard
        }
    }

    #[test]
    fn renders_bottom_up_and_swaps_rb() {
        let mut host = HostInstance::new(Box::new(Solid), 2.0, 2.0);
        host.textures_mut()
            .insert("sb/white.png", Texture::new(1, 1, vec![10, 20, 200, 255]));
        host.build();
        let mut buffer = vec![0u8; 2 * 2 * 4];
        // SAFETY: `buffer` is a live allocation of exactly `stride * height` bytes.
        assert!(unsafe { host.render_into(buffer.as_mut_ptr(), -8, 2, 2, 0.0, true) });
        // BGRA, bottom-up: row 1 holds the top-left red-ish pixel with R/B swapped.
        assert_eq!(&buffer[8..12], &[200, 20, 10, 255]);
    }
}
