# osbTK

**osu! Storyboarding Toolkit** - a Rust-first engine and SDK family for building
osu! storyboards, converting Storybrew/AeBrew-style C# effect scripts, and
driving the same storyboard model from TypeScript, C#, Python, Go, Java, C++,
Rust, a CLI, a headless server and the web/desktop UIs.

The host-facing pieces (the After Effects / Premiere Pro / DaVinci Resolve
plug-ins and the Tauri desktop app) live in the sibling `osb-to-plugin` repo and
consume this repo's runtime and SDKs.

## What it does

An osbTK effect script is a C# class that derives from
`StoryboardObjectGenerator` and overrides `Generate()`. It builds a storyboard
from layers, sprites and time-based commands (move/scale/rotate/fade/color/
additive/loops/triggers), driven by the beatmap, the song's FFT and
configurables. osbTK:

- **parses** that C# subset and **transpiles** it to the Rust engine;
- **runs and bakes** the storyboard (2D + 3D scenes, keyframes, timelines);
- **renders** it to pixels, exports `.osb`, and packages host plug-ins;
- exposes the whole surface as **typed SDKs** in seven languages.

Rust is the single source of truth: the engine, the wire contract, the API code
and the docs are written once, and the SDKs/bindings are generated or thin.

## Layout

```
crates/       the Rust workspace: osb-core -> osb-runtime -> osb-api -> osb-native,
              plus osb-server, osb-cli, osb-sdk, osb-wasm, osb-image(-gen),
              osb-media and osb-parser-ae
bindings/     generated language SDKs: C#, C++, Go, Java, Python (+ the checked-in OpenAPI spec)
packages/     TypeScript workspaces: shared, core (converter), native (wasm), sdk
docs/         VitePress site + architecture diagrams
scripts/      build, clean, OpenAPI and diagram tooling
tools/        C# linter and the AE export/recorder helpers
```

## Getting started

```bash
corepack enable
yarn install

yarn typecheck
yarn test              # JS/TS suites
yarn test:bindings     # C#, C++, Python, Go, Java and wasm

yarn build             # all TypeScript packages
yarn build:native      # the osb-native C ABI (release)
yarn openapi           # regenerate the contract and every SDK

yarn w:docs dev        # the VitePress site
```

## SDKs and bindings

| Language   | Package                               |
| ---------- | ------------------------------------- |
| TypeScript | `@tochii/osb-sdk-js` (`packages/sdk`) |
| C#         | `tochiRes.osbTK` (`bindings/csharp`)  |
| Python     | `osbtk` (`bindings/python`)           |
| Go         | `bindings/go`                         |
| Java       | `moe.tochi.osbtk` (`bindings/java`)   |
| C++        | `bindings/cpp` (header-only)          |
| Rust       | `osb-sdk` (`crates/osb-sdk`)          |

The SDKs are generated from one OpenAPI 3.1 contract
(`bindings/openapi/osbtk.openapi.json`) emitted by `crates/osb-api`. The
generated clients are normal OpenAPI clients; a small per-language interceptor
routes them to the native JSON bridge (`osbtk_exec` / `osbtk_runtime_exec`).

## Documentation

- `osbTK-ARCHITECTURE.md` - the authoritative engine/SDK architecture.
- `osbTK-PLAN.md` - milestones and status.
- `docs/` - the published VitePress site (`docs/architecture/*.mmd` diagrams).

## License

BSD-3-Clause. See `LICENSE` and `THIRD-PARTY-NOTICES.md` (the bundled OpenFX SDK
headers are BSD-3-Clause).
