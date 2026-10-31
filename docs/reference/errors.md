# Errors

The SDKs build on the shared [`@tochii/errors`](https://www.npmjs.com/package/@tochii/errors)
contract: every error carries a stable code, a coarse category, optional
structured context and an optional cause chain. The C# binding uses the matching
`TochiRes.Errors` NuGet package, so codes cross the language boundary unchanged.

## TypeScript

| Class           | Default code                              | Category     |
| --------------- | ----------------------------------------- | ------------ |
| `ResponseError` | derived from the response status (`Api*`) | `api`        |
| `FetchError`    | `TOCHIRES_API_UNAVAILABLE`                | `api`        |
| `RequiredError` | `TOCHIRES_VALIDATION_FAILED`              | `validation` |

`ResponseError` maps `400`, `401`, `403`, `404`, `408`, `409`, `422`, `429`,
`503` and `504` to their specific codes and falls back to `ApiBadRequest` or
`ApiUnavailable`.

```ts
import { ErrorCode, TochiResError } from '@tochii/errors';
import { ResponseError, osb } from '@tochii/osb-sdk-js';

try {
    await osb.audio.fft({ path: 'missing.wav' });
} catch (error) {
    if (error instanceof TochiResError) {
        console.error(error.code, error.category, error.message);
    }
    if (error instanceof ResponseError) {
        console.error(error.response.status);
    }
}
```

## C#

| Class                 | Code                          | Category |
| --------------------- | ----------------------------- | -------- |
| `osbTKException`      | `TOCHIRES_NATIVE_CALL_FAILED` | `native` |
| `Client.ApiException` | `TOCHIRES_API_BAD_REQUEST`    | `api`    |

Both derive from `TochiRes.Errors.TochiResException`, which exposes `Code`,
`Category` and `Context`.

```csharp
using TochiRes.Errors;
using tochiRes.osbTK;

try
{
    osb.Audio.Fft("missing.wav");
}
catch (TochiResException error)
{
    Console.Error.WriteLine($"{error.Code} ({error.Category}): {error.Message}");
}
```
