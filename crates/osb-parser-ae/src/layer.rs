//! Composition layer parsing.

use crate::error::{AeError, AeResult};
use crate::property::{indexed_group_to_map, parse_property, PropInput};
use crate::rifx::List;

/// A layer's render quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerQuality {
    /// Best quality.
    Best,
    /// Draft quality.
    Draft,
    /// Wireframe quality.
    Wireframe,
    /// Unknown quality.
    #[default]
    Unknown,
}

impl LayerQuality {
    fn from_u16(value: u16) -> Self {
        match value {
            0x0002 => Self::Best,
            0x0001 => Self::Draft,
            0x0000 => Self::Wireframe,
            _ => Self::Unknown,
        }
    }
}

/// A layer's sampling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerSamplingMode {
    /// Bilinear sampling.
    #[default]
    Bilinear,
    /// Bicubic sampling.
    Bicubic,
}

impl LayerSamplingMode {
    fn from_bit(bit: u8) -> Self {
        if bit == 1 {
            Self::Bicubic
        } else {
            Self::Bilinear
        }
    }
}

/// A layer's frame-blending mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerFrameBlendMode {
    /// Frame mix.
    #[default]
    FrameMix,
    /// Pixel motion.
    PixelMotion,
}

impl LayerFrameBlendMode {
    fn from_bit(bit: u8) -> Self {
        if bit == 1 {
            Self::PixelMotion
        } else {
            Self::FrameMix
        }
    }
}

/// A single layer in a composition.
#[derive(Debug, Clone, Default)]
pub struct Layer {
    /// The layer's 1-based index in the composition.
    pub index: u32,
    /// The layer name.
    pub name: String,
    /// The id of the source item (footage/composition) the layer draws.
    pub source_id: u32,
    /// Composition start time in seconds (`Layer.startTime`).
    pub start_time: f64,
    /// In point in layer time, seconds (measured from `start_time`, unstretched).
    pub in_point: f64,
    /// Out point in layer time, seconds.
    pub out_point: f64,
    /// Stretch in percent (negative = time-reversed).
    pub stretch: f64,
    /// The layer id (`Layer.id`); referenced by parents and mattes.
    pub id: u32,
    /// Parent layer id, `0` = none.
    pub parent_id: u32,
    /// Label colour (0-16).
    pub label: u8,
    /// Blending mode code.
    pub blending_mode: u8,
    /// Track matte type: 0 none, 1 Alpha, 2 Alpha Inverted, 3 Luma, 4 Luma Inverted.
    pub track_matte_type: u8,
    /// Layer type code: 0 AV, 1 light, 2 camera, 3 text, 4 shape, 5 3D model, 7 mesh.
    pub layer_type: u8,
    /// Render quality.
    pub quality: LayerQuality,
    /// Sampling mode.
    pub sampling_mode: LayerSamplingMode,
    /// Frame-blend mode.
    pub frame_blend_mode: LayerFrameBlendMode,
    /// Guide layer enabled.
    pub guide_enabled: bool,
    /// Solo enabled.
    pub solo_enabled: bool,
    /// 3D layer enabled.
    pub three_d_enabled: bool,
    /// Adjustment layer enabled.
    pub adjustment_layer_enabled: bool,
    /// Collapse transformations enabled.
    pub collapse_transform_enabled: bool,
    /// Shy enabled.
    pub shy_enabled: bool,
    /// Locked.
    pub lock_enabled: bool,
    /// Frame blending enabled.
    pub frame_blend_enabled: bool,
    /// Motion blur enabled.
    pub motion_blur_enabled: bool,
    /// Effects enabled.
    pub effects_enabled: bool,
    /// Audio enabled.
    pub audio_enabled: bool,
    /// Video enabled.
    pub video_enabled: bool,
    /// The effect stack (`ADBE Effect Parade` children).
    pub effects: Vec<crate::property::Property>,
    /// The text properties (`ADBE Text Properties`), for text layers.
    pub text: Option<crate::property::Property>,
    /// Every top-level property group (transform, masks, effects, …).
    pub properties: Vec<crate::property::Property>,
}

fn bit(value: u8, index: u8) -> bool {
    (value >> index) & 1 == 1
}

/// A `s4` dividend / `u4` divisor pair at `offset`, in seconds.
fn ratio(ldta: &crate::rifx::Block, offset: usize) -> f64 {
    let dividend = ldta.be_u32(offset).map(|value| value as i32).unwrap_or(0);
    let divisor = ldta.be_u32(offset + 4).unwrap_or(0);
    if divisor == 0 {
        0.0
    } else {
        dividend as f64 / divisor as f64
    }
}

impl Layer {
    /// The composition in point in seconds: `start_time + in_point * stretch / 100`.
    pub fn composition_in_point(&self) -> f64 {
        self.start_time + self.in_point * self.stretch / 100.0
    }

    /// The composition out point in seconds.
    pub fn composition_out_point(&self) -> f64 {
        self.start_time + self.out_point * self.stretch / 100.0
    }
}

pub(crate) fn parse_layer(list: &List) -> AeResult<Layer> {
    let ldta = list
        .find_type("ldta")
        .ok_or_else(|| AeError::Malformed("layer has no ldta block".into()))?;

    let stretch_dividend = ldta.be_u32(0x08).map(|value| value as i32).unwrap_or(0) as f64;
    let stretch_divisor = ldta.be_u32(0x6C).unwrap_or(0) as f64;

    let mut layer = Layer {
        source_id: ldta.be_u32(0x28).unwrap_or(0),
        quality: LayerQuality::from_u16(ldta.be_u16(0x04).unwrap_or(0)),
        start_time: ratio(ldta, 0x0C),
        in_point: ratio(ldta, 0x14),
        out_point: ratio(ldta, 0x1C),
        stretch: if stretch_divisor == 0.0 {
            100.0
        } else {
            100.0 * stretch_dividend / stretch_divisor
        },
        id: ldta.be_u32(0x00).unwrap_or(0),
        parent_id: ldta.be_u32(0x84).unwrap_or(0),
        label: ldta.byte(0x3D).unwrap_or(0),
        blending_mode: ldta.byte(0x63).unwrap_or(0),
        track_matte_type: ldta.byte(0x6B).unwrap_or(0),
        layer_type: ldta.byte(0x83).unwrap_or(0),
        ..Layer::default()
    };

    // Layer attribute bits live at offsets 37..40.
    let attr0 = ldta.byte(37).unwrap_or(0);
    let attr1 = ldta.byte(38).unwrap_or(0);
    let attr2 = ldta.byte(39).unwrap_or(0);
    layer.sampling_mode = LayerSamplingMode::from_bit((attr0 >> 6) & 1);
    layer.frame_blend_mode = LayerFrameBlendMode::from_bit((attr0 >> 2) & 1);
    layer.guide_enabled = bit(attr0, 1);
    layer.solo_enabled = bit(attr1, 3);
    layer.three_d_enabled = bit(attr1, 2);
    layer.adjustment_layer_enabled = bit(attr1, 1);
    layer.collapse_transform_enabled = bit(attr2, 7);
    layer.shy_enabled = bit(attr2, 6);
    layer.lock_enabled = bit(attr2, 5);
    layer.frame_blend_enabled = bit(attr2, 4);
    layer.motion_blur_enabled = bit(attr2, 3);
    layer.effects_enabled = bit(attr2, 2);
    layer.audio_enabled = bit(attr2, 1);
    layer.video_enabled = bit(attr2, 0);

    if let Some(name) = list.find_type("Utf8") {
        layer.name = name.text();
    }

    let merged = list.sublist_merge("tdgp");
    let (names, map) = indexed_group_to_map(&merged);
    for (index, name) in names.iter().enumerate() {
        if let Some(group) = map.get(name) {
            if let Ok(mut prop) = parse_property(PropInput::List(group), name) {
                prop.index = (index + 1) as u32;
                layer.properties.push(prop);
            }
        }
    }
    if let Some(effects) = map.get("ADBE Effect Parade") {
        if let Ok(prop) = parse_property(PropInput::List(effects), "ADBE Effect Parade") {
            layer.effects = prop.properties;
        }
    }
    if let Some(text) = map.get("ADBE Text Properties") {
        if let Ok(prop) = parse_property(PropInput::List(text), "ADBE Text Properties") {
            layer.text = Some(prop);
        }
    }

    Ok(layer)
}
