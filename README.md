# osu! Storyboarding Toolkit

SDKs that provides useful tools for osu! storyboarding development in C#, Java, JavaScript/TypeScript, Go, Rust, Python, and C++.

See the full docs at [osb-toolkit.tochi.moe/docs](https://osb-toolkit.tochi.moe/docs).

## Projects using osbTK

- **AeToOsb:** https://github.com/tochiResources/AeToOsb
- **AeBrew:** https://github.com/tochiResources/AeBrew

## SDKs and bindings

| Language              | Package                               |
| --------------------- | ------------------------------------- |
| JavaScript/TypeScript | `@tochii/osb-sdk-js` (`packages/sdk`) |
| C#                    | `tochiRes.osbTK` (`bindings/csharp`)  |
| Python                | `osbtk` (`bindings/python`)           |
| Go                    | `bindings/go`                         |
| Java                  | `moe.tochi.osbtk` (`bindings/java`)   |
| C++                   | `bindings/cpp` (header-only)          |
| Rust                  | `osb-sdk` (`crates/osb-sdk`)          |

The SDKs are generated from one OpenAPI 3.1 contract
(`bindings/openapi/osbtk.openapi.json`) emitted by `crates/osb-api`. The
generated clients are clean, `<X>Options`-based surfaces over a small
per-language interceptor that routes them to the native JSON bridge
(`osbtk_exec` / `osbtk_runtime_exec`). Every options-based method also accepts
positional values, with overloads ordered by ascending parameter count; the
options forms come last. Single-field result wrappers are unwrapped
(`OkResult` becomes `void`, `BoolResult` a boolean, and so on).

Go has no overloading, so its clean clients stay options-only. The composed Go
surface is `osbtk/clientkit`: `clientkit.New()` returns the stateless clean
clients and `clientkit.NewRuntime` resolves handle-bound `Layer`/`Sprite`
values. The root `osbtk` facades keep the effect-runtime conveniences; its
bridge constructor is `NewClient` (the generated transport type stays
`APIClient`, which does not collide with any convenience name).

## Development

```bash
corepack enable
yarn install

yarn typecheck
yarn test              # JS/TS suites
yarn test:bindings     # C#, C++, Python, Go, Java and wasm

yarn build             # all TypeScript packages
yarn build:native      # the osb-native C ABI
yarn openapi           # regenerate the contract and every SDK

yarn w:docs dev        # the documentation site
```

## License

BSD-3-Clause. See [LICENSE](/LICENSE).
