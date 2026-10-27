//! Sprite rasterization and compositing into a frame buffer.

mod compositor;

pub use compositor::{origin_anchor, render, Concurrency, RenderOptions};
