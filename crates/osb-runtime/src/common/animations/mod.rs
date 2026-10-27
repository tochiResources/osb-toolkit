//! Easing, keyframes and `KeyframedValue` interpolation - the dimension-agnostic
//! animation core shared by the 2D and 3D storyboarding layers.

mod easing;
mod keyframe;
mod keyframed_value;
pub mod keyframes;
pub mod timeline;

pub use easing::{cubic_bezier, Easing};
pub use keyframe::Keyframe;
pub use keyframed_value::{Interpolate, KeyframedValue};
pub use keyframes::{Axis, Channel, FlattenMode};
pub use timeline::{flush_timeline, merge_timelines, Timeline, TimelineClip};
