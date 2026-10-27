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

    public AudioDurationResult AudioDuration() => api.AudioDuration();

    public FftResult Fft(FftOptions options) => api.Fft(new FftRequest(options.Magnitudes, options.Path, options.Time));

    public FftResult GetFft(GetFftOptions options) => api.GetFft(new GetFftRequest(options.Cutoff, options.Easing, options.Magnitudes, options.Time));
}
