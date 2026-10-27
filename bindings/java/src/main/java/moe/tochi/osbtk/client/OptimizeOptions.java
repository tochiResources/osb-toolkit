package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.OptimizeOptionsInput;
import java.util.List;

/**
 * An `.osb` optimization request.
 */
public final class OptimizeOptions {
    /**
     * The optimization options.
     */
    public final OptimizeOptionsInput options;
    /**
     * The `.osb` storyboard text.
     */
    public final String text;

    private OptimizeOptions(Builder builder) {
        this.options = builder.options;
        this.text = builder.text;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private OptimizeOptionsInput options;
        private String text;

        public Builder options(OptimizeOptionsInput value) {
            this.options = value;
            return this;
        }

        public Builder text(String value) {
            this.text = value;
            return this;
        }

        public OptimizeOptions build() {
            java.util.Objects.requireNonNull(options, "options");
            java.util.Objects.requireNonNull(text, "text");
            return new OptimizeOptions(this);
        }
    }
}
