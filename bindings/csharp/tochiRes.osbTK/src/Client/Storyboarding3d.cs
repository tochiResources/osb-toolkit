using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Storyboarding3d
{
    private readonly Storyboarding3dAPI api;

    public Storyboarding3d(Storyboarding3dAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Storyboarding3d;

    public BakedStoryboard Bake3d(Bake3dOptions options) => api.Bake3d(new Bake3dRequest(options.Input));
}
