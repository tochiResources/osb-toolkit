#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

/// <summary>
/// The clean `Core` client.
/// </summary>
public sealed class Core
{
    private readonly CoreAPI api;

    public Core(CoreAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Core;

    /// <summary>
    /// An easing evaluation request.
    /// </summary>
    /// <param name="Easing">The easing function.</param>
    /// <param name="T">The normalized time in `0.0..=1.0`.</param>
    /// <returns>The eased value</returns>
    public double ApplyEasing(Easing Easing, double T) =>
        ApplyEasing(new ApplyEasingOptions { Easing = Easing, T = T });

    /// <summary>
    /// An easing evaluation request.
    /// </summary>
    /// <param name="options">An easing evaluation request.</param>
    /// <returns>The eased value</returns>
    public double ApplyEasing(ApplyEasingOptions options) => api.ApplyEasing(new ApplyEasingRequest(options.Easing, options.T)).Value;

    /// <summary>
    /// A named-or-custom easing evaluation request.
    /// </summary>
    /// <param name="T">The normalized time in `0.0..=1.0`.</param>
    /// <returns>The eased value (named easing or cubic-bezier)</returns>
    public double EasingValue(double T) =>
        EasingValue(new EasingValueOptions { T = T });

    /// <summary>
    /// A named-or-custom easing evaluation request.
    /// </summary>
    /// <param name="T">The normalized time in `0.0..=1.0`.</param>
    /// <param name="Easing">A named easing (used when `cubicBezier` is absent).</param>
    /// <returns>The eased value (named easing or cubic-bezier)</returns>
    public double EasingValue(double T, Easing? Easing) =>
        EasingValue(new EasingValueOptions { T = T, Easing = Easing });

    /// <summary>
    /// A named-or-custom easing evaluation request.
    /// </summary>
    /// <param name="T">The normalized time in `0.0..=1.0`.</param>
    /// <param name="Easing">A named easing (used when `cubicBezier` is absent).</param>
    /// <param name="CubicBezier">A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.</param>
    /// <returns>The eased value (named easing or cubic-bezier)</returns>
    public double EasingValue(double T, Easing? Easing, List<double> CubicBezier) =>
        EasingValue(new EasingValueOptions { T = T, Easing = Easing, CubicBezier = CubicBezier });

    /// <summary>
    /// A named-or-custom easing evaluation request.
    /// </summary>
    /// <param name="options">A named-or-custom easing evaluation request.</param>
    /// <returns>The eased value (named easing or cubic-bezier)</returns>
    public double EasingValue(EasingValueOptions options) => api.EasingValue(new EasingValueRequest(options.CubicBezier, options.Easing, options.T)).Value;

    /// <summary>
    /// Calls the Core.version bridge operation.
    /// </summary>
    /// <returns>The runtime version</returns>
    public string Version() => api.Version().VarVersion;

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

    /// <summary>
    /// The anchor offset for an origin.
    /// </summary>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Width">The sprite width.</param>
    /// <param name="Height">The sprite height.</param>
    /// <returns>The anchor offset for the origin</returns>
    public Vector2 GetOriginVector(OsbOrigin Origin, double Width, double Height) =>
        GetOriginVector(new GetOriginVectorOptions { Origin = Origin, Width = Width, Height = Height });

    /// <summary>
    /// The anchor offset for an origin.
    /// </summary>
    /// <param name="options">The anchor offset for an origin.</param>
    /// <returns>The anchor offset for the origin</returns>
    public Vector2 GetOriginVector(GetOriginVectorOptions options) => api.GetOriginVector(new GetOriginVectorRequest(options.Height, options.Origin, options.Width));

    /// <summary>
    /// Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
    /// </summary>
    /// <param name="Position">The sprite centre position.</param>
    /// <param name="Size">The sprite size.</param>
    /// <param name="Rotation">The rotation in radians.</param>
    /// <param name="Origin">The anchor offset (from `GetOriginVector`).</param>
    /// <returns>Whether the sprite's bounds intersect the screen</returns>
    public bool InScreenBounds(Vector2 Position, Vector2 Size, double Rotation, Vector2 Origin) =>
        InScreenBounds(new InScreenBoundsOptions { Position = Position, Size = Size, Rotation = Rotation, Origin = Origin });

    /// <summary>
    /// Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
    /// </summary>
    /// <param name="options">Whether a rotated sprite's bounding box intersects the widescreen storyboard area.</param>
    /// <returns>Whether the sprite's bounds intersect the screen</returns>
    public bool InScreenBounds(InScreenBoundsOptions options) => api.InScreenBounds(new InScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size)).Value;

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>The storyboard summary</returns>
    public ParseOsbResult ParseOsb(string Text) =>
        ParseOsb(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>The storyboard summary</returns>
    public ParseOsbResult ParseOsb(TextOptions options) => api.ParseOsb(new TextRequest(options.Text));

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>The parsed beatmap</returns>
    public Beatmap ParseOsu(string Text) =>
        ParseOsu(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>The parsed beatmap</returns>
    public Beatmap ParseOsu(TextOptions options) => api.ParseOsu(new TextRequest(options.Text));
}
