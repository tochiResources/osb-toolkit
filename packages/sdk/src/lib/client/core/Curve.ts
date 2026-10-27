import { CurveAPI } from '../../api/CurveAPI';
import type { Box2 } from '../../models/Box2';
import type { OptionalPointResult } from '../../models/OptionalPointResult';
import type { PointListResult } from '../../models/PointListResult';
import type { ScalarResult } from '../../models/ScalarResult';
import type { Vector2 } from '../../models/Vector2';
import type { ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions } from '../options';

export class Curve {
    constructor(private readonly api: CurveAPI) {}

    static instanceOf(value: unknown): value is Curve {
        return value instanceof Curve;
    }

    async arc(options: ArcOptions): Promise<PointListResult> {
        return this.api.arc(options);
    }

    async boundingBox(options: CurveOptions): Promise<Box2> {
        return this.api.boundingBox(options);
    }

    async curvature(options: CurvePointOptions): Promise<ScalarResult> {
        return this.api.curvature(options);
    }

    async distanceTo(options: CurveDistanceOptions): Promise<ScalarResult> {
        return this.api.distanceTo(options);
    }

    async ellipse(options: EllipseOptions): Promise<PointListResult> {
        return this.api.ellipse(options);
    }

    async length(options: CurveOptions): Promise<ScalarResult> {
        return this.api.length(options);
    }

    async lineIntersect(options: LineIntersectOptions): Promise<OptionalPointResult> {
        return this.api.lineIntersect(options);
    }

    async normal(options: CurvePointOptions): Promise<Vector2> {
        return this.api.normal(options);
    }

    async offset(options: CurveOffsetOptions): Promise<PointListResult> {
        return this.api.offset(options);
    }

    async point(options: CurvePointOptions): Promise<Vector2> {
        return this.api.point(options);
    }

    async sample(options: CurveSampleOptions): Promise<PointListResult> {
        return this.api.sample(options);
    }

    async simplify(options: SimplifyOptions): Promise<PointListResult> {
        return this.api.simplify(options);
    }

    async tangent(options: CurvePointOptions): Promise<Vector2> {
        return this.api.tangent(options);
    }
}
