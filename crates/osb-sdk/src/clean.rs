//! The clean high-level Rust clients: one class per contract tag with
//! `<X>Options` inputs, mirroring the generated clean clients in the other SDKs.
//!
//! The stateless classes wrap `crate::Client`; [`Runtime`] owns a live
//! [`crate::Runtime`] and exposes the handle-bound `Generator`, `Layer` and
//! `Sprite` clients.

use serde_json::{json, Value};

use crate::error::Error;
use crate::models;

/// An easing evaluation request.
#[derive(Clone, Debug)]
pub struct ApplyEasingOptions {
    /// The easing function.
    pub easing: models::Easing,
    /// The normalized time in `0.0..=1.0`.
    pub t: f64,
}

/// A circular-arc sampling request.
#[derive(Clone, Debug)]
pub struct ArcOptions {
    /// The arc centre.
    pub center: models::Vector2,
    /// The radius.
    pub radius: f64,
    /// The start angle in degrees.
    pub start_angle: f64,
    /// The sweep angle in degrees.
    pub sweep_angle: f64,
    /// The number of points (default `64`).
    pub count: Option<i32>,
}

/// A `bake3d` request: a plain 3D scene.
#[derive(Clone, Debug)]
pub struct Bake3dOptions {
    /// The 3D scene.
    pub input: models::Scene3dInput,
}

/// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
#[derive(Clone, Debug)]
pub struct CommandOptions {
    /// The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
    pub kind: String,
    /// The command start time in milliseconds.
    pub start_time: f64,
    /// The command end time in milliseconds.
    pub end_time: f64,
    /// The easing function.
    pub easing: models::Easing,
    /// The start value (scalar, point or color).
    pub start: Option<serde_json::Value>,
    /// The end value (scalar, point or color).
    pub end: Option<serde_json::Value>,
    /// The parameter kind for a `parameter` command.
    pub parameter: Option<models::ParameterType>,
}

/// Create an animation sprite.
#[derive(Clone, Debug)]
pub struct CreateAnimationOptions {
    /// The texture path (relative to the mapset).
    pub path: String,
    /// The number of frames.
    pub frames: i32,
    /// Delay between frames, in milliseconds.
    pub frame_delay: f64,
    /// How the animation loops.
    pub loop_type: models::OsbLoopType,
    /// The sprite anchor origin.
    pub origin: models::OsbOrigin,
    /// The initial position.
    pub position: models::Vector2,
    /// The target layer name (omitted when `layerHandle` is set).
    pub layer: Option<String>,
}

/// Create a sound sample on a layer.
#[derive(Clone, Debug)]
pub struct CreateSampleOptions {
    /// The sample path (relative to the mapset).
    pub path: String,
    /// The sample time in milliseconds.
    pub time: f64,
    /// The sample volume (`0..=100`).
    pub volume: Option<f64>,
    /// The target layer name (omitted when `layerHandle` is set).
    pub layer: Option<String>,
}

/// Create a sprite.
#[derive(Clone, Debug)]
pub struct CreateSpriteOptions {
    /// The texture path (relative to the mapset).
    pub path: String,
    /// The sprite anchor origin.
    pub origin: models::OsbOrigin,
    /// The initial position; omitted means the sprite's default.
    pub position: Option<models::Vector2>,
    /// The target layer name (omitted when `layerHandle` is set).
    pub layer: Option<String>,
}

/// A curve hit-test request.
#[derive(Clone, Debug)]
pub struct CurveDistanceOptions {
    /// The curve.
    pub curve: models::CurveSpec,
    /// The query point.
    pub point: models::Vector2,
}

/// A curve offset request.
#[derive(Clone, Debug)]
pub struct CurveOffsetOptions {
    /// The curve.
    pub curve: models::CurveSpec,
    /// The signed offset along the curve's local normal.
    pub distance: f64,
}

/// A curve query.
#[derive(Clone, Debug)]
pub struct CurveOptions {
    /// The curve.
    pub curve: models::CurveSpec,
}

/// A curve query at a parametric or arc-length position.
#[derive(Clone, Debug)]
pub struct CurvePointOptions {
    /// The curve.
    pub curve: models::CurveSpec,
    /// The position in `0.0..=1.0`.
    pub t: f64,
    /// Sample by arc length instead of the raw parameter (default `false`).
    pub by_arc_length: Option<bool>,
}

/// A curve sampling request.
#[derive(Clone, Debug)]
pub struct CurveSampleOptions {
    /// The number of points to sample.
    pub count: i32,
    /// The curve.
    pub curve: models::CurveSpec,
    /// Sample by arc length instead of the raw parameter (default `false`).
    pub by_arc_length: Option<bool>,
}

/// A named-or-custom easing evaluation request.
#[derive(Clone, Debug)]
pub struct EasingValueOptions {
    /// The normalized time in `0.0..=1.0`.
    pub t: f64,
    /// A named easing (used when `cubicBezier` is absent).
    pub easing: Option<models::Easing>,
    /// A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
    pub cubic_bezier: Option<Vec<f64>>,
}

/// An ellipse sampling request.
#[derive(Clone, Debug)]
pub struct EllipseOptions {
    /// The ellipse centre.
    pub center: models::Vector2,
    /// The horizontal radius.
    pub radius_x: f64,
    /// The vertical radius.
    pub radius_y: f64,
    /// The number of points (default `64`).
    pub count: Option<i32>,
}

/// An image-sequence encoding request (frames -> video).
#[derive(Clone, Debug)]
pub struct EncodeVideoOptions {
    /// The input filename pattern (e.g. `frames/frame_%04d.png`).
    pub input_pattern: String,
    /// The output video path.
    pub output: String,
    /// The frame rate.
    pub fps: f64,
    /// The video codec (default `libx264`).
    pub codec: Option<String>,
    /// The constant-rate-factor quality (`0`-`51`; lower is better).
    pub crf: Option<i32>,
}

/// An audio-extraction request (media -> audio file).
#[derive(Clone, Debug)]
pub struct ExtractAudioOptions {
    /// The input media path.
    pub input: String,
    /// The output audio path (its extension selects the format).
    pub output: String,
}

/// A frame-extraction request (video -> image sequence).
#[derive(Clone, Debug)]
pub struct ExtractFramesOptions {
    /// The input media path.
    pub input: String,
    /// The directory to write frames into.
    pub output_dir: String,
    /// The maximum number of frames to extract.
    pub count: Option<i32>,
    /// The sampling rate (frames per second); defaults to the source rate.
    pub fps: Option<f64>,
    /// Scale frames to this height.
    pub height: Option<i32>,
    /// The start time in seconds.
    pub start: Option<f64>,
    /// Scale frames to this width.
    pub width: Option<i32>,
}

/// An FFT request against a `.wav` file.
#[derive(Clone, Debug)]
pub struct FftOptions {
    /// Path to the audio file.
    pub path: String,
    /// Number of frequency bins.
    pub magnitudes: Option<i32>,
    /// Sample time in milliseconds.
    pub time: Option<f64>,
}

/// An FFT request against the runtime's loaded audio.
#[derive(Clone, Debug)]
pub struct GetFftOptions {
    /// Sample time in milliseconds.
    pub time: f64,
    /// Number of frequency bins.
    pub magnitudes: i32,
    /// A frequency cut-off.
    pub cutoff: Option<f64>,
    /// The easing applied across the bins.
    pub easing: Option<models::Easing>,
}

/// The anchor offset for an origin.
#[derive(Clone, Debug)]
pub struct GetOriginVectorOptions {
    /// The sprite anchor origin.
    pub origin: models::OsbOrigin,
    /// The sprite width.
    pub width: f64,
    /// The sprite height.
    pub height: f64,
}

/// A motion-graphics helper call: a helper `name` plus its input.
#[derive(Clone, Debug)]
pub struct HelperOptions {
    /// The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
    pub name: String,
    /// The helper's input (a keyframe channel, a command list, or its parameters).
    pub input: Option<serde_json::Value>,
}

/// An image processing request: read `path`, apply `options`, write `output`.
#[derive(Clone, Debug)]
pub struct ImageProcessOptions {
    /// The processing options.
    pub options: models::ImageOptions,
    /// The output file path (its extension selects the format).
    pub output: String,
    /// The input file path.
    pub path: String,
}

/// Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
#[derive(Clone, Debug)]
pub struct InScreenBoundsOptions {
    /// The sprite centre position.
    pub position: models::Vector2,
    /// The sprite size.
    pub size: models::Vector2,
    /// The rotation in radians.
    pub rotation: f64,
    /// The anchor offset (from `GetOriginVector`).
    pub origin: models::Vector2,
}

/// Resolve (or create) a layer by name.
#[derive(Clone, Debug)]
pub struct LayerHandleOptions {
    /// The layer name.
    pub name: String,
}

/// A line-segment intersection request.
#[derive(Clone, Debug)]
pub struct LineIntersectOptions {
    /// The first segment's start.
    pub a1: models::Vector2,
    /// The first segment's end.
    pub a2: models::Vector2,
    /// The second segment's start.
    pub b1: models::Vector2,
    /// The second segment's end.
    pub b2: models::Vector2,
}

/// An `.osb` optimization request.
#[derive(Clone, Debug)]
pub struct OptimizeOptions {
    /// The optimization options.
    pub options: models::OptimizeOptionsInput,
    /// The `.osb` storyboard text.
    pub text: String,
}

/// A request carrying a single file path.
#[derive(Clone, Debug)]
pub struct PathOptions {
    /// The input file path.
    pub path: String,
}

/// A deterministic random request.
#[derive(Clone, Debug)]
pub struct RandomOptions {
    /// The exclusive upper bound.
    pub max: f64,
    /// The inclusive lower bound.
    pub min: f64,
}

/// A shape-rendering request: a canvas, a background and a shape list.
#[derive(Clone, Debug)]
pub struct RenderShapesOptions {
    /// The canvas width in pixels.
    pub width: i32,
    /// The canvas height in pixels.
    pub height: i32,
    /// The shapes to draw, in order.
    pub shapes: Vec<models::Shape>,
    /// The output file path (its extension selects the format).
    pub output: String,
    /// The background colour (default transparent).
    pub background: Option<models::RgbaColor>,
}

/// Record a depth index on a segment.
#[derive(Clone, Debug)]
pub struct SegmentAddIndexOptions {
    /// The index to record.
    pub index: f64,
}

/// Move a segment to another osu! layer (`ChangePosition`).
#[derive(Clone, Debug)]
pub struct SegmentChangePositionOptions {
    /// The destination osu! layer.
    pub osb_layer: models::OsbLayer,
}

/// Set a segment's reverse-depth flag.
#[derive(Clone, Debug)]
pub struct SegmentReverseDepthOptions {
    /// Whether children are emitted in reverse order.
    pub value: bool,
}

/// A Douglas-Peucker simplification request.
#[derive(Clone, Debug)]
pub struct SimplifyOptions {
    /// The polyline to simplify.
    pub points: Vec<models::Vector2>,
    /// The maximum deviation to allow.
    pub tolerance: f64,
}

/// A sprite handle pair plus a sample time.
#[derive(Clone, Debug)]
pub struct SpriteTimeOptions {
    /// The sample time in milliseconds.
    pub time: f64,
}

/// Start a loop group.
#[derive(Clone, Debug)]
pub struct StartLoopGroupOptions {
    /// The loop start time in milliseconds.
    pub start_time: f64,
    /// The number of iterations.
    pub count: f64,
}

/// Start a trigger group.
#[derive(Clone, Debug)]
pub struct StartTriggerGroupOptions {
    /// The trigger name (matches a hit object).
    pub name: String,
    /// The trigger start time in milliseconds.
    pub start_time: f64,
    /// The trigger end time in milliseconds.
    pub end_time: f64,
    /// The trigger group index.
    pub group: i32,
}

/// A request carrying a block of text (an effect script, `.osu` or `.osb`).
#[derive(Clone, Debug)]
pub struct TextOptions {
    /// The text payload.
    pub text: String,
}

/// A time query.
#[derive(Clone, Debug)]
pub struct TimeOptions {
    /// The time in milliseconds.
    pub time: f64,
}

/// The clean `Core` client.
pub struct Core;

impl Core {
    pub fn apply_easing(&self, options: ApplyEasingOptions) -> Result<f64, Error> {
        crate::Client::apply_easing(options.easing, options.t)
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

    pub fn bounding_box(&self, options: CurveOptions) -> Result<models::Box2, Error> {
        crate::Client::curve_bounding_box(options.curve)
    }

    pub fn curvature(&self, options: CurvePointOptions) -> Result<f64, Error> {
        crate::Client::curve_curvature(options.curve, options.t)
    }

    pub fn distance_to(&self, options: CurveDistanceOptions) -> Result<f64, Error> {
        crate::Client::curve_distance_to(options.curve, options.point)
    }

    pub fn easing_value(&self, options: EasingValueOptions) -> Result<f64, Error> {
        crate::Client::easing_value(options.t, options.easing, options.cubic_bezier)
    }

    pub fn ellipse(&self, options: EllipseOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_ellipse(
            options.center,
            options.radius_x,
            options.radius_y,
            options.count.unwrap_or(64).max(0) as usize,
        )
    }

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

    pub fn length(&self, options: CurveOptions) -> Result<f64, Error> {
        crate::Client::curve_length(options.curve)
    }

    pub fn line_intersect(
        &self,
        options: LineIntersectOptions,
    ) -> Result<models::OptionalPointResult, Error> {
        crate::Client::line_intersect(options.a1, options.a2, options.b1, options.b2)
    }

    pub fn normal(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_normal(options.curve, options.t)
    }

    pub fn offset(&self, options: CurveOffsetOptions) -> Result<models::PointListResult, Error> {
        crate::Client::curve_offset(options.curve, options.distance)
    }

    pub fn parse_osb(&self, options: TextOptions) -> Result<models::ParseOsbResult, Error> {
        crate::Client::parse_osb(&options.text)
    }

    pub fn parse_osu(&self, options: TextOptions) -> Result<models::Beatmap, Error> {
        crate::Client::parse_beatmap(&options.text)
    }

    pub fn point(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_point(
            options.curve,
            options.t,
            options.by_arc_length.unwrap_or(false),
        )
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

    pub fn tangent(&self, options: CurvePointOptions) -> Result<models::Vector2, Error> {
        crate::Client::curve_tangent(options.curve, options.t)
    }

    pub fn version(&self) -> Result<String, Error> {
        crate::Client::version()
    }
}

/// The clean `Diagnostics` client.
pub struct Diagnostics;

impl Diagnostics {
    pub fn diagnose_osb(&self, options: TextOptions) -> Result<models::DiagnosticReport, Error> {
        crate::Client::diagnose_osb(&options.text)
    }

    pub fn diagnose_osu(&self, options: TextOptions) -> Result<models::DiagnosticReport, Error> {
        crate::Client::diagnose_osu(&options.text)
    }
}

/// The clean `Helpers` client.
pub struct Helpers;

impl Helpers {
    pub fn call(&self, options: HelperOptions) -> Result<serde_json::Value, Error> {
        helper("Helpers.call", options)
    }

    pub fn commands(&self, options: HelperOptions) -> Result<serde_json::Value, Error> {
        helper("Helpers.commands", options)
    }

    pub fn keyframes(&self, options: HelperOptions) -> Result<serde_json::Value, Error> {
        helper("Helpers.keyframes", options)
    }
}

/// The clean `Image` client.
pub struct Image;

impl Image {
    pub fn info(&self, options: PathOptions) -> Result<models::ImageInfo, Error> {
        crate::Client::image_info(&options.path)
    }

    pub fn process(&self, options: ImageProcessOptions) -> Result<models::ImageInfo, Error> {
        crate::Client::image_process(&options.path, &options.output, options.options)
    }
}

/// The clean `Imagegen` client.
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

/// The clean `Media` client.
pub struct Media;

impl Media {
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

    pub fn info(&self, options: PathOptions) -> Result<models::MediaInfo, Error> {
        crate::Client::media_info(&options.path)
    }
}

/// The clean `Optimize` client.
pub struct Optimize;

impl Optimize {
    pub fn optimize_osb(&self, options: OptimizeOptions) -> Result<models::OptimizeResult, Error> {
        crate::Client::optimize_osb(&options.text, options.options)
    }
}

/// The clean `Storyboarding3d` client.
pub struct Storyboarding3d;

impl Storyboarding3d {
    pub fn bake3d(&self, options: Bake3dOptions) -> Result<models::BakedStoryboard, Error> {
        crate::Client::bake3d(options.input)
    }
}

/// The clean `Audio` client. `fft` is stateless; `get_fft` and
/// `audio_duration` need a live runtime.
pub struct Audio<'a> {
    runtime: Option<&'a mut crate::Runtime>,
}

impl<'a> Audio<'a> {
    pub fn audio_duration(&mut self) -> Result<f64, Error> {
        let runtime = self
            .runtime
            .as_mut()
            .ok_or_else(|| Error::Bridge("Audio.audioDuration needs a runtime".to_string()))?;
        let result: models::AudioDurationResult = runtime.exec(json!({
            "op": "Audio.audioDuration"
        }))?;
        Ok(result.audio_duration)
    }

    pub fn fft(&self, options: FftOptions) -> Result<models::FftResult, Error> {
        crate::Client::fft(
            &options.path,
            options.time.unwrap_or(0.0),
            options.magnitudes.unwrap_or(64).max(0) as usize,
        )
    }

    pub fn get_fft(&mut self, options: GetFftOptions) -> Result<models::FftResult, Error> {
        let runtime = self
            .runtime
            .as_mut()
            .ok_or_else(|| Error::Bridge("Audio.getFft needs a runtime".to_string()))?;
        let mut request = json!({
            "op": "Audio.getFft",
            "time": options.time,
            "magnitudes": options.magnitudes,
        });
        if let Some(cutoff) = options.cutoff {
            request["cutoff"] = json!(cutoff);
        }
        if let Some(easing) = options.easing {
            request["easing"] = json!(easing);
        }
        runtime.exec(request)
    }
}

/// The clean `Generator` client bound to a live runtime.
pub struct Generator<'a> {
    runtime: &'a mut crate::Runtime,
}

impl<'a> Generator<'a> {
    pub fn beatmap(&mut self) -> Result<Option<models::Beatmap>, Error> {
        self.runtime.exec(json!({ "op": "Generator.beatmap" }))
    }

    pub fn control_point_at(
        &mut self,
        options: TimeOptions,
    ) -> Result<Option<models::ControlPoint>, Error> {
        self.runtime
            .exec(json!({ "op": "Generator.controlPointAt", "time": options.time }))
    }

    pub fn import_osb(&mut self, options: TextOptions) -> Result<i32, Error> {
        let result: models::ImportOsbResult = self
            .runtime
            .exec(json!({ "op": "Generator.importOsb", "text": options.text }))?;
        Ok(result.sprite_count)
    }

    pub fn layer_handle(&mut self, options: LayerHandleOptions) -> Result<i32, Error> {
        let result: models::LayerHandle = self
            .runtime
            .exec(json!({ "op": "Generator.layerHandle", "name": options.name }))?;
        Ok(result.layer)
    }

    pub fn load_beatmap(
        &mut self,
        options: TextOptions,
    ) -> Result<models::LoadBeatmapResult, Error> {
        self.runtime
            .exec(json!({ "op": "Generator.loadBeatmap", "text": options.text }))
    }

    pub fn random(&mut self, options: RandomOptions) -> Result<f64, Error> {
        let result: models::RandomResult = self
            .runtime
            .exec(json!({ "op": "Generator.random", "max": options.max, "min": options.min }))?;
        Ok(result.value)
    }

    pub fn sprite_count(&mut self) -> Result<i32, Error> {
        let result: models::SpriteCountResult = self
            .runtime
            .exec(json!({ "op": "Generator.spriteCount" }))?;
        Ok(result.sprite_count)
    }

    pub fn storyboard(&mut self) -> Result<models::Storyboard, Error> {
        self.runtime.exec(json!({ "op": "Generator.storyboard" }))
    }

    pub fn timing_point_at(&mut self, options: TimeOptions) -> Result<models::ControlPoint, Error> {
        self.runtime
            .exec(json!({ "op": "Generator.timingPointAt", "time": options.time }))
    }
}

/// The clean `Layer` client bound to a live runtime.
pub struct Layer<'a> {
    runtime: &'a mut crate::Runtime,
    handle: models::LayerHandle,
}

impl<'a> Layer<'a> {
    /// The segment's handle.
    pub fn handle(&self) -> models::LayerHandle {
        self.handle.clone()
    }

    pub fn add_index(&mut self, options: SegmentAddIndexOptions) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(
            json!({ "op": "Layer.addIndex", "layer": self.handle.layer, "index": options.index }),
        )?;
        Ok(())
    }

    pub fn change_position(&mut self, options: SegmentChangePositionOptions) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.changePosition", "layer": self.handle.layer, "osbLayer": options.osb_layer }))?;
        Ok(())
    }

    pub fn clean(&mut self) -> Result<i32, Error> {
        let result: models::SegmentCleanResult = self
            .runtime
            .exec(json!({ "op": "Layer.clean", "layer": self.handle.layer }))?;
        Ok(result.removed)
    }

    pub fn create_animation(
        &mut self,
        options: CreateAnimationOptions,
    ) -> Result<Sprite<'_>, Error> {
        let handle: models::Handle = self.runtime.exec(json!({
            "op": "Layer.createAnimation",
            "layerHandle": self.handle.layer,
            "path": options.path,
            "frames": options.frames,
            "frameDelay": options.frame_delay,
            "loopType": options.loop_type,
            "origin": options.origin,
            "position": options.position,
        }))?;
        Ok(Sprite {
            runtime: &mut *self.runtime,
            handle,
        })
    }

    pub fn create_sample(&mut self, options: CreateSampleOptions) -> Result<(), Error> {
        let mut request = json!({
            "op": "Layer.createSample",
            "layerHandle": self.handle.layer,
            "path": options.path,
            "time": options.time,
            "volume": options.volume.unwrap_or(100.0),
        });
        if let Some(layer) = options.layer {
            request["layer"] = json!(layer);
        }
        let _: models::OkResult = self.runtime.exec(request)?;
        Ok(())
    }

    pub fn create_segment(&mut self) -> Result<Layer<'_>, Error> {
        let handle: models::LayerHandle = self
            .runtime
            .exec(json!({ "op": "Layer.createSegment", "layer": self.handle.layer }))?;
        Ok(Layer {
            runtime: &mut *self.runtime,
            handle,
        })
    }

    pub fn create_sprite(&mut self, options: CreateSpriteOptions) -> Result<Sprite<'_>, Error> {
        let mut request = json!({
            "op": "Layer.createSprite",
            "layerHandle": self.handle.layer,
            "path": options.path,
            "origin": options.origin,
        });
        if let Some(position) = options.position {
            request["position"] = json!(position);
        }
        if let Some(layer) = options.layer {
            request["layer"] = json!(layer);
        }
        let handle: models::Handle = self.runtime.exec(request)?;
        Ok(Sprite {
            runtime: &mut *self.runtime,
            handle,
        })
    }

    pub fn discard(&mut self) -> Result<i32, Error> {
        let result: models::SegmentDiscardResult = self
            .runtime
            .exec(json!({ "op": "Layer.discard", "layer": self.handle.layer }))?;
        Ok(result.layer_count)
    }

    pub fn discard_sprite(&mut self, sprite: i32) -> Result<bool, Error> {
        let result: models::BoolResult = self.runtime.exec(
            json!({ "op": "Layer.discardSprite", "layer": self.handle.layer, "sprite": sprite }),
        )?;
        Ok(result.value)
    }

    pub fn indexes(&mut self) -> Result<models::SegmentIndexesResult, Error> {
        self.runtime
            .exec(json!({ "op": "Layer.indexes", "layer": self.handle.layer }))
    }

    pub fn move_to_bottom(&mut self) -> Result<(), Error> {
        let _: models::OkResult = self
            .runtime
            .exec(json!({ "op": "Layer.moveToBottom", "layer": self.handle.layer }))?;
        Ok(())
    }

    pub fn move_to_top(&mut self) -> Result<(), Error> {
        let _: models::OkResult = self
            .runtime
            .exec(json!({ "op": "Layer.moveToTop", "layer": self.handle.layer }))?;
        Ok(())
    }

    pub fn reverse_depth(&mut self, options: SegmentReverseDepthOptions) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.reverseDepth", "layer": self.handle.layer, "value": options.value }))?;
        Ok(())
    }
}

/// The clean `Sprite` client bound to a live runtime.
pub struct Sprite<'a> {
    runtime: &'a mut crate::Runtime,
    handle: models::Handle,
}

impl<'a> Sprite<'a> {
    /// The sprite's layer handle.
    pub fn layer(&self) -> i32 {
        self.handle.layer
    }

    /// The sprite's index within its layer.
    pub fn sprite(&self) -> i32 {
        self.handle.sprite
    }

    pub fn command(&mut self, options: CommandOptions) -> Result<(), Error> {
        let mut request = json!({
            "op": "Sprite.command",
            "kind": options.kind,
            "layer": self.handle.layer,
            "sprite": self.handle.sprite,
            "startTime": options.start_time,
            "endTime": options.end_time,
            "easing": options.easing,
        });
        if let Some(start) = options.start {
            request["start"] = start;
        }
        if let Some(end) = options.end {
            request["end"] = end;
        }
        if let Some(parameter) = options.parameter {
            request["parameter"] = json!(parameter);
        }
        let _: models::OkResult = self.runtime.exec(request)?;
        Ok(())
    }

    pub fn end_group(&mut self) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.endGroup", "layer": self.handle.layer, "sprite": self.handle.sprite }))?;
        Ok(())
    }

    pub fn info(&mut self) -> Result<models::SpriteInfo, Error> {
        self.runtime.exec(json!({ "op": "Sprite.info", "layer": self.handle.layer, "sprite": self.handle.sprite }))
    }

    pub fn start_loop_group(&mut self, options: StartLoopGroupOptions) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.startLoopGroup", "layer": self.handle.layer, "sprite": self.handle.sprite, "startTime": options.start_time, "count": options.count }))?;
        Ok(())
    }

    pub fn start_trigger_group(&mut self, options: StartTriggerGroupOptions) -> Result<(), Error> {
        let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.startTriggerGroup", "layer": self.handle.layer, "sprite": self.handle.sprite, "name": options.name, "startTime": options.start_time, "endTime": options.end_time, "group": options.group }))?;
        Ok(())
    }

    pub fn state_at(&mut self, options: SpriteTimeOptions) -> Result<models::SpriteState, Error> {
        self.runtime.exec(json!({ "op": "Sprite.stateAt", "layer": self.handle.layer, "sprite": self.handle.sprite, "time": options.time }))
    }
}

/// A live storyboard runtime: the clean entry point for the handle-bound clients.
pub struct Runtime {
    inner: crate::Runtime,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(640.0, 480.0)
    }
}

impl Runtime {
    /// Creates a runtime of the given storyboard resolution.
    pub fn new(width: f64, height: f64) -> Self {
        Runtime {
            inner: crate::Runtime::new(width as f32, height as f32),
        }
    }

    /// The clean generator client bound to this runtime.
    pub fn generator(&mut self) -> Generator<'_> {
        Generator {
            runtime: &mut self.inner,
        }
    }

    /// The clean audio client bound to this runtime.
    pub fn audio(&mut self) -> Audio<'_> {
        Audio {
            runtime: Some(&mut self.inner),
        }
    }

    /// Resolves (or creates) a layer by name and binds its handle.
    pub fn layer(&mut self, name: &str) -> Result<Layer<'_>, Error> {
        let handle = self.inner.layer_handle(name)?;
        Ok(Layer {
            runtime: &mut self.inner,
            handle,
        })
    }

    /// Binds an existing layer/sprite pair.
    pub fn sprite(&mut self, layer: i32, sprite: i32) -> Sprite<'_> {
        Sprite {
            runtime: &mut self.inner,
            handle: models::Handle { layer, sprite },
        }
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
    pub audio: Audio<'static>,
    pub core: Core,
    pub diagnostics: Diagnostics,
    pub helpers: Helpers,
    pub image: Image,
    pub image_gen: Imagegen,
    pub media: Media,
    pub optimize: Optimize,
    pub storyboarding3d: Storyboarding3d,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            audio: Audio { runtime: None },
            core: Core,
            diagnostics: Diagnostics,
            helpers: Helpers,
            image: Image,
            image_gen: Imagegen,
            media: Media,
            optimize: Optimize,
            storyboarding3d: Storyboarding3d,
        }
    }
}
