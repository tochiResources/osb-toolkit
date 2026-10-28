import { ParsingAPI } from '../../api/ParsingAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ParseOsbResult } from '../../models/ParseOsbResult';
import type { TextOptions } from '../options';
import { isTextOptions } from '../options';

export class Parsing {
    constructor(private readonly api: ParsingAPI) {}

    static instanceOf(value: unknown): value is Parsing {
        return value instanceof Parsing;
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns The storyboard summary
     */
    async parseOsb(text: string): Promise<ParseOsbResult>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns The storyboard summary
     */
    async parseOsb(options: TextOptions): Promise<ParseOsbResult>;
    async parseOsb(optionsOrText?: TextOptions | string): Promise<ParseOsbResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsb(options);
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns The parsed beatmap
     */
    async parseOsu(text: string): Promise<Beatmap>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns The parsed beatmap
     */
    async parseOsu(options: TextOptions): Promise<Beatmap>;
    async parseOsu(optionsOrText?: TextOptions | string): Promise<Beatmap> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsu(options);
    }
}
