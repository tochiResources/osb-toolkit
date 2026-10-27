import { ParsingAPI } from '../../api/ParsingAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ParseOsbResult } from '../../models/ParseOsbResult';
import type { TextOptions } from '../options';

export class Parsing {
    constructor(private readonly api: ParsingAPI) {}

    static instanceOf(value: unknown): value is Parsing {
        return value instanceof Parsing;
    }

    async parseOsb(options: TextOptions): Promise<ParseOsbResult> {
        return this.api.parseOsb(options);
    }

    async parseOsu(options: TextOptions): Promise<Beatmap> {
        return this.api.parseOsu(options);
    }
}
