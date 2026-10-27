namespace tochiRes.osbTK.Native;

/// <summary>
/// Thin C# facade over the osb native engine (). One
/// implementation, shared with the Rust plug-ins, the TypeScript packages and the
/// other language bindings via the golden corpus.
/// </summary>
public static class osb
{
    /// <summary>The native runtime version string.</summary>
    public static string Version() => NativeMethods.Version();

    /// <summary>
    /// Bakes a plain 3D input (the shared <c>Bake3dInput</c> JSON shape) into
    /// `.osb`-compatible command spans, as JSON.
    /// </summary>
    public static string Bake3d(string inputJson) => NativeMethods.Bake3d(inputJson);

    /// <summary>
    /// Runs a JSON request (<c>{"op": "..."}</c>) against the whole runtime -
    /// `parseOsu`, `parseOsb`, `fft`, `bake3d`, `version` - and returns JSON.
    /// </summary>
    public static string Exec(string requestJson) => NativeMethods.Exec(requestJson);
}
