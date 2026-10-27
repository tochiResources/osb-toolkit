use crate::animations::Easing;
use crate::math::{Color4, Vec2};

/// The kinds of command a sprite timeline can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandType {
    Move,
    MoveX,
    MoveY,
    Scale,
    ScaleVec,
    Rotate,
    Fade,
    Color,
    ColorHsb,
    Parameter,
    /// An animatable sprite origin offset. Not an osu! `.osb` command; it only
    /// affects runtime rendering.
    Anchor,
    Loop,
    Trigger,
}

/// A command's endpoint value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandValue {
    Scalar(f64),
    Vector2(Vec2),
    Color(Color4),
    Parameter(ParameterTypeValue),
    None,
}

/// Numeric parameter payload for `P` commands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterTypeValue {
    FlipHorizontal(f64),
    FlipVertical(f64),
    Additive(f64),
}

impl CommandValue {
    pub fn as_scalar(&self) -> f64 {
        match self {
            CommandValue::Scalar(value) => *value,
            CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(value))
            | CommandValue::Parameter(ParameterTypeValue::FlipVertical(value))
            | CommandValue::Parameter(ParameterTypeValue::Additive(value)) => *value,
            _ => 0.0,
        }
    }

    pub fn as_vec2(&self) -> Vec2 {
        match self {
            CommandValue::Vector2(value) => *value,
            CommandValue::Scalar(value) => Vec2::new(*value, *value),
            _ => Vec2::ZERO,
        }
    }

    pub fn as_color(&self) -> Color4 {
        match self {
            CommandValue::Color(value) => *value,
            _ => Color4::WHITE,
        }
    }

    pub fn lerp(&self, other: &CommandValue, t: f64) -> CommandValue {
        match (self, other) {
            (CommandValue::Scalar(a), CommandValue::Scalar(b)) => {
                CommandValue::Scalar(a + (b - a) * t)
            }
            (CommandValue::Vector2(a), CommandValue::Vector2(b)) => {
                CommandValue::Vector2(a.lerp(*b, t))
            }
            (CommandValue::Color(a), CommandValue::Color(b)) => CommandValue::Color(a.lerp(*b, t)),
            (CommandValue::Parameter(a), CommandValue::Parameter(b)) => {
                CommandValue::Parameter(match (a, b) {
                    (
                        ParameterTypeValue::FlipHorizontal(x),
                        ParameterTypeValue::FlipHorizontal(y),
                    ) => ParameterTypeValue::FlipHorizontal(x + (y - x) * t),
                    (ParameterTypeValue::FlipVertical(x), ParameterTypeValue::FlipVertical(y)) => {
                        ParameterTypeValue::FlipVertical(x + (y - x) * t)
                    }
                    _ => ParameterTypeValue::Additive(a.value() + (b.value() - a.value()) * t),
                })
            }
            _ => *self,
        }
    }
}

impl ParameterTypeValue {
    pub fn value(&self) -> f64 {
        match self {
            ParameterTypeValue::FlipHorizontal(value)
            | ParameterTypeValue::FlipVertical(value)
            | ParameterTypeValue::Additive(value) => *value,
        }
    }
}

/// A single time-based command on a sprite timeline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command {
    pub start_time: f64,
    pub end_time: f64,
    pub easing: Easing,
    pub start_value: CommandValue,
    pub end_value: CommandValue,
}

impl Command {
    pub fn new(
        start_time: f64,
        end_time: f64,
        easing: Easing,
        start_value: CommandValue,
        end_value: CommandValue,
    ) -> Self {
        Command {
            start_time,
            end_time,
            easing,
            start_value,
            end_value,
        }
    }

    pub fn value_at(&self, time: f64) -> CommandValue {
        if time <= self.start_time {
            return self.start_value;
        }
        if time >= self.end_time {
            return self.end_value;
        }
        let span = self.end_time - self.start_time;
        let t = if span <= 0.0 {
            1.0
        } else {
            ((time - self.start_time) / span) as f32
        };
        let eased = self.easing.apply(t) as f64;
        self.start_value.lerp(&self.end_value, eased)
    }
}
