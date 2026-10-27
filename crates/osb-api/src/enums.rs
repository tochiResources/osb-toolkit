//! Wire enums, re-exported from the engine (the source of truth) so the contract
//! and the generated SDK types keep one definition. `HitObjectKind` is a
//! projection - the runtime's `Slider` carries path data - so it stays here.

use osb_runtime::mapset::HitObjectKind as RuntimeHitObjectKind;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use osb_runtime::animations::Easing;
pub use osb_runtime::storyboarding::{OsbLayer, OsbLoopType, OsbOrigin, ParameterType};

/// A beatmap hit object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum HitObjectKind {
    Circle,
    Slider,
    Spinner,
    Hold,
}

impl From<&RuntimeHitObjectKind> for HitObjectKind {
    fn from(kind: &RuntimeHitObjectKind) -> Self {
        match kind {
            RuntimeHitObjectKind::Circle => HitObjectKind::Circle,
            RuntimeHitObjectKind::Slider(_) => HitObjectKind::Slider,
            RuntimeHitObjectKind::Spinner => HitObjectKind::Spinner,
            RuntimeHitObjectKind::Hold => HitObjectKind::Hold,
        }
    }
}
