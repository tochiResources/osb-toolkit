package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.CurveAPI;
import moe.tochi.osbtk.model.ArcRequest;
import moe.tochi.osbtk.model.Box2;
import moe.tochi.osbtk.model.CurveDistanceRequest;
import moe.tochi.osbtk.model.CurveOffsetRequest;
import moe.tochi.osbtk.model.CurvePointRequest;
import moe.tochi.osbtk.model.CurveRequest;
import moe.tochi.osbtk.model.CurveSampleRequest;
import moe.tochi.osbtk.model.EllipseRequest;
import moe.tochi.osbtk.model.LineIntersectRequest;
import moe.tochi.osbtk.model.OptionalPointResult;
import moe.tochi.osbtk.model.PointListResult;
import moe.tochi.osbtk.model.ScalarResult;
import moe.tochi.osbtk.model.SimplifyRequest;
import moe.tochi.osbtk.model.Vector2;
import moe.tochi.osbtk.client.ArcOptions;
import moe.tochi.osbtk.client.CurveDistanceOptions;
import moe.tochi.osbtk.client.CurveOffsetOptions;
import moe.tochi.osbtk.client.CurveOptions;
import moe.tochi.osbtk.client.CurvePointOptions;
import moe.tochi.osbtk.client.CurveSampleOptions;
import moe.tochi.osbtk.client.EllipseOptions;
import moe.tochi.osbtk.client.LineIntersectOptions;
import moe.tochi.osbtk.client.SimplifyOptions;
import java.util.List;

public final class Curve {
    private final CurveAPI api;

    public Curve(CurveAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Curve;
    }

    public PointListResult arc(ArcOptions options) throws ApiException {
        return api.arc(new ArcRequest()
            .center(options.center)
            .count(options.count)
            .radius(options.radius)
            .startAngle(options.startAngle)
            .sweepAngle(options.sweepAngle));
    }

    public Box2 boundingBox(CurveOptions options) throws ApiException {
        return api.boundingBox(new CurveRequest()
            .curve(options.curve));
    }

    public ScalarResult curvature(CurvePointOptions options) throws ApiException {
        return api.curvature(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    public ScalarResult distanceTo(CurveDistanceOptions options) throws ApiException {
        return api.distanceTo(new CurveDistanceRequest()
            .curve(options.curve)
            .point(options.point));
    }

    public PointListResult ellipse(EllipseOptions options) throws ApiException {
        return api.ellipse(new EllipseRequest()
            .center(options.center)
            .count(options.count)
            .radiusX(options.radiusX)
            .radiusY(options.radiusY));
    }

    public ScalarResult length(CurveOptions options) throws ApiException {
        return api.length(new CurveRequest()
            .curve(options.curve));
    }

    public OptionalPointResult lineIntersect(LineIntersectOptions options) throws ApiException {
        return api.lineIntersect(new LineIntersectRequest()
            .a1(options.a1)
            .a2(options.a2)
            .b1(options.b1)
            .b2(options.b2));
    }

    public Vector2 normal(CurvePointOptions options) throws ApiException {
        return api.normal(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    public PointListResult offset(CurveOffsetOptions options) throws ApiException {
        return api.offset(new CurveOffsetRequest()
            .curve(options.curve)
            .distance(options.distance));
    }

    public Vector2 point(CurvePointOptions options) throws ApiException {
        return api.point(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    public PointListResult sample(CurveSampleOptions options) throws ApiException {
        return api.sample(new CurveSampleRequest()
            .byArcLength(options.byArcLength)
            .count(options.count)
            .curve(options.curve));
    }

    public PointListResult simplify(SimplifyOptions options) throws ApiException {
        return api.simplify(new SimplifyRequest()
            .points(options.points)
            .tolerance(options.tolerance));
    }

    public Vector2 tangent(CurvePointOptions options) throws ApiException {
        return api.tangent(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }
}
