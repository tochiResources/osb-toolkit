import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { defineConfig } from 'vitest/config';

const here = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
    resolve: {
        alias: {
            '@tochii/osb-toolkit-shared/fs': resolve(here, '../shared/src/fs.ts'),
            '@tochii/osb-toolkit-shared': resolve(here, '../shared/src/index.ts'),
        },
    },
    test: { environment: 'node', include: ['tests/**/*.test.ts'] },
});
