export interface RunCommandOptions {
    cwd?: string;
    env?: NodeJS.ProcessEnv;
    shell?: boolean;
}

export interface CommandResult {
    command: string;
    args: string[];
    code: number;
    stdout: string;
    stderr: string;
}
