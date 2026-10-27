#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

public sealed class Optimize
{
    private readonly OptimizeAPI api;

    public Optimize(OptimizeAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Optimize;

    public OptimizeResult OptimizeOsb(OptimizeOptionsInput Options, string Text) =>
        OptimizeOsb(new OptimizeOptions { Options = Options, Text = Text });

    public OptimizeResult OptimizeOsb(OptimizeOptions options) => api.OptimizeOsb(new OptimizeRequest(options.Options, options.Text));
}
