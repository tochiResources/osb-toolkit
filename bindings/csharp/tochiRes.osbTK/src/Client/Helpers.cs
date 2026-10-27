using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Helpers
{
    private readonly HelpersAPI api;

    public Helpers(HelpersAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Helpers;

    public HelperResult Call(HelperRequest body) => api.Call(body);

    public HelperResult Commands(HelperRequest body) => api.Commands(body);

    public HelperResult Keyframes(HelperRequest body) => api.Keyframes(body);
}
