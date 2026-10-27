import { MediaAPI } from '../../api/MediaAPI';
import type { EncodeVideoRequest } from '../../models/EncodeVideoRequest';
import type { ExtractAudioRequest } from '../../models/ExtractAudioRequest';
import type { ExtractFramesRequest } from '../../models/ExtractFramesRequest';
import type { FrameResult } from '../../models/FrameResult';
import type { MediaInfo } from '../../models/MediaInfo';
import type { OkResult } from '../../models/OkResult';
import type { PathRequest } from '../../models/PathRequest';

export class Media {
    constructor(private readonly api: MediaAPI) {}

    static instanceOf(value: unknown): value is Media {
        return value instanceof Media;
    }

    async encodeVideo(body: EncodeVideoRequest): Promise<MediaInfo> {
        return this.api.encodeVideo(body);
    }

    async extractAudio(body: ExtractAudioRequest): Promise<OkResult> {
        return this.api.extractAudio(body);
    }

    async extractFrames(body: ExtractFramesRequest): Promise<FrameResult> {
        return this.api.extractFrames(body);
    }

    async info(body: PathRequest): Promise<MediaInfo> {
        return this.api.info(body);
    }
}
