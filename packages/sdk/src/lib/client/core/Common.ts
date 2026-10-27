import { CommonAPI } from '../../api/CommonAPI';

export class Common {
    constructor(private readonly api: CommonAPI) {}

    static instanceOf(value: unknown): value is Common {
        return value instanceof Common;
    }

    async version(): Promise<string> {
        return (await this.api.version()).version;
    }
}
