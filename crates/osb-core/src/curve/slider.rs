use crate::curve::{Curve, Vec2};

/// The interpolation used by a slider's control points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveType {
    Linear,
    Bezier,
    Catmull,
    Perfect,
}

impl CurveType {
    pub fn from_char(c: char) -> Option<CurveType> {
        match c.to_ascii_uppercase() {
            'L' => Some(CurveType::Linear),
            'B' => Some(CurveType::Bezier),
            'C' => Some(CurveType::Catmull),
            'P' => Some(CurveType::Perfect),
            _ => None,
        }
    }
}

/// A slider's curve: the control points plus the flattened polyline used for
/// position sampling.
#[derive(Debug, Clone)]
pub struct SliderPath {
    pub control_points: Vec<Vec2>,
    pub curve_type: CurveType,
    pub expected_distance: f64,
    sampled: Vec<Vec2>,
    cumulative: Vec<f64>,
    total_length: f64,
}

impl SliderPath {
    pub fn new(control_points: Vec<Vec2>, curve_type: CurveType, expected_distance: f64) -> Self {
        let sampled = flatten(&control_points, curve_type);
        let mut cumulative = Vec::with_capacity(sampled.len());
        let mut total = 0.0;
        for (index, point) in sampled.iter().enumerate() {
            if index > 0 {
                total += point.distance(sampled[index - 1]);
            }
            cumulative.push(total);
        }
        SliderPath {
            control_points,
            curve_type,
            expected_distance,
            sampled,
            cumulative,
            total_length: total,
        }
    }

    pub fn length(&self) -> f64 {
        self.total_length
    }

    pub fn sampled(&self) -> &[Vec2] {
        &self.sampled
    }

    /// Position at a normalized progress in `0.0..=1.0` along the path.
    pub fn position_at(&self, progress: f64) -> Vec2 {
        if self.sampled.is_empty() {
            return Vec2::ZERO;
        }
        if self.sampled.len() == 1 || self.total_length == 0.0 {
            return self.sampled[0];
        }
        let target = progress.clamp(0.0, 1.0) * self.total_length;
        let index = self
            .cumulative
            .partition_point(|&length| length < target)
            .clamp(1, self.sampled.len() - 1);
        let a = self.sampled[index - 1];
        let b = self.sampled[index];
        let segment = self.cumulative[index] - self.cumulative[index - 1];
        let t = if segment == 0.0 {
            0.0
        } else {
            (target - self.cumulative[index - 1]) / segment
        };
        a.lerp(b, t)
    }
}

/// The flattened polyline for the control points and curve type. The core curve
/// math (de Casteljau, Catmull-Rom) comes from this crate's [`Curve`]; the osu!
/// slider-specific rule is that a `Bezier` is split into segments at repeated
/// consecutive control points.
fn flatten(control_points: &[Vec2], curve_type: CurveType) -> Vec<Vec2> {
    if control_points.len() < 2 {
        return control_points.to_vec();
    }
    match curve_type {
        CurveType::Linear => control_points.to_vec(),
        CurveType::Bezier => flatten_bezier(control_points),
        CurveType::Catmull | CurveType::Perfect => {
            Curve::CatmullRom(control_points.to_vec()).flatten()
        }
    }
}

fn flatten_bezier(control_points: &[Vec2]) -> Vec<Vec2> {
    let mut result: Vec<Vec2> = Vec::new();
    let mut start = 0usize;
    for index in 1..control_points.len() {
        if control_points[index] == control_points[index - 1] {
            push_segment(&mut result, &control_points[start..index]);
            start = index;
        }
    }
    push_segment(&mut result, &control_points[start..]);
    result
}

fn push_segment(result: &mut Vec<Vec2>, segment: &[Vec2]) {
    if segment.len() < 2 {
        return;
    }
    let points = Curve::Bezier(segment.to_vec()).flatten();
    let skip = usize::from(!result.is_empty());
    result.extend(points.into_iter().skip(skip));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_path_endpoints() {
        let path = SliderPath::new(
            vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)],
            CurveType::Linear,
            100.0,
        );
        assert!((path.length() - 100.0).abs() < 1e-3);
        assert_eq!(path.position_at(0.0), Vec2::new(0.0, 0.0));
        assert!((path.position_at(0.5).x - 50.0).abs() < 1.0);
        assert!((path.position_at(1.0).x - 100.0).abs() < 1e-3);
    }

    #[test]
    fn bezier_midpoint() {
        let path = SliderPath::new(
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(50.0, 100.0),
                Vec2::new(100.0, 0.0),
            ],
            CurveType::Bezier,
            0.0,
        );
        let mid = path.position_at(0.5);
        assert!((mid.x - 50.0).abs() < 1.0);
        assert!((mid.y - 50.0).abs() < 5.0);
    }
}
