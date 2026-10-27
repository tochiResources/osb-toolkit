//! File-operations API shared by every `osb-*` crate.
//!
//! This is the **single** place the toolkit talks to the filesystem. Crates must
//! go through these functions instead of calling `std::fs` directly, so path
//! handling, temp files, atomic writes and error context stay consistent and are
//! only implemented once.
//!
//! ```
//! # fn main() -> Result<(), osb_core::io::Error> {
//! let path = osb_core::io::temp_path("demo");
//! osb_core::io::write_string(&path, "hello")?;
//! assert_eq!(osb_core::io::read_string(&path)?, "hello");
//! osb_core::io::remove_file(&path)?;
//! # Ok(())
//! # }
//! ```
//!
//! Every fallible function returns [`Result`], carrying the offending path for
//! context. The error converts into [`std::io::Error`], so it composes with APIs
//! that already use `std::io::Result`.

pub mod path;

use std::fs::{File, Metadata, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use thiserror::Error;

pub use path::{
    extension, file_name, file_stem, is_absolute, join, normalize, normalize_separators, parent,
    relative_to, to_forward_slashes,
};

/// A file-operations error, annotated with the path it happened on.
#[derive(Debug, Error)]
#[error("{}: {source}", path.display())]
pub struct Error {
    path: PathBuf,
    #[source]
    source: std::io::Error,
}

impl Error {
    /// Wrap an I/O error with the path it happened on.
    pub fn new(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self {
            path: path.into(),
            source,
        }
    }

    /// The path the operation was acting on.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The underlying I/O error kind.
    pub fn kind(&self) -> std::io::ErrorKind {
        self.source.kind()
    }

    /// The underlying I/O error.
    pub fn source_error(&self) -> &std::io::Error {
        &self.source
    }
}

/// The toolkit's file-operation result type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        std::io::Error::new(error.source.kind(), error.to_string())
    }
}

/// One directory entry, resolved to a plain, owned value.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Full path of the entry.
    pub path: PathBuf,
    /// The entry's file name.
    pub name: String,
    /// Whether the entry is a directory.
    pub is_dir: bool,
    /// Whether the entry is a regular file.
    pub is_file: bool,
}

// ------------------------------------------------------------------ reading

/// Read a file into a byte vector.
pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|source| Error::new(path, source))
}

/// Read a file into a UTF-8 string.
pub fn read_string(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).map_err(|source| Error::new(path, source))
}

/// Read a file's lines, without trailing line terminators.
pub fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>> {
    Ok(read_string(path)?
        .lines()
        .map(|line| line.trim_end_matches('\r').to_string())
        .collect())
}

/// Open a file for reading.
pub fn open(path: impl AsRef<Path>) -> Result<File> {
    let path = path.as_ref();
    File::open(path).map_err(|source| Error::new(path, source))
}

/// Read from a reader until EOF into a byte vector.
pub fn read_all<R: Read>(mut reader: R) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .map_err(|source| Error::new(PathBuf::from("<reader>"), source))?;
    Ok(buffer)
}

// ------------------------------------------------------------------ writing

/// Write bytes to a file, creating or truncating it.
pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    std::fs::write(path, bytes).map_err(|source| Error::new(path, source))
}

/// Write a string to a file, creating or truncating it.
pub fn write_string(path: impl AsRef<Path>, contents: impl AsRef<str>) -> Result<()> {
    write(path, contents.as_ref().as_bytes())
}

/// Create a file for writing, creating or truncating it.
pub fn create(path: impl AsRef<Path>) -> Result<File> {
    let path = path.as_ref();
    File::create(path).map_err(|source| Error::new(path, source))
}

/// Append bytes to a file, creating it when missing.
pub fn append(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|source| Error::new(path, source))?;
    file.write_all(bytes.as_ref())
        .map_err(|source| Error::new(path, source))
}

/// Write bytes atomically: to a sibling temp file, then rename over the target.
pub fn write_atomic(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let directory = path.parent().filter(|p| !p.as_os_str().is_empty());
    let name = file_name(path).unwrap_or_else(|| "osb".to_string());
    let temp = match directory {
        Some(directory) => directory.join(format!(".{name}.tmp{}", std::process::id())),
        None => PathBuf::from(format!(".{name}.tmp{}", std::process::id())),
    };
    write(&temp, bytes)?;
    if let Err(error) = rename(&temp, path) {
        let _ = remove_file(&temp);
        return Err(error);
    }
    Ok(())
}

// ------------------------------------------------------------------ queries

/// Whether a path exists (file or directory).
pub fn exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// Whether a path is an existing regular file.
pub fn is_file(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

/// Whether a path is an existing directory.
pub fn is_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

/// Metadata for a path.
pub fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    let path = path.as_ref();
    std::fs::metadata(path).map_err(|source| Error::new(path, source))
}

/// The size of a file in bytes.
pub fn file_size(path: impl AsRef<Path>) -> Result<u64> {
    Ok(metadata(path)?.len())
}

/// Resolve a path to an absolute, canonical form.
pub fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    std::fs::canonicalize(path).map_err(|source| Error::new(path, source))
}

/// List a directory's entries, sorted by name.
pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<Entry>> {
    let path = path.as_ref();
    let reader = std::fs::read_dir(path).map_err(|source| Error::new(path, source))?;
    let mut entries = Vec::new();
    for entry in reader {
        let entry = entry.map_err(|source| Error::new(path, source))?;
        let entry_path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| Error::new(path, source))?;
        entries.push(Entry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry_path,
            is_dir: file_type.is_dir(),
            is_file: file_type.is_file(),
        });
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

// ---------------------------------------------------------------- mutating

/// Create a directory.
pub fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::create_dir(path).map_err(|source| Error::new(path, source))
}

/// Create a directory and any missing parents.
pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).map_err(|source| Error::new(path, source))
}

/// Copy a file, returning the number of bytes copied.
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    let from = from.as_ref();
    let to = to.as_ref();
    std::fs::copy(from, to).map_err(|source| Error::new(from, source))
}

/// Rename (or move) a file or directory.
pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    std::fs::rename(from, to).map_err(|source| Error::new(from, source))
}

/// Remove a file.
pub fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::remove_file(path).map_err(|source| Error::new(path, source))
}

/// Remove an empty directory.
pub fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::remove_dir(path).map_err(|source| Error::new(path, source))
}

/// Remove a directory and everything inside it.
pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::remove_dir_all(path).map_err(|source| Error::new(path, source))
}

/// Set a file's Unix permission bits (no-op on other platforms).
#[cfg(unix)]
pub fn set_mode(path: impl AsRef<Path>, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let path = path.as_ref();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .map_err(|source| Error::new(path, source))
}

/// Set a file's Unix permission bits (no-op on other platforms).
#[cfg(not(unix))]
pub fn set_mode(_path: impl AsRef<Path>, _mode: u32) -> Result<()> {
    Ok(())
}

// ------------------------------------------------------------------- temp

/// The system temporary directory.
pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
}

/// A unique path inside the temp directory, prefixed with `prefix`.
///
/// The path is not created; it is only guaranteed to be unique for the process.
pub fn temp_path(prefix: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    temp_dir().join(format!("{prefix}-{}-{seq}", std::process::id()))
}

/// Create (and return) a unique temporary directory prefixed with `prefix`.
pub fn temp_dir_create(prefix: &str) -> Result<PathBuf> {
    let path = temp_path(prefix);
    create_dir_all(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_bytes_and_text() {
        let path = temp_path("osb-io-roundtrip");
        write(&path, b"hello").unwrap();
        assert_eq!(read(&path).unwrap(), b"hello");
        write_string(&path, "line1\nline2\n").unwrap();
        assert_eq!(read_lines(&path).unwrap(), vec!["line1", "line2"]);
        assert!(is_file(&path));
        assert_eq!(file_size(&path).unwrap(), 12);
        remove_file(&path).unwrap();
    }

    #[test]
    fn atomic_write_replaces_contents() {
        let path = temp_path("osb-io-atomic");
        write_atomic(&path, b"one").unwrap();
        write_atomic(&path, b"two").unwrap();
        assert_eq!(read(&path).unwrap(), b"two");
        remove_file(&path).unwrap();
    }

    #[test]
    fn lists_and_removes_a_tree() {
        let dir = temp_dir_create("osb-io-tree").unwrap();
        write(dir.join("a.txt"), b"a").unwrap();
        create_dir_all(dir.join("nested")).unwrap();
        write(dir.join("nested").join("b.txt"), b"b").unwrap();

        let names: Vec<_> = read_dir(&dir)
            .unwrap()
            .into_iter()
            .map(|e| e.name)
            .collect();
        assert_eq!(names, vec!["a.txt", "nested"]);
        assert!(read_dir(&dir).unwrap()[1].is_dir);

        remove_dir_all(&dir).unwrap();
        assert!(!exists(&dir));
    }

    #[test]
    fn errors_carry_the_path_and_convert_to_io() {
        let path = temp_path("osb-io-missing");
        let error = read(&path).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert_eq!(error.path(), path.as_path());
        let io: std::io::Error = error.into();
        assert_eq!(io.kind(), std::io::ErrorKind::NotFound);
    }
}
