//! The osbTK foundation crate.
//!
//! One crate for everything below the runtime: the OpenTK/osuTK-compatible
//! math types, the shared file-operations API (native + WASM host bridge) and the
//! canonical curve math. The runtime and the capability libraries are built on
//! top of it; nothing here depends on them.

pub mod curve;
pub mod io;
pub mod io_wasm;
pub mod math;
