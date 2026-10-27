package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Record a depth index on a segment.
 */
public final class SegmentAddIndexOptions {
    /**
     * The index to record.
     */
    public final Double index;

    private SegmentAddIndexOptions(Builder builder) {
        this.index = builder.index;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double index;

        public Builder index(Double value) {
            this.index = value;
            return this;
        }

        public SegmentAddIndexOptions build() {
            java.util.Objects.requireNonNull(index, "index");
            return new SegmentAddIndexOptions(this);
        }
    }
}
