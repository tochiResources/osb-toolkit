using tochiRes.osbTK.Client;

using NativeFacade = global::tochiRes.osbTK.Native.osb;

namespace tochiRes.osbTK;

/// <summary>
/// The osb SDK entry point, organized into libraries: <see cref="Storyboarding"/>,
/// <see cref="Storyboarding3d"/>, <see cref="Animations"/>, <see cref="Audio"/>,
/// <see cref="Curve"/>, <see cref="Diagnostics"/>, <see cref="Optimize"/>,
/// <see cref="Image"/>, <see cref="ImageGen"/> and <see cref="Media"/>. Each
/// library lives in its own file under <c>Libraries/</c>.
/// </summary>
public static partial class osb
{
    internal static readonly OsbtkTransport StatelessTransport = new();
    internal static readonly Configuration DefaultConfiguration = new();

    /// <summary>The native runtime version string.</summary>
    public static string Version => NativeFacade.Version();
}
