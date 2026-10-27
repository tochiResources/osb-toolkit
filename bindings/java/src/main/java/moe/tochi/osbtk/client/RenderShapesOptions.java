package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.RgbaColor;
import moe.tochi.osbtk.model.Shape;
import java.util.List;

/**
 * A shape-rendering request: a canvas, a background and a shape list.
 */
public final class RenderShapesOptions {
    /**
     * The background colour (default transparent).
     */
    public final RgbaColor background;
    /**
     * The canvas height in pixels.
     */
    public final Integer height;
    /**
     * The output file path (its extension selects the format).
     */
    public final String output;
    /**
     * The shapes to draw, in order.
     */
    public final List<Shape> shapes;
    /**
     * The canvas width in pixels.
     */
    public final Integer width;

    private RenderShapesOptions(Builder builder) {
        this.background = builder.background;
        this.height = builder.height;
        this.output = builder.output;
        this.shapes = builder.shapes;
        this.width = builder.width;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private RgbaColor background;
        private Integer height;
        private String output;
        private List<Shape> shapes;
        private Integer width;

        public Builder background(RgbaColor value) {
            this.background = value;
            return this;
        }

        public Builder height(Integer value) {
            this.height = value;
            return this;
        }

        public Builder output(String value) {
            this.output = value;
            return this;
        }

        public Builder shapes(List<Shape> value) {
            this.shapes = value;
            return this;
        }

        public Builder width(Integer value) {
            this.width = value;
            return this;
        }

        public RenderShapesOptions build() {
            java.util.Objects.requireNonNull(height, "height");
            java.util.Objects.requireNonNull(output, "output");
            java.util.Objects.requireNonNull(shapes, "shapes");
            java.util.Objects.requireNonNull(width, "width");
            return new RenderShapesOptions(this);
        }
    }
}
