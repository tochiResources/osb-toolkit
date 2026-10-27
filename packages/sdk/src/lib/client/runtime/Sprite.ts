import { SpriteAPI } from '../../api/SpriteAPI';
import type { Easing } from '../../models/Easing';
import type { OkResult } from '../../models/OkResult';
import type { ParameterType } from '../../models/ParameterType';
import type { SpriteInfo } from '../../models/SpriteInfo';
import type { SpriteState } from '../../models/SpriteState';
import type { CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions } from '../options';
import { isCommandOptions, isSpriteTimeOptions, isStartLoopGroupOptions, isStartTriggerGroupOptions } from '../options';

export class Sprite {
    constructor(
        private readonly api: SpriteAPI,
        private readonly layer: number,
        private readonly sprite: number
    ) {}

    static instanceOf(value: unknown): value is Sprite {
        return value instanceof Sprite;
    }

    async command(options: CommandOptions): Promise<OkResult>;
    async command(kind: string, startTime: number, endTime: number, easing: Easing, start?: unknown, end?: unknown, parameter?: ParameterType | null): Promise<OkResult>;
    async command(optionsOrKind?: CommandOptions | string, startTime?: number, endTime?: number, easing?: Easing, start?: unknown, end?: unknown, parameter?: ParameterType | null): Promise<OkResult> {
        const options: CommandOptions = isCommandOptions(optionsOrKind)
            ? optionsOrKind
            : ({ kind: optionsOrKind, startTime: startTime, endTime: endTime, easing: easing, start: start, end: end, parameter: parameter } as CommandOptions);
        return this.api.command({ layer: this.layer, sprite: this.sprite, ...options } as Parameters<SpriteAPI['command']>[0]);
    }

    async endGroup(): Promise<OkResult> {
        return this.api.endGroup({ layer: this.layer, sprite: this.sprite });
    }

    async info(): Promise<SpriteInfo> {
        return this.api.info({ layer: this.layer, sprite: this.sprite });
    }

    async startLoopGroup(options: StartLoopGroupOptions): Promise<OkResult>;
    async startLoopGroup(startTime: number, count: number): Promise<OkResult>;
    async startLoopGroup(optionsOrStartTime?: StartLoopGroupOptions | number, count?: number): Promise<OkResult> {
        const options: StartLoopGroupOptions = isStartLoopGroupOptions(optionsOrStartTime)
            ? optionsOrStartTime
            : ({ startTime: optionsOrStartTime, count: count } as StartLoopGroupOptions);
        return this.api.startLoopGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    async startTriggerGroup(options: StartTriggerGroupOptions): Promise<OkResult>;
    async startTriggerGroup(name: string, startTime: number, endTime: number, group: number): Promise<OkResult>;
    async startTriggerGroup(optionsOrName?: StartTriggerGroupOptions | string, startTime?: number, endTime?: number, group?: number): Promise<OkResult> {
        const options: StartTriggerGroupOptions = isStartTriggerGroupOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, startTime: startTime, endTime: endTime, group: group } as StartTriggerGroupOptions);
        return this.api.startTriggerGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    async stateAt(options: SpriteTimeOptions): Promise<SpriteState>;
    async stateAt(time: number): Promise<SpriteState>;
    async stateAt(optionsOrTime?: SpriteTimeOptions | number): Promise<SpriteState> {
        const options: SpriteTimeOptions = isSpriteTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as SpriteTimeOptions);
        return this.api.stateAt({ layer: this.layer, sprite: this.sprite, ...options });
    }
}
