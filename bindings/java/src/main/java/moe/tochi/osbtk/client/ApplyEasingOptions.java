package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Easing;
import java.util.List;

/**
 * An easing evaluation request.
 */
public final class ApplyEasingOptions {
    /**
     * The easing function.
     */
    public final Easing easing;
    /**
     * The normalized time in `0.0..=1.0`.
     */
    public final Double t;

    private ApplyEasingOptions(Builder builder) {
        this.easing = builder.easing;
        this.t = builder.t;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Easing easing;
        private Double t;

        public Builder easing(Easing value) {
            this.easing = value;
            return this;
        }

        public Builder t(Double value) {
            this.t = value;
            return this;
        }

        public ApplyEasingOptions build() {
            java.util.Objects.requireNonNull(easing, "easing");
            java.util.Objects.requireNonNull(t, "t");
            return new ApplyEasingOptions(this);
        }
    }
}
