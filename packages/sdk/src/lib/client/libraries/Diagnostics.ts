import { DiagnosticsAPI } from '../../api/DiagnosticsAPI';
import type { DiagnosticReport } from '../../models/DiagnosticReport';
import type { TextOptions } from '../options';
import { isTextOptions } from '../options';

export class Diagnostics {
    constructor(private readonly api: DiagnosticsAPI) {}

    static instanceOf(value: unknown): value is Diagnostics {
        return value instanceof Diagnostics;
    }

    async diagnoseOsb(text: string): Promise<DiagnosticReport>;
    async diagnoseOsb(options: TextOptions): Promise<DiagnosticReport>;
    async diagnoseOsb(optionsOrText?: TextOptions | string): Promise<DiagnosticReport> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.diagnoseOsb(options);
    }

    async diagnoseOsu(text: string): Promise<DiagnosticReport>;
    async diagnoseOsu(options: TextOptions): Promise<DiagnosticReport>;
    async diagnoseOsu(optionsOrText?: TextOptions | string): Promise<DiagnosticReport> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.diagnoseOsu(options);
    }
}
