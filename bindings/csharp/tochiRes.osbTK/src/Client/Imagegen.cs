using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Imagegen
{
    private readonly ImagegenAPI api;

    public Imagegen(ImagegenAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Imagegen;

    public ImageInfo RenderShapes(RenderShapesRequest body) => api.RenderShapes(body);
}
