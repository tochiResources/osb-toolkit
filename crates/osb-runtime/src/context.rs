use std::path::PathBuf;

use crate::animations::Easing;
use crate::assets::Bitmap;
use crate::audio::AudioBuffer;
use crate::mapset::Beatmap;
use crate::math::Vec2;

/// The effect context: beatmap, audio, resolution and the deterministic RNG.
///
/// It is deliberately separate from the [`crate::storyboarding::Storyboard`] the
/// effect builds, so an effect can hold sprite references (borrowing the
/// storyboard) while it reads the beatmap or audio.
pub struct Context {
    pub resolution: Vec2,
    beatmap: Option<Beatmap>,
    audio: Option<AudioBuffer>,
    asset_root: Option<PathBuf>,
    rng: Rng,
}

impl Context {
    pub fn new(width: f32, height: f32) -> Self {
        Context {
            resolution: Vec2::new(width as f64, height as f64),
            beatmap: None,
            audio: None,
            asset_root: None,
            rng: Rng::new(0x9E37_79B9_7F4A_7C15),
        }
    }

    pub fn set_beatmap(&mut self, beatmap: Beatmap) {
        self.beatmap = Some(beatmap);
    }

    pub fn set_audio(&mut self, audio: AudioBuffer) {
        self.audio = Some(audio);
    }

    /// Sets the directory that relative mapset asset paths resolve against.
    pub fn set_asset_root(&mut self, root: impl Into<PathBuf>) {
        self.asset_root = Some(root.into());
    }

    pub fn beatmap(&self) -> Option<&Beatmap> {
        self.beatmap.as_ref()
    }

    pub fn audio_duration(&self) -> f64 {
        self.audio
            .as_ref()
            .map(AudioBuffer::duration)
            .unwrap_or(0.0)
    }

    /// FFT magnitudes at `time`; the optional path is
    /// accepted for signature compatibility and ignored for the single buffer.
    /// `magnitudes` is `f64` so generated code can pass a configurable directly.
    pub fn get_fft(
        &self,
        time_ms: f64,
        magnitudes: f64,
        _path: Option<&str>,
        easing: Easing,
        cutoff: f64,
    ) -> Vec<f64> {
        let count = magnitudes.max(0.0) as usize;
        self.audio
            .as_ref()
            .map(|audio| audio.get_fft(time_ms, count, easing, cutoff))
            .unwrap_or_else(|| vec![0.0; count])
    }

    /// The mapset image dimensions. Returns a
    /// zero-sized bitmap when the file cannot be read, matching "no asset".
    pub fn get_mapset_bitmap<P: AsRef<str>>(&self, path: P) -> Bitmap {
        let path = path.as_ref();
        let resolved = match &self.asset_root {
            Some(root) => root.join(path),
            None => PathBuf::from(path),
        };
        Bitmap::from_file(resolved).unwrap_or_default()
    }

    /// Reads a text asset, resolving relative paths against the asset root
    /// (used by the `.osb` importer's `OpenProjectFile` equivalent).
    pub fn read_text(&self, path: &str) -> Option<String> {
        let resolved = match &self.asset_root {
            Some(root) => root.join(path),
            None => PathBuf::from(path),
        };
        osb_core::io::read_string(resolved).ok()
    }

    pub fn seed(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
    }

    /// Deterministic float in `min..max`.
    pub fn random(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.rng.next_f64()
    }

    /// Deterministic integer in `min..max` (exclusive upper bound).
    pub fn random_int(&mut self, min: i64, max: i64) -> i64 {
        if max <= min {
            return min;
        }
        min + (self.rng.next_u64() % (max - min) as u64) as i64
    }
}

/// The deterministic RNG now lives in `osb-scene`; re-exported so
/// `crate::context::Rng` stays stable.
pub use crate::common::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_is_deterministic_for_seed() {
        let mut a = Context::new(1.0, 1.0);
        let mut b = Context::new(1.0, 1.0);
        a.seed(42);
        b.seed(42);
        assert_eq!(a.random(0.0, 1.0), b.random(0.0, 1.0));
    }
}
