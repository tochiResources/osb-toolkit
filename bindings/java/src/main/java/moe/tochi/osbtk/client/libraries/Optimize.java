package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.OptimizeAPI;
import moe.tochi.osbtk.model.OptimizeOptionsInput;
import moe.tochi.osbtk.model.OptimizeRequest;
import moe.tochi.osbtk.model.OptimizeResult;
import moe.tochi.osbtk.client.OptimizeOptions;
import java.util.List;

/**
 * The clean `Optimize` client.
 */
public final class Optimize {
    private final OptimizeAPI api;

    public Optimize(OptimizeAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Optimize;
    }

    /**
     * An `.osb` optimization request.
     *
     * @param options The optimization options.
     * @param text The `.osb` storyboard text.
     *
     * @return The optimized `.osb` text and stats
     */
    public OptimizeResult optimizeOsb(OptimizeOptionsInput options, String text) throws ApiException {
        return optimizeOsb(OptimizeOptions.builder().options(options).text(text).build());
    }

    /**
     * An `.osb` optimization request.
     *
     * @param options An `.osb` optimization request.
     *
     * @return The optimized `.osb` text and stats
     */
    public OptimizeResult optimizeOsb(OptimizeOptions options) throws ApiException {
        return api.optimizeOsb(new OptimizeRequest()
            .options(options.options)
            .text(options.text));
    }
}
