//! Shared JSON request helpers for the stateless (`exec`) and stateful
//! (`runtime`) bridges, so enum parsing lives in one place.

use osb_api::{Easing as ApiEasing, EasingResult};
use osb_runtime::animations::Easing;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::to_json;

/// Deserializes a string-enum field from the OpenAPI contract.
pub(crate) fn enum_field<T: DeserializeOwned>(value: &Value, key: &str) -> Option<T> {
    value
        .get(key)
        .and_then(|field| serde_json::from_value(field.clone()).ok())
}

/// Parses an easing from its contract name, falling back to the legacy numeric
/// id (`easing` or `id` field).
pub(crate) fn easing(value: &Value) -> Easing {
    if let Some(easing) = enum_field::<ApiEasing>(value, "easing") {
        return easing;
    }
    value
        .get("easing")
        .or_else(|| value.get("id"))
        .and_then(Value::as_u64)
        .and_then(|id| Easing::from_id(id as u8))
        .unwrap_or(Easing::None)
}

/// Handles the stateless `applyEasing` operation.
pub(crate) fn apply_easing(value: &Value) -> String {
    let t = value.get("t").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    to_json(&EasingResult {
        value: easing(value).apply(t) as f64,
    })
}
