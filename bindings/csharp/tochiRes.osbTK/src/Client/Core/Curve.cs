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

    public Box2 BoundingBox(CurveOptions options) => api.BoundingBox(new CurveRequest(options.Curve));

    public ScalarResult Curvature(CurvePointOptions options) => api.Curvature(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public ScalarResult DistanceTo(CurveDistanceOptions options) => api.DistanceTo(new CurveDistanceRequest(options.Curve, options.Point));

    public PointListResult Ellipse(EllipseOptions options) => api.Ellipse(new EllipseRequest(options.Center, options.Count, options.RadiusX, options.RadiusY));

    public ScalarResult Length(CurveOptions options) => api.Length(new CurveRequest(options.Curve));

    public OptionalPointResult LineIntersect(LineIntersectOptions options) => api.LineIntersect(new LineIntersectRequest(options.A1, options.A2, options.B1, options.B2));

    public Vector2 Normal(CurvePointOptions options) => api.Normal(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public PointListResult Offset(CurveOffsetOptions options) => api.Offset(new CurveOffsetRequest(options.Curve, options.Distance));

    public Vector2 Point(CurvePointOptions options) => api.Point(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));

    public PointListResult Sample(CurveSampleOptions options) => api.Sample(new CurveSampleRequest(options.ByArcLength, options.Count, options.Curve));

    public PointListResult Simplify(SimplifyOptions options) => api.Simplify(new SimplifyRequest(options.Points, options.Tolerance));

    public Vector2 Tangent(CurvePointOptions options) => api.Tangent(new CurvePointRequest(options.ByArcLength, options.Curve, options.T));
}
