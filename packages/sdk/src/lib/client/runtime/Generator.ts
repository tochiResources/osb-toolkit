import { GeneratorAPI } from '../../api/GeneratorAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ControlPoint } from '../../models/ControlPoint';
import type { LoadBeatmapResult } from '../../models/LoadBeatmapResult';
import type { Storyboard } from '../../models/Storyboard';
import type { LayerHandleOptions, RandomOptions, TextOptions, TimeOptions } from '../options';
import { isLayerHandleOptions, isRandomOptions, isTextOptions, isTimeOptions } from '../options';

export class Generator {
    constructor(private readonly api: GeneratorAPI) {}

    static instanceOf(value: unknown): value is Generator {
        return value instanceof Generator;
    }

    /**
     * Calls the Generator.beatmap bridge operation.
     *
     * @returns The loaded beatmap, or null
     */
    async beatmap(): Promise<Beatmap> {
        return this.api.beatmap();
    }

    /**
     * A time query.
     *
     * @param time The time in milliseconds.
     *
     * @returns The active control point, or null
     */
    async controlPointAt(time: number): Promise<ControlPoint>;
    /**
     * A time query.
     *
     * @param options A time query.
     *
     * @returns The active control point, or null
     */
    async controlPointAt(options: TimeOptions): Promise<ControlPoint>;
    async controlPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.controlPointAt(options);
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns The sprite count after import
     */
    async importOsb(text: string): Promise<number>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns The sprite count after import
     */
    async importOsb(options: TextOptions): Promise<number>;
    async importOsb(optionsOrText?: TextOptions | string): Promise<number> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return (await this.api.importOsb(options)).spriteCount;
    }

    /**
     * Resolve (or create) a layer by name.
     *
     * @param name The layer name.
     *
     * @returns The layer handle
     */
    async layerHandle(name: string): Promise<number>;
    /**
     * Resolve (or create) a layer by name.
     *
     * @param options Resolve (or create) a layer by name.
     *
     * @returns The layer handle
     */
    async layerHandle(options: LayerHandleOptions): Promise<number>;
    async layerHandle(optionsOrName?: LayerHandleOptions | string): Promise<number> {
        const options: LayerHandleOptions = isLayerHandleOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName } as LayerHandleOptions);
        return (await this.api.layerHandle(options)).layer;
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @returns A summary of the loaded beatmap
     */
    async loadBeatmap(text: string): Promise<LoadBeatmapResult>;
    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @returns A summary of the loaded beatmap
     */
    async loadBeatmap(options: TextOptions): Promise<LoadBeatmapResult>;
    async loadBeatmap(optionsOrText?: TextOptions | string): Promise<LoadBeatmapResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.loadBeatmap(options);
    }

    /**
     * A deterministic random request.
     *
     * @param max The exclusive upper bound.
     * @param min The inclusive lower bound.
     *
     * @returns A deterministic random value
     */
    async random(max: number, min: number): Promise<number>;
    /**
     * A deterministic random request.
     *
     * @param options A deterministic random request.
     *
     * @returns A deterministic random value
     */
    async random(options: RandomOptions): Promise<number>;
    async random(optionsOrMax?: RandomOptions | number, min?: number): Promise<number> {
        const options: RandomOptions = isRandomOptions(optionsOrMax)
            ? optionsOrMax
            : ({ max: optionsOrMax, min: min } as RandomOptions);
        return (await this.api.random(options)).value;
    }

    /**
     * Calls the Generator.spriteCount bridge operation.
     *
     * @returns The storyboard sprite count
     */
    async spriteCount(): Promise<number> {
        return (await this.api.spriteCount()).spriteCount;
    }

    /**
     * Calls the Generator.storyboard bridge operation.
     *
     * @returns The built storyboard model
     */
    async storyboard(): Promise<Storyboard> {
        return this.api.storyboard();
    }

    /**
     * A time query.
     *
     * @param time The time in milliseconds.
     *
     * @returns The active red timing point
     */
    async timingPointAt(time: number): Promise<ControlPoint>;
    /**
     * A time query.
     *
     * @param options A time query.
     *
     * @returns The active red timing point
     */
    async timingPointAt(options: TimeOptions): Promise<ControlPoint>;
    async timingPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.timingPointAt(options);
    }
}
