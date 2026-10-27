//! WASM-aware file operations for osbTK.
//!
//! On the OS these forward to [`osb_core::io`] (real filesystem). Compiled to
//! `wasm32` there is no filesystem, so operations go through a host bridge the
//! JavaScript side installs: under Node/Bun/Deno the bridge is backed by `fs`; in
//! a browser nothing is installed, so operations return [`Error::Unsupported`].
//! The host can be probed with [`environment`], and [`install_from_global`]
//! picks up `globalThis.__osbtk_io` when the embedder provides one.

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use thiserror::Error;

/// The environment a build is executing in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// A native OS process (not WASM).
    Native,
    /// Node.js.
    Node,
    /// Bun.
    Bun,
    /// Deno.
    Deno,
    /// A browser (or web worker) with no filesystem.
    Browser,
    /// A WASM host that could not be identified.
    Unknown,
}

impl Environment {
    /// The lowercase name of the environment (for the SDK).
    pub fn as_str(self) -> &'static str {
        match self {
            Environment::Native => "native",
            Environment::Node => "node",
            Environment::Bun => "bun",
            Environment::Deno => "deno",
            Environment::Browser => "browser",
            Environment::Unknown => "unknown",
        }
    }

    /// Whether real filesystem access is available here.
    pub fn has_filesystem(self) -> bool {
        matches!(
            self,
            Environment::Native | Environment::Node | Environment::Bun | Environment::Deno
        )
    }
}

/// Detects the environment the current build is running in (the OS).
#[cfg(not(target_arch = "wasm32"))]
pub fn environment() -> Environment {
    Environment::Native
}

/// Detects the environment the WASM module is running in by probing
/// `globalThis` for the host's globals.
#[cfg(target_arch = "wasm32")]
pub fn environment() -> Environment {
    use js_sys::{global, Reflect};
    use wasm_bindgen::JsValue;
    let global = global();
    let has = |name: &str| {
        Reflect::get(&global, &JsValue::from_str(name))
            .map(|value| !value.is_undefined() && !value.is_null())
            .unwrap_or(false)
    };
    if has("Deno") {
        return Environment::Deno;
    }
    if has("Bun") {
        return Environment::Bun;
    }
    if has("process") {
        return Environment::Node;
    }
    if has("window") || has("document") {
        return Environment::Browser;
    }
    Environment::Unknown
}

/// A file-operation error.
#[derive(Debug, Error)]
pub enum Error {
    /// File operations are unavailable (a browser, or no host bridge installed).
    #[error("file operations are not available in this environment")]
    Unsupported,
    /// The host bridge reported an error.
    #[error("host I/O error: {0}")]
    Host(String),
    /// A native filesystem error (`osb-io`).
    #[error(transparent)]
    IO(#[from] crate::io::Error),
}

/// The result of a file operation.
pub type Result<T> = std::result::Result<T, Error>;

/// The host bridge the WASM module calls for real I/O.
///
/// The JavaScript side installs one (backed by `fs` under Node/Bun/Deno, or a
/// virtual filesystem in the browser). Paths are passed as UTF-8 strings.
pub trait Host {
    fn read(&self, path: &str) -> std::result::Result<Vec<u8>, String>;
    fn write(&self, path: &str, bytes: &[u8]) -> std::result::Result<(), String>;
    fn exists(&self, path: &str) -> bool;
    fn is_file(&self, path: &str) -> bool;
    fn is_dir(&self, path: &str) -> bool;
    fn file_size(&self, path: &str) -> std::result::Result<u64, String>;
    fn list(&self, path: &str) -> std::result::Result<Vec<String>, String>;
    fn create_dir_all(&self, path: &str) -> std::result::Result<(), String>;
    fn remove_file(&self, path: &str) -> std::result::Result<(), String>;
    fn remove_dir_all(&self, path: &str) -> std::result::Result<(), String>;
    fn copy(&self, from: &str, to: &str) -> std::result::Result<(), String>;
    fn rename(&self, from: &str, to: &str) -> std::result::Result<(), String>;
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static HOST: RefCell<Option<Rc<dyn Host>>> = const { RefCell::new(None) };
}

/// Installs the host bridge. A no-op on the OS, where [`osb_core::io`] is used.
#[cfg(target_arch = "wasm32")]
pub fn install(host: Rc<dyn Host>) {
    HOST.with(|slot| *slot.borrow_mut() = Some(host));
}

/// Installs the host bridge. A no-op on the OS, where [`osb_core::io`] is used.
#[cfg(not(target_arch = "wasm32"))]
pub fn install(_host: Rc<dyn Host>) {}

#[cfg(target_arch = "wasm32")]
fn host() -> Option<Rc<dyn Host>> {
    HOST.with(|slot| slot.borrow().clone())
}

#[cfg(target_arch = "wasm32")]
fn call<T>(f: impl FnOnce(&dyn Host) -> std::result::Result<T, String>) -> Result<T> {
    match host() {
        Some(host) => f(host.as_ref()).map_err(Error::Host),
        None => Err(Error::Unsupported),
    }
}

#[cfg(target_arch = "wasm32")]
fn display(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// Read a file into a byte vector.
pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::read(path).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.read(&display(path)))
    }
}

/// Read a file into a UTF-8 string.
pub fn read_string(path: impl AsRef<Path>) -> Result<String> {
    String::from_utf8(read(path)?).map_err(|error| Error::Host(format!("invalid UTF-8: {error}")))
}

/// Write bytes to a file, creating or truncating it.
pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::write(path, bytes).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.write(&display(path), bytes.as_ref()))
    }
}

/// Write a string to a file, creating or truncating it.
pub fn write_string(path: impl AsRef<Path>, contents: impl AsRef<str>) -> Result<()> {
    write(path, contents.as_ref().as_bytes())
}

/// Whether a path exists (file or directory).
pub fn exists(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::exists(path)
    }
    #[cfg(target_arch = "wasm32")]
    {
        host()
            .map(|host| host.exists(&display(path)))
            .unwrap_or(false)
    }
}

/// Whether a path is an existing regular file.
pub fn is_file(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::is_file(path)
    }
    #[cfg(target_arch = "wasm32")]
    {
        host()
            .map(|host| host.is_file(&display(path)))
            .unwrap_or(false)
    }
}

/// Whether a path is an existing directory.
pub fn is_dir(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::is_dir(path)
    }
    #[cfg(target_arch = "wasm32")]
    {
        host()
            .map(|host| host.is_dir(&display(path)))
            .unwrap_or(false)
    }
}

/// The size of a file in bytes.
pub fn file_size(path: impl AsRef<Path>) -> Result<u64> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::file_size(path).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.file_size(&display(path)))
    }
}

/// List a directory's entry names, sorted.
pub fn list(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(crate::io::read_dir(path)
            .map_err(Error::IO)?
            .into_iter()
            .map(|entry| entry.name)
            .collect())
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.list(&display(path)))
    }
}

/// Create a directory and any missing parents.
pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::create_dir_all(path).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.create_dir_all(&display(path)))
    }
}

/// Copy a file.
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::copy(from, to).map_err(Error::IO)?;
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.copy(&display(from), &display(to)))
    }
}

/// Rename (or move) a file.
pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::rename(from, to).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.rename(&display(from), &display(to)))
    }
}

/// Remove a file.
pub fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::remove_file(path).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.remove_file(&display(path)))
    }
}

/// Remove a directory and everything inside it.
pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::remove_dir_all(path).map_err(Error::IO)
    }
    #[cfg(target_arch = "wasm32")]
    {
        call(|host| host.remove_dir_all(&display(path)))
    }
}

/// The system temporary directory (native only; unsupported in WASM).
pub fn temp_dir() -> Result<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(crate::io::temp_dir())
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(Error::Unsupported)
    }
}

/// Installs the host bridge from `globalThis.__osbtk_io` when the embedder
/// provides one, returning the detected environment.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_from_global() -> Environment {
    Environment::Native
}

/// Installs the host bridge from `globalThis.__osbtk_io`, if present.
#[cfg(target_arch = "wasm32")]
pub use js_host::install_from_global;

#[cfg(target_arch = "wasm32")]
mod js_host {
    use super::{environment, install, Environment, Host};
    use js_sys::{Array, Function, Reflect, Uint8Array};
    use std::rc::Rc;
    use wasm_bindgen::{JsCast, JsValue};

    /// A [`Host`] backed by `globalThis.__osbtk_io`, an object of JS functions.
    struct JsHost {
        object: JsValue,
    }

    impl JsHost {
        fn invoke(&self, name: &str, args: &[JsValue]) -> std::result::Result<JsValue, String> {
            let function = Reflect::get(&self.object, &JsValue::from_str(name))
                .ok()
                .and_then(|value| value.dyn_into::<Function>().ok())
                .ok_or_else(|| format!("`{name}` is not a function on __osbtk_io"))?;
            let array = Array::new();
            for arg in args {
                array.push(arg);
            }
            function
                .apply(&self.object, &array)
                .map_err(|error| js_message(&error))
        }
    }

    fn js_message(value: &JsValue) -> String {
        Reflect::get(value, &JsValue::from_str("message"))
            .ok()
            .and_then(|message| message.as_string())
            .unwrap_or_else(|| format!("{value:?}"))
    }

    impl Host for JsHost {
        fn read(&self, path: &str) -> std::result::Result<Vec<u8>, String> {
            let value = self.invoke("read", &[JsValue::from_str(path)])?;
            let bytes: Uint8Array = value
                .dyn_into()
                .map_err(|_| "read() must return a Uint8Array".to_string())?;
            Ok(bytes.to_vec())
        }

        fn write(&self, path: &str, bytes: &[u8]) -> std::result::Result<(), String> {
            self.invoke(
                "write",
                &[JsValue::from_str(path), Uint8Array::from(bytes).into()],
            )?;
            Ok(())
        }

        fn exists(&self, path: &str) -> bool {
            self.invoke("exists", &[JsValue::from_str(path)])
                .map(|value| value.is_truthy())
                .unwrap_or(false)
        }

        fn is_file(&self, path: &str) -> bool {
            self.invoke("isFile", &[JsValue::from_str(path)])
                .map(|value| value.is_truthy())
                .unwrap_or(false)
        }

        fn is_dir(&self, path: &str) -> bool {
            self.invoke("isDir", &[JsValue::from_str(path)])
                .map(|value| value.is_truthy())
                .unwrap_or(false)
        }

        fn file_size(&self, path: &str) -> std::result::Result<u64, String> {
            let value = self.invoke("fileSize", &[JsValue::from_str(path)])?;
            Ok(value.as_f64().unwrap_or(0.0) as u64)
        }

        fn list(&self, path: &str) -> std::result::Result<Vec<String>, String> {
            let value = self.invoke("list", &[JsValue::from_str(path)])?;
            let array: Array = value
                .dyn_into()
                .map_err(|_| "list() must return an array".to_string())?;
            Ok(array.iter().filter_map(|item| item.as_string()).collect())
        }

        fn create_dir_all(&self, path: &str) -> std::result::Result<(), String> {
            self.invoke("createDirAll", &[JsValue::from_str(path)])?;
            Ok(())
        }

        fn remove_file(&self, path: &str) -> std::result::Result<(), String> {
            self.invoke("removeFile", &[JsValue::from_str(path)])?;
            Ok(())
        }

        fn remove_dir_all(&self, path: &str) -> std::result::Result<(), String> {
            self.invoke("removeDirAll", &[JsValue::from_str(path)])?;
            Ok(())
        }

        fn copy(&self, from: &str, to: &str) -> std::result::Result<(), String> {
            self.invoke("copy", &[JsValue::from_str(from), JsValue::from_str(to)])?;
            Ok(())
        }

        fn rename(&self, from: &str, to: &str) -> std::result::Result<(), String> {
            self.invoke("rename", &[JsValue::from_str(from), JsValue::from_str(to)])?;
            Ok(())
        }
    }

    /// Installs the host from `globalThis.__osbtk_io`, if present, and returns
    /// the detected environment.
    pub fn install_from_global() -> Environment {
        let global = js_sys::global();
        let object =
            Reflect::get(&global, &JsValue::from_str("__osbtk_io")).unwrap_or(JsValue::UNDEFINED);
        if !object.is_undefined() && !object.is_null() {
            install(Rc::new(JsHost { object }));
        }
        environment()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_environment_has_a_filesystem() {
        assert_eq!(environment(), Environment::Native);
        assert!(environment().has_filesystem());
        assert_eq!(environment().as_str(), "native");
    }

    #[test]
    fn native_io_round_trips() {
        let dir = crate::io::temp_dir_create("osb-io-wasm").unwrap();
        let file = dir.join("a.txt");
        write_string(&file, "hello").unwrap();
        assert!(exists(&file));
        assert!(is_file(&file));
        assert!(!is_dir(&file));
        assert_eq!(read_string(&file).unwrap(), "hello");
        assert_eq!(file_size(&file).unwrap(), 5);

        create_dir_all(dir.join("nested")).unwrap();
        assert_eq!(list(&dir).unwrap(), vec!["a.txt", "nested"]);

        copy(&file, dir.join("b.txt")).unwrap();
        rename(dir.join("b.txt"), dir.join("c.txt")).unwrap();
        assert!(exists(dir.join("c.txt")));

        remove_file(&file).unwrap();
        remove_dir_all(&dir).unwrap();
        assert!(!exists(&dir));
    }
}
