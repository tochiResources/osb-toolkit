export type PluginTarget = 'after-effects' | 'davinci-resolve' | 'premiere-pro';

export type HostPlatform = 'windows' | 'macos' | 'linux';

export interface PluginTargetInfo {
    id: PluginTarget;
    label: string;
    npmPackage: string;
    hostApp: string;
    technology: string;
    artifact: string;
    platforms: HostPlatform[];
    requiresNativeToolchain: boolean;
    status: 'implemented' | 'scaffold';
}
