//! Exact face identities for shaded ink. A triangle's first vertex supplies its token;
//! the other corners keep their original shared positions, colours and smooth normals.

use std::collections::HashMap;
use session_rust::{Mesh, RenderMesh};
use crate::engine::gpu::arena::FacePlane;
use super::hosts::HostFace;
use super::mesh::COPLANAR_DOT;
use super::mesh_topology::MeshTopo;

/// A render mesh with a face token at each vertex and each logical kernel face.
pub struct FaceMesh {
    pub render: RenderMesh,
    pub ids: Vec<u32>,
    pub tokens: Vec<Vec<FaceSupport>>,
    pub hosts: Vec<HostFace>,
    pub planes: Vec<FacePlane>,
}

/// A whole planar polygon or one physical triangle of a warped polygon.
#[derive(Clone)]
pub struct FaceSupport {
    pub token: u32,
    pub vertices: Option<[usize; 3]>,
    pub normal: [f64; 3],
}

impl FaceSupport {
    /// Only actual triangle incidence grants support when a polygon is not planar.
    pub fn contains(&self, vertices: &[usize]) -> bool {
        self.vertices.is_none_or(|triangle| vertices.iter().all(|vertex| triangle.contains(vertex)))
    }
}

/// Global physical-face table base and the mesh's instance row.
pub struct FaceCx {
    pub base: u32,
    pub row: u32,
}

/// Reuse original vertices wherever their first-corner token is available. Copies keep
/// all attributes and can be reused by later triangles of the same planar region.
struct FirstVertices {
    render: RenderMesh,
    ids: Vec<u32>,
    copies: HashMap<(u32, u32), u32>,
}

impl FirstVertices {
    /// Preserve the original vertex rows; rebuild only triangle index order.
    fn new(render: &RenderMesh) -> Self {
        Self {
            render: RenderMesh { vertices: render.vertices.clone(), indices: Vec::with_capacity(render.indices.len()) },
            ids: vec![0; render.vertices.len()],
            copies: HashMap::new(),
        }
    }

    /// Rotate cyclically without changing winding or interpolated attributes. A shared
    /// corner need not carry this face's token unless it is the provoking first corner.
    fn push(&mut self, mut triangle: [u32; 3], token: u32) {
        let matching = triangle.iter().position(|index| self.ids[*index as usize] == token);
        let available = matching.or_else(|| triangle.iter().position(|index| self.ids[*index as usize] == 0));
        if let Some(corner) = available {
            triangle.rotate_left(corner);
            self.ids[triangle[0] as usize] = token;
        } else {
            let original = triangle[0];
            let copy = if let Some(index) = self.copies.get(&(original, token)) {
                *index
            } else {
                let index = u32::try_from(self.ids.len()).expect("face vertex index overflow");
                self.render.vertices.push(self.render.vertices[original as usize]);
                self.ids.push(token);
                self.copies.insert((original, token), index);
                index
            };
            triangle[0] = copy;
        }
        self.render.indices.extend(triangle);
    }
}

/// The exact triangle sequence used by `Mesh::to_render`, including cached triangulation.
fn face_triangles(mesh: &Mesh, face: usize) -> Vec<[usize; 3]> {
    let mut triangles = Vec::new();
    if let Some(cached) = mesh.triangulation.get(&face) && !cached.is_empty() {
        triangles.extend_from_slice(cached);
    } else if let Some(vertices) = mesh.face_vertices(face) {
        for i in 2..vertices.len() {
            triangles.push([vertices[0], vertices[i - 1], vertices[i]]);
        }
    }
    triangles.retain(|triangle| triangle.iter().all(|key| mesh.vertex.contains_key(key)));
    triangles
}

/// Find and compress one connected planar-region representative.
fn root(parents: &mut [usize], mut face: usize) -> usize {
    while parents[face] != face {
        parents[face] = parents[parents[face]];
        face = parents[face];
    }
    face
}

/// Merge only adjacent coplanar faces, using the same test that suppresses their shared ink.
fn regions(mesh: &Mesh, topo: &MeshTopo, planar: &[bool]) -> Vec<usize> {
    let mut parents: Vec<usize> = (0..topo.normals.len()).collect();
    let keys = mesh.faces();
    for pair in &topo.edge_faces {
        if pair[1] == u32::MAX { continue; }
        let (a, b) = (pair[0] as usize, pair[1] as usize);
        if !planar[a] || !planar[b] { continue; }
        let (Some(na), Some(nb)) = (topo.normals[a], topo.normals[b]) else { continue };
        let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
        if dot >= COPLANAR_DOT {
            let ra = root(&mut parents, a);
            let rb = root(&mut parents, b);
            if coplanar(mesh, keys[ra], keys[rb], topo.normals[ra].unwrap()) {
                parents[rb] = ra;
            }
        }
    }
    for face in 0..parents.len() { parents[face] = root(&mut parents, face); }
    parents
}

/// A normal threshold alone could merge a gradually curved surface into one false plane.
fn coplanar(mesh: &Mesh, first: usize, second: usize, normal: [f64; 3]) -> bool {
    let a = &mesh.vertex[&mesh.face[&first][0]];
    for key in &mesh.face[&second] {
        let b = &mesh.vertex[key];
        let distance = (b.x - a.x) * normal[0] + (b.y - a.y) * normal[1] + (b.z - a.z) * normal[2];
        let scale = a.x.abs().max(a.y.abs()).max(a.z.abs()).max(b.x.abs()).max(b.y.abs()).max(b.z.abs()).max(1.0);
        if distance.abs() > scale * 128.0 * f64::EPSILON { return false; }
    }
    true
}

/// Unit normal from the actual rendered triangle, including warped cached triangulations.
fn triangle_normal(points: &[[f64; 3]; 3]) -> [f64; 3] {
    let u = std::array::from_fn::<_, 3, _>(|i| points[1][i] - points[0][i]);
    let v = std::array::from_fn::<_, 3, _>(|i| points[2][i] - points[0][i]);
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if length == 0.0 { [0.0; 3] } else { n.map(|value| value / length) }
}

/// Add one exact physical plane and return its globally one-based identity.
fn add_plane(out: &mut FaceMesh, point: [f64; 3], normal: [f64; 3], cx: &FaceCx) -> u32 {
    let token = cx.base + out.planes.len() as u32 + 1;
    out.planes.push(FacePlane { point: point.map(|v| v as f32), instance_id: cx.row, normal: normal.map(|v| v as f32), _pad: 0 });
    token
}

/// Attach exact tokens while preserving the original triangle sequence and vertex values.
pub fn decorate(mesh: &Mesh, render: &RenderMesh, topo: &MeshTopo, cx: &FaceCx) -> FaceMesh {
    let faces = mesh.faces();
    let planar: Vec<bool> = faces.iter().enumerate().map(|(slot, face)| topo.normals[slot].is_some_and(|normal| coplanar(mesh, *face, *face, normal))).collect();
    let regions = regions(mesh, topo, &planar);
    let mut out = FaceMesh { render: RenderMesh::default(), ids: Vec::new(), tokens: vec![Vec::new(); regions.len()], hosts: Vec::new(), planes: Vec::new() };
    let mut vertices = FirstVertices::new(render);
    let mut region_tokens = vec![0u32; regions.len()];
    let mut cursor = 0usize;
    for (slot, face) in faces.iter().copied().enumerate() {
        let triangles = face_triangles(mesh, face);
        let region = regions[slot];
        let mut world = Vec::with_capacity(triangles.len());
        for triangle in triangles {
            let points = triangle.map(|key| { let point = &mesh.vertex[&key]; [point.x, point.y, point.z] });
            let normal = if planar[slot] { topo.normals[region].unwrap() } else { triangle_normal(&points) };
            let token = if planar[slot] {
                if region_tokens[region] == 0 { region_tokens[region] = add_plane(&mut out, points[0], normal, cx); }
                region_tokens[region]
            } else {
                add_plane(&mut out, points[0], normal, cx)
            };
            vertices.push(render.indices[cursor..cursor + 3].try_into().unwrap(), token);
            cursor += 3;
            if planar[slot] {
                if out.tokens[slot].is_empty() { out.tokens[slot].push(FaceSupport { token, vertices: None, normal }); }
                world.push(points);
            } else {
                out.tokens[slot].push(FaceSupport { token, vertices: Some(triangle), normal });
                out.hosts.push(HostFace { face: token, triangles: vec![points] });
            }
        }
        if !world.is_empty() { out.hosts.push(HostFace { face: region_tokens[region], triangles: world }); }
    }
    assert_eq!(cursor, render.indices.len(), "face token sequence disagrees with Mesh::to_render");
    out.render = vertices.render;
    out.ids = vertices.ids;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Color, Point};
    use super::super::mesh_topology::{SlotMap, mesh_topology};

    /// Build the same topology used by the live walk.
    fn topology(mesh: &Mesh) -> MeshTopo {
        let keys = mesh.vertices();
        let slots = SlotMap::new(&keys);
        let positions: Vec<_> = keys.iter().map(|key| {
            let p = &mesh.vertex[key];
            [p.x, p.y, p.z]
        }).collect();
        mesh_topology(mesh, &keys, &positions, &slots)
    }

    /// Tokens never change emitted geometry, including cached and face-coloured triangles.
    #[test]
    fn tokens_preserve_cached_colored_triangles() {
        let mut mesh = Mesh::create_box(20.0, 30.0, 40.0);
        let keys = mesh.faces();
        let vertices = mesh.face[&keys[0]].clone();
        mesh.triangulation.insert(keys[0], vec![[vertices[1], vertices[2], vertices[3]], [vertices[1], vertices[3], vertices[0]]]);
        for colored in [false, true] {
            if colored {
                mesh.set_facecolors(vec![Color::red(); keys.len()]);
                mesh.color_mode = session_rust::mesh::ColorMode::FACECOLORS;
            }
            let original = mesh.to_render();
            let result = decorate(&mesh, &original, &topology(&mesh), &FaceCx { base: 123, row: 5 });
            assert_eq!(result.tokens.len(), 6);
            assert_eq!(result.planes.len(), 6);
            assert_eq!(result.ids.len(), result.render.vertices.len());
            for (old, new) in original.indices.chunks_exact(3).zip(result.render.indices.chunks_exact(3)) {
                let first = bytemuck::bytes_of(&result.render.vertices[new[0] as usize]);
                let rotation = old.iter().position(|index| bytemuck::bytes_of(&original.vertices[*index as usize]) == first).expect("first vertex retains all original attributes");
                for corner in 0..3 {
                    assert_eq!(bytemuck::bytes_of(&original.vertices[old[(corner + rotation) % 3] as usize]), bytemuck::bytes_of(&result.render.vertices[new[corner] as usize]));
                }
                assert!(result.ids[new[0] as usize] >= 124);
            }
            for triangle in result.render.indices.chunks_exact(3) {
                let token = result.ids[triangle[0] as usize];
                let plane = &result.planes[(token - 124) as usize];
                assert_eq!(plane.instance_id, 5);
                for vertex in triangle {
                    let point = result.render.vertices[*vertex as usize].position;
                    let distance: f32 = (0..3).map(|axis| (point[axis] - plane.point[axis]) * plane.normal[axis]).sum();
                    assert!(distance.abs() < 1e-6, "physical face token points to a different plane");
                }
            }
            let unique: std::collections::HashSet<_> = result.tokens.iter().map(|parts| parts[0].token).collect();
            assert_eq!(unique.len(), 6, "opposite cube faces must never share support");
        }
    }

    /// Coplanar tessellation shares a token, while a small physical bend remains distinct.
    #[test]
    fn planar_regions_do_not_merge_bent_faces() {
        for rise in [0.0, 1e-5] {
            let mut mesh = Mesh::new();
            for point in [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [10.0, 10.0, rise]] {
                mesh.add_vertex(Point::new(point[0], point[1], point[2]), None);
            }
            mesh.add_face(vec![0, 1, 2], None);
            mesh.add_face(vec![1, 3, 2], None);
            let original = mesh.to_render();
            let result = decorate(&mesh, &original, &topology(&mesh), &FaceCx { base: 0, row: 0 });
            assert_eq!(result.tokens[0][0].token == result.tokens[1][0].token, rise == 0.0);
            assert_eq!(result.render.vertices.len(), 4, "these triangles have available first corners even when their planes differ");
        }
    }

    /// A warped polygon never gives an edge permission to draw over its other triangle.
    #[test]
    fn warped_polygon_support_tracks_actual_triangles() {
        let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0), Point::new(10.0, 10.0, 0.0), Point::new(0.0, 10.0, 5.0)];
        let mesh = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        let original = mesh.to_render();
        let result = decorate(&mesh, &original, &topology(&mesh), &FaceCx { base: 40, row: 3 });
        assert_eq!(result.tokens[0].len(), 2);
        let parts = &result.tokens[0];
        assert_ne!(parts[0].token, parts[1].token);
        assert!(parts[0].contains(&[0, 1]));
        assert!(!parts[1].contains(&[0, 1]));
        assert!(parts[1].contains(&[2, 3]));
        assert!(!parts[0].contains(&[2, 3]));
        for triangle in result.render.indices.chunks_exact(3) {
            let token = result.ids[triangle[0] as usize];
            let plane = &result.planes[(token - 41) as usize];
            for vertex in triangle {
                let point = result.render.vertices[*vertex as usize].position;
                let distance: f32 = (0..3).map(|axis| (point[axis] - plane.point[axis]) * plane.normal[axis]).sum();
                assert!(distance.abs() < 1e-6);
            }
        }
    }
}
