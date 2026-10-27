package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AnimationsAPI;
import moe.tochi.osbtk.model.ApplyEasingRequest;
import moe.tochi.osbtk.model.EasingResult;
import moe.tochi.osbtk.model.EasingValueRequest;
import java.util.List;

public final class Animations {
    private final AnimationsAPI api;

    public Animations(AnimationsAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Animations;
    }

    public EasingResult applyEasing(ApplyEasingRequest body) throws ApiException {
        return api.applyEasing(body);
    }

    public EasingResult easingValue(EasingValueRequest body) throws ApiException {
        return api.easingValue(body);
    }
}
