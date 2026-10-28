import { ImageAPI } from '../../api/ImageAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { ImageOptions } from '../../models/ImageOptions';
import type { ImageProcessOptions, PathOptions } from '../options';
import { isImageProcessOptions, isPathOptions } from '../options';

/**
 * The clean `Image` client.
 */
export class Image {
    constructor(private readonly api: ImageAPI) {}

    static instanceOf(value: unknown): value is Image {
        return value instanceof Image;
    }

    /**
     * A request carrying a single file path.
     *
     * @param path The input file path.
     *
     * @returns The image's metadata
     */
    async info(path: string): Promise<ImageInfo>;
    /**
     * A request carrying a single file path.
     *
     * @param options A request carrying a single file path.
     *
     * @returns The image's metadata
     */
    async info(options: PathOptions): Promise<ImageInfo>;
    async info(optionsOrPath?: PathOptions | string): Promise<ImageInfo> {
        const options: PathOptions = isPathOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath } as PathOptions);
        return this.api.info(options);
    }

    /**
     * An image processing request: read `path`, apply `options`, write `output`.
     *
     * @param options The processing options.
     * @param output The output file path (its extension selects the format).
     * @param path The input file path.
     *
     * @returns The processed image's metadata
     */
    async process(options: ImageOptions, output: string, path: string): Promise<ImageInfo>;
    /**
     * An image processing request: read `path`, apply `options`, write `output`.
     *
     * @param options An image processing request: read `path`, apply `options`, write `output`.
     *
     * @returns The processed image's metadata
     */
    async process(options: ImageProcessOptions): Promise<ImageInfo>;
    async process(optionsOrOptions?: ImageProcessOptions | ImageOptions, output?: string, path?: string): Promise<ImageInfo> {
        const options: ImageProcessOptions = isImageProcessOptions(optionsOrOptions)
            ? optionsOrOptions
            : ({ options: optionsOrOptions, output: output, path: path } as ImageProcessOptions);
        return this.api.process(options);
    }
}
