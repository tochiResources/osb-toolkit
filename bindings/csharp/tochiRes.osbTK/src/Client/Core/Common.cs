using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Common
{
    private readonly CommonAPI api;

    public Common(CommonAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Common;

    public VersionResult Version() => api.Version();
}
