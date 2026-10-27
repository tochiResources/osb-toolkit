import type { StoryboardCommand, StoryboardCommandKind, Vector3 } from '../../lib';
import type { Axis, Call, Channel, FlattenOptions } from './types';

/**
 * The shared keyframe transforms (M38), canonical in
 * `osb_runtime::animations::keyframes`. They accept a 2D, 3D or color channel,
 * and `transformCommands` applies any of them to a 2D command channel.
 */
export interface KeyframeHelpers {
    /** Calls any keyframe helper by name with a raw input object. */
    call<T = unknown>(name: string, input?: unknown): Promise<T>;
    /** Collapses an axis (or every axis) to a constant. */
    flatten(channel: Channel, options?: FlattenOptions): Promise<Channel>;
    /** Multiplies every component by `factor`. */
    scaleKeyframes(channel: Channel, factor: number): Promise<Channel>;
    /** Adds a constant `delta` to every component. */
    offsetKeyframes(channel: Channel, delta: number): Promise<Channel>;
    /** Reflects an axis (or every axis) about `pivot`. */
    mirror(channel: Channel, options?: { axis?: Axis; pivot?: number }): Promise<Channel>;
    /** Seeded jitter in `min..max`, preserving keyframe times. */
    randomize(
        channel: Channel,
        options?: { axis?: Axis; min?: number; max?: number; seed?: number; step?: number }
    ): Promise<Channel>;
    /** Reverses the channel in time about `[start, end]`. */
    reverseKeyframes(
        channel: Channel,
        options?: { start?: number; end?: number }
    ): Promise<Channel>;
    /** Tiles the `[start, end]` span `count` times. */
    repeatKeyframes(
        channel: Channel,
        options: { count: number; start: number; end: number; pingPong?: boolean }
    ): Promise<Channel>;
    /** Resamples to a fixed `step` (or to exactly `count` samples). */
    resampleKeyframes(
        channel: Channel,
        options?: { step?: number; count?: number }
    ): Promise<Channel>;
    /** Moving-average smoothing over a window of neighbouring keyframes. */
    smoothKeyframes(channel: Channel, window?: number): Promise<Channel>;
    /** Douglas-Peucker simplification of each component. */
    simplifyKeyframes(
        channel: Channel,
        options?: { tolerance?: number; decimals?: number }
    ): Promise<Channel>;
    /** Snaps keyframe times to a beat grid. */
    alignKeyframes(
        channel: Channel,
        options?: { beat?: number; offset?: number; subdivision?: number }
    ): Promise<Channel>;
    /** Re-eases each span (named easing). */
    easeKeyframes(
        channel: Channel,
        options?: { easing?: string; samples?: number }
    ): Promise<Channel>;
    /** Blends the channel toward `other` by `amount` (`0..=1`). */
    mergeKeyframes(channel: Channel, other: Channel, amount?: number): Promise<Channel>;
    /** A stagger delay for element `index`: `index * step`. */
    stagger(index: number, step: number): Promise<number>;
    /** A pulse oscillation channel. */
    pulse(options?: {
        base?: number;
        amplitude?: number;
        frequency?: number;
        phase?: number;
        step?: number;
        duration?: number;
    }): Promise<Channel>;
    /** Seeded random placements in a box. */
    scatter(options?: {
        seed?: number;
        count?: number;
        min?: number[];
        max?: number[];
        spread?: number;
    }): Promise<Vector3[]>;
    /**
     * Applies any keyframe transform to a 2D command channel: extract the `kind`
     * channel, apply `name` (e.g. `keyframe.flatten`) with `options`, and write
     * it back as commands.
     */
    transformCommands(
        commands: StoryboardCommand[],
        kind: StoryboardCommandKind,
        name: string,
        options?: unknown
    ): Promise<StoryboardCommand[]>;
}

/** Builds the shared keyframe helper client. */
export function createKeyframeHelpers(call: Call): KeyframeHelpers {
    return {
        call: (name, input) => call(name, input),
        flatten: (channel, options = {}) => call('keyframe.flatten', { channel, ...options }),
        scaleKeyframes: (channel, factor) => call('keyframe.scaleKeyframes', { channel, factor }),
        offsetKeyframes: (channel, delta) => call('keyframe.offsetKeyframes', { channel, delta }),
        mirror: (channel, options = {}) => call('keyframe.mirror', { channel, ...options }),
        randomize: (channel, options = {}) => call('keyframe.randomize', { channel, ...options }),
        reverseKeyframes: (channel, options = {}) =>
            call('keyframe.reverseKeyframes', { channel, ...options }),
        repeatKeyframes: (channel, options) =>
            call('keyframe.repeatKeyframes', { channel, ...options }),
        resampleKeyframes: (channel, options = {}) =>
            call('keyframe.resampleKeyframes', { channel, ...options }),
        smoothKeyframes: (channel, window = 3) =>
            call('keyframe.smoothKeyframes', { channel, window }),
        simplifyKeyframes: (channel, options = {}) =>
            call('keyframe.simplifyKeyframes', { channel, ...options }),
        alignKeyframes: (channel, options = {}) =>
            call('keyframe.alignKeyframes', { channel, ...options }),
        easeKeyframes: (channel, options = {}) =>
            call('keyframe.easeKeyframes', { channel, ...options }),
        mergeKeyframes: (channel, other, amount = 0.5) =>
            call('keyframe.mergeKeyframes', { channel, other, amount }),
        stagger: (index, step) => call('keyframe.stagger', { index, step }),
        pulse: (options = {}) => call('keyframe.pulse', options),
        scatter: (options = {}) => call('keyframe.scatter', options),
        transformCommands: (commands, kind, name, options = {}) =>
            call(name, { commands, kind, ...(options as object) }),
    };
}
