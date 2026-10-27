using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Image
{
    private readonly ImageAPI api;

    public Image(ImageAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Image;

    public ImageInfo Info(PathRequest body) => api.Info(body);

    public ImageInfo Process(ImageProcessRequest body) => api.Process(body);
}
