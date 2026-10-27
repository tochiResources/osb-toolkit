import { ImageAPI } from '../../api/ImageAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { ImageOptions } from '../../models/ImageOptions';
import type { ImageProcessOptions, PathOptions } from '../options';
import { isImageProcessOptions, isPathOptions } from '../options';

export class Image {
    constructor(private readonly api: ImageAPI) {}

    static instanceOf(value: unknown): value is Image {
        return value instanceof Image;
    }

    async info(options: PathOptions): Promise<ImageInfo>;
    async info(path: string): Promise<ImageInfo>;
    async info(optionsOrPath?: PathOptions | string): Promise<ImageInfo> {
        const options: PathOptions = isPathOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath } as PathOptions);
        return this.api.info(options);
    }

    async process(options: ImageProcessOptions): Promise<ImageInfo>;
    async process(options: ImageOptions, output: string, path: string): Promise<ImageInfo>;
    async process(optionsOrOptions?: ImageProcessOptions | ImageOptions, output?: string, path?: string): Promise<ImageInfo> {
        const options: ImageProcessOptions = isImageProcessOptions(optionsOrOptions)
            ? optionsOrOptions
            : ({ options: optionsOrOptions, output: output, path: path } as ImageProcessOptions);
        return this.api.process(options);
    }
}
