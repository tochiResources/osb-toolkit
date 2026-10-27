//! Small channel/scene constructors (`stagger`, `pulse`, `scatter`).

use super::channel::Channel;
use crate::common::animations::KeyframedValue;
use crate::common::Rng;
use crate::math::Vec3;
/// A stagger delay for element `index`: `index * step` ms.
pub fn stagger(index: usize, step: f64) -> f64 {
    index as f64 * step
}

/// A `pulse` oscillation channel (`Scalar`): `base + amplitude * sin(2*pi*f*t + phase)`.
pub fn pulse(
    base: f64,
    amplitude: f64,
    frequency: f64,
    phase: f64,
    step: f64,
    duration: f64,
) -> Channel {
    let step = if step > 0.0 { step } else { duration / 16.0 };
    let mut out = KeyframedValue::new();
    if step <= 0.0 {
        out.add(0.0, base + amplitude * phase.sin());
        return Channel::Scalar(out);
    }
    let mut t = 0.0;
    while t <= duration + 1e-9 {
        let value = base + amplitude * (std::f64::consts::TAU * frequency * t + phase).sin();
        out.add(t, value);
        t += step;
    }
    out.add(
        duration,
        base + amplitude * (std::f64::consts::TAU * frequency * duration + phase).sin(),
    );
    Channel::Scalar(out)
}

/// Seeded random placements (`Scatter`/`explode`): `count` points in the box
/// `[min, max]`, optionally biased radially outward by `spread`.
pub fn scatter(seed: u64, count: usize, min: Vec3, max: Vec3, spread: f64) -> Vec<Vec3> {
    let mut rng = Rng::new(seed);
    let center = (min + max) * 0.5;
    (0..count)
        .map(|_| {
            let unit = Vec3::new(
                rng.range(min.x, max.x),
                rng.range(min.y, max.y),
                rng.range(min.z, max.z),
            );
            if spread == 0.0 {
                unit
            } else {
                // Push each point away from the center by `spread`.
                (unit - center) * (1.0 + spread) + center
            }
        })
        .collect()
}
