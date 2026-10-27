package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * Create a sprite.
 */
public final class CreateSpriteOptions {
    /**
     * The target layer name (omitted when `layerHandle` is set).
     */
    public final String layer;
    /**
     * The sprite anchor origin.
     */
    public final OsbOrigin origin;
    /**
     * The texture path (relative to the mapset).
     */
    public final String path;
    /**
     * The initial position; omitted means the sprite's default.
     */
    public final Vector2 position;

    private CreateSpriteOptions(Builder builder) {
        this.layer = builder.layer;
        this.origin = builder.origin;
        this.path = builder.path;
        this.position = builder.position;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String layer;
        private OsbOrigin origin;
        private String path;
        private Vector2 position;

        public Builder layer(String value) {
            this.layer = value;
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

        public CreateSpriteOptions build() {
            java.util.Objects.requireNonNull(origin, "origin");
            java.util.Objects.requireNonNull(path, "path");
            return new CreateSpriteOptions(this);
        }
    }
}
