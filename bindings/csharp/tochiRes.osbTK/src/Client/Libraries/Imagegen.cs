#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

public sealed class Imagegen
{
    private readonly ImagegenAPI api;

    public Imagegen(ImagegenAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Imagegen;

    /// <summary>
    /// A shape-rendering request: a canvas, a background and a shape list.
    /// </summary>
    /// <param name="Width">The canvas width in pixels.</param>
    /// <param name="Height">The canvas height in pixels.</param>
    /// <param name="Shapes">The shapes to draw, in order.</param>
    /// <param name="Output">The output file path (its extension selects the format).</param>
    /// <returns>The rendered image's metadata</returns>
    public ImageInfo RenderShapes(int Width, int Height, List<Shape> Shapes, string Output) =>
        RenderShapes(new RenderShapesOptions { Width = Width, Height = Height, Shapes = Shapes, Output = Output });

    /// <summary>
    /// A shape-rendering request: a canvas, a background and a shape list.
    /// </summary>
    /// <param name="Width">The canvas width in pixels.</param>
    /// <param name="Height">The canvas height in pixels.</param>
    /// <param name="Shapes">The shapes to draw, in order.</param>
    /// <param name="Output">The output file path (its extension selects the format).</param>
    /// <param name="Background">The background colour (default transparent).</param>
    /// <returns>The rendered image's metadata</returns>
    public ImageInfo RenderShapes(int Width, int Height, List<Shape> Shapes, string Output, RgbaColor Background) =>
        RenderShapes(new RenderShapesOptions { Width = Width, Height = Height, Shapes = Shapes, Output = Output, Background = Background });

    /// <summary>
    /// A shape-rendering request: a canvas, a background and a shape list.
    /// </summary>
    /// <param name="options">A shape-rendering request: a canvas, a background and a shape list.</param>
    /// <returns>The rendered image's metadata</returns>
    public ImageInfo RenderShapes(RenderShapesOptions options) => api.RenderShapes(new RenderShapesRequest(options.Background, options.Height, options.Output, options.Shapes, options.Width));
}
