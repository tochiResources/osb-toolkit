package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A segment handle request.
 */
public final class SegmentOptions {


    private SegmentOptions(Builder builder) {

    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {




        public SegmentOptions build() {
            return new SegmentOptions(this);
        }
    }
}
