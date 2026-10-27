#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Animations
{
    private readonly AnimationsAPI api;

    public Animations(AnimationsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Animations;

    public double ApplyEasing(ApplyEasingOptions options) => api.ApplyEasing(new ApplyEasingRequest(options.Easing, options.T)).Value;

    public double ApplyEasing(Easing Easing, double T) =>
        ApplyEasing(new ApplyEasingOptions { Easing = Easing, T = T });

    public double EasingValue(EasingValueOptions options) => api.EasingValue(new EasingValueRequest(options.CubicBezier, options.Easing, options.T)).Value;

    public double EasingValue(double T, Easing? Easing = default, List<double> CubicBezier = default) =>
        EasingValue(new EasingValueOptions { T = T, Easing = Easing, CubicBezier = CubicBezier });
}
