package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImagegenAPI;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.RenderShapesRequest;
import moe.tochi.osbtk.client.RenderShapesOptions;
import java.util.List;

public final class Imagegen {
    private final ImagegenAPI api;

    public Imagegen(ImagegenAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Imagegen;
    }

    public ImageInfo renderShapes(RenderShapesOptions options) throws ApiException {
        return api.renderShapes(new RenderShapesRequest()
            .background(options.background)
            .height(options.height)
            .output(options.output)
            .shapes(options.shapes)
            .width(options.width));
    }
}
