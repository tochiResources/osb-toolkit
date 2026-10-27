import { AudioAPI } from '../../api/AudioAPI';
import type { AudioDurationResult } from '../../models/AudioDurationResult';
import type { FftResult } from '../../models/FftResult';
import type { FftOptions, GetFftOptions } from '../options';

export class Audio {
    constructor(private readonly api: AudioAPI) {}

    static instanceOf(value: unknown): value is Audio {
        return value instanceof Audio;
    }

    async audioDuration(): Promise<AudioDurationResult> {
        return this.api.audioDuration();
    }

    async fft(options: FftOptions): Promise<FftResult> {
        return this.api.fft(options);
    }

    async getFft(options: GetFftOptions): Promise<FftResult> {
        return this.api.getFft(options);
    }
}
