//! The image library: decode/encode and processing (M23).

use crate::error::Error;
use crate::models::{ImageInfo, ImageOptions};

/// Reads image metadata.
pub fn info(path: &str) -> Result<ImageInfo, Error> {
    crate::Client::image_info(path)
}

/// Processes an image (read, apply options, write).
pub fn process(path: &str, output: &str, options: ImageOptions) -> Result<ImageInfo, Error> {
    crate::Client::image_process(path, output, options)
}
