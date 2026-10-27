import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, dirname, join } from 'node:path';

/**
 * Runs the Roslyn syntax linter (`tools/csharp-lint`) over a consumer's C#
 * effect script as the first stage of conversion, so syntax errors are caught
 * before transpilation. The tool is a small .NET console app; when `dotnet` or
 * the tool is unavailable the lint is skipped (returns `[]`) rather than failing
 * conversion, keeping the converter usable without the .NET SDK.
 */
export interface CSharpLintDiagnostic {
    line: number;
    column: number;
    severity: 'error' | 'warning' | 'info';
    id: string;
    message: string;
}

const cache = new Map<string, CSharpLintDiagnostic[]>();

const LINE = /^(.*)\((\d+),(\d+)\): (error|warning|info): ([A-Za-z0-9]+): (.*)$/;

/** Locates the lint tool project, honouring `OSB_CSHARP_LINT`. */
function findToolProject(): string | undefined {
    const override = process.env.OSB_CSHARP_LINT;
    if (override && existsSync(join(override, 'csharp-lint.csproj'))) return override;

    let dir = process.cwd();
    for (let depth = 0; depth < 8; depth += 1) {
        const candidate = join(dir, 'tools', 'csharp-lint');
        if (existsSync(join(candidate, 'csharp-lint.csproj'))) return candidate;
        const parent = dirname(dir);
        if (parent === dir) break;
        dir = parent;
    }
    return undefined;
}

/** Parses the lint tool's `path(line,col): severity: ID: message` output. */
export function parseLintOutput(stdout: string): CSharpLintDiagnostic[] {
    const diagnostics: CSharpLintDiagnostic[] = [];
    for (const line of stdout.split(/\r?\n/)) {
        const match = LINE.exec(line);
        if (!match) continue;
        diagnostics.push({
            line: Number(match[2]),
            column: Number(match[3]),
            severity: match[4] as CSharpLintDiagnostic['severity'],
            id: match[5] as string,
            message: match[6] as string,
        });
    }
    return diagnostics;
}

/** Lints a C# source, returning syntax diagnostics (empty when unavailable). */
export function lintCSharpSource(source: string, fileName?: string): CSharpLintDiagnostic[] {
    const cached = cache.get(source);
    if (cached) return cached;

    const project = findToolProject();
    if (!project) return [];

    const dir = mkdtempSync(join(tmpdir(), 'osb-csharp-lint-'));
    // `fileName` may be an absolute path (the CLI passes one); keep only the base.
    const safeName = fileName ? basename(fileName) : 'Effect.cs';
    const file = join(dir, safeName.endsWith('.cs') ? safeName : 'Effect.cs');
    writeFileSync(file, source, 'utf8');

    // Prefer the built DLL (fast, no build noise); fall back to `dotnet run`.
    const dll = join(project, 'bin', 'Release', 'net10.0', 'osb-csharp-lint.dll');
    const args = existsSync(dll)
        ? [dll, file]
        : ['run', '--project', project, '-c', 'Release', '--', file];
    const result = spawnSync('dotnet', args, { encoding: 'utf8' });
    if (result.error || typeof result.stdout !== 'string') return [];

    // Ignore anything not about the file we linted (e.g. build output).
    const relevant = result.stdout
        .split(/\r?\n/)
        .filter(line => line.includes(file))
        .join('\n');
    const diagnostics = parseLintOutput(relevant);
    cache.set(source, diagnostics);
    return diagnostics;
}
