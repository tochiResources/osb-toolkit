package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ImagegenAPI;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.RenderShapesRequest;
import moe.tochi.osbtk.model.RgbaColor;
import moe.tochi.osbtk.model.Shape;
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

    public ImageInfo renderShapes(Integer width, Integer height, List<Shape> shapes, String output) throws ApiException {
        return renderShapes(RenderShapesOptions.builder().width(width).height(height).shapes(shapes).output(output).build());
    }

    public ImageInfo renderShapes(Integer width, Integer height, List<Shape> shapes, String output, RgbaColor background) throws ApiException {
        return renderShapes(RenderShapesOptions.builder().width(width).height(height).shapes(shapes).output(output).background(background).build());
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
