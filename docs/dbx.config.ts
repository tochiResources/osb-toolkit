import { defineConfig } from '@deltabox/toolchain';

export default defineConfig({
    toolchain: {
        project: {
            type: 'project',
            name: 'osb-toolkit-docs',
            scope: '@tochii',
            nodeVersion: '22',
            packageManager: 'yarn',
            packageManagerVersion: '4.17.1',
        },
    },
});
