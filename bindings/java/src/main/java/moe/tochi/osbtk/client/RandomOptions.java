package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A deterministic random request.
 */
public final class RandomOptions {
    /**
     * The exclusive upper bound.
     */
    public final Double max;
    /**
     * The inclusive lower bound.
     */
    public final Double min;

    private RandomOptions(Builder builder) {
        this.max = builder.max;
        this.min = builder.min;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double max;
        private Double min;

        public Builder max(Double value) {
            this.max = value;
            return this;
        }

        public Builder min(Double value) {
            this.min = value;
            return this;
        }

        public RandomOptions build() {
            java.util.Objects.requireNonNull(max, "max");
            java.util.Objects.requireNonNull(min, "min");
            return new RandomOptions(this);
        }
    }
}
