use crate::math::{Color4, Vec2};

use super::control_point::ControlPoint;
use super::hitobject::HitObject;
use super::stacking;

/// A parsed beatmap: metadata, difficulty, control points and hit objects.
#[derive(Debug, Clone, Default)]
pub struct Beatmap {
    pub format_version: u32,
    pub audio_filename: String,
    pub title: String,
    pub artist: String,
    pub creator: String,
    pub version: String,
    pub mode: u32,
    pub approach_rate: f64,
    pub circle_size: f64,
    pub overall_difficulty: f64,
    pub slider_multiplier: f64,
    pub slider_tick_rate: f64,
    pub stack_leniency: f64,
    /// The map's background filename from the `[Events]` section, if any.
    pub background_path: Option<String>,
    /// Combo colours from the `[Colours]` section (`Combo1..Combo8`).
    pub combo_colors: Vec<Color4>,
    pub timing_points: Vec<ControlPoint>,
    pub hit_objects: Vec<HitObject>,
}

impl Beatmap {
    pub fn new() -> Self {
        Beatmap {
            approach_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
            stack_leniency: 0.7,
            ..Default::default()
        }
    }

    /// The active red (uninherited) timing point at `time`.
    pub fn timing_point_at(&self, time: f64) -> Option<&ControlPoint> {
        self.timing_points
            .iter()
            .filter(|point| point.timing_change)
            .take_while(|point| point.time <= time)
            .last()
            .or_else(|| self.timing_points.iter().find(|point| point.timing_change))
    }

    /// The active red timing point at `time`, defaulting to 120 BPM when absent
    /// (never returns null).
    pub fn timing_point_at_value(&self, time: f64) -> ControlPoint {
        self.timing_point_at(time)
            .copied()
            .unwrap_or_else(|| ControlPoint::new(0.0, 500.0, true))
    }

    /// The active control point (red or green) at `time`.
    pub fn control_point_at(&self, time: f64) -> Option<&ControlPoint> {
        self.timing_points
            .iter()
            .take_while(|point| point.time <= time)
            .last()
            .or_else(|| self.timing_points.first())
    }

    pub fn bpm_at(&self, time: f64) -> f64 {
        self.timing_point_at(time)
            .map(ControlPoint::bpm)
            .unwrap_or(0.0)
    }

    pub fn beat_duration_at(&self, time: f64) -> f64 {
        self.timing_point_at(time)
            .map(|p| p.beat_duration)
            .unwrap_or(0.0)
    }

    /// Applies stacking to the parsed hit objects and records each object's
    /// visual stack offset.
    pub fn apply_stacking(&mut self) {
        stacking::apply_stacking(
            &mut self.hit_objects,
            self.stack_leniency,
            self.approach_rate,
        );
        let circle_size = self.circle_size;
        for object in &mut self.hit_objects {
            object.stack_offset = stacking::stack_offset(object.stack_height, circle_size);
        }
    }

    /// Assigns a combo colour to each hit object.
    pub fn apply_colors(&mut self) {
        let palette = if self.combo_colors.is_empty() {
            default_combo_colors()
        } else {
            self.combo_colors.clone()
        };
        let mut color_index = 0usize;
        for object in &mut self.hit_objects {
            if object.new_combo && color_index > 0 {
                color_index += 1;
            }
            object.color = palette[color_index % palette.len()];
        }
    }

    /// The stacked visual position of a hit object.
    pub fn stacked_position(&self, object: &HitObject) -> Vec2 {
        object.position + object.stack_offset
    }
}

/// osu!'s default combo colours, used when a map has no `[Colours]` section.
fn default_combo_colors() -> Vec<Color4> {
    vec![
        Color4::from_rgb(1.0, 0.75, 0.8),
        Color4::from_rgb(0.98, 0.88, 0.55),
        Color4::from_rgb(0.7, 0.9, 0.75),
        Color4::from_rgb(0.65, 0.82, 0.95),
        Color4::from_rgb(0.85, 0.72, 0.95),
        Color4::from_rgb(0.98, 0.72, 0.62),
    ]
}
