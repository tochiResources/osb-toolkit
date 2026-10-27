# Supported C# subset

The transpiler targets the subset of C# used by effect scripts.
Anything outside it produces a diagnostic rather than failing silently.

## Supported

- Field / local declarations: `int`, `double`, `float`, `bool`, `string`, `var`,
  known types (`Vector2`, `OsbOrigin`, …).
- `foreach (var x in xs) { … }` → an ES3-safe index loop.
- `for`, `while`, `if`/`else`, `return`, `break`, `continue`.
- Casts: `(int)`/`(long)`/`(short)`/`(byte)` → `Math.trunc(...)`;
  `(float)`/`(double)` → `Number(...)`; `(bool)` → `Boolean(...)`.
- Numeric suffixes (`0.05f`, `100d`) are stripped; `_` separators removed.
- Interpolated strings `$"bar {i}"` → template literals.
- Lambdas (`x => …`, `(a, b) => { … }`).
- The effect-script API: `GetLayer`, `Beatmap`, `GetFft`, `Random`, `AudioDuration`,
  `CancellationToken`, `Log`, `Assert`, `GetMapsetBitmap`, `GetProjectBitmap`.
- LINQ helpers on arrays: `FirstOrDefault`, `LastOrDefault`, `First`, `Last`,
  `Where`, `Select`, `Sum`, `Any`, `All`, `OrderBy`, `ToArray`, `ElementAt`,
  `Contains`, `Min`, `Max`.
- `Math.Ceiling`/`Log10`/`Clamp`/`Sign`/`Truncate` and `Math.*`.
- `new Vector2(...)` → `new Vec2(...)`; `new KeyframedValue<T>[n]` →
  `makeArray(n, () => new KeyframedValue())`.

## Reported as unsupported

- `switch`, `try`/`catch`, `lock`, `unsafe`, `fixed`, `goto`, `yield`.
- `ref` / `out` parameters are dropped.
- `is` / `as` expressions.

Run a conversion and check the **diagnostics** in the web UI or the CLI output to
see exactly what was skipped.
