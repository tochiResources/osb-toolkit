import { AnimationsAPI } from '../../api/AnimationsAPI';
import type { Easing } from '../../models/Easing';
import type { ApplyEasingOptions, EasingValueOptions } from '../options';
import { isApplyEasingOptions, isEasingValueOptions } from '../options';

export class Animations {
    constructor(private readonly api: AnimationsAPI) {}

    static instanceOf(value: unknown): value is Animations {
        return value instanceof Animations;
    }

    async applyEasing(options: ApplyEasingOptions): Promise<number>;
    async applyEasing(easing: Easing, t: number): Promise<number>;
    async applyEasing(optionsOrEasing?: ApplyEasingOptions | Easing, t?: number): Promise<number> {
        const options: ApplyEasingOptions = isApplyEasingOptions(optionsOrEasing)
            ? optionsOrEasing
            : ({ easing: optionsOrEasing, t: t } as ApplyEasingOptions);
        return (await this.api.applyEasing(options)).value;
    }

    async easingValue(options: EasingValueOptions): Promise<number>;
    async easingValue(t: number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<number>;
    async easingValue(optionsOrT?: EasingValueOptions | number, easing?: Easing | null, cubicBezier?: number[] | null): Promise<number> {
        const options: EasingValueOptions = isEasingValueOptions(optionsOrT)
            ? optionsOrT
            : ({ t: optionsOrT, easing: easing, cubicBezier: cubicBezier } as EasingValueOptions);
        return (await this.api.easingValue(options)).value;
    }
}
