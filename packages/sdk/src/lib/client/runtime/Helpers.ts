import { HelpersAPI } from '../../api/HelpersAPI';
import type { HelperOptions } from '../options';
import { isHelperOptions } from '../options';

export class Helpers {
    constructor(private readonly api: HelpersAPI) {}

    static instanceOf(value: unknown): value is Helpers {
        return value instanceof Helpers;
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @returns The shared motion-graphics helper's result
     */
    async call(name: string): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @returns The shared motion-graphics helper's result
     */
    async call(name: string, input: unknown): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @returns The shared motion-graphics helper's result
     */
    async call(options: HelperOptions): Promise<unknown>;
    async call(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.call(options)).value;
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @returns The helper's result (a command list or scalar)
     */
    async commands(name: string): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @returns The helper's result (a command list or scalar)
     */
    async commands(name: string, input: unknown): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @returns The helper's result (a command list or scalar)
     */
    async commands(options: HelperOptions): Promise<unknown>;
    async commands(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.commands(options)).value;
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @returns The helper's result (a keyframe channel or vectors)
     */
    async keyframes(name: string): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @returns The helper's result (a keyframe channel or vectors)
     */
    async keyframes(name: string, input: unknown): Promise<unknown>;
    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @returns The helper's result (a keyframe channel or vectors)
     */
    async keyframes(options: HelperOptions): Promise<unknown>;
    async keyframes(optionsOrName?: HelperOptions | string, input?: unknown): Promise<unknown> {
        const options: HelperOptions = isHelperOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, input: input } as HelperOptions);
        return (await this.api.keyframes(options)).value;
    }
}
