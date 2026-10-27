package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A time query.
 */
public final class TimeOptions {
    /**
     * The time in milliseconds.
     */
    public final Double time;

    private TimeOptions(Builder builder) {
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

        public TimeOptions build() {
            java.util.Objects.requireNonNull(time, "time");
            return new TimeOptions(this);
        }
    }
}
