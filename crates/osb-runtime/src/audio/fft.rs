use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

use crate::animations::Easing;
use crate::error::{RuntimeError, RuntimeResult};

/// A mono audio buffer with FFT support.
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    sample_rate: u32,
    samples: Vec<f32>,
}

const FFT_SIZE: usize = 4096;

impl AudioBuffer {
    pub fn from_mono(sample_rate: u32, samples: Vec<f32>) -> Self {
        AudioBuffer {
            sample_rate,
            samples,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn duration(&self) -> f64 {
        if self.sample_rate == 0 {
            0.0
        } else {
            self.samples.len() as f64 / self.sample_rate as f64
        }
    }

    /// Loads a 16-bit or 32-bit float PCM WAV file, down-mixed to mono.
    pub fn from_wav_file(path: impl AsRef<std::path::Path>) -> RuntimeResult<Self> {
        let bytes = osb_core::io::read(path)?;
        parse_wav(&bytes)
    }

    /// FFT magnitudes at `time` (milliseconds).
    ///
    /// Returns `magnitudes` normalized bins (`0.0..=1.0`) after applying `easing`;
    /// bins whose frequency fraction exceeds `cutoff` are zeroed when `cutoff > 0`.
    pub fn get_fft(
        &self,
        time_ms: f64,
        magnitudes: usize,
        easing: Easing,
        cutoff: f64,
    ) -> Vec<f64> {
        let mut bins = vec![0.0f32; magnitudes];
        if self.samples.is_empty() || magnitudes == 0 {
            return bins.into_iter().map(f64::from).collect();
        }
        let start = ((time_ms / 1000.0) * self.sample_rate as f64) as isize;
        let mut window = vec![Complex::new(0.0f32, 0.0); FFT_SIZE];
        for (i, slot) in window.iter_mut().enumerate() {
            let index = start + i as isize;
            let sample = if index >= 0 && (index as usize) < self.samples.len() {
                self.samples[index as usize]
            } else {
                0.0
            };
            let hann =
                0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32).cos();
            *slot = Complex::new(sample * hann, 0.0);
        }

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);
        fft.process(&mut window);

        let nyquist = FFT_SIZE / 2;
        let scale = 2.0 / FFT_SIZE as f32;
        for (bin, output) in bins.iter_mut().enumerate() {
            let index = ((bin + 1) * nyquist) / magnitudes;
            let magnitude = window[index.min(nyquist)].norm() * scale;
            *output = magnitude.clamp(0.0, 1.0);
        }

        for value in &mut bins {
            *value = easing.apply(*value);
        }

        if cutoff > 0.0 {
            let limit = (cutoff.clamp(0.0, 1.0) * magnitudes as f64) as usize;
            for value in bins.iter_mut().skip(limit) {
                *value = 0.0;
            }
        }

        bins.into_iter().map(f64::from).collect()
    }
}

fn parse_wav(bytes: &[u8]) -> RuntimeResult<AudioBuffer> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err(RuntimeError::Asset("not a RIFF/WAVE file".into()));
    }
    let mut cursor = 12;
    let mut channels = 1u16;
    let mut sample_rate = 44100u32;
    let mut bits = 16u16;
    let mut format = 1u16;
    let mut data: Option<&[u8]> = None;

    while cursor + 8 <= bytes.len() {
        let id = &bytes[cursor..cursor + 4];
        let size = u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        let body = cursor + 8;
        if id == b"fmt " && body + 16 <= bytes.len() {
            format = u16::from_le_bytes(bytes[body..body + 2].try_into().unwrap());
            channels = u16::from_le_bytes(bytes[body + 2..body + 4].try_into().unwrap());
            sample_rate = u32::from_le_bytes(bytes[body + 4..body + 8].try_into().unwrap());
            bits = u16::from_le_bytes(bytes[body + 14..body + 16].try_into().unwrap());
        } else if id == b"data" {
            let end = (body + size).min(bytes.len());
            data = Some(&bytes[body..end]);
        }
        cursor = body + size + (size & 1);
    }

    let data = data.ok_or_else(|| RuntimeError::Asset("missing WAV data chunk".into()))?;
    let channels = channels.max(1) as usize;
    let mut mono = Vec::new();

    match (format, bits) {
        (1, 16) => {
            for frame in data.chunks_exact(2 * channels) {
                let mut sum = 0.0f32;
                for channel in 0..channels {
                    let offset = channel * 2;
                    let sample = i16::from_le_bytes(frame[offset..offset + 2].try_into().unwrap());
                    sum += sample as f32 / 32768.0;
                }
                mono.push(sum / channels as f32);
            }
        }
        (3, 32) => {
            for frame in data.chunks_exact(4 * channels) {
                let mut sum = 0.0f32;
                for channel in 0..channels {
                    let offset = channel * 4;
                    sum += f32::from_le_bytes(frame[offset..offset + 4].try_into().unwrap());
                }
                mono.push(sum / channels as f32);
            }
        }
        _ => {
            return Err(RuntimeError::Asset(format!(
                "unsupported WAV format {format} with {bits}-bit samples"
            )));
        }
    }

    Ok(AudioBuffer::from_mono(sample_rate, mono))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_of_silence_is_zero() {
        let buffer = AudioBuffer::from_mono(44100, vec![0.0; 44100]);
        let bins = buffer.get_fft(0.0, 16, Easing::None, 0.0);
        assert_eq!(bins.len(), 16);
        assert!(bins.iter().all(|&value| value == 0.0));
    }

    #[test]
    fn fft_of_tone_has_energy() {
        let sample_rate = 44100u32;
        let samples: Vec<f32> = (0..sample_rate)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate as f32).sin())
            .collect();
        let buffer = AudioBuffer::from_mono(sample_rate, samples);
        let bins = buffer.get_fft(0.0, 64, Easing::None, 0.0);
        assert!(bins.iter().cloned().fold(0.0f64, f64::max) > 0.0);
    }

    #[test]
    fn cutoff_zeroes_high_bins() {
        let buffer = AudioBuffer::from_mono(44100, vec![0.5; 8192]);
        let bins = buffer.get_fft(0.0, 16, Easing::None, 0.5);
        assert!(bins[12..].iter().all(|&value| value == 0.0));
    }
}
