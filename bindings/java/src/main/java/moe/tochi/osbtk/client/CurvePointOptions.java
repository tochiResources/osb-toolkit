package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.CurveSpec;
import java.util.List;

/**
 * A curve query at a parametric or arc-length position.
 */
public final class CurvePointOptions {
    /**
     * Sample by arc length instead of the raw parameter (default `false`).
     */
    public final Boolean byArcLength;
    /**
     * The curve.
     */
    public final CurveSpec curve;
    /**
     * The position in `0.0..=1.0`.
     */
    public final Double t;

    private CurvePointOptions(Builder builder) {
        this.byArcLength = builder.byArcLength;
        this.curve = builder.curve;
        this.t = builder.t;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Boolean byArcLength;
        private CurveSpec curve;
        private Double t;

        public Builder byArcLength(Boolean value) {
            this.byArcLength = value;
            return this;
        }

        public Builder curve(CurveSpec value) {
            this.curve = value;
            return this;
        }

        public Builder t(Double value) {
            this.t = value;
            return this;
        }

        public CurvePointOptions build() {
            java.util.Objects.requireNonNull(curve, "curve");
            java.util.Objects.requireNonNull(t, "t");
            return new CurvePointOptions(this);
        }
    }
}
