#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Curve
{
    private readonly CurveAPI api;

    public Curve(CurveAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Curve;

    public PointListResult Arc(ArcOptions options) => api.Arc(new ArcRequest(options.Center, options.Count, options.Radius, options.StartAngle, options.SweepAngle));

    public PointListResult Arc(Vector2 Center, double Radius, double StartAngle, double SweepAngle, int? Count = default) =>
        Arc(new ArcOptions { Center = Center, Radius = Radius, StartAngle = StartAngle, SweepAngle = SweepAngle, Count = Count });

    public Box2 BoundingBox(CurveOptions options) => api.BoundingBox(new CurveRequest(options.Curve));

    public Box2 BoundingBox(CurveSpec Curve) =>
        BoundingBox(new CurveOptions { Curve = Curve });

    public double Curvature(CurvePointOptions options) => api.Curvature(new CurvePointRequest(options.ByArcLength, options.Curve, options.T)).Value;

    public double Curvature(CurveSpec Curve, double T, bool? ByArcLength = default) =>
        Curvature(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    public double DistanceTo(CurveDistanceOptions options) => api.DistanceTo(new CurveDistanceRequest(options.Curve, options.Point)).Value;

    public double DistanceTo(CurveSpec Curve, Vector2 Point) =>
        DistanceTo(new CurveDistanceOptions { Curve = Curve, Point = Point });

    public PointListResult Ellipse(EllipseOptions options) => api.Ellipse(new EllipseRequest(options.Center, options.Count, options.RadiusX, options.RadiusY));

    public PointListResult Ellipse(Vector2 Center, double RadiusX, double RadiusY, int? Count = default) =>
        Ellipse(new EllipseOptions { Center = Center, RadiusX = RadiusX, RadiusY = RadiusY, Count = Count });

    public double Length(CurveOptions options) => api.Length(new CurveRequest(options.Curve)).Value;

    public double Length(CurveSpec Curve) =>
        Length(new CurveOptions { Curve = Curve });

    public Vector2 LineIntersect(LineIntersectOptions options) => api.LineIntersect(new LineIntersectRequest(options.A1, options.A2, options.B1, options.B2)).Point;

    public Vector2 LineIntersect(Vector2 A1, Vector2 A2, Vector2 B1, Vector2 B2) =>
        LineIntersect(new LineIntersectOptions { A1 = A1, A2 = A2, B1 = B1, B2 = B2 });

    public Vector2 Normal(CurvePointOptions options) => api.Normal(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public Vector2 Normal(CurveSpec Curve, double T, bool? ByArcLength = default) =>
        Normal(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    public PointListResult Offset(CurveOffsetOptions options) => api.Offset(new CurveOffsetRequest(options.Curve, options.Distance));

    public PointListResult Offset(CurveSpec Curve, double Distance) =>
        Offset(new CurveOffsetOptions { Curve = Curve, Distance = Distance });

    public Vector2 Point(CurvePointOptions options) => api.Point(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public Vector2 Point(CurveSpec Curve, double T, bool? ByArcLength = default) =>
        Point(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });

    public PointListResult Sample(CurveSampleOptions options) => api.Sample(new CurveSampleRequest(options.ByArcLength, options.Count, options.Curve));

    public PointListResult Sample(int Count, CurveSpec Curve, bool? ByArcLength = default) =>
        Sample(new CurveSampleOptions { Count = Count, Curve = Curve, ByArcLength = ByArcLength });

    public PointListResult Simplify(SimplifyOptions options) => api.Simplify(new SimplifyRequest(options.Points, options.Tolerance));

    public PointListResult Simplify(List<Vector2> Points, double Tolerance) =>
        Simplify(new SimplifyOptions { Points = Points, Tolerance = Tolerance });

    public Vector2 Tangent(CurvePointOptions options) => api.Tangent(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public Vector2 Tangent(CurveSpec Curve, double T, bool? ByArcLength = default) =>
        Tangent(new CurvePointOptions { Curve = Curve, T = T, ByArcLength = ByArcLength });
}
