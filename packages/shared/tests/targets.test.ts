import { describe, expect, it } from 'vitest';

import { COMPATIBILITY, getCompatibility } from '../src/compat.js';
import { isPluginTarget, PLUGIN_TARGET_INFO } from '../src/targets.js';

describe('targets', () => {
    it('exposes the supported targets', () => {
        expect(Object.keys(PLUGIN_TARGET_INFO)).toHaveLength(3);
        expect(isPluginTarget('after-effects')).toBe(true);
        expect(isPluginTarget('blender')).toBe(false);
        expect(isPluginTarget('final-cut-pro')).toBe(false);
    });

    it('has compatibility data for every target', () => {
        for (const info of Object.values(PLUGIN_TARGET_INFO)) {
            expect(COMPATIBILITY[info.id]).toBeDefined();
            expect(getCompatibility(info.id).installPaths).toBeDefined();
        }
    });
});
