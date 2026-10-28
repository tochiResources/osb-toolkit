package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.AnimationsAPI;
import moe.tochi.osbtk.model.ApplyEasingRequest;
import moe.tochi.osbtk.model.Easing;
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

    /**
     * An easing evaluation request.
     *
     * @param easing The easing function.
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @return The eased value
     */
    public double applyEasing(Easing easing, Double t) throws ApiException {
        return applyEasing(ApplyEasingOptions.builder().easing(easing).t(t).build());
    }

    /**
     * An easing evaluation request.
     *
     * @param options An easing evaluation request.
     *
     * @return The eased value
     */
    public double applyEasing(ApplyEasingOptions options) throws ApiException {
        return api.applyEasing(new ApplyEasingRequest()
            .easing(options.easing)
            .t(options.t)).getValue();
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @return The eased value (named easing or cubic-bezier)
     */
    public double easingValue(Double t) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).build());
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     *
     * @return The eased value (named easing or cubic-bezier)
     */
    public double easingValue(Double t, Easing easing) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).easing(easing).build());
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     * @param cubicBezier A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
     *
     * @return The eased value (named easing or cubic-bezier)
     */
    public double easingValue(Double t, Easing easing, List<Double> cubicBezier) throws ApiException {
        return easingValue(EasingValueOptions.builder().t(t).easing(easing).cubicBezier(cubicBezier).build());
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param options A named-or-custom easing evaluation request.
     *
     * @return The eased value (named easing or cubic-bezier)
     */
    public double easingValue(EasingValueOptions options) throws ApiException {
        return api.easingValue(new EasingValueRequest()
            .cubicBezier(options.cubicBezier)
            .easing(options.easing)
            .t(options.t)).getValue();
    }
}
