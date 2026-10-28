import { MediaAPI } from '../../api/MediaAPI';
import type { FrameResult } from '../../models/FrameResult';
import type { MediaInfo } from '../../models/MediaInfo';
import type { EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions } from '../options';
import { isEncodeVideoOptions, isExtractAudioOptions, isExtractFramesOptions, isPathOptions } from '../options';

/**
 * The clean `Media` client.
 */
export class Media {
    constructor(private readonly api: MediaAPI) {}

    static instanceOf(value: unknown): value is Media {
        return value instanceof Media;
    }

    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     *
     * @returns The encoded video's metadata
     */
    async encodeVideo(inputPattern: string, output: string, fps: number): Promise<MediaInfo>;
    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     * @param codec The video codec (default `libx264`).
     *
     * @returns The encoded video's metadata
     */
    async encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null): Promise<MediaInfo>;
    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param inputPattern The input filename pattern (e.g. `frames/frame_%04d.png`).
     * @param output The output video path.
     * @param fps The frame rate.
     * @param codec The video codec (default `libx264`).
     * @param crf The constant-rate-factor quality (`0`-`51`; lower is better).
     *
     * @returns The encoded video's metadata
     */
    async encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null, crf: number | null): Promise<MediaInfo>;
    /**
     * An image-sequence encoding request (frames -> video).
     *
     * @param options An image-sequence encoding request (frames -> video).
     *
     * @returns The encoded video's metadata
     */
    async encodeVideo(options: EncodeVideoOptions): Promise<MediaInfo>;
    async encodeVideo(optionsOrInputPattern?: EncodeVideoOptions | string, output?: string, fps?: number, codec?: string | null, crf?: number | null): Promise<MediaInfo> {
        const options: EncodeVideoOptions = isEncodeVideoOptions(optionsOrInputPattern)
            ? optionsOrInputPattern
            : ({ inputPattern: optionsOrInputPattern, output: output, fps: fps, codec: codec, crf: crf } as EncodeVideoOptions);
        return this.api.encodeVideo(options);
    }

    /**
     * An audio-extraction request (media -> audio file).
     *
     * @param input The input media path.
     * @param output The output audio path (its extension selects the format).
     *
     * @returns Acknowledged
     */
    async extractAudio(input: string, output: string): Promise<void>;
    /**
     * An audio-extraction request (media -> audio file).
     *
     * @param options An audio-extraction request (media -> audio file).
     *
     * @returns Acknowledged
     */
    async extractAudio(options: ExtractAudioOptions): Promise<void>;
    async extractAudio(optionsOrInput?: ExtractAudioOptions | string, output?: string): Promise<void> {
        const options: ExtractAudioOptions = isExtractAudioOptions(optionsOrInput)
            ? optionsOrInput
            : ({ input: optionsOrInput, output: output } as ExtractAudioOptions);
        await this.api.extractAudio(options);
    }

    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string, count: number | null): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     * @param start The start time in seconds.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param input The input media path.
     * @param outputDir The directory to write frames into.
     * @param count The maximum number of frames to extract.
     * @param fps The sampling rate (frames per second); defaults to the source rate.
     * @param height Scale frames to this height.
     * @param start The start time in seconds.
     * @param width Scale frames to this width.
     *
     * @returns The extracted frame count
     */
    async extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null, width: number | null): Promise<FrameResult>;
    /**
     * A frame-extraction request (video -> image sequence).
     *
     * @param options A frame-extraction request (video -> image sequence).
     *
     * @returns The extracted frame count
     */
    async extractFrames(options: ExtractFramesOptions): Promise<FrameResult>;
    async extractFrames(optionsOrInput?: ExtractFramesOptions | string, outputDir?: string, count?: number | null, fps?: number | null, height?: number | null, start?: number | null, width?: number | null): Promise<FrameResult> {
        const options: ExtractFramesOptions = isExtractFramesOptions(optionsOrInput)
            ? optionsOrInput
            : ({ input: optionsOrInput, outputDir: outputDir, count: count, fps: fps, height: height, start: start, width: width } as ExtractFramesOptions);
        return this.api.extractFrames(options);
    }

    /**
     * A request carrying a single file path.
     *
     * @param path The input file path.
     *
     * @returns The media metadata
     */
    async info(path: string): Promise<MediaInfo>;
    /**
     * A request carrying a single file path.
     *
     * @param options A request carrying a single file path.
     *
     * @returns The media metadata
     */
    async info(options: PathOptions): Promise<MediaInfo>;
    async info(optionsOrPath?: PathOptions | string): Promise<MediaInfo> {
        const options: PathOptions = isPathOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath } as PathOptions);
        return this.api.info(options);
    }
}
