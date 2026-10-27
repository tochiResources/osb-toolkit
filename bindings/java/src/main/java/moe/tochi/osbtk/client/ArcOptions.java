package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * A circular-arc sampling request.
 */
public final class ArcOptions {
    /**
     * The arc centre.
     */
    public final Vector2 center;
    /**
     * The number of points (default `64`).
     */
    public final Integer count;
    /**
     * The radius.
     */
    public final Double radius;
    /**
     * The start angle in degrees.
     */
    public final Double startAngle;
    /**
     * The sweep angle in degrees.
     */
    public final Double sweepAngle;

    private ArcOptions(Builder builder) {
        this.center = builder.center;
        this.count = builder.count;
        this.radius = builder.radius;
        this.startAngle = builder.startAngle;
        this.sweepAngle = builder.sweepAngle;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Vector2 center;
        private Integer count;
        private Double radius;
        private Double startAngle;
        private Double sweepAngle;

        public Builder center(Vector2 value) {
            this.center = value;
            return this;
        }

        public Builder count(Integer value) {
            this.count = value;
            return this;
        }

        public Builder radius(Double value) {
            this.radius = value;
            return this;
        }

        public Builder startAngle(Double value) {
            this.startAngle = value;
            return this;
        }

        public Builder sweepAngle(Double value) {
            this.sweepAngle = value;
            return this;
        }

        public ArcOptions build() {
            java.util.Objects.requireNonNull(center, "center");
            java.util.Objects.requireNonNull(radius, "radius");
            java.util.Objects.requireNonNull(startAngle, "startAngle");
            java.util.Objects.requireNonNull(sweepAngle, "sweepAngle");
            return new ArcOptions(this);
        }
    }
}
