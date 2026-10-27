package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Easing;
import java.util.List;

/**
 * An FFT request against the runtime's loaded audio.
 */
public final class GetFftOptions {
    /**
     * A frequency cut-off.
     */
    public final Double cutoff;
    /**
     * The easing applied across the bins.
     */
    public final Easing easing;
    /**
     * Number of frequency bins.
     */
    public final Integer magnitudes;
    /**
     * Sample time in milliseconds.
     */
    public final Double time;

    private GetFftOptions(Builder builder) {
        this.cutoff = builder.cutoff;
        this.easing = builder.easing;
        this.magnitudes = builder.magnitudes;
        this.time = builder.time;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double cutoff;
        private Easing easing;
        private Integer magnitudes;
        private Double time;

        public Builder cutoff(Double value) {
            this.cutoff = value;
            return this;
        }

        public Builder easing(Easing value) {
            this.easing = value;
            return this;
        }

        public Builder magnitudes(Integer value) {
            this.magnitudes = value;
            return this;
        }

        public Builder time(Double value) {
            this.time = value;
            return this;
        }

        public GetFftOptions build() {
            java.util.Objects.requireNonNull(magnitudes, "magnitudes");
            java.util.Objects.requireNonNull(time, "time");
            return new GetFftOptions(this);
        }
    }
}
