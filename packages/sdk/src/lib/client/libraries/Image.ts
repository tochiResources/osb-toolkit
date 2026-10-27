import { ImageAPI } from '../../api/ImageAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { ImageProcessOptions, PathOptions } from '../options';

export class Image {
    constructor(private readonly api: ImageAPI) {}

    static instanceOf(value: unknown): value is Image {
        return value instanceof Image;
    }

    async info(options: PathOptions): Promise<ImageInfo> {
        return this.api.info(options);
    }

    async process(options: ImageProcessOptions): Promise<ImageInfo> {
        return this.api.process(options);
    }
}
