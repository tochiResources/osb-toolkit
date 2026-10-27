package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A request carrying a single file path.
 */
public final class PathOptions {
    /**
     * The input file path.
     */
    public final String path;

    private PathOptions(Builder builder) {
        this.path = builder.path;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String path;

        public Builder path(String value) {
            this.path = value;
            return this;
        }

        public PathOptions build() {
            java.util.Objects.requireNonNull(path, "path");
            return new PathOptions(this);
        }
    }
}
