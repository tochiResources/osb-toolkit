import { MediaAPI } from '../../api/MediaAPI';
import type { FrameResult } from '../../models/FrameResult';
import type { MediaInfo } from '../../models/MediaInfo';
import type { EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions } from '../options';
import { isEncodeVideoOptions, isExtractAudioOptions, isExtractFramesOptions, isPathOptions } from '../options';

export class Media {
    constructor(private readonly api: MediaAPI) {}

    static instanceOf(value: unknown): value is Media {
        return value instanceof Media;
    }

    async encodeVideo(inputPattern: string, output: string, fps: number): Promise<MediaInfo>;
    async encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null): Promise<MediaInfo>;
    async encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null, crf: number | null): Promise<MediaInfo>;
    async encodeVideo(options: EncodeVideoOptions): Promise<MediaInfo>;
    async encodeVideo(optionsOrInputPattern?: EncodeVideoOptions | string, output?: string, fps?: number, codec?: string | null, crf?: number | null): Promise<MediaInfo> {
        const options: EncodeVideoOptions = isEncodeVideoOptions(optionsOrInputPattern)
            ? optionsOrInputPattern
            : ({ inputPattern: optionsOrInputPattern, output: output, fps: fps, codec: codec, crf: crf } as EncodeVideoOptions);
        return this.api.encodeVideo(options);
    }

    async extractAudio(input: string, output: string): Promise<void>;
    async extractAudio(options: ExtractAudioOptions): Promise<void>;
    async extractAudio(optionsOrInput?: ExtractAudioOptions | string, output?: string): Promise<void> {
        const options: ExtractAudioOptions = isExtractAudioOptions(optionsOrInput)
            ? optionsOrInput
            : ({ input: optionsOrInput, output: output } as ExtractAudioOptions);
        await this.api.extractAudio(options);
    }

    async extractFrames(input: string, outputDir: string): Promise<FrameResult>;
    async extractFrames(input: string, outputDir: string, count: number | null): Promise<FrameResult>;
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null): Promise<FrameResult>;
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null): Promise<FrameResult>;
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null): Promise<FrameResult>;
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null, width: number | null): Promise<FrameResult>;
    async extractFrames(options: ExtractFramesOptions): Promise<FrameResult>;
    async extractFrames(optionsOrInput?: ExtractFramesOptions | string, outputDir?: string, count?: number | null, fps?: number | null, height?: number | null, start?: number | null, width?: number | null): Promise<FrameResult> {
        const options: ExtractFramesOptions = isExtractFramesOptions(optionsOrInput)
            ? optionsOrInput
            : ({ input: optionsOrInput, outputDir: outputDir, count: count, fps: fps, height: height, start: start, width: width } as ExtractFramesOptions);
        return this.api.extractFrames(options);
    }

    async info(path: string): Promise<MediaInfo>;
    async info(options: PathOptions): Promise<MediaInfo>;
    async info(optionsOrPath?: PathOptions | string): Promise<MediaInfo> {
        const options: PathOptions = isPathOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath } as PathOptions);
        return this.api.info(options);
    }
}
