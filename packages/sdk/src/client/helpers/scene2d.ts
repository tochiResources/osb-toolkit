import type { Color4, OsbOrigin, Storyboard, StoryboardCommand } from '../../lib';
import type { Axis, Call, Channel } from './types';
/** A 2D transform: each field is an optional keyframe channel (omitted = identity). */
export interface Scene2dTransform {
    position?: Channel;
    scale?: Channel;
    rotation?: Channel;
    opacity?: Channel;
    color?: Channel;
    anchor?: Channel;
}

/** A sprite leaf in a 2D scene. */
export interface Scene2dSprite {
    texturePath: string;
    origin: OsbOrigin;
    additive?: boolean;
    transform?: Scene2dTransform;
    commands?: StoryboardCommand[];
}

/** A node in a 2D scene tree. */
export interface Scene2dNode {
    name?: string;
    transform?: Scene2dTransform;
    sprite?: Scene2dSprite;
    children?: Scene2dNode[];
    inheritsOpacity?: boolean;
    inheritsColor?: boolean;
    inheritsTransform?: boolean;
}

/** A 2D scene graph. */
export interface Scene2d {
    root: Scene2dNode;
}

/** Options controlling a 2D scene resolution. */
export interface Scene2dOptions {
    startTime?: number;
    endTime?: number;
    timeStep?: number;
    tolerance2d?: number;
    tolerance1d?: number;
    decimals?: number;
}

/** A sprite resolved from a 2D scene. */
export interface ResolvedSprite2d {
    texturePath: string;
    origin: OsbOrigin;
    additive: boolean;
    commands: StoryboardCommand[];
    timeRange: { start: number; end: number };
}

/**
 * The 2D scene graph: the 2D counterpart of the 3D scene tree. A node can
 * be controlled from the top (translate/scale/rotate/flatten/ease/retime) and the
 * whole set cascades; `resolve` samples the composed transforms into per-sprite
 * command sets.
 */
export interface Scene2dHelpers {
    /** Calls any scene2d helper by name with a raw input object. */
    call<T = unknown>(name: string, input?: unknown): Promise<T>;
    /** Resolves the scene into per-sprite command sets. */
    resolve(scene: Scene2d, options?: Scene2dOptions): Promise<ResolvedSprite2d[]>;
    /** Bakes the scene into a full storyboard model (per-node layers). */
    bake(scene: Scene2d, options?: Scene2dOptions): Promise<Storyboard>;
    /** The scene's occupied `[start, end]` range, or `null`. */
    timeRange(scene: Scene2d): Promise<{ start: number; end: number } | null>;
    /** Translates the whole scene (children inherit). */
    translate(scene: Scene2d, delta: [number, number]): Promise<Scene2d>;
    /** Scales the whole scene about its origin. */
    scale(scene: Scene2d, factor: number): Promise<Scene2d>;
    /** Rotates the whole scene (radians). */
    rotate(scene: Scene2d, radians: number): Promise<Scene2d>;
    /** Sets a constant opacity on the root (children inherit). */
    setOpacity(scene: Scene2d, value: number): Promise<Scene2d>;
    /** Sets a constant tint on the root (children inherit). */
    setColor(scene: Scene2d, color: Color4): Promise<Scene2d>;
    /** Flattens the whole scene. */
    flatten(
        scene: Scene2d,
        options?: { axis?: Axis; mode?: 'mean' | 'first' | 'last' | 'value'; value?: number }
    ): Promise<Scene2d>;
    /** Re-eases the whole scene. */
    ease(scene: Scene2d, options?: { easing?: string; samples?: number }): Promise<Scene2d>;
    /** Shifts the whole scene in time. */
    shift(scene: Scene2d, delta: number): Promise<Scene2d>;
    /** Scales the whole scene in time. */
    scaleTime(scene: Scene2d, factor: number, anchor?: number): Promise<Scene2d>;
    /** Reverses the whole scene in time. */
    reverse(scene: Scene2d, start?: number, end?: number): Promise<Scene2d>;
}

/** Builds the 2D scene graph client. */
export function createScene2dHelpers(call: Call): Scene2dHelpers {
    return {
        call: (name, input) => call(name, input),
        resolve: (scene, options = {}) => call('scene2d.resolve', { scene, options }),
        bake: (scene, options = {}) => call('scene2d.bake', { scene, options }),
        timeRange: scene => call('scene2d.timeRange', { scene }),
        translate: (scene, delta) => call('scene2d.translate', { scene, delta }),
        scale: (scene, factor) => call('scene2d.scale', { scene, factor }),
        rotate: (scene, radians) => call('scene2d.rotate', { scene, radians }),
        setOpacity: (scene, value) => call('scene2d.setOpacity', { scene, value }),
        setColor: (scene, c) => call('scene2d.setColor', { scene, color: c }),
        flatten: (scene, options = {}) => call('scene2d.flatten', { scene, ...options }),
        ease: (scene, options = {}) => call('scene2d.ease', { scene, ...options }),
        shift: (scene, delta) => call('scene2d.shift', { scene, delta }),
        scaleTime: (scene, factor, anchor = 0) =>
            call('scene2d.scaleTime', { scene, factor, anchor }),
        reverse: (scene, start = 0, end = undefined) =>
            call('scene2d.reverse', { scene, start, end }),
    };
}
