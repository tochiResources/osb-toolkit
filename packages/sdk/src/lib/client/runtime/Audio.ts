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

    /**
     * Calls the Audio.audioDuration bridge operation.
     *
     * @returns The loaded audio duration
     */
    async audioDuration(): Promise<number> {
        return (await this.api.audioDuration()).audioDuration;
    }

    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     *
     * @returns The FFT magnitudes
     */
    async fft(path: string): Promise<FftResult>;
    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     * @param magnitudes Number of frequency bins.
     *
     * @returns The FFT magnitudes
     */
    async fft(path: string, magnitudes: number | null): Promise<FftResult>;
    /**
     * An FFT request against a `.wav` file.
     *
     * @param path Path to the audio file.
     * @param magnitudes Number of frequency bins.
     * @param time Sample time in milliseconds.
     *
     * @returns The FFT magnitudes
     */
    async fft(path: string, magnitudes: number | null, time: number | null): Promise<FftResult>;
    /**
     * An FFT request against a `.wav` file.
     *
     * @param options An FFT request against a `.wav` file.
     *
     * @returns The FFT magnitudes
     */
    async fft(options: FftOptions): Promise<FftResult>;
    async fft(optionsOrPath?: FftOptions | string, magnitudes?: number | null, time?: number | null): Promise<FftResult> {
        const options: FftOptions = isFftOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, magnitudes: magnitudes, time: time } as FftOptions);
        return this.api.fft(options);
    }

    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     *
     * @returns The FFT magnitudes
     */
    async getFft(time: number, magnitudes: number): Promise<FftResult>;
    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     * @param cutoff A frequency cut-off.
     *
     * @returns The FFT magnitudes
     */
    async getFft(time: number, magnitudes: number, cutoff: number | null): Promise<FftResult>;
    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param time Sample time in milliseconds.
     * @param magnitudes Number of frequency bins.
     * @param cutoff A frequency cut-off.
     * @param easing The easing applied across the bins.
     *
     * @returns The FFT magnitudes
     */
    async getFft(time: number, magnitudes: number, cutoff: number | null, easing: Easing | null): Promise<FftResult>;
    /**
     * An FFT request against the runtime's loaded audio.
     *
     * @param options An FFT request against the runtime's loaded audio.
     *
     * @returns The FFT magnitudes
     */
    async getFft(options: GetFftOptions): Promise<FftResult>;
    async getFft(optionsOrTime?: GetFftOptions | number, magnitudes?: number, cutoff?: number | null, easing?: Easing | null): Promise<FftResult> {
        const options: GetFftOptions = isGetFftOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime, magnitudes: magnitudes, cutoff: cutoff, easing: easing } as GetFftOptions);
        return this.api.getFft(options);
    }
}
