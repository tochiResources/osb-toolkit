import { DiagnosticsAPI } from '../../api/DiagnosticsAPI';
import type { DiagnosticReport } from '../../models/DiagnosticReport';
import type { TextOptions } from '../options';
import { isTextOptions } from '../options';

/**
 * The clean `Diagnostics` client.
 */
export class Diagnostics {
    constructor(private readonly api: DiagnosticsAPI) {}

    static instanceOf(value: unknown): value is Diagnostics {
        return value instanceof Diagnostics;
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns Diagnostics for an `.osb` storyboard
     */
    async diagnoseOsb(text: string): Promise<DiagnosticReport>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns Diagnostics for an `.osb` storyboard
     */
    async diagnoseOsb(options: TextOptions): Promise<DiagnosticReport>;
    async diagnoseOsb(optionsOrText?: TextOptions | string): Promise<DiagnosticReport> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.diagnoseOsb(options);
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns Diagnostics for an `.osu` beatmap
     */
    async diagnoseOsu(text: string): Promise<DiagnosticReport>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns Diagnostics for an `.osu` beatmap
     */
    async diagnoseOsu(options: TextOptions): Promise<DiagnosticReport>;
    async diagnoseOsu(optionsOrText?: TextOptions | string): Promise<DiagnosticReport> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.diagnoseOsu(options);
    }
}
