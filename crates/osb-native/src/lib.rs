//! C ABI for the whole `osb-runtime` public API ().
//!
//! A small, stable C surface drives the same engine from C#, C++, Python and
//! (via WASM) JavaScript/TypeScript. Stateful objects are opaque handles; rich
//! operations go through the JSON bridge [`osbtk_exec`] with typed convenience
//! entry points ([`osbtk_version`], [`osbtk_bake3d`]). Strings cross as
//! NUL-terminated UTF-8 and are released with [`osbtk_string_free`]; panics never
//! cross the boundary.
//!
//! Coverage: `mapset` (`.osu` parse), `storyboard` (`.osb` import), `audio` (FFT)
//! and `storyboarding3d` (scene bake); additional operations are added in
//! [`exec`](exec::exec) alone, so the C surface stays small and versioned.

mod bake;
mod exec;
mod ffi;
mod helpers;
mod json;
mod math_ops;
mod runtime;

pub use bake::{bake_scene_to_json, bake_to_json};
pub use exec::exec;
pub use ffi::{
    osbtk_bake3d, osbtk_exec, osbtk_runtime_exec, osbtk_runtime_free, osbtk_runtime_new,
    osbtk_string_free, osbtk_version,
};
pub use osb_api::{openapi, openapi_json};
pub use runtime::{runtime_exec, OsbtkRuntime};

/// The JSON error envelope shared by every entry point.
pub(crate) fn error_json(message: &str) -> String {
    serde_json::json!({ "error": message }).to_string()
}

/// Serializes a typed `osb-api` response, falling back to the error envelope.
pub(crate) fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|error| error_json(&error.to_string()))
}
