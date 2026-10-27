package moe.tochi.osbtk.client;

import java.util.List;

/**
 * An FFT request against a `.wav` file.
 */
public final class FftOptions {
    /**
     * Number of frequency bins.
     */
    public final Integer magnitudes;
    /**
     * Path to the audio file.
     */
    public final String path;
    /**
     * Sample time in milliseconds.
     */
    public final Double time;

    private FftOptions(Builder builder) {
        this.magnitudes = builder.magnitudes;
        this.path = builder.path;
        this.time = builder.time;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Integer magnitudes;
        private String path;
        private Double time;

        public Builder magnitudes(Integer value) {
            this.magnitudes = value;
            return this;
        }

        public Builder path(String value) {
            this.path = value;
            return this;
        }

        public Builder time(Double value) {
            this.time = value;
            return this;
        }

        public FftOptions build() {
            java.util.Objects.requireNonNull(path, "path");
            return new FftOptions(this);
        }
    }
}
