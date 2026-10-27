//! `wasm-bindgen` surface over the osbTK JSON runtime bridge. It mirrors `crates/osb-native` - `version`, `bake3d`, `exec` and a
//! `Runtime` class - so the same engine drives JavaScript/TypeScript in the
//! browser and Node.

use osb_native::{bake_to_json, exec as exec_bridge, runtime_exec, OsbtkRuntime};
use wasm_bindgen::prelude::*;

/// The runtime version string.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Bakes a plain 3D input (`Bake3dInput`) JSON into baked command-span JSON.
#[wasm_bindgen(js_name = bake3d)]
pub fn bake3d(input_json: &str) -> String {
    bake_to_json(input_json)
}

/// Runs a JSON request (`{"op": "...", ...}`) against the whole runtime.
#[wasm_bindgen(js_name = exec)]
pub fn exec_bridge_js(request_json: &str) -> String {
    exec_bridge(request_json)
}

/// A live runtime (storyboard + context) - the storyboard API.
#[wasm_bindgen]
pub struct Runtime {
    inner: OsbtkRuntime,
}

#[wasm_bindgen]
impl Runtime {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Runtime {
        Runtime {
            inner: OsbtkRuntime::new(width as f32, height as f32),
        }
    }

    /// Runs a JSON request (`createSprite`, `command`, `spriteStateAt`, ...).
    #[wasm_bindgen(js_name = exec)]
    pub fn exec(&mut self, request_json: &str) -> String {
        runtime_exec(&mut self.inner, request_json)
    }
}

// -- File operations (osb-io / osb-io-wasm) --------------------------------

fn io_error(error: osb_core::io_wasm::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// The detected host environment: `native` / `node` / `bun` / `deno` / `browser`
/// / `unknown`.
#[wasm_bindgen(js_name = ioEnvironment)]
pub fn io_environment() -> String {
    osb_core::io_wasm::environment().as_str().to_string()
}

/// Installs a filesystem host from `globalThis.__osbtk_io` when the embedder
/// provides one, returning the detected environment.
#[wasm_bindgen(js_name = ioInstallGlobalHost)]
pub fn io_install_global_host() -> String {
    osb_core::io_wasm::install_from_global()
        .as_str()
        .to_string()
}

/// Reads a file into a byte array.
#[wasm_bindgen(js_name = ioRead)]
pub fn io_read(path: &str) -> Result<Vec<u8>, JsValue> {
    osb_core::io_wasm::read(path).map_err(io_error)
}

/// Reads a file into a UTF-8 string.
#[wasm_bindgen(js_name = ioReadString)]
pub fn io_read_string(path: &str) -> Result<String, JsValue> {
    osb_core::io_wasm::read_string(path).map_err(io_error)
}

/// Writes bytes to a file.
#[wasm_bindgen(js_name = ioWrite)]
pub fn io_write(path: &str, bytes: &[u8]) -> Result<(), JsValue> {
    osb_core::io_wasm::write(path, bytes).map_err(io_error)
}

/// Writes a UTF-8 string to a file.
#[wasm_bindgen(js_name = ioWriteString)]
pub fn io_write_string(path: &str, contents: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::write_string(path, contents).map_err(io_error)
}

/// Whether a path exists.
#[wasm_bindgen(js_name = ioExists)]
pub fn io_exists(path: &str) -> bool {
    osb_core::io_wasm::exists(path)
}

/// Whether a path is a regular file.
#[wasm_bindgen(js_name = ioIsFile)]
pub fn io_is_file(path: &str) -> bool {
    osb_core::io_wasm::is_file(path)
}

/// Whether a path is a directory.
#[wasm_bindgen(js_name = ioIsDir)]
pub fn io_is_dir(path: &str) -> bool {
    osb_core::io_wasm::is_dir(path)
}

/// The size of a file in bytes.
#[wasm_bindgen(js_name = ioFileSize)]
pub fn io_file_size(path: &str) -> Result<f64, JsValue> {
    osb_core::io_wasm::file_size(path)
        .map(|size| size as f64)
        .map_err(io_error)
}

/// Lists a directory's entry names, sorted.
#[wasm_bindgen(js_name = ioList)]
pub fn io_list(path: &str) -> Result<Vec<String>, JsValue> {
    osb_core::io_wasm::list(path).map_err(io_error)
}

/// Creates a directory and any missing parents.
#[wasm_bindgen(js_name = ioCreateDirAll)]
pub fn io_create_dir_all(path: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::create_dir_all(path).map_err(io_error)
}

/// Copies a file.
#[wasm_bindgen(js_name = ioCopy)]
pub fn io_copy(from: &str, to: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::copy(from, to).map_err(io_error)
}

/// Renames (or moves) a file.
#[wasm_bindgen(js_name = ioRename)]
pub fn io_rename(from: &str, to: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::rename(from, to).map_err(io_error)
}

/// Removes a file.
#[wasm_bindgen(js_name = ioRemoveFile)]
pub fn io_remove_file(path: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::remove_file(path).map_err(io_error)
}

/// Removes a directory and everything inside it.
#[wasm_bindgen(js_name = ioRemoveDirAll)]
pub fn io_remove_dir_all(path: &str) -> Result<(), JsValue> {
    osb_core::io_wasm::remove_dir_all(path).map_err(io_error)
}
