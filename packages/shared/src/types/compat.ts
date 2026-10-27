import type { HostPlatform, PluginTarget } from './targets.js';

export interface HostVersionRange {
    app: string;
    min: string;
    max?: string;
    technology: string;
    notes: string;
}

export interface TargetCompatibility {
    target: PluginTarget;
    sdk: string;
    apiVersion: string;
    platforms: HostPlatform[];
    architectures: string[];
    artifact: string;
    installPaths: Partial<Record<HostPlatform, string[]>>;
    hostVersions: HostVersionRange[];
    notes: string[];
}
