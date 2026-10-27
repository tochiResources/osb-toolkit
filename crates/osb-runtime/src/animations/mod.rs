//! Easing, keyframes and `KeyframedValue` interpolation.
//!
//! The dimension-agnostic animation core (easing, keyframes, `KeyframedValue`,
//! keyframe channels and the timeline sandbox) lives in `osb-scene`; it is
//! re-exported here so `crate::animations::*` and `osb_runtime::animations::*`
//! paths stay stable. The helper libraries (`color_ops`, `easing_ops`) stay in
//! the runtime and build on the shared core.

pub mod color_ops;
pub mod easing_ops;

pub use crate::common::animations::keyframes;
pub use crate::common::animations::timeline;
pub use crate::common::animations::{
    cubic_bezier, Axis, Channel, Easing, FlattenMode, Interpolate, Keyframe, KeyframedValue,
};
pub use crate::common::animations::{flush_timeline, merge_timelines, Timeline, TimelineClip};

pub use color_ops::{ColorSpace, ColorStop};
