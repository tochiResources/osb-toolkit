//! Component-wise helpers shared by the channel modules.

use crate::common::animations::KeyframedValue;
use crate::math::{Color4, Vec2, Vec3};
pub(super) fn project_2d(
    kv: &KeyframedValue<Vec2>,
    f: impl Fn(Vec2) -> f64,
) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(keyframe.time, f(keyframe.value), keyframe.easing);
    }
    out
}

pub(super) fn project_3d(
    kv: &KeyframedValue<Vec3>,
    f: impl Fn(Vec3) -> f64,
) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(keyframe.time, f(keyframe.value), keyframe.easing);
    }
    out
}

pub(super) fn project_color(
    kv: &KeyframedValue<Color4>,
    f: impl Fn(Color4) -> f64,
) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(keyframe.time, f(keyframe.value), keyframe.easing);
    }
    out
}

pub(super) fn merge_2d(x: &KeyframedValue<f64>, y: &KeyframedValue<f64>) -> KeyframedValue<Vec2> {
    let mut out = KeyframedValue::new();
    for keyframe in x.keyframes() {
        out.add_eased(
            keyframe.time,
            Vec2::new(keyframe.value, y.value_at(keyframe.time).unwrap_or(0.0)),
            keyframe.easing,
        );
    }
    out
}

pub(super) fn merge_3d(
    x: &KeyframedValue<f64>,
    y: &KeyframedValue<f64>,
    z: &KeyframedValue<f64>,
) -> KeyframedValue<Vec3> {
    let mut out = KeyframedValue::new();
    for keyframe in x.keyframes() {
        out.add_eased(
            keyframe.time,
            Vec3::new(
                keyframe.value,
                y.value_at(keyframe.time).unwrap_or(0.0),
                z.value_at(keyframe.time).unwrap_or(0.0),
            ),
            keyframe.easing,
        );
    }
    out
}

pub(super) fn merge_color(
    r: &KeyframedValue<f64>,
    g: &KeyframedValue<f64>,
    b: &KeyframedValue<f64>,
    a: &KeyframedValue<f64>,
) -> KeyframedValue<Color4> {
    let mut out = KeyframedValue::new();
    for keyframe in r.keyframes() {
        let time = keyframe.time;
        out.add_eased(
            time,
            Color4::new(
                keyframe.value,
                g.value_at(time).unwrap_or(0.0),
                b.value_at(time).unwrap_or(0.0),
                a.value_at(time).unwrap_or(1.0),
            ),
            keyframe.easing,
        );
    }
    out
}

pub(super) fn scale_1d(kv: &KeyframedValue<f64>, factor: f64) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(keyframe.time, keyframe.value * factor, keyframe.easing);
    }
    out
}

pub(super) fn offset_1d(
    kv: &KeyframedValue<f64>,
    delta: impl Fn(f64) -> f64,
) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(
            keyframe.time,
            keyframe.value + delta(keyframe.time),
            keyframe.easing,
        );
    }
    out
}

pub(super) fn blend_1d(
    a: &KeyframedValue<f64>,
    b: &KeyframedValue<f64>,
    amount: f64,
) -> KeyframedValue<f64> {
    let mut times: Vec<f64> = a.keyframes().iter().map(|k| k.time).collect();
    for keyframe in b.keyframes() {
        if !times.iter().any(|t| (*t - keyframe.time).abs() < 1e-9) {
            times.push(keyframe.time);
        }
    }
    times.sort_by(|x, y| x.total_cmp(y));
    let mut out = KeyframedValue::new();
    for t in times {
        let av = a.value_at(t).unwrap_or(0.0);
        let bv = b.value_at(t).unwrap_or(av);
        out.add_eased(t, av + (bv - av) * amount, a.easing_at(t));
    }
    out
}

pub(super) fn time_range_1d(kv: &KeyframedValue<f64>) -> Option<(f64, f64)> {
    match (kv.keyframes().first(), kv.keyframes().last()) {
        (Some(first), Some(last)) => Some((first.time, last.time)),
        _ => None,
    }
}

pub(super) fn shift_1d(kv: &KeyframedValue<f64>, delta: f64) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(keyframe.time + delta, keyframe.value, keyframe.easing);
    }
    out
}

pub(super) fn scale_time_1d(
    kv: &KeyframedValue<f64>,
    factor: f64,
    anchor: f64,
) -> KeyframedValue<f64> {
    let mut out = KeyframedValue::new();
    for keyframe in kv.keyframes() {
        out.add_eased(
            anchor + (keyframe.time - anchor) * factor,
            keyframe.value,
            keyframe.easing,
        );
    }
    out
}
