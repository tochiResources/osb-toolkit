use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use crate::common::{OsbLayer, OsbOrigin};

/// A stable, non-borrowing handle to a layer in the [`super::Storyboard`].
///
/// Generated code (and the `.osb` importer) hold handles instead of `&mut`
/// references so a helper can hold a sprite while also calling `GetLayer`/
/// `Random`: a handle is `Copy` and does not borrow the storyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerHandle(pub usize);

/// A stable, non-borrowing handle to a sprite in the [`super::Storyboard`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteHandle {
    pub layer: usize,
    pub sprite: usize,
}

/// How an animation advances after its final frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub enum OsbLoopType {
    #[default]
    LoopOnce,
    LoopForever,
}

impl OsbLoopType {
    /// Parses an `.osb` loop type name (`LoopOnce`, `LoopForever`).
    pub fn from_name(name: &str) -> Option<OsbLoopType> {
        match name.trim() {
            "LoopOnce" => Some(OsbLoopType::LoopOnce),
            "LoopForever" => Some(OsbLoopType::LoopForever),
            _ => None,
        }
    }

    /// The canonical `.osb` loop type name.
    pub fn name(self) -> &'static str {
        match self {
            OsbLoopType::LoopOnce => "LoopOnce",
            OsbLoopType::LoopForever => "LoopForever",
        }
    }
}

/// A sprite parameter command kind (`P` in `.osb`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ParameterType {
    FlipHorizontal,
    FlipVertical,
    Additive,
}
