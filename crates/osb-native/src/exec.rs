//! The stateless JSON bridge (`osbtk_exec`). Every operation is dispatched here,
//! so the C surface stays small and new API is added in one place. Responses are
//! serialized from the typed `osb-api` models, so the wire shape and the OpenAPI
//! contract cannot drift.

use osb_api::response::OkResult;
use osb_api::{
    ArcRequest, Beatmap, BoolResult, CurveDistanceRequest, CurveKind, CurveOffsetRequest,
    CurvePointRequest, CurveRequest, CurveSampleRequest, CurveSpec, EasingResult, EllipseRequest,
    EncodeVideoRequest, ExtractAudioRequest, ExtractFramesRequest, FftResult,
    GetOriginVectorRequest, ImageProcessRequest, InScreenBoundsRequest, LineIntersectRequest,
    OptimizeRequest, OptimizeResult, OptionalPointResult, ParseOsbResult, PathRequest,
    PointListResult, RenderShapesRequest, ScalarResult, SimplifyRequest, VersionResult,
};
use osb_core::curve::{Curve, Vec2};
use osb_runtime::animations::Easing;
use osb_runtime::audio::AudioBuffer;
use osb_runtime::mapset::parse_osu;
use osb_runtime::storyboarding::Storyboard;
use osb_runtime::storyboarding::{OsbOrigin, OsbSprite};
use serde_json::Value;

use crate::bake::bake_to_json;
use crate::{error_json, to_json};

/// Runs a JSON request (`{"op": "...", ...}`) and returns a JSON response.
pub fn exec(request: &str) -> String {
    let value: Value = match serde_json::from_str(request) {
        Ok(value) => value,
        Err(error) => return error_json(&error.to_string()),
    };
    match value.get("op").and_then(Value::as_str).unwrap_or("") {
        "Common.version" => to_json(&VersionResult {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }),
        "Storyboarding3d.bake3d" => bake(&value),
        "Parsing.parseOsu" => parse_osu_op(&value),
        "Parsing.parseOsb" => parse_osb_op(&value),
        "Audio.fft" => fft_op(&value),
        "Animations.applyEasing" => crate::json::apply_easing(&value),
        "Animations.easingValue" => easing_value(&value),
        "math" => crate::math_ops::math_op(&value),
        "Curve.length" => curve_length_op(&value),
        "Curve.boundingBox" => curve_bounding_box_op(&value),
        "Curve.point" => curve_point_op(&value),
        "Curve.tangent" => curve_tangent_op(&value),
        "Curve.sample" => curve_sample_op(&value),
        "Curve.simplify" => simplify_op(&value),
        "Curve.normal" => curve_normal_op(&value),
        "Curve.curvature" => curve_curvature_op(&value),
        "Curve.distanceTo" => curve_distance_to_op(&value),
        "Curve.offset" => curve_offset_op(&value),
        "Curve.arc" => curve_arc_op(&value),
        "Curve.ellipse" => curve_ellipse_op(&value),
        "Curve.lineIntersect" => line_intersect_op(&value),
        "Diagnostics.diagnoseOsb" => diagnose_osb_op(&value),
        "Diagnostics.diagnoseOsu" => diagnose_osu_op(&value),
        "Optimize.optimizeOsb" => optimize_osb_op(&value),
        "Image.info" => image_info_op(&value),
        "Image.process" => image_process_op(&value),
        "Imagegen.renderShapes" => render_shapes_op(&value),
        "Media.info" => media_info_op(&value),
        "Media.extractFrames" => extract_frames_op(&value),
        "Media.encodeVideo" => encode_video_op(&value),
        "Media.extractAudio" => extract_audio_op(&value),
        "Origin.getOriginVector" => get_origin_vector_op(&value),
        "Origin.inScreenBounds" => in_screen_bounds_op(&value),
        "Helpers.keyframes" => crate::helpers::storyboarding3d_helper(&value),
        "Helpers.commands" => crate::helpers::storyboard_helper(&value),
        "Helpers.call" => crate::helpers::helper(&value),
        other => error_json(&format!("unknown op: {other}")),
    }
}

fn bake(value: &Value) -> String {
    match value.get("input") {
        Some(input) => bake_to_json(&input.to_string()),
        None => error_json("Storyboarding3d.bake3d requires `input`"),
    }
}

fn parse_osu_op(value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("Parsing.parseOsu requires `text`");
    };
    match parse_osu(text) {
        Ok(beatmap) => to_json(&Beatmap::from(&beatmap)),
        Err(error) => error_json(&error.to_string()),
    }
}

fn parse_osb_op(value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("Parsing.parseOsb requires `text`");
    };
    let mut storyboard = Storyboard::new();
    osb_runtime::storyboarding::import_osb(&mut storyboard, text);
    to_json(&ParseOsbResult {
        layer_count: storyboard.layer_count(),
        sprite_count: storyboard.sprite_count(),
    })
}

fn easing_value(value: &Value) -> String {
    let t = value.get("t").and_then(Value::as_f64).unwrap_or(0.0);
    if let Some(points) = value.get("cubicBezier").and_then(Value::as_array) {
        if points.len() == 4 {
            let p: Vec<f64> = points.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect();
            return to_json(&EasingResult {
                value: osb_runtime::animations::cubic_bezier(p[0], p[1], p[2], p[3], t),
            });
        }
    }
    to_json(&EasingResult {
        value: crate::json::easing(value).apply(t as f32) as f64,
    })
}

fn fft_op(value: &Value) -> String {
    let Some(path) = value.get("path").and_then(Value::as_str) else {
        return error_json("fft requires `path`");
    };
    let time = value.get("time").and_then(Value::as_f64).unwrap_or(0.0);
    let magnitudes = value
        .get("magnitudes")
        .and_then(Value::as_u64)
        .unwrap_or(64) as usize;
    match AudioBuffer::from_wav_file(path) {
        Ok(audio) => to_json(&FftResult {
            magnitudes: audio.get_fft(time, magnitudes, Easing::None, 0.0),
        }),
        Err(error) => error_json(&error.to_string()),
    }
}
fn curve_from_spec(spec: &CurveSpec) -> Result<Curve, String> {
    let points: Vec<Vec2> = spec.points.iter().map(|p| Vec2::new(p.x, p.y)).collect();
    match spec.kind {
        CurveKind::Linear => Ok(Curve::Polyline(points)),
        CurveKind::Quadratic => match points.as_slice() {
            [a, b, c] => Ok(Curve::Quadratic(*a, *b, *c)),
            _ => Err("a Quadratic curve needs exactly 3 points".to_string()),
        },
        CurveKind::Cubic => match points.as_slice() {
            [a, b, c, d] => Ok(Curve::Cubic(*a, *b, *c, *d)),
            _ => Err("a Cubic curve needs exactly 4 points".to_string()),
        },
        CurveKind::Bezier => Ok(Curve::Bezier(points)),
        CurveKind::CatmullRom => Ok(Curve::CatmullRom(points)),
    }
}

fn parse_request<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T, String> {
    serde_json::from_value(value.clone()).map_err(|error| error.to_string())
}

fn curve_length_op(value: &Value) -> String {
    let request: CurveRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&ScalarResult {
            value: curve.length(),
        }),
        Err(error) => error_json(&error),
    }
}

fn curve_bounding_box_op(value: &Value) -> String {
    let request: CurveRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&curve.bounding_box()),
        Err(error) => error_json(&error),
    }
}

fn curve_point_op(value: &Value) -> String {
    let request: CurvePointRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => {
            let point = if request.by_arc_length.unwrap_or(false) {
                curve.point_at_length(request.t)
            } else {
                curve.point_at(request.t)
            };
            to_json(&point)
        }
        Err(error) => error_json(&error),
    }
}

fn curve_tangent_op(value: &Value) -> String {
    let request: CurvePointRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&curve.tangent_at(request.t)),
        Err(error) => error_json(&error),
    }
}

fn curve_sample_op(value: &Value) -> String {
    let request: CurveSampleRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => {
            let points: Vec<osb_api::math::Vector2> = curve
                .sample(request.count, request.by_arc_length.unwrap_or(false))
                .into_iter()
                .collect();
            to_json(&PointListResult { points })
        }
        Err(error) => error_json(&error),
    }
}

fn simplify_op(value: &Value) -> String {
    let request: SimplifyRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let points: Vec<Vec2> = request.points.iter().map(|p| Vec2::new(p.x, p.y)).collect();
    let points: Vec<osb_api::math::Vector2> = osb_core::curve::simplify(&points, request.tolerance)
        .into_iter()
        .collect();
    to_json(&PointListResult { points })
}

fn curve_normal_op(value: &Value) -> String {
    let request: CurvePointRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&curve.normal_at(request.t)),
        Err(error) => error_json(&error),
    }
}

fn curve_curvature_op(value: &Value) -> String {
    let request: CurvePointRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&ScalarResult {
            value: curve.curvature_at(request.t),
        }),
        Err(error) => error_json(&error),
    }
}

fn curve_distance_to_op(value: &Value) -> String {
    let request: CurveDistanceRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => to_json(&ScalarResult {
            value: curve.distance_to_point(Vec2::new(request.point.x, request.point.y)),
        }),
        Err(error) => error_json(&error),
    }
}

fn curve_offset_op(value: &Value) -> String {
    let request: CurveOffsetRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match curve_from_spec(&request.curve) {
        Ok(curve) => {
            let points: Vec<osb_api::math::Vector2> =
                curve.offset(request.distance).into_iter().collect();
            to_json(&PointListResult { points })
        }
        Err(error) => error_json(&error),
    }
}

fn curve_arc_op(value: &Value) -> String {
    let request: ArcRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let points: Vec<osb_api::math::Vector2> = osb_core::curve::arc_points(
        Vec2::new(request.center.x, request.center.y),
        request.radius,
        request.start_angle,
        request.sweep_angle,
        request.count.unwrap_or(64) as usize,
    )
    .into_iter()
    .collect();
    to_json(&PointListResult { points })
}

fn curve_ellipse_op(value: &Value) -> String {
    let request: EllipseRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let points: Vec<osb_api::math::Vector2> = osb_core::curve::ellipse_points(
        Vec2::new(request.center.x, request.center.y),
        request.radius_x,
        request.radius_y,
        request.count.unwrap_or(64) as usize,
    )
    .into_iter()
    .collect();
    to_json(&PointListResult { points })
}

fn line_intersect_op(value: &Value) -> String {
    let request: LineIntersectRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let point = osb_core::curve::line_intersection(
        Vec2::new(request.a1.x, request.a1.y),
        Vec2::new(request.a2.x, request.a2.y),
        Vec2::new(request.b1.x, request.b1.y),
        Vec2::new(request.b2.x, request.b2.y),
    );
    to_json(&OptionalPointResult { point })
}

fn diagnose_osb_op(value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("diagnoseOsb requires `text`");
    };
    let mut storyboard = Storyboard::new();
    osb_runtime::storyboarding::import_osb(&mut storyboard, text);
    let diagnostics = osb_runtime::diagnostics::diagnose_storyboard(&storyboard);
    to_json(&osb_runtime::diagnostics::summarize(diagnostics))
}

fn diagnose_osu_op(value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("diagnoseOsu requires `text`");
    };
    match parse_osu(text) {
        Ok(beatmap) => {
            let diagnostics = osb_runtime::diagnostics::diagnose_beatmap(&beatmap);
            to_json(&osb_runtime::diagnostics::summarize(diagnostics))
        }
        Err(error) => error_json(&error.to_string()),
    }
}

fn optimize_osb_op(value: &Value) -> String {
    let request: OptimizeRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let mut storyboard = Storyboard::new();
    osb_runtime::storyboarding::import_osb(&mut storyboard, &request.text);
    let options = osb_runtime::optimize::OptimizeOptions {
        remove_redundant: request.options.remove_redundant.unwrap_or(false),
        deduplicate: request.options.deduplicate.unwrap_or(false),
        merge_adjacent: request.options.merge_adjacent.unwrap_or(false),
        split_threshold: request.options.split_threshold,
    };
    let stats = osb_runtime::optimize::optimize_storyboard(&mut storyboard, &options);
    let osb = osb_runtime::optimize::export_osb(&storyboard);
    to_json(&OptimizeResult { osb, stats })
}

fn image_info_op(value: &Value) -> String {
    let request: PathRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_image::image_info(&request.path) {
        Ok(info) => to_json(&info),
        Err(error) => error_json(&error),
    }
}

fn image_process_op(value: &Value) -> String {
    let request: ImageProcessRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_image::process(&request.path, &request.output, &request.options) {
        Ok(info) => to_json(&info),
        Err(error) => error_json(&error),
    }
}

fn render_shapes_op(value: &Value) -> String {
    let request: RenderShapesRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_image_gen::render(&request) {
        Ok(info) => to_json(&info),
        Err(error) => error_json(&error),
    }
}

fn media_info_op(value: &Value) -> String {
    let request: PathRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_media::media_info(&request.path) {
        Ok(info) => to_json(&info),
        Err(error) => error_json(&error),
    }
}

fn extract_frames_op(value: &Value) -> String {
    let request: ExtractFramesRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_media::extract_frames(&request) {
        Ok(result) => to_json(&result),
        Err(error) => error_json(&error),
    }
}

fn encode_video_op(value: &Value) -> String {
    let request: EncodeVideoRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_media::encode_video(&request) {
        Ok(info) => to_json(&info),
        Err(error) => error_json(&error),
    }
}

fn extract_audio_op(value: &Value) -> String {
    let request: ExtractAudioRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    match osb_media::extract_audio(&request) {
        Ok(()) => to_json(&OkResult { ok: true }),
        Err(error) => error_json(&error),
    }
}

fn get_origin_vector_op(value: &Value) -> String {
    let request: GetOriginVectorRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let origin: OsbOrigin = request.origin;
    let vector = OsbSprite::get_origin_vector(origin, request.width, request.height);
    to_json(&vector)
}

fn in_screen_bounds_op(value: &Value) -> String {
    let request: InScreenBoundsRequest = match parse_request(value) {
        Ok(request) => request,
        Err(error) => return error_json(&error),
    };
    let in_bounds = OsbSprite::in_screen_bounds(
        Vec2::new(request.position.x, request.position.y),
        Vec2::new(request.size.x, request.size.y),
        request.rotation,
        Vec2::new(request.origin.x, request.origin.y),
    );
    to_json(&BoolResult { value: in_bounds })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_the_version() {
        let value: Value = serde_json::from_str(&exec(r#"{"op":"Common.version"}"#)).unwrap();
        assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn parses_a_beatmap_summary() {
        let request = serde_json::json!({
            "op": "Parsing.parseOsu",
            "text": "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n\
                     [TimingPoints]\n0,500,4,2,1,100,1,0\n\
                     [HitObjects]\n256,192,1000,1,0\n"
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["audioFilename"], "song.mp3");
        assert_eq!(value["timingPoints"][0]["beatDuration"], 500.0);
        assert_eq!(value["timingPoints"][0]["bpm"], 120.0);
        assert_eq!(value["hitObjects"][0]["kind"], "Circle");
        assert_eq!(value["sliderMultiplier"], 1.4);
    }

    #[test]
    fn parses_an_osb_summary() {
        let request = serde_json::json!({
            "op": "Parsing.parseOsb",
            "text": "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n"
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["spriteCount"], 1);
    }

    #[test]
    fn rejects_unknown_operations() {
        let value: Value = serde_json::from_str(&exec(r#"{"op":"nope"}"#)).unwrap();
        assert!(value["error"].is_string());
    }

    #[test]
    fn measures_a_curve() {
        let request = serde_json::json!({
            "op": "Curve.length",
            "curve": { "kind": "Linear", "points": [{ "x": 0.0, "y": 0.0 }, { "x": 100.0, "y": 0.0 }] }
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["value"], 100.0);

        let request = serde_json::json!({
            "op": "Curve.boundingBox",
            "curve": { "kind": "Quadratic", "points": [
                { "x": 0.0, "y": 0.0 }, { "x": 50.0, "y": 100.0 }, { "x": 100.0, "y": 0.0 }
            ] }
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["min"]["x"], 0.0);
        assert_eq!(value["max"]["x"], 100.0);
        assert!((value["max"]["y"].as_f64().unwrap() - 50.0).abs() < 1e-6);
    }

    #[test]
    fn queries_and_samples_a_curve() {
        let curve = serde_json::json!({ "kind": "Linear", "points": [
            { "x": 0.0, "y": 0.0 }, { "x": 100.0, "y": 0.0 }
        ] });

        let request =
            serde_json::json!({ "op": "Curve.point", "curve": curve, "t": 0.5 }).to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["x"], 50.0);
        assert_eq!(value["y"], 0.0);

        let request =
            serde_json::json!({ "op": "Curve.tangent", "curve": curve, "t": 0.5 }).to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!((value["x"].as_f64().unwrap() - 1.0).abs() < 1e-6);

        let request =
            serde_json::json!({ "op": "Curve.sample", "curve": curve, "count": 3 }).to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["points"].as_array().unwrap().len(), 3);
        assert_eq!(value["points"][2]["x"], 100.0);
    }

    #[test]
    fn simplifies_a_polyline() {
        let request = serde_json::json!({
            "op": "Curve.simplify",
            "points": [
                { "x": 0.0, "y": 0.0 }, { "x": 1.0, "y": 0.05 },
                { "x": 2.0, "y": 0.0 }, { "x": 3.0, "y": 0.0 }
            ],
            "tolerance": 0.1
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["points"].as_array().unwrap().len(), 2);
        assert_eq!(value["points"][1]["x"], 3.0);
    }

    #[test]
    fn reports_curve_normal_curvature_distance_and_offset() {
        let curve = serde_json::json!({ "kind": "Linear", "points": [
            { "x": 0.0, "y": 0.0 }, { "x": 100.0, "y": 0.0 }
        ] });

        let request =
            serde_json::json!({ "op": "Curve.normal", "curve": curve, "t": 0.5 }).to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!(value["x"].as_f64().unwrap().abs() < 1e-6);
        assert!((value["y"].as_f64().unwrap().abs() - 1.0).abs() < 1e-6);

        let request =
            serde_json::json!({ "op": "Curve.curvature", "curve": curve, "t": 0.5 }).to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!(value["value"].as_f64().unwrap().abs() < 1e-6);

        let request = serde_json::json!({
            "op": "Curve.distanceTo",
            "curve": curve,
            "point": { "x": 50.0, "y": 10.0 }
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!((value["value"].as_f64().unwrap() - 10.0).abs() < 1e-6);

        let request = serde_json::json!({ "op": "Curve.offset", "curve": curve, "distance": 5.0 })
            .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["points"].as_array().unwrap().len(), 2);
        assert!((value["points"][0]["y"].as_f64().unwrap().abs() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn diagnoses_artifacts() {
        let request = serde_json::json!({
            "op": "Diagnostics.diagnoseOsb",
            "text": "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,100,0,1\n"
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!(value["diagnostics"].is_array());
        assert!(value["errorCount"].is_number());

        let request = serde_json::json!({
            "op": "Diagnostics.diagnoseOsu",
            "text": "osu file format v14\n\n[General]\nAudioFilename: \nMode: 0\n"
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert!(value["diagnostics"].is_array());
        assert!(value["errorCount"].as_u64().unwrap() >= 1);
    }

    #[test]
    fn optimizes_a_storyboard() {
        let request = serde_json::json!({
            "op": "Optimize.optimizeOsb",
            "text": "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n",
            "options": { "deduplicate": true }
        })
        .to_string();
        let value: Value = serde_json::from_str(&exec(&request)).unwrap();
        assert_eq!(value["stats"]["deduplicated"], 1);
        assert!(value["osb"].as_str().unwrap().contains("Sprite,Foreground"));
    }
}
