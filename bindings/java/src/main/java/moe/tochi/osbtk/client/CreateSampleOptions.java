package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Create a sound sample on a layer.
 */
public final class CreateSampleOptions {
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    public final String layer;
    /**
     * The sample path (relative to the mapset).
     */
    public final String path;
    /**
     * The sample time in milliseconds.
     */
    public final Double time;
    /**
     * The sample volume (`0..=100`).
     */
    public final Double volume;

    private CreateSampleOptions(Builder builder) {
        this.layer = builder.layer;
        this.path = builder.path;
        this.time = builder.time;
        this.volume = builder.volume;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String layer;
        private String path;
        private Double time;
        private Double volume;

        public Builder layer(String value) {
            this.layer = value;
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

        public Builder volume(Double value) {
            this.volume = value;
            return this;
        }

        public CreateSampleOptions build() {
            java.util.Objects.requireNonNull(path, "path");
            java.util.Objects.requireNonNull(time, "time");
            return new CreateSampleOptions(this);
        }
    }
}
