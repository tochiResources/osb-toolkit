package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.CommonAPI;
import java.util.List;

public final class Common {
    private final CommonAPI api;

    public Common(CommonAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Common;
    }

    public String version() throws ApiException {
        return api.version().getVersion();
    }
}
