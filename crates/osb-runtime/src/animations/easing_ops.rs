//! Easing-curve manipulation (M39).
//!
//! A canonical set of transforms over the named [`Easing`] functions and
//! cubic-bezier curves. They are pure and shared by the wire bridge and every
//! SDK, so an author can stretch, stagger, amplify, reverse, mirror, repeat,
//! compose, blend, remap and sample an easing before it is applied to a whole
//! command group, keyframe channel or timeline.

use crate::animations::{cubic_bezier, Easing};

/// Applies `easing` to `t`, remapped from `[0, 1]` onto `[from, to]` (so one
/// easing is stretched over a whole timeframe instead of a single command).
pub fn stretch_easing(easing: Easing, t: f64, from: f64, to: f64) -> f64 {
    if to <= from {
        let value: f32 = if t >= to { 1.0 } else { 0.0 };
        return easing.apply(value) as f64;
    }
    easing.apply(((t - from) / (to - from)).clamp(0.0, 1.0) as f32) as f64
}

/// Amplifies an easing: `linear + (eased - linear) * amount`. Unlike a naive
/// clamp, `amount > 1` strengthens the curve (including any overshoot) and
/// `amount < 1` relaxes it toward linear, keeping the endpoints fixed.
pub fn amplify_easing(easing: Easing, t: f64, amount: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let eased = easing.apply(t as f32) as f64;
    t + (eased - t) * amount
}

/// Time-reverses an easing (`e'(t) = 1 - e(1 - t)`), keeping the endpoints.
pub fn reverse_easing(easing: Easing, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - easing.apply((1.0 - t) as f32) as f64
}

/// Mirrors an easing about `t = 0.5` while keeping `0 -> 0` and `1 -> 1`.
pub fn mirror_easing(easing: Easing, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        0.5 - 0.5 * easing.apply((1.0 - 2.0 * t) as f32) as f64
    } else {
        0.5 + 0.5 * easing.apply((2.0 * t - 1.0) as f32) as f64
    }
}

/// Ping-pongs an easing: it runs forward over the first half and backward over
/// the second (`e'(t) = e(2t)` for `t < 0.5`, else `e(2 - 2t)`).
pub fn ping_pong_easing(easing: Easing, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        easing.apply((2.0 * t) as f32) as f64
    } else {
        easing.apply((2.0 - 2.0 * t) as f32) as f64
    }
}

/// Tiles an easing `count` times across `[0, 1]`.
pub fn repeat_easing(easing: Easing, t: f64, count: usize) -> f64 {
    let count = count.max(1);
    let t = t.clamp(0.0, 1.0) * count as f64;
    easing.apply((t - t.floor()).clamp(0.0, 1.0) as f32) as f64
}

/// Composes two easings: `outer(inner(t))`.
pub fn compose_easing(outer: Easing, inner: Easing, t: f64) -> f64 {
    outer.apply(inner.apply(t.clamp(0.0, 1.0) as f32)) as f64
}

/// Blends two easings by `amount` (`0 = a`, `1 = b`).
pub fn blend_easing(a: Easing, b: Easing, t: f64, amount: f64) -> f64 {
    let t = t.clamp(0.0, 1.0) as f32;
    let amount = amount.clamp(0.0, 1.0);
    (a.apply(t) as f64) * (1.0 - amount) + (b.apply(t) as f64) * amount
}

/// Evaluates a cubic-bezier easing (`easingFromBezier`).
pub fn bezier_easing(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
    cubic_bezier(x1, y1, x2, y2, t)
}

/// Amplifies a cubic-bezier easing about linear (see [`amplify_easing`]).
pub fn amplify_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64, amount: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let eased = cubic_bezier(x1, y1, x2, y2, t);
    t + (eased - t) * amount
}

/// One segment of a piecewise time remap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EasingSegment {
    /// The segment start in normalized time.
    pub start: f64,
    /// The segment end in normalized time.
    pub end: f64,
    /// The output value at `start`.
    pub from: f64,
    /// The output value at `end`.
    pub to: f64,
    /// The easing applied inside the segment.
    pub easing: Easing,
}

/// Evaluates a piecewise easing remap at `t`.
pub fn remap_easing(segments: &[EasingSegment], t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    for segment in segments {
        if t >= segment.start && t <= segment.end {
            let span = segment.end - segment.start;
            let local = if span <= 0.0 {
                1.0
            } else {
                (t - segment.start) / span
            };
            let eased = segment.easing.apply(local as f32) as f64;
            return segment.from + (segment.to - segment.from) * eased;
        }
    }
    // Outside every segment, clamp to the first/last output.
    if let Some(first) = segments.first() {
        if t < first.start {
            return first.from;
        }
    }
    segments.last().map(|s| s.to).unwrap_or(t)
}

/// Samples an easing at `count` evenly spaced points in `[0, 1]`.
pub fn sample_easing(easing: Easing, count: usize) -> Vec<f64> {
    let n = count.max(2);
    (0..n)
        .map(|i| easing.apply((i as f64 / (n - 1) as f64) as f32) as f64)
        .collect()
}

/// Samples a cubic-bezier easing at `count` evenly spaced points in `[0, 1]`.
pub fn sample_bezier(x1: f64, y1: f64, x2: f64, y2: f64, count: usize) -> Vec<f64> {
    let n = count.max(2);
    (0..n)
        .map(|i| cubic_bezier(x1, y1, x2, y2, i as f64 / (n - 1) as f64))
        .collect()
}

/// A stagger amount for element `index` (`index * step`), the easing-library
/// counterpart of the keyframe `stagger`.
pub fn stagger_easing(index: usize, step: f64) -> f64 {
    index as f64 * step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stretch_maps_a_subrange() {
        assert!((stretch_easing(Easing::InQuad, 50.0, 0.0, 100.0) - 0.25).abs() < 1e-6);
        assert!((stretch_easing(Easing::None, 5.0, 0.0, 10.0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn amplify_keeps_endpoints_and_strengthens() {
        // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2 = 0.
        assert!((amplify_easing(Easing::InQuad, 0.5, 2.0) - 0.0).abs() < 1e-6);
        assert!((amplify_easing(Easing::InQuad, 0.0, 5.0)).abs() < 1e-6);
        assert!((amplify_easing(Easing::InQuad, 1.0, 5.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn reverse_and_mirror_keep_endpoints() {
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert!(
                (reverse_easing(Easing::InQuad, t) - (1.0 - (1.0 - t) * (1.0 - t))).abs() < 1e-6
            );
        }
        assert!((mirror_easing(Easing::InQuad, 0.0)).abs() < 1e-6);
        assert!((mirror_easing(Easing::InQuad, 1.0) - 1.0).abs() < 1e-6);
        assert!((mirror_easing(Easing::InQuad, 0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn remap_is_piecewise() {
        let segments = [
            EasingSegment {
                start: 0.0,
                end: 0.5,
                from: 0.0,
                to: 1.0,
                easing: Easing::None,
            },
            EasingSegment {
                start: 0.5,
                end: 1.0,
                from: 1.0,
                to: 0.0,
                easing: Easing::None,
            },
        ];
        assert!((remap_easing(&segments, 0.25) - 0.5).abs() < 1e-6);
        assert!((remap_easing(&segments, 0.75) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn sampling_is_bounded() {
        let samples = sample_easing(Easing::OutQuad, 5);
        assert_eq!(samples.len(), 5);
        assert!(samples[0].abs() < 1e-6);
        assert!((samples[4] - 1.0).abs() < 1e-6);
    }
}
