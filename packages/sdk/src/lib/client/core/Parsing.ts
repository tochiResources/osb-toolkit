import { ParsingAPI } from '../../api/ParsingAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ParseOsbResult } from '../../models/ParseOsbResult';
import type { TextRequest } from '../../models/TextRequest';

export class Parsing {
    constructor(private readonly api: ParsingAPI) {}

    static instanceOf(value: unknown): value is Parsing {
        return value instanceof Parsing;
    }

    async parseOsb(body: TextRequest): Promise<ParseOsbResult> {
        return this.api.parseOsb(body);
    }

    async parseOsu(body: TextRequest): Promise<Beatmap> {
        return this.api.parseOsu(body);
    }
}
