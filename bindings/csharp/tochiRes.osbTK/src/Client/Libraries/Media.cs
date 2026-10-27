#nullable disable
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

    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps, string Codec = default, int? Crf = default) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps, Codec = Codec, Crf = Crf });

    public OkResult ExtractAudio(ExtractAudioOptions options) => api.ExtractAudio(new ExtractAudioRequest(options.Input, options.Output));

    public OkResult ExtractAudio(string Input, string Output) =>
        ExtractAudio(new ExtractAudioOptions { Input = Input, Output = Output });

    public FrameResult ExtractFrames(ExtractFramesOptions options) => api.ExtractFrames(new ExtractFramesRequest(options.Count, options.Fps, options.Height, options.Input, options.OutputDir, options.Start, options.Width));

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count = default, double? Fps = default, int? Height = default, double? Start = default, int? Width = default) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height, Start = Start, Width = Width });

    public MediaInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));

    public MediaInfo Info(string Path) =>
        Info(new PathOptions { Path = Path });
}
