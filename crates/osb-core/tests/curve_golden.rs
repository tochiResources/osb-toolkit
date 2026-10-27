//! The shared curve golden corpus. The same vectors are exercised through the
//! native bridge and every SDK, so Rust stays the source of truth for the shape
//! of the math ().

use std::path::PathBuf;

use osb_core::curve::{simplify, Curve, Vec2};
use serde::Deserialize;

#[derive(Deserialize)]
struct Corpus {
    curves: Vec<CurveCase>,
    simplifications: Vec<SimplifyCase>,
}

#[derive(Deserialize)]
struct CurveCase {
    name: String,
    kind: String,
    points: Vec<[f64; 2]>,
    #[serde(default)]
    length: Option<f64>,
    #[serde(rename = "boundingBox", default)]
    bounding_box: Option<BoundsCase>,
    #[serde(default)]
    queries: Vec<QueryCase>,
}

#[derive(Deserialize)]
struct BoundsCase {
    min: Point,
    max: Point,
}

#[derive(Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct QueryCase {
    t: f64,
    #[serde(rename = "byArcLength", default)]
    by_arc_length: bool,
    point: Point,
}

#[derive(Deserialize)]
struct SimplifyCase {
    name: String,
    points: Vec<[f64; 2]>,
    tolerance: f64,
    expect: Vec<[f64; 2]>,
}

fn vec2(pair: [f64; 2]) -> Vec2 {
    Vec2::new(pair[0], pair[1])
}

fn build(kind: &str, points: &[[f64; 2]]) -> Curve {
    let points: Vec<Vec2> = points.iter().copied().map(vec2).collect();
    match kind {
        "Linear" => Curve::Polyline(points),
        "Quadratic" => Curve::Quadratic(points[0], points[1], points[2]),
        "Cubic" => Curve::Cubic(points[0], points[1], points[2], points[3]),
        "Bezier" => Curve::Bezier(points),
        "CatmullRom" => Curve::CatmullRom(points),
        other => panic!("unknown curve kind: {other}"),
    }
}

#[test]
fn curve_golden_corpus() {
    let corpus_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join("curve.json");
    let corpus: Corpus =
        serde_json::from_str(&osb_core::io::read_string(corpus_path).unwrap()).unwrap();
    assert!(
        !corpus.curves.is_empty(),
        "the golden corpus must have curves"
    );

    for case in &corpus.curves {
        let curve = build(&case.kind, &case.points);
        if let Some(expected) = case.length {
            assert!(
                (curve.length() - expected).abs() < 1e-3,
                "{} length: {} != {expected}",
                case.name,
                curve.length()
            );
        }
        if let Some(bounds) = &case.bounding_box {
            let actual = curve.bounding_box();
            assert!(
                (actual.min.x - bounds.min.x).abs() < 1e-6,
                "{} bbox min.x",
                case.name
            );
            assert!(
                (actual.min.y - bounds.min.y).abs() < 1e-6,
                "{} bbox min.y",
                case.name
            );
            assert!(
                (actual.max.x - bounds.max.x).abs() < 1e-6,
                "{} bbox max.x",
                case.name
            );
            assert!(
                (actual.max.y - bounds.max.y).abs() < 1e-6,
                "{} bbox max.y",
                case.name
            );
        }
        for query in &case.queries {
            let point = if query.by_arc_length {
                curve.point_at_length(query.t)
            } else {
                curve.point_at(query.t)
            };
            assert!(
                (point.x - query.point.x).abs() < 1e-6,
                "{} point.x at {}",
                case.name,
                query.t
            );
            assert!(
                (point.y - query.point.y).abs() < 1e-6,
                "{} point.y at {}",
                case.name,
                query.t
            );
        }
    }

    for case in &corpus.simplifications {
        let points: Vec<Vec2> = case.points.iter().copied().map(vec2).collect();
        let expected: Vec<Vec2> = case.expect.iter().copied().map(vec2).collect();
        assert_eq!(simplify(&points, case.tolerance), expected, "{}", case.name);
    }
}
