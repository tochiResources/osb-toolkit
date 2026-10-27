import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

import { parseEffectScript } from '../src/csharp/parser/index.js';

const here = fileURLToPath(new URL('.', import.meta.url));
const examples = join(here, '..', 'examples');

describe('parseEffectScript', () => {
    it('parses the Background example', () => {
        const source = readFileSync(join(examples, 'Background.cs'), 'utf8');
        const script = parseEffectScript(source, { file: 'Background.cs' });

        expect(script.className).toBe('Background');
        expect(script.namespace).toBe('osbTKScripts');
        expect(script.baseClass).toBe('StoryboardObjectGenerator');
        expect(script.generateBody).toContain('GetMapsetBitmap');
        expect(script.parameters.map(p => p.name)).toEqual([
            'StartTime',
            'EndTime',
            'SpritePath',
            'Opacity',
        ]);
        expect(script.parameters.find(p => p.name === 'SpritePath')?.group).toBe('Sprite');
        expect(script.parameters.find(p => p.name === 'Opacity')?.defaultValue).toBe(0.2);
    });

    it('parses enum and vec2 parameters', () => {
        const source = readFileSync(join(examples, 'Spectrum.cs'), 'utf8');
        const script = parseEffectScript(source, { file: 'Spectrum.cs' });

        const origin = script.parameters.find(p => p.name === 'SpriteOrigin');
        expect(origin?.type).toBe('origin');
        expect(origin?.defaultValue).toBe('BottomLeft');
        expect(origin?.enumValues).toHaveLength(9);

        const position = script.parameters.find(p => p.name === 'Position');
        expect(position?.type).toBe('vec2');
        expect(position?.defaultValue).toEqual({ x: 0, y: 400 });
    });

    it('throws when Generate() is missing', () => {
        const source = 'public class Empty : StoryboardObjectGenerator { }';
        expect(() => parseEffectScript(source)).toThrow(/Generate/);
    });
});
