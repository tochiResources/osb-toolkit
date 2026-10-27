package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImagegenAPI;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.RenderShapesRequest;
import java.util.List;

public final class Imagegen {
    private final ImagegenAPI api;

    public Imagegen(ImagegenAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Imagegen;
    }

    public ImageInfo renderShapes(RenderShapesRequest body) throws ApiException {
        return api.renderShapes(body);
    }
}
