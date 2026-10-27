package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.OptimizeAPI;
import moe.tochi.osbtk.model.OptimizeOptionsInput;
import moe.tochi.osbtk.model.OptimizeRequest;
import moe.tochi.osbtk.model.OptimizeResult;
import moe.tochi.osbtk.client.OptimizeOptions;
import java.util.List;

public final class Optimize {
    private final OptimizeAPI api;

    public Optimize(OptimizeAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Optimize;
    }

    public OptimizeResult optimizeOsb(OptimizeOptionsInput options, String text) throws ApiException {
        return optimizeOsb(OptimizeOptions.builder().options(options).text(text).build());
    }

    public OptimizeResult optimizeOsb(OptimizeOptions options) throws ApiException {
        return api.optimizeOsb(new OptimizeRequest()
            .options(options.options)
            .text(options.text));
    }
}
