//! glTF 2.0 / GLB mesh importer ().
//!
//! Reads the position stream of every `TRIANGLES` primitive in the default
//! scene, applies the node hierarchy transforms, and welds the result into the
//! shared [`ParsedMesh`]. Buffers come from a GLB BIN chunk or an embedded
//! `data:` URI; external buffer files are not read (the importer is I/O-free).

use serde::Deserialize;

use crate::math::{Matrix4, Quaternion, Vec3, Vec4};

use super::parser::{ObjParseOptions, ParsedMesh, WeldedMesh};

/// Parses a glTF (`.gltf` JSON) or GLB (`.glb` binary) document.
pub fn parse_gltf(bytes: &[u8], options: &ObjParseOptions) -> ParsedMesh {
    let (json, bin) = match parse_glb(bytes) {
        Some((json, bin)) => (json, Some(bin)),
        None => (String::from_utf8_lossy(bytes).into_owned(), None),
    };
    let gltf: Gltf = match serde_json::from_str(&json) {
        Ok(gltf) => gltf,
        Err(_) => return ParsedMesh::default(),
    };
    let buffers = resolve_buffers(&gltf, bin.as_deref());

    let mut mesh = WeldedMesh::new(options);
    let roots: Vec<usize> = match gltf.scene.and_then(|index| gltf.scenes.get(index)) {
        Some(scene) => scene.nodes.clone(),
        None => (0..gltf.nodes.len()).collect(),
    };
    for root in roots {
        if root < gltf.nodes.len() {
            collect_node(&gltf, root, &Matrix4::identity(), &buffers, &mut mesh);
        }
    }
    mesh.finish()
}

/// The GLB container magic (`"glTF"` little-endian).
const GLB_MAGIC: u32 = 0x4654_6C67;
const CHUNK_JSON: u32 = 0x4E4F_534A;
const CHUNK_BIN: u32 = 0x004E_4942;
const MODE_TRIANGLES: u32 = 4;

/// Splits a GLB into its JSON chunk and BIN chunk (empty if absent).
fn parse_glb(bytes: &[u8]) -> Option<(String, Vec<u8>)> {
    if bytes.len() < 12 || u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) != GLB_MAGIC
    {
        return None;
    }
    let mut offset = 12;
    let mut json = None;
    let mut bin = Vec::new();
    while offset + 8 <= bytes.len() {
        let length = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        let kind = u32::from_le_bytes([
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]);
        let start = offset + 8;
        let end = start + length;
        if end > bytes.len() {
            break;
        }
        match kind {
            CHUNK_JSON => json = Some(String::from_utf8_lossy(&bytes[start..end]).into_owned()),
            CHUNK_BIN => bin = bytes[start..end].to_vec(),
            _ => {}
        }
        offset = end;
    }
    Some((json?, bin))
}

fn resolve_buffers(gltf: &Gltf, bin: Option<&[u8]>) -> Vec<Option<Vec<u8>>> {
    gltf.buffers
        .iter()
        .enumerate()
        .map(|(index, buffer)| {
            if let Some(uri) = &buffer.uri {
                uri.split(";base64,").nth(1).and_then(decode_base64)
            } else if index == 0 {
                bin.map(<[u8]>::to_vec)
            } else {
                None
            }
        })
        .collect()
}

fn collect_node(
    gltf: &Gltf,
    index: usize,
    parent: &Matrix4,
    buffers: &[Option<Vec<u8>>],
    mesh: &mut WeldedMesh,
) {
    let node = &gltf.nodes[index];
    let world = parent.mul(&node_matrix(node));
    if let Some(mesh_index) = node.mesh {
        if let Some(mesh_def) = gltf.meshes.get(mesh_index) {
            for primitive in &mesh_def.primitives {
                add_primitive(gltf, primitive, &world, buffers, mesh);
            }
        }
    }
    for &child in &node.children {
        if child < gltf.nodes.len() {
            collect_node(gltf, child, &world, buffers, mesh);
        }
    }
}

fn add_primitive(
    gltf: &Gltf,
    primitive: &Primitive,
    world: &Matrix4,
    buffers: &[Option<Vec<u8>>],
    mesh: &mut WeldedMesh,
) {
    if primitive.mode != MODE_TRIANGLES {
        return;
    }
    let Some(position_index) = primitive.attributes.position else {
        return;
    };
    let Some(accessor) = gltf.accessors.get(position_index) else {
        return;
    };
    let positions = read_vec3(accessor, &gltf.buffer_views, buffers);
    if positions.is_empty() {
        return;
    }
    let welded: Vec<u32> = positions
        .iter()
        .map(|p| {
            let world_point = world.transform_vec4(Vec4::new(p.x, p.y, p.z, 1.0));
            mesh.vertex(world_point.x, world_point.y, world_point.z)
        })
        .collect();

    match primitive.indices {
        Some(indices) => {
            let Some(accessor) = gltf.accessors.get(indices) else {
                return;
            };
            let indices = read_u32(accessor, &gltf.buffer_views, buffers);
            for triangle in indices.chunks(3) {
                if let [a, b, c] = triangle {
                    if let (Some(&a), Some(&b), Some(&c)) = (
                        welded.get(*a as usize),
                        welded.get(*b as usize),
                        welded.get(*c as usize),
                    ) {
                        mesh.add_face(vec![a, b, c]);
                    }
                }
            }
        }
        None => {
            for triangle in welded.chunks(3) {
                if let [a, b, c] = triangle {
                    mesh.add_face(vec![*a, *b, *c]);
                }
            }
        }
    }
}

/// The local transform of a node (an explicit `matrix`, else TRS).
fn node_matrix(node: &Node) -> Matrix4 {
    if let Some(matrix) = node.matrix {
        // glTF matrices are column-major, matching the runtime convention.
        return Matrix4 { m: matrix };
    }
    let translation = node.translation.unwrap_or([0.0, 0.0, 0.0]);
    let rotation = node.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let scale = node.scale.unwrap_or([1.0, 1.0, 1.0]);
    Matrix4::translation(Vec3::new(translation[0], translation[1], translation[2]))
        .mul(
            &Quaternion::new(rotation[0], rotation[1], rotation[2], rotation[3])
                .normalized()
                .to_matrix(),
        )
        .mul(&Matrix4::scale(Vec3::new(scale[0], scale[1], scale[2])))
}

fn read_vec3(accessor: &Accessor, views: &[BufferView], buffers: &[Option<Vec<u8>>]) -> Vec<Vec3> {
    const FLOAT: u32 = 5126;
    if accessor.component_type != FLOAT {
        return Vec::new();
    }
    let Some(buffer) = accessor_buffer(accessor, views, buffers) else {
        return Vec::new();
    };
    let stride = accessor.stride(views).unwrap_or(12);
    let base = accessor_base(accessor, views);
    let mut out = Vec::with_capacity(accessor.count);
    for i in 0..accessor.count {
        let offset = base + i * stride;
        if offset + 12 > buffer.len() {
            break;
        }
        let read = |o: usize| {
            f32::from_le_bytes([buffer[o], buffer[o + 1], buffer[o + 2], buffer[o + 3]]) as f64
        };
        out.push(Vec3::new(read(offset), read(offset + 4), read(offset + 8)));
    }
    out
}

fn read_u32(accessor: &Accessor, views: &[BufferView], buffers: &[Option<Vec<u8>>]) -> Vec<u32> {
    let size = match accessor.component_type {
        5121 => 1,
        5123 => 2,
        5125 => 4,
        _ => return Vec::new(),
    };
    let Some(buffer) = accessor_buffer(accessor, views, buffers) else {
        return Vec::new();
    };
    let stride = accessor.stride(views).unwrap_or(size);
    let base = accessor_base(accessor, views);
    let mut out = Vec::with_capacity(accessor.count);
    for i in 0..accessor.count {
        let offset = base + i * stride;
        if offset + size > buffer.len() {
            break;
        }
        let value = match accessor.component_type {
            5121 => buffer[offset] as u32,
            5123 => u16::from_le_bytes([buffer[offset], buffer[offset + 1]]) as u32,
            _ => u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]),
        };
        out.push(value);
    }
    out
}

fn accessor_base(accessor: &Accessor, views: &[BufferView]) -> usize {
    accessor
        .buffer_view
        .and_then(|index| views.get(index))
        .map(|view| view.byte_offset + accessor.byte_offset)
        .unwrap_or(accessor.byte_offset)
}

fn accessor_buffer<'a>(
    accessor: &Accessor,
    views: &[BufferView],
    buffers: &'a [Option<Vec<u8>>],
) -> Option<&'a [u8]> {
    let view = views.get(accessor.buffer_view?)?;
    buffers.get(view.buffer)?.as_deref()
}

/// A minimal standard-alphabet base64 decoder (`data:` URI buffers).
fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' | b'\n' | b'\r' | b' ' | b'\t' => continue,
            _ => return None,
        } as u32;
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
        }
    }
    Some(out)
}

#[derive(Deserialize)]
struct Gltf {
    #[serde(default)]
    scenes: Vec<Scene>,
    #[serde(default)]
    scene: Option<usize>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    meshes: Vec<MeshDef>,
    #[serde(default)]
    accessors: Vec<Accessor>,
    #[serde(rename = "bufferViews", default)]
    buffer_views: Vec<BufferView>,
    #[serde(default)]
    buffers: Vec<Buffer>,
}

#[derive(Deserialize)]
struct Scene {
    #[serde(default)]
    nodes: Vec<usize>,
}

#[derive(Deserialize)]
struct Node {
    #[serde(default)]
    mesh: Option<usize>,
    #[serde(default)]
    children: Vec<usize>,
    #[serde(default)]
    matrix: Option<[f64; 16]>,
    #[serde(default)]
    translation: Option<[f64; 3]>,
    #[serde(default)]
    rotation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<[f64; 3]>,
}

#[derive(Deserialize)]
struct MeshDef {
    #[serde(default)]
    primitives: Vec<Primitive>,
}

#[derive(Deserialize)]
struct Primitive {
    attributes: Attributes,
    #[serde(default)]
    indices: Option<usize>,
    #[serde(default = "default_mode")]
    mode: u32,
}

fn default_mode() -> u32 {
    MODE_TRIANGLES
}

#[derive(Deserialize)]
struct Attributes {
    #[serde(rename = "POSITION", default)]
    position: Option<usize>,
}

#[derive(Deserialize)]
struct Accessor {
    #[serde(rename = "bufferView", default)]
    buffer_view: Option<usize>,
    #[serde(rename = "byteOffset", default)]
    byte_offset: usize,
    #[serde(rename = "componentType")]
    component_type: u32,
    count: usize,
}

impl Accessor {
    fn stride(&self, views: &[BufferView]) -> Option<usize> {
        self.buffer_view
            .and_then(|index| views.get(index))
            .and_then(|view| view.byte_stride)
    }
}

#[derive(Deserialize)]
struct BufferView {
    buffer: usize,
    #[serde(rename = "byteOffset", default)]
    byte_offset: usize,
    #[serde(rename = "byteStride", default)]
    byte_stride: Option<usize>,
}

#[derive(Deserialize)]
struct Buffer {
    #[serde(default)]
    uri: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle_positions() -> Vec<f32> {
        vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0,
        ]
    }

    fn f32_bytes(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// A standalone base64 encoder so the JSON test can build a `data:` URI.
    fn encode_base64(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [
                chunk[0],
                *chunk.get(1).unwrap_or(&0),
                *chunk.get(2).unwrap_or(&0),
            ];
            let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
            out.push(ALPHABET[(n >> 18) as usize & 63] as char);
            out.push(ALPHABET[(n >> 12) as usize & 63] as char);
            out.push(if chunk.len() > 1 {
                ALPHABET[(n >> 6) as usize & 63] as char
            } else {
                '='
            });
            out.push(if chunk.len() > 2 {
                ALPHABET[n as usize & 63] as char
            } else {
                '='
            });
        }
        out
    }

    #[test]
    fn decodes_base64_vectors() {
        assert_eq!(decode_base64("TWFu"), Some(b"Man".to_vec()));
        assert_eq!(decode_base64("TQ=="), Some(b"M".to_vec()));
        assert_eq!(decode_base64("TWE="), Some(b"Ma".to_vec()));
    }

    #[test]
    fn parses_glb_with_bin_chunk() {
        let positions = f32_bytes(&triangle_positions());
        // u16 little-endian indices [0, 1, 2].
        let indices: Vec<u8> = vec![0, 0, 1, 0, 2, 0];
        let mut bin = positions.clone();
        bin.extend_from_slice(&indices);

        let json = format!(
            r#"{{"scene":0,"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0}}],
               "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}},"indices":1}}]}}],
               "accessors":[
                 {{"bufferView":0,"componentType":5126,"count":3}},
                 {{"bufferView":1,"componentType":5123,"count":3}}
               ],
               "bufferViews":[
                 {{"buffer":0,"byteOffset":0,"byteLength":{}}},
                 {{"buffer":0,"byteOffset":{},"byteLength":6}}
               ],
               "buffers":[{{"byteLength":{}}}]}}"#,
            positions.len(),
            positions.len(),
            bin.len()
        );

        let glb = build_glb(&json, &bin);
        let mesh = parse_gltf(&glb, &ObjParseOptions::default());
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2]);
    }

    #[test]
    fn parses_json_gltf_with_a_data_uri() {
        let positions = f32_bytes(&triangle_positions());
        let encoded = encode_base64(&positions);
        let json = format!(
            r#"{{"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0}}],
               "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}}}}]}}],
               "accessors":[{{"bufferView":0,"componentType":5126,"count":3}}],
               "bufferViews":[{{"buffer":0,"byteOffset":0,"byteLength":{}}}],
               "buffers":[{{"uri":"data:application/octet-stream;base64,{}","byteLength":{}}}]}}"#,
            positions.len(),
            encoded,
            positions.len()
        );

        let mesh = parse_gltf(json.as_bytes(), &ObjParseOptions::default());
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn applies_node_translation() {
        let positions = f32_bytes(&triangle_positions());
        let json = format!(
            r#"{{"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0,"translation":[5,0,0]}}],
               "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}}}}]}}],
               "accessors":[{{"bufferView":0,"componentType":5126,"count":3}}],
               "bufferViews":[{{"buffer":0,"byteOffset":0,"byteLength":{}}}],
               "buffers":[{{"uri":"data:application/octet-stream;base64,{}","byteLength":{}}}]}}"#,
            positions.len(),
            encode_base64(&positions),
            positions.len()
        );
        let mesh = parse_gltf(json.as_bytes(), &ObjParseOptions::default());
        assert!(mesh
            .vertices
            .iter()
            .all(|v| (v.x - 5.0).abs() < 1e-6 || v.x >= 5.0));
        assert!((mesh.vertices[0].x - 5.0).abs() < 1e-6);
    }

    #[test]
    fn ignores_non_triangles_and_missing_buffers() {
        let json = r#"{"nodes":[{"mesh":0}],
            "meshes":[{"primitives":[{"attributes":{"POSITION":0},"mode":1}]}],
            "accessors":[{"bufferView":0,"componentType":5126,"count":3}],
            "bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":36}],
            "buffers":[{"uri":"external.bin","byteLength":36}]}"#;
        let mesh = parse_gltf(json.as_bytes(), &ObjParseOptions::default());
        assert!(mesh.vertices.is_empty());
    }

    fn build_glb(json: &str, bin: &[u8]) -> Vec<u8> {
        let mut json_bytes = json.as_bytes().to_vec();
        while !json_bytes.len().is_multiple_of(4) {
            json_bytes.push(b' ');
        }
        let mut bin_bytes = bin.to_vec();
        while !bin_bytes.len().is_multiple_of(4) {
            bin_bytes.push(0);
        }
        let total = 12 + 8 + json_bytes.len() + 8 + bin_bytes.len();
        let mut out = Vec::with_capacity(total);
        out.extend_from_slice(&GLB_MAGIC.to_le_bytes());
        out.extend_from_slice(&2u32.to_le_bytes());
        out.extend_from_slice(&(total as u32).to_le_bytes());
        out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&CHUNK_JSON.to_le_bytes());
        out.extend_from_slice(&json_bytes);
        out.extend_from_slice(&(bin_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&CHUNK_BIN.to_le_bytes());
        out.extend_from_slice(&bin_bytes);
        out
    }
}
