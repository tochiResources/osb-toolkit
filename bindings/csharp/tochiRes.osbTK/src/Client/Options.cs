#nullable disable
using System.Collections.Generic;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

/// <summary>
/// An easing evaluation request.
/// </summary>
public sealed class ApplyEasingOptions
{
    /// <summary>
    /// The easing function.
    /// </summary>
    public Easing Easing { get; set; }

    /// <summary>
    /// The normalized time in `0.0..=1.0`.
    /// </summary>
    public double T { get; set; }
}

/// <summary>
/// A circular-arc sampling request.
/// </summary>
public sealed class ArcOptions
{
    /// <summary>
    /// The arc centre.
    /// </summary>
    public Vector2 Center { get; set; }

    /// <summary>
    /// The number of points (default `64`).
    /// </summary>
    public int? Count { get; set; }

    /// <summary>
    /// The radius.
    /// </summary>
    public double Radius { get; set; }

    /// <summary>
    /// The start angle in degrees.
    /// </summary>
    public double StartAngle { get; set; }

    /// <summary>
    /// The sweep angle in degrees.
    /// </summary>
    public double SweepAngle { get; set; }
}

/// <summary>
/// A `bake3d` request: a plain 3D scene.
/// </summary>
public sealed class Bake3dOptions
{
    /// <summary>
    /// The 3D scene.
    /// </summary>
    public Scene3dInput Input { get; set; }
}

/// <summary>
/// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
/// depending on `kind`.
/// </summary>
public sealed class CommandOptions
{
    /// <summary>
    /// The easing function.
    /// </summary>
    public Easing Easing { get; set; }

    /// <summary>
    /// The end value (scalar, point or color).
    /// </summary>
    public object End { get; set; }

    /// <summary>
    /// The command end time in milliseconds.
    /// </summary>
    public double EndTime { get; set; }

    /// <summary>
    /// The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
    /// `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
    /// `flipV`, `additive`).
    /// </summary>
    public string Kind { get; set; }

    /// <summary>
    /// The parameter kind for a `parameter` command.
    /// </summary>
    public ParameterType? Parameter { get; set; }

    /// <summary>
    /// The start value (scalar, point or color).
    /// </summary>
    public object Start { get; set; }

    /// <summary>
    /// The command start time in milliseconds.
    /// </summary>
    public double StartTime { get; set; }
}

/// <summary>
/// Create an animation sprite.
/// </summary>
public sealed class CreateAnimationOptions
{
    /// <summary>
    /// Delay between frames, in milliseconds.
    /// </summary>
    public double FrameDelay { get; set; }

    /// <summary>
    /// The number of frames.
    /// </summary>
    public int Frames { get; set; }

    /// <summary>
    /// The target layer name (omitted when `layerHandle` is set).
    /// </summary>
    public string Layer { get; set; }

    /// <summary>
    /// How the animation loops.
    /// </summary>
    public OsbLoopType LoopType { get; set; }

    /// <summary>
    /// The sprite anchor origin.
    /// </summary>
    public OsbOrigin Origin { get; set; }

    /// <summary>
    /// The texture path (relative to the mapset).
    /// </summary>
    public string Path { get; set; }

    /// <summary>
    /// The initial position.
    /// </summary>
    public Vector2 Position { get; set; }
}

/// <summary>
/// Create a sound sample on a layer.
/// </summary>
public sealed class CreateSampleOptions
{
    /// <summary>
    /// The target layer name (omitted when `layerHandle` is set).
    /// </summary>
    public string Layer { get; set; }

    /// <summary>
    /// The sample path (relative to the mapset).
    /// </summary>
    public string Path { get; set; }

    /// <summary>
    /// The sample time in milliseconds.
    /// </summary>
    public double Time { get; set; }

    /// <summary>
    /// The sample volume (`0..=100`).
    /// </summary>
    public double? Volume { get; set; }
}

/// <summary>
/// Create a sprite.
/// </summary>
public sealed class CreateSpriteOptions
{
    /// <summary>
    /// The target layer name (omitted when `layerHandle` is set).
    /// </summary>
    public string Layer { get; set; }

    /// <summary>
    /// The sprite anchor origin.
    /// </summary>
    public OsbOrigin Origin { get; set; }

    /// <summary>
    /// The texture path (relative to the mapset).
    /// </summary>
    public string Path { get; set; }

    /// <summary>
    /// The initial position; omitted means the sprite's default.
    /// </summary>
    public Vector2 Position { get; set; }
}

/// <summary>
/// A curve hit-test request.
/// </summary>
public sealed class CurveDistanceOptions
{
    /// <summary>
    /// The curve.
    /// </summary>
    public CurveSpec Curve { get; set; }

    /// <summary>
    /// The query point.
    /// </summary>
    public Vector2 Point { get; set; }
}

/// <summary>
/// A curve offset request.
/// </summary>
public sealed class CurveOffsetOptions
{
    /// <summary>
    /// The curve.
    /// </summary>
    public CurveSpec Curve { get; set; }

    /// <summary>
    /// The signed offset along the curve's local normal.
    /// </summary>
    public double Distance { get; set; }
}

/// <summary>
/// A curve query at a parametric or arc-length position.
/// </summary>
public sealed class CurvePointOptions
{
    /// <summary>
    /// Sample by arc length instead of the raw parameter (default `false`).
    /// </summary>
    public bool? ByArcLength { get; set; }

    /// <summary>
    /// The curve.
    /// </summary>
    public CurveSpec Curve { get; set; }

    /// <summary>
    /// The position in `0.0..=1.0`.
    /// </summary>
    public double T { get; set; }
}

/// <summary>
/// A curve query.
/// </summary>
public sealed class CurveOptions
{
    /// <summary>
    /// The curve.
    /// </summary>
    public CurveSpec Curve { get; set; }
}

/// <summary>
/// A curve sampling request.
/// </summary>
public sealed class CurveSampleOptions
{
    /// <summary>
    /// Sample by arc length instead of the raw parameter (default `false`).
    /// </summary>
    public bool? ByArcLength { get; set; }

    /// <summary>
    /// The number of points to sample.
    /// </summary>
    public int Count { get; set; }

    /// <summary>
    /// The curve.
    /// </summary>
    public CurveSpec Curve { get; set; }
}

/// <summary>
/// A named-or-custom easing evaluation request.
/// </summary>
public sealed class EasingValueOptions
{
    /// <summary>
    /// A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
    /// </summary>
    public List<double> CubicBezier { get; set; }

    /// <summary>
    /// A named easing (used when `cubicBezier` is absent).
    /// </summary>
    public Easing? Easing { get; set; }

    /// <summary>
    /// The normalized time in `0.0..=1.0`.
    /// </summary>
    public double T { get; set; }
}

/// <summary>
/// An ellipse sampling request.
/// </summary>
public sealed class EllipseOptions
{
    /// <summary>
    /// The ellipse centre.
    /// </summary>
    public Vector2 Center { get; set; }

    /// <summary>
    /// The number of points (default `64`).
    /// </summary>
    public int? Count { get; set; }

    /// <summary>
    /// The horizontal radius.
    /// </summary>
    public double RadiusX { get; set; }

    /// <summary>
    /// The vertical radius.
    /// </summary>
    public double RadiusY { get; set; }
}

/// <summary>
/// An image-sequence encoding request (frames -&gt; video).
/// </summary>
public sealed class EncodeVideoOptions
{
    /// <summary>
    /// The video codec (default `libx264`).
    /// </summary>
    public string Codec { get; set; }

    /// <summary>
    /// The constant-rate-factor quality (`0`-`51`; lower is better).
    /// </summary>
    public int? Crf { get; set; }

    /// <summary>
    /// The frame rate.
    /// </summary>
    public double Fps { get; set; }

    /// <summary>
    /// The input filename pattern (e.g. `frames/frame_%04d.png`).
    /// </summary>
    public string InputPattern { get; set; }

    /// <summary>
    /// The output video path.
    /// </summary>
    public string Output { get; set; }
}

/// <summary>
/// An audio-extraction request (media -&gt; audio file).
/// </summary>
public sealed class ExtractAudioOptions
{
    /// <summary>
    /// The input media path.
    /// </summary>
    public string Input { get; set; }

    /// <summary>
    /// The output audio path (its extension selects the format).
    /// </summary>
    public string Output { get; set; }
}

/// <summary>
/// A frame-extraction request (video -&gt; image sequence).
/// </summary>
public sealed class ExtractFramesOptions
{
    /// <summary>
    /// The maximum number of frames to extract.
    /// </summary>
    public int? Count { get; set; }

    /// <summary>
    /// The sampling rate (frames per second); defaults to the source rate.
    /// </summary>
    public double? Fps { get; set; }

    /// <summary>
    /// Scale frames to this height.
    /// </summary>
    public int? Height { get; set; }

    /// <summary>
    /// The input media path.
    /// </summary>
    public string Input { get; set; }

    /// <summary>
    /// The directory to write frames into.
    /// </summary>
    public string OutputDir { get; set; }

    /// <summary>
    /// The start time in seconds.
    /// </summary>
    public double? Start { get; set; }

    /// <summary>
    /// Scale frames to this width.
    /// </summary>
    public int? Width { get; set; }
}

/// <summary>
/// An FFT request against a `.wav` file.
/// </summary>
public sealed class FftOptions
{
    /// <summary>
    /// Number of frequency bins.
    /// </summary>
    public int? Magnitudes { get; set; }

    /// <summary>
    /// Path to the audio file.
    /// </summary>
    public string Path { get; set; }

    /// <summary>
    /// Sample time in milliseconds.
    /// </summary>
    public double? Time { get; set; }
}

/// <summary>
/// An FFT request against the runtime's loaded audio.
/// </summary>
public sealed class GetFftOptions
{
    /// <summary>
    /// A frequency cut-off.
    /// </summary>
    public double? Cutoff { get; set; }

    /// <summary>
    /// The easing applied across the bins.
    /// </summary>
    public Easing? Easing { get; set; }

    /// <summary>
    /// Number of frequency bins.
    /// </summary>
    public int Magnitudes { get; set; }

    /// <summary>
    /// Sample time in milliseconds.
    /// </summary>
    public double Time { get; set; }
}

/// <summary>
/// The anchor offset for an origin.
/// </summary>
public sealed class GetOriginVectorOptions
{
    /// <summary>
    /// The sprite height.
    /// </summary>
    public double Height { get; set; }

    /// <summary>
    /// The sprite anchor origin.
    /// </summary>
    public OsbOrigin Origin { get; set; }

    /// <summary>
    /// The sprite width.
    /// </summary>
    public double Width { get; set; }
}

/// <summary>
/// A motion-graphics helper call: a helper `name` plus its input.
/// </summary>
public sealed class HelperOptions
{
    /// <summary>
    /// The helper's input (a keyframe channel, a command list, or its parameters).
    /// </summary>
    public object Input { get; set; }

    /// <summary>
    /// The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
    /// </summary>
    public string Name { get; set; }
}

/// <summary>
/// An image processing request: read `path`, apply `options`, write `output`.
/// </summary>
public sealed class ImageProcessOptions
{
    /// <summary>
    /// The processing options.
    /// </summary>
    public ImageOptions Options { get; set; }

    /// <summary>
    /// The output file path (its extension selects the format).
    /// </summary>
    public string Output { get; set; }

    /// <summary>
    /// The input file path.
    /// </summary>
    public string Path { get; set; }
}

/// <summary>
/// Whether a rotated sprite's bounding box intersects the widescreen
/// storyboard area.
/// </summary>
public sealed class InScreenBoundsOptions
{
    /// <summary>
    /// The anchor offset (from `GetOriginVector`).
    /// </summary>
    public Vector2 Origin { get; set; }

    /// <summary>
    /// The sprite centre position.
    /// </summary>
    public Vector2 Position { get; set; }

    /// <summary>
    /// The rotation in radians.
    /// </summary>
    public double Rotation { get; set; }

    /// <summary>
    /// The sprite size.
    /// </summary>
    public Vector2 Size { get; set; }
}

/// <summary>
/// Resolve (or create) a layer by name.
/// </summary>
public sealed class LayerHandleOptions
{
    /// <summary>
    /// The layer name.
    /// </summary>
    public string Name { get; set; }
}

/// <summary>
/// A line-segment intersection request.
/// </summary>
public sealed class LineIntersectOptions
{
    /// <summary>
    /// The first segment's start.
    /// </summary>
    public Vector2 A1 { get; set; }

    /// <summary>
    /// The first segment's end.
    /// </summary>
    public Vector2 A2 { get; set; }

    /// <summary>
    /// The second segment's start.
    /// </summary>
    public Vector2 B1 { get; set; }

    /// <summary>
    /// The second segment's end.
    /// </summary>
    public Vector2 B2 { get; set; }
}

/// <summary>
/// An `.osb` optimization request.
/// </summary>
public sealed class OptimizeOptions
{
    /// <summary>
    /// The optimization options.
    /// </summary>
    public OptimizeOptionsInput Options { get; set; }

    /// <summary>
    /// The `.osb` storyboard text.
    /// </summary>
    public string Text { get; set; }
}

/// <summary>
/// A request carrying a single file path.
/// </summary>
public sealed class PathOptions
{
    /// <summary>
    /// The input file path.
    /// </summary>
    public string Path { get; set; }
}

/// <summary>
/// A deterministic random request.
/// </summary>
public sealed class RandomOptions
{
    /// <summary>
    /// The exclusive upper bound.
    /// </summary>
    public double Max { get; set; }

    /// <summary>
    /// The inclusive lower bound.
    /// </summary>
    public double Min { get; set; }
}

/// <summary>
/// A shape-rendering request: a canvas, a background and a shape list.
/// </summary>
public sealed class RenderShapesOptions
{
    /// <summary>
    /// The background colour (default transparent).
    /// </summary>
    public RgbaColor Background { get; set; }

    /// <summary>
    /// The canvas height in pixels.
    /// </summary>
    public int Height { get; set; }

    /// <summary>
    /// The output file path (its extension selects the format).
    /// </summary>
    public string Output { get; set; }

    /// <summary>
    /// The shapes to draw, in order.
    /// </summary>
    public List<Shape> Shapes { get; set; }

    /// <summary>
    /// The canvas width in pixels.
    /// </summary>
    public int Width { get; set; }
}

/// <summary>
/// Record a depth index on a segment.
/// </summary>
public sealed class SegmentAddIndexOptions
{
    /// <summary>
    /// The index to record.
    /// </summary>
    public double Index { get; set; }
}

/// <summary>
/// Move a segment to another osu! layer (`ChangePosition`).
/// </summary>
public sealed class SegmentChangePositionOptions
{
    /// <summary>
    /// The destination osu! layer.
    /// </summary>
    public OsbLayer OsbLayer { get; set; }
}

/// <summary>
/// Set a segment's reverse-depth flag.
/// </summary>
public sealed class SegmentReverseDepthOptions
{
    /// <summary>
    /// Whether children are emitted in reverse order.
    /// </summary>
    public bool Value { get; set; }
}

/// <summary>
/// A Douglas-Peucker simplification request.
/// </summary>
public sealed class SimplifyOptions
{
    /// <summary>
    /// The polyline to simplify.
    /// </summary>
    public List<Vector2> Points { get; set; }

    /// <summary>
    /// The maximum deviation to allow.
    /// </summary>
    public double Tolerance { get; set; }
}

/// <summary>
/// A sprite handle pair plus a sample time.
/// </summary>
public sealed class SpriteTimeOptions
{
    /// <summary>
    /// The sample time in milliseconds.
    /// </summary>
    public double Time { get; set; }
}

/// <summary>
/// Start a loop group.
/// </summary>
public sealed class StartLoopGroupOptions
{
    /// <summary>
    /// The number of iterations.
    /// </summary>
    public double Count { get; set; }

    /// <summary>
    /// The loop start time in milliseconds.
    /// </summary>
    public double StartTime { get; set; }
}

/// <summary>
/// Start a trigger group.
/// </summary>
public sealed class StartTriggerGroupOptions
{
    /// <summary>
    /// The trigger end time in milliseconds.
    /// </summary>
    public double EndTime { get; set; }

    /// <summary>
    /// The trigger group index.
    /// </summary>
    public int Group { get; set; }

    /// <summary>
    /// The trigger name (matches a hit object).
    /// </summary>
    public string Name { get; set; }

    /// <summary>
    /// The trigger start time in milliseconds.
    /// </summary>
    public double StartTime { get; set; }
}

/// <summary>
/// A request carrying a block of text (an effect script, `.osu` or `.osb`).
/// </summary>
public sealed class TextOptions
{
    /// <summary>
    /// The text payload.
    /// </summary>
    public string Text { get; set; }
}

/// <summary>
/// A time query.
/// </summary>
public sealed class TimeOptions
{
    /// <summary>
    /// The time in milliseconds.
    /// </summary>
    public double Time { get; set; }
}
