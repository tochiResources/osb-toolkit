import type { EffectScript, GeneratedFile, PluginTarget } from '@tochii/osb-toolkit-shared';

import type { GenerateForTargetOptions } from '../types/generate.js';
import { generateAfterEffectsPlugin } from './ae.js';
import { generateDaVinciResolvePlugin } from './ofx.js';
import { generatePremiereProPlugin } from './ppro.js';

export * from './ae.js';
export * from './common.js';
export * from './ofx.js';
export * from './ppro.js';
export * from './rust/bundle.js';
export * from './rust/index.js';

export function generateForTarget(
    script: EffectScript,
    target: PluginTarget,
    options: GenerateForTargetOptions = {}
): GeneratedFile[] {
    switch (target) {
        case 'after-effects':
            return generateAfterEffectsPlugin(script, options);
        case 'davinci-resolve':
            return generateDaVinciResolvePlugin(script, options);
        case 'premiere-pro':
            return generatePremiereProPlugin(script, options);
        default: {
            const exhaustive: never = target;
            throw new Error(`Unsupported target: ${String(exhaustive)}`);
        }
    }
}
