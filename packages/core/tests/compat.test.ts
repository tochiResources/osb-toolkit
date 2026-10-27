import { describe, expect, it } from 'vitest';

import {
    compatForFramework,
    isKnownTypeAlias,
    normalizeTypeName,
    resolveNamespace,
} from '../src/csharp/compat.js';

describe('compat', () => {
    it('normalizes OpenTK/osuTK and namespaced type spellings', () => {
        expect(normalizeTypeName('Vector2')).toBe('Vector2');
        expect(normalizeTypeName('osuTK.Vector2')).toBe('Vector2');
        expect(normalizeTypeName('OpenTK.Vector2')).toBe('Vector2');
        expect(normalizeTypeName('System.Numerics.Vector3')).toBe('Vector3');
        expect(normalizeTypeName('OpenTK.Graphics.Color4')).toBe('Color4');
        expect(normalizeTypeName('osuTK.Graphics.Color4')).toBe('Color4');
    });

    it('resolves namespaces to their canonical form', () => {
        expect(resolveNamespace('osuTK')).toBe('OpenTK');
        expect(resolveNamespace('OpenTK.Graphics')).toBe('OpenTK.Graphics');
        expect(resolveNamespace('osuTK.Graphics')).toBe('OpenTK.Graphics');
        expect(resolveNamespace('osbTKCommon.Scripting')).toBe('osbTKCommon.Scripting');
        expect(resolveNamespace('StorybrewCommon.Scripting')).toBe('osbTKCommon.Scripting');
        expect(resolveNamespace('AeBrewCommon.Scripting')).toBe('osbTKCommon.Scripting');
    });

    it('reports whether a spelling is a known type alias', () => {
        expect(isKnownTypeAlias('osuTK.Vector2')).toBe(true);
        expect(isKnownTypeAlias('osbTKCommon.Storyboarding.OsbSprite')).toBe(false);
    });

    it('filters framework compatibility by version (minimum 4.5.2)', () => {
        const min = compatForFramework('4.5.2');
        expect(min.some(entry => entry.member === 'Math.Round')).toBe(true);
        expect(min.some(entry => entry.member === 'Span<T>')).toBe(false);
        const modern = compatForFramework('4.7.2');
        expect(modern.some(entry => entry.member === 'Span<T>')).toBe(true);
    });
});
