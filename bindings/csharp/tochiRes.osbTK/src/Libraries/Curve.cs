using System.Collections.Generic;

using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using CurveApi = tochiRes.osbTK.Api.CurveAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The curve library: Bezier/Catmull-Rom curves, sampling, arc length,
    /// tangents, bounding boxes and Douglas-Peucker simplification.
    /// </summary>
    public static class Curve
    {
        private static readonly CurveApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>The arc length of a curve.</summary>
        public static double Length(CurveSpec curve)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Length(new CurveRequest(curve: curve)).Value;
        }

        /// <summary>The axis-aligned bounding box of a curve.</summary>
        public static Box2 BoundingBox(CurveSpec curve)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.BoundingBox(new CurveRequest(curve: curve));
        }

        /// <summary>The point at <paramref name="t"/>, parametrically or by arc length.</summary>
        public static Vector2 Point(CurveSpec curve, double t, bool byArcLength = false)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Point(new CurvePointRequest(byArcLength: byArcLength, curve: curve, t: t));
        }

        /// <summary>The unit tangent at <paramref name="t"/>.</summary>
        public static Vector2 Tangent(CurveSpec curve, double t)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Tangent(new CurvePointRequest(curve: curve, t: t));
        }

        /// <summary><paramref name="count"/> points along the curve, parametrically or by arc length.</summary>
        public static PointListResult Sample(CurveSpec curve, int count, bool byArcLength = false)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Sample(
                new CurveSampleRequest(byArcLength: byArcLength, count: count, curve: curve)
            );
        }

        /// <summary>Simplifies a polyline with the Douglas-Peucker algorithm.</summary>
        public static PointListResult Simplify(List<Vector2> points, double tolerance)
        {
            ArgumentNullException.ThrowIfNull(points);
            return Api.Simplify(new SimplifyRequest(points: points, tolerance: tolerance));
        }

        /// <summary>The unit normal at <paramref name="t"/>.</summary>
        public static Vector2 Normal(CurveSpec curve, double t)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Normal(new CurvePointRequest(curve: curve, t: t));
        }

        /// <summary>The curvature at <paramref name="t"/>.</summary>
        public static double Curvature(CurveSpec curve, double t)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Curvature(new CurvePointRequest(curve: curve, t: t)).Value;
        }

        /// <summary>The distance from <paramref name="point"/> to the curve.</summary>
        public static double DistanceTo(CurveSpec curve, Vector2 point)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.DistanceTo(new CurveDistanceRequest(curve: curve, point: point)).Value;
        }

        /// <summary>The polyline offset along the curve's local normal.</summary>
        public static PointListResult Offset(CurveSpec curve, double distance)
        {
            ArgumentNullException.ThrowIfNull(curve);
            return Api.Offset(new CurveOffsetRequest(curve: curve, distance: distance));
        }

        /// <summary>Points along a circular arc.</summary>
        public static PointListResult Arc(
            Vector2 center,
            double radius,
            double startAngle,
            double sweepAngle,
            int count
        ) =>
            Api.Arc(
                new ArcRequest(
                    center: center,
                    radius: radius,
                    startAngle: startAngle,
                    sweepAngle: sweepAngle,
                    count: count
                )
            );

        /// <summary>Points around an ellipse.</summary>
        public static PointListResult Ellipse(
            Vector2 center,
            double radiusX,
            double radiusY,
            int count
        ) =>
            Api.Ellipse(
                new EllipseRequest(
                    center: center,
                    radiusX: radiusX,
                    radiusY: radiusY,
                    count: count
                )
            );

        /// <summary>The intersection of two line segments, if any.</summary>
        public static OptionalPointResult LineIntersect(
            Vector2 a1,
            Vector2 a2,
            Vector2 b1,
            Vector2 b2
        ) => Api.LineIntersect(new LineIntersectRequest(a1: a1, a2: a2, b1: b1, b2: b2));
    }
}
