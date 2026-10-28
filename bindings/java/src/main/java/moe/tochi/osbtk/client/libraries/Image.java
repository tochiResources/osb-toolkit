package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImageAPI;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.ImageOptions;
import moe.tochi.osbtk.model.ImageProcessRequest;
import moe.tochi.osbtk.model.PathRequest;
import moe.tochi.osbtk.client.ImageProcessOptions;
import moe.tochi.osbtk.client.PathOptions;
import java.util.List;

public final class Image {
    private final ImageAPI api;

    public Image(ImageAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Image;
    }

    /**
     * A request carrying a single file path.
     *
     * @param path The input file path.
     *
     * @return The image's metadata
     */
    public ImageInfo info(String path) throws ApiException {
        return info(PathOptions.builder().path(path).build());
    }

    /**
     * A request carrying a single file path.
     *
     * @param options A request carrying a single file path.
     *
     * @return The image's metadata
     */
    public ImageInfo info(PathOptions options) throws ApiException {
        return api.info(new PathRequest()
            .path(options.path));
    }

    /**
     * An image processing request: read `path`, apply `options`, write `output`.
     *
     * @param options The processing options.
     * @param output The output file path (its extension selects the format).
     * @param path The input file path.
     *
     * @return The processed image's metadata
     */
    public ImageInfo process(ImageOptions options, String output, String path) throws ApiException {
        return process(ImageProcessOptions.builder().options(options).output(output).path(path).build());
    }

    /**
     * An image processing request: read `path`, apply `options`, write `output`.
     *
     * @param options An image processing request: read `path`, apply `options`, write `output`.
     *
     * @return The processed image's metadata
     */
    public ImageInfo process(ImageProcessOptions options) throws ApiException {
        return api.process(new ImageProcessRequest()
            .options(options.options)
            .output(options.output)
            .path(options.path));
    }
}
