using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Audio
{
    private readonly AudioAPI api;

    public Audio(AudioAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Audio;

    public AudioDurationResult AudioDuration() => api.AudioDuration();

    public FftResult Fft(FftRequest body) => api.Fft(body);

    public FftResult GetFft(GetFftRequest body) => api.GetFft(body);
}
