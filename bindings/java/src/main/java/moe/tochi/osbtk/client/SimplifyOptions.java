package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * A Douglas-Peucker simplification request.
 */
public final class SimplifyOptions {
    /**
     * The polyline to simplify.
     */
    public final List<Vector2> points;
    /**
     * The maximum deviation to allow.
     */
    public final Double tolerance;

    private SimplifyOptions(Builder builder) {
        this.points = builder.points;
        this.tolerance = builder.tolerance;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private List<Vector2> points;
        private Double tolerance;

        public Builder points(List<Vector2> value) {
            this.points = value;
            return this;
        }

        public Builder tolerance(Double value) {
            this.tolerance = value;
            return this;
        }

        public SimplifyOptions build() {
            java.util.Objects.requireNonNull(points, "points");
            java.util.Objects.requireNonNull(tolerance, "tolerance");
            return new SimplifyOptions(this);
        }
    }
}
