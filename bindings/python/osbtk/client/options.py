from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any, Dict, List, Optional

from ..models.curve_spec import CurveSpec
from ..models.easing import Easing
from ..models.image_options import ImageOptions
from ..models.optimize_options_input import OptimizeOptionsInput
from ..models.osb_layer import OsbLayer
from ..models.osb_loop_type import OsbLoopType
from ..models.osb_origin import OsbOrigin
from ..models.parameter_type import ParameterType
from ..models.rgba_color import RgbaColor
from ..models.scene3d_input import Scene3dInput
from ..models.shape import Shape
from ..models.vector2 import Vector2

@dataclass(kw_only=True)
class ApplyEasingOptions:
    """An easing evaluation request."""
    # The easing function.
    easing: Easing
    # The normalized time in `0.0..=1.0`.
    t: float

@dataclass(kw_only=True)
class ArcOptions:
    """A circular-arc sampling request."""
    # The arc centre.
    center: Vector2
    # The radius.
    radius: float
    # The start angle in degrees.
    startAngle: float
    # The sweep angle in degrees.
    sweepAngle: float
    # The number of points (default `64`).
    count: Optional[int] = None

@dataclass(kw_only=True)
class Bake3dOptions:
    """A `bake3d` request: a plain 3D scene."""
    # The 3D scene.
    input: Scene3dInput

@dataclass(kw_only=True)
class CommandOptions:
    """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
depending on `kind`."""
    # The easing function.
    easing: Easing
    # The command end time in milliseconds.
    endTime: float
    # The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
    # `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
    # `flipV`, `additive`).
    kind: str
    # The command start time in milliseconds.
    startTime: float
    # The end value (scalar, point or color).
    end: Optional[Dict[str, Any]] = None
    # The parameter kind for a `parameter` command.
    parameter: Optional[ParameterType] = None
    # The start value (scalar, point or color).
    start: Optional[Dict[str, Any]] = None

@dataclass(kw_only=True)
class CreateAnimationOptions:
    """Create an animation sprite."""
    # Delay between frames, in milliseconds.
    frameDelay: float
    # The number of frames.
    frames: int
    # How the animation loops.
    loopType: OsbLoopType
    # The sprite anchor origin.
    origin: OsbOrigin
    # The texture path (relative to the mapset).
    path: str
    # The initial position.
    position: Vector2
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None

@dataclass(kw_only=True)
class CreateSampleOptions:
    """Create a sound sample on a layer."""
    # The sample path (relative to the mapset).
    path: str
    # The sample time in milliseconds.
    time: float
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None
    # The sample volume (`0..=100`).
    volume: Optional[float] = None

@dataclass(kw_only=True)
class CreateSpriteOptions:
    """Create a sprite."""
    # The sprite anchor origin.
    origin: OsbOrigin
    # The texture path (relative to the mapset).
    path: str
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None
    # The initial position; omitted means the sprite's default.
    position: Optional[Vector2] = None

@dataclass(kw_only=True)
class CurveDistanceOptions:
    """A curve hit-test request."""
    # The curve.
    curve: CurveSpec
    # The query point.
    point: Vector2

@dataclass(kw_only=True)
class CurveOffsetOptions:
    """A curve offset request."""
    # The curve.
    curve: CurveSpec
    # The signed offset along the curve's local normal.
    distance: float

@dataclass(kw_only=True)
class CurvePointOptions:
    """A curve query at a parametric or arc-length position."""
    # The curve.
    curve: CurveSpec
    # The position in `0.0..=1.0`.
    t: float
    # Sample by arc length instead of the raw parameter (default `false`).
    byArcLength: Optional[bool] = None

@dataclass(kw_only=True)
class CurveOptions:
    """A curve query."""
    # The curve.
    curve: CurveSpec

@dataclass(kw_only=True)
class CurveSampleOptions:
    """A curve sampling request."""
    # The number of points to sample.
    count: int
    # The curve.
    curve: CurveSpec
    # Sample by arc length instead of the raw parameter (default `false`).
    byArcLength: Optional[bool] = None

@dataclass(kw_only=True)
class EasingValueOptions:
    """A named-or-custom easing evaluation request."""
    # The normalized time in `0.0..=1.0`.
    t: float
    # A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
    cubicBezier: Optional[List[float]] = None
    # A named easing (used when `cubicBezier` is absent).
    easing: Optional[Easing] = None

@dataclass(kw_only=True)
class EllipseOptions:
    """An ellipse sampling request."""
    # The ellipse centre.
    center: Vector2
    # The horizontal radius.
    radiusX: float
    # The vertical radius.
    radiusY: float
    # The number of points (default `64`).
    count: Optional[int] = None

@dataclass(kw_only=True)
class EncodeVideoOptions:
    """An image-sequence encoding request (frames -> video)."""
    # The frame rate.
    fps: float
    # The input filename pattern (e.g. `frames/frame_%04d.png`).
    inputPattern: str
    # The output video path.
    output: str
    # The video codec (default `libx264`).
    codec: Optional[str] = None
    # The constant-rate-factor quality (`0`-`51`; lower is better).
    crf: Optional[int] = None

@dataclass(kw_only=True)
class ExtractAudioOptions:
    """An audio-extraction request (media -> audio file)."""
    # The input media path.
    input: str
    # The output audio path (its extension selects the format).
    output: str

@dataclass(kw_only=True)
class ExtractFramesOptions:
    """A frame-extraction request (video -> image sequence)."""
    # The input media path.
    input: str
    # The directory to write frames into.
    outputDir: str
    # The maximum number of frames to extract.
    count: Optional[int] = None
    # The sampling rate (frames per second); defaults to the source rate.
    fps: Optional[float] = None
    # Scale frames to this height.
    height: Optional[int] = None
    # The start time in seconds.
    start: Optional[float] = None
    # Scale frames to this width.
    width: Optional[int] = None

@dataclass(kw_only=True)
class FftOptions:
    """An FFT request against a `.wav` file."""
    # Path to the audio file.
    path: str
    # Number of frequency bins.
    magnitudes: Optional[int] = None
    # Sample time in milliseconds.
    time: Optional[float] = None

@dataclass(kw_only=True)
class GetFftOptions:
    """An FFT request against the runtime's loaded audio."""
    # Number of frequency bins.
    magnitudes: int
    # Sample time in milliseconds.
    time: float
    # A frequency cut-off.
    cutoff: Optional[float] = None
    # The easing applied across the bins.
    easing: Optional[Easing] = None

@dataclass(kw_only=True)
class GetOriginVectorOptions:
    """The anchor offset for an origin."""
    # The sprite height.
    height: float
    # The sprite anchor origin.
    origin: OsbOrigin
    # The sprite width.
    width: float

@dataclass(kw_only=True)
class HandleOptions:
    """A sprite handle pair."""


@dataclass(kw_only=True)
class HelperOptions:
    """A motion-graphics helper call: a helper `name` plus its input."""
    # The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
    name: str
    # The helper's input (a keyframe channel, a command list, or its parameters).
    input: Optional[str] = None

@dataclass(kw_only=True)
class ImageProcessOptions:
    """An image processing request: read `path`, apply `options`, write `output`."""
    # The processing options.
    options: ImageOptions
    # The output file path (its extension selects the format).
    output: str
    # The input file path.
    path: str

@dataclass(kw_only=True)
class InScreenBoundsOptions:
    """Whether a rotated sprite's bounding box intersects the widescreen
storyboard area."""
    # The anchor offset (from `GetOriginVector`).
    origin: Vector2
    # The sprite centre position.
    position: Vector2
    # The rotation in radians.
    rotation: float
    # The sprite size.
    size: Vector2

@dataclass(kw_only=True)
class LayerHandleOptions:
    """Resolve (or create) a layer by name."""
    # The layer name.
    name: str

@dataclass(kw_only=True)
class LineIntersectOptions:
    """A line-segment intersection request."""
    # The first segment's start.
    a1: Vector2
    # The first segment's end.
    a2: Vector2
    # The second segment's start.
    b1: Vector2
    # The second segment's end.
    b2: Vector2

@dataclass(kw_only=True)
class OptimizeOptions:
    """An `.osb` optimization request."""
    # The optimization options.
    options: OptimizeOptionsInput
    # The `.osb` storyboard text.
    text: str

@dataclass(kw_only=True)
class PathOptions:
    """A request carrying a single file path."""
    # The input file path.
    path: str

@dataclass(kw_only=True)
class RandomOptions:
    """A deterministic random request."""
    # The exclusive upper bound.
    max: float
    # The inclusive lower bound.
    min: float

@dataclass(kw_only=True)
class RenderShapesOptions:
    """A shape-rendering request: a canvas, a background and a shape list."""
    # The canvas height in pixels.
    height: int
    # The output file path (its extension selects the format).
    output: str
    # The shapes to draw, in order.
    shapes: List[Shape]
    # The canvas width in pixels.
    width: int
    # The background colour (default transparent).
    background: Optional[RgbaColor] = None

@dataclass(kw_only=True)
class SegmentAddIndexOptions:
    """Record a depth index on a segment."""
    # The index to record.
    index: float

@dataclass(kw_only=True)
class SegmentChangePositionOptions:
    """Move a segment to another osu! layer (`ChangePosition`)."""
    # The destination osu! layer.
    osbLayer: OsbLayer

@dataclass(kw_only=True)
class SegmentOptions:
    """A segment handle request."""


@dataclass(kw_only=True)
class SegmentReverseDepthOptions:
    """Set a segment's reverse-depth flag."""
    # Whether children are emitted in reverse order.
    value: bool

@dataclass(kw_only=True)
class SimplifyOptions:
    """A Douglas-Peucker simplification request."""
    # The polyline to simplify.
    points: List[Vector2]
    # The maximum deviation to allow.
    tolerance: float

@dataclass(kw_only=True)
class SpriteTimeOptions:
    """A sprite handle pair plus a sample time."""
    # The sample time in milliseconds.
    time: float

@dataclass(kw_only=True)
class StartLoopGroupOptions:
    """Start a loop group."""
    # The number of iterations.
    count: float
    # The loop start time in milliseconds.
    startTime: float

@dataclass(kw_only=True)
class StartTriggerGroupOptions:
    """Start a trigger group."""
    # The trigger end time in milliseconds.
    endTime: float
    # The trigger group index.
    group: int
    # The trigger name (matches a hit object).
    name: str
    # The trigger start time in milliseconds.
    startTime: float

@dataclass(kw_only=True)
class TextOptions:
    """A request carrying a block of text (an effect script, `.osu` or `.osb`)."""
    # The text payload.
    text: str

@dataclass(kw_only=True)
class TimeOptions:
    """A time query."""
    # The time in milliseconds.
    time: float


def to_kwargs(options: object) -> Dict[str, Any]:
    """Convert a clean options dataclass to request model keyword arguments."""
    return {key: value for key, value in asdict(options).items() if value is not None}
