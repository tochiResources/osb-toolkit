#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

/// <summary>
/// The clean `Diagnostics` client.
/// </summary>
public sealed class Diagnostics
{
    private readonly DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Diagnostics;

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>Diagnostics for an `.osb` storyboard</returns>
    public DiagnosticReport DiagnoseOsb(string Text) =>
        DiagnoseOsb(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>Diagnostics for an `.osb` storyboard</returns>
    public DiagnosticReport DiagnoseOsb(TextOptions options) => api.DiagnoseOsb(new TextRequest(options.Text));

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>Diagnostics for an `.osu` beatmap</returns>
    public DiagnosticReport DiagnoseOsu(string Text) =>
        DiagnoseOsu(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>Diagnostics for an `.osu` beatmap</returns>
    public DiagnosticReport DiagnoseOsu(TextOptions options) => api.DiagnoseOsu(new TextRequest(options.Text));
}
