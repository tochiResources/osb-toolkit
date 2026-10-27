package moe.tochi.osbtk.client;

import java.util.List;

/**
 * An audio-extraction request (media -> audio file).
 */
public final class ExtractAudioOptions {
    /**
     * The input media path.
     */
    public final String input;
    /**
     * The output audio path (its extension selects the format).
     */
    public final String output;

    private ExtractAudioOptions(Builder builder) {
        this.input = builder.input;
        this.output = builder.output;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String input;
        private String output;

        public Builder input(String value) {
            this.input = value;
            return this;
        }

        public Builder output(String value) {
            this.output = value;
            return this;
        }

        public ExtractAudioOptions build() {
            java.util.Objects.requireNonNull(input, "input");
            java.util.Objects.requireNonNull(output, "output");
            return new ExtractAudioOptions(this);
        }
    }
}
