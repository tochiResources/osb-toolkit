//! The osu! slider path now lives in the canonical `osb-curve` library (M24).
//! It is re-exported here so `osb_runtime::mapset::{CurveType, SliderPath}`
//! keeps working.

pub use osb_core::curve::{CurveType, SliderPath};
