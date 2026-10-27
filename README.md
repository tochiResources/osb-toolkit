# osbTK

Convert **effect scripts** (`.cs`) into native plug-ins for
**After Effects**, **DaVinci Resolve** and **Premiere Pro**.

It is a Yarn v4 monorepo (same tooling baseline as `@tochii/aetoosb`: turbo, biome,
commitlint, tsdown, vitest, zod) plus a cross-platform **Tauri v2** desktop app and a
**pure web** UI (Vite 8 + React + CSS Modules).

## Layout

```
osb-toolkit/
â”œâ”€â”€ packages/
â”‚   â”œâ”€â”€ shared/           @tochii/osb-toolkit-shared        shared types, errors, IR, compatibility data
â”‚   â”œâ”€â”€ core/             @tochii/osb-toolkit-core          C# parser + transpiler + generators
â”‚   â”œâ”€â”€ after-effects/    @tochii/osb-toolkit-to-after-effects   PF_ Effect SDK
â”‚   â”œâ”€â”€ davinci-resolve/  @tochii/osb-toolkit-to-davinci-resolve OpenFX 1.4
â”‚   â””â”€â”€ premiere-pro/     @tochii/osb-toolkit-to-premiere-pro    AE PF_ SDK + PrSDK
â”œâ”€â”€ apps/
â”‚   â”œâ”€â”€ frontend/         @tochii/osb-toolkit-web          pure web UI (Vite 8, React, CSS Modules)
â”‚   â””â”€â”€ client/           @tochii/osb-toolkit-client       Tauri v2 desktop app (uses the web UI)
â””â”€â”€ docs/                 @tochii/osb-toolkit-docs         VitePress documentation
```

Every code workspace keeps its type declarations in `src/types/*`; runtime
modules and the package barrel import/re-export them from there. `tsc` never
emits â€” `tsdown` produces all `.js`/`.d.ts` into `dist`.

Each program package has a `src/build/` folder with the tooling needed to compile a
plug-in for that host.

## Quick start

```bash
corepack enable
yarn install
yarn typecheck
yarn test
yarn build

# web UI
yarn dev:web

# desktop app (requires Rust + the Tauri prerequisites)
yarn dev:client
```

## Converting a script

```bash
# After Effects (default target)
yarn workspace @tochii/osb-toolkit-core cli packages/core/examples/Background.cs -t after-effects -o dist

# other targets
yarn workspace @tochii/osb-toolkit-to-davinci-resolve cli packages/core/examples/Background.cs
yarn workspace @tochii/osb-toolkit-to-premiere-pro cli packages/core/examples/Background.cs
```

The converter:

1. **Parses** the C# effect class (`[Configurable]` fields, `[Group]`/`[Description]`
   attributes, the `Generate()` body and helper methods).
2. **Transpiles** the C# subset to the target language (ES3-safe `foreach`, casts,
   LINQ helpers, the effect-script API surface, string interpolation, â€¦).
3. **Generates** a target-specific plug-in project and build files.

## After Effects

The After Effects target produces a native **PF\_ Effect** plug-in:

- `<Name>/native/<Name>.cpp` â€” a real effect that declares one parameter per
  `[Configurable]` and composites the storyboard into the output layer.
- `<Name>/native/` â€” `CMakeLists.txt`, `<Name>PiPL.r` and a header.

Build the native plug-in:

```bash
yarn workspace @tochii/osb-toolkit-to-after-effects cli packages/core/examples/Background.cs --build --sdk <path-to-AfterEffectsSDK>/Examples
```

Set `AE_SDK_DIR` to the SDK `Examples` directory (containing `Headers/`, `Util/`,
`Resources/`). The default search also looks for `AfterEffectsSDK_26.5_win/Examples`.

## Compatibility notes

Per-host compatibility (SDK versions, architectures, install paths and gotchas) lives
in `@tochii/osb-toolkit-shared` and `compat.ts` and is surfaced in the web UI. Highlights:

- **After Effects** â€” PF_ Effect SDK; `.aex` (Windows) / `.plugin` (macOS); CC 2014+.
- **DaVinci Resolve** â€” OpenFX 1.4; `.ofx.bundle`; Resolve 14+; Resolve is a strict
  OFX host (never mutate parameter UI in the constructor).
- **Premiere Pro** â€” AE `PF_` effect ABI + PrSDK; `.aex`; 64-bit only; no SmartFX.

## License

BSD-3-Clause. See [`LICENSE`](./LICENSE) and [`THIRD-PARTY-NOTICES.md`](./THIRD-PARTY-NOTICES.md)
(the bundled OpenFX SDK headers are BSD-3-Clause).
