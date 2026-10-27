package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Scene3dInput;
import java.util.List;

/**
 * A `bake3d` request: a plain 3D scene.
 */
public final class Bake3dOptions {
    /**
     * The 3D scene.
     */
    public final Scene3dInput input;

    private Bake3dOptions(Builder builder) {
        this.input = builder.input;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Scene3dInput input;

        public Builder input(Scene3dInput value) {
            this.input = value;
            return this;
        }

        public Bake3dOptions build() {
            java.util.Objects.requireNonNull(input, "input");
            return new Bake3dOptions(this);
        }
    }
}
