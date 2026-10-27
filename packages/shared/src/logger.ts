import type { CreateLoggerOptions, LogEntry, Logger, LogLevel, LogSink } from './types/logger.js';

const LEVEL_WEIGHT: Record<LogLevel, number> = { debug: 10, info: 20, warn: 30, error: 40 };

export function createLogger(options: CreateLoggerOptions = {}): Logger {
    const level = options.level ?? 'info';
    const scope = options.scope;
    const sink = options.sink ?? defaultSink;

    const emit = (entryLevel: LogLevel, message: string, detail?: unknown) => {
        if (LEVEL_WEIGHT[entryLevel] < LEVEL_WEIGHT[level]) return;
        sink({ level: entryLevel, message, scope, detail, time: Date.now() });
    };

    return {
        debug: (message, detail) => emit('debug', message, detail),
        info: (message, detail) => emit('info', message, detail),
        warn: (message, detail) => emit('warn', message, detail),
        error: (message, detail) => emit('error', message, detail),
        child: childScope =>
            createLogger({ level, scope: scope ? `${scope}:${childScope}` : childScope, sink }),
    };
}

function defaultSink(entry: LogEntry): void {
    const prefix = entry.scope ? `[${entry.scope}]` : '';
    const line = `${prefix} ${entry.message}`.trim();
    if (entry.level === 'error') {
        console.error(line, entry.detail ?? '');
    } else if (entry.level === 'warn') {
        console.warn(line, entry.detail ?? '');
    } else {
        console.log(line, entry.detail ?? '');
    }
}

export function createMemorySink(): { sink: LogSink; entries: LogEntry[] } {
    const entries: LogEntry[] = [];
    return { sink: entry => entries.push(entry), entries };
}
