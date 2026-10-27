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

    async beatmap(): Promise<Beatmap> {
        return this.api.beatmap();
    }

    async controlPointAt(time: number): Promise<ControlPoint>;
    async controlPointAt(options: TimeOptions): Promise<ControlPoint>;
    async controlPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.controlPointAt(options);
    }

    async importOsb(text: string): Promise<number>;
    async importOsb(options: TextOptions): Promise<number>;
    async importOsb(optionsOrText?: TextOptions | string): Promise<number> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return (await this.api.importOsb(options)).spriteCount;
    }

    async layerHandle(name: string): Promise<number>;
    async layerHandle(options: LayerHandleOptions): Promise<number>;
    async layerHandle(optionsOrName?: LayerHandleOptions | string): Promise<number> {
        const options: LayerHandleOptions = isLayerHandleOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName } as LayerHandleOptions);
        return (await this.api.layerHandle(options)).layer;
    }

    async loadBeatmap(text: string): Promise<LoadBeatmapResult>;
    async loadBeatmap(options: TextOptions): Promise<LoadBeatmapResult>;
    async loadBeatmap(optionsOrText?: TextOptions | string): Promise<LoadBeatmapResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.loadBeatmap(options);
    }

    async random(max: number, min: number): Promise<number>;
    async random(options: RandomOptions): Promise<number>;
    async random(optionsOrMax?: RandomOptions | number, min?: number): Promise<number> {
        const options: RandomOptions = isRandomOptions(optionsOrMax)
            ? optionsOrMax
            : ({ max: optionsOrMax, min: min } as RandomOptions);
        return (await this.api.random(options)).value;
    }

    async spriteCount(): Promise<number> {
        return (await this.api.spriteCount()).spriteCount;
    }

    async storyboard(): Promise<Storyboard> {
        return this.api.storyboard();
    }

    async timingPointAt(time: number): Promise<ControlPoint>;
    async timingPointAt(options: TimeOptions): Promise<ControlPoint>;
    async timingPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.timingPointAt(options);
    }
}
