# Error codes

Error handling builds on the shared `@tochii/errors` package. The base class,
guards and serialization are reused; osbTK adds its own stable codes
under the `OSB_` prefix (classified by `osbTKCategoryOf`).

Import from `@tochii/osb-toolkit-shared/errors` so the code set stays in
one place.

| Code                        | Category  | Meaning                                        |
| --------------------------- | --------- | ---------------------------------------------- |
| `OSB_PARSE_ERROR`           | parse     | The effect script could not be parsed.         |
| `OSB_INVALID_SCRIPT`        | parse     | Valid C#, but not a usable effect script.      |
| `OSB_UNSUPPORTED_CONSTRUCT` | transpile | A C# construct has no supported transpilation. |
| `OSB_TRANSPILE_ERROR`       | transpile | The C# and JS transpilation failed.            |
| `OSB_GENERATION_ERROR`      | generate  | A target plug-in could not be generated.       |
| `OSB_UNSUPPORTED_TARGET`    | generate  | The requested target is unknown.               |
| `OSB_INVALID_OPTION`        | option    | An option passed to the converter is invalid.  |
| `OSB_BUILD_ERROR`           | build     | A native plug-in build failed.                 |
| `OSB_SDK_NOT_FOUND`         | sdk       | A required host SDK could not be located.      |
| `OSB_IO_ERROR`              | io        | A filesystem operation failed.                 |

## Classes

| Class                       | Default code                |
| --------------------------- | --------------------------- |
| `ParseError`                | `OSB_PARSE_ERROR`           |
| `InvalidScriptError`        | `OSB_INVALID_SCRIPT`        |
| `UnsupportedConstructError` | `OSB_UNSUPPORTED_CONSTRUCT` |
| `TranspileError`            | `OSB_TRANSPILE_ERROR`       |
| `GenerationError`           | `OSB_GENERATION_ERROR`      |
| `UnsupportedTargetError`    | `OSB_UNSUPPORTED_TARGET`    |
| `OptionError`               | `OSB_INVALID_OPTION`        |
| `BuildError`                | `OSB_BUILD_ERROR`           |
| `SdkError`                  | `OSB_SDK_NOT_FOUND`         |
| `IOError`                   | `OSB_IO_ERROR`              |

## Example

```ts
import { osbTKError, isosbTKError, ParseError } from '@tochii/osb-toolkit-shared/errors';

try {
    // ...
} catch (error) {
    if (isosbTKError(error)) {
        console.error(error.osbCode, error.osbCategory, error.message);
    }
}
```
