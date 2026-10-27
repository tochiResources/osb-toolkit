use std::collections::HashMap;
use std::path::Path;

use crate::error::{RuntimeError, RuntimeResult};

/// An RGBA8 texture.
#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    /// Row-major RGBA pixels, `width * height * 4` bytes.
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Texture {
            width,
            height,
            pixels,
        }
    }

    pub fn from_rgba8(width: u32, height: u32, pixels: Vec<u8>) -> RuntimeResult<Self> {
        if pixels.len() != (width as usize) * (height as usize) * 4 {
            return Err(RuntimeError::Asset("texture buffer size mismatch".into()));
        }
        Ok(Texture {
            width,
            height,
            pixels,
        })
    }

    pub fn from_png_file(path: impl AsRef<Path>) -> RuntimeResult<Self> {
        let bytes = osb_core::io::read(path)?;
        Texture::from_png_bytes(&bytes)
    }

    pub fn from_png_bytes(bytes: &[u8]) -> RuntimeResult<Self> {
        let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
            .map_err(|error| RuntimeError::Asset(format!("failed to decode PNG: {error}")))?
            .to_rgba8();
        Ok(Texture {
            width: image.width(),
            height: image.height(),
            pixels: image.into_raw(),
        })
    }

    /// Nearest-neighbour sample; returns transparent black outside the bounds.
    pub fn sample(&self, x: f64, y: f64) -> [u8; 4] {
        if x < 0.0 || y < 0.0 || x >= self.width as f64 || y >= self.height as f64 {
            return [0, 0, 0, 0];
        }
        let index = ((y as usize) * self.width as usize + x as usize) * 4;
        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        ]
    }
}

/// Resolves sprite texture paths to loaded textures, honouring user overrides.
#[derive(Debug, Default, Clone)]
pub struct TextureStore {
    textures: HashMap<String, Texture>,
}

impl TextureStore {
    pub fn new() -> Self {
        TextureStore {
            textures: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: impl Into<String>, texture: Texture) {
        self.textures.insert(normalize(&path.into()), texture);
    }

    /// Registers an override so a script's default path resolves to a host asset.
    pub fn override_path(&mut self, script_path: impl Into<String>, texture: Texture) {
        self.insert(script_path, texture);
    }

    pub fn get(&self, path: &str) -> Option<&Texture> {
        self.textures.get(&normalize(path))
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }
}

fn normalize(path: &str) -> String {
    path.replace('\\', "/").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_normalizes_paths() {
        let mut store = TextureStore::new();
        store.insert("SB\\Bar.png", Texture::new(1, 1, vec![255, 0, 0, 255]));
        assert!(store.get("sb/bar.png").is_some());
    }

    #[test]
    fn sample_out_of_bounds_is_transparent() {
        let texture = Texture::new(1, 1, vec![255, 255, 255, 255]);
        assert_eq!(texture.sample(-1.0, 0.0), [0, 0, 0, 0]);
        assert_eq!(texture.sample(0.0, 0.0), [255, 255, 255, 255]);
    }
}
