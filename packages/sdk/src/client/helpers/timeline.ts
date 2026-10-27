import type { Call, Channel } from './types';

/** A keyframe channel placed at `start` on a timeline. */
export interface TimelineClip {
    start: number;
    channel: Channel;
}

/**
 * A timeline: an ordered set of keyframe clips plus nested child timelines.
 * Children share the parent's time coordinate, so `shift`/`scale`/`reverse` on a
 * parent move its children and `flush` flattens the whole tree.
 */
export interface Timeline {
    clips: TimelineClip[];
    children?: Timeline[];
}

/**
 * The timeline sandbox: author motion as a sequence of clips - grouped
 * into sub-timelines where useful - and `flush` it to resolved channels.
 */
export interface TimelineHelpers {
    /** The timeline's total duration. */
    duration(timeline: Timeline): Promise<number>;
    /** The timeline's occupied `[start, end]` range, or `null`. */
    timeRange(timeline: Timeline): Promise<{ start: number; end: number } | null>;
    /** Moves the timeline (and its children) in time. */
    shift(timeline: Timeline, delta: number): Promise<Timeline>;
    /** Scales the timeline (and its children) in time about `anchor`. */
    scale(timeline: Timeline, factor: number, anchor?: number): Promise<Timeline>;
    /** Reverses the timeline (and its children) in time. */
    reverse(timeline: Timeline, start?: number, end?: number): Promise<Timeline>;
    /** Tiles the timeline `count` times. */
    repeat(timeline: Timeline, count?: number, pingPong?: boolean): Promise<Timeline>;
    /** Offsets each clip by `index * step`. */
    stagger(timeline: Timeline, step: number): Promise<Timeline>;
    /** A sub-timeline over `[start, end]`, rebased to `0`. */
    slice(timeline: Timeline, start: number, end?: number): Promise<Timeline>;
    /** Resolves the timeline tree to flat absolute-time clips. */
    flush(timeline: Timeline): Promise<Timeline>;
    /** Overlays another timeline's clips and children. */
    merge(a: Timeline, b: Timeline): Promise<Timeline>;
    /** Adds a clip to the timeline. */
    add(timeline: Timeline, channel: Channel, start?: number): Promise<Timeline>;
    /** Adds a nested child timeline (a group). */
    group(timeline: Timeline, child?: Timeline): Promise<Timeline>;
    /** Every clip in the tree, in the shared time coordinate. */
    allClips(timeline: Timeline): Promise<TimelineClip[]>;
    /** The total clip count across the tree. */
    clipCount(timeline: Timeline): Promise<number>;
}

/** Builds the timeline sandbox client. */
export function createTimelineHelpers(call: Call): TimelineHelpers {
    return {
        duration: tl => call('timeline.durationOf', { timeline: tl }),
        timeRange: tl => call('timeline.timeRangeOf', { timeline: tl }),
        shift: (tl, delta) => call('timeline.shiftTimeline', { timeline: tl, delta }),
        scale: (tl, factor, anchor = 0) =>
            call('timeline.scaleTimeline', { timeline: tl, factor, anchor }),
        reverse: (tl, start = 0, end = undefined) =>
            call('timeline.reverseTimeline', { timeline: tl, start, end }),
        repeat: (tl, count = 2, pingPong = false) =>
            call('timeline.repeatTimeline', { timeline: tl, count, pingPong }),
        stagger: (tl, step) => call('timeline.staggerTimeline', { timeline: tl, step }),
        slice: (tl, start, end = undefined) =>
            call('timeline.sliceTimeline', { timeline: tl, start, end }),
        flush: tl => call('timeline.flushTimeline', { timeline: tl }),
        merge: (a, b) => call('timeline.mergeTimelines', { timeline: a, other: b }),
        add: (tl, channel, start = 0) =>
            call('timeline.addToTimeline', { timeline: tl, channel, start }),
        group: (tl, child = { clips: [] }) => call('timeline.addGroup', { timeline: tl, child }),
        allClips: tl => call('timeline.allClips', { timeline: tl }),
        clipCount: tl => call('timeline.clipCount', { timeline: tl }),
    };
}
