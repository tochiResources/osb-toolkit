//! The [`Channel`] type and its structural accessors.

use super::axis::Axis;
use super::helpers::{merge_2d, merge_3d, merge_color, project_2d, project_3d, project_color};
use crate::common::animations::KeyframedValue;
use crate::math::{Color4, Vec2, Vec3};
/// A numeric keyframe channel of one, two, three or four components.
///
/// The shape is not tied to a dimension: a scalar/2-vector/3-vector covers 2D
/// and 3D motion, and a [`Color`](Channel::Color) covers color/color-hsb command
/// channels (RGBA). Every helper works on any shape, so 2D commands, 3D nodes and
/// color channels share one implementation.
#[derive(Debug, Clone, PartialEq)]
pub enum Channel {
    Scalar(KeyframedValue<f64>),
    Vector2(KeyframedValue<Vec2>),
    Vector3(KeyframedValue<Vec3>),
    /// A four-component color channel (red/green/blue/alpha).
    Color(KeyframedValue<Color4>),
}

impl Channel {
    pub fn component_count(&self) -> usize {
        match self {
            Channel::Scalar(_) => 1,
            Channel::Vector2(_) => 2,
            Channel::Vector3(_) => 3,
            Channel::Color(_) => 4,
        }
    }

    /// Whether the channel has no keyframes.
    pub fn is_empty(&self) -> bool {
        match self {
            Channel::Scalar(kv) => kv.is_empty(),
            Channel::Vector2(kv) => kv.is_empty(),
            Channel::Vector3(kv) => kv.is_empty(),
            Channel::Color(kv) => kv.is_empty(),
        }
    }

    /// The keyframe count.
    pub fn len(&self) -> usize {
        match self {
            Channel::Scalar(kv) => kv.len(),
            Channel::Vector2(kv) => kv.len(),
            Channel::Vector3(kv) => kv.len(),
            Channel::Color(kv) => kv.len(),
        }
    }

    /// The first/last keyframe times, or `None` when empty.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let times: Vec<f64> = match self {
            Channel::Scalar(kv) => kv.keyframes().iter().map(|k| k.time).collect(),
            Channel::Vector2(kv) => kv.keyframes().iter().map(|k| k.time).collect(),
            Channel::Vector3(kv) => kv.keyframes().iter().map(|k| k.time).collect(),
            Channel::Color(kv) => kv.keyframes().iter().map(|k| k.time).collect(),
        };
        match (times.first(), times.last()) {
            (Some(first), Some(last)) => Some((*first, *last)),
            _ => None,
        }
    }

    /// The first component on `axis`, or `None` when the channel has no such
    /// component (a scalar only has `X`, a 2-vector has `X`/`Y`, a color has
    /// `X`/`Y`/`Z`/`W`).
    pub fn component(&self, axis: Axis) -> Option<KeyframedValue<f64>> {
        match self {
            Channel::Scalar(kv) => match axis {
                Axis::X => Some(kv.clone()),
                _ => None,
            },
            Channel::Vector2(kv) => match axis {
                Axis::X => Some(project_2d(kv, |v| v.x)),
                Axis::Y => Some(project_2d(kv, |v| v.y)),
                Axis::Z | Axis::W => None,
            },
            Channel::Vector3(kv) => match axis {
                Axis::X => Some(project_3d(kv, |v| v.x)),
                Axis::Y => Some(project_3d(kv, |v| v.y)),
                Axis::Z => Some(project_3d(kv, |v| v.z)),
                Axis::W => None,
            },
            Channel::Color(kv) => match axis {
                Axis::X => Some(project_color(kv, |c| c.r)),
                Axis::Y => Some(project_color(kv, |c| c.g)),
                Axis::Z => Some(project_color(kv, |c| c.b)),
                Axis::W => Some(project_color(kv, |c| c.a)),
            },
        }
    }

    /// Renders the channel with `axis`'s samples replaced by `values`, keeping
    /// the channel's shape. Missing components are left untouched.
    pub fn with_component(&self, axis: Axis, values: KeyframedValue<f64>) -> Channel {
        match self {
            Channel::Scalar(_) => Channel::Scalar(values),
            Channel::Vector2(kv) => {
                let mut rebuilt = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    let t = keyframe.time;
                    let base = keyframe.value;
                    let v = values.value_at(t).unwrap_or(0.0);
                    let value = match axis {
                        Axis::X => Vec2::new(v, base.y),
                        Axis::Y => Vec2::new(base.x, v),
                        Axis::Z | Axis::W => base,
                    };
                    rebuilt.add_eased(t, value, keyframe.easing);
                }
                Channel::Vector2(rebuilt)
            }
            Channel::Vector3(kv) => {
                let mut rebuilt = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    let t = keyframe.time;
                    let base = keyframe.value;
                    let v = values.value_at(t).unwrap_or(0.0);
                    let value = match axis {
                        Axis::X => Vec3::new(v, base.y, base.z),
                        Axis::Y => Vec3::new(base.x, v, base.z),
                        Axis::Z => Vec3::new(base.x, base.y, v),
                        Axis::W => base,
                    };
                    rebuilt.add_eased(t, value, keyframe.easing);
                }
                Channel::Vector3(rebuilt)
            }
            Channel::Color(kv) => {
                let mut rebuilt = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    let t = keyframe.time;
                    let base = keyframe.value;
                    let v = values.value_at(t).unwrap_or(0.0);
                    let value = match axis {
                        Axis::X => Color4::new(v, base.g, base.b, base.a),
                        Axis::Y => Color4::new(base.r, v, base.b, base.a),
                        Axis::Z => Color4::new(base.r, base.g, v, base.a),
                        Axis::W => Color4::new(base.r, base.g, base.b, v),
                    };
                    rebuilt.add_eased(t, value, keyframe.easing);
                }
                Channel::Color(rebuilt)
            }
        }
    }

    /// Applies `f` to each component independently.
    pub(super) fn map_components(
        &self,
        f: impl Fn(KeyframedValue<f64>) -> KeyframedValue<f64>,
    ) -> Channel {
        match self {
            Channel::Scalar(kv) => Channel::Scalar(f(kv.clone())),
            Channel::Vector2(kv) => {
                let x = f(project_2d(kv, |v| v.x));
                let y = f(project_2d(kv, |v| v.y));
                Channel::Vector2(merge_2d(&x, &y))
            }
            Channel::Vector3(kv) => {
                let x = f(project_3d(kv, |v| v.x));
                let y = f(project_3d(kv, |v| v.y));
                let z = f(project_3d(kv, |v| v.z));
                Channel::Vector3(merge_3d(&x, &y, &z))
            }
            Channel::Color(kv) => {
                let r = f(project_color(kv, |c| c.r));
                let g = f(project_color(kv, |c| c.g));
                let b = f(project_color(kv, |c| c.b));
                let a = f(project_color(kv, |c| c.a));
                Channel::Color(merge_color(&r, &g, &b, &a))
            }
        }
    }

    /// Applies `f` to each keyframe of a color channel. Other channel shapes are
    /// returned unchanged, so a color-space transform is a no-op on non-colors.
    pub fn map_color(&self, mut f: impl FnMut(Color4) -> Color4) -> Channel {
        match self {
            Channel::Color(kv) => {
                let mut out = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    out.add_eased(keyframe.time, f(keyframe.value), keyframe.easing);
                }
                Channel::Color(out)
            }
            other => other.clone(),
        }
    }

    /// Whether the channel holds colors.
    pub fn is_color(&self) -> bool {
        matches!(self, Channel::Color(_))
    }

    /// The channel's keyframes as colors, when it is a color channel.
    pub fn colors(&self) -> Option<Vec<(f64, Color4)>> {
        match self {
            Channel::Color(kv) => Some(
                kv.keyframes()
                    .iter()
                    .map(|key| (key.time, key.value))
                    .collect(),
            ),
            _ => None,
        }
    }
}
