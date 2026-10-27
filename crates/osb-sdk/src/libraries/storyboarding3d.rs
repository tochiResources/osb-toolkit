//! The 3D storyboard library: a sibling that reuses the whole
//! [`storyboarding`](super::storyboarding) surface and adds the 3D -> 2D bake and
//! the motion-graphics helpers, so the 3D API is exposed alongside `Storyboarding`
//! (M28, M38).

use crate::error::Error;
use crate::models::{BakedStoryboard, Scene3dInput};

/// `.osu`/`.osb` parsing, re-exported from the storyboarding library.
pub use super::storyboarding::{parse_osb, parse_osu};

/// Bakes a 3D scene into a 2D storyboard.
pub fn bake3d(scene: Scene3dInput) -> Result<BakedStoryboard, Error> {
    crate::Client::bake3d(scene)
}

/// The motion-graphics keyframe helpers (M38). These now live in the shared
/// [`crate::helpers`] surface so 2D and 3D use the same code; this module is a
/// convenience re-export kept for `Storyboarding3d` continuity.
pub mod helpers {
    /// The shared keyframe channel transforms (2D and 3D; Z is optional).
    pub use crate::helpers::keyframe::*;
    /// The strictly 3D constructors (`orbit`/`spin`/`tumble`/`pathFollow`).
    pub use crate::helpers::scene3d::*;
    /// The shared channel types.
    pub use crate::helpers::{Channel, ChannelKey};
}
