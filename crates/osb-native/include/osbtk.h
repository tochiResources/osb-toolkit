/*
 * osbTK C ABI - the surface every language binding drives (section 5.13, M14i).
 *
 * Generated with `cbindgen` from `crates/osb-native` (see `cbindgen.toml`); do
 * not edit by hand. Coverage: mapset (`.osu` parse), storyboard (`.osb` import),
 * audio (FFT) and storyboard3d (scene bake); new operations are added to the
 * JSON bridge (`osbtk_exec`) alone.
 *
 * Strings are UTF-8, NUL-terminated, and owned by the caller: release every
 * `char *` returned here with `osbtk_string_free`.
 */
#ifndef OSBTK_H
#define OSBTK_H

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque handle to a live runtime (storyboard + context). */
typedef struct OsbtkRuntime OsbtkRuntime;

/* The runtime version string. */
char *osbtk_version(void);

/* Bakes a `Bake3dInput` JSON into baked command-span JSON.
 * `input_json` may be NULL (returns an error JSON). */
char *osbtk_bake3d(const char *input_json);

/* Runs a JSON request (`{"op": "...", ...}`) against the runtime; returns JSON.
 * `request_json` may be NULL (returns an error JSON). */
char *osbtk_exec(const char *request_json);

/* Creates a runtime handle of the given resolution. Free with osbtk_runtime_free. */
OsbtkRuntime *osbtk_runtime_new(double width, double height);

/* Releases a runtime handle. NULL is ignored. */
void osbtk_runtime_free(OsbtkRuntime *runtime);

/* Runs a JSON request against a runtime handle (layers/sprites/commands/sampling). */
char *osbtk_runtime_exec(OsbtkRuntime *runtime, const char *request_json);

/* Releases a string returned by any `osbtk_*` function. NULL is ignored. */
void osbtk_string_free(char *ptr);

#ifdef __cplusplus
}
#endif

#endif /* OSBTK_H */
