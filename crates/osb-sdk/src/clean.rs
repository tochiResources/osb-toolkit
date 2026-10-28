//! The clean high-level Rust clients: one class per contract tag with
//! `<X>Options` inputs, mirroring the generated clean clients in the other SDKs.
//!
//! The runtime classes stay the hand-written [`crate::Runtime`]/[`crate::Segment`]
//! surface; these clients cover the stateless core and library operations over
//! `crate::Client`.

use serde_json::{json, Value};

use crate::error::Error;
use crate::models;

/// The common surface (`Common.version`).
pub struct Common;

impl Common {
    /// The native runtime version string.
    pub fn version(&self) -> Result<String, Error> {
        crate::Client::version()
    }
}

/// A curve query.
pub struct CurveOptions {
    pub curve: models::CurveSpec,
}

/// A curve query at a parametric or arc-length position.
pub struct CurvePointOptions {
    pub curve: models::CurveSpec,
    pub t: f64,
    pub by_arc_length: Option<bool>,
}

/// A curve sampling request.
pub struct CurveSampleOptions {
    pub count: i32,
    pub curve: models::CurveSpec,
    pub by_arc_length: Option<bool>,
}

/// A curve hit-test request.
pub struct CurveDistanceOptions {
    pub curve: models::CurveSpec,
    pub point: models::Vector2,
}

/// A curve offset request.
pub struct CurveOffsetOptions {
    pub curve: models::CurveSpec,
    pub distance: f64,
}

/// A circular-arc sampling request.
pub struct ArcOptions {
    pub center: models::Vector2,
    pub radius: f64,
    pub start_angle: f64,
    pub sweep_angle: f64,
    pub count: Option<i32>,
}

/// An ellipse sampling request.
pub struct EllipseOptions {
    pub center: models::Vector2,
    pub radius_x: f64,
    pub radius_y: f64,
    pub count: Option<i32>,
}

/// A line-segment intersection request.
pub struct LineIntersectOptions {
    pub a1: models::Vector2,
    pub a2: models::Vector2,
    pub b1: models::Vector2,
    pub b2: models::Vector2,
}

/// A polyline simplification request.
pub struct SimplifyOptions {
    pub points: Vec<models::Vector2>,
    pub tolerance: f64,
}

/// The curve library.
pub struct Curve;

impl Curve {
    pub fn length(&self, options: CurveOptions) -> Result<f64, Error> {
        crate::Client::curve_length(options.curve)
    }

    pub fn bounding_box(&self, options: CurveOptions) -> Result<models::Box2, Error> {
        crate::Client::curve_bounding_box(options.curve)
    }

    pub fn point(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_point(
            options.curve,
            options.t,
            options.by_arc_length.unwrap_or(false),
        )
    }

    pub fn tangent(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_tangent(options.curve, options.t)
    }

    pub fn sample(&self, options: CurveSampleOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_sample(
            options.curve,
            options.count as usize,
            options.by_arc_length.unwrap_or(false),
        )
    }

    pub fn simplify(&self, options: SimplifyOptions) -> Result<models::PointListResult, Error> {
        crate::Client::simplify(options.points, options.tolerance)
    }

    pub fn normal(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_normal(options.curve, options.t)
    }

    pub fn curvature(&self, options: CurvePointOptions) -> Result<f64, Error> {
        crate::Client::curve_curvature(options.curve, options.t)
    }

    pub fn distance_to(&self, options: CurveDistanceOptions) -> Result<f64, Error> {
        crate::Client::curve_distance_to(options.curve, options.point)
    }

    pub fn offset(&self, options: CurveOffsetOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_offset(options.curve, options.distance)
    }

    pub fn arc(&self, options: ArcOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_arc(
            options.center,
            options.radius,
            options.start_angle,
            options.sweep_angle,
            options.count.unwrap_or(64).max(0) as usize,
        )
    }

    pub fn ellipse(&self, options: EllipseOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_ellipse(
            options.center,
            options.radius_x,
            options.radius_y,
            options.count.unwrap_or(64).max(0) as usize,
        )
    }

    pub fn line_intersect(
        &self,
        options: LineIntersectOptions,
    ) -> Result<models::OptionalPointResult, Error> {
        crate::Client::line_intersect(options.a1, options.a2, options.b1, options.b2)
    }
}

/// A named easing evaluation request.
pub struct ApplyEasingOptions {
    pub easing: models::Easing,
    pub t: f64,
}

/// A named-or-custom easing evaluation request.
pub struct EasingValueOptions {
    pub t: f64,
    pub easing: Option<models::Easing>,
    pub cubic_bezier: Option<Vec<f64>>,
}

/// The animations library.
pub struct Animations;

impl Animations {
    pub fn apply_easing(&self, options: ApplyEasingOptions) -> Result<f64, Error> {
        crate::Client::apply_easing(options.easing, options.t)
    }

    pub fn easing_value(&self, options: EasingValueOptions) -> Result<f64, Error> {
        crate::Client::easing_value(options.t, options.easing, options.cubic_bezier)
    }
}

/// A request carrying a block of text.
pub struct TextOptions {
    pub text: String,
}

/// The stateless parsers.
pub struct Parsing;

impl Parsing {
    pub fn parse_osu(&self, options: TextOptions) -> Result<models::Beatmap, Error> {
        crate::Client::parse_beatmap(&options.text)
    }

    pub fn parse_osb(&self, options: TextOptions) -> Result<models::ParseOsbResult, Error> {
        crate::Client::parse_osb(&options.text)
    }
}

/// A request to resolve an origin's anchor offset.
pub struct GetOriginVectorOptions {
    pub origin: models::OsbOrigin,
    pub width: f64,
    pub height: f64,
}

/// Whether a rotated sprite's bounding box intersects the screen.
pub struct InScreenBoundsOptions {
    pub position: models::Vector2,
    pub size: models::Vector2,
    pub rotation: f64,
    pub origin: models::Vector2,
}

/// The stateless origin queries.
pub struct Origin;

impl Origin {
    pub fn get_origin_vector(
        &self,
        options: GetOriginVectorOptions,
    ) -> Result<models::Vector2, Error> {
        crate::Client::get_origin_vector(options.origin, options.width, options.height)
    }

    pub fn in_screen_bounds(&self, options: InScreenBoundsOptions) -> Result<bool, Error> {
        crate::Client::in_screen_bounds(
            options.position,
            options.size,
            options.rotation,
            options.origin,
        )
    }
}

/// An FFT request.
pub struct FftOptions {
    pub path: String,
    pub time: Option<f64>,
    pub magnitudes: Option<i32>,
}

/// The audio library.
pub struct Audio;

impl Audio {
    pub fn fft(&self, options: FftOptions) -> Result<models::FftResult, Error> {
        crate::Client::fft(
            &options.path,
            options.time.unwrap_or(0.0),
            options.magnitudes.unwrap_or(64).max(0) as usize,
        )
    }
}

/// The diagnostics library.
pub struct Diagnostics;

impl Diagnostics {
    pub fn diagnose_osb(&self, options: TextOptions) -> Result<models::DiagnosticReport, Error> {
        crate::Client::diagnose_osb(&options.text)
    }

    pub fn diagnose_osu(&self, options: TextOptions) -> Result<models::DiagnosticReport, Error> {
        crate::Client::diagnose_osu(&options.text)
    }
}

/// An optimization request.
pub struct OptimizeOptions {
    pub text: String,
    pub options: models::OptimizeOptionsInput,
}

/// The optimization library.
pub struct Optimize;

impl Optimize {
    pub fn optimize_osb(&self, options: OptimizeOptions) -> Result<models::OptimizeResult, Error> {
        crate::Client::optimize_osb(&options.text, options.options)
    }
}

/// A request carrying a single file path.
pub struct PathOptions {
    pub path: String,
}

/// An image processing request.
pub struct ImageProcessOptions {
    pub options: models::ImageOptions,
    pub output: String,
    pub path: String,
}

/// The image library.
pub struct Image;

impl Image {
    pub fn info(&self, options: PathOptions) -> Result<models::ImageInfo, Error> {
        crate::Client::image_info(&options.path)
    }

    pub fn process(&self, options: ImageProcessOptions) -> Result<models::ImageInfo, Error> {
        crate::Client::image_process(&options.path, &options.output, options.options)
    }
}

/// A primitive-shape render request.
pub struct RenderShapesOptions {
    pub width: i32,
    pub height: i32,
    pub shapes: Vec<models::Shape>,
    pub output: String,
    pub background: Option<models::RgbaColor>,
}

/// The image-generation library.
pub struct Imagegen;

impl Imagegen {
    pub fn render_shapes(&self, options: RenderShapesOptions) -> Result<models::ImageInfo, Error> {
        crate::Client::render_shapes(models::RenderShapesRequest {
            background: options.background.map(|value| Some(Box::new(value))),
            height: options.height,
            output: options.output,
            shapes: options.shapes,
            width: options.width,
        })
    }
}

/// A frame-extraction request.
pub struct ExtractFramesOptions {
    pub input: String,
    pub output_dir: String,
    pub count: Option<i32>,
    pub fps: Option<f64>,
    pub height: Option<i32>,
    pub start: Option<f64>,
    pub width: Option<i32>,
}

/// A video-encoding request.
pub struct EncodeVideoOptions {
    pub input_pattern: String,
    pub output: String,
    pub fps: f64,
    pub codec: Option<String>,
    pub crf: Option<i32>,
}

/// An audio-extraction request.
pub struct ExtractAudioOptions {
    pub input: String,
    pub output: String,
}

/// The media library.
pub struct Media;

impl Media {
    pub fn info(&self, options: PathOptions) -> Result<models::MediaInfo, Error> {
        crate::Client::media_info(&options.path)
    }

    pub fn extract_frames(
        &self,
        options: ExtractFramesOptions,
    ) -> Result<models::FrameResult, Error> {
        crate::Client::extract_frames(models::ExtractFramesRequest {
            count: options.count.map(Some),
            fps: options.fps.map(Some),
            height: options.height.map(Some),
            input: options.input,
            output_dir: options.output_dir,
            start: options.start.map(Some),
            width: options.width.map(Some),
        })
    }

    pub fn encode_video(&self, options: EncodeVideoOptions) -> Result<models::MediaInfo, Error> {
        crate::Client::encode_video(models::EncodeVideoRequest {
            codec: options.codec.map(Some),
            crf: options.crf.map(Some),
            fps: options.fps,
            input_pattern: options.input_pattern,
            output: options.output,
        })
    }

    pub fn extract_audio(&self, options: ExtractAudioOptions) -> Result<(), Error> {
        crate::Client::extract_audio(&options.input, &options.output)
    }
}

/// A 3D bake request.
pub struct Bake3dOptions {
    pub input: models::Scene3dInput,
}

/// The 3D storyboard library.
pub struct Storyboarding3d;

impl Storyboarding3d {
    pub fn bake3d(&self, options: Bake3dOptions) -> Result<models::BakedStoryboard, Error> {
        crate::Client::bake3d(options.input)
    }
}

/// A motion-graphics helper call.
pub struct HelperOptions {
    pub name: String,
    pub input: Option<Value>,
}

/// The shared motion-graphics helper surface.
pub struct Helpers;

impl Helpers {
    pub fn call(&self, options: HelperOptions) -> Result<Value, Error> {
        helper("Helpers.call", options)
    }

    pub fn commands(&self, options: HelperOptions) -> Result<Value, Error> {
        helper("Helpers.commands", options)
    }

    pub fn keyframes(&self, options: HelperOptions) -> Result<Value, Error> {
        helper("Helpers.keyframes", options)
    }
}

fn helper(op: &str, options: HelperOptions) -> Result<Value, Error> {
    let result: models::HelperResult = crate::client::call(json!({
        "op": op,
        "name": options.name,
        "input": options.input.unwrap_or(Value::Object(Default::default())),
    }))?;
    Ok(result.value.unwrap_or(Value::Null))
}

/// The composed stateless clean client, mirroring the other SDKs' entry points.
pub struct Client {
    pub common: Common,
    pub curve: Curve,
    pub animations: Animations,
    pub parsing: Parsing,
    pub origin: Origin,
    pub audio: Audio,
    pub diagnostics: Diagnostics,
    pub optimize: Optimize,
    pub image: Image,
    pub image_gen: Imagegen,
    pub media: Media,
    pub storyboarding3d: Storyboarding3d,
    pub helpers: Helpers,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            common: Common,
            curve: Curve,
            animations: Animations,
            parsing: Parsing,
            origin: Origin,
            audio: Audio,
            diagnostics: Diagnostics,
            optimize: Optimize,
            image: Image,
            image_gen: Imagegen,
            media: Media,
            storyboarding3d: Storyboarding3d,
            helpers: Helpers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drives_the_clean_core_and_library_clients() {
        let client = Client::new();
        assert!(!client.common.version().unwrap().is_empty());
        assert_eq!(
            client
                .animations
                .apply_easing(ApplyEasingOptions {
                    easing: models::Easing::InQuad,
                    t: 0.5,
                })
                .unwrap(),
            0.25
        );

        let curve = models::CurveSpec {
            kind: models::CurveKind::Linear,
            points: vec![
                models::Vector2 { x: 0.0, y: 0.0 },
                models::Vector2 { x: 100.0, y: 0.0 },
            ],
        };
        assert_eq!(
            client
                .curve
                .length(CurveOptions {
                    curve: curve.clone()
                })
                .unwrap(),
            100.0
        );
        assert_eq!(
            client
                .curve
                .point(CurvePointOptions {
                    curve: curve.clone(),
                    t: 0.5,
                    by_arc_length: None,
                })
                .unwrap()
                .x,
            50.0
        );
        assert_eq!(
            client
                .curve
                .sample(CurveSampleOptions {
                    count: 3,
                    curve,
                    by_arc_length: None,
                })
                .unwrap()
                .points
                .len(),
            3
        );
    }

    #[test]
    fn accepts_structured_helper_input() {
        let client = Client::new();
        let result = client
            .helpers
            .call(HelperOptions {
                name: "keyframe.scaleKeyframes".to_string(),
                input: Some(json!({
                    "channel": {
                        "keys": [
                            {"time": 0.0, "value": [1.0, 2.0]},
                            {"time": 100.0, "value": [3.0, 4.0]}
                        ]
                    },
                    "factor": 2.0
                })),
            })
            .unwrap();
        assert_eq!(result["keys"][1]["value"][0], 6.0);
        assert_eq!(result["keys"][1]["value"][1], 8.0);
    }
}
