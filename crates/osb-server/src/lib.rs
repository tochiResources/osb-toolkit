//! The osbTK service layer: the single implementation behind the CLI and the
//! headless HTTP API (section 5.6, M15/M16).
//!
//! Conversion shells out to the bundled Node converter CLI (`osbtk-fx-convert`,
//! `packages/core/dist/cli.js`) - the transpiler lives in TypeScript - while the
//! 3D bake runs in-process through `osb-native` and builds drive `cargo`.

mod auth;
mod http;
mod jobs;
mod package;
mod ratelimit;
mod service;
mod zip;

pub use auth::default_path as default_auth_path;
pub use auth::{Auth, AuthConfig, AuthError, Claims, LoginRequest, TokenPair, UserView};
pub use http::{router, serve, spawn, ServerConfig, ServerHandle, ServerState};
pub use jobs::{
    JobId, JobQueue, JobRecord, JobRunner, JobSpec, JobStatus, QueueConfig, QueueError,
};
pub use package::{Component, Installer, InstallerConfig, License, Options, ProductInfo, Scope};
pub use ratelimit::{RateLimitConfig, RateLimiter};
pub use service::{
    Artifact, ArtifactKind, BuildRequest, ConvertRequest, ConvertResult, GeneratedFile, Service,
    ServiceConfig, ServiceError,
};
