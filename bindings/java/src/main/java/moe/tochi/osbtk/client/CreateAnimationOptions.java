package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.OsbLoopType;
import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * Create an animation sprite.
 */
public final class CreateAnimationOptions {
    /**
     * Delay between frames, in milliseconds.
     */
    public final Double frameDelay;
    /**
     * The number of frames.
     */
    public final Integer frames;
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    public final String layer;
    /**
     * How the animation loops.
     */
    public final OsbLoopType loopType;
    /**
     * The sprite anchor origin.
     */
    public final OsbOrigin origin;
    /**
     * The texture path (relative to the mapset).
     */
    public final String path;
    /**
     * The initial position.
     */
    public final Vector2 position;

    private CreateAnimationOptions(Builder builder) {
        this.frameDelay = builder.frameDelay;
        this.frames = builder.frames;
        this.layer = builder.layer;
        this.loopType = builder.loopType;
        this.origin = builder.origin;
        this.path = builder.path;
        this.position = builder.position;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double frameDelay;
        private Integer frames;
        private String layer;
        private OsbLoopType loopType;
        private OsbOrigin origin;
        private String path;
        private Vector2 position;

        public Builder frameDelay(Double value) {
            this.frameDelay = value;
            return this;
        }

        public Builder frames(Integer value) {
            this.frames = value;
            return this;
        }

        public Builder layer(String value) {
            this.layer = value;
            return this;
        }

        public Builder loopType(OsbLoopType value) {
            this.loopType = value;
            return this;
        }

        public Builder origin(OsbOrigin value) {
            this.origin = value;
            return this;
        }

        public Builder path(String value) {
            this.path = value;
            return this;
        }

        public Builder position(Vector2 value) {
            this.position = value;
            return this;
        }

        public CreateAnimationOptions build() {
            java.util.Objects.requireNonNull(frameDelay, "frameDelay");
            java.util.Objects.requireNonNull(frames, "frames");
            java.util.Objects.requireNonNull(loopType, "loopType");
            java.util.Objects.requireNonNull(origin, "origin");
            java.util.Objects.requireNonNull(path, "path");
            java.util.Objects.requireNonNull(position, "position");
            return new CreateAnimationOptions(this);
        }
    }
}
