package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Easing;
import java.util.List;

/**
 * A named-or-custom easing evaluation request.
 */
public final class EasingValueOptions {
    /**
     * A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
     */
    public final List<Double> cubicBezier;
    /**
     * A named easing (used when `cubicBezier` is absent).
     */
    public final Easing easing;
    /**
     * The normalized time in `0.0..=1.0`.
     */
    public final Double t;

    private EasingValueOptions(Builder builder) {
        this.cubicBezier = builder.cubicBezier;
        this.easing = builder.easing;
        this.t = builder.t;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private List<Double> cubicBezier;
        private Easing easing;
        private Double t;

        public Builder cubicBezier(List<Double> value) {
            this.cubicBezier = value;
            return this;
        }

        public Builder easing(Easing value) {
            this.easing = value;
            return this;
        }

        public Builder t(Double value) {
            this.t = value;
            return this;
        }

        public EasingValueOptions build() {
            java.util.Objects.requireNonNull(t, "t");
            return new EasingValueOptions(this);
        }
    }
}
