//! Wire shapes for the parsed beatmap (`[General]`, `[TimingPoints]`,
//! `[HitObjects]`).

use osb_runtime::mapset::{Beatmap as RuntimeBeatmap, ControlPoint as RuntimeControlPoint};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::enums::HitObjectKind;
use crate::math::Color4;

/// A timing or difficulty control point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ControlPoint {
    /// The point's start time in milliseconds.
    pub time: f64,
    /// Beat duration in milliseconds (only meaningful on red lines).
    pub beat_duration: f64,
    /// Slider velocity multiplier (`1.0` on red lines).
    pub speed_multiplier: f64,
    /// Tempo in beats per minute derived from the beat duration.
    pub bpm: f64,
    /// Whether this is a red (uninherited) line.
    pub timing_change: bool,
    /// Whether the first bar line is omitted.
    pub omit_first_bar_line: bool,
    /// The osu! effect flags bitfield.
    pub effect_flags: i32,
}

impl From<&RuntimeControlPoint> for ControlPoint {
    fn from(point: &RuntimeControlPoint) -> Self {
        ControlPoint {
            time: point.time,
            beat_duration: point.beat_duration,
            speed_multiplier: point.speed_multiplier,
            bpm: point.bpm(),
            timing_change: point.timing_change,
            omit_first_bar_line: point.omit_first_bar_line,
            effect_flags: point.effect_flags,
        }
    }
}

/// A single beatmap hit object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HitObject {
    /// The object's start time in milliseconds.
    pub start_time: f64,
    /// The object's end time in milliseconds (equal to `start_time` for circles).
    pub end_time: f64,
    /// Horizontal position in osu! pixels.
    pub x: f64,
    /// Vertical position in osu! pixels.
    pub y: f64,
    /// The object kind.
    pub kind: HitObjectKind,
    /// Whether the object starts a new combo.
    pub new_combo: bool,
    /// The stacking height used for visual offsets.
    pub stack_height: i32,
    /// The combo color assigned to the object.
    pub color: Color4,
}

impl From<&osb_runtime::mapset::HitObject> for HitObject {
    fn from(hit_object: &osb_runtime::mapset::HitObject) -> Self {
        HitObject {
            start_time: hit_object.start_time,
            end_time: hit_object.end_time,
            x: hit_object.position.x,
            y: hit_object.position.y,
            kind: (&hit_object.kind).into(),
            new_combo: hit_object.new_combo,
            stack_height: hit_object.stack_height,
            color: hit_object.color,
        }
    }
}

/// A parsed beatmap: metadata, difficulty, control points and hit objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
    /// The `.osu` format version.
    pub format_version: u32,
    /// The audio filename referenced by the map.
    pub audio_filename: String,
    /// The map title.
    pub title: String,
    /// The map artist.
    pub artist: String,
    /// The map creator.
    pub creator: String,
    /// The difficulty name.
    pub version: String,
    /// The ruleset mode (`0` = osu!standard, `1` = taiko, `2` = catch, `3` = mania).
    pub mode: u32,
    /// Approach rate.
    pub approach_rate: f64,
    /// Circle size.
    pub circle_size: f64,
    /// Overall difficulty.
    pub overall_difficulty: f64,
    /// Slider velocity multiplier.
    pub slider_multiplier: f64,
    /// Slider tick rate.
    pub slider_tick_rate: f64,
    /// Stack leniency.
    pub stack_leniency: f64,
    /// The map's background filename from `[Events]`, if any.
    pub background_path: Option<String>,
    /// Combo colors from `[Colours]` (empty when the map has none).
    pub combo_colors: Vec<Color4>,
    /// The map's control points.
    pub timing_points: Vec<ControlPoint>,
    /// The map's hit objects.
    pub hit_objects: Vec<HitObject>,
}

impl From<&RuntimeBeatmap> for Beatmap {
    fn from(beatmap: &RuntimeBeatmap) -> Self {
        Beatmap {
            format_version: beatmap.format_version,
            audio_filename: beatmap.audio_filename.clone(),
            title: beatmap.title.clone(),
            artist: beatmap.artist.clone(),
            creator: beatmap.creator.clone(),
            version: beatmap.version.clone(),
            mode: beatmap.mode,
            approach_rate: beatmap.approach_rate,
            circle_size: beatmap.circle_size,
            overall_difficulty: beatmap.overall_difficulty,
            slider_multiplier: beatmap.slider_multiplier,
            slider_tick_rate: beatmap.slider_tick_rate,
            stack_leniency: beatmap.stack_leniency,
            background_path: beatmap.background_path.clone(),
            combo_colors: beatmap.combo_colors.to_vec(),
            timing_points: beatmap
                .timing_points
                .iter()
                .map(ControlPoint::from)
                .collect(),
            hit_objects: beatmap.hit_objects.iter().map(HitObject::from).collect(),
        }
    }
}
