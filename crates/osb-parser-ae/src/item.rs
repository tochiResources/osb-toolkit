//! Project item (folder / composition / footage) parsing.

use crate::error::{AeError, AeResult};
use crate::layer::{parse_layer, Layer};
use crate::rifx::List;

/// The kind of a project item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemType {
    /// A folder containing other items.
    Folder,
    /// A composition with dimensions, duration and layers.
    Composition,
    /// Imported footage with a source file.
    Footage,
    /// An unrecognised item type.
    #[default]
    Unknown,
}

/// The kind of source behind a footage item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FootageType {
    /// A solid colour source.
    Solid,
    /// A placeholder source.
    Placeholder,
    /// Any other source.
    #[default]
    Other,
}

impl FootageType {
    fn from_u16(value: u16) -> Self {
        match value {
            0x09 => Self::Solid,
            0x02 => Self::Placeholder,
            _ => Self::Other,
        }
    }
}

/// A generalised project item: a folder, composition or footage.
#[derive(Debug, Clone, Default)]
pub struct Item {
    /// The project-wide item id (`0` for the root folder).
    pub id: u32,
    /// The item name.
    pub name: String,
    /// The item kind.
    pub item_type: ItemType,
    /// Child items (folders only).
    pub folder_contents: Vec<Item>,
    /// `[width, height]` for footage and compositions.
    pub footage_dimensions: [u16; 2],
    /// Frame rate for footage and compositions.
    pub footage_framerate: f64,
    /// Duration in seconds for footage and compositions.
    pub footage_seconds: f64,
    /// The footage source kind.
    pub footage_type: FootageType,
    /// Background colour for compositions.
    pub background_color: [u8; 3],
    /// Layers (compositions only).
    pub composition_layers: Vec<Layer>,
}

/// The trailing ASCII file name inside an `opti` block (image footage), if any.
fn trailing_file_name(data: &[u8]) -> Option<String> {
    let end = data.iter().rposition(|&byte| byte != 0)? + 1;
    let mut start = end;
    while start > 0 && (0x20..=0x7e).contains(&data[start - 1]) {
        start -= 1;
    }
    if end - start < 3 {
        return None;
    }
    let name = String::from_utf8_lossy(&data[start..end]).into_owned();
    name.contains('.').then_some(name)
}

/// The file name from an `Als2`/`alas` asset descriptor (`"fullpath":"…"`).
fn asset_file_name(pin: &List) -> Option<String> {
    let text = pin.sublist_find("Als2")?.find_type("alas")?.text();
    let key = "\"fullpath\":\"";
    let start = text.find(key)? + key.len();
    let rest = &text[start..];
    let end = rest.find('"')?;
    let path = rest[..end].replace("\\\\", "\\");
    let name = path.rsplit(['\\', '/']).next()?;
    (!name.is_empty()).then(|| name.to_string())
}

fn replace_nul_with_space(data: &[u8]) -> String {
    let start = data
        .iter()
        .position(|&byte| byte != 0)
        .unwrap_or(data.len());
    let end = data
        .iter()
        .rposition(|&byte| byte != 0)
        .map_or(start, |i| i + 1);
    let bytes: Vec<u8> = data[start..end]
        .iter()
        .map(|&byte| if byte == 0 { b' ' } else { byte })
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

pub(crate) fn parse_item(list: &List) -> AeResult<Item> {
    let is_root = list.identifier == "Fold";
    let mut item = Item::default();

    if is_root {
        item.id = 0;
        item.name = "root".to_string();
        item.item_type = ItemType::Folder;
    } else {
        let name = list
            .find_type("Utf8")
            .ok_or_else(|| AeError::Malformed("item has no Utf8 name".into()))?;
        item.name = name.text();

        let idta = list
            .find_type("idta")
            .ok_or_else(|| AeError::Malformed("item has no idta block".into()))?;
        item.id = idta.be_u32(16).unwrap_or(0);
        item.item_type = match idta.be_u16(0).unwrap_or(0) {
            0x01 => ItemType::Folder,
            0x04 => ItemType::Composition,
            0x07 => ItemType::Footage,
            _ => ItemType::Unknown,
        };
    }

    match item.item_type {
        ItemType::Folder => {
            let sfdr = list.sublist_merge("Sfdr");
            let mut children = list.sublists("Item");
            children.extend(sfdr.sublists("Item"));
            for child in children {
                item.folder_contents.push(parse_item(child)?);
            }
        }
        ItemType::Footage => {
            parse_footage(list, &mut item)?;
        }
        ItemType::Composition => {
            parse_composition(list, &mut item)?;
        }
        ItemType::Unknown => {}
    }

    Ok(item)
}

fn parse_footage(list: &List, item: &mut Item) -> AeResult<()> {
    let pin = list
        .sublist_find("Pin ")
        .ok_or_else(|| AeError::Malformed("footage has no Pin list".into()))?;
    let sspc = pin
        .find_type("sspc")
        .ok_or_else(|| AeError::Malformed("footage has no sspc block".into()))?;

    item.footage_dimensions = [
        sspc.be_u32(30).unwrap_or(0) as u16,
        sspc.be_u32(34).unwrap_or(0) as u16,
    ];
    let framerate = sspc.be_u32(56).unwrap_or(0);
    let framerate_fraction = sspc.be_u16(60).unwrap_or(0);
    item.footage_framerate = framerate as f64 + framerate_fraction as f64 / (1u32 << 16) as f64;
    let seconds_dividend = sspc.be_u32(38).unwrap_or(0);
    let seconds_divisor = sspc.be_u32(42).unwrap_or(0);
    item.footage_seconds = if seconds_divisor == 0 {
        0.0
    } else {
        seconds_dividend as f64 / seconds_divisor as f64
    };

    if let Some(opti) = pin.find_type("opti") {
        if let Some(bytes) = opti.bytes() {
            if bytes.len() >= 6 {
                item.footage_type = FootageType::from_u16(u16::from_be_bytes([bytes[4], bytes[5]]));
            }
            match item.footage_type {
                FootageType::Solid => {
                    let end = 255.min(bytes.len());
                    let start = 26.min(end);
                    item.name = replace_nul_with_space(&bytes[start..end]);
                }
                FootageType::Placeholder => {
                    let start = 10.min(bytes.len());
                    item.name = replace_nul_with_space(&bytes[start..]);
                }
                FootageType::Other => {}
            }
            // Imported footage stores its file name at the end of `opti` (images)
            // or in the `alas` asset descriptor (audio/video) rather than in the
            // item's (empty) `Utf8` block.
            if item.name.is_empty() {
                item.name = trailing_file_name(bytes)
                    .or_else(|| asset_file_name(pin))
                    .unwrap_or_default();
            }
        }
    }

    Ok(())
}

fn parse_composition(list: &List, item: &mut Item) -> AeResult<()> {
    let cdta = list
        .find_type("cdta")
        .ok_or_else(|| AeError::Malformed("composition has no cdta block".into()))?;

    // `cdta` layout (offsets follow the struct, not the reference's stale
    // comments): fps rational at 4/8, seconds rational at 44/48, background at
    // 52, width/height at 140/142, frame rate at 156.
    item.footage_dimensions = [cdta.be_u16(140).unwrap_or(0), cdta.be_u16(142).unwrap_or(0)];
    let framerate_divisor = cdta.be_u32(4).unwrap_or(0);
    let framerate_dividend = cdta.be_u32(8).unwrap_or(0);
    item.footage_framerate = if framerate_divisor == 0 {
        0.0
    } else {
        framerate_dividend as f64 / framerate_divisor as f64
    };
    let seconds_dividend = cdta.be_u32(44).unwrap_or(0);
    let seconds_divisor = cdta.be_u32(48).unwrap_or(0);
    item.footage_seconds = if seconds_divisor == 0 {
        0.0
    } else {
        seconds_dividend as f64 / seconds_divisor as f64
    };
    if let Some(bytes) = cdta.bytes() {
        if bytes.len() >= 55 {
            item.background_color = [bytes[52], bytes[53], bytes[54]];
        }
    }

    for (index, layer_list) in list.sublists("Layr").into_iter().enumerate() {
        let mut layer = parse_layer(layer_list)?;
        layer.index = (index + 1) as u32;
        item.composition_layers.push(layer);
    }

    Ok(())
}
