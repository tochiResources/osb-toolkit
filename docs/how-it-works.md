# How it works

The Rust workspace is the source of truth: `crates/osb-api` declares the wire
contract, the runtime crates implement it, and every SDK is generated or thin.

## 1. Contract

`crates/osb-api` declares the wire contract (one tag per class, one
`operationId` per operation). `yarn openapi` emits the OpenAPI document and the
per-language clients and models, and a test in `crates/osb-api/src/doc.rs`
enforces that every `operationId` is scoped to its tag and matches its path
segment.

## 2. Engine

`crates/osb-runtime` owns the storyboard model: layers, sprites, animations,
timelines, loops and triggers, 3D scene baking, and render-time sampling.
`crates/osb-core` provides math, I/O and curve primitives. The capability crates
add image processing (`osb-image`, `osb-image-gen`), media (`osb-media`) and
After Effects script parsing (`osb-parser-ae`).

## 3. Native bridge

`crates/osb-native` exposes the C ABI (`osbtk_exec` / `osbtk_runtime_exec`) over
JSON, and `crates/osb-wasm` exposes the same engine to JavaScript and the
browser. Both ship as the surfaces the generated SDKs drive.

## 4. SDK layer

The generated clients install a small interceptor that routes each call to the
native JSON bridge. The stateless surface is a single `Core` class; the
runtime/handle-bound surfaces are `Generator`, `Layer`, `Sprite` and friends.
