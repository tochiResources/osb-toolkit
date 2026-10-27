import { AnimationsAPI } from '../../api/AnimationsAPI';
import type { Easing } from '../../models/Easing';
import type { EasingResult } from '../../models/EasingResult';
import type { ApplyEasingOptions, EasingValueOptions } from '../options';
import { isApplyEasingOptions, isEasingValueOptions } from '../options';

export class Animations {
    constructor(private readonly api: AnimationsAPI) {}

    static instanceOf(value: unknown): value is Animations {
        return value instanceof Animations;
    }

    async applyEasing(options: ApplyEasingOptions): Promise<EasingResult>;
    async applyEasing(easing: Easing, t: number): Promise<EasingResult>;
    async applyEasing(optionsOrEasing?: ApplyEasingOptions | Easing, t?: number): Promise<EasingResult> {
        const options: ApplyEasingOptions = isApplyEasingOptions(optionsOrEasing)
            ? optionsOrEasing
            : ({ easing: optionsOrEasing, t: t } as ApplyEasingOptions);
        return this.api.applyEasing(options);
    }

    async easingValue(options: EasingValueOptions): Promise<EasingResult>;
    async easingValue(t: number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<EasingResult>;
    async easingValue(optionsOrT?: EasingValueOptions | number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<EasingResult> {
        const options: EasingValueOptions = isEasingValueOptions(optionsOrT)
            ? optionsOrT
            : ({ t: optionsOrT, easing: easing, cubicBezier: cubicBezier } as EasingValueOptions);
        return this.api.easingValue(options);
    }
}
