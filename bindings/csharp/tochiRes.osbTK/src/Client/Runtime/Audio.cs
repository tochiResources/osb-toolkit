#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Audio
{
    private readonly AudioAPI api;

    public Audio(AudioAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Audio;

    public double AudioDuration() => api.AudioDuration().AudioDuration;

    public FftResult Fft(string Path) =>
        Fft(new FftOptions { Path = Path });

    public FftResult Fft(string Path, int? Magnitudes) =>
        Fft(new FftOptions { Path = Path, Magnitudes = Magnitudes });

    public FftResult Fft(string Path, int? Magnitudes, double? Time) =>
        Fft(new FftOptions { Path = Path, Magnitudes = Magnitudes, Time = Time });

    public FftResult Fft(FftOptions options) => api.Fft(new FftRequest(options.Magnitudes, options.Path, options.Time));

    public FftResult GetFft(double Time, int Magnitudes) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes });

    public FftResult GetFft(double Time, int Magnitudes, double? Cutoff) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes, Cutoff = Cutoff });

    public FftResult GetFft(double Time, int Magnitudes, double? Cutoff, Easing? Easing) =>
        GetFft(new GetFftOptions { Time = Time, Magnitudes = Magnitudes, Cutoff = Cutoff, Easing = Easing });

    public FftResult GetFft(GetFftOptions options) => api.GetFft(new GetFftRequest(options.Cutoff, options.Easing, options.Magnitudes, options.Time));
}
