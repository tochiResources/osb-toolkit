package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImageAPI;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.ImageProcessRequest;
import moe.tochi.osbtk.model.PathRequest;
import java.util.List;

public final class Image {
    private final ImageAPI api;

    public Image(ImageAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Image;
    }

    public ImageInfo info(PathRequest body) throws ApiException {
        return api.info(body);
    }

    public ImageInfo process(ImageProcessRequest body) throws ApiException {
        return api.process(body);
    }
}
