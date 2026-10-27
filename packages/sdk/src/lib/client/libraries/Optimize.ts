import { OptimizeAPI } from '../../api/OptimizeAPI';
import type { OptimizeResult } from '../../models/OptimizeResult';
import type { OptimizeOptions } from '../options';

export class Optimize {
    constructor(private readonly api: OptimizeAPI) {}

    static instanceOf(value: unknown): value is Optimize {
        return value instanceof Optimize;
    }

    async optimizeOsb(options: OptimizeOptions): Promise<OptimizeResult> {
        return this.api.optimizeOsb(options);
    }
}
