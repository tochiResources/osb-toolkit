//! The shared, dimension-agnostic modules used by the 2D (`storyboarding`) and
//! 3D (`storyboarding3d`) layers: the animation core (easing, keyframes,
//! `KeyframedValue`, channels), the timeline sandbox, the shared osu! layer and
//! origin types, and the deterministic RNG.

pub mod animations;
pub mod layer;
mod rng;

pub use layer::{OsbLayer, OsbOrigin};
pub use rng::Rng;
