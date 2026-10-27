import { DiagnosticsAPI } from '../../api/DiagnosticsAPI';
import type { DiagnosticReport } from '../../models/DiagnosticReport';
import type { TextOptions } from '../options';

export class Diagnostics {
    constructor(private readonly api: DiagnosticsAPI) {}

    static instanceOf(value: unknown): value is Diagnostics {
        return value instanceof Diagnostics;
    }

    async diagnoseOsb(options: TextOptions): Promise<DiagnosticReport> {
        return this.api.diagnoseOsb(options);
    }

    async diagnoseOsu(options: TextOptions): Promise<DiagnosticReport> {
        return this.api.diagnoseOsu(options);
    }
}
