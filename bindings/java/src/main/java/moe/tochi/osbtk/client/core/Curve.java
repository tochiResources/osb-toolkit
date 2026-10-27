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
import moe.tochi.osbtk.model.CurveSpec;
import moe.tochi.osbtk.model.EllipseRequest;
import moe.tochi.osbtk.model.LineIntersectRequest;
import moe.tochi.osbtk.model.PointListResult;
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

    public PointListResult arc(Vector2 center, Double radius, Double startAngle, Double sweepAngle) throws ApiException {
        return arc(ArcOptions.builder().center(center).radius(radius).startAngle(startAngle).sweepAngle(sweepAngle).build());
    }

    public PointListResult arc(Vector2 center, Double radius, Double startAngle, Double sweepAngle, Integer count) throws ApiException {
        return arc(ArcOptions.builder().center(center).radius(radius).startAngle(startAngle).sweepAngle(sweepAngle).count(count).build());
    }

    public PointListResult arc(ArcOptions options) throws ApiException {
        return api.arc(new ArcRequest()
            .center(options.center)
            .count(options.count)
            .radius(options.radius)
            .startAngle(options.startAngle)
            .sweepAngle(options.sweepAngle));
    }

    public Box2 boundingBox(CurveSpec curve) throws ApiException {
        return boundingBox(CurveOptions.builder().curve(curve).build());
    }

    public Box2 boundingBox(CurveOptions options) throws ApiException {
        return api.boundingBox(new CurveRequest()
            .curve(options.curve));
    }

    public double curvature(CurveSpec curve, Double t) throws ApiException {
        return curvature(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    public double curvature(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return curvature(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    public double curvature(CurvePointOptions options) throws ApiException {
        return api.curvature(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t)).getValue();
    }

    public double distanceTo(CurveSpec curve, Vector2 point) throws ApiException {
        return distanceTo(CurveDistanceOptions.builder().curve(curve).point(point).build());
    }

    public double distanceTo(CurveDistanceOptions options) throws ApiException {
        return api.distanceTo(new CurveDistanceRequest()
            .curve(options.curve)
            .point(options.point)).getValue();
    }

    public PointListResult ellipse(Vector2 center, Double radiusX, Double radiusY) throws ApiException {
        return ellipse(EllipseOptions.builder().center(center).radiusX(radiusX).radiusY(radiusY).build());
    }

    public PointListResult ellipse(Vector2 center, Double radiusX, Double radiusY, Integer count) throws ApiException {
        return ellipse(EllipseOptions.builder().center(center).radiusX(radiusX).radiusY(radiusY).count(count).build());
    }

    public PointListResult ellipse(EllipseOptions options) throws ApiException {
        return api.ellipse(new EllipseRequest()
            .center(options.center)
            .count(options.count)
            .radiusX(options.radiusX)
            .radiusY(options.radiusY));
    }

    public double length(CurveSpec curve) throws ApiException {
        return length(CurveOptions.builder().curve(curve).build());
    }

    public double length(CurveOptions options) throws ApiException {
        return api.length(new CurveRequest()
            .curve(options.curve)).getValue();
    }

    public Vector2 lineIntersect(Vector2 a1, Vector2 a2, Vector2 b1, Vector2 b2) throws ApiException {
        return lineIntersect(LineIntersectOptions.builder().a1(a1).a2(a2).b1(b1).b2(b2).build());
    }

    public Vector2 lineIntersect(LineIntersectOptions options) throws ApiException {
        return api.lineIntersect(new LineIntersectRequest()
            .a1(options.a1)
            .a2(options.a2)
            .b1(options.b1)
            .b2(options.b2)).getPoint();
    }

    public Vector2 normal(CurveSpec curve, Double t) throws ApiException {
        return normal(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    public Vector2 normal(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return normal(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    public Vector2 normal(CurvePointOptions options) throws ApiException {
        return api.normal(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    public PointListResult offset(CurveSpec curve, Double distance) throws ApiException {
        return offset(CurveOffsetOptions.builder().curve(curve).distance(distance).build());
    }

    public PointListResult offset(CurveOffsetOptions options) throws ApiException {
        return api.offset(new CurveOffsetRequest()
            .curve(options.curve)
            .distance(options.distance));
    }

    public Vector2 point(CurveSpec curve, Double t) throws ApiException {
        return point(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    public Vector2 point(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return point(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    public Vector2 point(CurvePointOptions options) throws ApiException {
        return api.point(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    public PointListResult sample(Integer count, CurveSpec curve) throws ApiException {
        return sample(CurveSampleOptions.builder().count(count).curve(curve).build());
    }

    public PointListResult sample(Integer count, CurveSpec curve, Boolean byArcLength) throws ApiException {
        return sample(CurveSampleOptions.builder().count(count).curve(curve).byArcLength(byArcLength).build());
    }

    public PointListResult sample(CurveSampleOptions options) throws ApiException {
        return api.sample(new CurveSampleRequest()
            .byArcLength(options.byArcLength)
            .count(options.count)
            .curve(options.curve));
    }

    public PointListResult simplify(List<Vector2> points, Double tolerance) throws ApiException {
        return simplify(SimplifyOptions.builder().points(points).tolerance(tolerance).build());
    }

    public PointListResult simplify(SimplifyOptions options) throws ApiException {
        return api.simplify(new SimplifyRequest()
            .points(options.points)
            .tolerance(options.tolerance));
    }

    public Vector2 tangent(CurveSpec curve, Double t) throws ApiException {
        return tangent(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    public Vector2 tangent(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return tangent(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    public Vector2 tangent(CurvePointOptions options) throws ApiException {
        return api.tangent(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }
}
