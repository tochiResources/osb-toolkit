//! The diagnostics library: storyboard and mapset analysis.

use crate::error::Error;
use crate::models::DiagnosticReport;

/// Diagnoses an `.osb` storyboard.
pub fn diagnose_osb(text: &str) -> Result<DiagnosticReport, Error> {
    crate::Client::diagnose_osb(text)
}

/// Diagnoses an `.osu` beatmap.
pub fn diagnose_osu(text: &str) -> Result<DiagnosticReport, Error> {
    crate::Client::diagnose_osu(text)
}
