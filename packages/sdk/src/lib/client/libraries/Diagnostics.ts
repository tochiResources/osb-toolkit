import { DiagnosticsAPI } from '../../api/DiagnosticsAPI';
import type { DiagnosticReport } from '../../models/DiagnosticReport';
import type { TextRequest } from '../../models/TextRequest';

export class Diagnostics {
    constructor(private readonly api: DiagnosticsAPI) {}

    static instanceOf(value: unknown): value is Diagnostics {
        return value instanceof Diagnostics;
    }

    async diagnoseOsb(body: TextRequest): Promise<DiagnosticReport> {
        return this.api.diagnoseOsb(body);
    }

    async diagnoseOsu(body: TextRequest): Promise<DiagnosticReport> {
        return this.api.diagnoseOsu(body);
    }
}
