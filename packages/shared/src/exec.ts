import { spawn } from 'node:child_process';

import type { CommandResult, RunCommandOptions } from './types/exec.js';

export async function runCommand(
    command: string,
    args: string[] = [],
    options: RunCommandOptions = {}
): Promise<CommandResult> {
    return new Promise((resolvePromise, reject) => {
        const child = spawn(command, args, {
            cwd: options.cwd,
            env: { ...process.env, ...options.env },
            shell: options.shell ?? process.platform === 'win32',
        });
        let stdout = '';
        let stderr = '';
        child.stdout?.on('data', chunk => {
            stdout += String(chunk);
        });
        child.stderr?.on('data', chunk => {
            stderr += String(chunk);
        });
        child.on('error', reject);
        child.on('close', code => {
            resolvePromise({ command, args, code: code ?? 0, stdout, stderr });
        });
    });
}

export async function hasCommand(
    command: string,
    versionArgs: string[] = ['--version']
): Promise<boolean> {
    try {
        const result = await runCommand(command, versionArgs);
        return result.code === 0;
    } catch {
        return false;
    }
}

export function isDarwin(): boolean {
    return process.platform === 'darwin';
}

export function isWindows(): boolean {
    return process.platform === 'win32';
}
