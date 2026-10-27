package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A request carrying a block of text (an effect script, `.osu` or `.osb`).
 */
public final class TextOptions {
    /**
     * The text payload.
     */
    public final String text;

    private TextOptions(Builder builder) {
        this.text = builder.text;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String text;

        public Builder text(String value) {
            this.text = value;
            return this;
        }

        public TextOptions build() {
            java.util.Objects.requireNonNull(text, "text");
            return new TextOptions(this);
        }
    }
}
