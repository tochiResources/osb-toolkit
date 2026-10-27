use std::path::Path;

/// Image dimensions, as used by effect scripts (via `GetMapsetBitmap`) to size
/// and place sprites.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Bitmap {
    pub width: f64,
    pub height: f64,
}

impl Bitmap {
    pub fn new(width: f64, height: f64) -> Self {
        Bitmap { width, height }
    }

    /// Reads only the dimensions from an image file, without decoding pixels.
    pub fn from_file(path: impl AsRef<Path>) -> Option<Self> {
        let (width, height) = image::image_dimensions(path).ok()?;
        Some(Bitmap {
            width: width as f64,
            height: height as f64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_is_none() {
        assert!(Bitmap::from_file("does-not-exist.png").is_none());
    }
}
