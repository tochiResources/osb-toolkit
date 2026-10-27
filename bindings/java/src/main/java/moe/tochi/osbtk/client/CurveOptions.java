package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.CurveSpec;
import java.util.List;

/**
 * A curve query.
 */
public final class CurveOptions {
    /**
     * The curve.
     */
    public final CurveSpec curve;

    private CurveOptions(Builder builder) {
        this.curve = builder.curve;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private CurveSpec curve;

        public Builder curve(CurveSpec value) {
            this.curve = value;
            return this;
        }

        public CurveOptions build() {
            java.util.Objects.requireNonNull(curve, "curve");
            return new CurveOptions(this);
        }
    }
}
