import { defineConfig } from '@deltabox/toolchain';

export default defineConfig({
    toolchain: {
        project: {
            type: 'monorepo-workspace',
            name: 'osb-toolkit-core',
            scope: '@tochii',
            nodeVersion: '22',
            packageManager: 'yarn',
            packageManagerVersion: '4.17.1',
        },
    },
});
