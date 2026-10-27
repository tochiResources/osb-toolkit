import { ImageAPI } from '../../api/ImageAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { ImageProcessRequest } from '../../models/ImageProcessRequest';
import type { PathRequest } from '../../models/PathRequest';

export class Image {
    constructor(private readonly api: ImageAPI) {}

    static instanceOf(value: unknown): value is Image {
        return value instanceof Image;
    }

    async info(body: PathRequest): Promise<ImageInfo> {
        return this.api.info(body);
    }

    async process(body: ImageProcessRequest): Promise<ImageInfo> {
        return this.api.process(body);
    }
}
