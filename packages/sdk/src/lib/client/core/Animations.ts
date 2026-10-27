import { AnimationsAPI } from '../../api/AnimationsAPI';
import type { EasingResult } from '../../models/EasingResult';
import type { ApplyEasingOptions, EasingValueOptions } from '../options';

export class Animations {
    constructor(private readonly api: AnimationsAPI) {}

    static instanceOf(value: unknown): value is Animations {
        return value instanceof Animations;
    }

    async applyEasing(options: ApplyEasingOptions): Promise<EasingResult> {
        return this.api.applyEasing(options);
    }

    async easingValue(options: EasingValueOptions): Promise<EasingResult> {
        return this.api.easingValue(options);
    }
}
