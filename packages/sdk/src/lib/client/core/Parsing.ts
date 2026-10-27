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

    async parseOsb(options: TextOptions): Promise<ParseOsbResult>;
    async parseOsb(text: string): Promise<ParseOsbResult>;
    async parseOsb(optionsOrText?: TextOptions | string): Promise<ParseOsbResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsb(options);
    }

    async parseOsu(options: TextOptions): Promise<Beatmap>;
    async parseOsu(text: string): Promise<Beatmap>;
    async parseOsu(optionsOrText?: TextOptions | string): Promise<Beatmap> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.parseOsu(options);
    }
}
