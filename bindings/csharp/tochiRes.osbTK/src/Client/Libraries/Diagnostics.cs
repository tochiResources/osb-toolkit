#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

public sealed class Diagnostics
{
    private readonly DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Diagnostics;

    public DiagnosticReport DiagnoseOsb(TextOptions options) => api.DiagnoseOsb(new TextRequest(options.Text));

    public DiagnosticReport DiagnoseOsb(string Text) =>
        DiagnoseOsb(new TextOptions { Text = Text });

    public DiagnosticReport DiagnoseOsu(TextOptions options) => api.DiagnoseOsu(new TextRequest(options.Text));

    public DiagnosticReport DiagnoseOsu(string Text) =>
        DiagnoseOsu(new TextOptions { Text = Text });
}
