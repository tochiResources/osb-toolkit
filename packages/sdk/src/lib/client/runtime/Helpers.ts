import { HelpersAPI } from '../../api/HelpersAPI';
import type { HelperResult } from '../../models/HelperResult';
import type { HelperOptions } from '../options';
import { isHelperOptions } from '../options';

export class Helpers {
    constructor(private readonly api: HelpersAPI) {}

    static instanceOf(value: unknown): value is Helpers {
        return value instanceof Helpers;
    }

    async call(options: HelperOptions): Promise<HelperResult>;
    async call(name: string, input?: unknown): Promise<HelperResult>;
    async call(optionsOrName?: HelperOptions | string, input?: unknown): Promise<HelperResult> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return this.api.call(options);
    }

    async commands(options: HelperOptions): Promise<HelperResult>;
    async commands(name: string, input?: unknown): Promise<HelperResult>;
    async commands(optionsOrName?: HelperOptions | string, input?: unknown): Promise<HelperResult> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return this.api.commands(options);
    }

    async keyframes(options: HelperOptions): Promise<HelperResult>;
    async keyframes(name: string, input?: unknown): Promise<HelperResult>;
    async keyframes(optionsOrName?: HelperOptions | string, input?: unknown): Promise<HelperResult> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return this.api.keyframes(options);
    }
}
