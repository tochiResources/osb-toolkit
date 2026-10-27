//! The image-generation library: draw primitive shapes (M25).

use crate::error::Error;
use crate::models::{ImageInfo, RenderShapesRequest};

/// Renders primitive shapes to a new image.
pub fn render_shapes(request: RenderShapesRequest) -> Result<ImageInfo, Error> {
    crate::Client::render_shapes(request)
}
