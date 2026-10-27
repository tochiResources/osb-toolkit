export type { SerializedError, TochiResErrorOptions } from '@tochii/errors';

export type { HostVersionRange, TargetCompatibility } from './compat.js';
export type { osbTKDomainErrorOptions, osbTKErrorOptions } from './errors.js';
export type { CommandResult, RunCommandOptions } from './exec.js';
export type { WriteFilesOptions, WrittenFile } from './fs.js';
export type {
    ConvertOptions,
    ConvertResult,
    Diagnostic,
    DiagnosticSeverity,
    EffectEnumValue,
    EffectField,
    EffectMethod,
    EffectParameter,
    EffectParameterType,
    EffectScript,
    GeneratedFile,
    OsbEasing,
    OsbLayer,
    OsbOrigin,
} from './ir.js';
export type { CreateLoggerOptions, LogEntry, Logger, LogLevel, LogSink } from './logger.js';
export type {
    CreatePluginSpecOptions,
    PluginParameter,
    PluginSpec,
    PluginVersion,
} from './plugin-spec.js';
export type { HostPlatform, PluginTarget, PluginTargetInfo } from './targets.js';
