# CLI

## Core CLI — `osbtk-fx-convert`

`@tochii/osb-toolkit-core` ships the generic converter.

```bash
yarn workspace @tochii/osb-toolkit-core cli <Effect.cs...> [options]
```

| Option                           | Description                                                                      |
| -------------------------------- | -------------------------------------------------------------------------------- |
| `-t, --target <target>`          | `after-effects` \| `davinci-resolve` \| `premiere-pro` (default `after-effects`) |
| `-o, --out <dir>`                | Output directory (default `dist`)                                                |
| `--name <name>`                  | Override the plug-in name                                                        |
| `--id <id>`                      | Override the reverse-DNS plug-in id                                              |
| `--category <name>`              | Override the category                                                            |
| `--width <px>` / `--height <px>` | Composition size                                                                 |
| `--json`                         | Machine-readable output                                                          |
| `-q, --quiet`                    | Only warnings and errors                                                         |

## Per-target CLIs

Each program package adds a thin CLI that writes the output and can build the
native plug-in:

```bash
yarn workspace @tochii/osb-toolkit-to-after-effects cli <Effect.cs> --build --sdk <AE SDK Examples>
yarn workspace @tochii/osb-toolkit-to-davinci-resolve cli <Effect.cs> --build --ofx-sdk <openfx>
yarn workspace @tochii/osb-toolkit-to-premiere-pro cli <Effect.cs> --artifact aex
```

## Programmatic use

```ts
import { convertEffectScript } from '@tochii/osb-toolkit-core';

const result = convertEffectScript(source, {
    target: 'after-effects',
    sourceFile: 'Background.cs',
});

for (const file of result.files) {
    console.log(file.path);
}
```

The After Effects package exposes an async variant that also writes the files to
disk:

```ts
import { convertToAfterEffects } from '@tochii/osb-toolkit-to-after-effects';

const result = await convertToAfterEffects(source, { sourceFile: 'Background.cs' });
```
