export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
    level: LogLevel;
    message: string;
    scope?: string;
    detail?: unknown;
    time: number;
}

export interface Logger {
    debug(message: string, detail?: unknown): void;
    info(message: string, detail?: unknown): void;
    warn(message: string, detail?: unknown): void;
    error(message: string, detail?: unknown): void;
    child(scope: string): Logger;
}

export type LogSink = (entry: LogEntry) => void;

export interface CreateLoggerOptions {
    level?: LogLevel;
    scope?: string;
    sink?: LogSink;
}
