//! Path helpers shared by every crate.
//!
//! These are pure string/path transforms (no I/O), so they are safe to use from
//! `no_std`-ish contexts and are cheap to test. Anything that touches the
//! filesystem lives in the crate root.

use std::path::{Component, Path, PathBuf};

/// Normalise a path lexically: collapse repeated separators and resolve `.`/`..`
/// without touching the filesystem (a leading `..` is preserved).
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

/// A path rendered with forward slashes (the toolkit's canonical form).
pub fn to_forward_slashes(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// A string rendered with forward slashes and collapsed duplicate separators.
pub fn normalize_separators(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_slash = false;
    for ch in value.chars() {
        if ch == '\\' || ch == '/' {
            if !last_slash {
                out.push('/');
            }
            last_slash = true;
        } else {
            out.push(ch);
            last_slash = false;
        }
    }
    out
}

/// Join `base` and `relative` without normalising.
pub fn join(base: impl AsRef<Path>, relative: impl AsRef<Path>) -> PathBuf {
    base.as_ref().join(relative.as_ref())
}

/// The path of `target` relative to `base`, when `target` is nested under `base`.
pub fn relative_to(base: &Path, target: &Path) -> Option<PathBuf> {
    let base = normalize(base);
    let target = normalize(target);
    target.strip_prefix(&base).ok().map(Path::to_path_buf)
}

/// The final component of a path as an owned `String`.
pub fn file_name(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
}

/// The file name without its extension.
pub fn file_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .map(|name| name.to_string_lossy().into_owned())
}

/// The extension (without the leading dot), lower-cased.
pub fn extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
}

/// The parent directory of a path, as an owned `PathBuf`.
pub fn parent(path: &Path) -> Option<PathBuf> {
    path.parent().map(Path::to_path_buf)
}

/// Whether a path is absolute.
pub fn is_absolute(path: &Path) -> bool {
    path.is_absolute()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_dot_and_parent_segments() {
        assert_eq!(normalize(Path::new("a/./b/../c")), PathBuf::from("a/c"));
        assert_eq!(normalize(Path::new("../a")), PathBuf::from("../a"));
    }

    #[test]
    fn renders_forward_slashes() {
        assert_eq!(normalize_separators("a\\b\\\\c"), "a/b/c");
        assert_eq!(to_forward_slashes(Path::new("a\\b")), "a/b");
    }

    #[test]
    fn strips_a_base_prefix() {
        assert_eq!(
            relative_to(Path::new("/root"), Path::new("/root/a/b.txt")),
            Some(PathBuf::from("a/b.txt"))
        );
        assert_eq!(
            relative_to(Path::new("/root"), Path::new("/other/b.txt")),
            None
        );
    }
}
