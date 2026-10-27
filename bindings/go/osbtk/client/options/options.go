package options

import (
	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// An easing evaluation request.
type ApplyEasingOptions struct {
	// The easing function.
	Easing osbtk.Easing
	// The normalized time in `0.0..=1.0`.
	T float64
}

// A circular-arc sampling request.
type ArcOptions struct {
	// The arc centre.
	Center osbtk.Vector2
	// The number of points (default `64`).
	Count *int32
	// The radius.
	Radius float64
	// The start angle in degrees.
	StartAngle float64
	// The sweep angle in degrees.
	SweepAngle float64
}

// A `bake3d` request: a plain 3D scene.
type Bake3dOptions struct {
	// The 3D scene.
	Input osbtk.Scene3dInput
}

// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
// depending on `kind`.
type CommandOptions struct {
	// The easing function.
	Easing osbtk.Easing
	// The end value (scalar, point or color).
	End map[string]interface{}
	// The command end time in milliseconds.
	EndTime float64
	// The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
	// `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
	// `flipV`, `additive`).
	Kind string
	// The parameter kind for a `parameter` command.
	Parameter *osbtk.ParameterType
	// The start value (scalar, point or color).
	Start map[string]interface{}
	// The command start time in milliseconds.
	StartTime float64
}

// Create an animation sprite.
type CreateAnimationOptions struct {
	// Delay between frames, in milliseconds.
	FrameDelay float64
	// The number of frames.
	Frames int32
	// The target layer name (omitted when `layerHandle` is set).
	Layer *string
	// How the animation loops.
	LoopType osbtk.OsbLoopType
	// The sprite anchor origin.
	Origin osbtk.OsbOrigin
	// The texture path (relative to the mapset).
	Path string
	// The initial position.
	Position osbtk.Vector2
}

// Create a sound sample on a layer.
type CreateSampleOptions struct {
	// The target layer name (omitted when `layerHandle` is set).
	Layer *string
	// The sample path (relative to the mapset).
	Path string
	// The sample time in milliseconds.
	Time float64
	// The sample volume (`0..=100`).
	Volume *float64
}

// Create a sprite.
type CreateSpriteOptions struct {
	// The target layer name (omitted when `layerHandle` is set).
	Layer *string
	// The sprite anchor origin.
	Origin osbtk.OsbOrigin
	// The texture path (relative to the mapset).
	Path string
	// The initial position; omitted means the sprite's default.
	Position *osbtk.Vector2
}

// A curve hit-test request.
type CurveDistanceOptions struct {
	// The curve.
	Curve osbtk.CurveSpec
	// The query point.
	Point osbtk.Vector2
}

// A curve offset request.
type CurveOffsetOptions struct {
	// The curve.
	Curve osbtk.CurveSpec
	// The signed offset along the curve's local normal.
	Distance float64
}

// A curve query at a parametric or arc-length position.
type CurvePointOptions struct {
	// Sample by arc length instead of the raw parameter (default `false`).
	ByArcLength *bool
	// The curve.
	Curve osbtk.CurveSpec
	// The position in `0.0..=1.0`.
	T float64
}

// A curve query.
type CurveOptions struct {
	// The curve.
	Curve osbtk.CurveSpec
}

// A curve sampling request.
type CurveSampleOptions struct {
	// Sample by arc length instead of the raw parameter (default `false`).
	ByArcLength *bool
	// The number of points to sample.
	Count int32
	// The curve.
	Curve osbtk.CurveSpec
}

// A named-or-custom easing evaluation request.
type EasingValueOptions struct {
	// A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
	CubicBezier []float64
	// A named easing (used when `cubicBezier` is absent).
	Easing *osbtk.Easing
	// The normalized time in `0.0..=1.0`.
	T float64
}

// An ellipse sampling request.
type EllipseOptions struct {
	// The ellipse centre.
	Center osbtk.Vector2
	// The number of points (default `64`).
	Count *int32
	// The horizontal radius.
	RadiusX float64
	// The vertical radius.
	RadiusY float64
}

// An image-sequence encoding request (frames -> video).
type EncodeVideoOptions struct {
	// The video codec (default `libx264`).
	Codec *string
	// The constant-rate-factor quality (`0`-`51`; lower is better).
	Crf *int32
	// The frame rate.
	Fps float64
	// The input filename pattern (e.g. `frames/frame_%04d.png`).
	InputPattern string
	// The output video path.
	Output string
}

// An audio-extraction request (media -> audio file).
type ExtractAudioOptions struct {
	// The input media path.
	Input string
	// The output audio path (its extension selects the format).
	Output string
}

// A frame-extraction request (video -> image sequence).
type ExtractFramesOptions struct {
	// The maximum number of frames to extract.
	Count *int32
	// The sampling rate (frames per second); defaults to the source rate.
	Fps *float64
	// Scale frames to this height.
	Height *int32
	// The input media path.
	Input string
	// The directory to write frames into.
	OutputDir string
	// The start time in seconds.
	Start *float64
	// Scale frames to this width.
	Width *int32
}

// An FFT request against a `.wav` file.
type FftOptions struct {
	// Number of frequency bins.
	Magnitudes *int32
	// Path to the audio file.
	Path string
	// Sample time in milliseconds.
	Time *float64
}

// An FFT request against the runtime's loaded audio.
type GetFftOptions struct {
	// A frequency cut-off.
	Cutoff *float64
	// The easing applied across the bins.
	Easing *osbtk.Easing
	// Number of frequency bins.
	Magnitudes int32
	// Sample time in milliseconds.
	Time float64
}

// The anchor offset for an origin.
type GetOriginVectorOptions struct {
	// The sprite height.
	Height float64
	// The sprite anchor origin.
	Origin osbtk.OsbOrigin
	// The sprite width.
	Width float64
}

// A sprite handle pair.
type HandleOptions struct {
}

// A motion-graphics helper call: a helper `name` plus its input.
type HelperOptions struct {
	// The helper's input (a keyframe channel, a command list, or its parameters).
	Input *string
	// The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
	Name string
}

// An image processing request: read `path`, apply `options`, write `output`.
type ImageProcessOptions struct {
	// The processing options.
	Options osbtk.ImageOptions
	// The output file path (its extension selects the format).
	Output string
	// The input file path.
	Path string
}

// Whether a rotated sprite's bounding box intersects the widescreen
// storyboard area.
type InScreenBoundsOptions struct {
	// The anchor offset (from `GetOriginVector`).
	Origin osbtk.Vector2
	// The sprite centre position.
	Position osbtk.Vector2
	// The rotation in radians.
	Rotation float64
	// The sprite size.
	Size osbtk.Vector2
}

// Resolve (or create) a layer by name.
type LayerHandleOptions struct {
	// The layer name.
	Name string
}

// A line-segment intersection request.
type LineIntersectOptions struct {
	// The first segment's start.
	A1 osbtk.Vector2
	// The first segment's end.
	A2 osbtk.Vector2
	// The second segment's start.
	B1 osbtk.Vector2
	// The second segment's end.
	B2 osbtk.Vector2
}

// An `.osb` optimization request.
type OptimizeOptions struct {
	// The optimization options.
	Options osbtk.OptimizeOptionsInput
	// The `.osb` storyboard text.
	Text string
}

// A request carrying a single file path.
type PathOptions struct {
	// The input file path.
	Path string
}

// A deterministic random request.
type RandomOptions struct {
	// The exclusive upper bound.
	Max float64
	// The inclusive lower bound.
	Min float64
}

// A shape-rendering request: a canvas, a background and a shape list.
type RenderShapesOptions struct {
	// The background colour (default transparent).
	Background *osbtk.RgbaColor
	// The canvas height in pixels.
	Height int32
	// The output file path (its extension selects the format).
	Output string
	// The shapes to draw, in order.
	Shapes []osbtk.Shape
	// The canvas width in pixels.
	Width int32
}

// Record a depth index on a segment.
type SegmentAddIndexOptions struct {
	// The index to record.
	Index float64
}

// Move a segment to another osu! layer (`ChangePosition`).
type SegmentChangePositionOptions struct {
	// The destination osu! layer.
	OsbLayer osbtk.OsbLayer
}

// A segment handle request.
type SegmentOptions struct {
}

// Set a segment's reverse-depth flag.
type SegmentReverseDepthOptions struct {
	// Whether children are emitted in reverse order.
	Value bool
}

// A Douglas-Peucker simplification request.
type SimplifyOptions struct {
	// The polyline to simplify.
	Points []osbtk.Vector2
	// The maximum deviation to allow.
	Tolerance float64
}

// A sprite handle pair plus a sample time.
type SpriteTimeOptions struct {
	// The sample time in milliseconds.
	Time float64
}

// Start a loop group.
type StartLoopGroupOptions struct {
	// The number of iterations.
	Count float64
	// The loop start time in milliseconds.
	StartTime float64
}

// Start a trigger group.
type StartTriggerGroupOptions struct {
	// The trigger end time in milliseconds.
	EndTime float64
	// The trigger group index.
	Group int32
	// The trigger name (matches a hit object).
	Name string
	// The trigger start time in milliseconds.
	StartTime float64
}

// A request carrying a block of text (an effect script, `.osu` or `.osb`).
type TextOptions struct {
	// The text payload.
	Text string
}

// A time query.
type TimeOptions struct {
	// The time in milliseconds.
	Time float64
}
