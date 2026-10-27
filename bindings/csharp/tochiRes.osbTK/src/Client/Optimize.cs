using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Optimize
{
    private readonly OptimizeAPI api;

    public Optimize(OptimizeAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Optimize;

    public OptimizeResult OptimizeOsb(OptimizeRequest body) => api.OptimizeOsb(body);
}
