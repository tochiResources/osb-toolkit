//! RIFX (big-endian RIFF) chunk reader.
//!
//! After Effects `.aep` files are RIFF containers written big-endian ("RIFX").
//! The file is a root list whose identifier is `Egg!`; inside are typed chunks
//! (`nhed`, `Fold`, `ExEn`, …), where `LIST` chunks nest further lists.
//!
//! The reader mirrors the behaviour of the reference Go parser (`rioam2/rifx`):
//! it reads whole blocks up to the header's size, treats a `LIST` as a nested
//! list, and recovers from an oversized final block by absorbing the remainder
//! as an anonymous block.

use std::io::Read;

use crate::error::{AeError, AeResult};

/// A chunk's payload: either raw bytes or a nested list.
#[derive(Debug, Clone)]
pub enum Data {
    /// Opaque chunk bytes.
    Bytes(Vec<u8>),
    /// A nested `LIST` chunk.
    List(List),
}

/// A single RIFX chunk.
#[derive(Debug, Clone)]
pub struct Block {
    /// The four-character chunk type (`nhed`, `Utf8`, `LIST`, …).
    pub id: String,
    /// The declared payload size in bytes.
    pub size: u32,
    /// The chunk payload.
    pub data: Data,
}

impl Block {
    /// The raw bytes of a non-list block.
    pub fn bytes(&self) -> Option<&[u8]> {
        match &self.data {
            Data::Bytes(bytes) => Some(bytes),
            Data::List(_) => None,
        }
    }

    /// The nested list of a `LIST` block.
    pub fn list(&self) -> Option<&List> {
        match &self.data {
            Data::List(list) => Some(list),
            Data::Bytes(_) => None,
        }
    }

    /// The payload decoded as text.
    ///
    /// A payload that itself wraps a `Utf8` chunk (`Utf8<len><bytes>`) has the
    /// inner string returned; otherwise the bytes are decoded and NUL-trimmed.
    pub fn text(&self) -> String {
        let Some(data) = self.bytes() else {
            return String::new();
        };
        if data.len() >= 8 && data[..4].eq_ignore_ascii_case(b"utf8") {
            let length = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
            if length + 8 > self.size as usize || length + 8 > data.len() {
                return trim_nul(data);
            }
            return trim_nul(&data[8..8 + length]);
        }
        trim_nul(data)
    }

    /// A big-endian `u16` at `offset`, when the payload is long enough.
    pub fn be_u16(&self, offset: usize) -> Option<u16> {
        let data = self.bytes()?;
        let slice = data.get(offset..offset + 2)?;
        Some(u16::from_be_bytes([slice[0], slice[1]]))
    }

    /// A big-endian `u32` at `offset`, when the payload is long enough.
    pub fn be_u32(&self, offset: usize) -> Option<u32> {
        let data = self.bytes()?;
        let slice = data.get(offset..offset + 4)?;
        Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// A single byte at `offset`, when the payload is long enough.
    pub fn byte(&self, offset: usize) -> Option<u8> {
        self.bytes()?.get(offset).copied()
    }
}

/// A RIFX list: an identifier plus ordered blocks.
#[derive(Debug, Clone, Default)]
pub struct List {
    /// The list identifier (`Egg!`, `Fold`, `Item`, `tdgp`, …).
    pub identifier: String,
    /// The list's child blocks.
    pub blocks: Vec<Block>,
}

impl List {
    /// The first block matching `predicate`.
    pub fn find(&self, predicate: impl Fn(&Block) -> bool) -> Option<&Block> {
        self.blocks.iter().find(|block| predicate(block))
    }

    /// The first block with the given chunk type.
    pub fn find_type(&self, id: &str) -> Option<&Block> {
        self.find(|block| block.id == id)
    }

    /// The first nested list with the given identifier.
    pub fn sublist_find(&self, identifier: &str) -> Option<&List> {
        self.blocks.iter().find_map(|block| match &block.data {
            Data::List(list) if list.identifier == identifier => Some(list),
            _ => None,
        })
    }

    /// Every nested list with the given identifier, in order.
    pub fn sublists(&self, identifier: &str) -> Vec<&List> {
        self.blocks
            .iter()
            .filter_map(|block| match &block.data {
                Data::List(list) if list.identifier == identifier => Some(list),
                _ => None,
            })
            .collect()
    }

    /// Concatenate the blocks of every nested list with the given identifier.
    pub fn sublist_merge(&self, identifier: &str) -> List {
        let mut merged = List {
            identifier: identifier.to_string(),
            blocks: Vec::new(),
        };
        for list in self.sublists(identifier) {
            merged.blocks.extend(list.blocks.iter().cloned());
        }
        merged
    }
}

pub(crate) fn trim_nul(data: &[u8]) -> String {
    let start = data
        .iter()
        .position(|&byte| byte != 0)
        .unwrap_or(data.len());
    let end = data
        .iter()
        .rposition(|&byte| byte != 0)
        .map_or(start, |i| i + 1);
    String::from_utf8_lossy(&data[start..end]).into_owned()
}

fn ascii(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn read_array<R: Read, const N: usize>(reader: &mut R) -> AeResult<[u8; N]> {
    let mut buffer = [0u8; N];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}

/// Read a RIFX root list from a reader.
pub fn read_root<R: Read>(reader: &mut R) -> AeResult<List> {
    let magic = read_array::<R, 4>(reader)?;
    if &magic != b"RIFX" {
        return Err(AeError::NotRifx);
    }
    let size = u32::from_be_bytes(read_array::<R, 4>(reader)?);
    let (list, _consumed) = read_list(reader, size)?;
    Ok(list)
}

fn read_list<R: Read>(reader: &mut R, limit: u32) -> AeResult<(List, u32)> {
    let identifier = ascii(&read_array::<R, 4>(reader)?);
    let mut read_bytes: u32 = 4;
    let mut blocks = Vec::new();

    while read_bytes < limit {
        let remaining = limit.saturating_sub(read_bytes);
        let (block, consumed) = read_block(reader, remaining)?;
        if consumed == 0 {
            break;
        }
        read_bytes = read_bytes.saturating_add(consumed);
        blocks.push(block);
    }

    Ok((List { identifier, blocks }, read_bytes))
}

fn read_block<R: Read>(reader: &mut R, limit: u32) -> AeResult<(Block, u32)> {
    let mut read_bytes: u32 = 0;

    let type_bytes = read_array::<R, 4>(reader)?;
    read_bytes += 4;
    let size_bytes = read_array::<R, 4>(reader)?;
    read_bytes += 4;
    let size = u32::from_be_bytes(size_bytes);
    let mut id = ascii(&type_bytes);

    let mut anonymous = false;
    let data = if size > limit.saturating_sub(read_bytes) {
        // Oversized/malformed final block: absorb the rest of the list.
        let rest_len = limit.saturating_sub(read_bytes) as usize;
        let mut rest = vec![0u8; rest_len];
        reader.read_exact(&mut rest)?;
        read_bytes = read_bytes.saturating_add(rest_len as u32);
        let mut combined = Vec::with_capacity(8 + rest.len());
        combined.extend_from_slice(&type_bytes);
        combined.extend_from_slice(&size_bytes);
        combined.extend_from_slice(&rest);
        anonymous = true;
        Data::Bytes(combined)
    } else if id == "LIST" {
        let (list, consumed) = read_list(reader, size)?;
        read_bytes = read_bytes.saturating_add(consumed);
        Data::List(list)
    } else {
        let mut payload = vec![0u8; size as usize];
        reader.read_exact(&mut payload)?;
        read_bytes += size;
        Data::Bytes(payload)
    };

    // Blocks are padded to an even byte boundary.
    if !size.is_multiple_of(2) {
        let _ = read_array::<R, 1>(reader)?;
        read_bytes += 1;
    }

    if anonymous {
        id = "ANON".to_string();
    }

    Ok((Block { id, size, data }, read_bytes))
}
