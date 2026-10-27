using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using AudioApi = tochiRes.osbTK.Api.AudioAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The audio library: FFT.</summary>
    public static class Audio
    {
        private static readonly AudioApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Reads FFT magnitudes from a `.wav` file.</summary>
        public static FftResult Fft(FftRequest request)
        {
            ArgumentNullException.ThrowIfNull(request);
            return Api.Fft(request);
        }
    }
}
