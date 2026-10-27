//! Storyboard and mapset diagnostics.
//!
//! Command diagnostics - per-sprite **command cost**,
//! **overlapping**, **duplicated** and **redundant (no-op)** commands, invalid
//! time ranges and empty textures - plus mapset sanity, returning structured
//! findings (a stable [`DiagnosticCode`], a severity, an optional layer/sprite/
//! time location and a human-readable description) that consumers can log.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::mapset::Beatmap;
use crate::storyboarding::{Command, CommandType, Layer, Storyboard};

/// The severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum DiagnosticSeverity {
    /// Informational only.
    Info,
    /// A likely mistake.
    Warning,
    /// A definite problem.
    Error,
}

/// A stable, machine-readable diagnostic code.
///
/// The code is the source of truth: it is the enum, and its wire value is the
/// dotted code string (e.g. `osb.command.overlap`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum DiagnosticCode {
    /// A sprite's texture path is empty.
    #[serde(rename = "osb.sprite.empty_texture")]
    SpriteEmptyTexture,
    /// A sprite has a high command cost.
    #[serde(rename = "osb.sprite.high_command_cost")]
    SpriteHighCommandCost,
    /// A command ends before it starts.
    #[serde(rename = "osb.command.invalid_range")]
    CommandInvalidRange,
    /// A command is a no-op (zero duration, unchanged value).
    #[serde(rename = "osb.command.noop")]
    CommandNoop,
    /// A command duplicates an earlier command of the same kind.
    #[serde(rename = "osb.command.duplicate")]
    CommandDuplicate,
    /// Two commands of the same kind overlap in time.
    #[serde(rename = "osb.command.overlap")]
    CommandOverlap,
    /// The beatmap has no audio filename.
    #[serde(rename = "osu.audio.missing")]
    OsuAudioMissing,
    /// The beatmap has no timing points.
    #[serde(rename = "osu.timing.none")]
    OsuTimingNone,
    /// The beatmap has no hit objects.
    #[serde(rename = "osu.hitobjects.none")]
    OsuHitObjectsNone,
    /// A timing point has a non-positive beat duration.
    #[serde(rename = "osu.timing.invalid")]
    OsuTimingInvalid,
}

impl DiagnosticCode {
    /// The stable dotted code string (its wire value).
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticCode::SpriteEmptyTexture => "osb.sprite.empty_texture",
            DiagnosticCode::SpriteHighCommandCost => "osb.sprite.high_command_cost",
            DiagnosticCode::CommandInvalidRange => "osb.command.invalid_range",
            DiagnosticCode::CommandNoop => "osb.command.noop",
            DiagnosticCode::CommandDuplicate => "osb.command.duplicate",
            DiagnosticCode::CommandOverlap => "osb.command.overlap",
            DiagnosticCode::OsuAudioMissing => "osu.audio.missing",
            DiagnosticCode::OsuTimingNone => "osu.timing.none",
            DiagnosticCode::OsuHitObjectsNone => "osu.hitobjects.none",
            DiagnosticCode::OsuTimingInvalid => "osu.timing.invalid",
        }
    }
}

impl std::fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A single diagnostic finding.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// The diagnostic code.
    pub code: DiagnosticCode,
    /// The severity.
    pub severity: DiagnosticSeverity,
    /// A human-readable description.
    pub message: String,
    /// The layer index, when the finding is layer-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<usize>,
    /// The sprite index, when the finding is sprite-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<usize>,
    /// The time in milliseconds, when the finding is time-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

/// A report: the findings plus per-severity counts.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    /// The findings, in analysis order.
    pub diagnostics: Vec<Diagnostic>,
    /// The number of `Error` findings.
    pub error_count: usize,
    /// The number of `Warning` findings.
    pub warning_count: usize,
    /// The number of `Info` findings.
    pub info_count: usize,
}

/// The command cost above which a sprite is flagged as expensive.
const HIGH_COMMAND_COST: usize = 500;

/// Analyzes a storyboard, returning its findings.
pub fn diagnose_storyboard(storyboard: &Storyboard) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (layer_index, layer) in storyboard.layers().enumerate() {
        diagnose_layer(layer, layer_index, &mut diagnostics);
    }
    diagnostics
}

fn diagnose_layer(layer: &Layer, layer_index: usize, out: &mut Vec<Diagnostic>) {
    for (sprite_index, sprite) in layer.sprites().iter().enumerate() {
        if sprite.exclude {
            continue;
        }
        if sprite.texture_path.trim().is_empty() {
            out.push(finding(
                DiagnosticCode::SpriteEmptyTexture,
                DiagnosticSeverity::Error,
                format!("sprite {sprite_index} on layer {layer_index} has an empty texture path"),
                Some(layer_index),
                Some(sprite_index),
                None,
            ));
        }
        let cost = command_cost(sprite.commands());
        if cost > HIGH_COMMAND_COST {
            out.push(finding(
                DiagnosticCode::SpriteHighCommandCost,
                DiagnosticSeverity::Info,
                format!(
                    "sprite {sprite_index} on layer {layer_index} has a high command cost ({cost})"
                ),
                Some(layer_index),
                Some(sprite_index),
                None,
            ));
        }
        diagnose_timeline(sprite.commands(), layer_index, sprite_index, out);
        for loop_group in sprite.loops() {
            diagnose_timeline(loop_group.commands(), layer_index, sprite_index, out);
        }
        for trigger in sprite.triggers() {
            diagnose_timeline(trigger.commands(), layer_index, sprite_index, out);
        }
    }
}

fn command_cost(commands: &HashMap<CommandType, Vec<Command>>) -> usize {
    commands.values().map(Vec::len).sum()
}

fn diagnose_timeline(
    commands: &HashMap<CommandType, Vec<Command>>,
    layer: usize,
    sprite: usize,
    out: &mut Vec<Diagnostic>,
) {
    for (kind, list) in commands {
        for command in list {
            if command.end_time < command.start_time {
                out.push(finding(
                    DiagnosticCode::CommandInvalidRange,
                    DiagnosticSeverity::Error,
                    format!("{kind:?} command ends before it starts"),
                    Some(layer),
                    Some(sprite),
                    Some(command.start_time),
                ));
            } else if command.start_time == command.end_time
                && command.start_value == command.end_value
            {
                out.push(finding(
                    DiagnosticCode::CommandNoop,
                    DiagnosticSeverity::Info,
                    format!("{kind:?} command is a no-op (zero duration, unchanged value)"),
                    Some(layer),
                    Some(sprite),
                    Some(command.start_time),
                ));
            }
        }

        for (index, command) in list.iter().enumerate() {
            if list[index + 1..].contains(command) {
                out.push(finding(
                    DiagnosticCode::CommandDuplicate,
                    DiagnosticSeverity::Warning,
                    format!("duplicate {kind:?} command"),
                    Some(layer),
                    Some(sprite),
                    Some(command.start_time),
                ));
            }
        }

        let mut sorted: Vec<&Command> = list.iter().collect();
        sorted.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for pair in sorted.windows(2) {
            if pair[0] != pair[1] && pair[1].start_time < pair[0].end_time {
                out.push(finding(
                    DiagnosticCode::CommandOverlap,
                    DiagnosticSeverity::Warning,
                    format!("overlapping {kind:?} commands"),
                    Some(layer),
                    Some(sprite),
                    Some(pair[1].start_time),
                ));
            }
        }
    }
}

/// Analyzes a beatmap, returning its findings.
pub fn diagnose_beatmap(beatmap: &Beatmap) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if beatmap.audio_filename.trim().is_empty() {
        diagnostics.push(finding(
            DiagnosticCode::OsuAudioMissing,
            DiagnosticSeverity::Warning,
            "the beatmap has no audio filename",
            None,
            None,
            None,
        ));
    }
    if beatmap.timing_points.is_empty() {
        diagnostics.push(finding(
            DiagnosticCode::OsuTimingNone,
            DiagnosticSeverity::Error,
            "the beatmap has no timing points",
            None,
            None,
            None,
        ));
    }
    if beatmap.hit_objects.is_empty() {
        diagnostics.push(finding(
            DiagnosticCode::OsuHitObjectsNone,
            DiagnosticSeverity::Warning,
            "the beatmap has no hit objects",
            None,
            None,
            None,
        ));
    }
    for (index, point) in beatmap.timing_points.iter().enumerate() {
        if point.timing_change && point.beat_duration <= 0.0 {
            diagnostics.push(finding(
                DiagnosticCode::OsuTimingInvalid,
                DiagnosticSeverity::Error,
                format!("timing point {index} has a non-positive beat duration"),
                None,
                None,
                Some(point.time),
            ));
        }
    }
    diagnostics
}

/// Wraps findings into a report with per-severity counts.
pub fn summarize(diagnostics: Vec<Diagnostic>) -> DiagnosticReport {
    let mut report = DiagnosticReport {
        diagnostics,
        error_count: 0,
        warning_count: 0,
        info_count: 0,
    };
    for diagnostic in &report.diagnostics {
        match diagnostic.severity {
            DiagnosticSeverity::Error => report.error_count += 1,
            DiagnosticSeverity::Warning => report.warning_count += 1,
            DiagnosticSeverity::Info => report.info_count += 1,
        }
    }
    report
}

fn finding(
    code: DiagnosticCode,
    severity: DiagnosticSeverity,
    message: impl Into<String>,
    layer: Option<usize>,
    sprite: Option<usize>,
    time: Option<f64>,
) -> Diagnostic {
    Diagnostic {
        code,
        severity,
        message: message.into(),
        layer,
        sprite,
        time,
    }
}

#[cfg(test)]
mod tests {
    use crate::animations::Easing;
    use crate::math::Vec2;
    use crate::storyboarding::OsbOrigin;

    use super::*;

    fn codes(report: &DiagnosticReport) -> Vec<DiagnosticCode> {
        report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect()
    }

    #[test]
    fn flags_duplicate_overlap_and_noop_commands() {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer("Foreground")
            .create_sprite("sb/dot.png", OsbOrigin::Centre);
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None);
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None); // duplicate
        sprite.fade(50.0, 150.0, 1.0, 0.0, Easing::None); // overlaps the first
        sprite.move_(200.0, 200.0, Vec2::ZERO, Vec2::ZERO, Easing::None); // no-op

        let report = summarize(diagnose_storyboard(&storyboard));
        let codes = codes(&report);
        assert!(codes.contains(&DiagnosticCode::CommandDuplicate));
        assert!(codes.contains(&DiagnosticCode::CommandOverlap));
        assert!(codes.contains(&DiagnosticCode::CommandNoop));
        assert!(report.warning_count >= 2);
        assert!(report.info_count >= 1);
    }

    #[test]
    fn flags_an_empty_texture_and_an_invalid_range() {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer("Foreground")
            .create_sprite("", OsbOrigin::Centre);
        sprite.fade(100.0, 0.0, 0.0, 1.0, Easing::None);

        let report = summarize(diagnose_storyboard(&storyboard));
        let codes = codes(&report);
        assert!(codes.contains(&DiagnosticCode::SpriteEmptyTexture));
        assert!(codes.contains(&DiagnosticCode::CommandInvalidRange));
        assert!(report.error_count >= 2);
    }

    #[test]
    fn a_clean_storyboard_has_no_findings() {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer("Foreground")
            .create_sprite("sb/dot.png", OsbOrigin::Centre);
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None);
        sprite.move_(
            100.0,
            200.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Easing::None,
        );

        let report = summarize(diagnose_storyboard(&storyboard));
        assert!(report.diagnostics.is_empty(), "{:?}", codes(&report));
    }

    #[test]
    fn flags_mapset_sanity() {
        let beatmap = Beatmap::default();
        let report = summarize(diagnose_beatmap(&beatmap));
        let codes = codes(&report);
        assert!(codes.contains(&DiagnosticCode::OsuAudioMissing));
        assert!(codes.contains(&DiagnosticCode::OsuTimingNone));
        assert!(codes.contains(&DiagnosticCode::OsuHitObjectsNone));
    }
}
