//! A minimal, dependency-free ZIP writer (stored entries) used to deliver
//! directory artifacts such as an OpenFX `.ofx.bundle` as a single download.

use std::io;
use std::path::Path;
use std::sync::OnceLock;

/// Appends a ZIP archive containing `entries` (asset path -> bytes).
pub fn store(entries: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    let mut count = 0u16;

    for (name, data) in entries {
        let offset = out.len() as u32;
        let crc = crc32(data);
        let size = data.len() as u32;
        let name_bytes = name.as_bytes();

        push_u32(&mut out, 0x0403_4b50);
        push_u16(&mut out, 20);
        push_u16(&mut out, 0x0800);
        push_u16(&mut out, 0);
        push_u16(&mut out, 0);
        push_u16(&mut out, 0x0021);
        push_u32(&mut out, crc);
        push_u32(&mut out, size);
        push_u32(&mut out, size);
        push_u16(&mut out, name_bytes.len() as u16);
        push_u16(&mut out, 0);
        out.extend_from_slice(name_bytes);
        out.extend_from_slice(data);

        push_u32(&mut central, 0x0201_4b50);
        push_u16(&mut central, 20);
        push_u16(&mut central, 20);
        push_u16(&mut central, 0x0800);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0x0021);
        push_u32(&mut central, crc);
        push_u32(&mut central, size);
        push_u32(&mut central, size);
        push_u16(&mut central, name_bytes.len() as u16);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u32(&mut central, 0);
        push_u32(&mut central, offset);
        central.extend_from_slice(name_bytes);
        count += 1;
    }

    let cd_offset = out.len() as u32;
    let cd_size = central.len() as u32;
    out.extend_from_slice(&central);
    push_u32(&mut out, 0x0605_4b50);
    push_u16(&mut out, 0);
    push_u16(&mut out, 0);
    push_u16(&mut out, count);
    push_u16(&mut out, count);
    push_u32(&mut out, cd_size);
    push_u32(&mut out, cd_offset);
    push_u16(&mut out, 0);
    out
}

/// Zips a directory tree; each entry is named `<prefix>/<relative path>`.
pub fn zip_directory(dir: &Path, prefix: &str) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();
    collect(dir, prefix, &mut entries)?;
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(store(&entries))
}

fn collect(dir: &Path, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) -> io::Result<()> {
    for entry in osb_core::io::read_dir(dir)? {
        let name = format!("{}/{}", prefix, entry.name);
        if entry.is_dir {
            collect(&entry.path, &name, out)?;
        } else {
            out.push((name, osb_core::io::read(&entry.path)?));
        }
    }
    Ok(())
}

/// CRC-32 (IEEE 802.3), as required by the ZIP format.
pub fn crc32(data: &[u8]) -> u32 {
    let table = crc_table();
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc = (crc >> 8) ^ table[((crc ^ byte as u32) & 0xFF) as usize];
    }
    !crc
}

fn crc_table() -> &'static [u32; 256] {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut table = [0u32; 256];
        for (i, entry) in table.iter_mut().enumerate() {
            let mut crc = i as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
            *entry = crc;
        }
        table
    })
}

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_matches_the_reference_vector() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn archive_has_a_local_header_and_end_of_central_directory() {
        let bytes = store(&[("a.txt".into(), b"hello".to_vec())]);
        assert_eq!(&bytes[0..4], &[0x50, 0x4b, 0x03, 0x04]);
        assert_eq!(
            &bytes[bytes.len() - 22..bytes.len() - 18],
            &[0x50, 0x4b, 0x05, 0x06]
        );
    }

    #[test]
    fn zip_directory_includes_nested_files_with_forward_slashes() {
        let root = osb_core::io::temp_dir().join(format!("osbtk-zip-{}", std::process::id()));
        let nested = root.join("Contents").join("Win64");
        osb_core::io::create_dir_all(&nested).unwrap();
        osb_core::io::write(nested.join("Thing.ofx"), b"binary").unwrap();
        osb_core::io::write(root.join("README.txt"), b"hi").unwrap();

        let bytes = zip_directory(&root, "Thing.ofx.bundle").unwrap();
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("Thing.ofx.bundle/Contents/Win64/Thing.ofx"));
        assert!(text.contains("Thing.ofx.bundle/README.txt"));
        assert!(!text.contains('\\'));

        osb_core::io::remove_dir_all(&root).ok();
    }
}
