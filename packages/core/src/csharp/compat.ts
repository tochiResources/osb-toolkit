/**
 * Type/namespace compatibility for C# effect scripts.
 *
 * Effect scripts target the OpenTK math types while some forks use
 * osuTK (same API, different assembly/namespace); the two also differ across
 * .NET Framework versions. This module is the single place that maps every
 * accepted spelling onto the canonical short type name the transpiler knows,
 * and records the framework-dependent API differences so the resolver can pick
 * the right semantics for a script.
 */

/** A namespace and the equivalent namespaces/assemblies scripts may use. */
export const NAMESPACE_ALIASES: Record<string, readonly string[]> = {
    OpenTK: ['osuTK', 'OpenTK.Mathematics'],
    'OpenTK.Graphics': ['osuTK.Graphics', 'OpenTK.Graphics.OpenGL'],
    'OpenTK.Input': ['osuTK.Input'],
    'osbTKCommon.Storyboarding': ['StorybrewCommon.Storyboarding', 'AeBrewCommon.Storyboarding'],
    'osbTKCommon.Storyboarding.Util': [
        'StorybrewCommon.Storyboarding.Util',
        'AeBrewCommon.Storyboarding.Util',
    ],
    'osbTKCommon.Storyboarding3d': [
        'StorybrewCommon.Storyboarding3d',
        'AeBrewCommon.Storyboarding3d',
    ],
    'osbTKCommon.Scripting': ['StorybrewCommon.Scripting', 'AeBrewCommon.Scripting'],
    'osbTKCommon.Mapset': ['StorybrewCommon.Mapset', 'AeBrewCommon.Mapset'],
    'osbTKCommon.Animations': ['StorybrewCommon.Animations', 'AeBrewCommon.Animations'],
    'osbTKCommon.Util': ['StorybrewCommon.Util', 'AeBrewCommon.Util'],
    'osbTKCommon.Subtitles': ['StorybrewCommon.Subtitles', 'AeBrewCommon.Subtitles'],
};

/**
 * Canonical type name -> the namespaced spellings that must resolve to it.
 * Only namespaces that differ between OpenTK/osuTK or that scripts qualify
 * explicitly are listed; unqualified names resolve to themselves.
 */
export const TYPE_ALIASES: Record<string, readonly string[]> = {
    Vector2: [
        'OpenTK.Vector2',
        'osuTK.Vector2',
        'System.Numerics.Vector2',
        'OpenTK.Mathematics.Vector2',
    ],
    Vector3: [
        'OpenTK.Vector3',
        'osuTK.Vector3',
        'System.Numerics.Vector3',
        'OpenTK.Mathematics.Vector3',
    ],
    Vector4: [
        'OpenTK.Vector4',
        'osuTK.Vector4',
        'System.Numerics.Vector4',
        'OpenTK.Mathematics.Vector4',
    ],
    Matrix4: ['OpenTK.Matrix4', 'osuTK.Matrix4', 'OpenTK.Mathematics.Matrix4'],
    Quaternion: ['OpenTK.Quaternion', 'osuTK.Quaternion', 'OpenTK.Mathematics.Quaternion'],
    Color4: [
        'OpenTK.Graphics.Color4',
        'osuTK.Graphics.Color4',
        'OpenTK.Color4',
        'System.Drawing.Color',
    ],
};

/** Lookup: namespaced spelling -> canonical short name. */
const QUALIFIED_TO_CANONICAL: Map<string, string> = (() => {
    const map = new Map<string, string>();
    for (const [canonical, aliases] of Object.entries(TYPE_ALIASES)) {
        map.set(canonical, canonical);
        for (const alias of aliases) map.set(alias, canonical);
    }
    return map;
})();

/**
 * A framework-dependent API difference. `since` is the minimum .NET Framework
 * version the behaviour applies from; `until` (exclusive) bounds it, so the
 * resolver can pick the variant that matches a script's target framework.
 */
export interface ApiCompatEntry {
    /** Canonical runtime member the C# member maps to. */
    member: string;
    /** Minimum .NET Framework version, e.g. `4.5.2`. */
    since: string;
    /** Exclusive upper bound, or `null` for "current". */
    until: string | null;
    /** Behavioural notes / semantics the transpiler must honour. */
    note?: string;
}

/**
 * API semantics that vary across .NET Framework versions (minimum 4.5.2).
 * The base behaviour is the 4.5.2 one; entries only record the differences.
 */
export const FRAMEWORK_COMPAT: readonly ApiCompatEntry[] = [
    {
        member: 'Math.Round',
        since: '4.5.2',
        until: '5.0',
        note: 'MidpointRounding.AwayFromZero is the default; the runtime round matches.',
    },
    {
        member: 'string.Empty',
        since: '4.5.2',
        until: null,
        note: 'Always the empty string; mapped to String::new().',
    },
    {
        member: 'IReadOnlyList',
        since: '4.5.2',
        until: null,
        note: 'Available from 4.5.2; treated as a slice.',
    },
    {
        member: 'Span<T>',
        since: '4.7.2',
        until: null,
        note: 'Not available before 4.7.2; scripts must not rely on it for the minimum target.',
    },
];

/** Compares two dotted version strings numerically. */
function compareVersions(a: string, b: string): number {
    const pa = a.split('.').map(Number);
    const pb = b.split('.').map(Number);
    for (let i = 0; i < Math.max(pa.length, pb.length); i += 1) {
        const diff = (pa[i] ?? 0) - (pb[i] ?? 0);
        if (diff !== 0) return diff;
    }
    return 0;
}

/**
 * Strips a namespace/assembly qualifier from a type name and resolves aliases
 * (e.g. `osuTK.Vector2` -> `Vector2`, `OpenTK.Graphics.Color4` -> `Color4`).
 */
export function normalizeTypeName(name: string): string {
    const trimmed = name.replace(/\s+/g, '');
    if (!trimmed) return trimmed;
    const canonical = QUALIFIED_TO_CANONICAL.get(trimmed);
    if (canonical) return canonical;
    // Unknown qualifier: drop everything up to the last dot.
    const lastDot = trimmed.lastIndexOf('.');
    return lastDot >= 0 ? trimmed.slice(lastDot + 1) : trimmed;
}

/** Resolves a namespace to its canonical form (e.g. `osuTK` -> `OpenTK`). */
export function resolveNamespace(namespace: string): string {
    for (const [canonical, aliases] of Object.entries(NAMESPACE_ALIASES)) {
        if (namespace === canonical || aliases.includes(namespace)) return canonical;
    }
    return namespace;
}

/** Whether `name` is a known math/type alias (any spelling). */
export function isKnownTypeAlias(name: string): boolean {
    return QUALIFIED_TO_CANONICAL.has(name.replace(/\s+/g, ''));
}

/**
 * The compatibility entries that apply to a target framework version
 * (defaults to the project minimum, 4.5.2).
 */
export function compatForFramework(version = '4.5.2'): ApiCompatEntry[] {
    return FRAMEWORK_COMPAT.filter(
        entry =>
            compareVersions(version, entry.since) >= 0 &&
            (entry.until === null || compareVersions(version, entry.until) < 0)
    );
}
