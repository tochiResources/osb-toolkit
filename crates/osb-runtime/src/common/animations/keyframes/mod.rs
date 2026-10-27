//! Pure motion-graphics helpers over keyframe channels (M38).
//!
//! Every helper here is a pure function over a [`Channel`] (a keyframed scalar,
//! 2-vector or 3-vector) or a small constructor that produces one. They carry no
//! runtime state, so they are the canonical implementation shared by the wire
//! bridge (`osb-native`), the SDK facades, the 2D command helpers and the 3D bake
//! alike. Because a [`Channel`] is dimension-agnostic, **the Z axis is always
//! optional**: a 2D author passes a scalar or 2-vector channel and the
//! axis-aware helpers simply leave the missing components alone. The strictly 3D
//! constructors (`orbit`/`spin`/`tumble`/`pathFollow`) live in the 3D helper
//! module (in the runtime) and reuse this module.

mod axis;
mod channel;
mod constructors;
mod helpers;
mod sampling;
mod transforms;

pub use axis::{Axis, FlattenMode};
pub use channel::Channel;
pub use constructors::{pulse, scatter, stagger};

#[cfg(test)]
mod tests;
