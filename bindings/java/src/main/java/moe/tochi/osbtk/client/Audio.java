package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AudioAPI;
import moe.tochi.osbtk.model.AudioDurationResult;
import moe.tochi.osbtk.model.FftRequest;
import moe.tochi.osbtk.model.FftResult;
import moe.tochi.osbtk.model.GetFftRequest;
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

    public FftResult fft(FftRequest body) throws ApiException {
        return api.fft(body);
    }

    public FftResult getFft(GetFftRequest body) throws ApiException {
        return api.getFft(body);
    }
}
