use super::*;
use crate::common::animations::{Easing, KeyframedValue};
use crate::math::{Color4, Vec2, Vec3};
fn scalar(keys: &[(f64, f64)]) -> Channel {
    let mut kv = KeyframedValue::new();
    for (time, value) in keys {
        kv.add(*time, *value);
    }
    Channel::Scalar(kv)
}

fn vec3(keys: &[(f64, [f64; 3])]) -> Channel {
    let mut kv = KeyframedValue::new();
    for (time, value) in keys {
        kv.add(*time, Vec3::new(value[0], value[1], value[2]));
    }
    Channel::Vector3(kv)
}

fn color(keys: &[(f64, [f64; 4])]) -> Channel {
    let mut kv = KeyframedValue::new();
    for (time, value) in keys {
        kv.add(*time, Color4::new(value[0], value[1], value[2], value[3]));
    }
    Channel::Color(kv)
}

fn values(channel: &Channel) -> Vec<f64> {
    let keyframes: Vec<(f64, f64)> = match channel {
        Channel::Scalar(kv) => kv.keyframes().iter().map(|k| (k.time, k.value)).collect(),
        Channel::Vector2(kv) => kv.keyframes().iter().map(|k| (k.time, k.value.x)).collect(),
        Channel::Vector3(kv) => kv.keyframes().iter().map(|k| (k.time, k.value.x)).collect(),
        Channel::Color(kv) => kv.keyframes().iter().map(|k| (k.time, k.value.r)).collect(),
    };
    keyframes.into_iter().map(|(_, v)| v).collect()
}

#[test]
fn flatten_reduces_each_axis_to_one_keyframe() {
    let channel = vec3(&[(0.0, [0.0, 10.0, 0.0]), (100.0, [5.0, 20.0, 0.0])]);
    let flattened = channel.flatten(Some(Axis::Y), FlattenMode::Mean);
    // The Y axis is now constant at its mean; the other axes keep their keys.
    assert_eq!(flattened.len(), 2);
    let y = flattened.component(Axis::Y).unwrap();
    assert_eq!(y.value_at(0.0), Some(15.0));
    assert_eq!(y.value_at(100.0), Some(15.0));
    assert_eq!(
        flattened.component(Axis::X).unwrap().value_at(100.0),
        Some(5.0)
    );
    // Flattening every axis collapses to a single keyframe per axis.
    let all = channel.flatten(None, FlattenMode::Last);
    assert_eq!(all.len(), 1);
    assert_eq!(all.component(Axis::Y).unwrap().value_at(0.0), Some(20.0));
}

#[test]
fn scale_and_offset_are_component_wise() {
    let channel = vec3(&[(0.0, [1.0, 2.0, 3.0]), (10.0, [2.0, 4.0, 6.0])]);
    let scaled = channel.scale(2.0).offset(1.0);
    assert_eq!(
        scaled.component(Axis::Z).unwrap().value_at(10.0),
        Some(13.0)
    );
}

#[test]
fn mirror_reflects_about_a_pivot() {
    let channel = scalar(&[(0.0, 10.0), (10.0, 20.0)]);
    let mirrored = channel.mirror(None, 100.0);
    assert_eq!(values(&mirrored), vec![190.0, 180.0]);
}

#[test]
fn reverse_maps_times_inside_the_span() {
    let channel = scalar(&[(0.0, 0.0), (30.0, 30.0), (100.0, 100.0)]);
    let reversed = channel.reverse(0.0, 100.0);
    let mut kv = KeyframedValue::new();
    if let Channel::Scalar(inner) = &reversed {
        for keyframe in inner.keyframes() {
            kv.add(keyframe.time, keyframe.value);
        }
    }
    assert_eq!(kv.value_at(70.0), Some(30.0));
    assert_eq!(kv.value_at(0.0), Some(100.0));
}

#[test]
fn repeat_ping_pongs() {
    let channel = scalar(&[(0.0, 0.0), (10.0, 10.0)]);
    let repeated = channel.repeat(3, true, 0.0, 10.0);
    assert_eq!(repeated.len(), 4); // shared boundaries collapse the tile
    let inner = match &repeated {
        Channel::Scalar(kv) => kv,
        _ => unreachable!(),
    };
    assert_eq!(inner.value_at(15.0), Some(5.0)); // second copy reversed
}

#[test]
fn randomize_is_deterministic_for_a_seed() {
    let channel = scalar(&[(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)]);
    let a = channel.randomize(None, -1.0, 1.0, 42, 0.0);
    let b = channel.randomize(None, -1.0, 1.0, 42, 0.0);
    assert_eq!(a, b);
    let c = channel.randomize(None, -1.0, 1.0, 43, 0.0);
    assert_ne!(a, c);
}

#[test]
fn resample_produces_a_regular_step() {
    let channel = scalar(&[(0.0, 0.0), (100.0, 100.0)]);
    let resampled = channel.resample(25.0, 0);
    assert_eq!(resampled.len(), 5);
}

#[test]
fn align_snaps_to_the_grid() {
    let channel = scalar(&[(10.0, 1.0), (130.0, 2.0)]);
    let aligned = channel.align(100.0, 0.0, 2);
    let inner = match &aligned {
        Channel::Scalar(kv) => kv,
        _ => unreachable!(),
    };
    let times: Vec<f64> = inner.keyframes().iter().map(|k| k.time).collect();
    assert_eq!(times, vec![0.0, 150.0]);
}

#[test]
fn pulse_builds_a_channel() {
    let pulse = pulse(1.0, 0.5, 1.0, 0.0, 100.0, 1000.0);
    assert!(pulse.len() >= 2);
}

#[test]
fn scatter_is_seeded() {
    let a = scatter(7, 4, Vec3::ZERO, Vec3::ONE, 0.0);
    let b = scatter(7, 4, Vec3::ZERO, Vec3::ONE, 0.0);
    assert_eq!(a, b);
    assert_eq!(a.len(), 4);
}

#[test]
fn keyframe_easing_shapes_interpolation_and_survives_transforms() {
    let mut kv = KeyframedValue::new();
    kv.add_eased(0.0, 0.0f64, Easing::InQuad);
    kv.add(100.0, 1.0f64);
    // InQuad(0.5) = 0.25.
    assert!((kv.value_at(50.0).unwrap() - 0.25).abs() < 1e-6);
    assert_eq!(kv.easing_at(50.0), Easing::InQuad);

    // A value transform keeps the segment's easing.
    let channel = Channel::Scalar(kv);
    let scaled = channel.scale(2.0);
    let inner = match &scaled {
        Channel::Scalar(kv) => kv,
        _ => unreachable!(),
    };
    assert_eq!(inner.easing_at(50.0), Easing::InQuad);
    assert!((inner.value_at(50.0).unwrap() - 0.5).abs() < 1e-6);

    // A vector projection carries the easing through.
    let mut v2 = KeyframedValue::new();
    v2.add_eased(0.0, Vec2::new(0.0, 0.0), Easing::OutQuad);
    v2.add(100.0, Vec2::new(1.0, 2.0));
    let projected = Channel::Vector2(v2).component(Axis::X).unwrap();
    assert_eq!(projected.easing_at(50.0), Easing::OutQuad);
}

#[test]
fn color_channels_use_the_same_helpers() {
    let channel = color(&[(0.0, [0.0, 0.5, 1.0, 1.0]), (100.0, [1.0, 0.5, 0.0, 0.5])]);
    assert_eq!(channel.component_count(), 4);

    // Scale/offset act per component (RGBA).
    let scaled = channel.scale(2.0);
    let rgba = match &scaled {
        Channel::Color(kv) => kv.value_at(0.0).unwrap(),
        _ => unreachable!(),
    };
    assert_eq!(rgba, Color4::new(0.0, 1.0, 2.0, 2.0));

    // Alpha is the W axis.
    let alpha = channel.component(Axis::W).unwrap();
    assert_eq!(alpha.value_at(0.0), Some(1.0));
    assert_eq!(alpha.value_at(100.0), Some(0.5));

    // Flattening only the red axis keeps the others keyframed.
    let flattened = channel.flatten(Some(Axis::X), FlattenMode::Last);
    let red = flattened.component(Axis::X).unwrap();
    assert_eq!(red.value_at(0.0), Some(1.0));
    assert_eq!(red.value_at(100.0), Some(1.0));
    assert_eq!(
        flattened.component(Axis::Y).unwrap().value_at(100.0),
        Some(0.5)
    );

    // Mirroring every component about 0.5 inverts the color.
    let mirrored = channel.mirror(None, 0.5);
    let first = match &mirrored {
        Channel::Color(kv) => kv.value_at(0.0).unwrap(),
        _ => unreachable!(),
    };
    assert!((first.r - 1.0).abs() < 1e-9);
    assert!((first.b - 0.0).abs() < 1e-9);
}
