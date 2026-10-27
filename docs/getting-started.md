# Getting started

osbTK turns an **effect script** (`.cs`) into a
native plug-in for one of three hosts:

| Target          | Technology         | Artifact      |
| --------------- | ------------------ | ------------- |
| After Effects   | PF_ Effect SDK     | `.aex`        |
| DaVinci Resolve | OpenFX 1.4         | `.ofx.bundle` |
| Premiere Pro    | AE PF_ SDK + PrSDK | `.aex`        |

## Install

```bash
corepack enable
yarn install
yarn build
```

## Convert a script

The core package ships a CLI:

```bash
yarn workspace @tochii/osb-toolkit-core cli path/to/Background.cs -t after-effects -o dist
```

`-t` selects the target (`after-effects`, `davinci-resolve`, `premiere-pro`). The
output directory receives a folder named after the effect containing the
generated sources, build files and a README.

## Use the web UI

```bash
yarn dev:web
```

Open the printed URL, drop a `.cs` file, pick a target and press **Convert**. All
conversion happens in the browser â€” no upload.

## Use the desktop app

```bash
yarn dev:client
```

The Tauri app embeds the same web UI and adds native file access.

## Next

- [How it works](/how-it-works)
- [CLI reference](/reference/cli)
