using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Media
{
    private readonly MediaAPI api;

    public Media(MediaAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Media;

    public MediaInfo EncodeVideo(EncodeVideoRequest body) => api.EncodeVideo(body);

    public OkResult ExtractAudio(ExtractAudioRequest body) => api.ExtractAudio(body);

    public FrameResult ExtractFrames(ExtractFramesRequest body) => api.ExtractFrames(body);

    public MediaInfo Info(PathRequest body) => api.Info(body);
}
