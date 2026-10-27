import { defineConfig } from 'tsdown';

export default defineConfig({
    entry: { index: 'src/index.ts', core: 'src/core/index.ts' },
    format: ['esm'],
    platform: 'node',
    fixedExtension: false,
    deps: { neverBundle: [/^node:/, '@tochii/osb-toolkit-native'] },
    dts: { tsconfig: 'tsconfig.dts.json', sourcemap: false },
    clean: true,
    sourcemap: false,
    target: 'es2022',
    tsconfig: 'tsconfig.build.json',
});
