import { describe, expect, it } from 'vitest';

import { createPluginSpec, pluginSpecSchema } from '../src/plugin-spec.js';
import { camelCase, pascalCase, reverseDnsId, slugify, snakeUpper } from '../src/slug.js';

describe('slug', () => {
    it('slugifies mixed-case names', () => {
        expect(slugify('HitObjectHighlight')).toBe('hit-object-highlight');
        expect(slugify('  My Effect!! ')).toBe('my-effect');
    });

    it('converts to pascal and camel case', () => {
        expect(pascalCase('my effect name')).toBe('MyEffectName');
        expect(camelCase('my effect name')).toBe('myEffectName');
    });

    it('produces upper snake case', () => {
        expect(snakeUpper('SpriteScale')).toBe('SPRITE_SCALE');
    });

    it('builds reverse dns ids', () => {
        expect(reverseDnsId('com.tochiresources.osbtk', 'Hit Object')).toBe(
            'com.tochiresources.osbtk.hitobject'
        );
    });
});

describe('createPluginSpec', () => {
    it('fills defaults and validates', () => {
        const spec = createPluginSpec({
            target: 'after-effects',
            name: 'Spectrum',
            parameters: [
                {
                    name: 'BarCount',
                    label: 'Bar Count',
                    scriptName: 'barCount',
                    csType: 'int',
                    type: 'int',
                    defaultValue: 96,
                    order: 0,
                },
            ],
        });
        expect(spec.id).toBe('com.tochiresources.osbtk.spectrum');
        expect(spec.matchName).toBe('TOCHIRES Spectrum');
        expect(() => pluginSpecSchema.parse(spec)).not.toThrow();
    });
});
