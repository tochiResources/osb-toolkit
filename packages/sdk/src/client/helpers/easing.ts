import type { Easing } from '../../lib';
import type { Call } from './types';

/**
 * The easing-curve manipulation helpers (M39): stretch, amplify, reverse,
 * mirror, ping-pong, repeat, compose, blend and sample easings/beziers.
 */
export interface EasingHelpers {
    /** Applies an easing to `t`, remapped onto `[from, to]` (stretch over a timeframe). */
    stretch(easing: Easing, t: number, from: number, to: number): Promise<number>;
    /** Amplifies an easing about linear by `amount` (`>1` stronger). */
    amplify(easing: Easing, t: number, amount: number): Promise<number>;
    /** Time-reverses an easing. */
    reverse(easing: Easing, t: number): Promise<number>;
    /** Mirrors an easing about `t = 0.5`. */
    mirror(easing: Easing, t: number): Promise<number>;
    /** Ping-pongs an easing. */
    pingPong(easing: Easing, t: number): Promise<number>;
    /** Tiles an easing `count` times. */
    repeat(easing: Easing, t: number, count?: number): Promise<number>;
    /** Composes two easings: `outer(inner(t))`. */
    compose(outer: Easing, inner: Easing, t: number): Promise<number>;
    /** Blends two easings by `amount`. */
    blend(a: Easing, b: Easing, t: number, amount?: number): Promise<number>;
    /** Evaluates a cubic-bezier easing. */
    bezier(x1: number, y1: number, x2: number, y2: number, t: number): Promise<number>;
    /** Amplifies a cubic-bezier easing about linear. */
    amplifyBezier(
        x1: number,
        y1: number,
        x2: number,
        y2: number,
        t: number,
        amount?: number
    ): Promise<number>;
    /** Samples an easing at `count` points in `[0, 1]`. */
    sample(easing: Easing, count?: number): Promise<number[]>;
    /** Samples a cubic-bezier easing at `count` points in `[0, 1]`. */
    sampleBezier(x1: number, y1: number, x2: number, y2: number, count?: number): Promise<number[]>;
    /** Evaluates a piecewise easing remap. */
    remap(segments: unknown[], t: number): Promise<number>;
    /** A stagger amount for element `index` (`index * step`). */
    stagger(index: number, step: number): Promise<number>;
}

/** Builds the easing-curve manipulation client. */
export function createEasingHelpers(call: Call): EasingHelpers {
    return {
        stretch: (easing, t, from, to) => call('easing.stretchEasing', { easing, t, from, to }),
        amplify: (easing, t, amount) => call('easing.amplifyEasing', { easing, t, amount }),
        reverse: (easing, t) => call('easing.reverseEasing', { easing, t }),
        mirror: (easing, t) => call('easing.mirrorEasing', { easing, t }),
        pingPong: (easing, t) => call('easing.pingPongEasing', { easing, t }),
        repeat: (easing, t, count = 2) => call('easing.repeatEasing', { easing, t, count }),
        compose: (outer, inner, t) => call('easing.composeEasing', { outer, inner, t }),
        blend: (a, b, t, amount = 0.5) => call('easing.blendEasing', { a, b, t, amount }),
        bezier: (x1, y1, x2, y2, t) => call('easing.bezierValue', { x1, y1, x2, y2, t }),
        amplifyBezier: (x1, y1, x2, y2, t, amount = 2) =>
            call('easing.amplifyBezier', { x1, y1, x2, y2, t, amount }),
        sample: (easing, count = 8) => call('easing.sampleEasing', { easing, count }),
        sampleBezier: (x1, y1, x2, y2, count = 8) =>
            call('easing.sampleBezier', { x1, y1, x2, y2, count }),
        remap: (segments, t) => call('easing.remapEasing', { segments, t }),
        stagger: (index, step) => call('easing.staggerEasing', { index, step }),
    };
}
