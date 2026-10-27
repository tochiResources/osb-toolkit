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

export function isApplyEasingOptions(value: unknown): value is ApplyEasingOptions {
    return typeof value === 'object' && value !== null && 'easing' in value && 't' in value;
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

export function isArcOptions(value: unknown): value is ArcOptions {
    return typeof value === 'object' && value !== null && 'center' in value && 'radius' in value && 'startAngle' in value && 'sweepAngle' in value;
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

export function isBake3dOptions(value: unknown): value is Bake3dOptions {
    return typeof value === 'object' && value !== null && 'input' in value;
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
    end?: unknown;
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
    start?: unknown;
    /**
     * The command start time in milliseconds.
     */
    startTime: number;
}

export function isCommandOptions(value: unknown): value is CommandOptions {
    return typeof value === 'object' && value !== null && 'easing' in value && 'endTime' in value && 'kind' in value && 'startTime' in value;
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

export function isCreateAnimationOptions(value: unknown): value is CreateAnimationOptions {
    return typeof value === 'object' && value !== null && 'frameDelay' in value && 'frames' in value && 'loopType' in value && 'origin' in value && 'path' in value && 'position' in value;
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

export function isCreateSampleOptions(value: unknown): value is CreateSampleOptions {
    return typeof value === 'object' && value !== null && 'path' in value && 'time' in value;
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

export function isCreateSpriteOptions(value: unknown): value is CreateSpriteOptions {
    return typeof value === 'object' && value !== null && 'origin' in value && 'path' in value;
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

export function isCurveDistanceOptions(value: unknown): value is CurveDistanceOptions {
    return typeof value === 'object' && value !== null && 'curve' in value && 'point' in value;
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

export function isCurveOffsetOptions(value: unknown): value is CurveOffsetOptions {
    return typeof value === 'object' && value !== null && 'curve' in value && 'distance' in value;
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

export function isCurvePointOptions(value: unknown): value is CurvePointOptions {
    return typeof value === 'object' && value !== null && 'curve' in value && 't' in value;
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

export function isCurveOptions(value: unknown): value is CurveOptions {
    return typeof value === 'object' && value !== null && 'curve' in value;
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

export function isCurveSampleOptions(value: unknown): value is CurveSampleOptions {
    return typeof value === 'object' && value !== null && 'count' in value && 'curve' in value;
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

export function isEasingValueOptions(value: unknown): value is EasingValueOptions {
    return typeof value === 'object' && value !== null && 't' in value;
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

export function isEllipseOptions(value: unknown): value is EllipseOptions {
    return typeof value === 'object' && value !== null && 'center' in value && 'radiusX' in value && 'radiusY' in value;
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

export function isEncodeVideoOptions(value: unknown): value is EncodeVideoOptions {
    return typeof value === 'object' && value !== null && 'fps' in value && 'inputPattern' in value && 'output' in value;
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

export function isExtractAudioOptions(value: unknown): value is ExtractAudioOptions {
    return typeof value === 'object' && value !== null && 'input' in value && 'output' in value;
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

export function isExtractFramesOptions(value: unknown): value is ExtractFramesOptions {
    return typeof value === 'object' && value !== null && 'input' in value && 'outputDir' in value;
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

export function isFftOptions(value: unknown): value is FftOptions {
    return typeof value === 'object' && value !== null && 'path' in value;
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

export function isGetFftOptions(value: unknown): value is GetFftOptions {
    return typeof value === 'object' && value !== null && 'magnitudes' in value && 'time' in value;
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

export function isGetOriginVectorOptions(value: unknown): value is GetOriginVectorOptions {
    return typeof value === 'object' && value !== null && 'height' in value && 'origin' in value && 'width' in value;
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

export function isHelperOptions(value: unknown): value is HelperOptions {
    return typeof value === 'object' && value !== null && 'name' in value;
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

export function isImageProcessOptions(value: unknown): value is ImageProcessOptions {
    return typeof value === 'object' && value !== null && 'options' in value && 'output' in value && 'path' in value;
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

export function isInScreenBoundsOptions(value: unknown): value is InScreenBoundsOptions {
    return typeof value === 'object' && value !== null && 'origin' in value && 'position' in value && 'rotation' in value && 'size' in value;
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

export function isLayerHandleOptions(value: unknown): value is LayerHandleOptions {
    return typeof value === 'object' && value !== null && 'name' in value;
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

export function isLineIntersectOptions(value: unknown): value is LineIntersectOptions {
    return typeof value === 'object' && value !== null && 'a1' in value && 'a2' in value && 'b1' in value && 'b2' in value;
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

export function isOptimizeOptions(value: unknown): value is OptimizeOptions {
    return typeof value === 'object' && value !== null && 'options' in value && 'text' in value;
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

export function isPathOptions(value: unknown): value is PathOptions {
    return typeof value === 'object' && value !== null && 'path' in value;
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

export function isRandomOptions(value: unknown): value is RandomOptions {
    return typeof value === 'object' && value !== null && 'max' in value && 'min' in value;
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

export function isRenderShapesOptions(value: unknown): value is RenderShapesOptions {
    return typeof value === 'object' && value !== null && 'height' in value && 'output' in value && 'shapes' in value && 'width' in value;
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

export function isSegmentAddIndexOptions(value: unknown): value is SegmentAddIndexOptions {
    return typeof value === 'object' && value !== null && 'index' in value;
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

export function isSegmentChangePositionOptions(value: unknown): value is SegmentChangePositionOptions {
    return typeof value === 'object' && value !== null && 'osbLayer' in value;
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

export function isSegmentReverseDepthOptions(value: unknown): value is SegmentReverseDepthOptions {
    return typeof value === 'object' && value !== null && 'value' in value;
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

export function isSimplifyOptions(value: unknown): value is SimplifyOptions {
    return typeof value === 'object' && value !== null && 'points' in value && 'tolerance' in value;
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

export function isSpriteTimeOptions(value: unknown): value is SpriteTimeOptions {
    return typeof value === 'object' && value !== null && 'time' in value;
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

export function isStartLoopGroupOptions(value: unknown): value is StartLoopGroupOptions {
    return typeof value === 'object' && value !== null && 'count' in value && 'startTime' in value;
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

export function isStartTriggerGroupOptions(value: unknown): value is StartTriggerGroupOptions {
    return typeof value === 'object' && value !== null && 'endTime' in value && 'group' in value && 'name' in value && 'startTime' in value;
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

export function isTextOptions(value: unknown): value is TextOptions {
    return typeof value === 'object' && value !== null && 'text' in value;
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

export function isTimeOptions(value: unknown): value is TimeOptions {
    return typeof value === 'object' && value !== null && 'time' in value;
}
