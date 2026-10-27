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

    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps });

    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps, string Codec) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps, Codec = Codec });

    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps, string Codec, int? Crf) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps, Codec = Codec, Crf = Crf });

    public MediaInfo EncodeVideo(EncodeVideoOptions options) => api.EncodeVideo(new EncodeVideoRequest(options.Codec, options.Crf, options.Fps, options.InputPattern, options.Output));

    public void ExtractAudio(string Input, string Output) =>
        ExtractAudio(new ExtractAudioOptions { Input = Input, Output = Output });

    public void ExtractAudio(ExtractAudioOptions options)
    {
        api.ExtractAudio(new ExtractAudioRequest(options.Input, options.Output));
    }

    public FrameResult ExtractFrames(string Input, string OutputDir) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir });

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count });

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps });

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height });

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height, double? Start) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height, Start = Start });

    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height, double? Start, int? Width) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height, Start = Start, Width = Width });

    public FrameResult ExtractFrames(ExtractFramesOptions options) => api.ExtractFrames(new ExtractFramesRequest(options.Count, options.Fps, options.Height, options.Input, options.OutputDir, options.Start, options.Width));

    public MediaInfo Info(string Path) =>
        Info(new PathOptions { Path = Path });

    public MediaInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));
}
