use crate::curve::{sample, Box2, Vec2};

/// The interpolation a control-point list is interpreted as.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub enum CurveKind {
    /// A polyline through the points.
    Linear,
    /// A single quadratic Bezier (exactly three points).
    Quadratic,
    /// A single cubic Bezier (exactly four points).
    Cubic,
    /// A Bezier of any degree (de Casteljau over the points).
    Bezier,
    /// A Catmull-Rom spline through the points.
    CatmullRom,
}

/// A curve: control points plus the interpolation that joins them.
#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    /// A polyline through the points.
    Polyline(Vec<Vec2>),
    /// A quadratic Bezier.
    Quadratic(Vec2, Vec2, Vec2),
    /// A cubic Bezier.
    Cubic(Vec2, Vec2, Vec2, Vec2),
    /// A Bezier of any degree.
    Bezier(Vec<Vec2>),
    /// A Catmull-Rom spline through the points.
    CatmullRom(Vec<Vec2>),
}

impl Curve {
    /// The point at the parametric position `t` in `0.0..=1.0`.
    pub fn point_at(&self, t: f64) -> Vec2 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Curve::Polyline(points) => polyline_point(points, t),
            Curve::Quadratic(a, b, c) => bezier_point(&[*a, *b, *c], t),
            Curve::Cubic(a, b, c, d) => bezier_point(&[*a, *b, *c, *d], t),
            Curve::Bezier(points) => bezier_point(points, t),
            Curve::CatmullRom(points) => catmull_point(points, t),
        }
    }

    /// The unit tangent at the parametric position `t`.
    pub fn tangent_at(&self, t: f64) -> Vec2 {
        const H: f64 = 1e-4;
        let t = t.clamp(0.0, 1.0);
        let delta = self.point_at((t + H).min(1.0)) - self.point_at((t - H).max(0.0));
        delta.normalized()
    }

    /// The arc length of the curve.
    pub fn length(&self) -> f64 {
        sample::length(self)
    }

    /// The point at the arc-length fraction `t` in `0.0..=1.0`.
    pub fn point_at_length(&self, t: f64) -> Vec2 {
        sample::point_at_length(self, t)
    }

    /// The axis-aligned bounding box of the flattened curve.
    pub fn bounding_box(&self) -> Box2 {
        sample::bounding_box(self)
    }

    /// The flattened polyline approximation of the curve.
    pub fn flatten(&self) -> Vec<Vec2> {
        sample::flatten(self)
    }

    /// `count` points along the curve, parametrically or by arc length.
    pub fn sample(&self, count: usize, by_arc_length: bool) -> Vec<Vec2> {
        sample::sample(self, count, by_arc_length)
    }

    /// The unit normal at the parametric position `t` (the tangent rotated 90°).
    pub fn normal_at(&self, t: f64) -> Vec2 {
        self.tangent_at(t).perpendicular()
    }

    /// The curvature at the parametric position `t`.
    pub fn curvature_at(&self, t: f64) -> f64 {
        sample::curvature(self, t)
    }

    /// The closest point on the curve to `point`.
    pub fn nearest_point(&self, point: Vec2) -> Vec2 {
        sample::nearest_point(self, point)
    }

    /// The distance from `point` to the curve.
    pub fn distance_to_point(&self, point: Vec2) -> f64 {
        sample::distance_to_point(self, point)
    }

    /// The flattened polyline offset by `distance` along its local normal.
    pub fn offset(&self, distance: f64) -> Vec<Vec2> {
        sample::offset(self, distance)
    }
}

fn polyline_point(points: &[Vec2], t: f64) -> Vec2 {
    match points.len() {
        0 => Vec2::ZERO,
        1 => points[0],
        count => {
            let scaled = t * (count - 1) as f64;
            let index = (scaled.floor() as usize).min(count - 2);
            points[index].lerp(points[index + 1], scaled - index as f64)
        }
    }
}

fn bezier_point(points: &[Vec2], t: f64) -> Vec2 {
    match points.len() {
        0 => Vec2::ZERO,
        1 => points[0],
        _ => {
            let mut work = points.to_vec();
            while work.len() > 1 {
                for index in 0..work.len() - 1 {
                    work[index] = work[index].lerp(work[index + 1], t);
                }
                work.pop();
            }
            work[0]
        }
    }
}

fn catmull_point(points: &[Vec2], t: f64) -> Vec2 {
    let count = points.len();
    match count {
        0 => Vec2::ZERO,
        1 => points[0],
        _ => {
            let scaled = t * (count - 1) as f64;
            let index = (scaled.floor() as usize).min(count - 2);
            let p0 = if index == 0 {
                points[0]
            } else {
                points[index - 1]
            };
            let p1 = points[index];
            let p2 = points[index + 1];
            let p3 = if index + 2 < count {
                points[index + 2]
            } else {
                points[index + 1]
            };
            catmull_rom(p0, p1, p2, p3, scaled - index as f64)
        }
    }
}

fn catmull_rom(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f64) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let x = 0.5
        * ((2.0 * p1.x)
            + (-p0.x + p2.x) * t
            + (2.0 * p0.x - 5.0 * p1.x + 4.0 * p2.x - p3.x) * t2
            + (-p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x) * t3);
    let y = 0.5
        * ((2.0 * p1.y)
            + (-p0.y + p2.y) * t
            + (2.0 * p0.y - 5.0 * p1.y + 4.0 * p2.y - p3.y) * t2
            + (-p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y) * t3);
    Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_midpoint() {
        let curve = Curve::Polyline(vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)]);
        assert_eq!(curve.point_at(0.5), Vec2::new(50.0, 0.0));
        assert!((curve.tangent_at(0.5).x - 1.0).abs() < 1e-6);
    }

    #[test]
    fn quadratic_apex() {
        let curve = Curve::Quadratic(
            Vec2::new(0.0, 0.0),
            Vec2::new(50.0, 100.0),
            Vec2::new(100.0, 0.0),
        );
        assert_eq!(curve.point_at(0.5), Vec2::new(50.0, 50.0));
    }

    #[test]
    fn cubic_midpoint() {
        let curve = Curve::Cubic(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 100.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(100.0, 0.0),
        );
        assert_eq!(curve.point_at(0.5), Vec2::new(50.0, 75.0));
    }
}
