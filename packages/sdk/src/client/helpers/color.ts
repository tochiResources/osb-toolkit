import type { Color4 } from '../../lib';
import type { Call, Channel } from './types';

/** A gradient stop: an offset in `0..=1` and its color. */
export interface ColorStop {
    offset: number;
    color: Color4;
}

/** A color space a color helper operates in. HSV is the same as HSB. */
export type ColorSpace = 'rgb' | 'hsl' | 'hsb' | 'hsv' | 'hwb';

/**
 * The color-space manipulation and CSS/Sass-style helpers: offset/scale/
 * randomize in any space (RGB/HSL/HSB/HWB), `lighten`/`darken`, `saturate`/
 * `desaturate`, `adjustHue`/`complement`, `grayscale`/`invert`, `mix`/`tint`/
 * `shade`, `opacify`/`transparentize`, and multi-stop gradients.
 */
export interface ColorHelpers {
    /** Calls any color helper by name with a raw input object. */
    call<T = unknown>(name: string, input?: unknown): Promise<T>;
    /** Converts a color into its `(x, y, z, w)` components in `space`. */
    toSpace(color: Color4, space?: ColorSpace): Promise<number[]>;
    /** Builds a color from `(x, y, z, w)` components in `space`. */
    fromSpace(value: number[], space?: ColorSpace): Promise<Color4>;
    /** Adds a per-component `delta` in `space`. */
    offset(color: Color4, options: { space?: ColorSpace; delta: number[] }): Promise<Color4>;
    /** Multiplies each component by a per-component `factor` in `space`. */
    scale(color: Color4, options: { space?: ColorSpace; factor: number[] }): Promise<Color4>;
    /** Jitters each component by a seeded per-component range in `space`. */
    randomize(
        color: Color4,
        options?: { space?: ColorSpace; seed?: number; min?: number[]; max?: number[] }
    ): Promise<Color4>;
    /** Applies a named CSS/Sass helper with extra options. */
    apply(color: Color4, op: string, options?: Record<string, unknown>): Promise<Color4>;
    lighten(color: Color4, amount: number): Promise<Color4>;
    darken(color: Color4, amount: number): Promise<Color4>;
    saturate(color: Color4, amount: number): Promise<Color4>;
    desaturate(color: Color4, amount: number): Promise<Color4>;
    adjustHue(color: Color4, degrees: number): Promise<Color4>;
    complement(color: Color4): Promise<Color4>;
    grayscale(color: Color4): Promise<Color4>;
    invert(color: Color4): Promise<Color4>;
    mix(a: Color4, b: Color4, weight?: number): Promise<Color4>;
    tint(color: Color4, weight?: number): Promise<Color4>;
    shade(color: Color4, weight?: number): Promise<Color4>;
    opacify(color: Color4, amount: number): Promise<Color4>;
    transparentize(color: Color4, amount: number): Promise<Color4>;
    withAlpha(color: Color4, alpha: number): Promise<Color4>;
    /** Samples a multi-stop gradient at `t` (`0..=1`). */
    gradient(
        stops: ColorStop[],
        t: number,
        options?: { space?: ColorSpace; repeat?: boolean }
    ): Promise<Color4>;
    /** Builds a color channel by sampling a gradient `count` times over `[start, end]`. */
    gradientChannel(
        stops: ColorStop[],
        options?: {
            space?: ColorSpace;
            count?: number;
            start?: number;
            end?: number;
            repeat?: boolean;
        }
    ): Promise<Channel>;
    /** Offsets a whole color channel in `space`. */
    offsetChannel(
        channel: Channel,
        options: { space?: ColorSpace; delta: number[] }
    ): Promise<Channel>;
    /** Scales a whole color channel in `space`. */
    scaleChannel(
        channel: Channel,
        options: { space?: ColorSpace; factor: number[] }
    ): Promise<Channel>;
    /** Randomizes a whole color channel in `space`. */
    randomizeChannel(
        channel: Channel,
        options?: { space?: ColorSpace; seed?: number; min?: number[]; max?: number[] }
    ): Promise<Channel>;
    /** Applies a named helper to every keyframe of a color channel. */
    applyChannel(channel: Channel, op: string, options?: Record<string, unknown>): Promise<Channel>;
}

/** Builds the color-space and CSS/Sass helper client. */
export function createColorHelpers(call: Call): ColorHelpers {
    return {
        call: (name, input) => call(name, input),
        toSpace: (c, space = 'rgb') => call('color.toSpace', { color: c, space }),
        fromSpace: (value, space = 'rgb') => call('color.fromSpace', { value, space }),
        offset: (c, options) => call('color.offset', { color: c, ...options }),
        scale: (c, options) => call('color.scale', { color: c, ...options }),
        randomize: (c, options = {}) => call('color.randomize', { color: c, ...options }),
        apply: (c, op, options = {}) => call('color.apply', { color: c, op, ...options }),
        lighten: (c, amount) => call('color.apply', { color: c, op: 'lighten', amount }),
        darken: (c, amount) => call('color.apply', { color: c, op: 'darken', amount }),
        saturate: (c, amount) => call('color.apply', { color: c, op: 'saturate', amount }),
        desaturate: (c, amount) => call('color.apply', { color: c, op: 'desaturate', amount }),
        adjustHue: (c, degrees) => call('color.apply', { color: c, op: 'adjustHue', degrees }),
        complement: c => call('color.apply', { color: c, op: 'complement' }),
        grayscale: c => call('color.apply', { color: c, op: 'grayscale' }),
        invert: c => call('color.apply', { color: c, op: 'invert' }),
        mix: (a, b, weight = 0.5) => call('color.apply', { color: a, op: 'mix', other: b, weight }),
        tint: (c, weight = 0.5) => call('color.apply', { color: c, op: 'tint', weight }),
        shade: (c, weight = 0.5) => call('color.apply', { color: c, op: 'shade', weight }),
        opacify: (c, amount) => call('color.apply', { color: c, op: 'opacify', amount }),
        transparentize: (c, amount) =>
            call('color.apply', { color: c, op: 'transparentize', amount }),
        withAlpha: (c, alpha) => call('color.apply', { color: c, op: 'withAlpha', alpha }),
        gradient: (stops, t, options = {}) => call('color.gradient', { stops, t, ...options }),
        gradientChannel: (stops, options = {}) =>
            call('color.gradientChannel', { stops, ...options }),
        offsetChannel: (channel, options) => call('color.offsetChannel', { channel, ...options }),
        scaleChannel: (channel, options) => call('color.scaleChannel', { channel, ...options }),
        randomizeChannel: (channel, options = {}) =>
            call('color.randomizeChannel', { channel, ...options }),
        applyChannel: (channel, op, options = {}) =>
            call('color.applyChannel', { channel, op, ...options }),
    };
}
