package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.ImageOptions;
import java.util.List;

/**
 * An image processing request: read `path`, apply `options`, write `output`.
 */
public final class ImageProcessOptions {
    /**
     * The processing options.
     */
    public final ImageOptions options;
    /**
     * The output file path (its extension selects the format).
     */
    public final String output;
    /**
     * The input file path.
     */
    public final String path;

    private ImageProcessOptions(Builder builder) {
        this.options = builder.options;
        this.output = builder.output;
        this.path = builder.path;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private ImageOptions options;
        private String output;
        private String path;

        public Builder options(ImageOptions value) {
            this.options = value;
            return this;
        }

        public Builder output(String value) {
            this.output = value;
            return this;
        }

        public Builder path(String value) {
            this.path = value;
            return this;
        }

        public ImageProcessOptions build() {
            java.util.Objects.requireNonNull(options, "options");
            java.util.Objects.requireNonNull(output, "output");
            java.util.Objects.requireNonNull(path, "path");
            return new ImageProcessOptions(this);
        }
    }
}
