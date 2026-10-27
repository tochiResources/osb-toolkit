package moe.tochi.osbtk.client;

import java.util.List;

/**
 * A sprite handle pair.
 */
public final class HandleOptions {


    private HandleOptions(Builder builder) {

    }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {




        public HandleOptions build() {
            return new HandleOptions(this);
        }
    }
}
