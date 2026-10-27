package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Set a segment's reverse-depth flag.
 */
public final class SegmentReverseDepthOptions {
    /**
     * Whether children are emitted in reverse order.
     */
    public final Boolean value;

    private SegmentReverseDepthOptions(Builder builder) {
        this.value = builder.value;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Boolean value;

        public Builder value(Boolean value) {
            this.value = value;
            return this;
        }

        public SegmentReverseDepthOptions build() {
            java.util.Objects.requireNonNull(value, "value");
            return new SegmentReverseDepthOptions(this);
        }
    }
}
