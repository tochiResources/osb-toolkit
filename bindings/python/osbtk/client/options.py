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

@dataclass
class ApplyEasingOptions:
    """An easing evaluation request."""
    # The easing function.
    easing: Easing
    # The normalized time in `0.0..=1.0`.
    t: float

@dataclass
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

@dataclass
class Bake3dOptions:
    """A `bake3d` request: a plain 3D scene."""
    # The 3D scene.
    input: Scene3dInput

@dataclass
class CommandOptions:
    """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
depending on `kind`."""
    # The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
    # `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
    # `flipV`, `additive`).
    kind: str
    # The command start time in milliseconds.
    startTime: float
    # The command end time in milliseconds.
    endTime: float
    # The easing function.
    easing: Easing
    # The start value (scalar, point or color).
    start: Optional[Dict[str, Any]] = None
    # The end value (scalar, point or color).
    end: Optional[Dict[str, Any]] = None
    # The parameter kind for a `parameter` command.
    parameter: Optional[ParameterType] = None

@dataclass
class CreateAnimationOptions:
    """Create an animation sprite."""
    # The texture path (relative to the mapset).
    path: str
    # The number of frames.
    frames: int
    # Delay between frames, in milliseconds.
    frameDelay: float
    # How the animation loops.
    loopType: OsbLoopType
    # The sprite anchor origin.
    origin: OsbOrigin
    # The initial position.
    position: Vector2
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None

@dataclass
class CreateSampleOptions:
    """Create a sound sample on a layer."""
    # The sample path (relative to the mapset).
    path: str
    # The sample time in milliseconds.
    time: float
    # The sample volume (`0..=100`).
    volume: Optional[float] = None
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None

@dataclass
class CreateSpriteOptions:
    """Create a sprite."""
    # The texture path (relative to the mapset).
    path: str
    # The sprite anchor origin.
    origin: OsbOrigin
    # The initial position; omitted means the sprite's default.
    position: Optional[Vector2] = None
    # The target layer name (omitted when `layerHandle` is set).
    layer: Optional[str] = None

@dataclass
class CurveDistanceOptions:
    """A curve hit-test request."""
    # The curve.
    curve: CurveSpec
    # The query point.
    point: Vector2

@dataclass
class CurveOffsetOptions:
    """A curve offset request."""
    # The curve.
    curve: CurveSpec
    # The signed offset along the curve's local normal.
    distance: float

@dataclass
class CurvePointOptions:
    """A curve query at a parametric or arc-length position."""
    # The curve.
    curve: CurveSpec
    # The position in `0.0..=1.0`.
    t: float
    # Sample by arc length instead of the raw parameter (default `false`).
    byArcLength: Optional[bool] = None

@dataclass
class CurveOptions:
    """A curve query."""
    # The curve.
    curve: CurveSpec

@dataclass
class CurveSampleOptions:
    """A curve sampling request."""
    # The number of points to sample.
    count: int
    # The curve.
    curve: CurveSpec
    # Sample by arc length instead of the raw parameter (default `false`).
    byArcLength: Optional[bool] = None

@dataclass
class EasingValueOptions:
    """A named-or-custom easing evaluation request."""
    # The normalized time in `0.0..=1.0`.
    t: float
    # A named easing (used when `cubicBezier` is absent).
    easing: Optional[Easing] = None
    # A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
    cubicBezier: Optional[List[float]] = None

@dataclass
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

@dataclass
class EncodeVideoOptions:
    """An image-sequence encoding request (frames -> video)."""
    # The input filename pattern (e.g. `frames/frame_%04d.png`).
    inputPattern: str
    # The output video path.
    output: str
    # The frame rate.
    fps: float
    # The video codec (default `libx264`).
    codec: Optional[str] = None
    # The constant-rate-factor quality (`0`-`51`; lower is better).
    crf: Optional[int] = None

@dataclass
class ExtractAudioOptions:
    """An audio-extraction request (media -> audio file)."""
    # The input media path.
    input: str
    # The output audio path (its extension selects the format).
    output: str

@dataclass
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

@dataclass
class FftOptions:
    """An FFT request against a `.wav` file."""
    # Path to the audio file.
    path: str
    # Number of frequency bins.
    magnitudes: Optional[int] = None
    # Sample time in milliseconds.
    time: Optional[float] = None

@dataclass
class GetFftOptions:
    """An FFT request against the runtime's loaded audio."""
    # Sample time in milliseconds.
    time: float
    # Number of frequency bins.
    magnitudes: int
    # A frequency cut-off.
    cutoff: Optional[float] = None
    # The easing applied across the bins.
    easing: Optional[Easing] = None

@dataclass
class GetOriginVectorOptions:
    """The anchor offset for an origin."""
    # The sprite anchor origin.
    origin: OsbOrigin
    # The sprite width.
    width: float
    # The sprite height.
    height: float

@dataclass
class HandleOptions:
    """A sprite handle pair."""


@dataclass
class HelperOptions:
    """A motion-graphics helper call: a helper `name` plus its input."""
    # The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
    name: str
    # The helper's input (a keyframe channel, a command list, or its parameters).
    input: Optional[Any] = None

@dataclass
class ImageProcessOptions:
    """An image processing request: read `path`, apply `options`, write `output`."""
    # The processing options.
    options: ImageOptions
    # The output file path (its extension selects the format).
    output: str
    # The input file path.
    path: str

@dataclass
class InScreenBoundsOptions:
    """Whether a rotated sprite's bounding box intersects the widescreen
storyboard area."""
    # The sprite centre position.
    position: Vector2
    # The sprite size.
    size: Vector2
    # The rotation in radians.
    rotation: float
    # The anchor offset (from `GetOriginVector`).
    origin: Vector2

@dataclass
class LayerHandleOptions:
    """Resolve (or create) a layer by name."""
    # The layer name.
    name: str

@dataclass
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

@dataclass
class OptimizeOptions:
    """An `.osb` optimization request."""
    # The optimization options.
    options: OptimizeOptionsInput
    # The `.osb` storyboard text.
    text: str

@dataclass
class PathOptions:
    """A request carrying a single file path."""
    # The input file path.
    path: str

@dataclass
class RandomOptions:
    """A deterministic random request."""
    # The exclusive upper bound.
    max: float
    # The inclusive lower bound.
    min: float

@dataclass
class RenderShapesOptions:
    """A shape-rendering request: a canvas, a background and a shape list."""
    # The canvas width in pixels.
    width: int
    # The canvas height in pixels.
    height: int
    # The shapes to draw, in order.
    shapes: List[Shape]
    # The output file path (its extension selects the format).
    output: str
    # The background colour (default transparent).
    background: Optional[RgbaColor] = None

@dataclass
class SegmentAddIndexOptions:
    """Record a depth index on a segment."""
    # The index to record.
    index: float

@dataclass
class SegmentChangePositionOptions:
    """Move a segment to another osu! layer (`ChangePosition`)."""
    # The destination osu! layer.
    osbLayer: OsbLayer

@dataclass
class SegmentOptions:
    """A segment handle request."""


@dataclass
class SegmentReverseDepthOptions:
    """Set a segment's reverse-depth flag."""
    # Whether children are emitted in reverse order.
    value: bool

@dataclass
class SimplifyOptions:
    """A Douglas-Peucker simplification request."""
    # The polyline to simplify.
    points: List[Vector2]
    # The maximum deviation to allow.
    tolerance: float

@dataclass
class SpriteTimeOptions:
    """A sprite handle pair plus a sample time."""
    # The sample time in milliseconds.
    time: float

@dataclass
class StartLoopGroupOptions:
    """Start a loop group."""
    # The loop start time in milliseconds.
    startTime: float
    # The number of iterations.
    count: float

@dataclass
class StartTriggerGroupOptions:
    """Start a trigger group."""
    # The trigger name (matches a hit object).
    name: str
    # The trigger start time in milliseconds.
    startTime: float
    # The trigger end time in milliseconds.
    endTime: float
    # The trigger group index.
    group: int

@dataclass
class TextOptions:
    """A request carrying a block of text (an effect script, `.osu` or `.osb`)."""
    # The text payload.
    text: str

@dataclass
class TimeOptions:
    """A time query."""
    # The time in milliseconds.
    time: float


def to_kwargs(options: object) -> Dict[str, Any]:
    """Convert a clean options dataclass to request model keyword arguments."""
    return {key: value for key, value in asdict(options).items() if value is not None}
