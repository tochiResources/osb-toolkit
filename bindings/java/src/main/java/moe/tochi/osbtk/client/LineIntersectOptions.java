package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Vector2;
import java.util.List;

/**
 * A line-segment intersection request.
 */
public final class LineIntersectOptions {
    /**
     * The first segment's start.
     */
    public final Vector2 a1;
    /**
     * The first segment's end.
     */
    public final Vector2 a2;
    /**
     * The second segment's start.
     */
    public final Vector2 b1;
    /**
     * The second segment's end.
     */
    public final Vector2 b2;

    private LineIntersectOptions(Builder builder) {
        this.a1 = builder.a1;
        this.a2 = builder.a2;
        this.b1 = builder.b1;
        this.b2 = builder.b2;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Vector2 a1;
        private Vector2 a2;
        private Vector2 b1;
        private Vector2 b2;

        public Builder a1(Vector2 value) {
            this.a1 = value;
            return this;
        }

        public Builder a2(Vector2 value) {
            this.a2 = value;
            return this;
        }

        public Builder b1(Vector2 value) {
            this.b1 = value;
            return this;
        }

        public Builder b2(Vector2 value) {
            this.b2 = value;
            return this;
        }

        public LineIntersectOptions build() {
            java.util.Objects.requireNonNull(a1, "a1");
            java.util.Objects.requireNonNull(a2, "a2");
            java.util.Objects.requireNonNull(b1, "b1");
            java.util.Objects.requireNonNull(b2, "b2");
            return new LineIntersectOptions(this);
        }
    }
}
