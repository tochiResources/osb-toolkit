package moe.tochi.osbtk.client;

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
import java.util.List;

public final class Curve {
    private final CurveAPI api;

    public Curve(CurveAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Curve;
    }

    public PointListResult arc(ArcRequest body) throws ApiException {
        return api.arc(body);
    }

    public Box2 boundingBox(CurveRequest body) throws ApiException {
        return api.boundingBox(body);
    }

    public ScalarResult curvature(CurvePointRequest body) throws ApiException {
        return api.curvature(body);
    }

    public ScalarResult distanceTo(CurveDistanceRequest body) throws ApiException {
        return api.distanceTo(body);
    }

    public PointListResult ellipse(EllipseRequest body) throws ApiException {
        return api.ellipse(body);
    }

    public ScalarResult length(CurveRequest body) throws ApiException {
        return api.length(body);
    }

    public OptionalPointResult lineIntersect(LineIntersectRequest body) throws ApiException {
        return api.lineIntersect(body);
    }

    public Vector2 normal(CurvePointRequest body) throws ApiException {
        return api.normal(body);
    }

    public PointListResult offset(CurveOffsetRequest body) throws ApiException {
        return api.offset(body);
    }

    public Vector2 point(CurvePointRequest body) throws ApiException {
        return api.point(body);
    }

    public PointListResult sample(CurveSampleRequest body) throws ApiException {
        return api.sample(body);
    }

    public PointListResult simplify(SimplifyRequest body) throws ApiException {
        return api.simplify(body);
    }

    public Vector2 tangent(CurvePointRequest body) throws ApiException {
        return api.tangent(body);
    }
}
