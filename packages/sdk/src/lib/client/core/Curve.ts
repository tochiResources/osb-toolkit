import { CurveAPI } from '../../api/CurveAPI';
import type { Box2 } from '../../models/Box2';
import type { CurveSpec } from '../../models/CurveSpec';
import type { PointListResult } from '../../models/PointListResult';
import type { Vector2 } from '../../models/Vector2';
import type { ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions } from '../options';
import { isArcOptions, isCurveDistanceOptions, isCurveOffsetOptions, isCurveOptions, isCurvePointOptions, isCurveSampleOptions, isEllipseOptions, isLineIntersectOptions, isSimplifyOptions } from '../options';

/**
 * The clean `Curve` client.
 */
export class Curve {
    constructor(private readonly api: CurveAPI) {}

    static instanceOf(value: unknown): value is Curve {
        return value instanceof Curve;
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
}
