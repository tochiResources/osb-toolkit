//! Built-in 3D object parser (section 5.12): parses common mesh/scene formats
//! into the `storyboarding3d` mesh model, with fine-grained options.
//!
//! Supports Wavefront OBJ (`v`/`f` and `/`-separated index triples) and STL
//! (ASCII and binary). The scene-JSON shape is the plain [`super::input::Bake3dInput`]
//! consumed by [`super::input::build_scene`]; glTF/GLB is a follow-up.

use std::collections::{BTreeMap, HashMap};

use crate::math::{Vec2, Vec3};

use super::mesh::{Face, MeshData};

/// Which world axis is "up" in the source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpAxis {
    /// Y-up (glTF, most engines). No conversion.
    #[default]
    Y,
    /// Z-up (Blender default). Swapped to Y-up.
    Z,
}

/// Fine-grained options for parsing and materializing a mesh.
#[derive(Debug, Clone)]
pub struct ObjParseOptions {
    /// Uniform scale applied to every vertex.
    pub scale: f64,
    /// Source up-axis; the parser converts to the engine's Y-up.
    pub up_axis: UpAxis,
    /// The sprite path each face will render with.
    pub sprite_path: String,
    /// The sprite bitmap size (for scale conversion).
    pub bitmap_size: Vec2,
    /// Material name -> sprite path, carried onto the resulting [`MeshData`].
    pub materials: BTreeMap<String, String>,
}

impl Default for ObjParseOptions {
    fn default() -> Self {
        ObjParseOptions {
            scale: 1.0,
            up_axis: UpAxis::Y,
            sprite_path: "sb/triangle.png".into(),
            bitmap_size: Vec2::new(64.0, 64.0),
            materials: BTreeMap::new(),
        }
    }
}

/// A parsed mesh (before it becomes a [`MeshData`]).
#[derive(Debug, Clone, Default)]
pub struct ParsedMesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl ParsedMesh {
    /// Materializes the mesh into the projection model.
    pub fn into_mesh(self, options: &ObjParseOptions) -> MeshData {
        MeshData {
            vertices: self.vertices,
            faces: self.faces,
            bitmap_size: options.bitmap_size,
            materials: options.materials.clone(),
        }
    }
}

/// Parses Wavefront OBJ text into vertices and faces.
///
/// Handles `v x y z` (optionally `w`) and `f` with `a`, `a/b`, `a//b`, `a/b/c`
/// tokens; negative (relative) indices are supported.
pub fn parse_obj(text: &str, options: &ObjParseOptions) -> ParsedMesh {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut material: Option<String> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x = parts.next().and_then(parse_f64).unwrap_or(0.0);
                let y = parts.next().and_then(parse_f64).unwrap_or(0.0);
                let z = parts.next().and_then(parse_f64).unwrap_or(0.0);
                vertices.push(transform_vertex(x, y, z, options));
            }
            Some("usemtl") => {
                material = parts.next().map(str::to_string);
            }
            Some("f") => {
                let mut indices = Vec::new();
                for token in parts {
                    let raw = token.split('/').next().unwrap_or("");
                    if let Ok(index) = raw.parse::<i64>() {
                        indices.push(resolve_index(index, vertices.len()));
                    }
                }
                if indices.len() >= 3 {
                    faces.push(Face {
                        indices,
                        material: material.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    ParsedMesh { vertices, faces }
}

fn parse_f64(token: &str) -> Option<f64> {
    token.parse::<f64>().ok()
}

/// Applies the up-axis conversion and uniform scale to a source vertex.
fn transform_vertex(x: f64, y: f64, z: f64, options: &ObjParseOptions) -> Vec3 {
    let v = match options.up_axis {
        UpAxis::Y => Vec3::new(x, y, z),
        UpAxis::Z => Vec3::new(x, z, -y),
    };
    v * options.scale
}

/// Resolves a 1-based (or negative relative) OBJ index to a 0-based index.
fn resolve_index(index: i64, count: usize) -> u32 {
    if index > 0 {
        (index - 1) as u32
    } else {
        (count as i64 + index).max(0) as u32
    }
}

/// Parses an STL file, detecting binary vs ASCII from its layout, into vertices
/// and triangle faces. Duplicate corner positions are welded.
pub fn parse_stl(bytes: &[u8], options: &ObjParseOptions) -> ParsedMesh {
    if is_binary_stl(bytes) {
        parse_binary_stl(bytes, options)
    } else {
        parse_ascii_stl(&String::from_utf8_lossy(bytes), options)
    }
}

/// Binary STL: an 80-byte header, a `u32` triangle count, then 50 bytes per
/// triangle (12 `f32` + a `u16` attribute count).
fn is_binary_stl(bytes: &[u8]) -> bool {
    if bytes.len() < 84 {
        return false;
    }
    let count = u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]) as usize;
    84 + count * 50 == bytes.len()
}

fn parse_binary_stl(bytes: &[u8], options: &ObjParseOptions) -> ParsedMesh {
    let count = u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]) as usize;
    let mut mesh = WeldedMesh::new(options);
    for triangle in 0..count {
        let base = 84 + triangle * 50 + 12;
        let mut indices = [0u32; 3];
        for (corner, index) in indices.iter_mut().enumerate() {
            let offset = base + corner * 12;
            let read = |i: usize| {
                f32::from_le_bytes([
                    bytes[offset + i * 4],
                    bytes[offset + i * 4 + 1],
                    bytes[offset + i * 4 + 2],
                    bytes[offset + i * 4 + 3],
                ]) as f64
            };
            *index = mesh.vertex(read(0), read(1), read(2));
        }
        mesh.faces.push(Face::new(indices.to_vec()));
    }
    mesh.finish()
}

/// ASCII STL: `solid`/`facet`/`outer loop`/`vertex x y z`/`endloop`/`endfacet`.
fn parse_ascii_stl(text: &str, options: &ObjParseOptions) -> ParsedMesh {
    let mut mesh = WeldedMesh::new(options);
    let mut pending: Vec<u32> = Vec::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        if parts.next() != Some("vertex") {
            continue;
        }
        let x = parts.next().and_then(parse_f64).unwrap_or(0.0);
        let y = parts.next().and_then(parse_f64).unwrap_or(0.0);
        let z = parts.next().and_then(parse_f64).unwrap_or(0.0);
        pending.push(mesh.vertex(x, y, z));
        if pending.len() == 3 {
            mesh.faces.push(Face::new(std::mem::take(&mut pending)));
        }
    }
    mesh.finish()
}

/// Accumulates welded vertices and faces during an STL parse.
pub(crate) struct WeldedMesh<'a> {
    options: &'a ObjParseOptions,
    vertices: Vec<Vec3>,
    lookup: HashMap<(i64, i64, i64), u32>,
    faces: Vec<Face>,
}

impl<'a> WeldedMesh<'a> {
    pub(crate) fn new(options: &'a ObjParseOptions) -> Self {
        WeldedMesh {
            options,
            vertices: Vec::new(),
            lookup: HashMap::new(),
            faces: Vec::new(),
        }
    }

    pub(crate) fn vertex(&mut self, x: f64, y: f64, z: f64) -> u32 {
        let v = transform_vertex(x, y, z, self.options);
        let key = (
            (v.x * 1e6).round() as i64,
            (v.y * 1e6).round() as i64,
            (v.z * 1e6).round() as i64,
        );
        if let Some(&index) = self.lookup.get(&key) {
            return index;
        }
        let index = self.vertices.len() as u32;
        self.vertices.push(v);
        self.lookup.insert(key, index);
        index
    }

    pub(crate) fn add_face(&mut self, indices: Vec<u32>) {
        if indices.len() >= 3 {
            self.faces.push(Face::new(indices));
        }
    }

    pub(crate) fn finish(self) -> ParsedMesh {
        ParsedMesh {
            vertices: self.vertices,
            faces: self.faces,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIANGLE: &str = "# a triangle\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";

    #[test]
    fn parses_vertices_and_faces() {
        let mesh = parse_obj(TRIANGLE, &ObjParseOptions::default());
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2]);
    }

    #[test]
    fn parses_slash_index_triples() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nf 1/1 2/1 3/1\n";
        let mesh = parse_obj(obj, &ObjParseOptions::default());
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2]);
    }

    #[test]
    fn parses_quads_and_relative_indices() {
        let obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf -4 -3 -2 -1\n";
        let mesh = parse_obj(obj, &ObjParseOptions::default());
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2, 3]);
    }

    #[test]
    fn applies_scale_and_up_axis() {
        let options = ObjParseOptions {
            scale: 2.0,
            up_axis: UpAxis::Z,
            ..ObjParseOptions::default()
        };
        let mesh = parse_obj("v 1 2 3\n", &options);
        // Z-up (x, y, z) -> Y-up (x, z, -y), scaled by 2.
        assert_eq!(mesh.vertices[0], Vec3::new(2.0, 6.0, -4.0));
    }

    #[test]
    fn materializes_into_mesh3d() {
        let mesh = parse_obj(TRIANGLE, &ObjParseOptions::default()).into_mesh(&ObjParseOptions {
            bitmap_size: Vec2::new(32.0, 32.0),
            ..ObjParseOptions::default()
        });
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.bitmap_size, Vec2::new(32.0, 32.0));
    }

    #[test]
    fn parses_usemtl_and_carries_the_material_map() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nusemtl skin\nf 1 2 3\n";
        let mut options = ObjParseOptions::default();
        options
            .materials
            .insert("skin".into(), "sb/skin.png".into());
        let parsed = parse_obj(obj, &options);
        assert_eq!(parsed.faces[0].material.as_deref(), Some("skin"));
        let mesh = parsed.into_mesh(&options);
        assert_eq!(
            mesh.materials.get("skin").map(String::as_str),
            Some("sb/skin.png")
        );
    }

    const ASCII_STL: &str = "solid t\nfacet normal 0 0 1\nouter loop\n\
vertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid t\n";

    #[test]
    fn parses_ascii_stl_and_welds_vertices() {
        let mesh = parse_stl(ASCII_STL.as_bytes(), &ObjParseOptions::default());
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2]);
    }

    #[test]
    fn parses_binary_stl_and_detects_the_format() {
        let mut bytes = vec![0u8; 80];
        bytes[0] = b's';
        bytes.extend_from_slice(&1u32.to_le_bytes());
        let triangle: [f32; 12] = [
            0.0, 0.0, 1.0, // normal
            0.0, 0.0, 0.0, // v0
            1.0, 0.0, 0.0, // v1
            0.0, 1.0, 0.0, // v2
        ];
        for value in triangle {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&0u16.to_le_bytes());

        assert!(is_binary_stl(&bytes));
        let mesh = parse_stl(&bytes, &ObjParseOptions::default());
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, vec![0, 1, 2]);
    }

    #[test]
    fn stl_respects_scale_and_up_axis() {
        let options = ObjParseOptions {
            scale: 2.0,
            up_axis: UpAxis::Z,
            ..ObjParseOptions::default()
        };
        let stl = "solid t\nfacet normal 0 0 1\nouter loop\n\
vertex 1 2 3\nvertex 1 2 3\nvertex 1 2 3\nendloop\nendfacet\nendsolid t\n";
        let mesh = parse_stl(stl.as_bytes(), &options);
        // One welded vertex: Z-up (1,2,3) -> Y-up (1,3,-2), scaled by 2.
        assert_eq!(mesh.vertices.len(), 1);
        assert_eq!(mesh.vertices[0], Vec3::new(2.0, 6.0, -4.0));
    }
}
