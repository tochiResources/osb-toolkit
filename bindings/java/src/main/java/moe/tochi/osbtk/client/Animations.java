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

    public EasingResult applyEasing(ApplyEasingOptions options) throws ApiException {
        return api.applyEasing(new ApplyEasingRequest()
            .easing(options.easing)
            .t(options.t));
    }

    public EasingResult easingValue(EasingValueOptions options) throws ApiException {
        return api.easingValue(new EasingValueRequest()
            .cubicBezier(options.cubicBezier)
            .easing(options.easing)
            .t(options.t));
    }
}
