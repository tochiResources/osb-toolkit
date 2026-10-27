//! The value and time transforms over a [`Channel`].

use super::axis::{Axis, FlattenMode};
use super::channel::Channel;
use super::helpers::{offset_1d, scale_1d, time_range_1d};
use crate::common::animations::{Easing, Keyframe, KeyframedValue};
use crate::common::Rng;
use crate::math::{Vec2, Vec3};

impl Channel {
    /// Collapses an axis (or, with `None`, every axis) to a constant.
    pub fn flatten(&self, axis: Option<Axis>, mode: FlattenMode) -> Channel {
        let reduce = |kv: KeyframedValue<f64>| {
            let times: Vec<f64> = kv.keyframes().iter().map(|k| k.time).collect();
            let constant = match mode {
                FlattenMode::Mean => {
                    if times.is_empty() {
                        0.0
                    } else {
                        let sum: f64 = times.iter().map(|t| kv.value_at(*t).unwrap_or(0.0)).sum();
                        sum / times.len() as f64
                    }
                }
                FlattenMode::First => kv.keyframes().first().map(|k| k.value).unwrap_or(0.0),
                FlattenMode::Last => kv.keyframes().last().map(|k| k.value).unwrap_or(0.0),
                FlattenMode::Value(value) => value,
            };
            let anchor = times.first().copied().unwrap_or(0.0);
            let mut out = KeyframedValue::new();
            out.add(anchor, constant);
            out
        };
        match axis {
            Some(axis) => {
                if let Some(component) = self.component(axis) {
                    self.with_component(axis, reduce(component))
                } else {
                    self.clone()
                }
            }
            None => self.map_components(reduce),
        }
    }

    /// Multiplies every component by `factor`.
    pub fn scale(&self, factor: f64) -> Channel {
        self.map_components(|kv| scale_1d(&kv, factor))
    }

    /// Adds a scalar `delta` to every component.
    pub fn offset(&self, delta: f64) -> Channel {
        self.map_components(|kv| offset_1d(&kv, |_| delta))
    }

    /// Adds a time-varying `delta` channel (sampled at the receiver's times).
    pub fn offset_by(&self, delta: &KeyframedValue<f64>) -> Channel {
        self.map_components(|kv| offset_1d(&kv, |t| delta.value_at(t).unwrap_or(0.0)))
    }

    /// Reflects an axis about `pivot` (`v -> 2*pivot - v`).
    pub fn mirror(&self, axis: Option<Axis>, pivot: f64) -> Channel {
        let reflect = |kv: KeyframedValue<f64>| {
            let mut out = KeyframedValue::new();
            for keyframe in kv.keyframes() {
                out.add_eased(keyframe.time, 2.0 * pivot - keyframe.value, keyframe.easing);
            }
            out
        };
        match axis {
            Some(axis) => {
                if let Some(component) = self.component(axis) {
                    self.with_component(axis, reflect(component))
                } else {
                    self.clone()
                }
            }
            None => self.map_components(reflect),
        }
    }

    /// Jitters an axis (or every axis with `None`) with a seeded range, keeping
    /// keyframe times. When `step > 0` the jitter is quantized to that step.
    pub fn randomize(
        &self,
        axis: Option<Axis>,
        min: f64,
        max: f64,
        seed: u64,
        step: f64,
    ) -> Channel {
        let jitter = |kv: KeyframedValue<f64>| {
            let mut rng = Rng::new(seed);
            let mut out = KeyframedValue::new();
            for keyframe in kv.keyframes() {
                let mut delta = rng.range(min, max);
                if step > 0.0 {
                    delta = (delta / step).round() * step;
                }
                out.add_eased(keyframe.time, keyframe.value + delta, keyframe.easing);
            }
            out
        };
        match axis {
            Some(axis) => {
                if let Some(component) = self.component(axis) {
                    self.with_component(axis, jitter(component))
                } else {
                    self.clone()
                }
            }
            None => self.map_components(jitter),
        }
    }

    /// Reverses the channel in time about the `[start, end]` span; keys outside
    /// the span keep their time.
    pub fn reverse(&self, start: f64, end: f64) -> Channel {
        self.map_components(|kv| {
            let mut out = KeyframedValue::new();
            for keyframe in kv.keyframes() {
                let time = if keyframe.time >= start && keyframe.time <= end {
                    start + end - keyframe.time
                } else {
                    keyframe.time
                };
                out.add_eased(time, keyframe.value, keyframe.easing);
            }
            out
        })
    }

    /// Tiles the `[start, end]` span `count` times. With `ping_pong`, alternate
    /// copies play backwards.
    pub fn repeat(&self, count: usize, ping_pong: bool, start: f64, end: f64) -> Channel {
        let span = end - start;
        if count <= 1 || span <= 0.0 {
            return self.clone();
        }
        self.map_components(|kv| {
            let segment: Vec<Keyframe<f64>> = kv
                .keyframes()
                .iter()
                .filter(|k| k.time >= start && k.time <= end)
                .cloned()
                .collect();
            if segment.is_empty() {
                return kv.clone();
            }
            let mut out = KeyframedValue::new();
            // Keep keys outside the tiled span as-is.
            for keyframe in kv.keyframes() {
                if keyframe.time < start || keyframe.time > end {
                    out.add_eased(keyframe.time, keyframe.value, keyframe.easing);
                }
            }
            for copy in 0..count {
                let reverse = ping_pong && copy % 2 == 1;
                for keyframe in &segment {
                    let local = keyframe.time - start;
                    let local = if reverse { span - local } else { local };
                    out.add_eased(
                        start + copy as f64 * span + local,
                        keyframe.value,
                        keyframe.easing,
                    );
                }
            }
            out
        })
    }

    /// Resamples the channel to a fixed time step over its range (or to exactly
    /// `count` samples when `step <= 0`).
    pub fn resample(&self, step: f64, count: usize) -> Channel {
        self.map_components(|kv| {
            let Some((first, last)) = time_range_1d(&kv) else {
                return kv.clone();
            };
            if last <= first {
                return kv.clone();
            }
            let mut out = KeyframedValue::new();
            if step > 0.0 {
                let mut t = first;
                while t <= last + 1e-9 {
                    out.add(t, kv.value_at(t).unwrap_or(0.0));
                    t += step;
                }
                // Always include the final key.
                out.add(last, kv.value_at(last).unwrap_or(0.0));
            } else {
                let n = count.max(2);
                for i in 0..n {
                    let t = first + (last - first) * i as f64 / (n - 1) as f64;
                    out.add(t, kv.value_at(t).unwrap_or(0.0));
                }
            }
            out
        })
    }

    /// A moving-average smoothing over a window of neighbouring keyframes.
    pub fn smooth(&self, window: usize) -> Channel {
        self.map_components(|kv| {
            let keys = kv.keyframes();
            if window < 2 || keys.len() < 3 {
                return kv.clone();
            }
            let radius = window / 2;
            let mut out = KeyframedValue::new();
            for (i, keyframe) in keys.iter().enumerate() {
                let lo = i.saturating_sub(radius);
                let hi = (i + radius).min(keys.len() - 1);
                let slice = &keys[lo..=hi];
                let sum: f64 = slice.iter().map(|k| k.value).sum();
                out.add(keyframe.time, sum / slice.len() as f64);
            }
            out
        })
    }

    /// Douglas-Peucker simplification of each component (delegates to the
    /// canonical `KeyframedValue::simplify_1d`).
    pub fn simplify(&self, tolerance: f64, decimals: u32) -> Channel {
        self.map_components(|mut kv| {
            kv.simplify_1d(tolerance, decimals);
            kv
        })
    }

    /// Snaps keyframe times to a beat grid: `beat` ms per beat divided into
    /// `subdivision` steps, offset by `offset` ms.
    pub fn align(&self, beat: f64, offset: f64, subdivision: u32) -> Channel {
        let subdivision = subdivision.max(1);
        let step = beat / subdivision as f64;
        if step <= 0.0 {
            return self.clone();
        }
        self.map_components(|kv| {
            let mut out = KeyframedValue::new();
            for keyframe in kv.keyframes() {
                let snapped = offset + ((keyframe.time - offset) / step).round() * step;
                out.add_eased(snapped, keyframe.value, keyframe.easing);
            }
            out
        })
    }

    /// Re-eases each span by inserting `samples` intermediate keyframes so the
    /// resulting linear channel follows `easing` (named or cubic-bezier via
    /// [`Easing`]).
    pub fn ease(&self, easing: Easing, samples: usize) -> Channel {
        let samples = samples.max(1);
        self.map_components(|kv| {
            let keys = kv.keyframes();
            if keys.len() < 2 {
                return kv.clone();
            }
            let mut out = KeyframedValue::new();
            for pair in keys.windows(2) {
                let (a, b) = (&pair[0], &pair[1]);
                let span = b.time - a.time;
                if span <= 0.0 {
                    out.add(a.time, a.value);
                    continue;
                }
                for i in 0..samples {
                    let u = i as f32 / samples as f32;
                    let eased = easing.apply(u) as f64;
                    out.add(
                        a.time + span * u as f64,
                        a.value + (b.value - a.value) * eased,
                    );
                }
            }
            if let Some(last) = keys.last() {
                out.add(last.time, last.value);
            }
            out
        })
    }

    /// Re-eases each span following a cubic-bezier curve (like [`ease`], but the
    /// easing is a `(x1, y1, x2, y2)` control-point set).
    pub fn ease_bezier(&self, x1: f64, y1: f64, x2: f64, y2: f64, samples: usize) -> Channel {
        let samples = samples.max(1);
        self.map_components(|kv| {
            let keys = kv.keyframes();
            if keys.len() < 2 {
                return kv.clone();
            }
            let mut out = KeyframedValue::new();
            for pair in keys.windows(2) {
                let (a, b) = (&pair[0], &pair[1]);
                let span = b.time - a.time;
                if span <= 0.0 {
                    out.add(a.time, a.value);
                    continue;
                }
                for i in 0..samples {
                    let u = i as f64 / samples as f64;
                    let eased = crate::common::animations::cubic_bezier(x1, y1, x2, y2, u);
                    out.add(a.time + span * u, a.value + (b.value - a.value) * eased);
                }
            }
            if let Some(last) = keys.last() {
                out.add(last.time, last.value);
            }
            out
        })
    }

    /// Adds a 2D `delta` to the first two components (a scalar channel adds
    /// `delta.x`; colors are left untouched).
    pub fn translate(&self, delta: Vec2) -> Channel {
        match self {
            Channel::Scalar(kv) => Channel::Scalar(offset_1d(kv, |_| delta.x)),
            Channel::Vector2(kv) => {
                let mut out = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    out.add(keyframe.time, keyframe.value + delta);
                }
                Channel::Vector2(out)
            }
            Channel::Vector3(kv) => {
                let mut out = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    out.add(
                        keyframe.time,
                        Vec3::new(
                            keyframe.value.x + delta.x,
                            keyframe.value.y + delta.y,
                            keyframe.value.z,
                        ),
                    );
                }
                Channel::Vector3(out)
            }
            Channel::Color(_) => self.clone(),
        }
    }

    /// Multiplies the first two components by a 2D `factor` (a scalar channel
    /// uses `factor.x`; colors are left untouched).
    pub fn scale_xy(&self, factor: Vec2) -> Channel {
        match self {
            Channel::Scalar(kv) => Channel::Scalar(scale_1d(kv, factor.x)),
            Channel::Vector2(kv) => {
                let mut out = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    out.add(
                        keyframe.time,
                        Vec2::new(keyframe.value.x * factor.x, keyframe.value.y * factor.y),
                    );
                }
                Channel::Vector2(out)
            }
            Channel::Vector3(kv) => {
                let mut out = KeyframedValue::new();
                for keyframe in kv.keyframes() {
                    out.add(
                        keyframe.time,
                        Vec3::new(
                            keyframe.value.x * factor.x,
                            keyframe.value.y * factor.y,
                            keyframe.value.z,
                        ),
                    );
                }
                Channel::Vector3(out)
            }
            Channel::Color(_) => self.clone(),
        }
    }
}
