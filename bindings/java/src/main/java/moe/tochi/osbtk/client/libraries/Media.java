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

/**
 * The clean `Media` client.
 */
public final class Media {
    private final MediaAPI api;

    public Media(MediaAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Media;
    }

    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     *
     * @return The encoded video's metadata
     */
    public MediaInfo encodeVideo(String inputPattern, String output, Double fps) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).build());
    }

    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     * @param codec The video codec (default `libx264`).
     *
     * @return The encoded video's metadata
     */
    public MediaInfo encodeVideo(String inputPattern, String output, Double fps, String codec) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).codec(codec).build());
    }

    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     * @param codec The video codec (default `libx264`).
     * @param crf The constant-rate-factor quality (`0`-`51`; lower is better).
     *
     * @return The encoded video's metadata
     */
    public MediaInfo encodeVideo(String inputPattern, String output, Double fps, String codec, Integer crf) throws ApiException {
        return encodeVideo(EncodeVideoOptions.builder().inputPattern(inputPattern).output(output).fps(fps).codec(codec).crf(crf).build());
    }

    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param options An image-sequence encoding request (frames -> video).
     *
     * @return The encoded video's metadata
     */
    public MediaInfo encodeVideo(EncodeVideoOptions options) throws ApiException {
        return api.encodeVideo(new EncodeVideoRequest()
            .codec(options.codec)
            .crf(options.crf)
            .fps(options.fps)
            .inputPattern(options.inputPattern)
            .output(options.output));
    }

    /**
     * An audio-extraction request (media -> audio file).
     *
     * @param input The input media path.
     * @param output The output audio path (its extension selects the format).
     */
    public void extractAudio(String input, String output) throws ApiException {
        extractAudio(ExtractAudioOptions.builder().input(input).output(output).build());
    }

    /**
     * An audio-extraction request (media -> audio file).
     *
     * @param options An audio-extraction request (media -> audio file).
     */
    public void extractAudio(ExtractAudioOptions options) throws ApiException {
        api.extractAudio(new ExtractAudioRequest()
            .input(options.input)
            .output(options.output));
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir, Integer count) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     * @param start The start time in seconds.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height, Double start) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).start(start).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     * @param start The start time in seconds.
     * @param width Scale frames to this width.
     *
     * @return The extracted frame count
     */
    public FrameResult extractFrames(String input, String outputDir, Integer count, Double fps, Integer height, Double start, Integer width) throws ApiException {
        return extractFrames(ExtractFramesOptions.builder().input(input).outputDir(outputDir).count(count).fps(fps).height(height).start(start).width(width).build());
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param options A frame-extraction request (video -> image sequence).
     *
     * @return The extracted frame count
     */
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

    /**
     * A request carrying a single file path.
     *
     * @param path The input file path.
     *
     * @return The media metadata
     */
    public MediaInfo info(String path) throws ApiException {
        return info(PathOptions.builder().path(path).build());
    }

    /**
     * A request carrying a single file path.
     *
     * @param options A request carrying a single file path.
     *
     * @return The media metadata
     */
    public MediaInfo info(PathOptions options) throws ApiException {
        return api.info(new PathRequest()
            .path(options.path));
    }
}
