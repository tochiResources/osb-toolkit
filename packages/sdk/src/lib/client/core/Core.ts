import { CoreAPI } from '../../api/CoreAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { Box2 } from '../../models/Box2';
import type { CurveSpec } from '../../models/CurveSpec';
import type { Easing } from '../../models/Easing';
import type { OsbOrigin } from '../../models/OsbOrigin';
import type { ParseOsbResult } from '../../models/ParseOsbResult';
import type { PointListResult } from '../../models/PointListResult';
import type { Vector2 } from '../../models/Vector2';
import type { ApplyEasingOptions, ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EasingValueOptions, EllipseOptions, GetOriginVectorOptions, InScreenBoundsOptions, LineIntersectOptions, SimplifyOptions, TextOptions } from '../options';
import { isApplyEasingOptions, isArcOptions, isCurveDistanceOptions, isCurveOffsetOptions, isCurveOptions, isCurvePointOptions, isCurveSampleOptions, isEasingValueOptions, isEllipseOptions, isGetOriginVectorOptions, isInScreenBoundsOptions, isLineIntersectOptions, isSimplifyOptions, isTextOptions } from '../options';

/**
 * The clean `Core` client.
 */
export class Core {
    constructor(private readonly api: CoreAPI) {}

    static instanceOf(value: unknown): value is Core {
        return value instanceof Core;
    }

    /**
     * An easing evaluation request.
     *
     * @param easing The easing function.
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @returns The eased value
     */
    async applyEasing(easing: Easing, t: number): Promise<number>;
    /**
     * An easing evaluation request.
     *
     * @param options An easing evaluation request.
     *
     * @returns The eased value
     */
    async applyEasing(options: ApplyEasingOptions): Promise<number>;
    async applyEasing(optionsOrEasing?: ApplyEasingOptions | Easing, t?: number): Promise<number> {
        const options: ApplyEasingOptions = isApplyEasingOptions(optionsOrEasing)
            ? optionsOrEasing
            : ({ easing: optionsOrEasing, t: t } as ApplyEasingOptions);
        return (await this.api.applyEasing(options)).value;
    }

    /**
     * A circular-arc sampling request.
     *
     * @param center The arc centre.
     * @param radius The radius.
     * @param startAngle The start angle in degrees.
     * @param sweepAngle The sweep angle in degrees.
     *
     * @returns Points along a circular arc
     */
    async arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number): Promise<PointListResult>;
    /**
     * A circular-arc sampling request.
     *
     * @param center The arc centre.
     * @param radius The radius.
     * @param startAngle The start angle in degrees.
     * @param sweepAngle The sweep angle in degrees.
     * @param count The number of points (default `64`).
     *
     * @returns Points along a circular arc
     */
    async arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number, count: number | null): Promise<PointListResult>;
    /**
     * A circular-arc sampling request.
     *
     * @param options A circular-arc sampling request.
     *
     * @returns Points along a circular arc
     */
    async arc(options: ArcOptions): Promise<PointListResult>;
    async arc(optionsOrCenter?: ArcOptions | Vector2, radius?: number, startAngle?: number, sweepAngle?: number, count?: number | null): Promise<PointListResult> {
        const options: ArcOptions = isArcOptions(optionsOrCenter)
            ? optionsOrCenter
            : ({ center: optionsOrCenter, radius: radius, startAngle: startAngle, sweepAngle: sweepAngle, count: count } as ArcOptions);
        return this.api.arc(options);
    }

    /**
     * A curve query.
     *
     * @param curve The curve.
     *
     * @returns The curve's axis-aligned bounding box
     */
    async boundingBox(curve: CurveSpec): Promise<Box2>;
    /**
     * A curve query.
     *
     * @param options A curve query.
     *
     * @returns The curve's axis-aligned bounding box
     */
    async boundingBox(options: CurveOptions): Promise<Box2>;
    async boundingBox(optionsOrCurve?: CurveOptions | CurveSpec): Promise<Box2> {
        const options: CurveOptions = isCurveOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve } as CurveOptions);
        return this.api.boundingBox(options);
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @returns The curvature at `t`
     */
    async curvature(curve: CurveSpec, t: number): Promise<number>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @returns The curvature at `t`
     */
    async curvature(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<number>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @returns The curvature at `t`
     */
    async curvature(options: CurvePointOptions): Promise<number>;
    async curvature(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<number> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return (await this.api.curvature(options)).value;
    }

    /**
     * A curve hit-test request.
     *
     * @param curve The curve.
     * @param point The query point.
     *
     * @returns The distance from a point to the curve
     */
    async distanceTo(curve: CurveSpec, point: Vector2): Promise<number>;
    /**
     * A curve hit-test request.
     *
     * @param options A curve hit-test request.
     *
     * @returns The distance from a point to the curve
     */
    async distanceTo(options: CurveDistanceOptions): Promise<number>;
    async distanceTo(optionsOrCurve?: CurveDistanceOptions | CurveSpec, point?: Vector2): Promise<number> {
        const options: CurveDistanceOptions = isCurveDistanceOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, point: point } as CurveDistanceOptions);
        return (await this.api.distanceTo(options)).value;
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number, easing: Easing | null): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     * @param cubicBezier A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number, easing: Easing | null, cubicBezier: number[] | null): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param options A named-or-custom easing evaluation request.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(options: EasingValueOptions): Promise<number>;
    async easingValue(optionsOrT?: EasingValueOptions | number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<number> {
        const options: EasingValueOptions = isEasingValueOptions(optionsOrT)
            ? optionsOrT
            : ({ t: optionsOrT, easing: easing, cubicBezier: cubicBezier } as EasingValueOptions);
        return (await this.api.easingValue(options)).value;
    }

    /**
     * An ellipse sampling request.
     *
     * @param center The ellipse centre.
     * @param radiusX The horizontal radius.
     * @param radiusY The vertical radius.
     *
     * @returns Points around an ellipse
     */
    async ellipse(center: Vector2, radiusX: number, radiusY: number): Promise<PointListResult>;
    /**
     * An ellipse sampling request.
     *
     * @param center The ellipse centre.
     * @param radiusX The horizontal radius.
     * @param radiusY The vertical radius.
     * @param count The number of points (default `64`).
     *
     * @returns Points around an ellipse
     */
    async ellipse(center: Vector2, radiusX: number, radiusY: number, count: number | null): Promise<PointListResult>;
    /**
     * An ellipse sampling request.
     *
     * @param options An ellipse sampling request.
     *
     * @returns Points around an ellipse
     */
    async ellipse(options: EllipseOptions): Promise<PointListResult>;
    async ellipse(optionsOrCenter?: EllipseOptions | Vector2, radiusX?: number, radiusY?: number, count?: number | null): Promise<PointListResult> {
        const options: EllipseOptions = isEllipseOptions(optionsOrCenter)
            ? optionsOrCenter
            : ({ center: optionsOrCenter, radiusX: radiusX, radiusY: radiusY, count: count } as EllipseOptions);
        return this.api.ellipse(options);
    }

    /**
     * The anchor offset for an origin.
     *
     * @param origin The sprite anchor origin.
     * @param width The sprite width.
     * @param height The sprite height.
     *
     * @returns The anchor offset for the origin
     */
    async getOriginVector(origin: OsbOrigin, width: number, height: number): Promise<Vector2>;
    /**
     * The anchor offset for an origin.
     *
     * @param options The anchor offset for an origin.
     *
     * @returns The anchor offset for the origin
     */
    async getOriginVector(options: GetOriginVectorOptions): Promise<Vector2>;
    async getOriginVector(optionsOrOrigin?: GetOriginVectorOptions | OsbOrigin, width?: number, height?: number): Promise<Vector2> {
        const options: GetOriginVectorOptions = isGetOriginVectorOptions(optionsOrOrigin)
            ? optionsOrOrigin
            : ({ origin: optionsOrOrigin, width: width, height: height } as GetOriginVectorOptions);
        return this.api.getOriginVector(options);
    }

    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param position The sprite centre position.
     * @param size The sprite size.
     * @param rotation The rotation in radians.
     * @param origin The anchor offset (from `GetOriginVector`).
     *
     * @returns Whether the sprite's bounds intersect the screen
     */
    async inScreenBounds(position: Vector2, size: Vector2, rotation: number, origin: Vector2): Promise<boolean>;
    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param options Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @returns Whether the sprite's bounds intersect the screen
     */
    async inScreenBounds(options: InScreenBoundsOptions): Promise<boolean>;
    async inScreenBounds(optionsOrPosition?: InScreenBoundsOptions | Vector2, size?: Vector2, rotation?: number, origin?: Vector2): Promise<boolean> {
        const options: InScreenBoundsOptions = isInScreenBoundsOptions(optionsOrPosition)
            ? optionsOrPosition
            : ({ position: optionsOrPosition, size: size, rotation: rotation, origin: origin } as InScreenBoundsOptions);
        return (await this.api.inScreenBounds(options)).value;
    }

    /**
     * A curve query.
     *
     * @param curve The curve.
     *
     * @returns The curve's arc length
     */
    async length(curve: CurveSpec): Promise<number>;
    /**
     * A curve query.
     *
     * @param options A curve query.
     *
     * @returns The curve's arc length
     */
    async length(options: CurveOptions): Promise<number>;
    async length(optionsOrCurve?: CurveOptions | CurveSpec): Promise<number> {
        const options: CurveOptions = isCurveOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve } as CurveOptions);
        return (await this.api.length(options)).value;
    }

    /**
     * A line-segment intersection request.
     *
     * @param a1 The first segment's start.
     * @param a2 The first segment's end.
     * @param b1 The second segment's start.
     * @param b2 The second segment's end.
     *
     * @returns The intersection point, or null
     */
    async lineIntersect(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2): Promise<Vector2 | null>;
    /**
     * A line-segment intersection request.
     *
     * @param options A line-segment intersection request.
     *
     * @returns The intersection point, or null
     */
    async lineIntersect(options: LineIntersectOptions): Promise<Vector2 | null>;
    async lineIntersect(optionsOrA1?: LineIntersectOptions | Vector2, a2?: Vector2, b1?: Vector2, b2?: Vector2): Promise<Vector2 | null> {
        const options: LineIntersectOptions = isLineIntersectOptions(optionsOrA1)
            ? optionsOrA1
            : ({ a1: optionsOrA1, a2: a2, b1: b1, b2: b2 } as LineIntersectOptions);
        return (await this.api.lineIntersect(options)).point ?? null;
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @returns The unit normal at `t`
     */
    async normal(curve: CurveSpec, t: number): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @returns The unit normal at `t`
     */
    async normal(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @returns The unit normal at `t`
     */
    async normal(options: CurvePointOptions): Promise<Vector2>;
    async normal(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.normal(options);
    }

    /**
     * A curve offset request.
     *
     * @param curve The curve.
     * @param distance The signed offset along the curve's local normal.
     *
     * @returns The polyline offset along the curve's normal
     */
    async offset(curve: CurveSpec, distance: number): Promise<PointListResult>;
    /**
     * A curve offset request.
     *
     * @param options A curve offset request.
     *
     * @returns The polyline offset along the curve's normal
     */
    async offset(options: CurveOffsetOptions): Promise<PointListResult>;
    async offset(optionsOrCurve?: CurveOffsetOptions | CurveSpec, distance?: number): Promise<PointListResult> {
        const options: CurveOffsetOptions = isCurveOffsetOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, distance: distance } as CurveOffsetOptions);
        return this.api.offset(options);
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns The storyboard summary
     */
    async parseOsb(text: string): Promise<ParseOsbResult>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns The storyboard summary
     */
    async parseOsb(options: TextOptions): Promise<ParseOsbResult>;
    async parseOsb(optionsOrText?: TextOptions | string): Promise<ParseOsbResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsb(options);
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns The parsed beatmap
     */
    async parseOsu(text: string): Promise<Beatmap>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns The parsed beatmap
     */
    async parseOsu(options: TextOptions): Promise<Beatmap>;
    async parseOsu(optionsOrText?: TextOptions | string): Promise<Beatmap> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsu(options);
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @returns The point at `t`, parametrically or by arc length
     */
    async point(curve: CurveSpec, t: number): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @returns The point at `t`, parametrically or by arc length
     */
    async point(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @returns The point at `t`, parametrically or by arc length
     */
    async point(options: CurvePointOptions): Promise<Vector2>;
    async point(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.point(options);
    }

    /**
     * A curve sampling request.
     *
     * @param count The number of points to sample.
     * @param curve The curve.
     *
     * @returns `count` points along the curve
     */
    async sample(count: number, curve: CurveSpec): Promise<PointListResult>;
    /**
     * A curve sampling request.
     *
     * @param count The number of points to sample.
     * @param curve The curve.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @returns `count` points along the curve
     */
    async sample(count: number, curve: CurveSpec, byArcLength: boolean | null): Promise<PointListResult>;
    /**
     * A curve sampling request.
     *
     * @param options A curve sampling request.
     *
     * @returns `count` points along the curve
     */
    async sample(options: CurveSampleOptions): Promise<PointListResult>;
    async sample(optionsOrCount?: CurveSampleOptions | number, curve?: CurveSpec, byArcLength?: boolean | null): Promise<PointListResult> {
        const options: CurveSampleOptions = isCurveSampleOptions(optionsOrCount)
            ? optionsOrCount
            : ({ count: optionsOrCount, curve: curve, byArcLength: byArcLength } as CurveSampleOptions);
        return this.api.sample(options);
    }

    /**
     * A Douglas-Peucker simplification request.
     *
     * @param points The polyline to simplify.
     * @param tolerance The maximum deviation to allow.
     *
     * @returns The simplified polyline
     */
    async simplify(points: Vector2[], tolerance: number): Promise<PointListResult>;
    /**
     * A Douglas-Peucker simplification request.
     *
     * @param options A Douglas-Peucker simplification request.
     *
     * @returns The simplified polyline
     */
    async simplify(options: SimplifyOptions): Promise<PointListResult>;
    async simplify(optionsOrPoints?: SimplifyOptions | Vector2[], tolerance?: number): Promise<PointListResult> {
        const options: SimplifyOptions = isSimplifyOptions(optionsOrPoints)
            ? optionsOrPoints
            : ({ points: optionsOrPoints, tolerance: tolerance } as SimplifyOptions);
        return this.api.simplify(options);
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @returns The unit tangent at `t`
     */
    async tangent(curve: CurveSpec, t: number): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @returns The unit tangent at `t`
     */
    async tangent(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @returns The unit tangent at `t`
     */
    async tangent(options: CurvePointOptions): Promise<Vector2>;
    async tangent(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.tangent(options);
    }

    /**
     * Calls the Core.version bridge operation.
     *
     * @returns The runtime version
     */
    async version(): Promise<string> {
        return (await this.api.version()).version;
    }
}
