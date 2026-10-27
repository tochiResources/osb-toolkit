import { AudioAPI } from '../../api/AudioAPI';
import type { Easing } from '../../models/Easing';
import type { FftResult } from '../../models/FftResult';
import type { FftOptions, GetFftOptions } from '../options';
import { isFftOptions, isGetFftOptions } from '../options';

export class Audio {
    constructor(private readonly api: AudioAPI) {}

    static instanceOf(value: unknown): value is Audio {
        return value instanceof Audio;
    }

    async audioDuration(): Promise<number> {
        return (await this.api.audioDuration()).audioDuration;
    }

    async fft(options: FftOptions): Promise<FftResult>;
    async fft(path: string, magnitudes?: number | null, time?: number | null): Promise<FftResult>;
    async fft(optionsOrPath?: FftOptions | string, magnitudes?: number | null, time?: number | null): Promise<FftResult> {
        const options: FftOptions = isFftOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, magnitudes: magnitudes, time: time } as FftOptions);
        return this.api.fft(options);
    }

    async getFft(options: GetFftOptions): Promise<FftResult>;
    async getFft(time: number, magnitudes: number, cutoff?: number | null, easing?: Easing | null): Promise<FftResult>;
    async getFft(optionsOrTime?: GetFftOptions | number, magnitudes?: number, cutoff?: number | null, easing?: Easing | null): Promise<FftResult> {
        const options: GetFftOptions = isGetFftOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime, magnitudes: magnitudes, cutoff: cutoff, easing: easing } as GetFftOptions);
        return this.api.getFft(options);
    }
}
