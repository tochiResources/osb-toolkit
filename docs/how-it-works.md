# How it works

Conversion and rendering run on the Rust engine (`crates/osb-runtime`), driven by
the converter in `@tochii/osb-toolkit-core`.

## 1. Parse

`parseEffectScript(source)` reads the C# class:

- the `namespace`, class name and base class (`StoryboardObjectGenerator`);
- every `[Configurable]` field, with its `[Group]` / `[Description]` attributes,
  type and default value (numbers, strings, booleans, `Vector2`, enum members);
- the `Generate()` body and any helper methods.

The result is an `EffectScript` object (see `src/types/ir.ts` in
`@tochii/osb-toolkit-shared`).

## 2. Transpile

The C# subset is transpiled to **Rust**, mapping the effect-script API onto the
runtime: layer handles, `Beatmap`, FFT, LINQ helpers, math, string
interpolation and so on. Constructs it cannot translate are reported as
diagnostics instead of failing silently. See
[Supported C# subset](/reference/csharp-subset).

## 3. Generate

A target generator turns the parsed and transpiled script into a Rust plug-in
crate plus its build files. The host adapters (After Effects `PF_`, Premiere Pro,
OpenFX) are built and packaged by the sibling `osb-to-plugin` repo from the
generated crate.

## 4. Run and bake

The generated script implements the runtime `Effect` trait. `generate()` builds
a `Storyboard` once; render time samples every sprite's command timelines at the
host's current time and composites the visible sprites. 3D scenes are baked to
2D command spans, and `.osb` import/export plus optimization are available as
SDK operations.

## The SDK layer

`crates/osb-api` declares the wire contract (one tag per class, one
`operationId` per operation); `yarn openapi` emits the OpenAPI document and the
per-language clients, and a small interceptor routes each client to the native
JSON bridge (`osbtk_exec` / `osbtk_runtime_exec`).
