//! The strictly 3D motion-graphics helpers.
//!
//! The shared, dimension-agnostic keyframe logic lives in
//! [`crate::animations::keyframes`]; this module re-exports it and adds the
//! constructors that only make sense in 3D (`orbit`/`spin`/`tumble`/`pathFollow`).
//! 2D authors use the same keyframe helpers with scalar or 2-vector channels.

use crate::animations::KeyframedValue;
use crate::math::Vec3;

pub use crate::animations::keyframes::{pulse, scatter, stagger, Axis, Channel, FlattenMode};

/// A circular orbit position channel (`Vec3`) sampled over `count` keyframes.
pub fn orbit(
    center: Vec3,
    radius_x: f64,
    radius_y: f64,
    start_angle: f64,
    sweep: f64,
    count: usize,
) -> Channel {
    let n = count.max(2);
    let mut out = KeyframedValue::new();
    for i in 0..n {
        let u = i as f64 / (n - 1) as f64;
        let angle = start_angle + sweep * u;
        out.add(
            u,
            Vec3::new(
                center.x + radius_x * angle.cos(),
                center.y + radius_y * angle.sin(),
                center.z,
            ),
        );
    }
    Channel::Vector3(out)
}

/// A procedural spin channel (`Vec3` Euler rotation about `axis`) over
/// `[0, duration]`.
pub fn spin(axis: Axis, turns: f64, duration: f64, count: usize) -> Channel {
    let n = count.max(2);
    let total = turns * std::f64::consts::TAU;
    let mut out = KeyframedValue::new();
    for i in 0..n {
        let u = i as f64 / (n - 1) as f64;
        let angle = total * u;
        let value = match axis {
            Axis::X => Vec3::new(angle, 0.0, 0.0),
            Axis::Y => Vec3::new(0.0, angle, 0.0),
            // A spin has no fourth axis; `W` (a color's alpha) falls back to Z.
            Axis::Z | Axis::W => Vec3::new(0.0, 0.0, angle),
        };
        out.add(duration * u, value);
    }
    Channel::Vector3(out)
}

/// A `tumble` channel: independent spins about X, Y and Z at `[x, y, z]` turns.
pub fn tumble(turns: Vec3, duration: f64, count: usize) -> Channel {
    let n = count.max(2);
    let mut out = KeyframedValue::new();
    for i in 0..n {
        let u = i as f64 / (n - 1) as f64;
        out.add(
            duration * u,
            Vec3::new(
                turns.x * std::f64::consts::TAU * u,
                turns.y * std::f64::consts::TAU * u,
                turns.z * std::f64::consts::TAU * u,
            ),
        );
    }
    Channel::Vector3(out)
}

/// A `pathFollow` channel: a `Vec3` position sampled `count` times along a
/// polyline of `points`, optionally by arc length.
pub fn path_follow(points: &[Vec3], count: usize, by_arc_length: bool) -> Channel {
    let n = count.max(2);
    let mut out = KeyframedValue::new();
    if points.len() < 2 {
        for i in 0..n {
            out.add(i as f64, points.first().copied().unwrap_or(Vec3::ZERO));
        }
        return Channel::Vector3(out);
    }
    if by_arc_length {
        let mut cumulative = vec![0.0f64];
        for pair in points.windows(2) {
            let length = (pair[1] - pair[0]).length();
            cumulative.push(cumulative.last().copied().unwrap_or(0.0) + length);
        }
        let total = *cumulative.last().unwrap_or(&0.0);
        for i in 0..n {
            let target = total * i as f64 / (n - 1) as f64;
            let mut segment = 0;
            while segment + 1 < cumulative.len() && cumulative[segment + 1] < target {
                segment += 1;
            }
            let span = cumulative[segment + 1] - cumulative[segment];
            let u = if span <= 0.0 {
                0.0
            } else {
                (target - cumulative[segment]) / span
            };
            let value = points[segment] + (points[segment + 1] - points[segment]) * u;
            out.add(i as f64, value);
        }
    } else {
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let scaled = u * (points.len() - 1) as f64;
            let segment = (scaled.floor() as usize).min(points.len() - 2);
            let local = scaled - segment as f64;
            let value = points[segment] + (points[segment + 1] - points[segment]) * local;
            out.add(i as f64, value);
        }
    }
    Channel::Vector3(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_and_spin_build_channels() {
        let orbit = orbit(Vec3::ZERO, 10.0, 10.0, 0.0, std::f64::consts::TAU, 5);
        assert_eq!(orbit.len(), 5);
        let spin = spin(Axis::Z, 1.0, 1000.0, 5);
        assert_eq!(spin.len(), 5);
    }
}
