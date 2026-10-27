package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.HelpersAPI;
import moe.tochi.osbtk.model.HelperRequest;
import moe.tochi.osbtk.model.HelperResult;
import java.util.List;

public final class Helpers {
    private final HelpersAPI api;

    public Helpers(HelpersAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Helpers;
    }

    public HelperResult call(HelperRequest body) throws ApiException {
        return api.call(body);
    }

    public HelperResult commands(HelperRequest body) throws ApiException {
        return api.commands(body);
    }

    public HelperResult keyframes(HelperRequest body) throws ApiException {
        return api.keyframes(body);
    }
}
