//! The [Axis] and [FlattenMode] selectors for keyframe channels.

/// Which component of a channel an axis-aware helper acts on.
///
/// `X`/`Y`/`Z` address the vector axes (and, for a color channel, the red/green/
/// blue components); `W` addresses a color's alpha. A channel only has the
/// components its shape provides, so an axis that does not exist is a no-op.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
    /// The fourth (alpha) component of a color channel.
    W,
}

impl Axis {
    /// Parses `"x"`/`"y"`/`"z"`/`"w"` (case-insensitive).
    pub fn parse(name: &str) -> Option<Axis> {
        match name.to_ascii_lowercase().as_str() {
            "x" => Some(Axis::X),
            "y" => Some(Axis::Y),
            "z" => Some(Axis::Z),
            "w" | "a" | "alpha" => Some(Axis::W),
            _ => None,
        }
    }
}

/// How [`Channel::flatten`] reduces a channel to a constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlattenMode {
    /// The mean of all keyframe values.
    Mean,
    /// The first keyframe's value.
    First,
    /// The last keyframe's value.
    Last,
    /// A fixed value.
    Value(f64),
}
