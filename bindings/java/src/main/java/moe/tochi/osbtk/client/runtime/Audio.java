package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AudioAPI;
import moe.tochi.osbtk.model.AudioDurationResult;
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

    public AudioDurationResult audioDuration() throws ApiException {
        return api.audioDuration();
    }

    public FftResult fft(FftOptions options) throws ApiException {
        return api.fft(new FftRequest()
            .magnitudes(options.magnitudes)
            .path(options.path)
            .time(options.time));
    }

    public FftResult getFft(GetFftOptions options) throws ApiException {
        return api.getFft(new GetFftRequest()
            .cutoff(options.cutoff)
            .easing(options.easing)
            .magnitudes(options.magnitudes)
            .time(options.time));
    }
}
