use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::error::Error;
use crate::models::{
    BakedStoryboard, Beatmap, BoolResult, Box2, CurveSpec, DiagnosticReport, Easing, EasingResult,
    EncodeVideoRequest, ExtractFramesRequest, FftResult, FrameResult, ImageInfo, ImageOptions,
    MediaInfo, OptimizeOptionsInput, OptimizeResult, OptionalPointResult, OsbOrigin,
    ParseOsbResult, PointListResult, RenderShapesRequest, ScalarResult, Scene3dInput, Vector2,
    VersionResult,
};

/// The stateless SDK entry point: version, parsing, easing, FFT and the 3D bake.
pub struct Client;

impl Client {
    /// The native runtime version string.
    pub fn version() -> Result<String, Error> {
        Ok(call::<VersionResult>(json!({ "op": "Common.version" }))?.version)
    }

    /// Parses a `.osu` beatmap into the typed model.
    pub fn parse_beatmap(text: &str) -> Result<Beatmap, Error> {
        call(json!({ "op": "Parsing.parseOsu", "text": text }))
    }

    /// Parses a `.osb` storyboard into a summary.
    pub fn parse_osb(text: &str) -> Result<ParseOsbResult, Error> {
        call(json!({ "op": "Parsing.parseOsb", "text": text }))
    }

    /// Applies an easing function to a normalized time in `0..1`.
    pub fn apply_easing(easing: Easing, t: f64) -> Result<f64, Error> {
        Ok(call::<EasingResult>(
            json!({ "op": "Animations.applyEasing", "easing": easing, "t": t }),
        )?
        .value)
    }

    /// Evaluates a CSS-style cubic-bezier easing (`CustomEasing`)
    /// with control points `(x1, y1)`, `(x2, y2)` at a normalized time.
    pub fn apply_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> Result<f64, Error> {
        Ok(call::<EasingResult>(
            json!({ "op": "Animations.easingValue", "cubicBezier": [x1, y1, x2, y2], "t": t }),
        )?
        .value)
    }

    /// Reads FFT magnitudes from a `.wav` file.
    pub fn fft(path: &str, time: f64, magnitudes: usize) -> Result<FftResult, Error> {
        call(json!({ "op": "Audio.fft", "path": path, "time": time, "magnitudes": magnitudes }))
    }

    /// Bakes a 3D scene into a 2D storyboard (3D bake).
    pub fn bake3d(scene: Scene3dInput) -> Result<BakedStoryboard, Error> {
        call(json!({ "op": "Storyboarding3d.bake3d", "input": scene }))
    }

    /// The arc length of a curve.
    pub fn curve_length(curve: CurveSpec) -> Result<f64, Error> {
        Ok(call::<ScalarResult>(json!({ "op": "Curve.length", "curve": curve }))?.value)
    }

    /// The axis-aligned bounding box of a curve.
    pub fn curve_bounding_box(curve: CurveSpec) -> Result<Box2, Error> {
        call(json!({ "op": "Curve.boundingBox", "curve": curve }))
    }

    /// The point at `t`, parametrically or by arc length.
    pub fn curve_point(curve: CurveSpec, t: f64, by_arc_length: bool) -> Result<Vector2, Error> {
        call(json!({
            "op": "Curve.point",
            "curve": curve,
            "t": t,
            "byArcLength": by_arc_length,
        }))
    }

    /// The unit tangent at `t`.
    pub fn curve_tangent(curve: CurveSpec, t: f64) -> Result<Vector2, Error> {
        call(json!({ "op": "Curve.tangent", "curve": curve, "t": t }))
    }

    /// `count` points along the curve, parametrically or by arc length.
    pub fn curve_sample(
        curve: CurveSpec,
        count: usize,
        by_arc_length: bool,
    ) -> Result<PointListResult, Error> {
        call(json!({
            "op": "Curve.sample",
            "curve": curve,
            "count": count,
            "byArcLength": by_arc_length,
        }))
    }

    /// Simplifies a polyline with the Douglas-Peucker algorithm.
    pub fn simplify(points: Vec<Vector2>, tolerance: f64) -> Result<PointListResult, Error> {
        call(json!({ "op": "Curve.simplify", "points": points, "tolerance": tolerance }))
    }

    /// The unit normal at `t`.
    pub fn curve_normal(curve: CurveSpec, t: f64) -> Result<Vector2, Error> {
        call(json!({ "op": "Curve.normal", "curve": curve, "t": t }))
    }

    /// The curvature at `t`.
    pub fn curve_curvature(curve: CurveSpec, t: f64) -> Result<f64, Error> {
        Ok(call::<ScalarResult>(json!({ "op": "Curve.curvature", "curve": curve, "t": t }))?.value)
    }

    /// The distance from `point` to the curve.
    pub fn curve_distance_to(curve: CurveSpec, point: Vector2) -> Result<f64, Error> {
        Ok(call::<ScalarResult>(
            json!({ "op": "Curve.distanceTo", "curve": curve, "point": point }),
        )?
        .value)
    }

    /// The polyline offset along the curve's local normal.
    pub fn curve_offset(curve: CurveSpec, distance: f64) -> Result<PointListResult, Error> {
        call(json!({ "op": "Curve.offset", "curve": curve, "distance": distance }))
    }

    /// Points along a circular arc.
    pub fn curve_arc(
        center: Vector2,
        radius: f64,
        start_angle: f64,
        sweep_angle: f64,
        count: usize,
    ) -> Result<PointListResult, Error> {
        call(json!({
            "op": "Curve.arc",
            "center": center,
            "radius": radius,
            "startAngle": start_angle,
            "sweepAngle": sweep_angle,
            "count": count,
        }))
    }

    /// Points around an ellipse.
    pub fn curve_ellipse(
        center: Vector2,
        radius_x: f64,
        radius_y: f64,
        count: usize,
    ) -> Result<PointListResult, Error> {
        call(json!({
            "op": "Curve.ellipse",
            "center": center,
            "radiusX": radius_x,
            "radiusY": radius_y,
            "count": count,
        }))
    }

    /// The intersection of two line segments, if any.
    pub fn line_intersect(
        a1: Vector2,
        a2: Vector2,
        b1: Vector2,
        b2: Vector2,
    ) -> Result<OptionalPointResult, Error> {
        call(json!({ "op": "Curve.lineIntersect", "a1": a1, "a2": a2, "b1": b1, "b2": b2 }))
    }

    /// Diagnoses an `.osb` storyboard.
    pub fn diagnose_osb(text: &str) -> Result<DiagnosticReport, Error> {
        call(json!({ "op": "Diagnostics.diagnoseOsb", "text": text }))
    }

    /// Diagnoses an `.osu` beatmap.
    pub fn diagnose_osu(text: &str) -> Result<DiagnosticReport, Error> {
        call(json!({ "op": "Diagnostics.diagnoseOsu", "text": text }))
    }

    /// Optimizes an `.osb` storyboard, returning the optimized text and stats.
    pub fn optimize_osb(
        text: &str,
        options: OptimizeOptionsInput,
    ) -> Result<OptimizeResult, Error> {
        call(json!({ "op": "Optimize.optimizeOsb", "text": text, "options": options }))
    }

    /// Reads image metadata.
    pub fn image_info(path: &str) -> Result<ImageInfo, Error> {
        call(json!({ "op": "Image.info", "path": path }))
    }

    /// Processes an image (read `path`, apply `options`, write `output`).
    pub fn image_process(
        path: &str,
        output: &str,
        options: ImageOptions,
    ) -> Result<ImageInfo, Error> {
        call(json!({ "op": "Image.process", "path": path, "output": output, "options": options }))
    }

    /// Renders primitive shapes to a new image.
    pub fn render_shapes(request: RenderShapesRequest) -> Result<ImageInfo, Error> {
        let mut value = serde_json::to_value(request)?;
        value["op"] = json!("Imagegen.renderShapes");
        call(value)
    }

    /// Probes a media file for its duration, dimensions and codecs.
    pub fn media_info(path: &str) -> Result<MediaInfo, Error> {
        call(json!({ "op": "Media.info", "path": path }))
    }

    /// Extracts frames from a video into `output_dir`.
    pub fn extract_frames(request: ExtractFramesRequest) -> Result<FrameResult, Error> {
        let mut value = serde_json::to_value(request)?;
        value["op"] = json!("Media.extractFrames");
        call(value)
    }

    /// Encodes an image sequence into a video.
    pub fn encode_video(request: EncodeVideoRequest) -> Result<MediaInfo, Error> {
        let mut value = serde_json::to_value(request)?;
        value["op"] = json!("Media.encodeVideo");
        call(value)
    }

    /// Extracts the audio track of a media file.
    pub fn extract_audio(input: &str, output: &str) -> Result<(), Error> {
        let _ = call::<serde_json::Value>(
            json!({ "op": "Media.extractAudio", "input": input, "output": output }),
        )?;
        Ok(())
    }

    /// `OsbSprite.GetOriginVector`.
    pub fn get_origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Result<Vector2, Error> {
        call(
            json!({ "op": "Origin.getOriginVector", "origin": origin, "width": width, "height": height }),
        )
    }

    /// `OsbSprite.InScreenBounds`.
    pub fn in_screen_bounds(
        position: Vector2,
        size: Vector2,
        rotation: f64,
        origin: Vector2,
    ) -> Result<bool, Error> {
        Ok(call::<BoolResult>(json!({
            "op": "Origin.inScreenBounds", "position": position, "size": size,
            "rotation": rotation, "origin": origin
        }))?
        .value)
    }
}

/// Runs a stateless JSON request against the native bridge.
pub(crate) fn call<T: DeserializeOwned>(request: Value) -> Result<T, Error> {
    parse(&osb_native::exec(&request.to_string()))
}

/// Parses a bridge response, mapping the error envelope to [`Error::Bridge`].
pub(crate) fn parse<T: DeserializeOwned>(response: &str) -> Result<T, Error> {
    let value: Value = serde_json::from_str(response)?;
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        return Err(Error::Bridge(error.to_string()));
    }
    Ok(serde_json::from_value(value)?)
}
