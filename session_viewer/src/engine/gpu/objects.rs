//! `ObjectRows` - the per-object columns ONE upload carries (a delta, dropped after upload) -
//! and `InstanceTable`, the ONE owner of the object rows the GPU reads: the rows, their f64
//! translations, the sparse bounded rows, the re-anchor, the inside test, the two buffers and
//! their bind group.

use crate::engine::pipelines::Layouts;
use crate::math::{mat_scale, mat_to_f32, Aabb, Mat4};
use session_rust::Point;
use std::collections::BTreeMap;
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::instance::Instance;
use super::targets::Targets;

/// Re-anchor threshold band, world units: the table is rebased once the camera target drifts
/// a quarter of the view distance from the anchor, clamped to [MIN, MAX].
const REANCHOR_MIN: f64 = 1.0e3;
const REANCHOR_MAX: f64 = 1.0e5;

/// Re-anchors are throttled to this interval so a wheel-zoom gesture does not rebuild every tick.
const REANCHOR_THROTTLE_MS: f64 = 200.0;

/// Preserve the historical floor on the instance's thickness metadata.
const THICK_FLOOR: f32 = 0.001;

/// Scale the walk's thickness metadata into world units. Physical occlusion does not use it.
fn thickness(r: &ObjectRow) -> f32 {
    let scale = mat_scale(&mat_to_f32(&r.place)) as f32;
    let thin = r.thickness * scale;
    thin.max(THICK_FLOOR * r.bounds.placed(&r.place).diagonal())
}

/// One object as the walk reports it: its true placement, tint, flags, mesh-local box
/// (empty when the object has no volume the ink lanes care about) and vertex spacing.
#[derive(Clone)]
pub struct ObjectRow {
    pub place: Mat4,
    pub color: [f32; 4],
    pub flags: u32,
    pub bounds: Aabb,
    /// Meshes: the local vertex spacing; clouds: the point size in px. Read as a pen hint.
    pub spacing: f32,
    /// The row drew faces, so the per-frame inside test walks its box.
    pub faces: bool,
    /// The object's thickness in local units, orientation-free (the walk measures it).
    pub thickness: f32,
}

impl ObjectRow {
    /// A row with the file placement, white tint and no columns filled yet.
    pub fn new(place: Mat4, flags: u32) -> Self {
        Self { place, color: [1.0; 4], flags, bounds: Aabb::empty(), spacing: 0.0, faces: false, thickness: 0.0 }
    }
}

/// The object rows of one upload - THIS upload's rows only; `Scene.bases.obj` numbers them.
#[derive(Default)]
pub struct ObjectRows {
    pub rows: Vec<ObjectRow>,
}

/// What one `rebase_anchor` call reports: the anchor in force, whether the table was just
/// rebuilt, and whether a rebuild is due but throttled (the caller asks for another frame).
pub struct Rebase {
    pub anchor: Point,
    pub moved: bool,
    pub pending: bool,
}

/// A row that drew faces and carries a world box. The inside test walks these only.
struct BoundedRow {
    row: u32,
    lo: [f64; 3],
    hi: [f64; 3],
}

/// The object rows as the GPU sees them, the TRUE f64 translation per row, and the sparse
/// bounded rows. The anchored translations live in their own 16 B/row buffer.
pub struct InstanceTable {
    rows: Vec<Instance>,
    translation: Vec<[f64; 3]>,
    bounded: Vec<BoundedRow>,
    local_bounds: Vec<Aabb>,
    occluders: BTreeMap<u32, bool>,
    last_origin: Option<Point>,
    buffer: GrowBuf,
    translations: GrowBuf,
    last_rebase_ms: f64,
    /// Group 2 of every instance-reading pipeline; rebuilt when either buffer grows.
    pub group: wgpu::BindGroup,
    pub ink_group: wgpu::BindGroup,
}

/// Group 2: the rows at binding 0, the anchored translations at binding 1.
fn instance_group(ctx: &GpuCtx, l: &Layouts, rows: &wgpu::Buffer, translations: &wgpu::Buffer) -> wgpu::BindGroup {
    bind_group(ctx, &l.instance, "instances.bind_group", &[rows, translations])
}

/// Immutable physical scene inputs bound beside each ink lane's instance columns.
pub struct InkScene<'a> {
    pub targets: &'a Targets,
    pub planes: &'a wgpu::Buffer,
}

/// Group 2 for ink: instance columns plus the previous pass's immutable attachments.
fn ink_instance_group(ctx: &GpuCtx, l: &Layouts, buffers: [&wgpu::Buffer; 2], scene: &InkScene) -> wgpu::BindGroup {
    let targets = scene.targets;
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ink.instances.bind_group"),
        layout: &l.ink_instance,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffers[0].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: buffers[1].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&targets.depth_single) },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&targets.depth_msaa) },
            wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&targets.faces_single) },
            wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&targets.faces_msaa) },
            wgpu::BindGroupEntry { binding: 6, resource: scene.planes.as_entire_binding() },
        ],
    })
}

impl InstanceTable {
    /// Current anchored translations, including any replacement after growth/reanchor.
    pub(super) fn translation_buffer(&self) -> &wgpu::Buffer { &self.translations.buf }

    /// Reanchoring is the only translation rewrite outside append/reset, which separately
    /// invalidate the arena filter. Keep f64 bits: distinct anchors may round to equal f32s.
    pub(super) fn translation_origin_bits(&self) -> Option<[u64; 3]> {
        self.last_origin.as_ref().map(|p| [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()])
    }

    /// One placeholder row in both tables, so the first frame binds real buffers.
    pub fn new(ctx: &GpuCtx, l: &Layouts, scene: &InkScene) -> Self {
        let buffer = GrowBuf::new(ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, ROWS);
        let translations = GrowBuf::new(ctx, "instance.translations", 16, ROWS);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);
        let ink_group = ink_instance_group(ctx, l, [&buffer.buf, &translations.buf], scene);

        Self {
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            bounded: Vec::new(),
            local_bounds: Vec::new(),
            occluders: BTreeMap::new(),
            last_origin: None,
            buffer,
            translations,
            last_rebase_ms: 0.0,
            group,
            ink_group,
        }
    }

    /// Refresh sampled attachment and instance bindings after upload, resize, or release.
    pub fn rebind_ink(&mut self, ctx: &GpuCtx, l: &Layouts, scene: &InkScene) {
        self.ink_group = ink_instance_group(ctx, l, [&self.buffer.buf, &self.translations.buf], scene);
    }

    /// Append one upload's rows: cast once, keep the f64 translation, note the bounded ones,
    /// send only the new rows. The next frame rebases the whole table.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        if self.translation.is_empty() {
            self.rows.clear();
            self.buffer.reset();
            self.translations.reset();
        }
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.local_bounds.reserve(up.rows.len());
        for (i, r) in up.rows.iter().enumerate() {
            self.local_bounds.push(r.bounds);
            let world = r.bounds.placed(&r.place);
            if r.faces && world.is_finite() {
                let lo = [world.min[0] as f64, world.min[1] as f64, world.min[2] as f64];
                let hi = [world.max[0] as f64, world.max[1] as f64, world.max[2] as f64];
                self.bounded.push(BoundedRow { row: base + i as u32, lo, hi });
            }
            self.translation.push([r.place[12], r.place[13], r.place[14]]);
            let mut model = mat_to_f32(&r.place);
            model[12] = 0.0;
            model[13] = 0.0;
            model[14] = 0.0;
            let thickness = thickness(r);
            self.rows.push(Instance { model, color: r.color, flags: r.flags, thickness, spacing: r.spacing, _pad: 0 });
        }
        if self.rows.is_empty() {
            self.rows.push(Instance::placeholder());
        }

        let fresh = &self.rows[self.buffer.len() as usize..];
        if fresh.is_empty() {
            return;
        }
        let zeros = vec![[0.0f32; 4]; fresh.len()];
        let grew = self.buffer.append(ctx, fresh);
        if self.translations.append(ctx, &zeros) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }
        self.last_origin = None;
    }

    /// Only real physical face owners and resident cloud chunks enlarge the lookup region.
    /// Cloud chunks may extend an existing object after its initial prefix was uploaded.
    pub fn append_occlusion(&mut self, up: &super::upload::Upload) {
        let mut previous = None;
        for plane in &up.arena.face_planes {
            if previous == Some(plane.instance_id) { continue; }
            self.occluders.entry(plane.instance_id).or_insert(false);
            previous = Some(plane.instance_id);
        }
        for draw in &up.cloud.draws {
            self.occluders.insert(draw.instance, true);
            let bounds = &mut self.local_bounds[draw.instance as usize];
            let first = draw.first as usize * 3;
            let end = first + draw.count as usize * 3;
            for point in up.cloud.pos[first..end].chunks_exact(3) {
                bounds.grow([point[0], point[1], point[2]]);
            }
        }
    }

    /// A conservative union of physical raster footprints. Invalid or clipped bounds use
    /// the whole viewport; an inverted rectangle means that no physical writer exists.
    pub fn occluder_rect(&self, mvp: &[f32; 16], size: (u32, u32), cloud_size: f32) -> [f32; 4] {
        let mut rect = [f32::INFINITY, f32::INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY];
        for (&row, &cloud) in &self.occluders {
            let Some(model) = self.anchored_model(row) else { return [0.0, 0.0, size.0 as f32, size.1 as f32] };
            let bounds = &self.local_bounds[row as usize];
            // Splat radii are capped at eight pixels; preserve larger manifest floors too.
            let radius = if cloud { (self.rows[row as usize].spacing * cloud_size * 0.5).max(8.0) } else { 0.0 };
            let projection = super::occlusion_bounds::Projection { mvp, size, padding: radius as f64 + 2.0 };
            let Some(projected) = super::occlusion_bounds::project(bounds, &model, &projection)
                else { return [0.0, 0.0, size.0 as f32, size.1 as f32] };
            rect[0] = rect[0].min(projected[0]);
            rect[1] = rect[1].min(projected[1]);
            rect[2] = rect[2].max(projected[2]);
            rect[3] = rect[3].max(projected[3]);
        }
        if self.occluders.is_empty() { [1.0, 1.0, -1.0, -1.0] } else { rect }
    }

    /// The anchor the table is rebased about. A rebuild runs only when the camera target
    /// strays past the band from the current anchor; `origin` and `view_dist` are world units.
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let d = [a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() > thresh
            }
        };
        let moved = need && (self.last_origin.is_none() || now - self.last_rebase_ms > REANCHOR_THROTTLE_MS);
        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        Rebase { anchor: self.last_origin.clone().unwrap(), moved, pending: need && !moved }
    }

    /// Rebase every row's translation around `origin` in f64, cast, and rewrite the 16 B/row
    /// translation table; the 96 B rows are not touched.
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        let mut anchored: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());
        for t in &self.translation {
            anchored.push([(t[0] - origin[0]) as f32, (t[1] - origin[1]) as f32, (t[2] - origin[2]) as f32, 0.0]);
        }
        anchored.resize(self.rows.len(), [0.0; 4]);
        self.translations.write_at(ctx, 0, &anchored);
    }

    /// Row `i`'s model as a shader composes it: rotation/scale plus the anchored translation.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;
        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            model[12] = (t[0] - o[0]) as f32;
            model[13] = (t[1] - o[1]) as f32;
            model[14] = (t[2] - o[2]) as f32;
        }
        Some(model)
    }

    /// Per-frame refresh of `FLAG_INSIDE` over the bounded rows only; a row is written back
    /// only when its answer flips.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded.is_empty() {
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        let in_scene = scene.contains(ew);
        for b in &self.bounded {
            let inside = in_scene && (0..3).all(|k| ew[k] >= b.lo[k] && ew[k] <= b.hi[k]);
            let Some(row) = self.rows.get_mut(b.row as usize) else { continue };
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }
            row.flags ^= Instance::FLAG_INSIDE;
            self.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }

    /// Set or clear one flag bit on one row and write that row back.
    pub fn set_flag(&mut self, ctx: &GpuCtx, row: u32, bit: u32, on: bool) {
        let Some(r) = self.rows.get_mut(row as usize) else { return };
        let was = r.flags & bit != 0;
        if was == on {
            return;
        }
        r.flags ^= bit;
        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.translation.clear();
        self.bounded.clear();
        self.local_bounds.clear();
        self.occluders.clear();
        self.buffer.reset();
        self.translations.reset();
        self.last_origin = None;
    }

    /// Forget every row AND hand the memory back, both sides.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.bounded.shrink_to_fit();
        self.local_bounds.shrink_to_fit();
        self.rows.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }

    /// One instance row as the GPU sees it.
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
