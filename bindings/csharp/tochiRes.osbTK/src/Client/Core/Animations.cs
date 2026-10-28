#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

/// <summary>
/// The clean `Animations` client.
/// </summary>
public sealed class Animations
{
    private readonly AnimationsAPI api;

    public Animations(AnimationsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Animations;

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
}
