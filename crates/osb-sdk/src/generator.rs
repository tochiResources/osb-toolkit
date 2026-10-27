//! The effect-script generator facade (M31).
//!
//! A [`Generator`] exposes the
//! API effect scripts use (`GetLayer`, `CreateSprite`, `CreateAnimation`,
//! `CreateSample`, `GetBeatmap`, `GetFft`, timing/control points, `Random`) over
//! the [`Runtime`], and [`Generate`] is the effect entry point.

use crate::error::Error;
use crate::models::{
    Beatmap, ControlPoint, FftResult, Handle, LayerHandle, OsbLoopType, OsbOrigin, Vector2,
};
use crate::runtime::Runtime;

/// An effect-script storyboard generator.
pub struct Generator {
    runtime: Runtime,
    width: f64,
    height: f64,
}

impl Generator {
    /// Creates a generator for a storyboard of the given resolution.
    pub fn new(width: f64, height: f64) -> Self {
        Generator {
            runtime: Runtime::new(width as f32, height as f32),
            width,
            height,
        }
    }

    /// The storyboard width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// The storyboard height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// `GetLayer`: a layer handle by name (created on first use).
    pub fn get_layer(&mut self, name: &str) -> Result<LayerHandle, Error> {
        self.runtime.layer_handle(name)
    }

    /// `CreateSprite`.
    pub fn create_sprite(
        &mut self,
        layer: &str,
        path: &str,
        origin: OsbOrigin,
    ) -> Result<Handle, Error> {
        self.runtime.create_sprite(layer, path, origin)
    }

    /// `CreateSprite` with an initial position.
    pub fn create_sprite_at(
        &mut self,
        layer: &str,
        path: &str,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.runtime.create_sprite_at(layer, path, origin, position)
    }

    /// `CreateAnimation`.
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation(
        &mut self,
        layer: &str,
        path: &str,
        frames: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.runtime.create_animation(
            layer,
            path,
            frames,
            frame_delay,
            loop_type,
            origin,
            position,
        )
    }

    /// `CreateSample`.
    pub fn create_sample(
        &mut self,
        layer: &str,
        path: &str,
        time: f64,
        volume: f64,
    ) -> Result<(), Error> {
        self.runtime.create_sample(layer, path, time, volume)?;
        Ok(())
    }

    /// `GetBeatmap`.
    pub fn get_beatmap(&mut self) -> Result<Option<Beatmap>, Error> {
        self.runtime.beatmap()
    }

    /// `GetFft`.
    pub fn get_fft(&mut self, time: f64, magnitudes: usize) -> Result<FftResult, Error> {
        self.runtime.get_fft(time, magnitudes)
    }

    /// `GetTimingPointAt`.
    pub fn get_timing_point_at(&mut self, time: f64) -> Result<ControlPoint, Error> {
        self.runtime.timing_point_at(time)
    }

    /// `GetControlPointAt`.
    pub fn get_control_point_at(&mut self, time: f64) -> Result<Option<ControlPoint>, Error> {
        self.runtime.control_point_at(time)
    }

    /// `Random`.
    pub fn random(&mut self, min: f64, max: f64) -> Result<f64, Error> {
        self.runtime.random(min, max)
    }

    /// The number of sprites on the storyboard.
    pub fn sprite_count(&mut self) -> Result<i32, Error> {
        self.runtime.sprite_count()
    }

    /// The built storyboard model (`storyboard`), so `generate()`'s result can
    /// be read back like the runtime `Effect::generate` return value.
    pub fn storyboard(&mut self) -> Result<crate::models::Storyboard, Error> {
        self.runtime.storyboard()
    }

    /// Imports a `.osb` storyboard (`ImportOsb`).
    pub fn import_osb(&mut self, text: &str) -> Result<(), Error> {
        self.runtime.import_osb(text)?;
        Ok(())
    }

    /// `PositionAt`.
    pub fn position_at(&mut self, handle: &Handle, time: f64) -> Result<Vector2, Error> {
        Ok(*self.runtime.sprite_state_at(handle, time)?.position)
    }

    /// `ScaleAt`.
    pub fn scale_at(&mut self, handle: &Handle, time: f64) -> Result<Vector2, Error> {
        Ok(*self.runtime.sprite_state_at(handle, time)?.scale)
    }

    /// `RotationAt`.
    pub fn rotation_at(&mut self, handle: &Handle, time: f64) -> Result<f64, Error> {
        Ok(self.runtime.sprite_state_at(handle, time)?.rotation)
    }

    /// `OpacityAt`.
    pub fn opacity_at(&mut self, handle: &Handle, time: f64) -> Result<f64, Error> {
        Ok(self.runtime.sprite_state_at(handle, time)?.opacity)
    }

    /// `AnchorAt`: the sprite's sampled origin offset.
    pub fn anchor_at(&mut self, handle: &Handle, time: f64) -> Result<Vector2, Error> {
        self.runtime.anchor_at(handle, time)
    }

    /// The underlying [`Runtime`], for command authoring.
    pub fn runtime(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// `OsbSprite.GetOriginVector`.
    pub fn get_origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Result<Vector2, Error> {
        crate::Client::get_origin_vector(origin, width, height)
    }

    /// `OsbSprite.InScreenBounds`.
    pub fn in_screen_bounds(
        position: Vector2,
        size: Vector2,
        rotation: f64,
        origin: Vector2,
    ) -> Result<bool, Error> {
        crate::Client::in_screen_bounds(position, size, rotation, origin)
    }
}

/// An effect: implement [`Generate`] and run it with [`run`].
pub trait Generate {
    /// Builds the storyboard (`Generate`).
    fn generate(&mut self, generator: &mut Generator) -> Result<(), Error>;
}

/// Runs an effect's [`Generate`] against a fresh generator and returns it.
pub fn run<E: Generate>(effect: &mut E, width: f64, height: f64) -> Result<Generator, Error> {
    let mut generator = Generator::new(width, height);
    effect.generate(&mut generator)?;
    Ok(generator)
}

#[cfg(test)]
mod tests {
    use crate::models::Easing;

    use super::*;

    struct Pulse;

    impl Generate for Pulse {
        fn generate(&mut self, generator: &mut Generator) -> Result<(), Error> {
            let handle = generator.create_sprite("Foreground", "sb/dot.png", OsbOrigin::Centre)?;
            generator
                .runtime()
                .fade(&handle, 0.0, 100.0, 0.0, 1.0, Easing::None)?;
            Ok(())
        }
    }

    #[test]
    fn runs_a_generator_backed_effect() {
        let mut generator = run(&mut Pulse, 640.0, 480.0).unwrap();
        assert_eq!(generator.width(), 640.0);
        assert_eq!(generator.sprite_count().unwrap(), 1);
    }

    #[test]
    fn reads_back_the_built_storyboard() {
        let mut generator = run(&mut Pulse, 640.0, 480.0).unwrap();
        let storyboard = generator.storyboard().unwrap();
        let foreground = storyboard
            .layers
            .iter()
            .find(|layer| layer.identifier == "Foreground")
            .expect("foreground layer");
        assert_eq!(foreground.sprites[0].texture_path, "sb/dot.png");
        assert!(!foreground.sprites[0].commands.is_empty());
    }

    #[test]
    fn animates_and_samples_the_anchor() {
        let mut generator = Generator::new(640.0, 480.0);
        let handle = generator
            .create_sprite("Foreground", "sb/dot.png", OsbOrigin::Centre)
            .unwrap();
        generator
            .runtime()
            .anchor(
                &handle,
                0.0,
                100.0,
                Vector2::new(0.0, 0.0),
                Vector2::new(10.0, 20.0),
                Easing::None,
            )
            .unwrap();
        let mid = generator.anchor_at(&handle, 50.0).unwrap();
        assert!((mid.x - 5.0).abs() < 1e-6);
        assert!((mid.y - 10.0).abs() < 1e-6);
    }
}
