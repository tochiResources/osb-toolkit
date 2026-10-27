//! RIFX reader tests over hand-built chunks.

use osb_parser_ae::rifx;
use osb_parser_ae::AeError;

fn block(id: &str, data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(id.as_bytes());
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(data);
    if !data.len().is_multiple_of(2) {
        out.push(0);
    }
    out
}

fn list(id: &str, children: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(id.as_bytes());
    for child in children {
        body.extend_from_slice(child);
    }
    block("LIST", &body)
}

fn rifx(root_id: &str, blocks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(root_id.as_bytes());
    for block in blocks {
        body.extend_from_slice(block);
    }
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFX");
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    out.extend_from_slice(&body);
    out
}

#[test]
fn reads_nested_lists_and_text() {
    let bytes = rifx(
        "Egg!",
        &[
            block("nhed", &[0u8; 16]),
            list("Fold", &[list("Item", &[block("Utf8", b"Hello\0\0")])]),
        ],
    );
    let root = rifx::read_root(&mut std::io::Cursor::new(&bytes)).unwrap();
    assert_eq!(root.identifier, "Egg!");
    let fold = root.sublist_find("Fold").unwrap();
    let item = fold.sublist_find("Item").unwrap();
    assert_eq!(item.find_type("Utf8").unwrap().text(), "Hello");
}

#[test]
fn decodes_an_embedded_utf8_chunk() {
    // `Utf8` chunk: "Utf8" + length + bytes.
    let mut payload = Vec::new();
    payload.extend_from_slice(b"Utf8");
    payload.extend_from_slice(&3u32.to_be_bytes());
    payload.extend_from_slice(b"abc");
    let bytes = rifx("Egg!", &[block("name", &payload)]);
    let root = rifx::read_root(&mut std::io::Cursor::new(&bytes)).unwrap();
    assert_eq!(root.find_type("name").unwrap().text(), "abc");
}

#[test]
fn rejects_non_rifx_input() {
    let error = rifx::read_root(&mut std::io::Cursor::new(b"RIFF....")).unwrap_err();
    assert!(matches!(error, AeError::NotRifx));
}
