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

    public EasingResult ApplyEasing(ApplyEasingRequest body) => api.ApplyEasing(body);

    public EasingResult EasingValue(EasingValueRequest body) => api.EasingValue(body);
}
