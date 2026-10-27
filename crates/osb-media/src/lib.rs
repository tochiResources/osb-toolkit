//! Media library.
//!
//! Probes media, extracts frames (video -> image sequence), encodes frames
//! (image sequence -> video) and extracts audio, by shelling out to the
//! `ffmpeg`/`ffprobe` binaries. A `bundled ffmpeg` (or `ffmpeg-next`) can
//! replace the CLI calls later without changing this surface.

pub mod types;

pub use types::{
    EncodeVideoRequest, ExtractAudioRequest, ExtractFramesRequest, FrameResult, MediaInfo,
};

use std::path::Path;
use std::process::Command;

/// Whether `ffmpeg`/`ffprobe` are available on `PATH`.
pub fn available() -> bool {
    tool("ffprobe", &["-version"]) && tool("ffmpeg", &["-version"])
}

/// Probes a media file for its duration, dimensions and codecs.
pub fn media_info(path: &str) -> Result<MediaInfo, String> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            path,
        ])
        .output()
        .map_err(|error| format!("ffprobe: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    Ok(parse_info(&value))
}

fn parse_info(value: &serde_json::Value) -> MediaInfo {
    let duration = value["format"]["duration"]
        .as_str()
        .and_then(|text| text.parse::<f64>().ok())
        .unwrap_or(0.0);
    let format = value["format"]["format_name"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let mut info = MediaInfo {
        duration,
        width: 0,
        height: 0,
        format,
        video_codec: None,
        audio_codec: None,
    };
    if let Some(streams) = value["streams"].as_array() {
        for stream in streams {
            match stream["codec_type"].as_str() {
                Some("video") => {
                    info.width = stream["width"].as_u64().unwrap_or(0) as u32;
                    info.height = stream["height"].as_u64().unwrap_or(0) as u32;
                    info.video_codec = stream["codec_name"].as_str().map(str::to_string);
                }
                Some("audio") => {
                    info.audio_codec = stream["codec_name"].as_str().map(str::to_string);
                }
                _ => {}
            }
        }
    }
    info
}

/// Extracts frames from a video into `output_dir` as `frame_%04d.png`.
pub fn extract_frames(request: &ExtractFramesRequest) -> Result<FrameResult, String> {
    osb_core::io::create_dir_all(&request.output_dir).map_err(|error| error.to_string())?;
    let pattern = format!(
        "{}/frame_%04d.png",
        request.output_dir.trim_end_matches(['/', '\\'])
    );

    let mut command = Command::new("ffmpeg");
    command.args(["-y", "-v", "error"]);
    if let Some(start) = request.start {
        command.args(["-ss", &format!("{start}")]);
    }
    command.args(["-i", &request.input]);
    let mut filters = Vec::new();
    if let Some(fps) = request.fps {
        filters.push(format!("fps={fps}"));
    }
    if request.width.is_some() || request.height.is_some() {
        let width = request
            .width
            .map(|w| w.to_string())
            .unwrap_or_else(|| "-2".to_string());
        let height = request
            .height
            .map(|h| h.to_string())
            .unwrap_or_else(|| "-2".to_string());
        filters.push(format!("scale={width}:{height}"));
    }
    if !filters.is_empty() {
        command.args(["-vf", &filters.join(",")]);
    }
    if let Some(count) = request.count {
        command.args(["-frames:v", &count.to_string()]);
    }
    command.arg(&pattern);
    run(command)?;

    let count = osb_core::io::read_dir(&request.output_dir)
        .map(|entries| {
            entries
                .into_iter()
                .filter(|entry| entry.name.starts_with("frame_"))
                .count() as u32
        })
        .unwrap_or(0);
    Ok(FrameResult {
        count,
        output_dir: request.output_dir.clone(),
        pattern: "frame_%04d.png".to_string(),
    })
}

/// Encodes an image sequence into a video.
pub fn encode_video(request: &EncodeVideoRequest) -> Result<MediaInfo, String> {
    if let Some(parent) = Path::new(&request.output).parent() {
        if !parent.as_os_str().is_empty() {
            osb_core::io::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
    }
    let codec = request
        .codec
        .clone()
        .unwrap_or_else(|| "libx264".to_string());
    let mut command = Command::new("ffmpeg");
    command.args([
        "-y",
        "-v",
        "error",
        "-framerate",
        &format!("{}", request.fps),
        "-i",
        &request.input_pattern,
        "-c:v",
        &codec,
        "-pix_fmt",
        "yuv420p",
    ]);
    if let Some(crf) = request.crf {
        command.args(["-crf", &crf.to_string()]);
    }
    command.arg(&request.output);
    run(command)?;
    media_info(&request.output)
}

/// Extracts the audio track of a media file.
pub fn extract_audio(request: &ExtractAudioRequest) -> Result<(), String> {
    if let Some(parent) = Path::new(&request.output).parent() {
        if !parent.as_os_str().is_empty() {
            osb_core::io::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
    }
    let mut command = Command::new("ffmpeg");
    command.args(["-y", "-v", "error", "-i", &request.input, "-vn"]);
    command.arg(&request.output);
    run(command)
}

fn run(mut command: Command) -> Result<(), String> {
    let output = command
        .output()
        .map_err(|error| format!("ffmpeg: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn tool(name: &str, args: &[&str]) -> bool {
    Command::new(name)
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work_dir() -> std::path::PathBuf {
        let directory = osb_core::io::temp_dir().join("osbtk-media-tests");
        osb_core::io::create_dir_all(&directory).unwrap();
        directory
    }

    fn make_clip(path: &Path) {
        let status = Command::new("ffmpeg")
            .args([
                "-y",
                "-v",
                "error",
                "-f",
                "lavfi",
                "-i",
                "testsrc=duration=1:size=64x64:rate=10",
                path.to_str().unwrap(),
            ])
            .status()
            .unwrap();
        assert!(status.success(), "failed to synthesize a test clip");
    }

    #[test]
    fn probes_extracts_and_encodes() {
        if !available() {
            return;
        }
        let directory = work_dir();
        let clip = directory.join("clip.mp4");
        make_clip(&clip);

        let info = media_info(clip.to_str().unwrap()).unwrap();
        assert!(info.duration > 0.5);
        assert_eq!(info.width, 64);
        assert_eq!(info.height, 64);

        let frames_dir = directory.join("frames");
        let frames = extract_frames(&ExtractFramesRequest {
            input: clip.to_str().unwrap().to_string(),
            output_dir: frames_dir.to_str().unwrap().to_string(),
            fps: Some(5.0),
            start: None,
            count: Some(3),
            width: None,
            height: None,
        })
        .unwrap();
        assert_eq!(frames.count, 3);

        let output = directory.join("out.mp4");
        let encoded = encode_video(&EncodeVideoRequest {
            input_pattern: format!("{}/frame_%04d.png", frames_dir.to_str().unwrap()),
            output: output.to_str().unwrap().to_string(),
            fps: 5.0,
            codec: None,
            crf: Some(28),
        })
        .unwrap();
        assert_eq!(encoded.width, 64);
    }

    #[test]
    fn rejects_missing_files() {
        if !available() {
            return;
        }
        assert!(media_info("does-not-exist.mp4").is_err());
    }
}
