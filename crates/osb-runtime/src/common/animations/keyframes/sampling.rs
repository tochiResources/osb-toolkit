//! The sampling and time operations over a [`Channel`].

use super::axis::Axis;
use super::channel::Channel;
use super::helpers::{blend_1d, scale_time_1d, shift_1d};
use crate::math::{Color4, Vec2, Vec3};

impl Channel {
    /// Samples the channel as a scalar at `time` (a vector uses its first
    /// component, a color its red channel).
    pub fn scalar_at(&self, time: f64) -> Option<f64> {
        match self {
            Channel::Scalar(kv) => kv.value_at(time),
            Channel::Vector2(kv) => kv.value_at(time).map(|v| v.x),
            Channel::Vector3(kv) => kv.value_at(time).map(|v| v.x),
            Channel::Color(kv) => kv.value_at(time).map(|c| c.r),
        }
    }

    /// Samples the channel as a 2-vector at `time` (a scalar broadcasts; a
    /// 3-vector drops `z`; a color uses its red/green channels).
    pub fn vec2_at(&self, time: f64) -> Option<Vec2> {
        match self {
            Channel::Scalar(kv) => kv.value_at(time).map(|v| Vec2::new(v, v)),
            Channel::Vector2(kv) => kv.value_at(time),
            Channel::Vector3(kv) => kv.value_at(time).map(|v| Vec2::new(v.x, v.y)),
            Channel::Color(kv) => kv.value_at(time).map(|c| Vec2::new(c.r, c.g)),
        }
    }

    /// Samples the channel as a 3-vector at `time` (a scalar broadcasts; a
    /// 2-vector sets `z = 0`; a color uses its red/green/blue channels).
    pub fn vec3_at(&self, time: f64) -> Option<Vec3> {
        match self {
            Channel::Scalar(kv) => kv.value_at(time).map(|v| Vec3::new(v, v, v)),
            Channel::Vector2(kv) => kv.value_at(time).map(|v| Vec3::new(v.x, v.y, 0.0)),
            Channel::Vector3(kv) => kv.value_at(time),
            Channel::Color(kv) => kv.value_at(time).map(|c| Vec3::new(c.r, c.g, c.b)),
        }
    }

    /// Samples the channel as a color at `time`, when it is a color channel.
    pub fn color_at(&self, time: f64) -> Option<Color4> {
        match self {
            Channel::Color(kv) => kv.value_at(time),
            _ => None,
        }
    }

    /// Blends the channel toward `other` by `amount` (`0..=1`), evaluated over
    /// the union of both key times, per component.
    pub fn merge(&self, other: &Channel, amount: f64) -> Channel {
        let amount = amount.clamp(0.0, 1.0);
        let axes: &[Axis] = match self {
            Channel::Scalar(_) => &[Axis::X],
            Channel::Vector2(_) => &[Axis::X, Axis::Y],
            Channel::Vector3(_) => &[Axis::X, Axis::Y, Axis::Z],
            Channel::Color(_) => &[Axis::X, Axis::Y, Axis::Z, Axis::W],
        };
        let mut result = self.clone();
        for &axis in axes {
            let (Some(mine), Some(theirs)) = (self.component(axis), other.component(axis)) else {
                continue;
            };
            result = result.with_component(axis, blend_1d(&mine, &theirs, amount));
        }
        result
    }

    /// Shifts every keyframe time by `delta` (used by timelines).
    pub fn shift(&self, delta: f64) -> Channel {
        self.map_components(|kv| shift_1d(&kv, delta))
    }

    /// Scales every keyframe time about `anchor` by `factor`.
    pub fn scale_time(&self, factor: f64, anchor: f64) -> Channel {
        self.map_components(|kv| scale_time_1d(&kv, factor, anchor))
    }

    /// The channel's natural duration (last time minus first time), or `0`.
    pub fn duration(&self) -> f64 {
        self.time_range().map(|(a, b)| b - a).unwrap_or(0.0)
    }
}
