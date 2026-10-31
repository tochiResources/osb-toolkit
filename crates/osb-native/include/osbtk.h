/*
 * osbTK C ABI - the surface every language binding drives (section 5.13, M14i).
 *
 * Generated with `cbindgen` from `crates/osb-native`; do not edit by hand.
 * Coverage: mapset (`.osu` parse), storyboard (`.osb` import), audio (FFT) and
 * storyboarding3d (scene bake); new operations are added to the JSON bridge
 * (`osbtk_exec`) alone. Release every returned `char *` with `osbtk_string_free`.
 */

#ifndef OSBTK_H
#define OSBTK_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An opaque handle to a live runtime: the effect context (beatmap, audio, RNG)
 * plus the storyboard being built.
 */
typedef struct OsbTKRuntime OsbTKRuntime;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * The runtime version string.
 */
char *osbtk_version(void);

/**
 * Bakes a `Bake3dInput` JSON into baked command-span JSON.
 *
 * # Safety
 *
 * `input_json` must be a NUL-terminated UTF-8 C string, or null (which returns
 * an error JSON). The result must be released with [`osbtk_string_free`].
 */
char *osbtk_bake3d(const char *input_json);

/**
 * Runs a JSON request (`{"op": "...", ...}`) against the runtime, returning JSON.
 *
 * # Safety
 *
 * `request_json` must be a NUL-terminated UTF-8 C string, or null (which returns
 * an error JSON). The result must be released with [`osbtk_string_free`].
 */
char *osbtk_exec(const char *request_json);

/**
 * Releases a string returned by any `osbtk_*` function.
 *
 * # Safety
 *
 * `ptr` must be a pointer previously returned by an `osbtk_*` function and not
 * already freed; null is ignored.
 */
void osbtk_string_free(char *ptr);

/**
 * Creates a runtime handle (storyboard + context) of the given resolution.
 */
struct OsbTKRuntime *osbtk_runtime_new(double width, double height);

/**
 * Releases a runtime handle created by [`osbtk_runtime_new`].
 *
 * # Safety
 *
 * `runtime` must be a pointer returned by [`osbtk_runtime_new`] and not already
 * freed; null is ignored.
 */
void osbtk_runtime_free(struct OsbTKRuntime *runtime);

/**
 * Runs a JSON request against a runtime handle (layers/sprites/commands/sampling).
 *
 * # Safety
 *
 * `runtime` must be a live pointer from [`osbtk_runtime_new`]; `request_json`
 * must be a NUL-terminated UTF-8 C string or null. Free the result with
 * [`osbtk_string_free`].
 */
char *osbtk_runtime_exec(struct OsbTKRuntime *runtime, const char *request_json);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* OSBTK_H */
