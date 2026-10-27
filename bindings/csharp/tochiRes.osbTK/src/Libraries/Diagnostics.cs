using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using DiagnosticsApi = tochiRes.osbTK.Api.DiagnosticsAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The diagnostics library: storyboard and mapset analysis (M29).</summary>
    public static class Diagnostics
    {
        private static readonly DiagnosticsApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Diagnoses an `.osb` storyboard.</summary>
        public static DiagnosticReport DiagnoseOsb(string text) =>
            Api.DiagnoseOsb(new TextRequest(text: text));

        /// <summary>Diagnoses an `.osu` beatmap.</summary>
        public static DiagnosticReport DiagnoseOsu(string text) =>
            Api.DiagnoseOsu(new TextRequest(text: text));
    }
}
