package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AudioAPI;
import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.FftRequest;
import moe.tochi.osbtk.model.FftResult;
import moe.tochi.osbtk.model.GetFftRequest;
import moe.tochi.osbtk.client.FftOptions;
import moe.tochi.osbtk.client.GetFftOptions;
import java.util.List;

public final class Audio {
    private final AudioAPI api;

    public Audio(AudioAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Audio;
    }

    /**
     * Calls the Audio.audioDuration bridge operation.
     *
     * @return The loaded audio duration
     */
    public double audioDuration() throws ApiException {
        return api.audioDuration().getAudioDuration();
    }

    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     *
     * @return The FFT magnitudes
     */
    public FftResult fft(String path) throws ApiException {
        return fft(FftOptions.builder().path(path).build());
    }

    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     * @param magnitudes Number of frequency bins.
     *
     * @return The FFT magnitudes
     */
    public FftResult fft(String path, Integer magnitudes) throws ApiException {
        return fft(FftOptions.builder().path(path).magnitudes(magnitudes).build());
    }

    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     * @param magnitudes Number of frequency bins.
     * @param time Sample time in milliseconds.
     *
     * @return The FFT magnitudes
     */
    public FftResult fft(String path, Integer magnitudes, Double time) throws ApiException {
        return fft(FftOptions.builder().path(path).magnitudes(magnitudes).time(time).build());
    }

    /**
     * An FFT request against a `.wav` file.
     *
     * @param options An FFT request against a `.wav` file.
     *
     * @return The FFT magnitudes
     */
    public FftResult fft(FftOptions options) throws ApiException {
        return api.fft(new FftRequest()
            .magnitudes(options.magnitudes)
            .path(options.path)
            .time(options.time));
    }

    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     *
     * @return The FFT magnitudes
     */
    public FftResult getFft(Double time, Integer magnitudes) throws ApiException {
        return getFft(GetFftOptions.builder().time(time).magnitudes(magnitudes).build());
    }

    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     * @param cutoff A frequency cut-off.
     *
     * @return The FFT magnitudes
     */
    public FftResult getFft(Double time, Integer magnitudes, Double cutoff) throws ApiException {
        return getFft(GetFftOptions.builder().time(time).magnitudes(magnitudes).cutoff(cutoff).build());
    }

    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     * @param cutoff A frequency cut-off.
     * @param easing The easing applied across the bins.
     *
     * @return The FFT magnitudes
     */
    public FftResult getFft(Double time, Integer magnitudes, Double cutoff, Easing easing) throws ApiException {
        return getFft(GetFftOptions.builder().time(time).magnitudes(magnitudes).cutoff(cutoff).easing(easing).build());
    }

    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param options An FFT request against the runtime's loaded audio.
     *
     * @return The FFT magnitudes
     */
    public FftResult getFft(GetFftOptions options) throws ApiException {
        return api.getFft(new GetFftRequest()
            .cutoff(options.cutoff)
            .easing(options.easing)
            .magnitudes(options.magnitudes)
            .time(options.time));
    }
}
