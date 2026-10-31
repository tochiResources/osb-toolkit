using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;

using NativeFacade = global::tochiRes.osbTK.Native.osb;

namespace tochiRes.osbTK;

/// <summary>
/// The osb SDK entry point. The clean generated clients are the primary surface;
/// the hand-written pieces are only the domain conveniences the contract cannot
/// express: <see cref="Storyboarding"/> (the effect <see cref="Storyboarding.Runtime"/>,
/// its parsing helpers and the 2D command helpers), <see cref="Storyboarding3d"/>
/// (the 3D bake plus its keyframe helpers) and <see cref="Helpers"/> (the shared
/// motion-graphics helpers).
/// </summary>
public static partial class osb
{
    internal static readonly OsbtkTransport StatelessTransport = new();
    internal static readonly Configuration DefaultConfiguration = new();

    /// <summary>The native runtime version string.</summary>
    public static string Version => NativeFacade.Version();

    /// <summary>
    /// The non-runtime `Core` surface: the native version, the stateless
    /// parsers, easing and the curve/origin math.
    /// </summary>
    public static Client.Core.Core Core { get; } =
        new(new CoreAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The audio library: FFT.</summary>
    public static Client.Runtime.Audio Audio { get; } =
        new(new AudioAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The clean `Helpers` client; the <see cref="Helpers"/> facade stays available.</summary>
    public static Client.Runtime.Helpers HelpersClient { get; } =
        new(new HelpersAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The diagnostics library: storyboard and mapset analysis.</summary>
    public static Client.Libraries.Diagnostics Diagnostics { get; } =
        new(new DiagnosticsAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The optimization library: storyboard optimization and `.osb` round-trip.</summary>
    public static Client.Libraries.Optimize Optimize { get; } =
        new(new OptimizeAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The image library: decode/encode and processing.</summary>
    public static Client.Libraries.Image Image { get; } =
        new(new ImageAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The image-generation library: draw primitive shapes.</summary>
    public static Client.Libraries.Imagegen ImageGen { get; } =
        new(new ImagegenAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));

    /// <summary>The media library: probing, frame extraction/encoding and audio.</summary>
    public static Client.Libraries.Media Media { get; } =
        new(new MediaAPI(StatelessTransport, StatelessTransport, DefaultConfiguration));
}
