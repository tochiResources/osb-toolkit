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
generated clients are normal OpenAPI clients; a small per-language interceptor
routes them to the native JSON bridge (`osbtk_exec` / `osbtk_runtime_exec`).

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
