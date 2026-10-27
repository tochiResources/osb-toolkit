//! Layer/composition property (parameter) parsing.
//!
//! Properties live in `tdgp`/`parT` groups and carry a match name, an optional
//! user label and a value type. Effects are the `ADBE Effect Parade` group, and
//! text layers carry an `ADBE Text Properties` group.

use std::collections::HashMap;

use crate::error::AeResult;
use crate::rifx::{trim_nul, Block, List};

/// A keyframe's interpolation type (the `AEGP_KeyInterp` values).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interpolation {
    /// No interpolation recorded.
    #[default]
    Unknown,
    /// Linear interpolation.
    Linear,
    /// Bezier interpolation.
    Bezier,
    /// Hold (no interpolation).
    Hold,
}

impl Interpolation {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Linear,
            2 => Self::Bezier,
            3 => Self::Hold,
            _ => Self::Unknown,
        }
    }

    /// The human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Linear => "Linear",
            Self::Bezier => "Bezier",
            Self::Hold => "Hold",
        }
    }
}

/// One keyframe of an animated property stream.
///
/// The value and easing layout follows the py-aep `.aep` specification: numeric
/// streams carry per-dimension speeds and influences, spatial and colour streams
/// carry one temporal ease for the whole value plus spatial tangents.
#[derive(Debug, Clone, Default)]
pub struct Keyframe {
    /// The raw time in the stream's time units.
    pub time_units: i32,
    /// The time in seconds (`time_units / time_base`).
    pub time_seconds: f64,
    /// In-side interpolation.
    pub in_interpolation: Interpolation,
    /// Out-side interpolation.
    pub out_interpolation: Interpolation,
    /// The temporal-continuous flag.
    pub temporal_continuous: bool,
    /// The temporal auto-Bezier flag.
    pub temporal_auto_bezier: bool,
    /// The roving flag (spatial streams only).
    pub roving: bool,
    /// The keyframe label colour (0-16).
    pub label: u8,
    /// The spatial-continuous flag.
    pub spatial_continuous: bool,
    /// The spatial auto-Bezier flag.
    pub spatial_auto_bezier: bool,
    /// The value, one element per dimension.
    pub value: Vec<f64>,
    /// In spatial tangent, as an offset from the value (spatial streams only).
    pub in_tangent: Vec<f64>,
    /// Out spatial tangent, as an offset from the value (spatial streams only).
    pub out_tangent: Vec<f64>,
    /// In temporal speed, per dimension (spatial streams: one value).
    pub in_speed: Vec<f64>,
    /// In temporal influence, per dimension (a fraction).
    pub in_influence: Vec<f64>,
    /// Out temporal speed, per dimension.
    pub out_speed: Vec<f64>,
    /// Out temporal influence, per dimension.
    pub out_influence: Vec<f64>,
    /// The arc length of the segment to the next keyframe.
    pub segment_length: f64,
}

/// The value data of a property stream: a static value or a keyframe list, plus
/// any expression.
#[derive(Debug, Clone, Default)]
pub struct Stream {
    /// Number of value dimensions (1-3, or 4 for a colour).
    pub dimensions: u16,
    /// Whether the stream is animated (holds keyframes rather than a `cdat`).
    pub animated: bool,
    /// A valueless stream (text, markers, paths, gradients, orientation).
    pub valueless: bool,
    /// The values form a spatial motion path (Position, Anchor Point, points).
    pub spatial: bool,
    /// Keyframe time units per second of the owning layer's time.
    pub time_base: u32,
    /// The static value, when the stream is not animated.
    pub static_value: Vec<f64>,
    /// The keyframes, when the stream is animated.
    pub keyframes: Vec<Keyframe>,
    /// The expression source, when the stream has one.
    pub expression: Option<String>,
    /// Whether the expression is enabled.
    pub expression_enabled: bool,
}

fn be_u16_at(data: &[u8], offset: usize) -> Option<u16> {
    let slice = data.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([slice[0], slice[1]]))
}

fn be_u32_at(data: &[u8], offset: usize) -> Option<u32> {
    let slice = data.get(offset..offset + 4)?;
    Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn be_i32_at(data: &[u8], offset: usize) -> Option<i32> {
    Some(be_u32_at(data, offset)? as i32)
}

fn be_f64_at(data: &[u8], offset: usize) -> Option<f64> {
    let slice = data.get(offset..offset + 8)?;
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    Some(f64::from_be_bytes(bytes))
}

fn le_f64_at(data: &[u8], offset: usize) -> Option<f64> {
    let slice = data.get(offset..offset + 8)?;
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    Some(f64::from_le_bytes(bytes))
}

fn read_doubles(data: &[u8], offset: usize, count: usize) -> Vec<f64> {
    (0..count)
        .filter_map(|i| be_f64_at(data, offset + i * 8))
        .collect()
}

/// A property's value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyType {
    /// Single layer selection.
    LayerSelect,
    /// One-dimensional slider (also used for the `0x0a` shorthand).
    OneD,
    /// One-dimensional angle.
    Angle,
    /// Boolean checkbox.
    Boolean,
    /// Four-dimensional colour.
    Color,
    /// Two-dimensional point.
    TwoD,
    /// Single-valued selection.
    Select,
    /// A group/collection of properties.
    Group,
    /// Three-dimensional point.
    ThreeD,
    /// Unknown/custom type.
    #[default]
    Custom,
}

impl PropertyType {
    fn from_u16(value: u16) -> Self {
        match value {
            0x00 => Self::LayerSelect,
            0x02 | 0x0a => Self::OneD,
            0x03 => Self::Angle,
            0x04 => Self::Boolean,
            0x05 => Self::Color,
            0x06 => Self::TwoD,
            0x07 => Self::Select,
            0x0d => Self::Group,
            0x12 => Self::ThreeD,
            _ => Self::Custom,
        }
    }

    /// The human-readable type name.
    pub fn name(self) -> &'static str {
        match self {
            Self::LayerSelect => "LayerSelect",
            Self::OneD => "OneD",
            Self::Angle => "Angle",
            Self::Boolean => "Boolean",
            Self::Color => "Color",
            Self::TwoD => "TwoD",
            Self::Select => "Select",
            Self::Group => "Group",
            Self::ThreeD => "ThreeD",
            Self::Custom => "Custom",
        }
    }
}

/// A property object on a layer or nested inside another property.
#[derive(Debug, Clone, Default)]
pub struct Property {
    /// The After Effects match name (e.g. `ADBE Effect Parade`).
    pub match_name: String,
    /// The display name.
    pub name: String,
    /// A user-defined label, when one was set.
    pub label: String,
    /// The property's 1-based index within its parent.
    pub index: u32,
    /// The value type.
    pub property_type: PropertyType,
    /// Nested child properties.
    pub properties: Vec<Property>,
    /// Options for a `Select` property.
    pub select_options: Vec<String>,
    /// The property's value stream, for leaf properties.
    pub stream: Option<Stream>,
}

/// Either a list of blocks (`tdgp`) or a loose block sequence (`parT`).
pub(crate) enum PropInput<'a> {
    List(&'a List),
    Nodes(&'a [Node<'a>]),
}

/// A member of a grouped block sequence: a nested list or a plain block.
pub(crate) enum Node<'a> {
    List(&'a List),
    Block(&'a Block),
}

/// Group a `tdmn`-headed block sequence into match-name buckets.
///
/// Each `tdmn` block starts a group; following blocks belong to it until the
/// next `tdmn`. `ADBE Group End`/`ADBE Effect Built In Params` markers skip the
/// blocks that follow them.
fn pair_match_names<'a>(head: &'a List) -> (Vec<String>, Vec<Vec<Node<'a>>>) {
    let mut match_names: Vec<String> = Vec::new();
    let mut datum: Vec<Vec<Node<'a>>> = Vec::new();
    let mut group_idx: isize = -1;
    let mut skip = false;

    for block in &head.blocks {
        if block.id == "tdmn" {
            let match_name = block.text();
            if match_name == "ADBE Group End" || match_name == "ADBE Effect Built In Params" {
                skip = true;
                continue;
            }
            match_names.push(match_name);
            skip = false;
            group_idx += 1;
        } else if group_idx >= 0 && !skip {
            let idx = group_idx as usize;
            if idx >= datum.len() {
                datum.push(Vec::new());
            }
            match block.data {
                crate::rifx::Data::List(ref list) => datum[idx].push(Node::List(list)),
                crate::rifx::Data::Bytes(_) => datum[idx].push(Node::Block(block)),
            }
        }
    }

    (match_names, datum)
}

/// Map every match name to the first nested list in its group.
pub(crate) fn indexed_group_to_map(head: &List) -> (Vec<String>, HashMap<String, &List>) {
    let (names, contents) = pair_match_names(head);
    let mut map = HashMap::new();
    for (idx, name) in names.iter().enumerate() {
        if let Some(Node::List(list)) = contents.get(idx).and_then(|nodes| nodes.first()) {
            map.insert(name.clone(), *list);
        }
    }
    (names, map)
}

/// Parse a property from a list (`tdgp`) or a block sequence (`parT`).
pub(crate) fn parse_property(input: PropInput<'_>, match_name: &str) -> AeResult<Property> {
    let mut prop = Property {
        match_name: match_name.to_string(),
        name: match_name.to_string(),
        ..Property::default()
    };
    if match_name == "ADBE Effect Parade" {
        prop.name = "Effects".to_string();
    }

    match input {
        PropInput::List(head) => {
            // A leaf property's value stream, or an orientation wrapper holding one.
            if head.identifier == "tdbs" {
                return Ok(parse_stream(head, match_name));
            }
            if head.identifier == "otst" {
                if let Some(tdbs) = head.sublist_find("tdbs") {
                    return Ok(parse_stream(tdbs, match_name));
                }
            }

            let (names, map) = indexed_group_to_map(head);
            for (idx, name) in names.iter().enumerate() {
                if let Some(list) = map.get(name) {
                    if let Ok(mut sub) = parse_property(PropInput::List(list), name) {
                        sub.index = (idx + 1) as u32;
                        prop.properties.push(sub);
                    }
                }
            }

            // `sspc` groups carry their own name/label and a `parT` block sequence.
            if head.identifier == "sspc" {
                prop.property_type = PropertyType::Group;
                if let Some(fnam) = head.find_type("fnam") {
                    prop.name = fnam.text();
                }
                if let Some(tdgp) = head.sublist_find("tdgp") {
                    if let Some(tdsn) = tdgp.find_type("tdsn") {
                        let label = tdsn.text();
                        if label != "-_0_/-" {
                            prop.label = label;
                        }
                    }
                }
                let par_t = head.sublist_merge("parT");
                let (sub_names, sub_pards) = pair_match_names(&par_t);
                for (idx, name) in sub_names.iter().enumerate() {
                    if idx == 0 {
                        continue;
                    }
                    if let Some(nodes) = sub_pards.get(idx) {
                        if let Ok(mut sub) = parse_property(PropInput::Nodes(nodes), name) {
                            sub.index = idx as u32;
                            prop.properties.push(sub);
                        }
                    }
                }
            }
        }
        PropInput::Nodes(nodes) => {
            for node in nodes {
                let Node::Block(block) = node else {
                    continue;
                };
                match block.id.as_str() {
                    "pdnm" => {
                        let text = block.text();
                        if prop.property_type == PropertyType::Select {
                            prop.select_options = text.split('|').map(str::to_string).collect();
                        } else if !text.is_empty() {
                            prop.name = text;
                        }
                    }
                    "pard" => {
                        if let Some(kind) = block.be_u16(14) {
                            prop.property_type = PropertyType::from_u16(kind);
                        }
                        if let Some(bytes) = block.bytes() {
                            let end = 48.min(bytes.len());
                            let start = 16.min(end);
                            let name = trim_nul(&bytes[start..end]);
                            if !name.is_empty() {
                                prop.name = name;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(prop)
}

/// Parse a `LIST:tdbs` value stream into a leaf property.
fn parse_stream(list: &List, match_name: &str) -> Property {
    let mut prop = Property {
        match_name: match_name.to_string(),
        name: match_name.to_string(),
        ..Property::default()
    };

    if let Some(tdsn) = list.find_type("tdsn") {
        let label = tdsn.text();
        if !label.is_empty() && label != "-_0_/-" {
            prop.label = label;
        }
    }

    let Some(bytes) = list.find_type("tdb4").and_then(Block::bytes) else {
        return prop;
    };

    let dimensions = be_u16_at(bytes, 0x02).unwrap_or(1);
    let flags = be_u16_at(bytes, 0x04).unwrap_or(0);
    let valueless = be_u16_at(bytes, 0x38).is_some_and(|value| value & 1 != 0);
    let time_base = be_u32_at(bytes, 0x0C).unwrap_or(0);
    let expression_flags = be_u32_at(bytes, 0x74).unwrap_or(0);
    let has_expression = be_u32_at(bytes, 0x78) == Some(0x0100_0000);
    let single_ease = flags & 0x0002 != 0;

    let mut stream = Stream {
        dimensions,
        animated: flags & 0x0001 == 0,
        valueless,
        spatial: flags & 0x0008 != 0,
        time_base,
        expression: has_expression
            .then(|| list.find_type("Utf8").map(Block::text))
            .flatten(),
        expression_enabled: has_expression && expression_flags & 1 == 0,
        ..Stream::default()
    };

    if !stream.animated {
        if let Some(data) = list.find_type("cdat").and_then(Block::bytes) {
            stream.static_value = parse_static_value(data, dimensions, valueless);
        }
    } else if let Some(keyframe_list) = list.sublist_find("list") {
        stream.keyframes =
            parse_keyframes(keyframe_list, dimensions, single_ease, valueless, time_base);
    }

    prop.stream = Some(stream);
    prop
}

/// The static value of a `cdat`: the first `dimensions` doubles.
fn parse_static_value(data: &[u8], dimensions: u16, valueless: bool) -> Vec<f64> {
    if valueless {
        return Vec::new();
    }
    read_doubles(data, 0, dimensions as usize)
}

/// Parse the keyframe items of a `LIST:list` (`lhd3` header + `ldat` records).
fn parse_keyframes(
    list: &List,
    dimensions: u16,
    single_ease: bool,
    valueless: bool,
    time_base: u32,
) -> Vec<Keyframe> {
    let Some(header) = list.find_type("lhd3") else {
        return Vec::new();
    };
    let count = header.be_u32(0x08).unwrap_or(0) as usize;
    let item_size = header.be_u32(0x10).unwrap_or(0) as usize;
    let Some(data) = list.find_type("ldat").and_then(Block::bytes) else {
        return Vec::new();
    };
    if item_size == 0 {
        return Vec::new();
    }

    let mut keyframes = Vec::new();
    for index in 0..count {
        let start = index * item_size;
        let Some(item) = data.get(start..start + item_size) else {
            break;
        };
        keyframes.push(parse_keyframe(
            item,
            dimensions,
            single_ease,
            valueless,
            time_base,
        ));
    }
    keyframes
}

/// Parse one keyframe record.
fn parse_keyframe(
    item: &[u8],
    dimensions: u16,
    single_ease: bool,
    valueless: bool,
    time_base: u32,
) -> Keyframe {
    let time_units = be_i32_at(item, 0).unwrap_or(0);
    let key_flags = be_u16_at(item, 0x06).unwrap_or(0);

    let mut keyframe = Keyframe {
        time_units,
        time_seconds: if time_base == 0 {
            0.0
        } else {
            time_units as f64 / time_base as f64
        },
        in_interpolation: Interpolation::from_u8(item.get(0x04).copied().unwrap_or(0)),
        out_interpolation: Interpolation::from_u8(item.get(0x05).copied().unwrap_or(0)),
        temporal_continuous: key_flags & 0x0008 != 0,
        temporal_auto_bezier: key_flags & 0x0010 != 0,
        roving: key_flags & 0x0020 != 0,
        label: ((key_flags >> 6) & 0x1F) as u8,
        ..Keyframe::default()
    };

    let dims = dimensions as usize;

    if single_ease {
        let spatial_flags = be_u32_at(item, 0x08).unwrap_or(0);
        keyframe.spatial_continuous = spatial_flags & 1 != 0;
        keyframe.spatial_auto_bezier = spatial_flags & 2 != 0;
        keyframe.segment_length = be_f64_at(item, 0x10).unwrap_or(0.0);
        keyframe.in_speed = vec![be_f64_at(item, 0x18).unwrap_or(0.0)];
        keyframe.in_influence = vec![be_f64_at(item, 0x20).unwrap_or(0.0)];
        keyframe.out_speed = vec![be_f64_at(item, 0x28).unwrap_or(0.0)];
        keyframe.out_influence = vec![be_f64_at(item, 0x30).unwrap_or(0.0)];

        // Value block at 0x38: orientation is 3 little-endian angles, colours are
        // 4 doubles, spatial streams are value + in/out tangents.
        let remaining = item.len().saturating_sub(0x38);
        if valueless {
            if remaining >= 24 {
                keyframe.value = (0..3)
                    .filter_map(|i| le_f64_at(item, 0x38 + i * 8))
                    .collect();
            }
        } else if dims == 4 {
            keyframe.value = read_doubles(item, 0x38, 4);
        } else {
            keyframe.value = read_doubles(item, 0x38, dims);
            keyframe.in_tangent = read_doubles(item, 0x38 + dims * 8, dims);
            keyframe.out_tangent = read_doubles(item, 0x38 + 2 * dims * 8, dims);
        }
    } else if !valueless {
        // value, in speed, in influence, out speed, out influence — `dims` each.
        keyframe.value = read_doubles(item, 0x08, dims);
        keyframe.in_speed = read_doubles(item, 0x08 + dims * 8, dims);
        keyframe.in_influence = read_doubles(item, 0x08 + 2 * dims * 8, dims);
        keyframe.out_speed = read_doubles(item, 0x08 + 3 * dims * 8, dims);
        keyframe.out_influence = read_doubles(item, 0x08 + 4 * dims * 8, dims);
    }

    keyframe
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rifx::Data;

    fn block(id: &str, data: Vec<u8>) -> Block {
        Block {
            id: id.to_string(),
            size: data.len() as u32,
            data: Data::Bytes(data),
        }
    }

    fn list(id: &str, blocks: Vec<Block>) -> Block {
        Block {
            id: "LIST".to_string(),
            size: 0,
            data: Data::List(List {
                identifier: id.to_string(),
                blocks,
            }),
        }
    }

    fn put_f64(data: &mut [u8], offset: usize, value: f64) {
        data[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
    }

    fn tdb4(dimensions: u16, flags: u16, has_expression: bool) -> Vec<u8> {
        let mut bytes = vec![0u8; 124];
        bytes[2..4].copy_from_slice(&dimensions.to_be_bytes());
        bytes[4..6].copy_from_slice(&flags.to_be_bytes());
        bytes[0x0C..0x10].copy_from_slice(&24576u32.to_be_bytes());
        if has_expression {
            bytes[0x78..0x7C].copy_from_slice(&0x0100_0000u32.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn parses_a_spatial_keyframe_with_expression() {
        let mut item = vec![0u8; 128];
        item[0..4].copy_from_slice(&0i32.to_be_bytes());
        item[4] = 2; // in Bezier
        item[5] = 2; // out Bezier
        item[6..8].copy_from_slice(&0x0008u16.to_be_bytes()); // temporal continuous
        item[8..12].copy_from_slice(&1u32.to_be_bytes()); // spatial continuous
        put_f64(&mut item, 0x10, 0.0); // segment length
        put_f64(&mut item, 0x18, 0.5); // in speed
        put_f64(&mut item, 0x20, 0.33); // in influence
        put_f64(&mut item, 0x28, 0.7); // out speed
        put_f64(&mut item, 0x30, 0.2); // out influence
        put_f64(&mut item, 0x38, 100.0);
        put_f64(&mut item, 0x40, 200.0);
        put_f64(&mut item, 0x48, 0.0);
        put_f64(&mut item, 0x50, -5.0); // in tangent x
        put_f64(&mut item, 0x68, 5.0); // out tangent x

        let mut lhd3 = vec![0u8; 52];
        lhd3[0x08..0x0C].copy_from_slice(&1u32.to_be_bytes());
        lhd3[0x10..0x14].copy_from_slice(&128u32.to_be_bytes());

        let tdbs = List {
            identifier: "tdbs".to_string(),
            blocks: vec![
                block("tdb4", tdb4(3, 0x000E, true)),
                list("list", vec![block("lhd3", lhd3), block("ldat", item)]),
                block("Utf8", b"wiggle(1,10)".to_vec()),
            ],
        };

        let prop = parse_stream(&tdbs, "ADBE Position");
        let stream = prop.stream.expect("stream");
        assert!(stream.animated);
        assert!(stream.spatial);
        assert_eq!(stream.dimensions, 3);
        assert_eq!(stream.time_base, 24576);
        assert_eq!(stream.expression.as_deref(), Some("wiggle(1,10)"));
        assert!(stream.expression_enabled);

        assert_eq!(stream.keyframes.len(), 1);
        let keyframe = &stream.keyframes[0];
        assert_eq!(keyframe.time_seconds, 0.0);
        assert_eq!(keyframe.in_interpolation, Interpolation::Bezier);
        assert_eq!(keyframe.out_interpolation, Interpolation::Bezier);
        assert!(keyframe.temporal_continuous);
        assert!(keyframe.spatial_continuous);
        assert_eq!(keyframe.value, vec![100.0, 200.0, 0.0]);
        assert_eq!(keyframe.in_tangent, vec![-5.0, 0.0, 0.0]);
        assert_eq!(keyframe.out_tangent, vec![5.0, 0.0, 0.0]);
        assert_eq!(keyframe.in_speed, vec![0.5]);
        assert_eq!(keyframe.in_influence, vec![0.33]);
    }

    #[test]
    fn parses_a_static_numeric_value() {
        let mut cdat = vec![0u8; 40];
        put_f64(&mut cdat, 0, 42.0);

        let tdbs = List {
            identifier: "tdbs".to_string(),
            blocks: vec![block("tdb4", tdb4(1, 0x0001, false)), block("cdat", cdat)],
        };

        let prop = parse_stream(&tdbs, "ADBE Opacity");
        let stream = prop.stream.expect("stream");
        assert!(!stream.animated);
        assert!(stream.expression.is_none());
        assert_eq!(stream.static_value, vec![42.0]);
        assert!(stream.keyframes.is_empty());
    }

    #[test]
    fn parses_a_numeric_keyframe_with_per_dimension_ease() {
        let mut item = vec![0u8; 48];
        item[0..4].copy_from_slice(&24576i32.to_be_bytes()); // 1 second
        item[4] = 1;
        item[5] = 1;
        put_f64(&mut item, 0x08, 60.0); // value
        put_f64(&mut item, 0x10, 1.0); // in speed
        put_f64(&mut item, 0x18, 0.166_666_666_67); // in influence
        put_f64(&mut item, 0x20, 2.0); // out speed
        put_f64(&mut item, 0x28, 0.75); // out influence

        let mut lhd3 = vec![0u8; 52];
        lhd3[0x08..0x0C].copy_from_slice(&1u32.to_be_bytes());
        lhd3[0x10..0x14].copy_from_slice(&48u32.to_be_bytes());

        let tdbs = List {
            identifier: "tdbs".to_string(),
            blocks: vec![
                block("tdb4", tdb4(1, 0x0000, false)),
                list("list", vec![block("lhd3", lhd3), block("ldat", item)]),
            ],
        };

        let prop = parse_stream(&tdbs, "ADBE Opacity");
        let stream = prop.stream.expect("stream");
        assert!(stream.animated);
        assert!(!stream.spatial);
        let keyframe = &stream.keyframes[0];
        assert_eq!(keyframe.time_units, 24576);
        assert_eq!(keyframe.time_seconds, 1.0);
        assert_eq!(keyframe.in_interpolation, Interpolation::Linear);
        assert_eq!(keyframe.value, vec![60.0]);
        assert_eq!(keyframe.in_speed, vec![1.0]);
        assert_eq!(keyframe.out_influence, vec![0.75]);
    }
}
