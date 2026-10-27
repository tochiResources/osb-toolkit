//! The media library: probing, frame extraction/encoding and audio.

use crate::error::Error;
use crate::models::{EncodeVideoRequest, ExtractFramesRequest, FrameResult, MediaInfo};

/// Probes a media file.
pub fn info(path: &str) -> Result<MediaInfo, Error> {
    crate::Client::media_info(path)
}

/// Extracts frames from a video into `output_dir`.
pub fn extract_frames(request: ExtractFramesRequest) -> Result<FrameResult, Error> {
    crate::Client::extract_frames(request)
}

/// Encodes an image sequence into a video.
pub fn encode_video(request: EncodeVideoRequest) -> Result<MediaInfo, Error> {
    crate::Client::encode_video(request)
}

/// Extracts the audio track of a media file.
pub fn extract_audio(input: &str, output: &str) -> Result<(), Error> {
    crate::Client::extract_audio(input, output)
}
