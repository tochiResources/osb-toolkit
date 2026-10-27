use crate::math::Vec2;

/// Simplifies a polyline with the Douglas-Peucker algorithm, keeping the
/// endpoints and every point farther than `tolerance` from the retained chord.
pub fn simplify(points: &[Vec2], tolerance: f64) -> Vec<Vec2> {
    if points.len() < 3 || tolerance <= 0.0 {
        return points.to_vec();
    }
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[points.len() - 1] = true;
    simplify_range(points, 0, points.len() - 1, tolerance, &mut keep);
    points
        .iter()
        .zip(&keep)
        .filter_map(|(point, keep)| keep.then_some(*point))
        .collect()
}

fn simplify_range(points: &[Vec2], first: usize, last: usize, tolerance: f64, keep: &mut [bool]) {
    if last <= first + 1 {
        return;
    }
    let mut farthest = 0.0;
    let mut index = first;
    for (offset, point) in points[first + 1..last].iter().enumerate() {
        let distance = perpendicular_distance(*point, points[first], points[last]);
        if distance > farthest {
            farthest = distance;
            index = first + 1 + offset;
        }
    }
    if farthest > tolerance {
        keep[index] = true;
        simplify_range(points, first, index, tolerance, keep);
        simplify_range(points, index, last, tolerance, keep);
    }
}

fn perpendicular_distance(point: Vec2, start: Vec2, end: Vec2) -> f64 {
    let line = end - start;
    let length = line.length();
    if length == 0.0 {
        return point.distance(start);
    }
    ((line.x * (start.y - point.y) - (start.x - point.x) * line.y) / length).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_a_near_collinear_polyline() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.05),
            Vec2::new(2.0, 0.0),
            Vec2::new(3.0, 0.0),
        ];
        assert_eq!(
            simplify(&points, 0.1),
            vec![Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0)]
        );
    }

    #[test]
    fn keeps_a_corner() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 0.0),
        ];
        assert_eq!(simplify(&points, 0.5), points);
    }
}
