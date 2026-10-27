import {
    type Color4,
    Easing,
    type Handle,
    type OsbLayer,
    type OsbLoopType,
    type OsbOrigin,
    type ParameterType,
    type SegmentCleanResult,
    type SegmentDiscardResult,
    type SegmentIndexesResult,
    type SpriteInfo,
    type SpriteState,
    LayerAPI,
    SpriteAPI,
    type Vector2,
} from '../lib';

/**
 * A storyboard segment (the `StoryboardSegment` surface): a layer that creates
 * sprites, animations and samples, can nest sub-segments, and carries the
 * depth-ordering operations. The handle is implicit.
 */
export class Layer {
    constructor(
        private readonly api: LayerAPI,
        private readonly sprites: SpriteAPI,
        readonly name: string,
        readonly handle: number
    ) {}

    /** Creates a sprite on the segment. */
    async createSprite(texturePath: string, origin: OsbOrigin = 'Centre'): Promise<Sprite> {
        const handle = await this.api.createSprite({
            layerHandle: this.handle,
            path: texturePath,
            origin,
        });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    /** Creates a positioned sprite on the segment. */
    async createSpriteAt(
        texturePath: string,
        origin: OsbOrigin,
        position: Vector2
    ): Promise<Sprite> {
        const handle = await this.api.createSprite({
            layerHandle: this.handle,
            path: texturePath,
            origin,
            position,
        });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    /** Creates an animation sprite on the segment. */
    async createAnimation(
        texturePath: string,
        frames: number,
        frameDelay: number,
        loopType: OsbLoopType,
        origin: OsbOrigin,
        position: Vector2
    ): Promise<Sprite> {
        const handle = await this.api.createAnimation({
            layerHandle: this.handle,
            path: texturePath,
            frames,
            frameDelay,
            loopType,
            origin,
            position,
        });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    /** Adds a sound sample to the segment. */
    async createSample(samplePath: string, time: number, volume = 100): Promise<Layer> {
        await this.api.createSample({ layerHandle: this.handle, path: samplePath, time, volume });
        return this;
    }

    /** Creates a segment nested in this one (`CreateSegment`). */
    async createSegment(): Promise<Layer> {
        const handle = await this.api.createSegment({ layer: this.handle });
        return new Layer(this.api, this.sprites, '', handle.layer ?? 0);
    }

    /** Sets the reverse-depth flag (`ReverseDepth`). */
    async setReverseDepth(value: boolean): Promise<Layer> {
        await this.api.reverseDepth({ layer: this.handle, value });
        return this;
    }

    /** Records a depth index (`AddIndex`). */
    async addIndex(index: number): Promise<Layer> {
        await this.api.addIndex({ layer: this.handle, index });
        return this;
    }

    /** The depth indexes and bounds. */
    indexes(): Promise<SegmentIndexesResult> {
        return this.api.indexes({ layer: this.handle });
    }

    /** Moves the segment to another osu! layer (`ChangePosition`). */
    async changePosition(osbLayer: OsbLayer): Promise<Layer> {
        await this.api.changePosition({ layer: this.handle, osbLayer });
        return this;
    }

    /** Moves the segment on top of its siblings (`MoveToTop`). */
    async moveToTop(): Promise<Layer> {
        await this.api.moveToTop({ layer: this.handle });
        return this;
    }

    /** Moves the segment below its siblings (`MoveToBottom`). */
    async moveToBottom(): Promise<Layer> {
        await this.api.moveToBottom({ layer: this.handle });
        return this;
    }

    /** Removes zero-duration sprites (`Clean`). */
    clean(): Promise<SegmentCleanResult> {
        return this.api.clean({ layer: this.handle });
    }

    /** Removes the segment (`Discard`). */
    discard(): Promise<SegmentDiscardResult> {
        return this.api.discard({ layer: this.handle });
    }

    /** Removes a sprite from output (`DiscardSprite`). */
    async discardSprite(sprite: Sprite | number): Promise<Layer> {
        const index = typeof sprite === 'number' ? sprite : sprite.sprite;
        await this.api.discardSprite({ layer: this.handle, sprite: index });
        return this;
    }
}

/**
 * Normalizes the three command shapes - easing-first
 * (`Move(easing, start, end, …)`), time-range (`Move(start, end, …)`) and
 * time-only (`Move(time, …)`) - into `[startTime, endTime, easing, start, end]`.
 */
function normalize(
    args: unknown[],
    defaultEasing: Easing
): [number, number, Easing, unknown, unknown] {
    if (typeof args[0] === 'string') {
        return [args[1] as number, args[2] as number, args[0] as Easing, args[3], args[4]];
    }
    if (args.length === 2) {
        return [args[0] as number, args[0] as number, defaultEasing, args[1], args[1]];
    }
    return [
        args[0] as number,
        args[1] as number,
        (args[4] as Easing | undefined) ?? defaultEasing,
        args[2],
        args[3],
    ];
}

/**
 * A storyboard sprite. Command methods come in three shapes and are fluent; the
 * `*At` accessors sample them at a time.
 */
export class Sprite {
    constructor(
        private readonly api: SpriteAPI,
        readonly layer: number,
        readonly sprite: number
    ) {}

    /** The sprite's initial state and metadata. */
    info(): Promise<SpriteInfo> {
        return this.api.info({ layer: this.layer, sprite: this.sprite });
    }

    /** Adds a move command. */
    move(
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2,
        easing?: Easing
    ): Promise<Sprite>;
    move(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2
    ): Promise<Sprite>;
    move(time: number, position: Vector2): Promise<Sprite>;
    move(...args: unknown[]): Promise<Sprite> {
        return this.command('move', normalize(args, Easing.None));
    }

    /** Adds a horizontal move command. */
    moveX(
        startTime: number,
        endTime: number,
        start: number,
        end: number,
        easing?: Easing
    ): Promise<Sprite>;
    moveX(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: number,
        end: number
    ): Promise<Sprite>;
    moveX(time: number, x: number): Promise<Sprite>;
    moveX(...args: unknown[]): Promise<Sprite> {
        return this.command('moveX', normalize(args, Easing.None));
    }

    /** Adds a vertical move command. */
    moveY(
        startTime: number,
        endTime: number,
        start: number,
        end: number,
        easing?: Easing
    ): Promise<Sprite>;
    moveY(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: number,
        end: number
    ): Promise<Sprite>;
    moveY(time: number, y: number): Promise<Sprite>;
    moveY(...args: unknown[]): Promise<Sprite> {
        return this.command('moveY', normalize(args, Easing.None));
    }

    /** Adds a uniform scale command. */
    scale(
        startTime: number,
        endTime: number,
        start: number,
        end: number,
        easing?: Easing
    ): Promise<Sprite>;
    scale(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: number,
        end: number
    ): Promise<Sprite>;
    scale(time: number, value: number): Promise<Sprite>;
    scale(...args: unknown[]): Promise<Sprite> {
        return this.command('scale', normalize(args, Easing.None));
    }

    /** Adds a per-axis scale command. */
    scaleVec(
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2,
        easing?: Easing
    ): Promise<Sprite>;
    scaleVec(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2
    ): Promise<Sprite>;
    scaleVec(time: number, scale: Vector2): Promise<Sprite>;
    scaleVec(...args: unknown[]): Promise<Sprite> {
        return this.command('scaleVec', normalize(args, Easing.None));
    }

    /** Adds a rotation command (radians). */
    rotate(
        startTime: number,
        endTime: number,
        start: number,
        end: number,
        easing?: Easing
    ): Promise<Sprite>;
    rotate(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: number,
        end: number
    ): Promise<Sprite>;
    rotate(time: number, value: number): Promise<Sprite>;
    rotate(...args: unknown[]): Promise<Sprite> {
        return this.command('rotate', normalize(args, Easing.None));
    }

    /** Adds a fade command. */
    fade(
        startTime: number,
        endTime: number,
        start: number,
        end: number,
        easing?: Easing
    ): Promise<Sprite>;
    fade(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: number,
        end: number
    ): Promise<Sprite>;
    fade(time: number, opacity: number): Promise<Sprite>;
    fade(...args: unknown[]): Promise<Sprite> {
        return this.command('fade', normalize(args, Easing.None));
    }

    /** Adds a color command. */
    color(
        startTime: number,
        endTime: number,
        start: Color4,
        end: Color4,
        easing?: Easing
    ): Promise<Sprite>;
    color(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: Color4,
        end: Color4
    ): Promise<Sprite>;
    color(time: number, value: Color4): Promise<Sprite>;
    color(...args: unknown[]): Promise<Sprite> {
        return this.command('color', normalize(args, Easing.None));
    }

    /** Adds a hue/saturation/brightness color command. */
    colorHsb(
        startTime: number,
        endTime: number,
        start: Color4,
        end: Color4,
        easing?: Easing
    ): Promise<Sprite>;
    colorHsb(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: Color4,
        end: Color4
    ): Promise<Sprite>;
    colorHsb(time: number, value: Color4): Promise<Sprite>;
    colorHsb(...args: unknown[]): Promise<Sprite> {
        return this.command('colorHsb', normalize(args, Easing.None));
    }

    /**
     * Adds an animatable origin-offset command (`Anchor`). The offset is in
     * texture pixels and is added to the sprite's enum origin.
     */
    anchor(
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2,
        easing?: Easing
    ): Promise<Sprite>;
    anchor(
        easing: Easing,
        startTime: number,
        endTime: number,
        start: Vector2,
        end: Vector2
    ): Promise<Sprite>;
    anchor(time: number, point: Vector2): Promise<Sprite>;
    anchor(...args: unknown[]): Promise<Sprite> {
        return this.command('anchor', normalize(args, Easing.None));
    }

    /** Sets a constant origin offset (a zero-length `Anchor`). */
    setAnchor(point: Vector2): Promise<Sprite> {
        return this.anchor(0, 0, point, point, Easing.None);
    }

    /** Adds a parameter command (flip H/V or additive). */
    parameter(
        parameter: ParameterType,
        startTime: number,
        endTime: number,
        start = 0,
        end = 1,
        easing: Easing = Easing.None
    ): Promise<Sprite> {
        return this.command('parameter', [startTime, endTime, easing, start, end], parameter);
    }

    /** Adds a horizontal-flip command. */
    flipH(startTime: number, endTime: number): Promise<Sprite> {
        return this.parameter('FlipHorizontal', startTime, endTime);
    }

    /** Adds a vertical-flip command. */
    flipV(startTime: number, endTime: number): Promise<Sprite> {
        return this.parameter('FlipVertical', startTime, endTime);
    }

    /** Adds an additive-blend command. */
    additive(startTime: number, endTime: number): Promise<Sprite> {
        return this.parameter('Additive', startTime, endTime);
    }

    /** Starts a loop group; subsequent commands join it until `endGroup`. */
    async startLoopGroup(startTime: number, count: number): Promise<Sprite> {
        await this.api.startLoopGroup({ layer: this.layer, sprite: this.sprite, startTime, count });
        return this;
    }

    /** Starts a trigger group; subsequent commands join it until `endGroup`. */
    async startTriggerGroup(
        name: string,
        startTime: number,
        endTime: number,
        group = 0
    ): Promise<Sprite> {
        await this.api.startTriggerGroup({
            layer: this.layer,
            sprite: this.sprite,
            name,
            startTime,
            endTime,
            group,
        });
        return this;
    }

    /** Closes the active loop or trigger group. */
    async endGroup(): Promise<Sprite> {
        await this.api.endGroup({ layer: this.layer, sprite: this.sprite });
        return this;
    }

    /** The full sampled state at a time. */
    stateAt(time: number): Promise<SpriteState> {
        return this.api.stateAt({ layer: this.layer, sprite: this.sprite, time });
    }

    /** The sampled position at a time. */
    async positionAt(time: number): Promise<Vector2> {
        return (await this.stateAt(time)).position;
    }

    /** The sampled scale at a time. */
    async scaleAt(time: number): Promise<Vector2> {
        return (await this.stateAt(time)).scale;
    }

    /** The sampled rotation (radians) at a time. */
    async rotationAt(time: number): Promise<number> {
        return (await this.stateAt(time)).rotation;
    }

    /** The sampled opacity at a time. */
    async opacityAt(time: number): Promise<number> {
        return (await this.stateAt(time)).opacity;
    }

    /** The sampled color at a time. */
    async colorAt(time: number): Promise<Color4> {
        return (await this.stateAt(time)).color;
    }

    /** Whether the sprite is additive at a time. */
    async additiveAt(time: number): Promise<boolean> {
        return (await this.stateAt(time)).additive;
    }

    /** The sampled origin offset at a time (`AnchorAt`). */
    async anchorAt(time: number): Promise<Vector2> {
        return (await this.stateAt(time)).anchor;
    }

    private async command(
        kind: string,
        [startTime, endTime, easing, start, end]: [number, number, Easing, unknown, unknown],
        parameter?: ParameterType
    ): Promise<Sprite> {
        await this.api.command({
            kind,
            layer: this.layer,
            sprite: this.sprite,
            startTime,
            endTime,
            easing,
            start: start as object,
            end: end as object,
            parameter,
        } as Parameters<SpriteAPI['command']>[0]);
        return this;
    }
}

/** A handle returned by the runtime's create helpers, as a `Sprite`. */
export function spriteFrom(api: SpriteAPI, handle: Handle): Sprite {
    return new Sprite(api, handle.layer ?? 0, handle.sprite ?? 0);
}
