use serde_json::{json, Value};

use super::{call, Channel};
use crate::error::Error;
use crate::models::Color4;

/// A gradient stop: an offset in `0..=1` and its color.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColorStop {
    /// The stop position in `0..=1`.
    pub offset: f64,
    /// The color at the stop.
    pub color: Color4,
}

fn to_color(value: Value) -> Result<Color4, Error> {
    Ok(serde_json::from_value(value)?)
}

fn run_color(name: &str, input: Value) -> Result<Color4, Error> {
    to_color(call(name, input)?)
}

/// Converts a color into its `(x, y, z, w)` components in `space`.
pub fn to_space(color: Color4, space: &str) -> Result<Vec<f64>, Error> {
    Ok(serde_json::from_value(call(
        "color.toSpace",
        json!({ "color": color, "space": space }),
    )?)?)
}

/// Builds a color from `(x, y, z, w)` components in `space`.
pub fn from_space(values: [f64; 4], space: &str) -> Result<Color4, Error> {
    run_color(
        "color.fromSpace",
        json!({ "value": values, "space": space }),
    )
}

/// Adds a per-component `delta` in `space`.
pub fn offset(color: Color4, space: &str, delta: [f64; 4]) -> Result<Color4, Error> {
    run_color(
        "color.offset",
        json!({ "color": color, "space": space, "delta": delta }),
    )
}

/// Multiplies each component by a per-component `factor` in `space`.
pub fn scale(color: Color4, space: &str, factor: [f64; 4]) -> Result<Color4, Error> {
    run_color(
        "color.scale",
        json!({ "color": color, "space": space, "factor": factor }),
    )
}

/// Jitters each component by a seeded per-component range in `space`.
pub fn randomize(
    color: Color4,
    space: &str,
    seed: u64,
    min: [f64; 4],
    max: [f64; 4],
) -> Result<Color4, Error> {
    run_color(
        "color.randomize",
        json!({ "color": color, "space": space, "seed": seed, "min": min, "max": max }),
    )
}

/// Applies a named CSS/Sass helper (`"lighten"`, `"mix"`, ...) with `params`.
pub fn apply(color: Color4, op: &str, params: Value) -> Result<Color4, Error> {
    let mut input = if params.is_object() {
        params
    } else {
        json!({})
    };
    let object = input.as_object_mut().expect("object");
    object.insert("color".to_string(), serde_json::to_value(color)?);
    object.insert("op".to_string(), json!(op));
    run_color("color.apply", input)
}

/// `lighten($color, $amount)`.
pub fn lighten(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "lighten", json!({ "amount": amount }))
}

/// `darken($color, $amount)`.
pub fn darken(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "darken", json!({ "amount": amount }))
}

/// `saturate($color, $amount)`.
pub fn saturate(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "saturate", json!({ "amount": amount }))
}

/// `desaturate($color, $amount)`.
pub fn desaturate(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "desaturate", json!({ "amount": amount }))
}

/// `adjust-hue($color, $degrees)`.
pub fn adjust_hue(color: Color4, degrees: f64) -> Result<Color4, Error> {
    apply(color, "adjustHue", json!({ "degrees": degrees }))
}

/// `complement($color)`.
pub fn complement(color: Color4) -> Result<Color4, Error> {
    apply(color, "complement", json!({}))
}

/// `grayscale($color)`.
pub fn grayscale(color: Color4) -> Result<Color4, Error> {
    apply(color, "grayscale", json!({}))
}

/// `invert($color)`.
pub fn invert(color: Color4) -> Result<Color4, Error> {
    apply(color, "invert", json!({}))
}

/// `mix($a, $b, $weight)`.
pub fn mix(a: Color4, b: Color4, weight: f64) -> Result<Color4, Error> {
    apply(a, "mix", json!({ "other": b, "weight": weight }))
}

/// `tint($color, $weight)`.
pub fn tint(color: Color4, weight: f64) -> Result<Color4, Error> {
    apply(color, "tint", json!({ "weight": weight }))
}

/// `shade($color, $weight)`.
pub fn shade(color: Color4, weight: f64) -> Result<Color4, Error> {
    apply(color, "shade", json!({ "weight": weight }))
}

/// `opacify($color, $amount)`.
pub fn opacify(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "opacify", json!({ "amount": amount }))
}

/// `transparentize($color, $amount)`.
pub fn transparentize(color: Color4, amount: f64) -> Result<Color4, Error> {
    apply(color, "transparentize", json!({ "amount": amount }))
}

/// Sets the alpha channel.
pub fn with_alpha(color: Color4, alpha: f64) -> Result<Color4, Error> {
    apply(color, "withAlpha", json!({ "alpha": alpha }))
}

/// Samples a multi-stop gradient at `t` (`0..=1`).
pub fn gradient(stops: Vec<ColorStop>, t: f64, space: &str, repeat: bool) -> Result<Color4, Error> {
    run_color(
        "color.gradient",
        json!({ "stops": stops, "t": t, "space": space, "repeat": repeat }),
    )
}

/// Builds a color channel by sampling a gradient `count` times over
/// `[start, end]`.
pub fn gradient_channel(
    stops: Vec<ColorStop>,
    space: &str,
    count: usize,
    start: f64,
    end: f64,
    repeat: bool,
) -> Result<Channel, Error> {
    Ok(serde_json::from_value(call(
        "color.gradientChannel",
        json!({ "stops": stops, "space": space, "count": count,
                    "start": start, "end": end, "repeat": repeat }),
    )?)?)
}

/// Offsets a whole color channel in `space`.
pub fn offset_channel(channel: Channel, space: &str, delta: [f64; 4]) -> Result<Channel, Error> {
    Ok(serde_json::from_value(call(
        "color.offsetChannel",
        json!({ "channel": channel, "space": space, "delta": delta }),
    )?)?)
}

/// Scales a whole color channel in `space`.
pub fn scale_channel(channel: Channel, space: &str, factor: [f64; 4]) -> Result<Channel, Error> {
    Ok(serde_json::from_value(call(
        "color.scaleChannel",
        json!({ "channel": channel, "space": space, "factor": factor }),
    )?)?)
}

/// Randomizes a whole color channel in `space`.
pub fn randomize_channel(
    channel: Channel,
    space: &str,
    seed: u64,
    min: [f64; 4],
    max: [f64; 4],
) -> Result<Channel, Error> {
    Ok(serde_json::from_value(call(
        "color.randomizeChannel",
        json!({ "channel": channel, "space": space, "seed": seed, "min": min, "max": max }),
    )?)?)
}

/// Applies a named helper to every keyframe of a color channel.
pub fn apply_channel(channel: Channel, op: &str, params: Value) -> Result<Channel, Error> {
    let mut input = if params.is_object() {
        params
    } else {
        json!({})
    };
    let object = input.as_object_mut().expect("object");
    object.insert("channel".to_string(), serde_json::to_value(channel)?);
    object.insert("op".to_string(), json!(op));
    Ok(serde_json::from_value(call("color.applyChannel", input)?)?)
}
