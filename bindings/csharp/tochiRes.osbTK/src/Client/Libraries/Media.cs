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

    /// <summary>
    /// An image-sequence encoding request (frames -&gt; video).
    /// </summary>
    /// <param name="InputPattern">The input filename pattern (e.g. `frames/frame_%04d.png`).</param>
    /// <param name="Output">The output video path.</param>
    /// <param name="Fps">The frame rate.</param>
    /// <returns>The encoded video's metadata</returns>
    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps });

    /// <summary>
    /// An image-sequence encoding request (frames -&gt; video).
    /// </summary>
    /// <param name="InputPattern">The input filename pattern (e.g. `frames/frame_%04d.png`).</param>
    /// <param name="Output">The output video path.</param>
    /// <param name="Fps">The frame rate.</param>
    /// <param name="Codec">The video codec (default `libx264`).</param>
    /// <returns>The encoded video's metadata</returns>
    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps, string Codec) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps, Codec = Codec });

    /// <summary>
    /// An image-sequence encoding request (frames -&gt; video).
    /// </summary>
    /// <param name="InputPattern">The input filename pattern (e.g. `frames/frame_%04d.png`).</param>
    /// <param name="Output">The output video path.</param>
    /// <param name="Fps">The frame rate.</param>
    /// <param name="Codec">The video codec (default `libx264`).</param>
    /// <param name="Crf">The constant-rate-factor quality (`0`-`51`; lower is better).</param>
    /// <returns>The encoded video's metadata</returns>
    public MediaInfo EncodeVideo(string InputPattern, string Output, double Fps, string Codec, int? Crf) =>
        EncodeVideo(new EncodeVideoOptions { InputPattern = InputPattern, Output = Output, Fps = Fps, Codec = Codec, Crf = Crf });

    /// <summary>
    /// An image-sequence encoding request (frames -&gt; video).
    /// </summary>
    /// <param name="options">An image-sequence encoding request (frames -&gt; video).</param>
    /// <returns>The encoded video's metadata</returns>
    public MediaInfo EncodeVideo(EncodeVideoOptions options) => api.EncodeVideo(new EncodeVideoRequest(options.Codec, options.Crf, options.Fps, options.InputPattern, options.Output));

    /// <summary>
    /// An audio-extraction request (media -&gt; audio file).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="Output">The output audio path (its extension selects the format).</param>
    public void ExtractAudio(string Input, string Output) =>
        ExtractAudio(new ExtractAudioOptions { Input = Input, Output = Output });

    /// <summary>
    /// An audio-extraction request (media -&gt; audio file).
    /// </summary>
    /// <param name="options">An audio-extraction request (media -&gt; audio file).</param>
    public void ExtractAudio(ExtractAudioOptions options)
    {
        api.ExtractAudio(new ExtractAudioRequest(options.Input, options.Output));
    }

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <param name="Count">The maximum number of frames to extract.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <param name="Count">The maximum number of frames to extract.</param>
    /// <param name="Fps">The sampling rate (frames per second); defaults to the source rate.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <param name="Count">The maximum number of frames to extract.</param>
    /// <param name="Fps">The sampling rate (frames per second); defaults to the source rate.</param>
    /// <param name="Height">Scale frames to this height.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <param name="Count">The maximum number of frames to extract.</param>
    /// <param name="Fps">The sampling rate (frames per second); defaults to the source rate.</param>
    /// <param name="Height">Scale frames to this height.</param>
    /// <param name="Start">The start time in seconds.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height, double? Start) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height, Start = Start });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="Input">The input media path.</param>
    /// <param name="OutputDir">The directory to write frames into.</param>
    /// <param name="Count">The maximum number of frames to extract.</param>
    /// <param name="Fps">The sampling rate (frames per second); defaults to the source rate.</param>
    /// <param name="Height">Scale frames to this height.</param>
    /// <param name="Start">The start time in seconds.</param>
    /// <param name="Width">Scale frames to this width.</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(string Input, string OutputDir, int? Count, double? Fps, int? Height, double? Start, int? Width) =>
        ExtractFrames(new ExtractFramesOptions { Input = Input, OutputDir = OutputDir, Count = Count, Fps = Fps, Height = Height, Start = Start, Width = Width });

    /// <summary>
    /// A frame-extraction request (video -&gt; image sequence).
    /// </summary>
    /// <param name="options">A frame-extraction request (video -&gt; image sequence).</param>
    /// <returns>The extracted frame count</returns>
    public FrameResult ExtractFrames(ExtractFramesOptions options) => api.ExtractFrames(new ExtractFramesRequest(options.Count, options.Fps, options.Height, options.Input, options.OutputDir, options.Start, options.Width));

    /// <summary>
    /// A request carrying a single file path.
    /// </summary>
    /// <param name="Path">The input file path.</param>
    /// <returns>The media metadata</returns>
    public MediaInfo Info(string Path) =>
        Info(new PathOptions { Path = Path });

    /// <summary>
    /// A request carrying a single file path.
    /// </summary>
    /// <param name="options">A request carrying a single file path.</param>
    /// <returns>The media metadata</returns>
    public MediaInfo Info(PathOptions options) => api.Info(new PathRequest(options.Path));
}
