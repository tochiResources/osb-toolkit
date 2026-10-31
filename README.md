# osu! Storyboarding Toolkit

> Still in early development phase.
>
> - Most crates in [crates/](./crates) are well tested; `osb-parser-ae`, `osb-sdk` and `osb-wasm` are still early.
> - The architecture can be seen in the mermaid files at [docs/architecture/](./docs/architecture).

SDKs that provides useful tools for osu! storyboarding development in `C#`, `Java`, `JavaScript`/`TypeScript`, `Go`, `Rust`, `Python`, and `C++`.

See the full docs at [osb-toolkit.tochi.moe/docs](https://osb-toolkit.tochi.moe/docs).

> Docs not live yet.

## Projects using osbTK

- [AeToOsb](https://github.com/tochiResources/AeToOsb)
- [AeBrew](https://github.com/tochiResources/AeBrew)

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
([bindings/openapi/osbtk.openapi.json](/bindings/openapi/osbtk.openapi.json)) emitted by [crates/osb-api](/crates/osb-api).

The SDK clients uses a small per-language interceptor that routes them to the native JSON bridge (`osbtk_exec` / `osbtk_runtime_exec`).

Go has no overloading, so it's SDK client stay options-only unlike the other SDKs.

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

yarn w:docs dev        # run the documentation site
```

## License

MIT. See [LICENSE](/LICENSE).
