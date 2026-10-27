import { defineConfig } from 'tsdown';

export default defineConfig({
    entry: {
        index: 'src/index.ts',
        cli: 'src/cli.ts',
        convert: 'src/convert.ts',
        'csharp/index': 'src/csharp/index.ts',
        'csharp/parser': 'src/csharp/parser/index.ts',
        'csharp/rust': 'src/csharp/rust/index.ts',
        'csharp/tokenizer': 'src/csharp/tokenizer/index.ts',
        'generate/index': 'src/generate/index.ts',
        'generate/ae': 'src/generate/ae.ts',
        'generate/ofx': 'src/generate/ofx.ts',
        'generate/ppro': 'src/generate/ppro.ts',
        'generate/rust': 'src/generate/rust/index.ts',
        installer: 'src/installer.ts',
    },
    format: ['esm'],
    platform: 'node',
    fixedExtension: false,
    deps: { neverBundle: [/^node:/] },
    dts: { tsconfig: 'tsconfig.dts.json', sourcemap: false },
    clean: true,
    sourcemap: false,
    target: 'es2022',
    tsconfig: 'tsconfig.build.json',
});
