import type { Storyboard } from '../../lib';
import type { Axis, Call, Channel } from './types';

/** The strictly 3D constructors and the 3D world / unified storyboard bake. */
export interface Scene3dHelpers {
    /** A circular orbit position channel. */
    orbit(options?: {
        center?: number[];
        radius?: number;
        radiusX?: number;
        radiusY?: number;
        startAngle?: number;
        sweep?: number;
        count?: number;
    }): Promise<Channel>;
    /** A 2D circular orbit (`z = 0`). */
    orbit2d(options?: {
        center?: [number, number];
        radius?: number;
        radiusX?: number;
        radiusY?: number;
        startAngle?: number;
        sweep?: number;
        count?: number;
    }): Promise<Channel>;
    /** A procedural spin about an axis. */
    spin(options?: {
        axis?: Axis;
        turns?: number;
        duration?: number;
        count?: number;
    }): Promise<Channel>;
    /** Independent spins about X, Y and Z. */
    tumble(options?: { turns?: number[]; duration?: number; count?: number }): Promise<Channel>;
    /** A position channel sampled along a polyline. */
    pathFollow(
        points: number[][],
        options?: { count?: number; byArcLength?: boolean }
    ): Promise<Channel>;
    /**
     * Bakes a 3D world (`{ nodes, camera, options }`) into a full storyboard
     * model; `options2d` controls the embedding of any 2D subtrees.
     */
    bakeWorld(world: unknown, options2d?: unknown): Promise<Storyboard>;
    /**
     * Bakes a set of 2D and 3D worlds (`{ worlds2d, worlds3d, options, options2d }`)
     * into one storyboard model.
     */
    bakeScene(input: unknown): Promise<Storyboard>;
}

/** Builds the strictly 3D constructor + world-bake client. */
export function createScene3dHelpers(call: Call): Scene3dHelpers {
    return {
        orbit: (options = {}) => call('scene3d.orbit', options),
        orbit2d: (options = {}) => call('scene3d.orbit', options),
        spin: (options = {}) => call('scene3d.spin', options),
        tumble: (options = {}) => call('scene3d.tumble', options),
        pathFollow: (points, options = {}) => call('scene3d.pathFollow', { points, ...options }),
        bakeWorld: (world, options2d = {}) => call('scene3d.bakeWorld', { world, options2d }),
        bakeScene: input => call('scene3d.bakeScene', input),
    };
}
