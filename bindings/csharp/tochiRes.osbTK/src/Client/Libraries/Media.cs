using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Libraries;

public sealed class Media
{
    private readonly MediaAPI api;

    public Media(MediaAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Media;

    public MediaInfo EncodeVideo(EncodeVideoOptions options) => api.EncodeVideo(new EncodeVideoRequest(options.Codec, options.Crf, options.Fps, options.InputPattern, options.Output));

    public OkResult ExtractAudio(ExtractAudioOptions options) => api.ExtractAudio(new ExtractAudioRequest(options.Input, options.Output));

    public FrameResult ExtractFrames(ExtractFramesOptions options) => api.ExtractFrames(new ExtractFramesRequest(options.Count, options.Fps, options.Height, options.Input, options.OutputDir, options.Start, options.Width));

    public MediaInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));
}
