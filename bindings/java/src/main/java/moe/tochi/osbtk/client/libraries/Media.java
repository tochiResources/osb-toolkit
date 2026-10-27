package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.MediaAPI;
import moe.tochi.osbtk.model.EncodeVideoRequest;
import moe.tochi.osbtk.model.ExtractAudioRequest;
import moe.tochi.osbtk.model.ExtractFramesRequest;
import moe.tochi.osbtk.model.FrameResult;
import moe.tochi.osbtk.model.MediaInfo;
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

    public MediaInfo encodeVideo(String inputPattern, String output, Double fps) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).build());
    }

    public MediaInfo encodeVideo(String inputPattern, String output, Double fps, String codec) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).codec(codec).build());
    }

    public MediaInfo encodeVideo(String inputPattern, String output, Double fps, String codec, Integer crf) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).codec(codec).crf(crf).build());
    }

    public MediaInfo encodeVideo(EncodeVideoOptions options) throws ApiException {
        return api.encodeVideo(new EncodeVideoRequest()
            .codec(options.codec)
            .crf(options.crf)
            .fps(options.fps)
            .inputPattern(options.inputPattern)
            .output(options.output));
    }

    public void extractAudio(String input, String output) throws ApiException {
        extractAudio(ExtractAudioOptions.builder().input(input).output(output).build());
    }

    public void extractAudio(ExtractAudioOptions options) throws ApiException {
        api.extractAudio(new ExtractAudioRequest()
            .input(options.input)
            .output(options.output));
    }

    public FrameResult extractFrames(String input, String outputDir) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).build());
    }

    public FrameResult extractFrames(String input, String outputDir, Integer count) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).build());
    }

    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).build());
    }

    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).build());
    }

    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height, Double start) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).start(start).build());
    }

    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height, Double start, Integer width) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).start(start).width(width).build());
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

    public MediaInfo info(String path) throws ApiException {
        return info(PathOptions.builder().path(path).build());
    }

    public MediaInfo info(PathOptions options) throws ApiException {
        return api.info(new PathRequest()
            .path(options.path));
    }
}
