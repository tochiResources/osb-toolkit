using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Diagnostics
{
    private readonly DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Diagnostics;

    public DiagnosticReport DiagnoseOsb(TextRequest body) => api.DiagnoseOsb(body);

    public DiagnosticReport DiagnoseOsu(TextRequest body) => api.DiagnoseOsu(body);
}
