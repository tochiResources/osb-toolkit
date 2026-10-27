package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * Whether a rotated sprite's bounding box intersects the widescreen
 * storyboard area.
 */
public final class InScreenBoundsOptions {
    /**
     * The anchor offset (from `GetOriginVector`).
     */
    public final Vector2 origin;
    /**
     * The sprite centre position.
     */
    public final Vector2 position;
    /**
     * The rotation in radians.
     */
    public final Double rotation;
    /**
     * The sprite size.
     */
    public final Vector2 size;

    private InScreenBoundsOptions(Builder builder) {
        this.origin = builder.origin;
        this.position = builder.position;
        this.rotation = builder.rotation;
        this.size = builder.size;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Vector2 origin;
        private Vector2 position;
        private Double rotation;
        private Vector2 size;

        public Builder origin(Vector2 value) {
            this.origin = value;
            return this;
        }

        public Builder position(Vector2 value) {
            this.position = value;
            return this;
        }

        public Builder rotation(Double value) {
            this.rotation = value;
            return this;
        }

        public Builder size(Vector2 value) {
            this.size = value;
            return this;
        }

        public InScreenBoundsOptions build() {
            java.util.Objects.requireNonNull(origin, "origin");
            java.util.Objects.requireNonNull(position, "position");
            java.util.Objects.requireNonNull(rotation, "rotation");
            java.util.Objects.requireNonNull(size, "size");
            return new InScreenBoundsOptions(this);
        }
    }
}
