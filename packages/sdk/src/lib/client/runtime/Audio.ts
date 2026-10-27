import { AudioAPI } from '../../api/AudioAPI';
import type { AudioDurationResult } from '../../models/AudioDurationResult';
import type { FftRequest } from '../../models/FftRequest';
import type { FftResult } from '../../models/FftResult';
import type { GetFftRequest } from '../../models/GetFftRequest';

export class Audio {
    constructor(private readonly api: AudioAPI) {}

    static instanceOf(value: unknown): value is Audio {
        return value instanceof Audio;
    }

    async audioDuration(): Promise<AudioDurationResult> {
        return this.api.audioDuration();
    }

    async fft(body: FftRequest): Promise<FftResult> {
        return this.api.fft(body);
    }

    async getFft(body: GetFftRequest): Promise<FftResult> {
        return this.api.getFft(body);
    }
}
