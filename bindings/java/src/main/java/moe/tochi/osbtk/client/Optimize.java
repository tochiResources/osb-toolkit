package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.OptimizeAPI;
import moe.tochi.osbtk.model.OptimizeRequest;
import moe.tochi.osbtk.model.OptimizeResult;
import java.util.List;

public final class Optimize {
    private final OptimizeAPI api;

    public Optimize(OptimizeAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Optimize;
    }

    public OptimizeResult optimizeOsb(OptimizeRequest body) throws ApiException {
        return api.optimizeOsb(body);
    }
}
