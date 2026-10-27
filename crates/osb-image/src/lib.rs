//! Canonical image library.
//!
//! Decodes and encodes images (PNG/JPEG/BMP/GIF), and applies fine-grained
//! processing - resize, crop, rotate/flip, grayscale/invert, brightness,
//! contrast and Gaussian blur - through [`process`]. Images cross the bridge by
//! file path so the JSON contract stays small.

pub mod types;

pub use types::{ImageCrop, ImageInfo, ImageOptions, ImageProcessRequest};

use image::imageops::FilterType;
use image::DynamicImage;

/// Reads an image's metadata.
pub fn image_info(path: &str) -> Result<ImageInfo, String> {
    let image = image::open(path).map_err(|error| error.to_string())?;
    Ok(info(&image, path))
}

/// Reads `path`, applies `options`, writes `output`, and returns the result's
/// metadata.
pub fn process(path: &str, output: &str, options: &ImageOptions) -> Result<ImageInfo, String> {
    let mut image = image::open(path).map_err(|error| error.to_string())?;
    image = apply(image, options);
    if let Some(parent) = std::path::Path::new(output).parent() {
        if !parent.as_os_str().is_empty() {
            osb_core::io::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
    }
    image.save(output).map_err(|error| error.to_string())?;
    Ok(info(&image, output))
}

fn apply(mut image: DynamicImage, options: &ImageOptions) -> DynamicImage {
    image = resize(image, options.resize_width, options.resize_height);
    if let Some(crop) = options.crop {
        let x = crop.x.min(image.width().saturating_sub(1));
        let y = crop.y.min(image.height().saturating_sub(1));
        let width = crop.width.min(image.width() - x).max(1);
        let height = crop.height.min(image.height() - y).max(1);
        image = image.crop_imm(x, y, width, height);
    }
    if let Some(degrees) = options.rotate {
        image = match (degrees.round() as i64).rem_euclid(360) {
            90 => image.rotate90(),
            180 => image.rotate180(),
            270 => image.rotate270(),
            _ => image,
        };
    }
    if options.flip_horizontal.unwrap_or(false) {
        image = image.fliph();
    }
    if options.flip_vertical.unwrap_or(false) {
        image = image.flipv();
    }
    if options.grayscale.unwrap_or(false) {
        image = image.grayscale();
    }
    if options.invert.unwrap_or(false) {
        image.invert();
    }
    if let Some(brightness) = options.brightness {
        image = image.brighten((brightness.clamp(-1.0, 1.0) * 255.0) as i32);
    }
    if let Some(contrast) = options.contrast {
        image = image.adjust_contrast(contrast.clamp(-1.0, 1.0) as f32);
    }
    if let Some(sigma) = options.blur {
        if sigma > 0.0 {
            image = image.blur(sigma as f32);
        }
    }
    image
}

fn resize(image: DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    match (width, height) {
        (Some(width), Some(height)) => {
            image.resize(width.max(1), height.max(1), FilterType::Lanczos3)
        }
        (Some(width), None) => {
            let width = width.max(1);
            let height = derived(image.height(), image.width(), width);
            image.resize(width, height, FilterType::Lanczos3)
        }
        (None, Some(height)) => {
            let height = height.max(1);
            let width = derived(image.width(), image.height(), height);
            image.resize(width, height, FilterType::Lanczos3)
        }
        (None, None) => image,
    }
}

/// Scales `other` to match `target` on the axis whose `source` counterpart is
/// `source`, preserving aspect ratio.
fn derived(other: u32, source: u32, target: u32) -> u32 {
    if source == 0 {
        return target.max(1);
    }
    ((other as f64 * target as f64 / source as f64).round() as u32).max(1)
}

fn info(image: &DynamicImage, path: &str) -> ImageInfo {
    let format = std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_uppercase())
        .unwrap_or_default();
    ImageInfo {
        width: image.width(),
        height: image.height(),
        format,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_path() -> std::path::PathBuf {
        let directory = osb_core::io::temp_dir().join("osbtk-image-tests");
        osb_core::io::create_dir_all(&directory).unwrap();
        directory.join("sample.png")
    }

    fn write_sample(path: &std::path::Path) {
        let image = DynamicImage::new_rgba8(64, 32);
        image.save(path).unwrap();
    }

    #[test]
    fn reads_metadata() {
        let path = sample_path();
        write_sample(&path);
        let info = image_info(path.to_str().unwrap()).unwrap();
        assert_eq!(info.width, 64);
        assert_eq!(info.height, 32);
        assert_eq!(info.format, "PNG");
    }

    #[test]
    fn resizes_crops_and_transforms() {
        let path = sample_path();
        write_sample(&path);
        let output = path.with_file_name("out.png");
        let info = process(
            path.to_str().unwrap(),
            output.to_str().unwrap(),
            &ImageOptions {
                resize_width: Some(32),
                crop: Some(ImageCrop {
                    x: 0,
                    y: 0,
                    width: 16,
                    height: 16,
                }),
                grayscale: Some(true),
                rotate: Some(90.0),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(info.width, 16);
        assert_eq!(info.height, 16);
        assert_eq!(info.format, "PNG");
    }

    #[test]
    fn rejects_missing_files() {
        assert!(image_info("does-not-exist.png").is_err());
    }
}
