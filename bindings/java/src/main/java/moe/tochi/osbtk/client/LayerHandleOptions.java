package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Resolve (or create) a layer by name.
 */
public final class LayerHandleOptions {
    /**
     * The layer name.
     */
    public final String name;

    private LayerHandleOptions(Builder builder) {
        this.name = builder.name;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String name;

        public Builder name(String value) {
            this.name = value;
            return this;
        }

        public LayerHandleOptions build() {
            java.util.Objects.requireNonNull(name, "name");
            return new LayerHandleOptions(this);
        }
    }
}
