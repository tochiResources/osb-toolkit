package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.MediaAPI;
import moe.tochi.osbtk.model.EncodeVideoRequest;
import moe.tochi.osbtk.model.ExtractAudioRequest;
import moe.tochi.osbtk.model.ExtractFramesRequest;
import moe.tochi.osbtk.model.FrameResult;
import moe.tochi.osbtk.model.MediaInfo;
import moe.tochi.osbtk.model.OkResult;
import moe.tochi.osbtk.model.PathRequest;
import moe.tochi.osbtk.client.EncodeVideoOptions;
import moe.tochi.osbtk.client.ExtractAudioOptions;
import moe.tochi.osbtk.client.ExtractFramesOptions;
import moe.tochi.osbtk.client.PathOptions;
import java.util.List;

public final class Media {
    private final MediaAPI api;

    public Media(MediaAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Media;
    }

    public MediaInfo encodeVideo(EncodeVideoOptions options) throws ApiException {
        return api.encodeVideo(new EncodeVideoRequest()
            .codec(options.codec)
            .crf(options.crf)
            .fps(options.fps)
            .inputPattern(options.inputPattern)
            .output(options.output));
    }

    public OkResult extractAudio(ExtractAudioOptions options) throws ApiException {
        return api.extractAudio(new ExtractAudioRequest()
            .input(options.input)
            .output(options.output));
    }

    public FrameResult extractFrames(ExtractFramesOptions options) throws ApiException {
        return api.extractFrames(new ExtractFramesRequest()
            .count(options.count)
            .fps(options.fps)
            .height(options.height)
            .input(options.input)
            .outputDir(options.outputDir)
            .start(options.start)
            .width(options.width));
    }

    public MediaInfo info(PathOptions options) throws ApiException {
        return api.info(new PathRequest()
            .path(options.path));
    }
}
