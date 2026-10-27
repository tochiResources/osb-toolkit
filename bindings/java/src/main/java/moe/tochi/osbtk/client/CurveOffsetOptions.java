package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.CurveSpec;
import java.util.List;

/**
 * A curve offset request.
 */
public final class CurveOffsetOptions {
    /**
     * The curve.
     */
    public final CurveSpec curve;
    /**
     * The signed offset along the curve's local normal.
     */
    public final Double distance;

    private CurveOffsetOptions(Builder builder) {
        this.curve = builder.curve;
        this.distance = builder.distance;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private CurveSpec curve;
        private Double distance;

        public Builder curve(CurveSpec value) {
            this.curve = value;
            return this;
        }

        public Builder distance(Double value) {
            this.distance = value;
            return this;
        }

        public CurveOffsetOptions build() {
            java.util.Objects.requireNonNull(curve, "curve");
            java.util.Objects.requireNonNull(distance, "distance");
            return new CurveOffsetOptions(this);
        }
    }
}
