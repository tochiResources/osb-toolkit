import type { PluginTarget, PluginTargetInfo } from './types/targets.js';

export const PLUGIN_TARGETS = [
    'after-effects',
    'davinci-resolve',
    'premiere-pro',
] as const satisfies readonly PluginTarget[];

export const PLUGIN_TARGET_INFO: Record<PluginTarget, PluginTargetInfo> = {
    'after-effects': {
        id: 'after-effects',
        label: 'After Effects',
        npmPackage: '@tochii/osb-toolkit-to-after-effects',
        hostApp: 'Adobe After Effects',
        technology: 'PF_ Effect SDK',
        artifact: '.aex',
        platforms: ['windows', 'macos'],
        requiresNativeToolchain: true,
        status: 'implemented',
    },
    'davinci-resolve': {
        id: 'davinci-resolve',
        label: 'DaVinci Resolve',
        npmPackage: '@tochii/osb-toolkit-to-davinci-resolve',
        hostApp: 'Blackmagic DaVinci Resolve',
        technology: 'OpenFX (OFX 1.4)',
        artifact: '.ofx.bundle',
        platforms: ['windows', 'macos', 'linux'],
        requiresNativeToolchain: true,
        status: 'scaffold',
    },
    'premiere-pro': {
        id: 'premiere-pro',
        label: 'Premiere Pro',
        npmPackage: '@tochii/osb-toolkit-to-premiere-pro',
        hostApp: 'Adobe Premiere Pro',
        technology: 'AE PF_ Effect SDK + PrSDK',
        artifact: '.aex',
        platforms: ['windows', 'macos'],
        requiresNativeToolchain: true,
        status: 'scaffold',
    },
};

export function isPluginTarget(value: unknown): value is PluginTarget {
    return typeof value === 'string' && (PLUGIN_TARGETS as readonly string[]).includes(value);
}
