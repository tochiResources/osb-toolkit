package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A frame-extraction request (video -> image sequence).
 */
public final class ExtractFramesOptions {
    /**
     * The maximum number of frames to extract.
     */
    public final Integer count;
    /**
     * The sampling rate (frames per second); defaults to the source rate.
     */
    public final Double fps;
    /**
     * Scale frames to this height.
     */
    public final Integer height;
    /**
     * The input media path.
     */
    public final String input;
    /**
     * The directory to write frames into.
     */
    public final String outputDir;
    /**
     * The start time in seconds.
     */
    public final Double start;
    /**
     * Scale frames to this width.
     */
    public final Integer width;

    private ExtractFramesOptions(Builder builder) {
        this.count = builder.count;
        this.fps = builder.fps;
        this.height = builder.height;
        this.input = builder.input;
        this.outputDir = builder.outputDir;
        this.start = builder.start;
        this.width = builder.width;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Integer count;
        private Double fps;
        private Integer height;
        private String input;
        private String outputDir;
        private Double start;
        private Integer width;

        public Builder count(Integer value) {
            this.count = value;
            return this;
        }

        public Builder fps(Double value) {
            this.fps = value;
            return this;
        }

        public Builder height(Integer value) {
            this.height = value;
            return this;
        }

        public Builder input(String value) {
            this.input = value;
            return this;
        }

        public Builder outputDir(String value) {
            this.outputDir = value;
            return this;
        }

        public Builder start(Double value) {
            this.start = value;
            return this;
        }

        public Builder width(Integer value) {
            this.width = value;
            return this;
        }

        public ExtractFramesOptions build() {
            java.util.Objects.requireNonNull(input, "input");
            java.util.Objects.requireNonNull(outputDir, "outputDir");
            return new ExtractFramesOptions(this);
        }
    }
}
