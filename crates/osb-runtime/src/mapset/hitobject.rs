use crate::math::{Color4, Vec2};

use super::slider::SliderPath;

/// The kind-specific data of a hit object.
#[derive(Debug, Clone)]
pub enum HitObjectKind {
    Circle,
    Slider(SliderPath),
    Spinner,
    Hold,
}

/// A single beatmap hit object (circle, slider, spinner or mania hold note).
#[derive(Debug, Clone)]
pub struct HitObject {
    pub start_time: f64,
    pub end_time: f64,
    pub position: Vec2,
    pub kind: HitObjectKind,
    pub stack_height: i32,
    /// Visual stacking offset in osu! pixels.
    pub stack_offset: Vec2,
    /// The combo colour assigned to this object.
    pub color: Color4,
    /// Whether this object starts a new combo (osu! `kind` bit 2).
    pub new_combo: bool,
    pub combo_index: usize,
    pub combo_color_index: usize,
}

impl HitObject {
    pub fn new(start_time: f64, end_time: f64, position: Vec2, kind: HitObjectKind) -> Self {
        HitObject {
            start_time,
            end_time,
            position,
            kind,
            stack_height: 0,
            stack_offset: Vec2::ZERO,
            color: Color4::WHITE,
            new_combo: false,
            combo_index: 0,
            combo_color_index: 0,
        }
    }

    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }

    /// Position of the hit object at `time`, following slider paths.
    pub fn position_at_time(&self, time: f64) -> Vec2 {
        match &self.kind {
            HitObjectKind::Slider(path) => {
                let duration = self.duration();
                if duration <= 0.0 {
                    return self.position;
                }
                let progress = ((time - self.start_time) / duration).clamp(0.0, 1.0);
                path.position_at(progress)
            }
            _ => self.position,
        }
    }

    pub fn is_slider(&self) -> bool {
        matches!(self.kind, HitObjectKind::Slider(_))
    }

    pub fn is_spinner(&self) -> bool {
        matches!(self.kind, HitObjectKind::Spinner)
    }

    pub fn is_circle(&self) -> bool {
        matches!(self.kind, HitObjectKind::Circle)
    }

    pub fn is_hold(&self) -> bool {
        matches!(self.kind, HitObjectKind::Hold)
    }
}
