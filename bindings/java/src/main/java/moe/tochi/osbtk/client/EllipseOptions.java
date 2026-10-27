package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * An ellipse sampling request.
 */
public final class EllipseOptions {
    /**
     * The ellipse centre.
     */
    public final Vector2 center;
    /**
     * The number of points (default `64`).
     */
    public final Integer count;
    /**
     * The horizontal radius.
     */
    public final Double radiusX;
    /**
     * The vertical radius.
     */
    public final Double radiusY;

    private EllipseOptions(Builder builder) {
        this.center = builder.center;
        this.count = builder.count;
        this.radiusX = builder.radiusX;
        this.radiusY = builder.radiusY;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Vector2 center;
        private Integer count;
        private Double radiusX;
        private Double radiusY;

        public Builder center(Vector2 value) {
            this.center = value;
            return this;
        }

        public Builder count(Integer value) {
            this.count = value;
            return this;
        }

        public Builder radiusX(Double value) {
            this.radiusX = value;
            return this;
        }

        public Builder radiusY(Double value) {
            this.radiusY = value;
            return this;
        }

        public EllipseOptions build() {
            java.util.Objects.requireNonNull(center, "center");
            java.util.Objects.requireNonNull(radiusX, "radiusX");
            java.util.Objects.requireNonNull(radiusY, "radiusY");
            return new EllipseOptions(this);
        }
    }
}
