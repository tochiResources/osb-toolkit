/**
 * Shared types for the motion-graphics helper clients. The helper
 * surface is code-split by library (`keyframe`, `scene3d`, `command`, `origin`,
 * `easing`, `timeline`, `color`, `scene2d`); this module holds the types more than
 * one of them shares.
 */

/**
 * A keyframe channel: `{ time, value }` samples whose `value` has one, two,
 * three or four components. The Z axis is optional, so a scalar or 2-vector
 * channel is a 2D channel, a 3-vector channel is a 3D channel, and a
 * 4-component channel is a color. Each key may carry the `easing` that governs
 * the segment from that keyframe to the next (omitted means linear).
 */
export interface Channel {
    keys: { time: number; value: number[]; easing?: string }[];
}

/** Which component an axis-aware helper acts on. `w` is a color's alpha. */
export type Axis = 'x' | 'y' | 'z' | 'w';

/** How `flatten` reduces a channel. */
export interface FlattenOptions {
    axis?: Axis;
    /** `mean` (default), `first`, `last` or `value`. */
    mode?: 'mean' | 'first' | 'last' | 'value';
    /** The constant when `mode` is `value`. */
    value?: number;
}

/** A bridge executor bound to one helper namespace: `call('x.y', input)`. */
export type Call = <T = unknown>(name: string, input?: unknown) => Promise<T>;
