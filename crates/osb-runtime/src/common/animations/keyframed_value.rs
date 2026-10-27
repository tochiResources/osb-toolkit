use crate::math::{Color4, Vec2, Vec3};

use super::{Easing, Keyframe};

/// Linear interpolation between two values.
pub trait Interpolate: Clone {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;

    /// Rounds the value to `decimals` places during simplification; the default
    /// leaves the value untouched.
    fn round_to(&mut self, _decimals: u32) {}
}

macro_rules! impl_interpolate_scalar {
    ($($t:ty),*) => {
        $(impl Interpolate for $t {
            fn lerp(a: &Self, b: &Self, t: f64) -> Self {
                a + (b - a) * t as $t
            }

            fn round_to(&mut self, decimals: u32) {
                let factor = 10f32.powi(decimals as i32) as $t;
                *self = (*self * factor).round() / factor;
            }
        })*
    };
}

impl_interpolate_scalar!(f32, f64);

impl Interpolate for Vec2 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        Vec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
    }

    fn round_to(&mut self, decimals: u32) {
        self.x.round_to(decimals);
        self.y.round_to(decimals);
    }
}

impl Interpolate for Vec3 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        Vec3::new(
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t,
        )
    }

    fn round_to(&mut self, decimals: u32) {
        self.x.round_to(decimals);
        self.y.round_to(decimals);
        self.z.round_to(decimals);
    }
}

impl Interpolate for Color4 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        Color4::new(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }
}

/// A time-ordered set of keyframes with linear interpolation.
///
/// `add` keeps the list sorted, `value_at`
/// clamps outside the range, and the `simplify_*` methods run Douglas-Peucker over
/// the sampled polyline to reduce keyframe count while staying within a tolerance.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeyframedValue<T> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolate> KeyframedValue<T> {
    pub fn new() -> Self {
        KeyframedValue {
            keyframes: Vec::new(),
        }
    }

    pub fn from_keyframes(keyframes: Vec<Keyframe<T>>) -> Self {
        let mut value = KeyframedValue { keyframes };
        value
            .keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        value
    }

    pub fn keyframes(&self) -> &[Keyframe<T>] {
        &self.keyframes
    }

    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Inserts a keyframe, replacing any existing keyframe at the same time. A
    /// replaced keyframe keeps its easing; a new one is linear.
    pub fn add(&mut self, time: f64, value: T) {
        match self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&time).unwrap())
        {
            Ok(index) => self.keyframes[index].value = value,
            Err(index) => self.keyframes.insert(index, Keyframe::new(time, value)),
        }
    }

    /// Inserts a keyframe whose outgoing segment uses `easing`.
    pub fn add_eased(&mut self, time: f64, value: T, easing: Easing) {
        match self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&time).unwrap())
        {
            Ok(index) => {
                self.keyframes[index].value = value;
                self.keyframes[index].easing = easing;
            }
            Err(index) => self
                .keyframes
                .insert(index, Keyframe::with_easing(time, value, easing)),
        }
    }

    /// The easing governing the segment that contains `time`.
    pub fn easing_at(&self, time: f64) -> Easing {
        if self.keyframes.is_empty() {
            return Easing::None;
        }
        let index = self
            .keyframes
            .partition_point(|k| k.time <= time)
            .saturating_sub(1);
        self.keyframes
            .get(index)
            .map(|keyframe| keyframe.easing)
            .unwrap_or(Easing::None)
    }

    /// Alias for [`add`](Self::add), so transpiled `KeyframedValue.Add` maps onto
    /// the same method as the collection `Add`.
    pub fn push(&mut self, time: f64, value: T) {
        self.add(time, value);
    }

    /// Value at `time`, clamped to the first/last keyframe outside the range.
    pub fn value_at(&self, time: f64) -> Option<T> {
        let first = self.keyframes.first()?;
        if time <= first.time {
            return Some(first.value.clone());
        }
        let last = self.keyframes.last()?;
        if time >= last.time {
            return Some(last.value.clone());
        }
        let index = self
            .keyframes
            .partition_point(|k| k.time <= time)
            .saturating_sub(1);
        let a = &self.keyframes[index];
        let b = &self.keyframes[index + 1];
        let span = b.time - a.time;
        let t = if span == 0.0 {
            0.0
        } else {
            (time - a.time) / span
        };
        // The segment's easing shapes the interpolation (linear when `None`).
        let eased = a.easing.apply(t as f32) as f64;
        Some(T::lerp(&a.value, &b.value, eased))
    }

    pub fn for_each_pair<F: FnMut(&Keyframe<T>, &Keyframe<T>)>(&self, mut f: F) {
        for window in self.keyframes.windows(2) {
            f(&window[0], &window[1]);
        }
    }

    /// `ForEachPair(callback, defaultValue, transform)` shape: the default and
    /// transform keyframe generation has already happened here, so only the
    /// callback is applied.
    pub fn for_each_pair_with<C, D, M>(&self, mut callback: C, _default: D, _transform: M)
    where
        C: FnMut(&Keyframe<T>, &Keyframe<T>),
        M: Fn(T) -> T,
    {
        for window in self.keyframes.windows(2) {
            callback(&window[0], &window[1]);
        }
    }

    fn simplify<F: Fn(&T, &T) -> f64>(&mut self, tolerance: f64, decimals: u32, distance: F) {
        if self.keyframes.len() <= 2 {
            return;
        }
        let mut keep = vec![false; self.keyframes.len()];
        let last = self.keyframes.len() - 1;
        keep[0] = true;
        keep[last] = true;
        self.rdp(&mut keep, 0, last, tolerance, &distance);
        self.keyframes = self
            .keyframes
            .iter()
            .zip(keep)
            .filter(|&(_k, keep)| keep)
            .map(|(k, _keep)| k.clone())
            .collect();
        if decimals < 7 {
            for keyframe in &mut self.keyframes {
                keyframe.value.round_to(decimals);
            }
        }
    }

    fn rdp<F: Fn(&T, &T) -> f64>(
        &self,
        keep: &mut [bool],
        a: usize,
        b: usize,
        tolerance: f64,
        distance: &F,
    ) {
        if b <= a + 1 {
            return;
        }
        let ta = self.keyframes[a].time;
        let tb = self.keyframes[b].time;
        let mut max_distance = 0.0;
        let mut max_index = a;
        for i in (a + 1)..b {
            let t = if tb == ta {
                0.0
            } else {
                (self.keyframes[i].time - ta) / (tb - ta)
            };
            let interpolated = T::lerp(&self.keyframes[a].value, &self.keyframes[b].value, t);
            let d = distance(&self.keyframes[i].value, &interpolated);
            if d > max_distance {
                max_distance = d;
                max_index = i;
            }
        }
        if max_distance > tolerance {
            keep[max_index] = true;
            self.rdp(keep, a, max_index, tolerance, distance);
            self.rdp(keep, max_index, b, tolerance, distance);
        }
    }
}

impl KeyframedValue<f64> {
    pub fn simplify_1d(&mut self, tolerance: f64, decimals: u32) {
        self.simplify(tolerance, decimals, |a, b| (a - b).abs());
    }

    /// Quantize each value with `transform`,
    /// then run Douglas-Peucker simplification.
    pub fn simplify_1d_keyframes(&mut self, tolerance: f64, transform: impl Fn(f64) -> f64) {
        for keyframe in &mut self.keyframes {
            keyframe.value = transform(keyframe.value);
        }
        self.simplify(tolerance, 7, |a, b| (a - b).abs());
    }
}

impl KeyframedValue<f32> {
    pub fn simplify_1d(&mut self, tolerance: f64, decimals: u32) {
        self.simplify(tolerance, decimals, |a, b| (a - b).abs() as f64);
    }
}

impl KeyframedValue<Vec2> {
    pub fn simplify_2d(&mut self, tolerance: f64, decimals: u32) {
        self.simplify(tolerance, decimals, |a, b| {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            (dx * dx + dy * dy).sqrt()
        });
    }

    /// Quantize each value with `transform`, then simplify.
    pub fn simplify_2d_keyframes(&mut self, tolerance: f64, transform: impl Fn(Vec2) -> Vec2) {
        for keyframe in &mut self.keyframes {
            keyframe.value = transform(keyframe.value);
        }
        self.simplify(tolerance, 7, |a, b| {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            (dx * dx + dy * dy).sqrt()
        });
    }
}

impl KeyframedValue<Vec3> {
    pub fn simplify_3d(&mut self, tolerance: f64, decimals: u32) {
        self.simplify(tolerance, decimals, |a, b| {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        });
    }
}

impl KeyframedValue<Color4> {
    /// Douglas-Peucker simplification of a color channel, using the largest
    /// per-channel (RGBA) deviation as the distance.
    pub fn simplify_color(&mut self, tolerance: f64, decimals: u32) {
        self.simplify(tolerance, decimals, |a, b| {
            (a.r - b.r)
                .abs()
                .max((a.g - b.g).abs())
                .max((a.b - b.b).abs())
                .max((a.a - b.a).abs())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_at_interpolates_linearly() {
        let mut kf = KeyframedValue::new();
        kf.add(0.0, 0.0f32);
        kf.add(100.0, 10.0f32);
        assert_eq!(kf.value_at(50.0), Some(5.0));
        assert_eq!(kf.value_at(-10.0), Some(0.0));
        assert_eq!(kf.value_at(1000.0), Some(10.0));
    }

    #[test]
    fn add_replaces_same_time() {
        let mut kf = KeyframedValue::new();
        kf.add(10.0, 1.0f32);
        kf.add(10.0, 2.0f32);
        assert_eq!(kf.len(), 1);
        assert_eq!(kf.value_at(10.0), Some(2.0));
    }

    #[test]
    fn simplify_collapses_collinear_points() {
        let mut kf = KeyframedValue::new();
        for i in 0..=100 {
            kf.add(i as f64, (i as f64) * 0.1);
        }
        kf.simplify_1d(0.01, 3);
        assert_eq!(kf.len(), 2);
    }

    #[test]
    fn simplify_keeps_deviation() {
        let mut kf = KeyframedValue::new();
        kf.add(0.0, 0.0f32);
        kf.add(50.0, 10.0f32);
        kf.add(100.0, 0.0f32);
        kf.simplify_1d(0.01, 3);
        assert_eq!(kf.len(), 3);
    }
}
