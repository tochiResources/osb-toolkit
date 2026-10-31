import { defineConfig } from '@deltabox/toolchain';

export default defineConfig({
    toolchain: {
        project: {
            type: 'monorepo-workspace',
            name: 'osb-toolkit-native',
            scope: '@tochii',
            nodeVersion: '22',
            packageManager: 'yarn',
            packageManagerVersion: '4.17.1',
        },
        vite: { config: false },
    },
});
