package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A sprite handle pair plus a sample time.
 */
public final class SpriteTimeOptions {
    /**
     * The sample time in milliseconds.
     */
    public final Double time;

    private SpriteTimeOptions(Builder builder) {
        this.time = builder.time;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double time;

        public Builder time(Double value) {
            this.time = value;
            return this;
        }

        public SpriteTimeOptions build() {
            java.util.Objects.requireNonNull(time, "time");
            return new SpriteTimeOptions(this);
        }
    }
}
