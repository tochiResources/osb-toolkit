using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using AnimationsApi = tochiRes.osbTK.Api.AnimationsAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The animations library: easing evaluation.</summary>
    public static class Animations
    {
        private static readonly AnimationsApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Applies a named easing to a normalized time in `0..1`.</summary>
        public static double ApplyEasing(Easing easing, double t) =>
            Api.ApplyEasing(new ApplyEasingRequest(easing: easing, t: t)).Value;

        /// <summary>
        /// Evaluates a CSS-style cubic-bezier easing.
        /// with control points <c>(x1, y1)</c>, <c>(x2, y2)</c> at a normalized time.
        /// </summary>
        public static double ApplyCubicBezier(double x1, double y1, double x2, double y2, double t) =>
            Api.EasingValue(new EasingValueRequest(cubicBezier: [x1, y1, x2, y2], t: t)).Value;
    }
}
