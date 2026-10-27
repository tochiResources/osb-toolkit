import { describe, expect, it } from 'vitest';

import { classifyParameter } from '../src/parameters.js';
import type { EffectParameter } from '../src/types/ir.js';

function parameter(partial: Partial<EffectParameter>): EffectParameter {
    return { name: 'Value', csType: 'double', type: 'double', order: 0, ...partial };
}

describe('classifyParameter', () => {
    it('maps scalar types to sliders and angles', () => {
        expect(classifyParameter(parameter({ name: 'BarCount', type: 'int' })).control).toBe(
            'slider'
        );
        expect(classifyParameter(parameter({ name: 'Rotation', type: 'double' })).control).toBe(
            'angle'
        );
    });

    it('maps bools, colors, enums and points', () => {
        expect(classifyParameter(parameter({ name: 'Show', type: 'bool' })).control).toBe(
            'checkbox'
        );
        expect(classifyParameter(parameter({ name: 'Colour', type: 'color' })).control).toBe(
            'color'
        );
        expect(classifyParameter(parameter({ name: 'Origin', type: 'origin' })).control).toBe(
            'popup'
        );
        expect(classifyParameter(parameter({ name: 'Position', type: 'vec2' })).control).toBe(
            'point'
        );
    });

    it('classifies asset inputs by name and literal', () => {
        expect(classifyParameter(parameter({ name: 'SpritePath', type: 'string' })).assetKind).toBe(
            'sprite'
        );
        expect(classifyParameter(parameter({ name: 'AudioPath', type: 'string' })).assetKind).toBe(
            'audio'
        );
        expect(
            classifyParameter(parameter({ name: 'BackgroundImage', type: 'string' })).assetKind
        ).toBe('background');
        expect(classifyParameter(parameter({ name: 'FontName', type: 'string' })).assetKind).toBe(
            'font'
        );
        expect(
            classifyParameter(
                parameter({ name: 'File', type: 'string', defaultValueLiteral: '"sb/thing.png"' })
            ).control
        ).toBe('picker');
    });

    it('leaves plain strings as text controls', () => {
        expect(classifyParameter(parameter({ name: 'Label', type: 'string' })).control).toBe(
            'text'
        );
    });
});
