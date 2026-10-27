//! Mesh post-processing (section 5.12): merging coplanar triangle pairs that
//! share an edge into quads, so a parsed triangle soup renders as one affine
//! sprite per quad instead of two per triangle.

use std::collections::HashMap;

use crate::math::Vec3;

use super::mesh::Face;
use super::parser::ParsedMesh;

/// Merges coplanar triangle pairs sharing an edge into quads.
///
/// `tolerance` is the maximum distance (world units) a triangle's apex may sit
/// from its neighbour's plane for the pair to be considered coplanar. Only
/// convex quads are produced; unmatched triangles are kept as-is.
pub fn merge_triangles_to_quads(mesh: &ParsedMesh, tolerance: f64) -> ParsedMesh {
    let mut used = vec![false; mesh.faces.len()];
    let mut edges: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for (index, face) in mesh.faces.iter().enumerate() {
        if face.indices.len() != 3 {
            continue;
        }
        for corner in 0..3 {
            let key = edge_key(face.indices[corner], face.indices[(corner + 1) % 3]);
            edges.entry(key).or_default().push(index);
        }
    }

    let mut faces = Vec::with_capacity(mesh.faces.len());
    for index in 0..mesh.faces.len() {
        if used[index] {
            continue;
        }
        let face = &mesh.faces[index];
        if face.indices.len() != 3 {
            faces.push(face.clone());
            used[index] = true;
            continue;
        }
        if let Some(quad) = try_merge(mesh, index, &edges, &used, tolerance) {
            // A merged quad keeps its material only when both triangles share it.
            let material = if mesh.faces[quad.other].material == face.material {
                face.material.clone()
            } else {
                None
            };
            faces.push(Face {
                indices: quad.quad,
                material,
            });
            used[index] = true;
            used[quad.other] = true;
        } else {
            faces.push(face.clone());
            used[index] = true;
        }
    }

    ParsedMesh {
        vertices: mesh.vertices.clone(),
        faces,
    }
}

struct Merged {
    quad: Vec<u32>,
    other: usize,
}

fn try_merge(
    mesh: &ParsedMesh,
    index: usize,
    edges: &HashMap<(u32, u32), Vec<usize>>,
    used: &[bool],
    tolerance: f64,
) -> Option<Merged> {
    let face = &mesh.faces[index];
    let [i0, i1, i2] = [face.indices[0], face.indices[1], face.indices[2]];
    for (a, b, c) in [(i0, i1, i2), (i1, i2, i0), (i2, i0, i1)] {
        let Some(candidates) = edges.get(&edge_key(a, b)) else {
            continue;
        };
        for &other_index in candidates {
            if other_index == index || used[other_index] {
                continue;
            }
            let other = &mesh.faces[other_index];
            if other.indices.len() != 3 {
                continue;
            }
            let Some(d) = other
                .indices
                .iter()
                .copied()
                .find(|vertex| *vertex != a && *vertex != b)
            else {
                continue;
            };
            let quad = vec![c, a, d, b];
            if is_coplanar(mesh, a, b, c, d, tolerance) && is_convex(mesh, &quad) {
                return Some(Merged {
                    quad,
                    other: other_index,
                });
            }
        }
    }
    None
}

fn edge_key(a: u32, b: u32) -> (u32, u32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn is_coplanar(mesh: &ParsedMesh, a: u32, b: u32, c: u32, d: u32, tolerance: f64) -> bool {
    let (Some(pa), Some(pb), Some(pc), Some(pd)) = (
        vertex(mesh, a),
        vertex(mesh, b),
        vertex(mesh, c),
        vertex(mesh, d),
    ) else {
        return false;
    };
    let cross = (pb - pa).cross(pc - pa);
    if cross.length() < 1e-12 {
        return false;
    }
    let normal = cross.normalized();
    (pd - pa).dot(normal).abs() <= tolerance
}

fn is_convex(mesh: &ParsedMesh, quad: &[u32]) -> bool {
    let points: Vec<Vec3> = quad.iter().filter_map(|v| vertex(mesh, *v)).collect();
    if points.len() != 4 {
        return false;
    }
    let cross = (points[1] - points[0]).cross(points[2] - points[0]);
    if cross.length() < 1e-12 {
        return false;
    }
    let normal = cross.normalized();
    for i in 0..4 {
        let a = points[i];
        let b = points[(i + 1) % 4];
        let c = points[(i + 2) % 4];
        if (b - a).cross(c - b).dot(normal) < -1e-9 {
            return false;
        }
    }
    true
}

fn vertex(mesh: &ParsedMesh, index: u32) -> Option<Vec3> {
    mesh.vertices.get(index as usize).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> ParsedMesh {
        ParsedMesh {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            faces: vec![Face::new(vec![0, 1, 2]), Face::new(vec![0, 2, 3])],
        }
    }

    #[test]
    fn merges_a_coplanar_triangle_pair_into_a_quad() {
        let merged = merge_triangles_to_quads(&square(), 1e-6);
        assert_eq!(merged.faces.len(), 1);
        assert_eq!(merged.faces[0].indices.len(), 4);
    }

    #[test]
    fn keeps_non_coplanar_triangles_separate() {
        let mut mesh = square();
        // Fold the second triangle up out of the plane.
        mesh.vertices[3] = Vec3::new(0.0, 1.0, 5.0);
        let merged = merge_triangles_to_quads(&mesh, 1e-6);
        assert_eq!(merged.faces.len(), 2);
    }

    #[test]
    fn leaves_non_triangle_faces_untouched() {
        let mesh = ParsedMesh {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            faces: vec![Face::new(vec![0, 1, 2, 3])],
        };
        let merged = merge_triangles_to_quads(&mesh, 1e-6);
        assert_eq!(merged.faces.len(), 1);
        assert_eq!(merged.faces[0].indices.len(), 4);
    }
}
