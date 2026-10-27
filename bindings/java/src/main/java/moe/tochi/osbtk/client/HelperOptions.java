package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A motion-graphics helper call: a helper `name` plus its input.
 */
public final class HelperOptions {
    /**
     * The helper's input (a keyframe channel, a command list, or its parameters).
     */
    public final String input;
    /**
     * The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     */
    public final String name;

    private HelperOptions(Builder builder) {
        this.input = builder.input;
        this.name = builder.name;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private String input;
        private String name;

        public Builder input(String value) {
            this.input = value;
            return this;
        }

        public Builder name(String value) {
            this.name = value;
            return this;
        }

        public HelperOptions build() {
            java.util.Objects.requireNonNull(name, "name");
            return new HelperOptions(this);
        }
    }
}
