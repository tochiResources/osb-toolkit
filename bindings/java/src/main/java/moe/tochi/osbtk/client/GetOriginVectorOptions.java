package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.OsbOrigin;
import java.util.List;

/**
 * The anchor offset for an origin.
 */
public final class GetOriginVectorOptions {
    /**
     * The sprite height.
     */
    public final Double height;
    /**
     * The sprite anchor origin.
     */
    public final OsbOrigin origin;
    /**
     * The sprite width.
     */
    public final Double width;

    private GetOriginVectorOptions(Builder builder) {
        this.height = builder.height;
        this.origin = builder.origin;
        this.width = builder.width;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double height;
        private OsbOrigin origin;
        private Double width;

        public Builder height(Double value) {
            this.height = value;
            return this;
        }

        public Builder origin(OsbOrigin value) {
            this.origin = value;
            return this;
        }

        public Builder width(Double value) {
            this.width = value;
            return this;
        }

        public GetOriginVectorOptions build() {
            java.util.Objects.requireNonNull(height, "height");
            java.util.Objects.requireNonNull(origin, "origin");
            java.util.Objects.requireNonNull(width, "width");
            return new GetOriginVectorOptions(this);
        }
    }
}
