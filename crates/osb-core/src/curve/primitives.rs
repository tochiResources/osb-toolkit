//! Curve primitives that are not defined by an arbitrary control-point list:
//! circular arcs, ellipses and line-segment intersection.

use crate::math::Vec2;

/// `count` points along a circular arc, sampled from `start_angle` (degrees)
/// sweeping `sweep_angle` (degrees) about `center`.
pub fn arc_points(
    center: Vec2,
    radius: f64,
    start_angle: f64,
    sweep_angle: f64,
    count: usize,
) -> Vec<Vec2> {
    let count = count.max(2);
    let start = start_angle.to_radians();
    let sweep = sweep_angle.to_radians();
    (0..count)
        .map(|index| {
            let angle = start + sweep * index as f64 / (count - 1) as f64;
            Vec2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

/// `count` points around an ellipse centred at `center` with the given radii.
pub fn ellipse_points(center: Vec2, radius_x: f64, radius_y: f64, count: usize) -> Vec<Vec2> {
    let count = count.max(2);
    (0..count)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / count as f64;
            Vec2::new(
                center.x + radius_x * angle.cos(),
                center.y + radius_y * angle.sin(),
            )
        })
        .collect()
}

/// The intersection of the segments `a1`-`a2` and `b1`-`b2`, if any.
pub fn line_intersection(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let d1 = a2 - a1;
    let d2 = b2 - b1;
    let denominator = d1.x * d2.y - d1.y * d2.x;
    if denominator.abs() < 1e-12 {
        return None;
    }
    let t = ((b1.x - a1.x) * d2.y - (b1.y - a1.y) * d2.x) / denominator;
    let u = ((b1.x - a1.x) * d1.y - (b1.y - a1.y) * d1.x) / denominator;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(a1 + d1 * t)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_endpoints_and_midpoint() {
        let points = arc_points(Vec2::new(0.0, 0.0), 10.0, 0.0, 180.0, 3);
        assert_eq!(points.len(), 3);
        assert!((points[0].x - 10.0).abs() < 1e-9);
        assert!((points[1].x).abs() < 1e-9);
        assert!((points[1].y - 10.0).abs() < 1e-9);
        assert!((points[2].x + 10.0).abs() < 1e-9);
    }

    #[test]
    fn ellipse_covers_its_radii() {
        let points = ellipse_points(Vec2::new(5.0, 5.0), 4.0, 2.0, 4);
        assert_eq!(points.len(), 4);
        assert!((points[0].x - 9.0).abs() < 1e-9);
        assert!((points[1].y - 7.0).abs() < 1e-9);
    }

    #[test]
    fn segments_intersect_only_when_they_cross() {
        let hit = line_intersection(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(10.0, 0.0),
        );
        let point = hit.expect("segments cross");
        assert!((point.x - 5.0).abs() < 1e-9);
        assert!((point.y - 5.0).abs() < 1e-9);

        assert!(line_intersection(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
        )
        .is_none());
    }
}
