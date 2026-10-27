import type { TargetCompatibility } from './types/compat.js';
import type { PluginTarget } from './types/targets.js';

export const COMPATIBILITY: Record<PluginTarget, TargetCompatibility> = {
    'after-effects': {
        target: 'after-effects',
        sdk: 'After Effects SDK 26.5',
        apiVersion: 'PF_PLUG_IN_VERSION 13.28',
        platforms: ['windows', 'macos'],
        architectures: ['x64', 'arm64'],
        artifact: '.aex (Windows) / .plugin bundle (macOS)',
        installPaths: {
            windows: [
                '%APPDATA%\\Adobe\\After Effects\\<version>\\Support Files\\Plug-ins',
                '%PROGRAMFILES%\\Adobe\\Adobe After Effects <version>\\Support Files\\Plug-ins',
            ],
            macos: [
                '/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore',
                '~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore',
            ],
        },
        hostVersions: [
            {
                app: 'After Effects',
                min: 'CC 2014 (13.0)',
                technology: 'PF_ Effect SDK',
                notes: 'Effect API is stable across CC and all Creative Cloud releases; compile with the newest SDK for newest hosts.',
            },
        ],
        notes: [
            'Effects operate on pixels only: the plug-in renders the built storyboard into the layer.',
            'No ExtendScript and no external scripts; the effect logic is compiled in.',
            'PiPL resources are embedded at build time.',
        ],
    },
    'davinci-resolve': {
        target: 'davinci-resolve',
        sdk: 'OpenFX 1.4 (AcademySoftwareFoundation/openfx)',
        apiVersion: 'kOfxImageEffectPluginApiVersion = 1',
        platforms: ['windows', 'macos', 'linux'],
        architectures: ['x64', 'arm64 (Apple Silicon)'],
        artifact: '.ofx.bundle',
        installPaths: {
            windows: ['%COMMONPROGRAMFILES%\\OFX\\Plugins', 'OFX_PLUGIN_PATH'],
            macos: ['/Library/OFX/Plugins', 'OFX_PLUGIN_PATH'],
            linux: ['/usr/OFX/Plugins', 'OFX_PLUGIN_PATH'],
        },
        hostVersions: [
            {
                app: 'DaVinci Resolve',
                min: '14',
                technology: 'OpenFX 1.4',
                notes: 'Resolve 14+ loads OFX 1.4 plug-ins; OFX 1.5 features are not required.',
            },
        ],
        notes: [
            'Resolve is a strict OFX host: never mutate parameter UI in the constructor.',
            'Windows bundles live in Contents/Win64, macOS in Contents/MacOS (universal).',
            'OFX images are RGBA (unlike AE ARGB).',
        ],
    },
    'premiere-pro': {
        target: 'premiere-pro',
        sdk: 'Premiere Pro SDK 26.0 + After Effects Effect SDK',
        apiVersion: "PF_ ABI (appl_id 'PrMr')",
        platforms: ['windows', 'macos'],
        architectures: ['x64', 'arm64', 'x86_64 (macOS universal)'],
        artifact: '.aex (AE-style effect loaded from MediaCore)',
        installPaths: {
            windows: ['%PROGRAMFILES%\\Adobe\\Common\\Plugins\\7.0\\MediaCore'],
            macos: ['/Library/Application Support/Adobe/Common/Plugins/7.0/MediaCore'],
        },
        hostVersions: [
            {
                app: 'Premiere Pro',
                min: 'CC 2015 (9.0)',
                technology: 'AE Effect SDK subset',
                notes: 'Premiere is 64-bit only; AE-style effects load in both AE and Premiere from MediaCore.',
            },
        ],
        notes: [
            'Premiere does not support SmartFX; implement the non-smart PF_Cmd_RENDER path.',
            'PPro prefers BGRA pixel order; handle negative rowbytes.',
            'The Premiere SDK is license-gated and cannot be vendored in this repository.',
        ],
    },
};

export function getCompatibility(target: PluginTarget): TargetCompatibility {
    return COMPATIBILITY[target];
}
