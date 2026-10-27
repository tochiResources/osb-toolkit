import { spawnSync } from 'node:child_process';

import { describe, expect, it } from 'vitest';

import { lintCSharpSource, parseLintOutput } from '../src/csharp/lint.js';

describe('csharp lint output parsing', () => {
    it('parses the tool output into diagnostics', () => {
        const stdout = [
            "C:/tmp/Effect.cs(3,17): error: CS1525: Invalid expression term ';'",
            'C:/tmp/Effect.cs(5,9): warning: CS0168: The variable is declared but never used',
            'csharp-lint: 1 file(s), 1 error(s)',
        ].join('\n');
        const diagnostics = parseLintOutput(stdout);
        expect(diagnostics).toHaveLength(2);
        expect(diagnostics[0]).toMatchObject({
            line: 3,
            column: 17,
            severity: 'error',
            id: 'CS1525',
        });
        expect(diagnostics[1]?.severity).toBe('warning');
    });
});

const dotnetAvailable = (() => {
    const result = spawnSync('dotnet', ['--version'], { encoding: 'utf8' });
    return !result.error && result.status === 0;
})();

// The pipeline stage shells out to the Roslyn tool; skipped without the .NET SDK.
describe.skipIf(!dotnetAvailable)('csharp lint pipeline stage (requires dotnet)', () => {
    it('accepts a valid effect script', () => {
        expect(lintCSharpSource('public class X { public void Generate() { } }')).toHaveLength(0);
    });

    it('reports a syntax error before transpilation', () => {
        const diagnostics = lintCSharpSource('public class X { void M() { int x = ; } }');
        expect(diagnostics.some(diagnostic => diagnostic.severity === 'error')).toBe(true);
    });
});
