//! The C ABI entry points. Each returns a NUL-terminated UTF-8 JSON string that
//! the caller releases with [`osbtk_string_free`]; `catch_unwind` keeps a panic
//! from unwinding across the boundary.

use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::bake::bake_to_json;
use crate::error_json;
use crate::exec::exec;
use crate::runtime::{runtime_exec, OsbtkRuntime};

fn into_c_string(value: String) -> *mut c_char {
    CString::new(value)
        .map(CString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

fn guarded<F: FnOnce() -> String>(f: F) -> *mut c_char {
    into_c_string(
        catch_unwind(AssertUnwindSafe(f))
            .unwrap_or_else(|_| error_json("panic at the FFI boundary")),
    )
}

/// # Safety
/// `ptr` must be null or a NUL-terminated UTF-8 C string valid for the call.
unsafe fn read(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("null input".into());
    }
    CStr::from_ptr(ptr)
        .to_str()
        .map(str::to_string)
        .map_err(|error| error.to_string())
}

/// The runtime version string.
#[no_mangle]
pub extern "C" fn osbtk_version() -> *mut c_char {
    into_c_string(env!("CARGO_PKG_VERSION").to_string())
}

/// Bakes a `Bake3dInput` JSON into baked command-span JSON.
///
/// # Safety
///
/// `input_json` must be a NUL-terminated UTF-8 C string, or null (which returns
/// an error JSON). The result must be released with [`osbtk_string_free`].
#[no_mangle]
pub unsafe extern "C" fn osbtk_bake3d(input_json: *const c_char) -> *mut c_char {
    let input = read(input_json);
    guarded(move || match input {
        Ok(input) => bake_to_json(&input),
        Err(error) => error_json(&error),
    })
}

/// Runs a JSON request (`{"op": "...", ...}`) against the runtime, returning JSON.
///
/// # Safety
///
/// `request_json` must be a NUL-terminated UTF-8 C string, or null (which returns
/// an error JSON). The result must be released with [`osbtk_string_free`].
#[no_mangle]
pub unsafe extern "C" fn osbtk_exec(request_json: *const c_char) -> *mut c_char {
    let request = read(request_json);
    guarded(move || match request {
        Ok(request) => exec(&request),
        Err(error) => error_json(&error),
    })
}

/// Releases a string returned by any `osbtk_*` function.
///
/// # Safety
///
/// `ptr` must be a pointer previously returned by an `osbtk_*` function and not
/// already freed; null is ignored.
#[no_mangle]
pub unsafe extern "C" fn osbtk_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Creates a runtime handle (storyboard + context) of the given resolution.
#[no_mangle]
pub extern "C" fn osbtk_runtime_new(width: f64, height: f64) -> *mut OsbtkRuntime {
    Box::into_raw(Box::new(OsbtkRuntime::new(width as f32, height as f32)))
}

/// Releases a runtime handle created by [`osbtk_runtime_new`].
///
/// # Safety
///
/// `runtime` must be a pointer returned by [`osbtk_runtime_new`] and not already
/// freed; null is ignored.
#[no_mangle]
pub unsafe extern "C" fn osbtk_runtime_free(runtime: *mut OsbtkRuntime) {
    if !runtime.is_null() {
        drop(Box::from_raw(runtime));
    }
}

/// Runs a JSON request against a runtime handle (layers/sprites/commands/sampling).
///
/// # Safety
///
/// `runtime` must be a live pointer from [`osbtk_runtime_new`]; `request_json`
/// must be a NUL-terminated UTF-8 C string or null. Free the result with
/// [`osbtk_string_free`].
#[no_mangle]
pub unsafe extern "C" fn osbtk_runtime_exec(
    runtime: *mut OsbtkRuntime,
    request_json: *const c_char,
) -> *mut c_char {
    if runtime.is_null() {
        return guarded(|| error_json("null runtime"));
    }
    let runtime = &mut *runtime;
    let request = read(request_json);
    guarded(move || match request {
        Ok(request) => runtime_exec(runtime, &request),
        Err(error) => error_json(&error),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads and frees an `osbtk_*` result.
    unsafe fn take(ptr: *mut c_char) -> String {
        let value = CStr::from_ptr(ptr).to_str().unwrap().to_string();
        osbtk_string_free(ptr);
        value
    }

    #[test]
    fn exec_round_trips_through_the_c_abi() {
        let request = CString::new(r#"{"op":"Common.version"}"#).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&unsafe { take(osbtk_exec(request.as_ptr())) }).unwrap();
        assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));

        let null = unsafe { take(osbtk_exec(std::ptr::null())) };
        assert!(serde_json::from_str::<serde_json::Value>(&null).unwrap()["error"].is_string());
    }

    #[test]
    fn version_and_string_free_are_safe() {
        let version = unsafe { take(osbtk_version()) };
        assert!(!version.is_empty());
        unsafe { osbtk_string_free(std::ptr::null_mut()) };
    }
}
