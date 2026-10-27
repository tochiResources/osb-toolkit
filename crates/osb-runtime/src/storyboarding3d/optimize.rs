use serde::Serialize;

use crate::animations::KeyframedValue;
use crate::math::{Color4, Vec2};

/// The kind of a baked 2D command (`.osb` letters plus additive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandKind {
    Move,
    ScaleVec,
    Rotate,
    Fade,
    Color,
    Additive,
    /// A horizontal flip (`P,H,1`), emitted for mirrored mesh faces.
    FlipH,
}

/// A command's endpoint value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum BakedValue {
    Scalar(f64),
    Vector2(Vec2),
    Color(Color4),
}

/// A linear command span produced by the bake; consecutive simplified keyframes
/// become one span each.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BakedCommand {
    pub kind: CommandKind,
    pub start_time: f64,
    pub end_time: f64,
    pub start: BakedValue,
    pub end: BakedValue,
}

pub(crate) fn spans_2d(kind: CommandKind, value: &KeyframedValue<Vec2>) -> Vec<BakedCommand> {
    let keyframes = value.keyframes();
    if keyframes.is_empty() {
        return Vec::new();
    }
    if keyframes.len() == 1 {
        return vec![BakedCommand {
            kind,
            start_time: keyframes[0].time,
            end_time: keyframes[0].time,
            start: BakedValue::Vector2(keyframes[0].value),
            end: BakedValue::Vector2(keyframes[0].value),
        }];
    }
    keyframes
        .windows(2)
        .map(|pair| BakedCommand {
            kind,
            start_time: pair[0].time,
            end_time: pair[1].time,
            start: BakedValue::Vector2(pair[0].value),
            end: BakedValue::Vector2(pair[1].value),
        })
        .collect()
}

pub(crate) fn spans_1d(kind: CommandKind, value: &KeyframedValue<f64>) -> Vec<BakedCommand> {
    let keyframes = value.keyframes();
    if keyframes.is_empty() {
        return Vec::new();
    }
    if keyframes.len() == 1 {
        return vec![BakedCommand {
            kind,
            start_time: keyframes[0].time,
            end_time: keyframes[0].time,
            start: BakedValue::Scalar(keyframes[0].value),
            end: BakedValue::Scalar(keyframes[0].value),
        }];
    }
    keyframes
        .windows(2)
        .map(|pair| BakedCommand {
            kind,
            start_time: pair[0].time,
            end_time: pair[1].time,
            start: BakedValue::Scalar(pair[0].value),
            end: BakedValue::Scalar(pair[1].value),
        })
        .collect()
}

pub(crate) fn spans_color(value: &KeyframedValue<Color4>) -> Vec<BakedCommand> {
    let keyframes = value.keyframes();
    if keyframes.is_empty() {
        return Vec::new();
    }
    if keyframes.len() == 1 {
        return vec![BakedCommand {
            kind: CommandKind::Color,
            start_time: keyframes[0].time,
            end_time: keyframes[0].time,
            start: BakedValue::Color(keyframes[0].value),
            end: BakedValue::Color(keyframes[0].value),
        }];
    }
    keyframes
        .windows(2)
        .map(|pair| BakedCommand {
            kind: CommandKind::Color,
            start_time: pair[0].time,
            end_time: pair[1].time,
            start: BakedValue::Color(pair[0].value),
            end: BakedValue::Color(pair[1].value),
        })
        .collect()
}
