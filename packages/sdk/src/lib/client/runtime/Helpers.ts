import { HelpersAPI } from '../../api/HelpersAPI';
import type { HelperResult } from '../../models/HelperResult';
import type { HelperOptions } from '../options';

export class Helpers {
    constructor(private readonly api: HelpersAPI) {}

    static instanceOf(value: unknown): value is Helpers {
        return value instanceof Helpers;
    }

    async call(options: HelperOptions): Promise<HelperResult> {
        return this.api.call(options);
    }

    async commands(options: HelperOptions): Promise<HelperResult> {
        return this.api.commands(options);
    }

    async keyframes(options: HelperOptions): Promise<HelperResult> {
        return this.api.keyframes(options);
    }
}
