//! Static parser for Adobe After Effects `*.aep` project files.
//!
//! After Effects projects are RIFF containers written big-endian (`RIFX`). This
//! crate reads the chunk tree and exposes the useful structure - project bit
//! depth and expression engine, the item tree, compositions (dimensions,
//! framerate, duration), layers (source, flags, effects) and their properties -
//! **without a running After Effects**, so tools can inspect a project
//! statically and cheaply.
//!
//! ```
//! # fn main() -> Result<(), osb_parser_ae::AeError> {
//! let project = osb_parser_ae::from_bytes(b"not a project").unwrap_err();
//! # let _ = project;
//! # Ok(())
//! # }
//! ```
//!
//! The parser is a Rust port of the community
//! [`boltframe/aftereffects-aep-parser`](https://github.com/boltframe/aftereffects-aep-parser)
//! (which builds on `rioam2/rifx`), keeping the same offsets and flag bits.

mod error;
mod item;
mod layer;
mod property;
pub mod rifx;

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

pub use error::{AeError, AeResult};
pub use item::{FootageType, Item, ItemType};
pub use layer::{Layer, LayerFrameBlendMode, LayerQuality, LayerSamplingMode};
pub use property::{Interpolation, Keyframe, Property, PropertyType, Stream};
pub use rifx::{Block, Data, List};

/// Bits per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Bpc {
    /// 8 bits per channel.
    #[default]
    Eight,
    /// 16 bits per channel.
    Sixteen,
    /// 32 bits per channel.
    ThirtyTwo,
}

impl Bpc {
    fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Self::Sixteen,
            0x02 => Self::ThirtyTwo,
            _ => Self::Eight,
        }
    }
}

/// A parsed After Effects project.
#[derive(Debug, Clone)]
pub struct Project {
    /// The expression engine the project is set to (`JavaScript`/`ExtendScript`).
    pub expression_engine: String,
    /// The project's bit depth.
    pub depth: Bpc,
    /// The root project folder and everything under it.
    pub root_folder: Item,
}

impl Project {
    /// Every item in the project, depth-first from the root folder.
    pub fn items(&self) -> Items<'_> {
        Items {
            stack: vec![std::slice::from_ref(&self.root_folder).iter()],
        }
    }

    /// Every composition in the project.
    pub fn compositions(&self) -> impl Iterator<Item = &Item> {
        self.items()
            .filter(|item| item.item_type == ItemType::Composition)
    }

    /// Find an item by id.
    pub fn item(&self, id: u32) -> Option<&Item> {
        self.items().find(|item| item.id == id)
    }
}

/// Depth-first iterator over a project's items.
pub struct Items<'a> {
    stack: Vec<std::slice::Iter<'a, Item>>,
}

impl<'a> Iterator for Items<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(iter) = self.stack.last_mut() {
            if let Some(item) = iter.next() {
                if !item.folder_contents.is_empty() {
                    self.stack.push(item.folder_contents.iter());
                }
                return Some(item);
            }
            self.stack.pop();
        }
        None
    }
}

/// Open and parse an After Effects project file.
pub fn open(path: impl AsRef<Path>) -> AeResult<Project> {
    let file = osb_core::io::open(path).map_err(std::io::Error::from)?;
    from_reader(file)
}

/// Parse an After Effects project from a reader.
pub fn from_reader<R: Read>(mut reader: R) -> AeResult<Project> {
    let root = rifx::read_root(&mut reader)?;
    parse_project(&root)
}

/// Parse an After Effects project from an in-memory buffer.
pub fn from_bytes(bytes: &[u8]) -> AeResult<Project> {
    from_reader(std::io::Cursor::new(bytes))
}

fn parse_project(root: &List) -> AeResult<Project> {
    let expression_engine = root
        .sublist_find("ExEn")
        .and_then(|list| list.blocks.first())
        .map(Block::text)
        .unwrap_or_default();

    let nhed = root
        .find_type("nhed")
        .ok_or_else(|| AeError::Malformed("project has no nhed block".into()))?;
    let depth = nhed.byte(15).map(Bpc::from_u8).unwrap_or_default();

    let root_list = root
        .sublist_find("Fold")
        .ok_or_else(|| AeError::Malformed("project has no root folder (Fold)".into()))?;
    let mut root_folder = item::parse_item(root_list)?;

    fix_unnamed_layers(&mut root_folder);

    Ok(Project {
        expression_engine,
        depth,
        root_folder,
    })
}

/// Layers without an explicit name inherit their source item's name.
fn fix_unnamed_layers(root: &mut Item) {
    let mut names = HashMap::new();
    collect_names(root, &mut names);
    apply_names(root, &names);
}

fn collect_names(item: &Item, out: &mut HashMap<u32, String>) {
    out.insert(item.id, item.name.clone());
    for child in &item.folder_contents {
        collect_names(child, out);
    }
}

fn apply_names(item: &mut Item, names: &HashMap<u32, String>) {
    if item.item_type == ItemType::Composition {
        for layer in &mut item.composition_layers {
            if layer.name.is_empty() {
                if let Some(name) = names.get(&layer.source_id) {
                    layer.name = name.clone();
                }
            }
        }
    }
    for child in &mut item.folder_contents {
        apply_names(child, names);
    }
}
