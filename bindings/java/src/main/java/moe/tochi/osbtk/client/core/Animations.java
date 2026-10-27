package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AnimationsAPI;
import moe.tochi.osbtk.model.ApplyEasingRequest;
import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.EasingResult;
import moe.tochi.osbtk.model.EasingValueRequest;
import moe.tochi.osbtk.client.ApplyEasingOptions;
import moe.tochi.osbtk.client.EasingValueOptions;
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

    public EasingResult applyEasing(Easing easing, Double t) throws ApiException {
        return applyEasing(ApplyEasingOptions.builder().easing(easing).t(t).build());
    }

    public EasingResult easingValue(EasingValueOptions options) throws ApiException {
        return api.easingValue(new EasingValueRequest()
            .cubicBezier(options.cubicBezier)
            .easing(options.easing)
            .t(options.t));
    }

    public EasingResult easingValue(Double t) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).build());
    }

    public EasingResult easingValue(Double t, Easing easing) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).easing(easing).build());
    }

    public EasingResult easingValue(Double t, Easing easing, List<Double> cubicBezier) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).easing(easing).cubicBezier(cubicBezier).build());
    }
}
