package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.MediaAPI;
import moe.tochi.osbtk.model.EncodeVideoRequest;
import moe.tochi.osbtk.model.ExtractAudioRequest;
import moe.tochi.osbtk.model.ExtractFramesRequest;
import moe.tochi.osbtk.model.FrameResult;
import moe.tochi.osbtk.model.MediaInfo;
import moe.tochi.osbtk.model.OkResult;
import moe.tochi.osbtk.model.PathRequest;
import java.util.List;

public final class Media {
    private final MediaAPI api;

    public Media(MediaAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Media;
    }

    public MediaInfo encodeVideo(EncodeVideoRequest body) throws ApiException {
        return api.encodeVideo(body);
    }

    public OkResult extractAudio(ExtractAudioRequest body) throws ApiException {
        return api.extractAudio(body);
    }

    public FrameResult extractFrames(ExtractFramesRequest body) throws ApiException {
        return api.extractFrames(body);
    }

    public MediaInfo info(PathRequest body) throws ApiException {
        return api.info(body);
    }
}
