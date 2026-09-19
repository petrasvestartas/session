use crate::engine::gpu::patch::Span;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, Upload};
use session_rust::AABB;
use session_rust::{Geometry, NurbsSurface, RenderVertex};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Sample {
    pub index: u32,
    pub surface: u32,
    pub uv: [f64; 2],
    pub sign: f32,
}

pub struct SurfacePreview {
    samples: Vec<Sample>,
    vertices: Vec<RenderVertex>,
    controls: Vec<Vec<f64>>,
    pipes: Vec<CylinderSegment>,
    pipe_vertices: Vec<[usize; 2]>,
    pipe_normals: Vec<[usize; 2]>,
    chains: Vec<std::ops::Range<u32>>,
}

impl SurfacePreview {
    pub(crate) fn capture(
        up: &Upload,
        span: Span,
        local: crate::engine::gpu::patch::Counts,
        geometry: &Geometry,
    ) -> Option<Self> {
        if span.count.verts == 0
            || span.count.ribbons != 0
            || span.count.spheres != 0
            || span.count.dots != 0
        {
            return None;
        }

        let first = local.verts as usize;
        let end = first + span.count.verts as usize;
        let samples: Vec<_> = up
            .arena
            .surface_samples
            .iter()
            .filter(|sample| (first..end).contains(&(sample.index as usize)))
            .copied()
            .collect();

        if samples.len() != span.count.verts as usize {
            return None;
        }

        let mut positions = HashMap::<[u32; 3], Vec<usize>>::new();

        for (i, vertex) in up.arena.verts[first..end].iter().enumerate() {
            positions
                .entry(vertex.position.map(f32::to_bits))
                .or_default()
                .push(i);
        }

        let start_pipe = local.pipes as usize;
        let end_pipe = start_pipe + span.count.pipes as usize;
        let pipes = up.seg.pipes[start_pipe..end_pipe].to_vec();
        let pipe_vertices = pipes
            .iter()
            .map(|p| {
                Some([
                    *positions.get(&p.p0.map(f32::to_bits))?.first()?,
                    *positions.get(&p.p1.map(f32::to_bits))?.first()?,
                ])
            })
            .collect::<Option<Vec<_>>>()?;
        let pipe_normals = pipes
            .iter()
            .map(|p| {
                let candidates = positions.get(&p.p0.map(f32::to_bits))?;
                let first = *candidates.first()?;
                let other = candidates
                    .iter()
                    .copied()
                    .find(|&i| samples[i].surface != samples[first].surface)
                    .unwrap_or(first);
                Some([first, other])
            })
            .collect::<Option<Vec<_>>>()?;
        let chains = up
            .seg
            .pipe_chains
            .iter()
            .filter(|r| r.start >= local.pipes && r.end <= local.pipes + span.count.pipes)
            .map(|r| r.start - local.pipes..r.end - local.pipes)
            .collect();
        Some(Self {
            samples,
            vertices: up.arena.verts[first..end].to_vec(),
            controls: surfaces(geometry)?.iter().map(|s| s.m_cv.clone()).collect(),
            pipes,
            pipe_vertices,
            pipe_normals,
            chains,
        })
    }

    pub fn evaluate(&self, geometry: &Geometry) -> Option<(Vec<RenderVertex>, SegRows, AABB)> {
        let surfaces = surfaces(geometry)?;
        let changed: Vec<_> = surfaces
            .iter()
            .enumerate()
            .map(|(i, s)| self.controls.get(i) != Some(&s.m_cv))
            .collect();
        let mut bounds = AABB::empty();
        let mut vertices = Vec::with_capacity(self.samples.len());

        for (sample, baseline) in self.samples.iter().zip(&self.vertices) {
            let surface = surfaces.get(sample.surface as usize)?;

            if !changed[sample.surface as usize] {
                bounds.union_with_point(
                    baseline.position[0] as f64,
                    baseline.position[1] as f64,
                    baseline.position[2] as f64,
                );
                vertices.push(*baseline);
                continue;
            }

            let point = surface.point_at(sample.uv[0], sample.uv[1])?;
            let normal = surface.normal_at(sample.uv[0], sample.uv[1]);
            let position = point.to_f32();

            if position.iter().any(|p| !p.is_finite()) {
                return None;
            }

            bounds.union_with_point(position[0] as f64, position[1] as f64, position[2] as f64);
            vertices.push(RenderVertex {
                position,
                normal: [
                    normal[0] as f32 * sample.sign,
                    normal[1] as f32 * sample.sign,
                    normal[2] as f32 * sample.sign,
                ],
                color: baseline.color,
            });
        }

        let mut segments = SegRows {
            pipe_chains: self.chains.clone(),
            ..Default::default()
        };

        for ((pipe, ends), normals) in self
            .pipes
            .iter()
            .zip(&self.pipe_vertices)
            .zip(&self.pipe_normals)
        {
            let mut pipe = *pipe;
            pipe.p0 = vertices[ends[0]].position;
            pipe.p1 = vertices[ends[1]].position;
            let a = vertices[normals[0]].normal.map(f64::from);
            let b = vertices[normals[1]].normal.map(f64::from);
            pipe.facing = super::walk::encode::pack_facing(Some(&a), Some(&b));
            segments.pipes.push(pipe);
        }

        Some((vertices, segments, bounds))
    }

    pub fn allocated_bytes(&self) -> usize {
        self.samples.capacity() * std::mem::size_of::<Sample>()
            + self.vertices.capacity() * std::mem::size_of::<RenderVertex>()
            + self.controls.capacity() * std::mem::size_of::<Vec<f64>>()
            + self
                .controls
                .iter()
                .map(|v| v.capacity() * 8)
                .sum::<usize>()
            + self.pipes.capacity() * std::mem::size_of::<CylinderSegment>()
            + (self.pipe_vertices.capacity() + self.pipe_normals.capacity())
                * std::mem::size_of::<[usize; 2]>()
            + self.chains.capacity() * std::mem::size_of::<std::ops::Range<u32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::deform::{self, Target};
    use crate::app::walk::{Walk, WalkCx, walk_geometry};
    use crate::engine::gpu::patch::Counts;
    use session_rust::{BRep, Xform};
    use std::rc::Rc;

    #[test]
    fn joined_shell_preview_moves_source_samples_and_cancel_restores_them() {
        let source = Geometry::BRep(Rc::new(BRep::create_box(10.0, 10.0, 10.0)));
        let mut upload = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row: 3,
        };
        walk_geometry(&mut Walk::of(&mut upload), &cx, &source);
        let span = Span {
            start: Counts::default(),
            count: Counts::of(&upload),
        };
        let preview = SurfacePreview::capture(&upload, span, Counts::default(), &source)
            .expect("joined box has parameter provenance");
        let (before, _, _) = preview.evaluate(&source).unwrap();
        assert_eq!(before.len(), upload.arena.verts.len());

        for (a, b) in before.iter().zip(&upload.arena.verts) {
            assert_eq!(
                a.position, b.position,
                "initial sample is the drawn source position"
            );
        }

        let changed =
            deform::transform(&source, Target::Face(0), &Xform::translation(0.0, 0.0, 2.0))
                .unwrap();
        let Geometry::BRep(brep) = &changed else {
            unreachable!()
        };
        assert!(brep.is_solid(), "source shell remains joined");
        let (after, pipes, _) = preview.evaluate(&changed).unwrap();
        assert!(
            before
                .iter()
                .zip(&after)
                .any(|(a, b)| a.position != b.position)
        );
        assert!(
            before
                .iter()
                .zip(&after)
                .any(|(a, b)| a.position == b.position)
        );

        for pipe in pipes.pipes {
            assert!(after.iter().any(|v| v.position == pipe.p0));
            assert!(after.iter().any(|v| v.position == pipe.p1));
        }

        let (cancelled, _, _) = preview.evaluate(&source).unwrap();

        for (a, b) in before.iter().zip(&cancelled) {
            assert_eq!(a.position, b.position);
        }
    }
}

fn surfaces(geometry: &Geometry) -> Option<&[NurbsSurface]> {
    match geometry {
        Geometry::BRep(b) => Some(&b.m_surfaces),
        Geometry::NurbsSurface(s) => Some(std::slice::from_ref(s)),
        Geometry::Element(e) => match e.geometry() {
            session_rust::element::ElementGeometry::BRep(b) => Some(&b.m_surfaces),
            _ => None,
        },
        _ => None,
    }
}
