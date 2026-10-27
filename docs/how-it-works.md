# How it works

Conversion has three stages, all in `@tochii/osb-toolkit-core`.

## 1. Parse

`parseEffectScript(source)` reads the C# class:

- the `namespace`, class name and base class (`StoryboardObjectGenerator`);
- every `[Configurable]` field, with its `[Group]` / `[Description]` attributes,
  type and default value (numbers, strings, booleans, `Vector2`, enum members);
- the `Generate()` body and any helper methods.

The result is an `EffectScript` object (see `src/types/ir.ts` in
`@tochii/osb-toolkit-shared`).

## 2. Transpile

`transpileCSharp(body)` rewrites the C# subset to JavaScript:

- declarations → `let`/`const`;
- `foreach` → an ES3-safe index loop;
- casts → `Math.trunc(...)` / `Number(...)`;
- the effect-script API (`GetLayer` → `getLayer`, `Beatmap` → `beatmap`,
  LINQ helpers, `Math` mapping, string interpolation, …).

Constructs it cannot translate are reported as diagnostics instead of failing
silently. See [Supported C# subset](/reference/csharp-subset).

## 3. Generate

A target generator turns the parsed + transpiled script into a set of files
(`GeneratedFile[]`): plug-in source, project/build files, a manifest and a README.
Each program package owns its generator and its `src/build` tooling.

## The After Effects pipeline

The After Effects target also embeds a small **effect runtime** in the emitted
ExtendScript. The script records the storyboard (layers, sprites, commands) and
materialises it into a composition with keyframed layers. The native AEGP plug-in
executes that same script from a menu command.

Because After Effects ExtendScript is ES3, the generated `.jsx` is down-levelled
with Babel before it is written.
