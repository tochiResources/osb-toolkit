package moe.tochi.osbtk.client;

import java.util.List;

/**
 * An image-sequence encoding request (frames -> video).
 */
public final class EncodeVideoOptions {
    /**
     * The video codec (default `libx264`).
     */
    public final String codec;
    /**
     * The constant-rate-factor quality (`0`-`51`; lower is better).
     */
    public final Integer crf;
    /**
     * The frame rate.
     */
    public final Double fps;
    /**
     * The input filename pattern (e.g. `frames/frame_%04d.png`).
     */
    public final String inputPattern;
    /**
     * The output video path.
     */
    public final String output;

    private EncodeVideoOptions(Builder builder) {
        this.codec = builder.codec;
        this.crf = builder.crf;
        this.fps = builder.fps;
        this.inputPattern = builder.inputPattern;
        this.output = builder.output;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String codec;
        private Integer crf;
        private Double fps;
        private String inputPattern;
        private String output;

        public Builder codec(String value) {
            this.codec = value;
            return this;
        }

        public Builder crf(Integer value) {
            this.crf = value;
            return this;
        }

        public Builder fps(Double value) {
            this.fps = value;
            return this;
        }

        public Builder inputPattern(String value) {
            this.inputPattern = value;
            return this;
        }

        public Builder output(String value) {
            this.output = value;
            return this;
        }

        public EncodeVideoOptions build() {
            java.util.Objects.requireNonNull(fps, "fps");
            java.util.Objects.requireNonNull(inputPattern, "inputPattern");
            java.util.Objects.requireNonNull(output, "output");
            return new EncodeVideoOptions(this);
        }
    }
}
