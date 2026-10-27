import { HelpersAPI } from '../../api/HelpersAPI';
import type { HelperRequest } from '../../models/HelperRequest';
import type { HelperResult } from '../../models/HelperResult';

export class Helpers {
    constructor(private readonly api: HelpersAPI) {}

    static instanceOf(value: unknown): value is Helpers {
        return value instanceof Helpers;
    }

    async call(body: HelperRequest): Promise<HelperResult> {
        return this.api.call(body);
    }

    async commands(body: HelperRequest): Promise<HelperResult> {
        return this.api.commands(body);
    }

    async keyframes(body: HelperRequest): Promise<HelperResult> {
        return this.api.keyframes(body);
    }
}
