#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Storyboarding3d
{
    private readonly Storyboarding3dAPI api;

    public Storyboarding3d(Storyboarding3dAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Storyboarding3d;

    /// <summary>
    /// A `bake3d` request: a plain 3D scene.
    /// </summary>
    /// <param name="Input">The 3D scene.</param>
    /// <returns>Baked command spans</returns>
    public BakedStoryboard Bake3d(Scene3dInput Input) =>
        Bake3d(new Bake3dOptions { Input = Input });

    /// <summary>
    /// A `bake3d` request: a plain 3D scene.
    /// </summary>
    /// <param name="options">A `bake3d` request: a plain 3D scene.</param>
    /// <returns>Baked command spans</returns>
    public BakedStoryboard Bake3d(Bake3dOptions options) => api.Bake3d(new Bake3dRequest(options.Input));
}
