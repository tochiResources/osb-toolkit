package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImageAPI;
import moe.tochi.osbtk.model.ImageInfo;
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

    public ImageInfo info(PathOptions options) throws ApiException {
        return api.info(new PathRequest()
            .path(options.path));
    }

    public ImageInfo process(ImageProcessOptions options) throws ApiException {
        return api.process(new ImageProcessRequest()
            .options(options.options)
            .output(options.output)
            .path(options.path));
    }
}
