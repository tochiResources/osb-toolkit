#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

/// <summary>
/// The clean `Curve` client.
/// </summary>
public sealed class Curve
{
    private readonly CurveAPI api;

    public Curve(CurveAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Curve;

    /// <summary>
    /// A circular-arc sampling request.
    /// </summary>
    /// <param name="Center">The arc centre.</param>
    /// <param name="Radius">The radius.</param>
    /// <param name="StartAngle">The start angle in degrees.</param>
    /// <param name="SweepAngle">The sweep angle in degrees.</param>
    /// <returns>Points along a circular arc</returns>
    public PointListResult Arc(Vector2 Center, double Radius, double StartAngle, double SweepAngle) =>
        Arc(new ArcOptions { Center = Center, Radius = Radius, StartAngle = StartAngle, SweepAngle = SweepAngle });

    /// <summary>
    /// A circular-arc sampling request.
    /// </summary>
    /// <param name="Center">The arc centre.</param>
    /// <param name="Radius">The radius.</param>
    /// <param name="StartAngle">The start angle in degrees.</param>
    /// <param name="SweepAngle">The sweep angle in degrees.</param>
    /// <param name="Count">The number of points (default `64`).</param>
    /// <returns>Points along a circular arc</returns>
    public PointListResult Arc(Vector2 Center, double Radius, double StartAngle, double SweepAngle, int? Count) =>
        Arc(new ArcOptions { Center = Center, Radius = Radius, StartAngle = StartAngle, SweepAngle = SweepAngle, Count = Count });

    /// <summary>
    /// A circular-arc sampling request.
    /// </summary>
    /// <param name="options">A circular-arc sampling request.</param>
    /// <returns>Points along a circular arc</returns>
    public PointListResult Arc(ArcOptions options) => api.Arc(new ArcRequest(options.Center, options.Count, options.Radius, options.StartAngle, options.SweepAngle));

    /// <summary>
    /// A curve query.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <returns>The curve's axis-aligned bounding box</returns>
    public Box2 BoundingBox(CurveSpec Curve) =>
        BoundingBox(new CurveOptions { Curve = Curve });

    /// <summary>
    /// A curve query.
    /// </summary>
    /// <param name="options">A curve query.</param>
    /// <returns>The curve's axis-aligned bounding box</returns>
    public Box2 BoundingBox(CurveOptions options) => api.BoundingBox(new CurveRequest(options.Curve));

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <returns>The curvature at `t`</returns>
    public double Curvature(CurveSpec Curve, double T) =>
        Curvature(new CurvePointOptions { Curve = Curve, T = T });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <param name="ByArcLength">Sample by arc length instead of the raw parameter (default `false`).</param>
    /// <returns>The curvature at `t`</returns>
    public double Curvature(CurveSpec Curve, double T, bool? ByArcLength) =>
        Curvature(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="options">A curve query at a parametric or arc-length position.</param>
    /// <returns>The curvature at `t`</returns>
    public double Curvature(CurvePointOptions options) => api.Curvature(new CurvePointRequest(options.ByArcLength, options.Curve, options.T)).Value;

    /// <summary>
    /// A curve hit-test request.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="Point">The query point.</param>
    /// <returns>The distance from a point to the curve</returns>
    public double DistanceTo(CurveSpec Curve, Vector2 Point) =>
        DistanceTo(new CurveDistanceOptions { Curve = Curve, Point = Point });

    /// <summary>
    /// A curve hit-test request.
    /// </summary>
    /// <param name="options">A curve hit-test request.</param>
    /// <returns>The distance from a point to the curve</returns>
    public double DistanceTo(CurveDistanceOptions options) => api.DistanceTo(new CurveDistanceRequest(options.Curve, options.Point)).Value;

    /// <summary>
    /// An ellipse sampling request.
    /// </summary>
    /// <param name="Center">The ellipse centre.</param>
    /// <param name="RadiusX">The horizontal radius.</param>
    /// <param name="RadiusY">The vertical radius.</param>
    /// <returns>Points around an ellipse</returns>
    public PointListResult Ellipse(Vector2 Center, double RadiusX, double RadiusY) =>
        Ellipse(new EllipseOptions { Center = Center, RadiusX = RadiusX, RadiusY = RadiusY });

    /// <summary>
    /// An ellipse sampling request.
    /// </summary>
    /// <param name="Center">The ellipse centre.</param>
    /// <param name="RadiusX">The horizontal radius.</param>
    /// <param name="RadiusY">The vertical radius.</param>
    /// <param name="Count">The number of points (default `64`).</param>
    /// <returns>Points around an ellipse</returns>
    public PointListResult Ellipse(Vector2 Center, double RadiusX, double RadiusY, int? Count) =>
        Ellipse(new EllipseOptions { Center = Center, RadiusX = RadiusX, RadiusY = RadiusY, Count = Count });

    /// <summary>
    /// An ellipse sampling request.
    /// </summary>
    /// <param name="options">An ellipse sampling request.</param>
    /// <returns>Points around an ellipse</returns>
    public PointListResult Ellipse(EllipseOptions options) => api.Ellipse(new EllipseRequest(options.Center, options.Count, options.RadiusX, options.RadiusY));

    /// <summary>
    /// A curve query.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <returns>The curve's arc length</returns>
    public double Length(CurveSpec Curve) =>
        Length(new CurveOptions { Curve = Curve });

    /// <summary>
    /// A curve query.
    /// </summary>
    /// <param name="options">A curve query.</param>
    /// <returns>The curve's arc length</returns>
    public double Length(CurveOptions options) => api.Length(new CurveRequest(options.Curve)).Value;

    /// <summary>
    /// A line-segment intersection request.
    /// </summary>
    /// <param name="A1">The first segment's start.</param>
    /// <param name="A2">The first segment's end.</param>
    /// <param name="B1">The second segment's start.</param>
    /// <param name="B2">The second segment's end.</param>
    /// <returns>The intersection point, or null</returns>
    public Vector2 LineIntersect(Vector2 A1, Vector2 A2, Vector2 B1, Vector2 B2) =>
        LineIntersect(new LineIntersectOptions { A1 = A1, A2 = A2, B1 = B1, B2 = B2 });

    /// <summary>
    /// A line-segment intersection request.
    /// </summary>
    /// <param name="options">A line-segment intersection request.</param>
    /// <returns>The intersection point, or null</returns>
    public Vector2 LineIntersect(LineIntersectOptions options) => api.LineIntersect(new LineIntersectRequest(options.A1, options.A2, options.B1, options.B2)).Point;

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <returns>The unit normal at `t`</returns>
    public Vector2 Normal(CurveSpec Curve, double T) =>
        Normal(new CurvePointOptions { Curve = Curve, T = T });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <param name="ByArcLength">Sample by arc length instead of the raw parameter (default `false`).</param>
    /// <returns>The unit normal at `t`</returns>
    public Vector2 Normal(CurveSpec Curve, double T, bool? ByArcLength) =>
        Normal(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="options">A curve query at a parametric or arc-length position.</param>
    /// <returns>The unit normal at `t`</returns>
    public Vector2 Normal(CurvePointOptions options) => api.Normal(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    /// <summary>
    /// A curve offset request.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="Distance">The signed offset along the curve's local normal.</param>
    /// <returns>The polyline offset along the curve's normal</returns>
    public PointListResult Offset(CurveSpec Curve, double Distance) =>
        Offset(new CurveOffsetOptions { Curve = Curve, Distance = Distance });

    /// <summary>
    /// A curve offset request.
    /// </summary>
    /// <param name="options">A curve offset request.</param>
    /// <returns>The polyline offset along the curve's normal</returns>
    public PointListResult Offset(CurveOffsetOptions options) => api.Offset(new CurveOffsetRequest(options.Curve, options.Distance));

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <returns>The point at `t`, parametrically or by arc length</returns>
    public Vector2 Point(CurveSpec Curve, double T) =>
        Point(new CurvePointOptions { Curve = Curve, T = T });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <param name="ByArcLength">Sample by arc length instead of the raw parameter (default `false`).</param>
    /// <returns>The point at `t`, parametrically or by arc length</returns>
    public Vector2 Point(CurveSpec Curve, double T, bool? ByArcLength) =>
        Point(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="options">A curve query at a parametric or arc-length position.</param>
    /// <returns>The point at `t`, parametrically or by arc length</returns>
    public Vector2 Point(CurvePointOptions options) => api.Point(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    /// <summary>
    /// A curve sampling request.
    /// </summary>
    /// <param name="Count">The number of points to sample.</param>
    /// <param name="Curve">The curve.</param>
    /// <returns>`count` points along the curve</returns>
    public PointListResult Sample(int Count, CurveSpec Curve) =>
        Sample(new CurveSampleOptions { Count = Count, Curve = Curve });

    /// <summary>
    /// A curve sampling request.
    /// </summary>
    /// <param name="Count">The number of points to sample.</param>
    /// <param name="Curve">The curve.</param>
    /// <param name="ByArcLength">Sample by arc length instead of the raw parameter (default `false`).</param>
    /// <returns>`count` points along the curve</returns>
    public PointListResult Sample(int Count, CurveSpec Curve, bool? ByArcLength) =>
        Sample(new CurveSampleOptions { Count = Count, Curve = Curve, ByArcLength = ByArcLength });

    /// <summary>
    /// A curve sampling request.
    /// </summary>
    /// <param name="options">A curve sampling request.</param>
    /// <returns>`count` points along the curve</returns>
    public PointListResult Sample(CurveSampleOptions options) => api.Sample(new CurveSampleRequest(options.ByArcLength, options.Count, options.Curve));

    /// <summary>
    /// A Douglas-Peucker simplification request.
    /// </summary>
    /// <param name="Points">The polyline to simplify.</param>
    /// <param name="Tolerance">The maximum deviation to allow.</param>
    /// <returns>The simplified polyline</returns>
    public PointListResult Simplify(List<Vector2> Points, double Tolerance) =>
        Simplify(new SimplifyOptions { Points = Points, Tolerance = Tolerance });

    /// <summary>
    /// A Douglas-Peucker simplification request.
    /// </summary>
    /// <param name="options">A Douglas-Peucker simplification request.</param>
    /// <returns>The simplified polyline</returns>
    public PointListResult Simplify(SimplifyOptions options) => api.Simplify(new SimplifyRequest(options.Points, options.Tolerance));

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <returns>The unit tangent at `t`</returns>
    public Vector2 Tangent(CurveSpec Curve, double T) =>
        Tangent(new CurvePointOptions { Curve = Curve, T = T });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="Curve">The curve.</param>
    /// <param name="T">The position in `0.0..=1.0`.</param>
    /// <param name="ByArcLength">Sample by arc length instead of the raw parameter (default `false`).</param>
    /// <returns>The unit tangent at `t`</returns>
    public Vector2 Tangent(CurveSpec Curve, double T, bool? ByArcLength) =>
        Tangent(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    /// <summary>
    /// A curve query at a parametric or arc-length position.
    /// </summary>
    /// <param name="options">A curve query at a parametric or arc-length position.</param>
    /// <returns>The unit tangent at `t`</returns>
    public Vector2 Tangent(CurvePointOptions options) => api.Tangent(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));
}
