use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Discriminants match osu!'s `.osb` easing ids so command records round-trip
// without a lookup table.
/// The 35 osu! storyboard easing functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, ToSchema)]
#[repr(u8)]
pub enum Easing {
    #[default]
    None = 0,
    Out = 1,
    In = 2,
    InQuad = 3,
    OutQuad = 4,
    InOutQuad = 5,
    InCubic = 6,
    OutCubic = 7,
    InOutCubic = 8,
    InQuart = 9,
    OutQuart = 10,
    InOutQuart = 11,
    InQuint = 12,
    OutQuint = 13,
    InOutQuint = 14,
    InSine = 15,
    OutSine = 16,
    InOutSine = 17,
    InExpo = 18,
    OutExpo = 19,
    InOutExpo = 20,
    InCirc = 21,
    OutCirc = 22,
    InOutCirc = 23,
    InElastic = 24,
    OutElastic = 25,
    OutElasticHalf = 26,
    OutElasticQuarter = 27,
    InOutElastic = 28,
    InBack = 29,
    OutBack = 30,
    InOutBack = 31,
    InBounce = 32,
    OutBounce = 33,
    InOutBounce = 34,
}

const PI: f32 = std::f32::consts::PI;

impl Easing {
    /// Applies the easing to a normalized `t` in `0.0..=1.0`.
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::None => t,
            Easing::Out => (t * PI / 2.0).sin(),
            Easing::In => 1.0 - (t * PI / 2.0).cos(),
            Easing::InQuad => t * t,
            Easing::OutQuad => t * (2.0 - t),
            Easing::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::InCubic => t * t * t,
            Easing::OutCubic => {
                let u = t - 1.0;
                u * u * u + 1.0
            }
            Easing::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let u = 2.0 * t - 2.0;
                    (t - 1.0) * u * u + 1.0
                }
            }
            Easing::InQuart => t * t * t * t,
            Easing::OutQuart => {
                let u = t - 1.0;
                1.0 - u * u * u * u
            }
            Easing::InOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let u = t - 1.0;
                    1.0 - 8.0 * u * u * u * u
                }
            }
            Easing::InQuint => t * t * t * t * t,
            Easing::OutQuint => {
                let u = t - 1.0;
                u * u * u * u * u + 1.0
            }
            Easing::InOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let u = t - 1.0;
                    1.0 + 16.0 * u * u * u * u * u
                }
            }
            Easing::InSine => 1.0 - (t * PI / 2.0).cos(),
            Easing::OutSine => (t * PI / 2.0).sin(),
            Easing::InOutSine => 0.5 - 0.5 * (PI * t).cos(),
            Easing::InExpo => 2.0f32.powf(10.0 * (t - 1.0)),
            Easing::OutExpo => 1.0 - 2.0f32.powf(-10.0 * t),
            Easing::InOutExpo => {
                if t < 0.5 {
                    2.0f32.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0f32.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            Easing::InCirc => 1.0 - (1.0 - t * t).sqrt(),
            Easing::OutCirc => ((2.0 - t) * t).sqrt(),
            Easing::InOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
                } else {
                    ((-(2.0 * t - 3.0) * (2.0 * t - 1.0)).sqrt() + 1.0) / 2.0
                }
            }
            Easing::InElastic => {
                -2.0f32.powf(10.0 * (t - 1.0)) * ((t - 1.075) * 2.0 * PI / 0.3).sin()
            }
            Easing::OutElastic => {
                2.0f32.powf(-10.0 * t) * ((t - 0.075) * 2.0 * PI / 0.3).sin() + 1.0
            }
            Easing::OutElasticHalf => {
                2.0f32.powf(-10.0 * t) * ((0.5 * t - 0.075) * 2.0 * PI / 0.3).sin() + 1.0
            }
            Easing::OutElasticQuarter => {
                2.0f32.powf(-10.0 * t) * ((0.25 * t - 0.075) * 2.0 * PI / 0.3).sin() + 1.0
            }
            Easing::InOutElastic => {
                if t < 0.5 {
                    -(2.0f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * 2.0 * PI / 4.5).sin())
                        / 2.0
                } else {
                    (2.0f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * 2.0 * PI / 4.5).sin())
                        / 2.0
                        + 1.0
                }
            }
            Easing::InBack => {
                const S: f32 = 1.70158;
                t * t * ((S + 1.0) * t - S)
            }
            Easing::OutBack => {
                const S: f32 = 1.70158;
                let u = t - 1.0;
                u * u * ((S + 1.0) * u + S) + 1.0
            }
            Easing::InOutBack => {
                const S: f32 = 1.70158 * 1.525;
                if t < 0.5 {
                    let u = 2.0 * t;
                    (u * u * ((S + 1.0) * u - S)) / 2.0
                } else {
                    let u = 2.0 * t - 2.0;
                    (u * u * ((S + 1.0) * u + S) + 2.0) / 2.0
                }
            }
            Easing::InBounce => 1.0 - out_bounce(1.0 - t),
            Easing::OutBounce => out_bounce(t),
            Easing::InOutBounce => {
                if t < 0.5 {
                    (1.0 - out_bounce(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + out_bounce(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }

    pub fn from_id(id: u8) -> Option<Easing> {
        if id <= 34 {
            Some(unsafe { std::mem::transmute::<u8, Easing>(id) })
        } else {
            None
        }
    }

    /// The osu! `.osb` easing id (the discriminant).
    pub fn id(self) -> u8 {
        self as u8
    }
}

/// Evaluates a CSS-style cubic-bezier easing with control points
/// `(x1, y1)`, `(x2, y2)` at a normalized time `t`.
///
/// The curve runs from `(0, 0)` to `(1, 1)`; the x axis is solved for the
/// parameter `u` with Newton's method, then the y axis is evaluated there.
pub fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let mut u = t;
    for _ in 0..12 {
        let error = bezier_axis(x1, x2, u) - t;
        let slope = bezier_axis_derivative(x1, x2, u);
        if slope.abs() < 1e-7 {
            break;
        }
        let next = (u - error / slope).clamp(0.0, 1.0);
        if (next - u).abs() < 1e-9 {
            u = next;
            break;
        }
        u = next;
    }
    bezier_axis(y1, y2, u)
}

fn bezier_axis(a: f64, b: f64, u: f64) -> f64 {
    let v = 1.0 - u;
    3.0 * v * v * u * a + 3.0 * v * u * u * b + u * u * u
}

fn bezier_axis_derivative(a: f64, b: f64, u: f64) -> f64 {
    let v = 1.0 - u;
    3.0 * v * v * a + 6.0 * v * u * (b - a) + 3.0 * u * u * (1.0 - b)
}

fn out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_are_fixed() {
        for id in 0..=34u8 {
            let e = Easing::from_id(id).unwrap();
            assert!((e.apply(0.0) - 0.0).abs() < 0.02, "{e:?} at 0");
            assert!((e.apply(1.0) - 1.0).abs() < 0.02, "{e:?} at 1");
        }
    }

    #[test]
    fn linear_and_quad() {
        assert_eq!(Easing::None.apply(0.25), 0.25);
        assert!((Easing::InQuad.apply(0.5) - 0.25).abs() < 1e-6);
        assert!((Easing::OutQuad.apply(0.5) - 0.75).abs() < 1e-6);
    }
}
