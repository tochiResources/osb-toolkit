import { OptimizeAPI } from '../../api/OptimizeAPI';
import type { OptimizeOptionsInput } from '../../models/OptimizeOptionsInput';
import type { OptimizeResult } from '../../models/OptimizeResult';
import type { OptimizeOptions } from '../options';
import { isOptimizeOptions } from '../options';

/**
 * The clean `Optimize` client.
 */
export class Optimize {
    constructor(private readonly api: OptimizeAPI) {}

    static instanceOf(value: unknown): value is Optimize {
        return value instanceof Optimize;
    }

    /**
     * An `.osb` optimization request.
     *
     * @param options The optimization options.
     * @param text The `.osb` storyboard text.
     *
     * @returns The optimized `.osb` text and stats
     */
    async optimizeOsb(options: OptimizeOptionsInput, text: string): Promise<OptimizeResult>;
    /**
     * An `.osb` optimization request.
     *
     * @param options An `.osb` optimization request.
     *
     * @returns The optimized `.osb` text and stats
     */
    async optimizeOsb(options: OptimizeOptions): Promise<OptimizeResult>;
    async optimizeOsb(optionsOrOptions?: OptimizeOptions | OptimizeOptionsInput, text?: string): Promise<OptimizeResult> {
        const options: OptimizeOptions = isOptimizeOptions(optionsOrOptions)
            ? optionsOrOptions
            : ({ options: optionsOrOptions, text: text } as OptimizeOptions);
        return this.api.optimizeOsb(options);
    }
}
