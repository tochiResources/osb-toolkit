import { GeneratorAPI } from '../../api/GeneratorAPI';
import type { Beatmap } from '../../models/Beatmap';
import type { ControlPoint } from '../../models/ControlPoint';
import type { ImportOsbResult } from '../../models/ImportOsbResult';
import type { LayerHandle } from '../../models/LayerHandle';
import type { LayerHandleRequest } from '../../models/LayerHandleRequest';
import type { LoadBeatmapResult } from '../../models/LoadBeatmapResult';
import type { RandomRequest } from '../../models/RandomRequest';
import type { RandomResult } from '../../models/RandomResult';
import type { SpriteCountResult } from '../../models/SpriteCountResult';
import type { Storyboard } from '../../models/Storyboard';
import type { TextRequest } from '../../models/TextRequest';
import type { TimeRequest } from '../../models/TimeRequest';

export class Generator {
    constructor(private readonly api: GeneratorAPI) {}

    static instanceOf(value: unknown): value is Generator {
        return value instanceof Generator;
    }

    async beatmap(): Promise<Beatmap> {
        return this.api.beatmap();
    }

    async controlPointAt(body: TimeRequest): Promise<ControlPoint> {
        return this.api.controlPointAt(body);
    }

    async importOsb(body: TextRequest): Promise<ImportOsbResult> {
        return this.api.importOsb(body);
    }

    async layerHandle(body: LayerHandleRequest): Promise<LayerHandle> {
        return this.api.layerHandle(body);
    }

    async loadBeatmap(body: TextRequest): Promise<LoadBeatmapResult> {
        return this.api.loadBeatmap(body);
    }

    async random(body: RandomRequest): Promise<RandomResult> {
        return this.api.random(body);
    }

    async spriteCount(): Promise<SpriteCountResult> {
        return this.api.spriteCount();
    }

    async storyboard(): Promise<Storyboard> {
        return this.api.storyboard();
    }

    async timingPointAt(body: TimeRequest): Promise<ControlPoint> {
        return this.api.timingPointAt(body);
    }
}
