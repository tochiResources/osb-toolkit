using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Curve
{
    private readonly CurveAPI api;

    public Curve(CurveAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Curve;

    public PointListResult Arc(ArcRequest body) => api.Arc(body);

    public Box2 BoundingBox(CurveRequest body) => api.BoundingBox(body);

    public ScalarResult Curvature(CurvePointRequest body) => api.Curvature(body);

    public ScalarResult DistanceTo(CurveDistanceRequest body) => api.DistanceTo(body);

    public PointListResult Ellipse(EllipseRequest body) => api.Ellipse(body);

    public ScalarResult Length(CurveRequest body) => api.Length(body);

    public OptionalPointResult LineIntersect(LineIntersectRequest body) => api.LineIntersect(body);

    public Vector2 Normal(CurvePointRequest body) => api.Normal(body);

    public PointListResult Offset(CurveOffsetRequest body) => api.Offset(body);

    public Vector2 Point(CurvePointRequest body) => api.Point(body);

    public PointListResult Sample(CurveSampleRequest body) => api.Sample(body);

    public PointListResult Simplify(SimplifyRequest body) => api.Simplify(body);

    public Vector2 Tangent(CurvePointRequest body) => api.Tangent(body);
}
