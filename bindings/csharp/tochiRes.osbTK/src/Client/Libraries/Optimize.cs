#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

/// <summary>
/// The clean `Optimize` client.
/// </summary>
public sealed class Optimize
{
    private readonly OptimizeAPI api;

    public Optimize(OptimizeAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Optimize;

    /// <summary>
    /// An `.osb` optimization request.
    /// </summary>
    /// <param name="Options">The optimization options.</param>
    /// <param name="Text">The `.osb` storyboard text.</param>
    /// <returns>The optimized `.osb` text and stats</returns>
    public OptimizeResult OptimizeOsb(OptimizeOptionsInput Options, string Text) =>
        OptimizeOsb(new OptimizeOptions { Options = Options, Text = Text });

    /// <summary>
    /// An `.osb` optimization request.
    /// </summary>
    /// <param name="options">An `.osb` optimization request.</param>
    /// <returns>The optimized `.osb` text and stats</returns>
    public OptimizeResult OptimizeOsb(OptimizeOptions options) => api.OptimizeOsb(new OptimizeRequest(options.Options, options.Text));
}
