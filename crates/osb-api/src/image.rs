//! The image library wire types. The metadata/options/request types live in
//! `osb-image` (the source of truth) and are re-exported here so the contract,
//! the paths and the generated SDK models keep one definition. `PathRequest` is
//! a generic bridge request shared by several paths.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use osb_image::{ImageCrop, ImageInfo, ImageOptions, ImageProcessRequest};

/// A request carrying a single file path.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PathRequest {
    /// The input file path.
    pub path: String,
}
