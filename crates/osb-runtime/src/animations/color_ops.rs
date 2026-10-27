//! Color-space manipulation, CSS/Sass-style helpers and gradients (M40).
//!
//! A canonical set of pure transforms over [`Color4`] values and color keyframe
//! [`Channel`]s. Because a command is a keyframe channel at heart, these helpers
//! work on the same `Channel` the other motion helpers use: a color command
//! channel (`Color`/`ColorHsb`) round-trips through them exactly like a 2D or 3D
//! channel. They are shared by the wire bridge and every SDK.
//!
//! The helpers cover the CSS/Sass surface - `lighten`/`darken`, `saturate`/
//! `desaturate`, `adjustHue`/`complement`, `grayscale`/`invert`, `mix`/`tint`/
//! `shade`, `opacify`/`transparentize`, `scale`/`offset`/`randomize` in any space
//! (RGB, HSL, HSB/HSV, HWB) - plus multi-stop **linear gradients**.

use crate::animations::keyframes::Channel;
use crate::animations::KeyframedValue;
use crate::context::Rng;
use crate::math::{Color4, Vec4};

/// A color space a color helper operates in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// Red/green/blue/alpha, each `0..=1`.
    Rgb,
    /// Hue/saturation/lightness/alpha (hue in turns `0..=1`).
    Hsl,
    /// Hue/saturation/brightness/alpha (HSV is the same as HSB).
    Hsb,
    /// Hue/whiteness/blackness/alpha (the CSS `hwb()` space).
    Hwb,
}

impl ColorSpace {
    /// Parses `"rgb"`/`"hsl"`/`"hsb"`/`"hsv"`/`"hwb"` (case-insensitive).
    pub fn parse(name: &str) -> Option<ColorSpace> {
        match name.to_ascii_lowercase().as_str() {
            "rgb" | "rgba" => Some(ColorSpace::Rgb),
            "hsl" | "hsla" => Some(ColorSpace::Hsl),
            "hsb" | "hsv" | "hsba" | "hsva" => Some(ColorSpace::Hsb),
            "hwb" => Some(ColorSpace::Hwb),
            _ => None,
        }
    }

    /// The canonical name.
    pub fn name(self) -> &'static str {
        match self {
            ColorSpace::Rgb => "rgb",
            ColorSpace::Hsl => "hsl",
            ColorSpace::Hsb => "hsb",
            ColorSpace::Hwb => "hwb",
        }
    }

    /// Converts a color into `(x, y, z, w)` in this space.
    pub fn to_space(self, color: Color4) -> Vec4 {
        match self {
            ColorSpace::Rgb => Vec4::new(color.r, color.g, color.b, color.a),
            ColorSpace::Hsl => color.to_hsl(),
            ColorSpace::Hsb => color.to_hsb(),
            ColorSpace::Hwb => to_hwb(color),
        }
    }

    /// Converts `(x, y, z, w)` in this space back to a color.
    pub fn from_space(self, value: Vec4) -> Color4 {
        match self {
            ColorSpace::Rgb => Color4::new(value.x, value.y, value.z, value.w),
            ColorSpace::Hsl => Color4::from_hsl(value),
            ColorSpace::Hsb => Color4::from_hsb(value.x, value.y, value.z, value.w),
            ColorSpace::Hwb => from_hwb(value),
        }
    }

    /// Wraps the hue and clamps the other components into their natural range.
    fn normalize(self, value: &mut Vec4) {
        value.x = value.x.fract();
        if value.x < 0.0 {
            value.x += 1.0;
        }
        value.y = value.y.clamp(0.0, 1.0);
        value.z = value.z.clamp(0.0, 1.0);
        value.w = value.w.clamp(0.0, 1.0);
    }
}

/// Converts a color to the CSS `hwb()` space.
pub fn to_hwb(color: Color4) -> Vec4 {
    let hsb = color.to_hsb();
    let whiteness = color.r.min(color.g).min(color.b);
    let blackness = 1.0 - color.r.max(color.g).max(color.b);
    Vec4::new(hsb.x, whiteness, blackness, color.a)
}

/// Builds a color from the CSS `hwb()` space.
pub fn from_hwb(value: Vec4) -> Color4 {
    let hue = value.x.fract();
    let whiteness = value.y.max(0.0);
    let blackness = value.z.max(0.0);
    if whiteness + blackness >= 1.0 {
        let gray = whiteness / (whiteness + blackness);
        return Color4::new(gray, gray, gray, value.w);
    }
    let pure = Color4::from_hsb(hue, 1.0, 1.0, value.w);
    let scale = 1.0 - whiteness - blackness;
    Color4::new(
        pure.r * scale + whiteness,
        pure.g * scale + whiteness,
        pure.b * scale + whiteness,
        value.w,
    )
}

// -- component ops in a chosen space ----------------------------------------

/// Applies a per-component transform in `space`, then normalizes the result.
fn transform_in_space(color: Color4, space: ColorSpace, f: impl Fn(f64, usize) -> f64) -> Color4 {
    let v = space.to_space(color);
    let components = [v.x, v.y, v.z, v.w];
    let mut out = Vec4::new(
        f(components[0], 0),
        f(components[1], 1),
        f(components[2], 2),
        f(components[3], 3),
    );
    space.normalize(&mut out);
    space.from_space(out)
}

/// Adds a per-component `delta` in `space`.
pub fn offset_color(color: Color4, space: ColorSpace, delta: Vec4) -> Color4 {
    let deltas = [delta.x, delta.y, delta.z, delta.w];
    transform_in_space(color, space, |value, index| value + deltas[index])
}

/// Multiplies each component by a per-component `factor` in `space`.
pub fn scale_color(color: Color4, space: ColorSpace, factor: Vec4) -> Color4 {
    let factors = [factor.x, factor.y, factor.z, factor.w];
    transform_in_space(color, space, |value, index| value * factors[index])
}

/// Jitters each component by a seeded per-component range in `space`.
pub fn randomize_color(
    color: Color4,
    space: ColorSpace,
    rng: &mut Rng,
    min: Vec4,
    max: Vec4,
) -> Color4 {
    let mins = [min.x, min.y, min.z, min.w];
    let maxs = [max.x, max.y, max.z, max.w];
    let deltas = [
        rng.range(mins[0], maxs[0]),
        rng.range(mins[1], maxs[1]),
        rng.range(mins[2], maxs[2]),
        rng.range(mins[3], maxs[3]),
    ];
    transform_in_space(color, space, |value, index| value + deltas[index])
}

/// Replaces a single component (`0 = r/hue`, `1 = g/sat`, `2 = b/light`, `3 = a`)
/// in `space`.
pub fn set_component(color: Color4, space: ColorSpace, index: usize, value: f64) -> Color4 {
    let mut v = space.to_space(color);
    match index {
        0 => v.x = value,
        1 => v.y = value,
        2 => v.z = value,
        _ => v.w = value,
    }
    space.normalize(&mut v);
    space.from_space(v)
}

// -- CSS / Sass helpers ------------------------------------------------------

/// `lighten($color, $amount)`: raises HSL lightness.
pub fn lighten(color: Color4, amount: f64) -> Color4 {
    let hsl = color.to_hsl();
    Color4::from_hsl(Vec4::new(
        hsl.x,
        hsl.y,
        (hsl.z + amount).clamp(0.0, 1.0),
        hsl.w,
    ))
}

/// `darken($color, $amount)`: lowers HSL lightness.
pub fn darken(color: Color4, amount: f64) -> Color4 {
    lighten(color, -amount)
}

/// `saturate($color, $amount)`: raises HSL saturation.
pub fn saturate(color: Color4, amount: f64) -> Color4 {
    let hsl = color.to_hsl();
    Color4::from_hsl(Vec4::new(
        hsl.x,
        (hsl.y + amount).clamp(0.0, 1.0),
        hsl.z,
        hsl.w,
    ))
}

/// `desaturate($color, $amount)`: lowers HSL saturation.
pub fn desaturate(color: Color4, amount: f64) -> Color4 {
    saturate(color, -amount)
}

/// `adjust-hue($color, $degrees)`: rotates the hue by `degrees`.
pub fn adjust_hue(color: Color4, degrees: f64) -> Color4 {
    let hsl = color.to_hsl();
    let hue = (hsl.x + degrees / 360.0).rem_euclid(1.0);
    Color4::from_hsl(Vec4::new(hue, hsl.y, hsl.z, hsl.w))
}

/// `complement($color)`: the 180-degree hue rotation.
pub fn complement(color: Color4) -> Color4 {
    adjust_hue(color, 180.0)
}

/// `grayscale($color)`: drops HSL saturation to zero.
pub fn grayscale(color: Color4) -> Color4 {
    let hsl = color.to_hsl();
    Color4::from_hsl(Vec4::new(hsl.x, 0.0, hsl.z, hsl.w))
}

/// `invert($color)`: the RGB inverse, keeping alpha.
pub fn invert(color: Color4) -> Color4 {
    Color4::new(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, color.a)
}

/// `mix($a, $b, $weight)`: the Sass weighted mix, accounting for alpha.
/// `weight` is the amount of `a` (`0..=1`, default `0.5`).
pub fn mix(a: Color4, b: Color4, weight: f64) -> Color4 {
    let p = weight.clamp(0.0, 1.0);
    let w = p * 2.0 - 1.0;
    let alpha = a.a - b.a;
    let product = w * alpha;
    let w1 = if (product + 1.0).abs() < 1e-9 {
        w
    } else {
        (w + alpha) / (1.0 + product)
    };
    let w1 = (w1 + 1.0) * 0.5;
    let w2 = 1.0 - w1;
    Color4::new(
        a.r * w1 + b.r * w2,
        a.g * w1 + b.g * w2,
        a.b * w1 + b.b * w2,
        a.a * p + b.a * (1.0 - p),
    )
}

/// `tint($color, $weight)`: mixes toward white.
pub fn tint(color: Color4, weight: f64) -> Color4 {
    mix(Color4::WHITE, color, weight)
}

/// `shade($color, $weight)`: mixes toward black.
pub fn shade(color: Color4, weight: f64) -> Color4 {
    mix(Color4::BLACK, color, weight)
}

/// `opacify($color, $amount)` / `fade-in`: raises alpha.
pub fn opacify(color: Color4, amount: f64) -> Color4 {
    Color4::new(
        color.r,
        color.g,
        color.b,
        (color.a + amount).clamp(0.0, 1.0),
    )
}

/// `transparentize($color, $amount)` / `fade-out`: lowers alpha.
pub fn transparentize(color: Color4, amount: f64) -> Color4 {
    opacify(color, -amount)
}

/// Sets the alpha channel.
pub fn with_alpha(color: Color4, alpha: f64) -> Color4 {
    Color4::new(color.r, color.g, color.b, alpha.clamp(0.0, 1.0))
}

// -- gradients ---------------------------------------------------------------

/// One stop of a gradient: an offset in `0..=1` and its color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    /// The stop position in `0..=1`.
    pub offset: f64,
    /// The color at the stop.
    pub color: Color4,
}

impl ColorStop {
    /// Builds a stop.
    pub fn new(offset: f64, color: Color4) -> Self {
        ColorStop { offset, color }
    }
}

/// Samples a multi-stop gradient at `t` (`0..=1`), interpolating between the
/// surrounding stops in `space`. With `repeat`, `t` wraps; otherwise it clamps.
/// A single stop (or none) returns that stop (or white).
pub fn sample_gradient(stops: &[ColorStop], t: f64, space: ColorSpace, repeat: bool) -> Color4 {
    if stops.is_empty() {
        return Color4::WHITE;
    }
    let mut stops = stops.to_vec();
    stops.sort_by(|a, b| a.offset.total_cmp(&b.offset));
    if stops.len() == 1 {
        return stops[0].color;
    }
    let t = if repeat {
        t.rem_euclid(1.0)
    } else {
        t.clamp(0.0, 1.0)
    };
    if t <= stops[0].offset {
        return stops[0].color;
    }
    if t >= stops[stops.len() - 1].offset {
        return stops[stops.len() - 1].color;
    }
    for pair in stops.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if t >= a.offset && t <= b.offset {
            let span = b.offset - a.offset;
            let local = if span <= 0.0 {
                0.0
            } else {
                (t - a.offset) / span
            };
            return interpolate_color(a.color, b.color, local, space);
        }
    }
    stops[stops.len() - 1].color
}

/// Interpolates two colors in `space` by `amount` (`0..=1`).
pub fn interpolate_color(a: Color4, b: Color4, amount: f64, space: ColorSpace) -> Color4 {
    let va = space.to_space(a);
    let vb = space.to_space(b);
    let mut value = Vec4::new(
        va.x + (vb.x - va.x) * amount,
        va.y + (vb.y - va.y) * amount,
        va.z + (vb.z - va.z) * amount,
        va.w + (vb.w - va.w) * amount,
    );
    space.normalize(&mut value);
    space.from_space(value)
}

/// Builds a color channel by sampling a gradient `count` times over
/// `[start, end]` (like [`crate::animations::keyframes::pulse`] for colors).
#[allow(clippy::too_many_arguments)]
pub fn gradient_channel(
    stops: &[ColorStop],
    space: ColorSpace,
    count: usize,
    start: f64,
    end: f64,
    repeat: bool,
) -> Channel {
    let mut out = KeyframedValue::new();
    let count = count.max(2);
    let span = end - start;
    if span <= 0.0 {
        out.add(start, sample_gradient(stops, 0.0, space, repeat));
        return Channel::Color(out);
    }
    for i in 0..count {
        let u = i as f64 / (count - 1) as f64;
        out.add(start + span * u, sample_gradient(stops, u, space, repeat));
    }
    Channel::Color(out)
}

// -- channel-level wrappers --------------------------------------------------

/// Maps every keyframe of a color channel through `f` (non-colors pass through).
pub fn map_color(channel: &Channel, f: impl Fn(Color4) -> Color4) -> Channel {
    channel.map_color(f)
}

/// Offsets a color channel in `space`.
pub fn offset_channel(channel: &Channel, space: ColorSpace, delta: Vec4) -> Channel {
    channel.map_color(|color| offset_color(color, space, delta))
}

/// Scales a color channel in `space`.
pub fn scale_channel(channel: &Channel, space: ColorSpace, factor: Vec4) -> Channel {
    channel.map_color(|color| scale_color(color, space, factor))
}

/// Randomizes a color channel in `space` with a seed.
pub fn randomize_channel(
    channel: &Channel,
    space: ColorSpace,
    seed: u64,
    min: Vec4,
    max: Vec4,
) -> Channel {
    let mut rng = Rng::new(seed);
    channel.map_color(|color| randomize_color(color, space, &mut rng, min, max))
}

/// Applies any single-color transform to a whole color channel.
pub fn transform_channel(channel: &Channel, f: impl Fn(Color4) -> Color4) -> Channel {
    channel.map_color(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lighten_and_darken_move_lightness() {
        let gray = Color4::new(0.5, 0.5, 0.5, 1.0);
        let lighter = lighten(gray, 0.25);
        assert!((lighter.r - 0.75).abs() < 1e-6);
        let darker = darken(gray, 0.25);
        assert!((darker.r - 0.25).abs() < 1e-6);
    }

    #[test]
    fn saturate_desaturate_and_grayscale() {
        let red = Color4::new(1.0, 0.0, 0.0, 1.0);
        let gray = grayscale(red);
        assert!((gray.r - gray.g).abs() < 1e-6 && (gray.g - gray.b).abs() < 1e-6);
        let muted = desaturate(red, 0.5);
        assert!(muted.g > red.g);
    }

    #[test]
    fn adjust_hue_rotates_through_the_wheel() {
        let red = Color4::new(1.0, 0.0, 0.0, 1.0);
        let green = adjust_hue(red, 120.0);
        assert!(green.g > 0.9 && green.r < 0.1, "green = {:?}", green);
        let back = adjust_hue(green, 240.0);
        assert!((back.r - 1.0).abs() < 1e-6, "back = {:?}", back);
    }

    #[test]
    fn complement_and_invert() {
        let red = Color4::new(1.0, 0.0, 0.0, 1.0);
        let cyan = complement(red);
        assert!(cyan.g > 0.9 && cyan.b > 0.9 && cyan.r < 0.1);
        let inverted = invert(red);
        assert_eq!(inverted, Color4::new(0.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn mix_tint_shade() {
        let white = Color4::WHITE;
        let black = Color4::BLACK;
        let mid = mix(black, white, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
        let tinted = tint(black, 0.25);
        assert!((tinted.r - 0.25).abs() < 1e-6);
        let shaded = shade(white, 0.25);
        assert!((shaded.r - 0.75).abs() < 1e-6);
    }

    #[test]
    fn offset_and_scale_in_each_space() {
        let base = Color4::new(0.2, 0.4, 0.6, 1.0);
        let rgb = offset_color(base, ColorSpace::Rgb, Vec4::new(0.1, 0.1, 0.1, 0.0));
        assert!((rgb.r - 0.3).abs() < 1e-6);
        // An HSL lightness offset brightens while keeping the hue.
        let hsl = offset_color(base, ColorSpace::Hsl, Vec4::new(0.0, 0.0, 0.1, 0.0));
        assert!(hsl.r > base.r);
        // HSB and HSL differ but both round-trip.
        for space in [
            ColorSpace::Rgb,
            ColorSpace::Hsl,
            ColorSpace::Hsb,
            ColorSpace::Hwb,
        ] {
            let round = space.from_space(space.to_space(base));
            assert!((round.r - base.r).abs() < 1e-6, "{space:?}");
        }
    }

    #[test]
    fn randomize_is_seeded() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        let base = Color4::new(0.5, 0.5, 0.5, 1.0);
        let delta = Vec4::new(-0.1, -0.1, -0.1, 0.0);
        let ca = randomize_color(
            base,
            ColorSpace::Hsl,
            &mut a,
            delta,
            Vec4::new(0.1, 0.1, 0.1, 0.0),
        );
        let cb = randomize_color(
            base,
            ColorSpace::Hsl,
            &mut b,
            delta,
            Vec4::new(0.1, 0.1, 0.1, 0.0),
        );
        assert_eq!(ca, cb);
    }

    #[test]
    fn linear_gradient_samples_stops() {
        let stops = [
            ColorStop::new(0.0, Color4::BLACK),
            ColorStop::new(1.0, Color4::WHITE),
        ];
        let mid = sample_gradient(&stops, 0.5, ColorSpace::Rgb, false);
        assert!((mid.r - 0.5).abs() < 1e-6);
        // Out of range clamps.
        assert_eq!(
            sample_gradient(&stops, 2.0, ColorSpace::Rgb, false),
            Color4::WHITE
        );
        // Repeating wraps.
        let wrapped = sample_gradient(&stops, 1.25, ColorSpace::Rgb, true);
        assert!((wrapped.r - 0.25).abs() < 1e-6);
    }

    #[test]
    fn gradient_channel_samples_a_color_channel() {
        let stops = [
            ColorStop::new(0.0, Color4::BLACK),
            ColorStop::new(1.0, Color4::WHITE),
        ];
        let channel = gradient_channel(&stops, ColorSpace::Rgb, 5, 0.0, 100.0, false);
        assert_eq!(channel.component_count(), 4);
        let colors = channel.colors().unwrap();
        assert_eq!(colors.len(), 5);
        assert!((colors[2].1.r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn channel_color_space_ops() {
        let mut kv = KeyframedValue::new();
        kv.add(0.0, Color4::new(1.0, 0.0, 0.0, 1.0));
        let channel = Channel::Color(kv);
        let lightened = map_color(&channel, |c| lighten(c, 0.2));
        let colors = lightened.colors().unwrap();
        assert!(colors[0].1.r > 1.0 - 1e-6 && colors[0].1.g > 0.0);
    }
}
