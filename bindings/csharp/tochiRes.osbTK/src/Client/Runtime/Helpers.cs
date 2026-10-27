#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Helpers
{
    private readonly HelpersAPI api;

    public Helpers(HelpersAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Helpers;

    public object Call(HelperOptions options) => api.Call(new HelperRequest(options.Input, options.Name)).Value;

    public object Call(string Name, string Input = default) =>
        Call(new HelperOptions { Name = Name, Input = Input });

    public object Commands(HelperOptions options) => api.Commands(new HelperRequest(options.Input, options.Name)).Value;

    public object Commands(string Name, string Input = default) =>
        Commands(new HelperOptions { Name = Name, Input = Input });

    public object Keyframes(HelperOptions options) => api.Keyframes(new HelperRequest(options.Input, options.Name)).Value;

    public object Keyframes(string Name, string Input = default) =>
        Keyframes(new HelperOptions { Name = Name, Input = Input });
}
