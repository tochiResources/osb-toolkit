#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

public sealed class Image
{
    private readonly ImageAPI api;

    public Image(ImageAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Image;

    /// <summary>
    /// A request carrying a single file path.
    /// </summary>
    /// <param name="Path">The input file path.</param>
    /// <returns>The image's metadata</returns>
    public ImageInfo Info(string Path) =>
        Info(new PathOptions { Path = Path });

    /// <summary>
    /// A request carrying a single file path.
    /// </summary>
    /// <param name="options">A request carrying a single file path.</param>
    /// <returns>The image's metadata</returns>
    public ImageInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));

    /// <summary>
    /// An image processing request: read `path`, apply `options`, write `output`.
    /// </summary>
    /// <param name="Options">The processing options.</param>
    /// <param name="Output">The output file path (its extension selects the format).</param>
    /// <param name="Path">The input file path.</param>
    /// <returns>The processed image's metadata</returns>
    public ImageInfo Process(ImageOptions Options, string Output, string Path) =>
        Process(new ImageProcessOptions { Options = Options, Output = Output, Path = Path });

    /// <summary>
    /// An image processing request: read `path`, apply `options`, write `output`.
    /// </summary>
    /// <param name="options">An image processing request: read `path`, apply `options`, write `output`.</param>
    /// <returns>The processed image's metadata</returns>
    public ImageInfo Process(ImageProcessOptions options) => api.Process(new ImageProcessRequest(options.Options, options.Output, options.Path));
}
