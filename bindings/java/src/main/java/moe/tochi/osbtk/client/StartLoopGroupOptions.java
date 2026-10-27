package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Start a loop group.
 */
public final class StartLoopGroupOptions {
    /**
     * The number of iterations.
     */
    public final Double count;
    /**
     * The loop start time in milliseconds.
     */
    public final Double startTime;

    private StartLoopGroupOptions(Builder builder) {
        this.count = builder.count;
        this.startTime = builder.startTime;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double count;
        private Double startTime;

        public Builder count(Double value) {
            this.count = value;
            return this;
        }

        public Builder startTime(Double value) {
            this.startTime = value;
            return this;
        }

        public StartLoopGroupOptions build() {
            java.util.Objects.requireNonNull(count, "count");
            java.util.Objects.requireNonNull(startTime, "startTime");
            return new StartLoopGroupOptions(this);
        }
    }
}
