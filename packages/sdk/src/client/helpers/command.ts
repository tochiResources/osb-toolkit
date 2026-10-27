import type { Easing, StoryboardCommand, StoryboardCommandKind } from '../../lib';
import type { Call } from './types';

/** What `Helpers.command` filters/quantizes on. */
export interface CommandSelector {
    /** Restrict to one command kind. */
    kind?: StoryboardCommandKind;
    /** Keep commands overlapping `start`. */
    start?: number;
    /** Keep commands overlapping `end`. */
    end?: number;
}

/** The 2D command-list utilities. */
export interface CommandHelpers {
    /** Calls any command helper by name with a raw input object. */
    call<T = unknown>(name: string, input?: unknown): Promise<T>;
    /** Selects a timeframe (and/or kind) of commands without mutating the source. */
    selectCommands(
        commands: StoryboardCommand[],
        selector?: CommandSelector
    ): Promise<StoryboardCommand[]>;
    /** Moves a command set in time by `delta` ms. */
    shiftCommands(commands: StoryboardCommand[], delta: number): Promise<StoryboardCommand[]>;
    /** Stretches a command set in time about `anchor` by `factor`. */
    scaleCommands(
        commands: StoryboardCommand[],
        factor: number,
        anchor?: number
    ): Promise<StoryboardCommand[]>;
    /** Reverses a command set in time about `[start, end]`. */
    reverseCommands(
        commands: StoryboardCommand[],
        start: number,
        end: number
    ): Promise<StoryboardCommand[]>;
    /** Merges adjacent commands that are continuous in time and value. */
    mergeCommands(commands: StoryboardCommand[], tolerance?: number): Promise<StoryboardCommand[]>;
    /** Splits commands spanning `at` into two linear pieces. */
    splitCommands(commands: StoryboardCommand[], at: number): Promise<StoryboardCommand[]>;
    /** Collapses a command channel to a constant. */
    flattenCommands(
        commands: StoryboardCommand[],
        options?: {
            kind?: StoryboardCommandKind;
            mode?: 'mean' | 'first' | 'last' | 'value';
            value?: number;
        }
    ): Promise<StoryboardCommand[]>;
    /** Seeded jitter of command values. */
    randomizeCommands(
        commands: StoryboardCommand[],
        options?: { min?: number; max?: number; seed?: number }
    ): Promise<StoryboardCommand[]>;
    /** Keeps the commands matching a kind/time selector. */
    filterCommands(
        commands: StoryboardCommand[],
        selector?: CommandSelector
    ): Promise<StoryboardCommand[]>;
    /** Removes the commands matching a kind/time selector. */
    removeCommands(
        commands: StoryboardCommand[],
        selector?: CommandSelector
    ): Promise<StoryboardCommand[]>;
    /** Applies a scalar value transform (`value * scale + offset`) to every command. */
    mapCommands(
        commands: StoryboardCommand[],
        options?: { valueScale?: number; valueOffset?: number }
    ): Promise<StoryboardCommand[]>;
    /** Clones a command set `count` times, each copy shifted by `offset` ms. */
    duplicateCommands(
        commands: StoryboardCommand[],
        options?: { offset?: number; count?: number }
    ): Promise<StoryboardCommand[]>;
    /** Snaps command times to a beat grid (and values to `valueStep`). */
    alignCommands(
        commands: StoryboardCommand[],
        options?: { beat?: number; offset?: number; subdivision?: number; valueStep?: number }
    ): Promise<StoryboardCommand[]>;
    /** Rounds command times/values to a step. */
    quantizeCommands(
        commands: StoryboardCommand[],
        options?: { timeStep?: number; valueStep?: number }
    ): Promise<StoryboardCommand[]>;
    /** Sets an easing on every command (apply an easing to a whole group/scene). */
    setEasing(commands: StoryboardCommand[], easing: Easing): Promise<StoryboardCommand[]>;
    /** Re-eases a command channel with a named easing, resolving it into commands. */
    applyEasing(
        commands: StoryboardCommand[],
        kind: StoryboardCommandKind,
        easing: Easing,
        samples?: number
    ): Promise<StoryboardCommand[]>;
    /** Re-eases a command channel with a cubic-bezier, resolving it into commands. */
    applyBezier(
        commands: StoryboardCommand[],
        kind: StoryboardCommandKind,
        x1: number,
        y1: number,
        x2: number,
        y2: number,
        samples?: number
    ): Promise<StoryboardCommand[]>;
}

/** Builds the 2D command-list helper client. */
export function createCommandHelpers(call: Call): CommandHelpers {
    return {
        call: (name, input) => call(`command.${name}`, input),
        selectCommands: (commands, selector = {}) =>
            call('command.selectCommands', { commands, ...selector }),
        shiftCommands: (commands, delta) => call('command.shiftCommands', { commands, delta }),
        scaleCommands: (commands, factor, anchor = 0) =>
            call('command.scaleCommands', { commands, factor, anchor }),
        reverseCommands: (commands, start, end) =>
            call('command.reverseCommands', { commands, start, end }),
        mergeCommands: (commands, tolerance = 0) =>
            call('command.mergeCommands', { commands, tolerance }),
        splitCommands: (commands, at) => call('command.splitCommands', { commands, at }),
        flattenCommands: (commands, options = {}) =>
            call('command.flattenCommands', { commands, ...options }),
        randomizeCommands: (commands, options = {}) =>
            call('command.randomizeCommands', { commands, ...options }),
        filterCommands: (commands, selector = {}) =>
            call('command.filterCommands', { commands, ...selector }),
        removeCommands: (commands, selector = {}) =>
            call('command.removeCommands', { commands, ...selector }),
        mapCommands: (commands, options = {}) =>
            call('command.mapCommands', { commands, ...options }),
        duplicateCommands: (commands, options = {}) =>
            call('command.duplicateCommands', { commands, ...options }),
        alignCommands: (commands, options = {}) =>
            call('command.alignCommands', { commands, ...options }),
        quantizeCommands: (commands, options = {}) =>
            call('command.quantizeCommands', { commands, ...options }),
        setEasing: (commands, easing) => call('command.setEasing', { commands, easing }),
        applyEasing: (commands, kind, easing, samples = 8) =>
            call('command.applyEasing', { commands, kind, easing, samples }),
        applyBezier: (commands, kind, x1, y1, x2, y2, samples = 8) =>
            call('command.applyBezier', { commands, kind, x1, y1, x2, y2, samples }),
    };
}
