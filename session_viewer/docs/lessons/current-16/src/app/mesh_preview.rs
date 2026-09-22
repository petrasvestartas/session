use super::deform::{Target, mesh_keys};
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::patch::{Counts, Span};
use crate::engine::gpu::segments::{CylinderSegment, SegRows};
use crate::engine::gpu::{Gpu, Upload};
use session_rust::{Geometry, Mesh, RenderVertex, Xform};
use std::collections::{HashMap, HashSet};

/// A mesh's GPU rows, each tagged with its source vertex key.
pub struct MeshPreview {
    vertices: Vec<(usize, RenderVertex)>,       // (vertex key, GPU vertex)
    pipes: Vec<([usize; 2], CylinderSegment)>, // (edge keys, GPU pipe)
    spheres: Vec<(usize, GlyphPoint)>,         // (vertex key, vertex sphere)
    dots: Vec<(usize, GlyphPoint)>,            // (vertex key, dot)
    span: Span,                                // where the rows sit on the GPU
}

/// The GPU rows one drag touches.
pub struct Gesture {
    vertices: Vec<(u32, RenderVertex, bool)>,     // (GPU index, original, moves)
    pipes: Vec<(u32, CylinderSegment, [bool; 2])>, // (GPU index, original, each end moves)
    spheres: Vec<(u32, GlyphPoint, bool)>,        // (GPU index, original, moves)
    dots: Vec<(u32, GlyphPoint, bool)>,           // (GPU index, original, moves)
}

/// The mesh inside a geometry, if any.
pub fn mesh(geometry: &Geometry) -> Option<&Mesh> {
    match geometry {
        Geometry::Mesh(mesh) => Some(mesh),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => Some(mesh),
            _ => None,
        },
        _ => None,
    }
}

impl MeshPreview {
    /// Match the uploaded rows back to the mesh's vertex keys.
    pub(crate) fn capture(
        up: &Upload,
        span: Span,
        local: Counts,
        geometry: &Geometry,
    ) -> Option<Self> {
        let mesh = mesh(geometry)?;

        if span.count.ribbons != 0 {
            return None; // not a plain mesh upload
        }

        let mut keys = mesh.vertices(); // one GPU vertex per key

        // face colours split every triangle: keys per triangle corner
        if mesh.color_mode == session_rust::mesh::ColorMode::FACECOLORS
            && mesh.get_facecolors().len() == mesh.face.len()
        {
            keys.clear();

            for face in mesh.faces() {
                let ring = &mesh.face[&face];
                let triangles = mesh
                    .triangulation
                    .get(&face)
                    .filter(|t| !t.is_empty())
                    .cloned()
                    .unwrap_or_else(|| {
                        (1..ring.len().saturating_sub(1))
                            .map(|i| [ring[0], ring[i], ring[i + 1]])
                            .collect()
                    });

                for triangle in triangles {
                    if triangle.iter().all(|key| mesh.vertex.contains_key(key)) {
                        keys.extend(triangle);
                    }
                }
            }
        }

        if keys.len() != span.count.verts as usize {
            return None; // upload does not match
        }

        let vertices = keys
            .into_iter()
            .zip(
                up.arena.verts[local.verts as usize..(local.verts + span.count.verts) as usize]
                    .iter()
                    .copied(),
            )
            .collect();
        // edges in the order the walk numbered them
        let mut seen = HashSet::new();
        let mut edges = Vec::new();

        for face in mesh.faces() {
            let ring = &mesh.face[&face];

            for i in 0..ring.len() {
                let pair = [
                    ring[i].min(ring[(i + 1) % ring.len()]),
                    ring[i].max(ring[(i + 1) % ring.len()]),
                ];

                if seen.insert(pair) {
                    edges.push(pair);
                }
            }
        }

        let pipes = (local.pipes..local.pipes + span.count.pipes)
            .map(|i| {
                Some((
                    *edges.get(*up.seg.pipe_ids.get(i as usize)? as usize)?,
                    up.seg.pipes[i as usize],
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        // markers are matched by position; a shared position gives up
        let mut positions = HashMap::new();

        for (&key, v) in &mesh.vertex {
            let p = [v.x as f32, v.y as f32, v.z as f32].map(f32::to_bits);
            positions
                .entry(p)
                .and_modify(|value| *value = None)
                .or_insert(Some(key));
        }

        let markers = |items: &[GlyphPoint]| {
            items
                .iter()
                .map(|g| Some(((*positions.get(&g.center.map(f32::to_bits))?)?, *g)))
                .collect::<Option<Vec<_>>>()
        };
        Some(Self {
            vertices,
            pipes,
            spheres: markers(
                &up.glyph.spheres
                    [local.spheres as usize..(local.spheres + span.count.spheres) as usize],
            )?,
            dots: markers(
                &up.glyph.dots[local.dots as usize..(local.dots + span.count.dots) as usize],
            )?,
            span,
        })
    }

    /// The rows a drag of `target` touches.
    pub fn begin(&self, geometry: &Geometry, target: Target) -> Option<Gesture> {
        let mesh = mesh(geometry)?;
        let selected: HashSet<_> = mesh_keys(mesh, target).ok()?.into_iter().collect(); // keys that move
        let mut affected = selected.clone(); // keys whose faces change

        for ring in mesh.face.values() {
            if ring.iter().any(|k| selected.contains(k)) {
                affected.extend(ring);
            }
        }

        let vertices = self
            .vertices
            .iter()
            .enumerate()
            .filter(|(_, (k, _))| affected.contains(k))
            .map(|(i, (k, v))| (self.span.start.verts + i as u32, *v, selected.contains(k)))
            .collect();
        let pipes = self
            .pipes
            .iter()
            .enumerate()
            .filter(|(_, (keys, _))| keys.iter().any(|k| affected.contains(k)))
            .map(|(i, (keys, p))| {
                (
                    self.span.start.pipes + i as u32,
                    *p,
                    keys.map(|k| selected.contains(&k)),
                )
            })
            .collect();
        let markers = |items: &[(usize, GlyphPoint)], start| {
            items
                .iter()
                .enumerate()
                .filter(|(_, (k, _))| affected.contains(k))
                .map(|(i, (k, g))| (start + i as u32, *g, selected.contains(k)))
                .collect()
        };
        Some(Gesture {
            vertices,
            pipes,
            spheres: markers(&self.spheres, self.span.start.spheres),
            dots: markers(&self.dots, self.span.start.dots),
        })
    }

    /// Memory held by the preview.
    pub fn allocated_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<(usize, RenderVertex)>()
            + self.pipes.capacity() * std::mem::size_of::<([usize; 2], CylinderSegment)>()
            + (self.spheres.capacity() + self.dots.capacity())
                * std::mem::size_of::<(usize, GlyphPoint)>()
    }
}

impl Gesture {
    /// Patch the GPU rows with `delta` applied, or put them back.
    pub fn apply(&self, gpu: &mut Gpu, delta: &Xform, restore: bool) {
        // transform a point by the 4x4 matrix
        let position = |p: [f32; 3]| {
            let m = &delta.m;
            std::array::from_fn(|r| {
                (m[r] * p[0] as f64 + m[4 + r] * p[1] as f64 + m[8 + r] * p[2] as f64 + m[12 + r])
                    as f32
            })
        };

        for &(index, mut v, moved) in &self.vertices {
            if !restore {
                if moved {
                    v.position = position(v.position);
                }

                v.normal = [0.; 3]; // flat shading while dragging
            }

            gpu.arena.patch_vertices(&gpu.ctx, index, &[v]);
        }

        for &(index, mut p, moved) in &self.pipes {
            if !restore {
                if moved[0] {
                    p.p0 = position(p.p0);
                }

                if moved[1] {
                    p.p1 = position(p.p1);
                }

                p.facing = u32::MAX; // always draw while dragging
            }

            gpu.segments.patch_pipes(
                &gpu.ctx,
                index,
                &SegRows {
                    pipes: vec![p],
                    ..Default::default()
                },
            );
        }

        for (items, sphere) in [(&self.spheres, true), (&self.dots, false)] {
            for &(index, mut g, moved) in items {
                if !restore {
                    if moved {
                        g.center = position(g.center);
                    }

                    g.facing = u32::MAX;
                    g.facing_ext = [u32::MAX; 2];
                }

                gpu.glyphs.patch_marker(&gpu.ctx, index, sphere, g);
            }
        }

        gpu.objects.geometry_changed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::{Walk, WalkCx, walk_geometry};
    use session_rust::Point;
    use std::rc::Rc;

    /// A drag touches only the rows around the target.
    #[test]
    fn large_mesh_gesture_only_patches_local_neighborhood() {
        let mut mesh = Mesh::new();

        for y in 0..101 {
            for x in 0..101 {
                mesh.add_vertex(Point::new(x as f64, y as f64, 0.), Some(y * 101 + x));
            }
        }

        for y in 0..100 {
            for x in 0..100 {
                let a = y * 101 + x;
                mesh.add_face(vec![a, a + 1, a + 102], None);
                mesh.add_face(vec![a, a + 102, a + 101], None);
            }
        }

        let geometry = Geometry::Mesh(Rc::new(mesh));
        let mut up = Upload::default();
        walk_geometry(
            &mut Walk::of(&mut up),
            &WalkCx {
                vert_base: 0,
                cloud_px: 0.,
                row: 0,
                // --8<-- [start:step-11]
                attributes: false,
                // --8<-- [end:step-11]
            },
            &geometry,
        );
        let span = Span {
            start: Counts::default(),
            count: Counts::of(&up),
        };
        let preview = MeshPreview::capture(&up, span, Counts::default(), &geometry).unwrap();
        let gesture = preview.begin(&geometry, Target::Face(10000)).unwrap();
        assert!(
            gesture.vertices.len() < 30,
            "drag cost follows adjacency, not the 10,201-vertex mesh"
        );
        assert_eq!(gesture.vertices.iter().filter(|v| v.2).count(), 3);
        assert!(gesture.pipes.len() < 100);
        assert_eq!(
            super::mesh(&geometry).unwrap().vertex[&0].z,
            0.,
            "preview does not mutate source"
        );
    }
}
