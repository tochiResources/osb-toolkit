import type { EffectScript, GeneratedFile } from '@tochii/osb-toolkit-shared';

import type { PproGeneratorOptions } from '../types/generate.js';
import { generateRustEffectCrate } from './rust/index.js';

export function generatePremiereProPlugin(
    script: EffectScript,
    options: PproGeneratorOptions = {}
): GeneratedFile[] {
    return generateRustEffectCrate(script, 'premiere-pro', options);
}
