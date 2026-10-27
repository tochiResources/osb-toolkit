use crate::curve::Curve;
use crate::curve::{Box2, Vec2};

/// Samples per curve segment used to flatten non-linear curves.
pub const SAMPLES_PER_SEGMENT: usize = 64;

/// The flattened polyline approximation of the curve. A `Polyline` is returned
/// verbatim (its length and bounds are then exact).
pub fn flatten(curve: &Curve) -> Vec<Vec2> {
    match curve {
        Curve::Polyline(points) => points.clone(),
        other => {
            let steps = segment_steps(other);
            (0..=steps)
                .map(|step| other.point_at(step as f64 / steps as f64))
                .collect()
        }
    }
}

fn segment_count(curve: &Curve) -> usize {
    match curve {
        Curve::Polyline(points) | Curve::CatmullRom(points) => points.len().saturating_sub(1),
        _ => 1,
    }
}

fn segment_steps(curve: &Curve) -> usize {
    segment_count(curve).max(1) * SAMPLES_PER_SEGMENT
}

/// The arc length of the curve (exact for a `Polyline`, sampled otherwise).
pub fn length(curve: &Curve) -> f64 {
    let points = flatten(curve);
    points
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum()
}

fn cumulative(points: &[Vec2]) -> Vec<f64> {
    let mut cumulative = Vec::with_capacity(points.len());
    let mut total = 0.0;
    for (index, point) in points.iter().enumerate() {
        if index > 0 {
            total += point.distance(points[index - 1]);
        }
        cumulative.push(total);
    }
    cumulative
}

/// The point at the arc-length fraction `t` in `0.0..=1.0`.
pub fn point_at_length(curve: &Curve, t: f64) -> Vec2 {
    let points = flatten(curve);
    if points.is_empty() {
        return Vec2::ZERO;
    }
    if points.len() == 1 {
        return points[0];
    }
    let cumulative = cumulative(&points);
    let total = cumulative[points.len() - 1];
    if total == 0.0 {
        return points[0];
    }
    let target = t.clamp(0.0, 1.0) * total;
    let index = cumulative
        .partition_point(|&length| length < target)
        .clamp(1, points.len() - 1);
    let segment = cumulative[index] - cumulative[index - 1];
    let local = if segment == 0.0 {
        0.0
    } else {
        (target - cumulative[index - 1]) / segment
    };
    points[index - 1].lerp(points[index], local)
}

/// The axis-aligned bounding box of the flattened curve.
pub fn bounding_box(curve: &Curve) -> Box2 {
    let points = flatten(curve);
    let Some((first, rest)) = points.split_first() else {
        return Box2::default();
    };
    let mut min = *first;
    let mut max = *first;
    for point in rest {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }
    Box2::new(min, max)
}

/// `count` points along the curve, parametrically or by arc length.
pub fn sample(curve: &Curve, count: usize, by_arc_length: bool) -> Vec<Vec2> {
    match count {
        0 => Vec::new(),
        1 => vec![curve.point_at(0.0)],
        count => (0..count)
            .map(|index| {
                let t = index as f64 / (count - 1) as f64;
                if by_arc_length {
                    point_at_length(curve, t)
                } else {
                    curve.point_at(t)
                }
            })
            .collect(),
    }
}

/// The curvature (the reciprocal of the radius of the osculating circle) at the
/// parametric position `t`, estimated with central differences.
pub fn curvature(curve: &Curve, t: f64) -> f64 {
    const H: f64 = 1e-4;
    let t = t.clamp(0.0, 1.0);
    let low = (t - H).max(0.0);
    let high = (t + H).min(1.0);
    if high <= low {
        return 0.0;
    }
    let middle = 0.5 * (low + high);
    let a = curve.point_at(low);
    let b = curve.point_at(middle);
    let c = curve.point_at(high);
    let half = 0.5 * (high - low);
    let first = (c - a) / (high - low);
    let second = (c - b * 2.0 + a) / (half * half);
    let speed = first.length_squared();
    if speed == 0.0 {
        return 0.0;
    }
    (first.x * second.y - first.y * second.x).abs() / speed.powf(1.5)
}

fn closest_on_segment(point: Vec2, start: Vec2, end: Vec2) -> Vec2 {
    let segment = end - start;
    let length_squared = segment.length_squared();
    if length_squared == 0.0 {
        return start;
    }
    let t = ((point - start).dot(segment) / length_squared).clamp(0.0, 1.0);
    start + segment * t
}

/// The closest point on the (flattened) curve to `point`.
pub fn nearest_point(curve: &Curve, point: Vec2) -> Vec2 {
    let points = flatten(curve);
    match points.as_slice() {
        [] => Vec2::ZERO,
        [single] => *single,
        _ => {
            let mut best = points[0];
            let mut best_distance = f64::INFINITY;
            for pair in points.windows(2) {
                let candidate = closest_on_segment(point, pair[0], pair[1]);
                let distance = candidate.distance(point);
                if distance < best_distance {
                    best_distance = distance;
                    best = candidate;
                }
            }
            best
        }
    }
}

/// The distance from `point` to the (flattened) curve.
pub fn distance_to_point(curve: &Curve, point: Vec2) -> f64 {
    nearest_point(curve, point).distance(point)
}

/// The flattened polyline offset by `distance` along its local normal.
pub fn offset(curve: &Curve, distance: f64) -> Vec<Vec2> {
    let points = flatten(curve);
    let count = points.len();
    if count < 2 {
        return points;
    }
    let mut result = Vec::with_capacity(count);
    for (index, point) in points.iter().enumerate() {
        let direction = if index == 0 {
            points[1] - points[0]
        } else if index == count - 1 {
            points[count - 1] - points[count - 2]
        } else {
            points[index + 1] - points[index - 1]
        };
        let normal = direction.normalized().perpendicular();
        result.push(*point + normal * distance);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_length_is_exact() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        assert!((length(&curve) - 100.0).abs() < 1e-9);
        assert!((point_at_length(&curve, 0.5).x - 50.0).abs() < 1e-9);
    }

    #[test]
    fn bounding_box_covers_the_curve() {
        let curve = Curve::Quadratic(
            Vec2::new(0.0, 0.0),
            Vec2::new(50.0, 100.0),
            Vec2::new(100.0, 0.0),
        );
        let bounds = bounding_box(&curve);
        assert_eq!(bounds.min, Vec2::new(0.0, 0.0));
        assert!((bounds.max.x - 100.0).abs() < 1e-6);
        assert!((bounds.max.y - 50.0).abs() < 1e-6);
    }

    #[test]
    fn sampling_yields_endpoints() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)]);
        let points = sample(&curve, 5, false);
        assert_eq!(points.len(), 5);
        assert_eq!(points[0], Vec2::new(0.0, 0.0));
        assert_eq!(points[4], Vec2::new(10.0, 0.0));
    }

    #[test]
    fn normal_is_perpendicular_to_the_tangent() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        let normal = curve.normal_at(0.5);
        assert!(normal.x.abs() < 1e-6);
        assert!((normal.y.abs() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn curvature_of_a_line_is_zero() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        assert!(curve.curvature_at(0.5).abs() < 1e-6);
    }

    #[test]
    fn nearest_point_and_distance() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        let nearest = curve.nearest_point(Vec2::new(50.0, 10.0));
        assert!((nearest.x - 50.0).abs() < 1e-6);
        assert!(nearest.y.abs() < 1e-6);
        assert!((curve.distance_to_point(Vec2::new(50.0, 10.0)) - 10.0).abs() < 1e-6);
    }

    #[test]
    fn offset_shifts_a_line_along_its_normal() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        let offset = curve.offset(5.0);
        assert_eq!(offset.len(), 2);
        assert!((offset[0].y.abs() - 5.0).abs() < 1e-6);
        assert!((offset[1].y.abs() - 5.0).abs() < 1e-6);
    }
}
