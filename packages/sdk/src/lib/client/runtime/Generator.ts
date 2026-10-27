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

export class Generator {
    constructor(private readonly api: GeneratorAPI) {}

    static instanceOf(value: unknown): value is Generator {
        return value instanceof Generator;
    }

    async beatmap(): Promise<Beatmap> {
        return this.api.beatmap();
    }

    async controlPointAt(options: TimeOptions): Promise<ControlPoint> {
        return this.api.controlPointAt(options);
    }

    async importOsb(options: TextOptions): Promise<ImportOsbResult> {
        return this.api.importOsb(options);
    }

    async layerHandle(options: LayerHandleOptions): Promise<LayerHandle> {
        return this.api.layerHandle(options);
    }

    async loadBeatmap(options: TextOptions): Promise<LoadBeatmapResult> {
        return this.api.loadBeatmap(options);
    }

    async random(options: RandomOptions): Promise<RandomResult> {
        return this.api.random(options);
    }

    async spriteCount(): Promise<SpriteCountResult> {
        return this.api.spriteCount();
    }

    async storyboard(): Promise<Storyboard> {
        return this.api.storyboard();
    }

    async timingPointAt(options: TimeOptions): Promise<ControlPoint> {
        return this.api.timingPointAt(options);
    }
}
