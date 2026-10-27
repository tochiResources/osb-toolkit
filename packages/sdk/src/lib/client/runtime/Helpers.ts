import { HelpersAPI } from '../../api/HelpersAPI';
import type { HelperOptions } from '../options';
import { isHelperOptions } from '../options';

export class Helpers {
    constructor(private readonly api: HelpersAPI) {}

    static instanceOf(value: unknown): value is Helpers {
        return value instanceof Helpers;
    }

    async call(name: string): Promise<unknown>;
    async call(name: string, input: unknown): Promise<unknown>;
    async call(options: HelperOptions): Promise<unknown>;
    async call(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.call(options)).value;
    }

    async commands(name: string): Promise<unknown>;
    async commands(name: string, input: unknown): Promise<unknown>;
    async commands(options: HelperOptions): Promise<unknown>;
    async commands(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.commands(options)).value;
    }

    async keyframes(name: string): Promise<unknown>;
    async keyframes(name: string, input: unknown): Promise<unknown>;
    async keyframes(options: HelperOptions): Promise<unknown>;
    async keyframes(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.keyframes(options)).value;
    }
}
