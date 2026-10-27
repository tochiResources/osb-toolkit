//! The media library wire types, re-exported from `osb-media` (the source of
//! truth) so the contract, the paths and the generated SDK models keep one
//! definition.

pub use osb_media::{
    EncodeVideoRequest, ExtractAudioRequest, ExtractFramesRequest, FrameResult, MediaInfo,
};
