use crate::common::animations::Easing;

/// A single keyframe: a value at a point in time (milliseconds) plus the easing
/// that governs the segment **from this keyframe to the next**.
///
/// Easing is per-segment (like a CSS timing function or a GSAP ease), so a
/// keyframed channel preserves the shape of motion - not just the values - and
/// the command <-> channel bridge can round-trip a command's easing. The default
/// is [`Easing::None`] (linear).
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe<T> {
    pub time: f64,
    pub value: T,
    pub easing: Easing,
}

impl<T> Keyframe<T> {
    /// A linear keyframe.
    pub const fn new(time: f64, value: T) -> Self {
        Keyframe {
            time,
            value,
            easing: Easing::None,
        }
    }

    /// A keyframe whose outgoing segment uses `easing`.
    pub const fn with_easing(time: f64, value: T, easing: Easing) -> Self {
        Keyframe {
            time,
            value,
            easing,
        }
    }
}
