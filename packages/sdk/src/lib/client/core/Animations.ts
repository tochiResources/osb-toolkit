import { AnimationsAPI } from '../../api/AnimationsAPI';
import type { Easing } from '../../models/Easing';
import type { ApplyEasingOptions, EasingValueOptions } from '../options';
import { isApplyEasingOptions, isEasingValueOptions } from '../options';

/**
 * The clean `Animations` client.
 */
export class Animations {
    constructor(private readonly api: AnimationsAPI) {}

    static instanceOf(value: unknown): value is Animations {
        return value instanceof Animations;
    }

    /**
     * An easing evaluation request.
     *
     * @param easing The easing function.
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @returns The eased value
     */
    async applyEasing(easing: Easing, t: number): Promise<number>;
    /**
     * An easing evaluation request.
     *
     * @param options An easing evaluation request.
     *
     * @returns The eased value
     */
    async applyEasing(options: ApplyEasingOptions): Promise<number>;
    async applyEasing(optionsOrEasing?: ApplyEasingOptions | Easing, t?: number): Promise<number> {
        const options: ApplyEasingOptions = isApplyEasingOptions(optionsOrEasing)
            ? optionsOrEasing
            : ({ easing: optionsOrEasing, t: t } as ApplyEasingOptions);
        return (await this.api.applyEasing(options)).value;
    }

    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number, easing: Easing | null): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param t The normalized time in `0.0..=1.0`.
     * @param easing A named easing (used when `cubicBezier` is absent).
     * @param cubicBezier A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(t: number, easing: Easing | null, cubicBezier: number[] | null): Promise<number>;
    /**
     * A named-or-custom easing evaluation request.
     *
     * @param options A named-or-custom easing evaluation request.
     *
     * @returns The eased value (named easing or cubic-bezier)
     */
    async easingValue(options: EasingValueOptions): Promise<number>;
    async easingValue(optionsOrT?: EasingValueOptions | number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<number> {
        const options: EasingValueOptions = isEasingValueOptions(optionsOrT)
            ? optionsOrT
            : ({ t: optionsOrT, easing: easing, cubicBezier: cubicBezier } as EasingValueOptions);
        return (await this.api.easingValue(options)).value;
    }
}
