package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.CurveSpec;
import java.util.List;

/**
 * A curve sampling request.
 */
public final class CurveSampleOptions {
    /**
     * Sample by arc length instead of the raw parameter (default `false`).
     */
    public final Boolean byArcLength;
    /**
     * The number of points to sample.
     */
    public final Integer count;
    /**
     * The curve.
     */
    public final CurveSpec curve;

    private CurveSampleOptions(Builder builder) {
        this.byArcLength = builder.byArcLength;
        this.count = builder.count;
        this.curve = builder.curve;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Boolean byArcLength;
        private Integer count;
        private CurveSpec curve;

        public Builder byArcLength(Boolean value) {
            this.byArcLength = value;
            return this;
        }

        public Builder count(Integer value) {
            this.count = value;
            return this;
        }

        public Builder curve(CurveSpec value) {
            this.curve = value;
            return this;
        }

        public CurveSampleOptions build() {
            java.util.Objects.requireNonNull(count, "count");
            java.util.Objects.requireNonNull(curve, "curve");
            return new CurveSampleOptions(this);
        }
    }
}
