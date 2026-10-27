import { CurveAPI } from '../../api/CurveAPI';
import type { ArcRequest } from '../../models/ArcRequest';
import type { Box2 } from '../../models/Box2';
import type { CurveDistanceRequest } from '../../models/CurveDistanceRequest';
import type { CurveOffsetRequest } from '../../models/CurveOffsetRequest';
import type { CurvePointRequest } from '../../models/CurvePointRequest';
import type { CurveRequest } from '../../models/CurveRequest';
import type { CurveSampleRequest } from '../../models/CurveSampleRequest';
import type { EllipseRequest } from '../../models/EllipseRequest';
import type { LineIntersectRequest } from '../../models/LineIntersectRequest';
import type { OptionalPointResult } from '../../models/OptionalPointResult';
import type { PointListResult } from '../../models/PointListResult';
import type { ScalarResult } from '../../models/ScalarResult';
import type { SimplifyRequest } from '../../models/SimplifyRequest';
import type { Vector2 } from '../../models/Vector2';

export class Curve {
    constructor(private readonly api: CurveAPI) {}

    static instanceOf(value: unknown): value is Curve {
        return value instanceof Curve;
    }

    async arc(body: ArcRequest): Promise<PointListResult> {
        return this.api.arc(body);
    }

    async boundingBox(body: CurveRequest): Promise<Box2> {
        return this.api.boundingBox(body);
    }

    async curvature(body: CurvePointRequest): Promise<ScalarResult> {
        return this.api.curvature(body);
    }

    async distanceTo(body: CurveDistanceRequest): Promise<ScalarResult> {
        return this.api.distanceTo(body);
    }

    async ellipse(body: EllipseRequest): Promise<PointListResult> {
        return this.api.ellipse(body);
    }

    async length(body: CurveRequest): Promise<ScalarResult> {
        return this.api.length(body);
    }

    async lineIntersect(body: LineIntersectRequest): Promise<OptionalPointResult> {
        return this.api.lineIntersect(body);
    }

    async normal(body: CurvePointRequest): Promise<Vector2> {
        return this.api.normal(body);
    }

    async offset(body: CurveOffsetRequest): Promise<PointListResult> {
        return this.api.offset(body);
    }

    async point(body: CurvePointRequest): Promise<Vector2> {
        return this.api.point(body);
    }

    async sample(body: CurveSampleRequest): Promise<PointListResult> {
        return this.api.sample(body);
    }

    async simplify(body: SimplifyRequest): Promise<PointListResult> {
        return this.api.simplify(body);
    }

    async tangent(body: CurvePointRequest): Promise<Vector2> {
        return this.api.tangent(body);
    }
}
