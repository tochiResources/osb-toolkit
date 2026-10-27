import { AnimationsAPI } from '../../api/AnimationsAPI';
import type { ApplyEasingRequest } from '../../models/ApplyEasingRequest';
import type { EasingResult } from '../../models/EasingResult';
import type { EasingValueRequest } from '../../models/EasingValueRequest';

export class Animations {
    constructor(private readonly api: AnimationsAPI) {}

    static instanceOf(value: unknown): value is Animations {
        return value instanceof Animations;
    }

    async applyEasing(body: ApplyEasingRequest): Promise<EasingResult> {
        return this.api.applyEasing(body);
    }

    async easingValue(body: EasingValueRequest): Promise<EasingResult> {
        return this.api.easingValue(body);
    }
}
