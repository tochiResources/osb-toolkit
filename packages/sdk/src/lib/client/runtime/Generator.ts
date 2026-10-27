import { GeneratorAPI } from '../../api/GeneratorAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ControlPoint } from '../../models/ControlPoint';
import type { ImportOsbResult } from '../../models/ImportOsbResult';
import type { LayerHandle } from '../../models/LayerHandle';
import type { LoadBeatmapResult } from '../../models/LoadBeatmapResult';
import type { RandomResult } from '../../models/RandomResult';
import type { SpriteCountResult } from '../../models/SpriteCountResult';
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

    async controlPointAt(options: TimeOptions): Promise<ControlPoint>;
    async controlPointAt(time: number): Promise<ControlPoint>;
    async controlPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.controlPointAt(options);
    }

    async importOsb(options: TextOptions): Promise<ImportOsbResult>;
    async importOsb(text: string): Promise<ImportOsbResult>;
    async importOsb(optionsOrText?: TextOptions | string): Promise<ImportOsbResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.importOsb(options);
    }

    async layerHandle(options: LayerHandleOptions): Promise<LayerHandle>;
    async layerHandle(name: string): Promise<LayerHandle>;
    async layerHandle(optionsOrName?: LayerHandleOptions | string): Promise<LayerHandle> {
        const options: LayerHandleOptions = isLayerHandleOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName } as LayerHandleOptions);
        return this.api.layerHandle(options);
    }

    async loadBeatmap(options: TextOptions): Promise<LoadBeatmapResult>;
    async loadBeatmap(text: string): Promise<LoadBeatmapResult>;
    async loadBeatmap(optionsOrText?: TextOptions | string): Promise<LoadBeatmapResult> {
        const options: TextOptions = isTextOptions(optionsOrText)
            ? optionsOrText
            : ({ text: optionsOrText } as TextOptions);
        return this.api.loadBeatmap(options);
    }

    async random(options: RandomOptions): Promise<RandomResult>;
    async random(max: number, min: number): Promise<RandomResult>;
    async random(optionsOrMax?: RandomOptions | number, min?: number): Promise<RandomResult> {
        const options: RandomOptions = isRandomOptions(optionsOrMax)
            ? optionsOrMax
            : ({ max: optionsOrMax, min: min } as RandomOptions);
        return this.api.random(options);
    }

    async spriteCount(): Promise<SpriteCountResult> {
        return this.api.spriteCount();
    }

    async storyboard(): Promise<Storyboard> {
        return this.api.storyboard();
    }

    async timingPointAt(options: TimeOptions): Promise<ControlPoint>;
    async timingPointAt(time: number): Promise<ControlPoint>;
    async timingPointAt(optionsOrTime?: TimeOptions | number): Promise<ControlPoint> {
        const options: TimeOptions = isTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as TimeOptions);
        return this.api.timingPointAt(options);
    }
}
