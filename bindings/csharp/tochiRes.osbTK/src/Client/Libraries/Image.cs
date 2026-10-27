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

    public ImageInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));

    public ImageInfo Process(ImageProcessOptions options) => api.Process(new ImageProcessRequest(options.Options, options.Output, options.Path));
}
