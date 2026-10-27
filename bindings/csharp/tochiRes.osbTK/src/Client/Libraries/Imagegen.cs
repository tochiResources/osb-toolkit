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

    public ImageInfo RenderShapes(RenderShapesOptions options) => api.RenderShapes(new RenderShapesRequest(options.Background, options.Height, options.Output, options.Shapes, options.Width));

    public ImageInfo RenderShapes(int Width, int Height, List<Shape> Shapes, string Output, RgbaColor Background = default) =>
        RenderShapes(new RenderShapesOptions { Width = Width, Height = Height, Shapes = Shapes, Output = Output, Background = Background });
}
