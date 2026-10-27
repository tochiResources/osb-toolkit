import type { EffectScript, GeneratedFile } from '@tochii/osb-toolkit-shared';

import type { AeGeneratorOptions } from '../types/generate.js';
import { generateRustEffectCrate } from './rust/index.js';

export function generateAfterEffectsPlugin(
    script: EffectScript,
    options: AeGeneratorOptions = {}
): GeneratedFile[] {
    return generateRustEffectCrate(script, 'after-effects', options);
}
