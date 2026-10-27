using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Animations
{
    private readonly AnimationsAPI api;

    public Animations(AnimationsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Animations;

    public EasingResult ApplyEasing(ApplyEasingOptions options) => api.ApplyEasing(new ApplyEasingRequest(options.Easing, options.T));

    public EasingResult EasingValue(EasingValueOptions options) => api.EasingValue(new EasingValueRequest(options.CubicBezier, options.Easing, options.T));
}
