//! Near-plane clipping (). Projecting points behind the
//! camera with `|w|` produces garbage; this module clips a polygon in clip
//! space against `w >= near` (Sutherland-Hodgman) so partially-behind quads and
//! mesh faces are correct.

use crate::math::{Matrix4, Vec3, Vec4};

/// Transforms a local/world point into clip space (`wvp * (point, 1)`).
pub fn to_clip(wvp: &Matrix4, point: Vec3) -> Vec4 {
    wvp.transform_vec4(Vec4::new(point.x, point.y, point.z, 1.0))
}

fn lerp4(a: Vec4, b: Vec4, t: f64) -> Vec4 {
    Vec4::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
        a.w + (b.w - a.w) * t,
    )
}

/// Sutherland-Hodgman clip of `points` (clip space) against the near plane
/// `w >= near`. Returns the clipped polygon (empty when fully behind).
pub fn clip_near(points: &[Vec4], near: f64) -> Vec<Vec4> {
    if points.is_empty() {
        return Vec::new();
    }
    let signed_distance = |p: &Vec4| p.w - near;
    let mut input = points.to_vec();
    let mut output: Vec<Vec4> = Vec::with_capacity(input.len() + 1);

    for i in 0..input.len() {
        let current = input[i];
        let previous = input[(i + input.len() - 1) % input.len()];
        let dc = signed_distance(&current);
        let dp = signed_distance(&previous);

        if dc >= 0.0 {
            if dp < 0.0 {
                let t = dp / (dp - dc);
                output.push(lerp4(previous, current, t));
            }
            output.push(current);
        } else if dp >= 0.0 {
            let t = dp / (dp - dc);
            output.push(lerp4(previous, current, t));
        }
    }

    input.clear();
    output
}

/// Whether a clip-space point is in front of the near plane.
pub fn in_front(point: &Vec4, near: f64) -> bool {
    point.w >= near
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, w: f64) -> Vec4 {
        Vec4::new(x, y, 0.0, w)
    }

    #[test]
    fn keeps_a_polygon_fully_in_front() {
        let quad = [
            v(0.0, 0.0, 2.0),
            v(1.0, 0.0, 2.0),
            v(1.0, 1.0, 2.0),
            v(0.0, 1.0, 2.0),
        ];
        assert_eq!(clip_near(&quad, 1.0).len(), 4);
    }

    #[test]
    fn clips_a_polygon_crossing_the_near_plane() {
        // Two points in front (w=2), two behind (w=0).
        let quad = [
            v(0.0, 0.0, 2.0),
            v(1.0, 0.0, 2.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ];
        let clipped = clip_near(&quad, 1.0);
        // The behind edge is replaced by an interpolated (w=1) edge.
        assert!(clipped.len() >= 3);
        assert!(clipped.iter().all(|p| p.w >= 1.0 - 1e-9));
    }

    #[test]
    fn drops_a_polygon_fully_behind() {
        let quad = [
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(1.0, 1.0, -1.0),
            v(0.0, 1.0, -1.0),
        ];
        assert!(clip_near(&quad, 1.0).is_empty());
    }
}
