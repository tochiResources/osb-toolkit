import { CommonAPI } from '../../api/CommonAPI';
import type { VersionResult } from '../../models/VersionResult';

export class Common {
    constructor(private readonly api: CommonAPI) {}

    static instanceOf(value: unknown): value is Common {
        return value instanceof Common;
    }

    async version(): Promise<VersionResult> {
        return this.api.version();
    }
}
