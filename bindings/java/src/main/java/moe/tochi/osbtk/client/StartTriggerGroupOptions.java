package moe.tochi.osbtk.client;

import java.util.List;

/**
 * Start a trigger group.
 */
public final class StartTriggerGroupOptions {
    /**
     * The trigger end time in milliseconds.
     */
    public final Double endTime;
    /**
     * The trigger group index.
     */
    public final Integer group;
    /**
     * The trigger name (matches a hit object).
     */
    public final String name;
    /**
     * The trigger start time in milliseconds.
     */
    public final Double startTime;

    private StartTriggerGroupOptions(Builder builder) {
        this.endTime = builder.endTime;
        this.group = builder.group;
        this.name = builder.name;
        this.startTime = builder.startTime;
    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private Double endTime;
        private Integer group;
        private String name;
        private Double startTime;

        public Builder endTime(Double value) {
            this.endTime = value;
            return this;
        }

        public Builder group(Integer value) {
            this.group = value;
            return this;
        }

        public Builder name(String value) {
            this.name = value;
            return this;
        }

        public Builder startTime(Double value) {
            this.startTime = value;
            return this;
        }

        public StartTriggerGroupOptions build() {
            java.util.Objects.requireNonNull(endTime, "endTime");
            java.util.Objects.requireNonNull(group, "group");
            java.util.Objects.requireNonNull(name, "name");
            java.util.Objects.requireNonNull(startTime, "startTime");
            return new StartTriggerGroupOptions(this);
        }
    }
}
