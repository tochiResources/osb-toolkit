package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.CurveSpec;
import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * A curve hit-test request.
 */
public final class CurveDistanceOptions {
    /**
     * The curve.
     */
    public final CurveSpec curve;
    /**
     * The query point.
     */
    public final Vector2 point;

    private CurveDistanceOptions(Builder builder) {
        this.curve = builder.curve;
        this.point = builder.point;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private CurveSpec curve;
        private Vector2 point;

        public Builder curve(CurveSpec value) {
            this.curve = value;
            return this;
        }

        public Builder point(Vector2 value) {
            this.point = value;
            return this;
        }

        public CurveDistanceOptions build() {
            java.util.Objects.requireNonNull(curve, "curve");
            java.util.Objects.requireNonNull(point, "point");
            return new CurveDistanceOptions(this);
        }
    }
}
