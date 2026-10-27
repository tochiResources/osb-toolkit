# CLI

`@tochii/osb-toolkit-core` ships the converter as `osbtk-fx-convert`.

```bash
yarn workspace @tochii/osb-toolkit-core cli <Effect.cs...> [options]
```

| Option                           | Description                                                                      |
| -------------------------------- | -------------------------------------------------------------------------------- |
| `-t, --target <target>`          | `after-effects` \| `davinci-resolve` \| `premiere-pro` (default `after-effects`) |
| `-o, --out <dir>`                | Output directory (default `dist`)                                                |
| `--name <name>`                  | Override the effect name                                                         |
| `--id <id>`                      | Override the reverse-DNS id                                                      |
| `--category <name>`              | Override the category                                                            |
| `--width <px>` / `--height <px>` | Composition size                                                                 |
| `--lib <path...>`                | External library file(s) or library folder(s)                                    |
| `--bundle` / `--separate`        | Bundle all effects into one crate, or one crate per effect                       |
| `--combine-parameters`           | Merge bundled parameters by `[Group]`                                            |
| `--json`                         | Machine-readable output                                                          |
| `-q, --quiet`                    | Only warnings and errors                                                         |

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
