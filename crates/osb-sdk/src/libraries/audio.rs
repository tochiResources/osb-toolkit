//! The audio library: FFT.

use crate::error::Error;
use crate::models::FftResult;

/// Reads FFT magnitudes from a `.wav` file.
pub fn fft(path: &str, time: f64, magnitudes: usize) -> Result<FftResult, Error> {
    crate::Client::fft(path, time, magnitudes)
}
