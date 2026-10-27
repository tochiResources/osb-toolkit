import type { CurveSpec } from '../models/CurveSpec';
import type { Easing } from '../models/Easing';
import type { ImageOptions } from '../models/ImageOptions';
import type { OptimizeOptionsInput } from '../models/OptimizeOptionsInput';
import type { OsbLayer } from '../models/OsbLayer';
import type { OsbLoopType } from '../models/OsbLoopType';
import type { OsbOrigin } from '../models/OsbOrigin';
import type { ParameterType } from '../models/ParameterType';
import type { RgbaColor } from '../models/RgbaColor';
import type { Scene3dInput } from '../models/Scene3dInput';
import type { Shape } from '../models/Shape';
import type { Vector2 } from '../models/Vector2';

/**
 * An easing evaluation request.
 */
export interface ApplyEasingOptions {
    /**
     * The easing function.
     */
    easing: Easing;
    /**
     * The normalized time in `0.0..=1.0`.
     */
    t: number;
}

/**
 * A circular-arc sampling request.
 */
export interface ArcOptions {
    /**
     * The arc centre.
     */
    center: Vector2;
    /**
     * The number of points (default `64`).
     */
    count?: number | null;
    /**
     * The radius.
     */
    radius: number;
    /**
     * The start angle in degrees.
     */
    startAngle: number;
    /**
     * The sweep angle in degrees.
     */
    sweepAngle: number;
}

/**
 * A `bake3d` request: a plain 3D scene.
 */
export interface Bake3dOptions {
    /**
     * The 3D scene.
     */
    input: Scene3dInput;
}

/**
 * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
 * depending on `kind`.
 */
export interface CommandOptions {
    /**
     * The easing function.
     */
    easing: Easing;
    /**
     * The end value (scalar, point or color).
     */
    end?: object;
    /**
     * The command end time in milliseconds.
     */
    endTime: number;
    /**
     * The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
     * `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
     * `flipV`, `additive`).
     */
    kind: string;
    /**
     * The parameter kind for a `parameter` command.
     */
    parameter?: ParameterType | null;
    /**
     * The start value (scalar, point or color).
     */
    start?: object;
    /**
     * The command start time in milliseconds.
     */
    startTime: number;
}

/**
 * Create an animation sprite.
 */
export interface CreateAnimationOptions {
    /**
     * Delay between frames, in milliseconds.
     */
    frameDelay: number;
    /**
     * The number of frames.
     */
    frames: number;
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    layer?: string | null;
    /**
     * How the animation loops.
     */
    loopType: OsbLoopType;
    /**
     * The sprite anchor origin.
     */
    origin: OsbOrigin;
    /**
     * The texture path (relative to the mapset).
     */
    path: string;
    /**
     * The initial position.
     */
    position: Vector2;
}

/**
 * Create a sound sample on a layer.
 */
export interface CreateSampleOptions {
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    layer?: string | null;
    /**
     * The sample path (relative to the mapset).
     */
    path: string;
    /**
     * The sample time in milliseconds.
     */
    time: number;
    /**
     * The sample volume (`0..=100`).
     */
    volume?: number | null;
}

/**
 * Create a sprite.
 */
export interface CreateSpriteOptions {
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    layer?: string | null;
    /**
     * The sprite anchor origin.
     */
    origin: OsbOrigin;
    /**
     * The texture path (relative to the mapset).
     */
    path: string;
    /**
     * The initial position; omitted means the sprite's default.
     */
    position?: Vector2 | null;
}

/**
 * A curve hit-test request.
 */
export interface CurveDistanceOptions {
    /**
     * The curve.
     */
    curve: CurveSpec;
    /**
     * The query point.
     */
    point: Vector2;
}

/**
 * A curve offset request.
 */
export interface CurveOffsetOptions {
    /**
     * The curve.
     */
    curve: CurveSpec;
    /**
     * The signed offset along the curve's local normal.
     */
    distance: number;
}

/**
 * A curve query at a parametric or arc-length position.
 */
export interface CurvePointOptions {
    /**
     * Sample by arc length instead of the raw parameter (default `false`).
     */
    byArcLength?: boolean | null;
    /**
     * The curve.
     */
    curve: CurveSpec;
    /**
     * The position in `0.0..=1.0`.
     */
    t: number;
}

/**
 * A curve query.
 */
export interface CurveOptions {
    /**
     * The curve.
     */
    curve: CurveSpec;
}

/**
 * A curve sampling request.
 */
export interface CurveSampleOptions {
    /**
     * Sample by arc length instead of the raw parameter (default `false`).
     */
    byArcLength?: boolean | null;
    /**
     * The number of points to sample.
     */
    count: number;
    /**
     * The curve.
     */
    curve: CurveSpec;
}

/**
 * A named-or-custom easing evaluation request.
 */
export interface EasingValueOptions {
    /**
     * A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
     */
    cubicBezier?: number[] | null;
    /**
     * A named easing (used when `cubicBezier` is absent).
     */
    easing?: Easing | null;
    /**
     * The normalized time in `0.0..=1.0`.
     */
    t: number;
}

/**
 * An ellipse sampling request.
 */
export interface EllipseOptions {
    /**
     * The ellipse centre.
     */
    center: Vector2;
    /**
     * The number of points (default `64`).
     */
    count?: number | null;
    /**
     * The horizontal radius.
     */
    radiusX: number;
    /**
     * The vertical radius.
     */
    radiusY: number;
}

/**
 * An image-sequence encoding request (frames -> video).
 */
export interface EncodeVideoOptions {
    /**
     * The video codec (default `libx264`).
     */
    codec?: string | null;
    /**
     * The constant-rate-factor quality (`0`-`51`; lower is better).
     */
    crf?: number | null;
    /**
     * The frame rate.
     */
    fps: number;
    /**
     * The input filename pattern (e.g. `frames/frame_%04d.png`).
     */
    inputPattern: string;
    /**
     * The output video path.
     */
    output: string;
}

/**
 * An audio-extraction request (media -> audio file).
 */
export interface ExtractAudioOptions {
    /**
     * The input media path.
     */
    input: string;
    /**
     * The output audio path (its extension selects the format).
     */
    output: string;
}

/**
 * A frame-extraction request (video -> image sequence).
 */
export interface ExtractFramesOptions {
    /**
     * The maximum number of frames to extract.
     */
    count?: number | null;
    /**
     * The sampling rate (frames per second); defaults to the source rate.
     */
    fps?: number | null;
    /**
     * Scale frames to this height.
     */
    height?: number | null;
    /**
     * The input media path.
     */
    input: string;
    /**
     * The directory to write frames into.
     */
    outputDir: string;
    /**
     * The start time in seconds.
     */
    start?: number | null;
    /**
     * Scale frames to this width.
     */
    width?: number | null;
}

/**
 * An FFT request against a `.wav` file.
 */
export interface FftOptions {
    /**
     * Number of frequency bins.
     */
    magnitudes?: number | null;
    /**
     * Path to the audio file.
     */
    path: string;
    /**
     * Sample time in milliseconds.
     */
    time?: number | null;
}

/**
 * An FFT request against the runtime's loaded audio.
 */
export interface GetFftOptions {
    /**
     * A frequency cut-off.
     */
    cutoff?: number | null;
    /**
     * The easing applied across the bins.
     */
    easing?: Easing | null;
    /**
     * Number of frequency bins.
     */
    magnitudes: number;
    /**
     * Sample time in milliseconds.
     */
    time: number;
}

/**
 * The anchor offset for an origin.
 */
export interface GetOriginVectorOptions {
    /**
     * The sprite height.
     */
    height: number;
    /**
     * The sprite anchor origin.
     */
    origin: OsbOrigin;
    /**
     * The sprite width.
     */
    width: number;
}

/**
 * A sprite handle pair.
 */
export interface HandleOptions {

}

/**
 * A motion-graphics helper call: a helper `name` plus its input.
 */
export interface HelperOptions {
    /**
     * The helper's input (a keyframe channel, a command list, or its parameters).
     */
    input?: unknown;
    /**
     * The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     */
    name: string;
}

/**
 * An image processing request: read `path`, apply `options`, write `output`.
 */
export interface ImageProcessOptions {
    /**
     * The processing options.
     */
    options: ImageOptions;
    /**
     * The output file path (its extension selects the format).
     */
    output: string;
    /**
     * The input file path.
     */
    path: string;
}

/**
 * Whether a rotated sprite's bounding box intersects the widescreen
 * storyboard area.
 */
export interface InScreenBoundsOptions {
    /**
     * The anchor offset (from `GetOriginVector`).
     */
    origin: Vector2;
    /**
     * The sprite centre position.
     */
    position: Vector2;
    /**
     * The rotation in radians.
     */
    rotation: number;
    /**
     * The sprite size.
     */
    size: Vector2;
}

/**
 * Resolve (or create) a layer by name.
 */
export interface LayerHandleOptions {
    /**
     * The layer name.
     */
    name: string;
}

/**
 * A line-segment intersection request.
 */
export interface LineIntersectOptions {
    /**
     * The first segment's start.
     */
    a1: Vector2;
    /**
     * The first segment's end.
     */
    a2: Vector2;
    /**
     * The second segment's start.
     */
    b1: Vector2;
    /**
     * The second segment's end.
     */
    b2: Vector2;
}

/**
 * An `.osb` optimization request.
 */
export interface OptimizeOptions {
    /**
     * The optimization options.
     */
    options: OptimizeOptionsInput;
    /**
     * The `.osb` storyboard text.
     */
    text: string;
}

/**
 * A request carrying a single file path.
 */
export interface PathOptions {
    /**
     * The input file path.
     */
    path: string;
}

/**
 * A deterministic random request.
 */
export interface RandomOptions {
    /**
     * The exclusive upper bound.
     */
    max: number;
    /**
     * The inclusive lower bound.
     */
    min: number;
}

/**
 * A shape-rendering request: a canvas, a background and a shape list.
 */
export interface RenderShapesOptions {
    /**
     * The background colour (default transparent).
     */
    background?: RgbaColor | null;
    /**
     * The canvas height in pixels.
     */
    height: number;
    /**
     * The output file path (its extension selects the format).
     */
    output: string;
    /**
     * The shapes to draw, in order.
     */
    shapes: Shape[];
    /**
     * The canvas width in pixels.
     */
    width: number;
}

/**
 * Record a depth index on a segment.
 */
export interface SegmentAddIndexOptions {
    /**
     * The index to record.
     */
    index: number;
}

/**
 * Move a segment to another osu! layer (`ChangePosition`).
 */
export interface SegmentChangePositionOptions {
    /**
     * The destination osu! layer.
     */
    osbLayer: OsbLayer;
}

/**
 * A segment handle request.
 */
export interface SegmentOptions {

}

/**
 * Set a segment's reverse-depth flag.
 */
export interface SegmentReverseDepthOptions {
    /**
     * Whether children are emitted in reverse order.
     */
    value: boolean;
}

/**
 * A Douglas-Peucker simplification request.
 */
export interface SimplifyOptions {
    /**
     * The polyline to simplify.
     */
    points: Vector2[];
    /**
     * The maximum deviation to allow.
     */
    tolerance: number;
}

/**
 * A sprite handle pair plus a sample time.
 */
export interface SpriteTimeOptions {
    /**
     * The sample time in milliseconds.
     */
    time: number;
}

/**
 * Start a loop group.
 */
export interface StartLoopGroupOptions {
    /**
     * The number of iterations.
     */
    count: number;
    /**
     * The loop start time in milliseconds.
     */
    startTime: number;
}

/**
 * Start a trigger group.
 */
export interface StartTriggerGroupOptions {
    /**
     * The trigger end time in milliseconds.
     */
    endTime: number;
    /**
     * The trigger group index.
     */
    group: number;
    /**
     * The trigger name (matches a hit object).
     */
    name: string;
    /**
     * The trigger start time in milliseconds.
     */
    startTime: number;
}

/**
 * A request carrying a block of text (an effect script, `.osu` or `.osb`).
 */
export interface TextOptions {
    /**
     * The text payload.
     */
    text: string;
}

/**
 * A time query.
 */
export interface TimeOptions {
    /**
     * The time in milliseconds.
     */
    time: number;
}
