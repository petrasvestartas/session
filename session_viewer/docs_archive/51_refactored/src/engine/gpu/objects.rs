//! `ObjectRows` - the per-object columns ONE upload carries (true placement, tint, flags, local
//! AABB, vertex spacing; a delta, dropped after upload) - and `InstanceTable`, the ONE owner of
//! the object rows the GPU reads: the rows themselves, their f64 translations, the sparse list
//! of bounded rows, the re-anchor, the inside test, the two buffers and their bind group.

use crate::engine::pipelines::Layouts;
use crate::math::{mat_to_f32, xform_point_f64, Aabb, Mat4};
use session_rust::Point;
use super::buffers::{GpuCtx, GrowBuf};
use super::instance::Instance;

/// Re-anchor threshold band, WORLD units (mm): the table is rebased once the camera target
/// drifts a quarter of the view distance from the anchor, clamped to [MIN, MAX] - a zoomed-out
/// pan does not rebuild constantly, a zoomed-in pan re-anchors before f32 precision goes.
/// Within the band only the view matrix changes; f32 error at 1e5 mm from the anchor = 6e-3 mm.
const REANCHOR_MIN: f64 = 1.0e3;
const REANCHOR_MAX: f64 = 1.0e5;

/// The object columns of one upload, aligned by row - THIS upload's rows only. The walk numbers
/// them from `Scene.bases.obj`, so a row index is global while the columns are a delta.
#[derive(Default)]
pub struct ObjectRows {
    /// TRUE world model + tint + flags per object; the instance rows are rebased from it.
    pub rows: Vec<(Mat4, [f32; 4], u32)>,
    /// Mesh-local AABB per object row. None for linework/points/clouds: only the solid lane's
    /// facing cull needs it (see `Instance::FLAG_INSIDE`).
    pub bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per object row, world units. 0 = unknown (linework, points, clouds),
    /// which the ink lanes read as "never density-cull".
    pub spacing: Vec<f32>,
}

/// What one `rebase_anchor` call reports: the anchor in force, whether the table was just
/// rebuilt (the splats are stale), and whether a rebuild is due but throttled - the caller
/// then asks for another frame, or an idle viewer would keep the drifted anchor forever.
pub struct Rebase {
    pub anchor: Point,
    pub moved: bool,
    pub pending: bool,
}

/// A row that carries a world AABB - a mesh that drew ink. The inside test walks these and
/// never the whole table: 3 of 744,040 rows on the ten-sheet scene.
pub struct BoundedRow {
    pub row: u32,
    pub lo: [f64; 3],
    pub hi: [f64; 3],
}

/// The object rows as the GPU sees them (rotation/scale, tint, flags - the translation column
/// ZERO), the TRUE f64 translation per row, and the sparse bounded rows. The anchored
/// translations live in their own 16 B/row buffer: a re-anchor rewrites that, never the rows.
/// 96 + 24 B per object on the CPU, plus 32 B per bounded row.
pub struct InstanceTable {
    rows: Vec<Instance>,
    translation: Vec<[f64; 3]>,
    bounded: Vec<BoundedRow>,
    last_origin: Option<Point>, // rebuild skips when the camera target did not move
    buffer: GrowBuf, // the rows; written at append and per flipped inside flag
    translations: GrowBuf, // `[f32; 4]` per row; `rebuild` rewrites it whole on every re-anchor
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    /// Group 2 of every instance-reading pipeline (rows + translations); rebuilt when either grows.
    pub group: wgpu::BindGroup,
}

/// Group 2: the rows at binding 0, the anchored translations at binding 1.
fn instance_group(ctx: &GpuCtx, l: &Layouts, rows: &wgpu::Buffer, translations: &wgpu::Buffer) -> wgpu::BindGroup {
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("instances.bind_group"),
        layout: &l.instance,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: rows.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: translations.as_entire_binding() },
        ],
    })
}

impl InstanceTable {
    /// One placeholder row in both tables, so the first frame binds real buffers and draws
    /// nothing from them.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buffer = GrowBuf::new(ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, usage);
        let translations = GrowBuf::new(ctx, "instance.translations", 16, usage);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);

        Self {
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            bounded: Vec::new(),
            last_origin: None,
            buffer,
            translations,
            last_rebase_ms: 0.0,
            group,
        }
    }

    /// Append one upload's rows: cast once, keep the f64 translation, note the bounded ones,
    /// send only the new rows. The next frame rebases the whole table (`last_origin` cleared).
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        debug_assert_eq!(up.rows.len(), up.bounds.len());
        if self.translation.is_empty() {
            // First upload, or a rebuild that rewound everything: start the GPU tables over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.rows.clear();
            self.buffer.reset();
            self.translations.reset();
        }
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        for (i, (m, color, flags)) in up.rows.iter().enumerate() {
            // The diagonal, not an axis, is the extent: a flat sheet has a zero-thickness axis
            // and would clamp its ink lift to nothing.
            let world = up.bounds[i].map(|(lo, hi)| world_aabb(m, lo, hi));
            let extent = world.map_or(0.0, |(lo, hi)| diagonal(lo, hi));
            if let Some((lo, hi)) = world {
                self.bounded.push(BoundedRow { row: base + i as u32, lo, hi });
            }
            self.translation.push([m[12], m[13], m[14]]);
            let mut model = mat_to_f32(m);
            model[12] = 0.0;
            model[13] = 0.0;
            model[14] = 0.0;
            self.rows.push(Instance {
                model,
                color: *color,
                flags: *flags,
                extent,
                spacing: up.spacing.get(i).copied().unwrap_or(0.0),
                _pad: 0,
            });
        }

        if self.rows.is_empty() {
            self.rows.push(Instance::placeholder());
        }
        // The translations for the new rows are zero until the next frame rebases the whole
        // table (`last_origin` cleared below); the append only makes room and keeps the lengths equal.
        let fresh = &self.rows[self.buffer.len() as usize..];
        let zeros = vec![[0.0f32; 4]; fresh.len()];
        let grew = self.buffer.append(ctx, fresh);
        if self.translations.append(ctx, &zeros) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }
        self.last_origin = None; // force the next frame to rebase against the new table
    }

    /// The anchor the instance table is rebased about. A full rebuild runs only when the camera
    /// target strays past the `REANCHOR_MIN`/`REANCHOR_MAX` band from the current anchor - orbit
    /// never moves the target, and pan/zoom within the band just changes the view matrix.
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// below a no-op at 1/1000 scale, which silently turns camera-relative rendering off: the
    /// symptom is geometry that jitters and then clips away entirely as you zoom in, because the
    /// f32 mvp is differencing two large world magnitudes.
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > thresh
            }
        };
        // Throttled: during a wheel-zoom gesture the target moves every tick, and an every-frame
        // rebuild is the motion jank the rule forbids. Between rebuilds the old anchor stays
        // valid - farther from the eye than the band likes costs f32 precision, never a wrong image.
        let moved = need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none());
        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        Rebase { anchor: self.last_origin.clone().unwrap(), moved, pending: need && !moved }
    }

    /// Rebase every row's translation around `origin`: an f64 subtract against the TRUE world
    /// translation, then the cast to f32, into the 16 B/row translation table - the 96 B rows
    /// are not touched. What the GPU sees never holds a coordinate bigger than the camera's
    /// distance from `origin`, however far the scene sits from (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        let mut anchored: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());
        for t in &self.translation {
            anchored.push([(t[0] - origin[0]) as f32, (t[1] - origin[1]) as f32, (t[2] - origin[2]) as f32, 0.0]);
        }
        anchored.resize(self.rows.len(), [0.0; 4]); // the placeholder row of an empty scene
        ctx.queue.write_buffer(&self.translations.buf, 0, bytemuck::cast_slice(&anchored));
    }

    /// Row `i`'s model as a shader composes it: rotation/scale plus the anchored translation.
    /// The splat records fold this with the camera on the CPU.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;
        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            model[12] = (t[0] - o[0]) as f32;
            model[13] = (t[1] - o[1]) as f32;
            model[14] = (t[2] - o[2]) as f32;
        }
        Some(model)
    }

    /// Per-frame refresh of Instance::FLAG_INSIDE. The facing cull in both edge lanes assumes the
    /// eye is OUTSIDE the solid (both adjacent faces turned away = hidden edge); from inside, every
    /// face points away and the whole object loses its wireframe the moment the camera crosses a
    /// face. Per-edge data cannot tell "far side of the solid" from "eye inside it" - that
    /// difference is global - so the CPU answers it per BOUNDED row from the world AABBs, and the
    /// answer rides the instance row. Only a row whose answer FLIPS is written (96 B at its own
    /// offset), which orbit/zoom almost never does; the row's own flag bit is the change detector.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded.is_empty() {
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= scene.min[k] as f64 && ew[k] <= scene.max[k] as f64);
        for b in &self.bounded {
            let inside = in_scene && (0..3).all(|k| ew[k] >= b.lo[k] && ew[k] <= b.hi[k]);
            let Some(row) = self.rows.get_mut(b.row as usize) else { continue };
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }
            row.flags ^= Instance::FLAG_INSIDE;
            ctx.queue.write_buffer(&self.buffer.buf, b.row as u64 * std::mem::size_of::<Instance>() as u64, bytemuck::bytes_of(row));
        }
    }

    /// Forget every row AND hand the memory back, both sides: the one-row placeholder again,
    /// so a cleared scene holds nothing (`reset` keeps capacity for a rebuild).
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.bounded.shrink_to_fit();
        self.rows.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }

    /// Forget every row; the buffers keep their capacity. `bounded` goes with the rows it indexes:
    /// a scene cleared and then DRAWN before the next upload would test stale rows otherwise.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.translation.clear();
        self.bounded.clear();
        self.buffer.reset();
        self.translations.reset();
    }

    /// One instance row, as the GPU sees it (rebased about the anchor).
    pub fn row(&self, i: u32) -> Option<&Instance> {
        self.rows.get(i as usize)
    }

    /// Rows in the table - the frame's object count.
    pub fn len(&self) -> u32 {
        self.rows.len() as u32
    }

    /// The anchor the rows are rebased about, as the shaders read it; zero before the first frame.
    pub fn anchor_f32(&self) -> [f32; 3] {
        self.last_origin.as_ref().map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3])
    }
}

/// The AABB diagonal, world units - the lift clamp `Instance::extent` carries.
fn diagonal(lo: [f64; 3], hi: [f64; 3]) -> f32 {
    ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
}

/// World AABB of a local box: the 8 corners through the true transform. Conservative for
/// rotated placements - FLAG_INSIDE is a hint, not a cull.
fn world_aabb(m: &Mat4, lo: [f32; 3], hi: [f32; 3]) -> ([f64; 3], [f64; 3]) {
    let mut wlo = [f64::INFINITY; 3];
    let mut whi = [f64::NEG_INFINITY; 3];
    for c in 0..8 {
        let p = xform_point_f64(m, [
            (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
            (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
            (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
        ]);
        for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
    }
    (wlo, whi)
}
