import type { EffectScript, GeneratedFile } from '@tochii/osb-toolkit-shared';

import type { OfxGeneratorOptions } from '../types/generate.js';
import { generateRustEffectCrate } from './rust/index.js';

export function generateDaVinciResolvePlugin(
    script: EffectScript,
    options: OfxGeneratorOptions = {}
): GeneratedFile[] {
    return generateRustEffectCrate(script, 'davinci-resolve', options);
}
