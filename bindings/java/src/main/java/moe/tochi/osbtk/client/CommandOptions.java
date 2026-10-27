package moe.tochi.osbtk.client;

import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.ParameterType;
import java.util.List;

/**
 * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
 * depending on `kind`.
 */
public final class CommandOptions {
    /**
     * The easing function.
     */
    public final Easing easing;
    /**
     * The end value (scalar, point or color).
     */
    public final Object end;
    /**
     * The command end time in milliseconds.
     */
    public final Double endTime;
    /**
     * The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
     * `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
     * `flipV`, `additive`).
     */
    public final String kind;
    /**
     * The parameter kind for a `parameter` command.
     */
    public final ParameterType parameter;
    /**
     * The start value (scalar, point or color).
     */
    public final Object start;
    /**
     * The command start time in milliseconds.
     */
    public final Double startTime;

    private CommandOptions(Builder builder) {
        this.easing = builder.easing;
        this.end = builder.end;
        this.endTime = builder.endTime;
        this.kind = builder.kind;
        this.parameter = builder.parameter;
        this.start = builder.start;
        this.startTime = builder.startTime;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Easing easing;
        private Object end;
        private Double endTime;
        private String kind;
        private ParameterType parameter;
        private Object start;
        private Double startTime;

        public Builder easing(Easing value) {
            this.easing = value;
            return this;
        }

        public Builder end(Object value) {
            this.end = value;
            return this;
        }

        public Builder endTime(Double value) {
            this.endTime = value;
            return this;
        }

        public Builder kind(String value) {
            this.kind = value;
            return this;
        }

        public Builder parameter(ParameterType value) {
            this.parameter = value;
            return this;
        }

        public Builder start(Object value) {
            this.start = value;
            return this;
        }

        public Builder startTime(Double value) {
            this.startTime = value;
            return this;
        }

        public CommandOptions build() {
            java.util.Objects.requireNonNull(easing, "easing");
            java.util.Objects.requireNonNull(endTime, "endTime");
            java.util.Objects.requireNonNull(kind, "kind");
            java.util.Objects.requireNonNull(startTime, "startTime");
            return new CommandOptions(this);
        }
    }
}
