import { SpriteAPI } from '../../api/SpriteAPI';
import type { OkResult } from '../../models/OkResult';
import type { SpriteInfo } from '../../models/SpriteInfo';
import type { SpriteState } from '../../models/SpriteState';
import type { CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions } from '../options';

export class Sprite {
    constructor(
        private readonly api: SpriteAPI,
        private readonly layer: number,
        private readonly sprite: number
    ) {}

    static instanceOf(value: unknown): value is Sprite {
        return value instanceof Sprite;
    }

    async command(options: CommandOptions): Promise<OkResult> {
        return this.api.command({ layer: this.layer, sprite: this.sprite, ...options });
    }

    async endGroup(): Promise<OkResult> {
        return this.api.endGroup({ layer: this.layer, sprite: this.sprite });
    }

    async info(): Promise<SpriteInfo> {
        return this.api.info({ layer: this.layer, sprite: this.sprite });
    }

    async startLoopGroup(options: StartLoopGroupOptions): Promise<OkResult> {
        return this.api.startLoopGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    async startTriggerGroup(options: StartTriggerGroupOptions): Promise<OkResult> {
        return this.api.startTriggerGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    async stateAt(options: SpriteTimeOptions): Promise<SpriteState> {
        return this.api.stateAt({ layer: this.layer, sprite: this.sprite, ...options });
    }
}
