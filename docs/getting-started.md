# Getting started

osbTK is the shared Rust engine and SDK family behind the osu! storyboard
toolkit.

## Install

```bash
corepack enable
yarn install
```

## Verify

```bash
yarn typecheck
yarn test
```

## Build

```bash
yarn build          # TypeScript packages
yarn build:native   # the osb-native C ABI
yarn openapi        # regenerate the contract and every SDK
```

## Use an SDK

```ts
import { Core, osb } from '@tochii/osb-sdk-js';

const beatmap = await Core.parsing.parseOsu({ text });
const report = await osb.diagnostics.diagnoseOsu({ text });
```

The same surface is generated for C#, Python, Go, Java, C++ and Rust.

## Next

- [How it works](/how-it-works)
- [SDK reference](/reference/api/)
