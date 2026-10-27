import { MediaAPI } from '../../api/MediaAPI';
import type { FrameResult } from '../../models/FrameResult';
import type { MediaInfo } from '../../models/MediaInfo';
import type { OkResult } from '../../models/OkResult';
import type { EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions } from '../options';

export class Media {
    constructor(private readonly api: MediaAPI) {}

    static instanceOf(value: unknown): value is Media {
        return value instanceof Media;
    }

    async encodeVideo(options: EncodeVideoOptions): Promise<MediaInfo> {
        return this.api.encodeVideo(options);
    }

    async extractAudio(options: ExtractAudioOptions): Promise<OkResult> {
        return this.api.extractAudio(options);
    }

    async extractFrames(options: ExtractFramesOptions): Promise<FrameResult> {
        return this.api.extractFrames(options);
    }

    async info(options: PathOptions): Promise<MediaInfo> {
        return this.api.info(options);
    }
}
