using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;

namespace osbTK.CSharpLint;

/// <summary>
/// Syntax-level C# lint for effect scripts: parses each file and reports
/// diagnostics as <c>path(line,col): severity: ID: message</c>, exiting non-zero
/// on errors. Effect scripts reference scripting types that are not
/// available here, so this pass is deliberately syntax-only; the converter
/// supplies the semantic pass.
/// </summary>
internal static class Program
{
    private static int Main(string[] args)
    {
        string[] roots = args.Length > 0 ? args : ["."];
        string[] files = roots
            .SelectMany(Expand)
            .Where(IsCSharp)
            .Distinct()
            .OrderBy(file => file, StringComparer.Ordinal)
            .ToArray();

        if (files.Length == 0)
        {
            Console.Error.WriteLine("csharp-lint: no .cs files found");
            return 0;
        }

        int errors = 0;
        foreach (string file in files)
        {
            string text = File.ReadAllText(file);
            SyntaxTree tree = CSharpSyntaxTree.ParseText(text, path: file);
            foreach (Diagnostic diagnostic in tree.GetDiagnostics())
            {
                if (diagnostic.Severity < DiagnosticSeverity.Warning)
                {
                    continue;
                }

                if (diagnostic.Severity == DiagnosticSeverity.Error)
                {
                    errors++;
                }

                FileLinePositionSpan span = diagnostic.Location.GetLineSpan();
                Console.WriteLine(
                    $"{file}({span.StartLinePosition.Line + 1},{span.StartLinePosition.Character + 1}): "
                        + $"{diagnostic.Severity.ToString().ToLowerInvariant()}: "
                        + $"{diagnostic.Id}: {diagnostic.GetMessage()}"
                );
            }
        }

        Console.WriteLine($"csharp-lint: {files.Length} file(s), {errors} error(s)");
        return errors == 0 ? 0 : 1;
    }

    private static IEnumerable<string> Expand(string path)
    {
        if (File.Exists(path))
        {
            yield return path;
            yield break;
        }

        if (!Directory.Exists(path))
        {
            yield break;
        }

        foreach (string file in Directory.EnumerateFiles(path, "*.cs", SearchOption.AllDirectories))
        {
            string relative = Path.GetRelativePath(path, file);
            if (relative.StartsWith("bin", StringComparison.Ordinal)
                || relative.StartsWith("obj", StringComparison.Ordinal)
                || relative.Contains($"{Path.DirectorySeparatorChar}bin{Path.DirectorySeparatorChar}")
                || relative.Contains($"{Path.DirectorySeparatorChar}obj{Path.DirectorySeparatorChar}"))
            {
                continue;
            }

            yield return file;
        }
    }

    private static bool IsCSharp(string file) =>
        file.EndsWith(".cs", StringComparison.OrdinalIgnoreCase);
}
