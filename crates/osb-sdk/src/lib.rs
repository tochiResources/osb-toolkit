//! Generated Rust SDK over the osbTK native bridge ().
//!
//! The models in [`models`] are generated from the OpenAPI contract in
//! `crates/osb-api`; [`Client`] and [`Runtime`] drive them through the one
//! canonical engine (`crates/osb-native`), so a Rust consumer gets the same typed
//! surface as the C#, TypeScript, Python, C++, Go and Java SDKs. This crate is a
//! consumer of the runtime, distinct from the `osb-runtime`/`osb-native`
//! source-of-truth crates.

// The generated models carry their own (unused) imports and style; they are
// treated as generated code and not linted.
#[allow(warnings, clippy::all)]
pub mod models;

mod client;
mod error;
mod generator;
pub mod helpers;
mod libraries;
mod runtime;

pub use client::Client;
pub use error::Error;
pub use generator::{run, Generate, Generator};
pub use libraries::{
    animations, audio, curve, diagnostics, image, image_gen, media, optimize, storyboarding,
    storyboarding3d,
};
pub use runtime::{Runtime, Segment};

/// The shared file-operations API (`osb-io`): the single place the toolkit talks
/// to the filesystem. Re-exported so SDK consumers use the canonical paths,
/// reads, writes and temp-file helpers rather than `std::fs`.
pub use osb_core::io;
