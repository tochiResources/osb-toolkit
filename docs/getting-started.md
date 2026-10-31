# Getting started

osbTK is the shared engine and SDK family behind the osu! storyboard toolkit.
The host plug-ins (After Effects, Premiere Pro, DaVinci Resolve) and the desktop
app live in the sibling `osb-to-plugin` repo and consume this repo's runtime.

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

## Convert a script

The converter CLI lives in the `osb-to-plugin` repo as `@tochii/osb-toolkit-core` (run this from that repo):

```bash
yarn workspace @tochii/osb-toolkit-core cli path/to/Background.cs -t after-effects -o dist
```

`-t` selects the target (`after-effects`, `davinci-resolve`, `premiere-pro`). It
emits the generated Rust crate and build files; the host adapters are built in
`osb-to-plugin`.

## Use an SDK

```ts
import { Core, osb } from '@tochii/osb-sdk-js';

const beatmap = await Core.parsing.parseOsu({ text });
const report = await osb.diagnostics.diagnoseOsu({ text });
```

The same surface is generated for C#, Python, Go, Java, C++ and Rust.

## Next

- [How it works](/how-it-works)
- [CLI reference](/reference/cli)
