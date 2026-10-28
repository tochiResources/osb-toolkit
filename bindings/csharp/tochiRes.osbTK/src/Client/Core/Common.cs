#nullable disable
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

    /// <summary>
    /// Calls the Common.version bridge operation.
    /// </summary>
    /// <returns>The runtime version</returns>
    public string Version() => api.Version().VarVersion;
}
