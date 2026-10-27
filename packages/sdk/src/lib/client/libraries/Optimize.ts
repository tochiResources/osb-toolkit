import { OptimizeAPI } from '../../api/OptimizeAPI';
import type { OptimizeRequest } from '../../models/OptimizeRequest';
import type { OptimizeResult } from '../../models/OptimizeResult';

export class Optimize {
    constructor(private readonly api: OptimizeAPI) {}

    static instanceOf(value: unknown): value is Optimize {
        return value instanceof Optimize;
    }

    async optimizeOsb(body: OptimizeRequest): Promise<OptimizeResult> {
        return this.api.optimizeOsb(body);
    }
}
