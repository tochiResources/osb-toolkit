import { SpriteAPI } from '../../api/SpriteAPI';
import type { Easing } from '../../models/Easing';
import type { ParameterType } from '../../models/ParameterType';
import type { SpriteInfo } from '../../models/SpriteInfo';
import type { SpriteState } from '../../models/SpriteState';
import type { CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions } from '../options';
import { isCommandOptions, isSpriteTimeOptions, isStartLoopGroupOptions, isStartTriggerGroupOptions } from '../options';

/**
 * The clean `Sprite` client.
 */
export class Sprite {
    constructor(
        private readonly api: SpriteAPI,
        private readonly layer: number,
        private readonly sprite: number
    ) {}

    static instanceOf(value: unknown): value is Sprite {
        return value instanceof Sprite;
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     *
     * @returns Acknowledged
     */
    async command(kind: string, startTime: number, endTime: number, easing: Easing): Promise<void>;
    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     *
     * @returns Acknowledged
     */
    async command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown): Promise<void>;
    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     * @param end The end value (scalar, point or color).
     *
     * @returns Acknowledged
     */
    async command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown, end: unknown): Promise<void>;
    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     * @param end The end value (scalar, point or color).
     * @param parameter The parameter kind for a `parameter` command.
     *
     * @returns Acknowledged
     */
    async command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown, end: unknown, parameter: ParameterType | null): Promise<void>;
    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param options A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @returns Acknowledged
     */
    async command(options: CommandOptions): Promise<void>;
    async command(optionsOrKind?: CommandOptions | string, startTime?: number, endTime?: number, easing?: Easing, start?: unknown, end?: unknown, parameter?: ParameterType | null): Promise<void> {
        const options: CommandOptions = isCommandOptions(optionsOrKind)
            ? optionsOrKind
            : ({ kind: optionsOrKind, startTime: startTime, endTime: endTime, easing: easing, start: start, end: end, parameter: parameter } as CommandOptions);
        await this.api.command({ layer: this.layer, sprite: this.sprite, ...options } as Parameters<SpriteAPI['command']>[0]);
    }

    /**
     * A sprite handle pair.
     *
     * @returns Acknowledged
     */
    async endGroup(): Promise<void> {
        await this.api.endGroup({ layer: this.layer, sprite: this.sprite });
    }

    /**
     * A sprite handle pair.
     *
     * @returns The sprite's initial state
     */
    async info(): Promise<SpriteInfo> {
        return this.api.info({ layer: this.layer, sprite: this.sprite });
    }

    /**
     * Start a loop group.
     *
     * @param startTime The loop start time in milliseconds.
     * @param count The number of iterations.
     *
     * @returns Acknowledged
     */
    async startLoopGroup(startTime: number, count: number): Promise<void>;
    /**
     * Start a loop group.
     *
     * @param options Start a loop group.
     *
     * @returns Acknowledged
     */
    async startLoopGroup(options: StartLoopGroupOptions): Promise<void>;
    async startLoopGroup(optionsOrStartTime?: StartLoopGroupOptions | number, count?: number): Promise<void> {
        const options: StartLoopGroupOptions = isStartLoopGroupOptions(optionsOrStartTime)
            ? optionsOrStartTime
            : ({ startTime: optionsOrStartTime, count: count } as StartLoopGroupOptions);
        await this.api.startLoopGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    /**
     * Start a trigger group.
     *
     * @param name The trigger name (matches a hit object).
     * @param startTime The trigger start time in milliseconds.
     * @param endTime The trigger end time in milliseconds.
     * @param group The trigger group index.
     *
     * @returns Acknowledged
     */
    async startTriggerGroup(name: string, startTime: number, endTime: number, group: number): Promise<void>;
    /**
     * Start a trigger group.
     *
     * @param options Start a trigger group.
     *
     * @returns Acknowledged
     */
    async startTriggerGroup(options: StartTriggerGroupOptions): Promise<void>;
    async startTriggerGroup(optionsOrName?: StartTriggerGroupOptions | string, startTime?: number, endTime?: number, group?: number): Promise<void> {
        const options: StartTriggerGroupOptions = isStartTriggerGroupOptions(optionsOrName)
            ? optionsOrName
            : ({ name: optionsOrName, startTime: startTime, endTime: endTime, group: group } as StartTriggerGroupOptions);
        await this.api.startTriggerGroup({ layer: this.layer, sprite: this.sprite, ...options });
    }

    /**
     * A sprite handle pair plus a sample time.
     *
     * @param time The sample time in milliseconds.
     *
     * @returns The sampled sprite state
     */
    async stateAt(time: number): Promise<SpriteState>;
    /**
     * A sprite handle pair plus a sample time.
     *
     * @param options A sprite handle pair plus a sample time.
     *
     * @returns The sampled sprite state
     */
    async stateAt(options: SpriteTimeOptions): Promise<SpriteState>;
    async stateAt(optionsOrTime?: SpriteTimeOptions | number): Promise<SpriteState> {
        const options: SpriteTimeOptions = isSpriteTimeOptions(optionsOrTime)
            ? optionsOrTime
            : ({ time: optionsOrTime } as SpriteTimeOptions);
        return this.api.stateAt({ layer: this.layer, sprite: this.sprite, ...options });
    }
}
