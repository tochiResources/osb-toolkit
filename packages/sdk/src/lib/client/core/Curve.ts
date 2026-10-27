import { CurveAPI } from '../../api/CurveAPI';
import type { Box2 } from '../../models/Box2';
import type { CurveSpec } from '../../models/CurveSpec';
import type { PointListResult } from '../../models/PointListResult';
import type { Vector2 } from '../../models/Vector2';
import type { ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions } from '../options';
import { isArcOptions, isCurveDistanceOptions, isCurveOffsetOptions, isCurveOptions, isCurvePointOptions, isCurveSampleOptions, isEllipseOptions, isLineIntersectOptions, isSimplifyOptions } from '../options';

export class Curve {
    constructor(private readonly api: CurveAPI) {}

    static instanceOf(value: unknown): value is Curve {
        return value instanceof Curve;
    }

    async arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number): Promise<PointListResult>;
    async arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number, count: number | null): Promise<PointListResult>;
    async arc(options: ArcOptions): Promise<PointListResult>;
    async arc(optionsOrCenter?: ArcOptions | Vector2, radius?: number, startAngle?: number, sweepAngle?: number, count?: number | null): Promise<PointListResult> {
        const options: ArcOptions = isArcOptions(optionsOrCenter)
            ? optionsOrCenter
            : ({ center: optionsOrCenter, radius: radius, startAngle: startAngle, sweepAngle: sweepAngle, count: count } as ArcOptions);
        return this.api.arc(options);
    }

    async boundingBox(curve: CurveSpec): Promise<Box2>;
    async boundingBox(options: CurveOptions): Promise<Box2>;
    async boundingBox(optionsOrCurve?: CurveOptions | CurveSpec): Promise<Box2> {
        const options: CurveOptions = isCurveOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve } as CurveOptions);
        return this.api.boundingBox(options);
    }

    async curvature(curve: CurveSpec, t: number): Promise<number>;
    async curvature(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<number>;
    async curvature(options: CurvePointOptions): Promise<number>;
    async curvature(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<number> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return (await this.api.curvature(options)).value;
    }

    async distanceTo(curve: CurveSpec, point: Vector2): Promise<number>;
    async distanceTo(options: CurveDistanceOptions): Promise<number>;
    async distanceTo(optionsOrCurve?: CurveDistanceOptions | CurveSpec, point?: Vector2): Promise<number> {
        const options: CurveDistanceOptions = isCurveDistanceOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, point: point } as CurveDistanceOptions);
        return (await this.api.distanceTo(options)).value;
    }

    async ellipse(center: Vector2, radiusX: number, radiusY: number): Promise<PointListResult>;
    async ellipse(center: Vector2, radiusX: number, radiusY: number, count: number | null): Promise<PointListResult>;
    async ellipse(options: EllipseOptions): Promise<PointListResult>;
    async ellipse(optionsOrCenter?: EllipseOptions | Vector2, radiusX?: number, radiusY?: number, count?: number | null): Promise<PointListResult> {
        const options: EllipseOptions = isEllipseOptions(optionsOrCenter)
            ? optionsOrCenter
            : ({ center: optionsOrCenter, radiusX: radiusX, radiusY: radiusY, count: count } as EllipseOptions);
        return this.api.ellipse(options);
    }

    async length(curve: CurveSpec): Promise<number>;
    async length(options: CurveOptions): Promise<number>;
    async length(optionsOrCurve?: CurveOptions | CurveSpec): Promise<number> {
        const options: CurveOptions = isCurveOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve } as CurveOptions);
        return (await this.api.length(options)).value;
    }

    async lineIntersect(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2): Promise<Vector2 | null>;
    async lineIntersect(options: LineIntersectOptions): Promise<Vector2 | null>;
    async lineIntersect(optionsOrA1?: LineIntersectOptions | Vector2, a2?: Vector2, b1?: Vector2, b2?: Vector2): Promise<Vector2 | null> {
        const options: LineIntersectOptions = isLineIntersectOptions(optionsOrA1)
            ? optionsOrA1
            : ({ a1: optionsOrA1, a2: a2, b1: b1, b2: b2 } as LineIntersectOptions);
        return (await this.api.lineIntersect(options)).point ?? null;
    }

    async normal(curve: CurveSpec, t: number): Promise<Vector2>;
    async normal(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    async normal(options: CurvePointOptions): Promise<Vector2>;
    async normal(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.normal(options);
    }

    async offset(curve: CurveSpec, distance: number): Promise<PointListResult>;
    async offset(options: CurveOffsetOptions): Promise<PointListResult>;
    async offset(optionsOrCurve?: CurveOffsetOptions | CurveSpec, distance?: number): Promise<PointListResult> {
        const options: CurveOffsetOptions = isCurveOffsetOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, distance: distance } as CurveOffsetOptions);
        return this.api.offset(options);
    }

    async point(curve: CurveSpec, t: number): Promise<Vector2>;
    async point(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    async point(options: CurvePointOptions): Promise<Vector2>;
    async point(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.point(options);
    }

    async sample(count: number, curve: CurveSpec): Promise<PointListResult>;
    async sample(count: number, curve: CurveSpec, byArcLength: boolean | null): Promise<PointListResult>;
    async sample(options: CurveSampleOptions): Promise<PointListResult>;
    async sample(optionsOrCount?: CurveSampleOptions | number, curve?: CurveSpec, byArcLength?: boolean | null): Promise<PointListResult> {
        const options: CurveSampleOptions = isCurveSampleOptions(optionsOrCount)
            ? optionsOrCount
            : ({ count: optionsOrCount, curve: curve, byArcLength: byArcLength } as CurveSampleOptions);
        return this.api.sample(options);
    }

    async simplify(points: Vector2[], tolerance: number): Promise<PointListResult>;
    async simplify(options: SimplifyOptions): Promise<PointListResult>;
    async simplify(optionsOrPoints?: SimplifyOptions | Vector2[], tolerance?: number): Promise<PointListResult> {
        const options: SimplifyOptions = isSimplifyOptions(optionsOrPoints)
            ? optionsOrPoints
            : ({ points: optionsOrPoints, tolerance: tolerance } as SimplifyOptions);
        return this.api.simplify(options);
    }

    async tangent(curve: CurveSpec, t: number): Promise<Vector2>;
    async tangent(curve: CurveSpec, t: number, byArcLength: boolean | null): Promise<Vector2>;
    async tangent(options: CurvePointOptions): Promise<Vector2>;
    async tangent(optionsOrCurve?: CurvePointOptions | CurveSpec, t?: number, byArcLength?: boolean | null): Promise<Vector2> {
        const options: CurvePointOptions = isCurvePointOptions(optionsOrCurve)
            ? optionsOrCurve
            : ({ curve: optionsOrCurve, t: t, byArcLength: byArcLength } as CurvePointOptions);
        return this.api.tangent(options);
    }
}
