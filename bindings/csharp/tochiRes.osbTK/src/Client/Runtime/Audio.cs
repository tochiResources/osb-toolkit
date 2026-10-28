#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

/// <summary>
/// The clean `Audio` client.
/// </summary>
public sealed class Audio
{
    private readonly AudioAPI api;

    public Audio(AudioAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Audio;

    /// <summary>
    /// Calls the Audio.audioDuration bridge operation.
    /// </summary>
    /// <returns>The loaded audio duration</returns>
    public double AudioDuration() => api.AudioDuration().AudioDuration;

    /// <summary>
    /// An FFT request against a `.wav` file.
    /// </summary>
    /// <param name="Path">Path to the audio file.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult Fft(string Path) =>
        Fft(new FftOptions { Path = Path });

    /// <summary>
    /// An FFT request against a `.wav` file.
    /// </summary>
    /// <param name="Path">Path to the audio file.</param>
    /// <param name="Magnitudes">Number of frequency bins.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult Fft(string Path, int? Magnitudes) =>
        Fft(new FftOptions { Path = Path, Magnitudes = Magnitudes });

    /// <summary>
    /// An FFT request against a `.wav` file.
    /// </summary>
    /// <param name="Path">Path to the audio file.</param>
    /// <param name="Magnitudes">Number of frequency bins.</param>
    /// <param name="Time">Sample time in milliseconds.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult Fft(string Path, int? Magnitudes, double? Time) =>
        Fft(new FftOptions { Path = Path, Magnitudes = Magnitudes, Time = Time });

    /// <summary>
    /// An FFT request against a `.wav` file.
    /// </summary>
    /// <param name="options">An FFT request against a `.wav` file.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult Fft(FftOptions options) => api.Fft(new FftRequest(options.Magnitudes, options.Path, options.Time));

    /// <summary>
    /// An FFT request against the runtime's loaded audio.
    /// </summary>
    /// <param name="Time">Sample time in milliseconds.</param>
    /// <param name="Magnitudes">Number of frequency bins.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult GetFft(double Time, int Magnitudes) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes });

    /// <summary>
    /// An FFT request against the runtime's loaded audio.
    /// </summary>
    /// <param name="Time">Sample time in milliseconds.</param>
    /// <param name="Magnitudes">Number of frequency bins.</param>
    /// <param name="Cutoff">A frequency cut-off.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult GetFft(double Time, int Magnitudes, double? Cutoff) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes, Cutoff = Cutoff });

    /// <summary>
    /// An FFT request against the runtime's loaded audio.
    /// </summary>
    /// <param name="Time">Sample time in milliseconds.</param>
    /// <param name="Magnitudes">Number of frequency bins.</param>
    /// <param name="Cutoff">A frequency cut-off.</param>
    /// <param name="Easing">The easing applied across the bins.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult GetFft(double Time, int Magnitudes, double? Cutoff, Easing? Easing) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes, Cutoff = Cutoff, Easing = Easing });

    /// <summary>
    /// An FFT request against the runtime's loaded audio.
    /// </summary>
    /// <param name="options">An FFT request against the runtime's loaded audio.</param>
    /// <returns>The FFT magnitudes</returns>
    public FftResult GetFft(GetFftOptions options) => api.GetFft(new GetFftRequest(options.Cutoff, options.Easing, options.Magnitudes, options.Time));
}
