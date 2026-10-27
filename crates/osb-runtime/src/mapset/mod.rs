//! Beatmap (`.osu`) parsing and the hit-object/timing model.

mod beatmap;
mod control_point;
mod hitobject;
mod parser;
mod slider;
mod stacking;

pub use beatmap::Beatmap;
pub use control_point::ControlPoint;
pub use hitobject::{HitObject, HitObjectKind};
pub use parser::{parse_osu, parse_osu_file};
pub use slider::{CurveType, SliderPath};
pub use stacking::{apply_stacking, stack_offset};
