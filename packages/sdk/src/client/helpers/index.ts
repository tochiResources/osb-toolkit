import type { HelpersAPI } from '../../lib';
import { type ColorHelpers, createColorHelpers } from './color';
import { type CommandHelpers, createCommandHelpers } from './command';
import { createEasingHelpers, type EasingHelpers } from './easing';
import { createKeyframeHelpers, type KeyframeHelpers } from './keyframe';
import { createOriginHelpers, type OriginHelpers } from './origin';
import { createScene2dHelpers, type Scene2dHelpers } from './scene2d';
import { createScene3dHelpers, type Scene3dHelpers } from './scene3d';
import { createTimelineHelpers, type TimelineHelpers } from './timeline';

export * from './color';
export * from './command';
export * from './easing';
export * from './keyframe';
export * from './origin';
export * from './scene2d';
export * from './scene3d';
export * from './timeline';
export * from './types';

/**
 * The shared motion-graphics helper surface: one `helper` op namespaced
 * by library. 2D and 3D share the keyframe logic (Z is optional), `origin`
 * carries the shared anchor/origin resolution, and `color`/`scene2d` add the
 * color-space and 2D scene helpers.
 */
export interface Helpers {
    /** Calls any helper by name (`keyframe.*`, `scene3d.*`, `command.*`, `origin.*`, `color.*`, `scene2d.*`). */
    call<T = unknown>(name: string, input?: unknown): Promise<T>;
    keyframe: KeyframeHelpers;
    command: CommandHelpers;
    origin: OriginHelpers;
    easing: EasingHelpers;
    timeline: TimelineHelpers;
    color: ColorHelpers;
    scene2d: Scene2dHelpers;
    scene3d: Scene3dHelpers;
}

/** Builds the shared helper client over the generated `helper` client. */
export function createHelpers(api: HelpersAPI): Helpers {
    const call = async <T>(name: string, input?: unknown): Promise<T> =>
        (await api.call({ name, input: input ?? {} })).value as T;

    return {
        call,
        keyframe: createKeyframeHelpers(call),
        command: createCommandHelpers(call),
        origin: createOriginHelpers(call),
        easing: createEasingHelpers(call),
        timeline: createTimelineHelpers(call),
        color: createColorHelpers(call),
        scene2d: createScene2dHelpers(call),
        scene3d: createScene3dHelpers(call),
    };
}
