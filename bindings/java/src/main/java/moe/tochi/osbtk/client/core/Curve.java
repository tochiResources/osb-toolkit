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

    /**
     * A circular-arc sampling request.
     *
     * @param center The arc centre.
     * @param radius The radius.
     * @param startAngle The start angle in degrees.
     * @param sweepAngle The sweep angle in degrees.
     *
     * @return Points along a circular arc
     */
    public PointListResult arc(Vector2 center, Double radius, Double startAngle, Double sweepAngle) throws ApiException {
        return arc(ArcOptions.builder().center(center).radius(radius).startAngle(startAngle).sweepAngle(sweepAngle).build());
    }

    /**
     * A circular-arc sampling request.
     *
     * @param center The arc centre.
     * @param radius The radius.
     * @param startAngle The start angle in degrees.
     * @param sweepAngle The sweep angle in degrees.
     * @param count The number of points (default `64`).
     *
     * @return Points along a circular arc
     */
    public PointListResult arc(Vector2 center, Double radius, Double startAngle, Double sweepAngle, Integer count) throws ApiException {
        return arc(ArcOptions.builder().center(center).radius(radius).startAngle(startAngle).sweepAngle(sweepAngle).count(count).build());
    }

    /**
     * A circular-arc sampling request.
     *
     * @param options A circular-arc sampling request.
     *
     * @return Points along a circular arc
     */
    public PointListResult arc(ArcOptions options) throws ApiException {
        return api.arc(new ArcRequest()
            .center(options.center)
            .count(options.count)
            .radius(options.radius)
            .startAngle(options.startAngle)
            .sweepAngle(options.sweepAngle));
    }

    /**
     * A curve query.
     *
     * @param curve The curve.
     *
     * @return The curve's axis-aligned bounding box
     */
    public Box2 boundingBox(CurveSpec curve) throws ApiException {
        return boundingBox(CurveOptions.builder().curve(curve).build());
    }

    /**
     * A curve query.
     *
     * @param options A curve query.
     *
     * @return The curve's axis-aligned bounding box
     */
    public Box2 boundingBox(CurveOptions options) throws ApiException {
        return api.boundingBox(new CurveRequest()
            .curve(options.curve));
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @return The curvature at `t`
     */
    public double curvature(CurveSpec curve, Double t) throws ApiException {
        return curvature(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @return The curvature at `t`
     */
    public double curvature(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return curvature(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @return The curvature at `t`
     */
    public double curvature(CurvePointOptions options) throws ApiException {
        return api.curvature(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t)).getValue();
    }

    /**
     * A curve hit-test request.
     *
     * @param curve The curve.
     * @param point The query point.
     *
     * @return The distance from a point to the curve
     */
    public double distanceTo(CurveSpec curve, Vector2 point) throws ApiException {
        return distanceTo(CurveDistanceOptions.builder().curve(curve).point(point).build());
    }

    /**
     * A curve hit-test request.
     *
     * @param options A curve hit-test request.
     *
     * @return The distance from a point to the curve
     */
    public double distanceTo(CurveDistanceOptions options) throws ApiException {
        return api.distanceTo(new CurveDistanceRequest()
            .curve(options.curve)
            .point(options.point)).getValue();
    }

    /**
     * An ellipse sampling request.
     *
     * @param center The ellipse centre.
     * @param radiusX The horizontal radius.
     * @param radiusY The vertical radius.
     *
     * @return Points around an ellipse
     */
    public PointListResult ellipse(Vector2 center, Double radiusX, Double radiusY) throws ApiException {
        return ellipse(EllipseOptions.builder().center(center).radiusX(radiusX).radiusY(radiusY).build());
    }

    /**
     * An ellipse sampling request.
     *
     * @param center The ellipse centre.
     * @param radiusX The horizontal radius.
     * @param radiusY The vertical radius.
     * @param count The number of points (default `64`).
     *
     * @return Points around an ellipse
     */
    public PointListResult ellipse(Vector2 center, Double radiusX, Double radiusY, Integer count) throws ApiException {
        return ellipse(EllipseOptions.builder().center(center).radiusX(radiusX).radiusY(radiusY).count(count).build());
    }

    /**
     * An ellipse sampling request.
     *
     * @param options An ellipse sampling request.
     *
     * @return Points around an ellipse
     */
    public PointListResult ellipse(EllipseOptions options) throws ApiException {
        return api.ellipse(new EllipseRequest()
            .center(options.center)
            .count(options.count)
            .radiusX(options.radiusX)
            .radiusY(options.radiusY));
    }

    /**
     * A curve query.
     *
     * @param curve The curve.
     *
     * @return The curve's arc length
     */
    public double length(CurveSpec curve) throws ApiException {
        return length(CurveOptions.builder().curve(curve).build());
    }

    /**
     * A curve query.
     *
     * @param options A curve query.
     *
     * @return The curve's arc length
     */
    public double length(CurveOptions options) throws ApiException {
        return api.length(new CurveRequest()
            .curve(options.curve)).getValue();
    }

    /**
     * A line-segment intersection request.
     *
     * @param a1 The first segment's start.
     * @param a2 The first segment's end.
     * @param b1 The second segment's start.
     * @param b2 The second segment's end.
     *
     * @return The intersection point, or null
     */
    public Vector2 lineIntersect(Vector2 a1, Vector2 a2, Vector2 b1, Vector2 b2) throws ApiException {
        return lineIntersect(LineIntersectOptions.builder().a1(a1).a2(a2).b1(b1).b2(b2).build());
    }

    /**
     * A line-segment intersection request.
     *
     * @param options A line-segment intersection request.
     *
     * @return The intersection point, or null
     */
    public Vector2 lineIntersect(LineIntersectOptions options) throws ApiException {
        return api.lineIntersect(new LineIntersectRequest()
            .a1(options.a1)
            .a2(options.a2)
            .b1(options.b1)
            .b2(options.b2)).getPoint();
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @return The unit normal at `t`
     */
    public Vector2 normal(CurveSpec curve, Double t) throws ApiException {
        return normal(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @return The unit normal at `t`
     */
    public Vector2 normal(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return normal(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @return The unit normal at `t`
     */
    public Vector2 normal(CurvePointOptions options) throws ApiException {
        return api.normal(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    /**
     * A curve offset request.
     *
     * @param curve The curve.
     * @param distance The signed offset along the curve's local normal.
     *
     * @return The polyline offset along the curve's normal
     */
    public PointListResult offset(CurveSpec curve, Double distance) throws ApiException {
        return offset(CurveOffsetOptions.builder().curve(curve).distance(distance).build());
    }

    /**
     * A curve offset request.
     *
     * @param options A curve offset request.
     *
     * @return The polyline offset along the curve's normal
     */
    public PointListResult offset(CurveOffsetOptions options) throws ApiException {
        return api.offset(new CurveOffsetRequest()
            .curve(options.curve)
            .distance(options.distance));
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @return The point at `t`, parametrically or by arc length
     */
    public Vector2 point(CurveSpec curve, Double t) throws ApiException {
        return point(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @return The point at `t`, parametrically or by arc length
     */
    public Vector2 point(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return point(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @return The point at `t`, parametrically or by arc length
     */
    public Vector2 point(CurvePointOptions options) throws ApiException {
        return api.point(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }

    /**
     * A curve sampling request.
     *
     * @param count The number of points to sample.
     * @param curve The curve.
     *
     * @return `count` points along the curve
     */
    public PointListResult sample(Integer count, CurveSpec curve) throws ApiException {
        return sample(CurveSampleOptions.builder().count(count).curve(curve).build());
    }

    /**
     * A curve sampling request.
     *
     * @param count The number of points to sample.
     * @param curve The curve.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @return `count` points along the curve
     */
    public PointListResult sample(Integer count, CurveSpec curve, Boolean byArcLength) throws ApiException {
        return sample(CurveSampleOptions.builder().count(count).curve(curve).byArcLength(byArcLength).build());
    }

    /**
     * A curve sampling request.
     *
     * @param options A curve sampling request.
     *
     * @return `count` points along the curve
     */
    public PointListResult sample(CurveSampleOptions options) throws ApiException {
        return api.sample(new CurveSampleRequest()
            .byArcLength(options.byArcLength)
            .count(options.count)
            .curve(options.curve));
    }

    /**
     * A Douglas-Peucker simplification request.
     *
     * @param points The polyline to simplify.
     * @param tolerance The maximum deviation to allow.
     *
     * @return The simplified polyline
     */
    public PointListResult simplify(List<Vector2> points, Double tolerance) throws ApiException {
        return simplify(SimplifyOptions.builder().points(points).tolerance(tolerance).build());
    }

    /**
     * A Douglas-Peucker simplification request.
     *
     * @param options A Douglas-Peucker simplification request.
     *
     * @return The simplified polyline
     */
    public PointListResult simplify(SimplifyOptions options) throws ApiException {
        return api.simplify(new SimplifyRequest()
            .points(options.points)
            .tolerance(options.tolerance));
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     *
     * @return The unit tangent at `t`
     */
    public Vector2 tangent(CurveSpec curve, Double t) throws ApiException {
        return tangent(CurvePointOptions.builder().curve(curve).t(t).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param curve The curve.
     * @param t The position in `0.0..=1.0`.
     * @param byArcLength Sample by arc length instead of the raw parameter (default `false`).
     *
     * @return The unit tangent at `t`
     */
    public Vector2 tangent(CurveSpec curve, Double t, Boolean byArcLength) throws ApiException {
        return tangent(CurvePointOptions.builder().curve(curve).t(t).byArcLength(byArcLength).build());
    }

    /**
     * A curve query at a parametric or arc-length position.
     *
     * @param options A curve query at a parametric or arc-length position.
     *
     * @return The unit tangent at `t`
     */
    public Vector2 tangent(CurvePointOptions options) throws ApiException {
        return api.tangent(new CurvePointRequest()
            .byArcLength(options.byArcLength)
            .curve(options.curve)
            .t(options.t));
    }
}
