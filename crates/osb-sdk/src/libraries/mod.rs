//! The SDK organized into libraries, mirroring the contract's tags. Each library
//! is its own module so the surface is navigable, e.g.
//! `osb_sdk::libraries::curve::length`.

pub mod animations;
pub mod audio;
pub mod curve;
pub mod diagnostics;
pub mod image;
pub mod image_gen;
pub mod media;
pub mod optimize;
pub mod storyboarding;
pub mod storyboarding3d;
