package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.OsbLayer;
import java.util.List;

/**
 * Move a segment to another osu! layer (`ChangePosition`).
 */
public final class SegmentChangePositionOptions {
    /**
     * The destination osu! layer.
     */
    public final OsbLayer osbLayer;

    private SegmentChangePositionOptions(Builder builder) {
        this.osbLayer = builder.osbLayer;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private OsbLayer osbLayer;

        public Builder osbLayer(OsbLayer value) {
            this.osbLayer = value;
            return this;
        }

        public SegmentChangePositionOptions build() {
            java.util.Objects.requireNonNull(osbLayer, "osbLayer");
            return new SegmentChangePositionOptions(this);
        }
    }
}
