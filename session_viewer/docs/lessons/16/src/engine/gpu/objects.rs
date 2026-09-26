// --8<-- [start:rows]
use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
use super::hull::{Hull, placed_box};
use super::instance::Instance;
use super::targets::Targets;
use crate::engine::pipelines::Layouts;
use session_rust::{AABB, Point, Xform};

/// Smallest camera drift, world units, that moves the anchor.
const REANCHOR_MIN: f64 = 1.0e3;

/// Largest camera drift, world units, before the anchor must move.
const REANCHOR_MAX: f64 = 1.0e5;

/// Least time between two anchor moves, ms.
const REANCHOR_THROTTLE_MS: f64 = 200.0;

/// One object row as the CPU builds it.
#[derive(Clone)]
pub struct ObjectRow {
    pub place: Xform,       // world placement
    pub color: [f32; 4],    // rgba tint
    pub edge_color: u32,    // packed edge color
    pub flags: u32,         // Instance::FLAG_* bits
    pub bounds: AABB,       // box in the object's own space
    pub spacing: f32,       // vertex spacing, or point size for clouds
    pub faces: bool,        // true when the object drew faces
    pub hull: Option<Hull>, // extreme points in the object's own space, for an exact turned box
}

impl ObjectRow {
    /// A row with a placement and flags, everything else empty.
    pub fn new(place: Xform, flags: u32) -> Self {
        Self {
            place,
            color: [1.0; 4],
            edge_color: 0,
            flags,
            bounds: AABB::empty(),
            spacing: 0.0,
            faces: false,
            hull: None,
        }
    }
}

/// Object rows of one upload.
#[derive(Default)]
pub struct ObjectRows {
    pub rows: Vec<ObjectRow>, // one per object
}

/// Result of a `rebase_anchor` call.
pub struct Rebase {
    pub anchor: Point, // current scene origin
    pub moved: bool,   // true when the table was rebuilt now
    pub pending: bool, // true when a rebuild waits on the throttle
}
// --8<-- [end:rows]

// --8<-- [start:boxes]
/// A row with faces and its world box, for the inside test.
struct BoundedRow {
    row: u32,     // object row
    lo: [f64; 3], // box minimum
    hi: [f64; 3], // box maximum
}

/// The row's box in world space: exact over its extreme points, else its own box turned.
pub fn world_box(r: &ObjectRow) -> AABB {
    placed_bounds(&r.bounds, r.hull.as_deref(), &r.place)
}

/// `local` placed by `place`, exact when the extreme points are known.
fn placed_bounds(local: &AABB, hull: Option<&[[f32; 3]]>, place: &Xform) -> AABB {
    match hull {
        Some(points) if local.is_valid() => placed_box(points, place),
        _ => local.transformed(place),
    }
}

/// SSAO contact radius: 5% of the box diagonal.
fn ambient_radius(bounds: &AABB) -> f32 {
    if !bounds.is_valid() {
        return 0.0;
    }
    (0.05 * bounds.hx.hypot(bounds.hy).hypot(bounds.hz)).max(0.01) as f32
}

/// Split a placement into matrix, translation and world box.
fn placed_row(
    local: &AABB,
    hull: Option<&[[f32; 3]]>,
    place: &Xform,
) -> ([f32; 16], [f64; 3], AABB) {
    let world = placed_bounds(local, hull, place);
    let mut model = place.to_f32();
    // translation goes in its own table
    model[12] = 0.0;
    model[13] = 0.0;
    model[14] = 0.0;
    (
        model,
        [place.m[12], place.m[13], place.m[14]],
        if world.is_valid() {
            world
        } else {
            AABB::empty()
        },
    )
}

/// Add `row` to the sorted `rows`, or take it out.
fn track_in(rows: &mut Vec<u32>, row: u32, on: bool) {
    match (rows.binary_search(&row), on) {
        (Err(at), true) => rows.insert(at, row),
        (Ok(at), false) => {
            rows.remove(at);
        }
        _ => {}
    }
}

/// True when two boxes are the same, bit for bit.
fn same_box(a: &AABB, b: &AABB) -> bool {
    [a.cx, a.cy, a.cz, a.hx, a.hy, a.hz]
        .iter()
        .zip([b.cx, b.cy, b.cz, b.hx, b.hy, b.hz])
        .all(|(x, y)| x.to_bits() == y.to_bits())
}

/// True when two rows share the same extreme points, or neither has any.
fn same_hull(a: Option<&Hull>, b: Option<&Hull>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Hull::ptr_eq(a, b),
        (a, b) => a.is_none() && b.is_none(),
    }
}

// f32 keeps about 7 digits: at 1,000,000 mm a 1 mm step is lost, so the GPU gets positions relative to a nearby anchor.
/// Translation relative to the origin, as the GPU reads it.
fn anchored(t: [f64; 3], origin: &Point) -> [f32; 4] {
    [
        (t[0] - origin[0]) as f32,
        (t[1] - origin[1]) as f32,
        (t[2] - origin[2]) as f32,
        0.0,
    ]
}
// --8<-- [end:boxes]

// --8<-- [start:table]
/// The object rows on the GPU and their exact positions on the CPU.
pub struct InstanceTable {
    geometry_revision: u64,             // bumps when anything moves or hides
    rows: Vec<Instance>,                // the rows, as uploaded
    translation: Vec<[f64; 3]>,         // exact world position per row
    local_bounds: Vec<AABB>,            // box per row, in the object's own space
    hulls: Vec<Option<Hull>>,           // extreme points per row; empty until a row has some
    widget: Option<u32>,                // identity row the gumball draws with
    bounded: Vec<BoundedRow>,           // rows with faces, for the inside test
    bounded_at: Vec<u32>,               // index of each row in `bounded`, u32::MAX = none
    world_bounds: Vec<AABB>,            // box per row in world space
    clipping: Vec<u32>,                 // rows of clipping planes, sorted
    closed: Vec<u32>,                   // rows of verified closed solids, sorted
    last_origin: Option<Point>,         // origin the GPU positions are measured from
    buffer: GrowBuf,                    // Instance rows on the GPU
    translations: GrowBuf,              // positions minus the scene origin, on the GPU
    last_rebase_ms: f64,                // when the origin last moved
    pub group: wgpu::BindGroup,         // group 2: rows and translations
    ink_group: Option<wgpu::BindGroup>, // group 2 for ink, with depth textures; register:ink
}

/// Bind group 2: rows at binding 0, translations at 1.
fn instance_group(
    ctx: &GpuCtx,
    l: &Layouts,
    rows: &wgpu::Buffer,
    translations: &wgpu::Buffer,
) -> wgpu::BindGroup {
    bind_group(
        ctx,
        &l.instance,
        "instances.bind_group",
        &[rows, translations],
    )
}
// --8<-- [end:table]

// --8<-- [start:table-append]
impl InstanceTable {
    /// Instance rows for read-only GPU passes.
    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buf
    }

    pub fn geometry_revision(&self) -> u64 {
        self.geometry_revision
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.buffer.buf.size() + self.translations.buf.size()
    }

    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            ROWS,
        );
        let translations = GrowBuf::new(ctx, "instance.translations", 16, ROWS);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);

        Self {
            geometry_revision: 0,
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            local_bounds: Vec::new(),
            hulls: Vec::new(),
            widget: None,
            bounded: Vec::new(),
            bounded_at: Vec::new(),
            world_bounds: Vec::new(),
            clipping: Vec::new(),
            closed: Vec::new(),
            last_origin: None,
            buffer,
            translations,
            last_rebase_ms: 0.0,
            group,
            ink_group: None, // register:ink
        }
    }

    /// Append one upload's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);

        // drop the widget row first; it must stay last
        if let Some(widget) = self.widget.take() {
            let keep = widget as usize;
            self.rows.truncate(keep);
            self.translation.truncate(keep);
            self.local_bounds.truncate(keep);
            self.hulls.truncate(keep);
            self.world_bounds.truncate(keep);
            self.bounded_at.truncate(keep);
            // rewind both buffers by re-appending the kept rows
            self.buffer.reset();
            self.translations.reset();

            if keep > 0 {
                let rows: Vec<Instance> = self.rows.clone();
                let anchored_rows: Vec<[f32; 4]> = match &self.last_origin {
                    Some(origin) => self
                        .translation
                        .iter()
                        .map(|t| anchored(*t, origin))
                        .collect(),
                    None => vec![[0.0f32; 4]; keep],
                };
                let grew = self.buffer.append(ctx, &rows);

                if self.translations.append(ctx, &anchored_rows) || grew {
                    self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
                }
            }
        }

        // first upload replaces the placeholder
        if self.translation.is_empty() {
            self.rows.clear();
            self.world_bounds.clear();
            self.local_bounds.clear();
            self.hulls.clear();
            self.bounded_at.clear();
            self.buffer.reset();
            self.translations.reset();
        }

        // row number of the first new object
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.world_bounds.reserve(up.rows.len());
        self.local_bounds.reserve(up.rows.len());
        self.bounded_at.reserve(up.rows.len());

        for (i, r) in up.rows.iter().enumerate() {
            let world = world_box(r);

            // rows with faces join the inside test
            if r.faces && world.is_valid() {
                let lo = world.min_point();
                let hi = world.max_point();
                self.bounded_at.push(self.bounded.len() as u32);
                self.bounded.push(BoundedRow {
                    row: base + i as u32,
                    lo: [lo[0], lo[1], lo[2]],
                    hi: [hi[0], hi[1], hi[2]],
                });
            } else {
                self.bounded_at.push(u32::MAX);
            }

            self.world_bounds.push(if world.is_valid() {
                world
            } else {
                AABB::empty()
            });
            self.local_bounds.push(r.bounds);
            self.set_hull(base + i as u32, r.hull.clone());

            if r.flags & (Instance::FLAG_CLIPPING_PLANE | Instance::FLAG_CLOSED) != 0 {
                self.track(base + i as u32, r.flags);
            }

            self.translation
                .push([r.place.m[12], r.place.m[13], r.place.m[14]]);
            // matrix without translation
            let mut model = r.place.to_f32();
            model[12] = 0.0;
            model[13] = 0.0;
            model[14] = 0.0;
            self.rows.push(Instance {
                model,
                color: r.color,
                flags: r.flags,
                ao_radius: ambient_radius(&world),
                spacing: r.spacing,
                _pad: r.edge_color,
            });
        }

        if self.rows.is_empty() {
            self.rows.push(Instance::placeholder());
        }

        // rows not yet on the GPU
        let first = self.buffer.len() as usize;
        let fresh = &self.rows[first..];

        if fresh.is_empty() {
            return;
        }

        // measured from the current origin; zeros wait for the first rebase
        let translations: Vec<[f32; 4]> = (first..self.rows.len())
            .map(|i| match (&self.last_origin, self.translation.get(i)) {
                (Some(origin), Some(t)) => anchored(*t, origin),
                _ => [0.0; 4],
            })
            .collect();
        let grew = self.buffer.append(ctx, fresh);

        if self.translations.append(ctx, &translations) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }
    }
// --8<-- [end:table-append]

// --8<-- [start:anchor]
    /// Measure from the camera again at the next frame, as after loading a document.
    pub fn forget_anchor(&mut self) {
        self.last_origin = None;
    }

    /// Move the scene origin when the camera drifted far enough.
    pub fn rebase_anchor(
        &mut self,
        ctx: &GpuCtx,
        origin: &Point,
        view_dist: f64,
        now: f64,
    ) -> Rebase {
        // a quarter of the view distance, clamped
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        // drifted past the threshold?
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let d = [a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() > thresh
            }
        };
        // rebuild now unless throttled
        let moved = need
            && (self.last_origin.is_none() || now - self.last_rebase_ms > REANCHOR_THROTTLE_MS);

        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }

        Rebase {
            // set by rebuild above or earlier
            anchor: self.last_origin.clone().unwrap(),
            moved,
            pending: need && !moved,
        }
    }

    /// Recompute every relative translation for a new origin.
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.last_origin = Some(origin.clone());
        let mut rebased: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());

        for t in &self.translation {
            rebased.push(anchored(*t, origin));
        }

        rebased.resize(self.rows.len(), [0.0; 4]);
        self.translations.write_at(ctx, 0, &rebased);
    }

    /// Row `i`'s full matrix as the shader composes it.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;

        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            let a = anchored(*t, o);
            model[12] = a[0];
            model[13] = a[1];
            model[14] = a[2];
        }

        Some(model)
    }
// --8<-- [end:anchor]

// --8<-- [start:inside]
    /// Set FLAG_INSIDE on rows whose box contains the eye.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &AABB) {
        if self.bounded.is_empty() {
            return;
        }

        let Some(origin) = self.last_origin.clone() else {
            return;
        };
        // eye in world space
        let ew = [
            origin[0] + eye[0] as f64,
            origin[1] + eye[1] as f64,
            origin[2] + eye[2] as f64,
        ];
        let in_scene = scene.contains(&Point::new(ew[0], ew[1], ew[2]));

        for b in &self.bounded {
            let mut inside = in_scene;

            if inside {
                for (coordinate, (low, high)) in ew.iter().zip(b.lo.iter().zip(&b.hi)) {
                    if !(coordinate >= low && coordinate <= high) {
                        inside = false;
                        break;
                    }
                }
            }

            let Some(row) = self.rows.get_mut(b.row as usize) else {
                continue;
            };

            // write only when the answer changed
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }

            row.flags ^= Instance::FLAG_INSIDE;
            self.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }

    /// Row the gumball draws with: identity, appended once, last.
    pub fn widget_row(&mut self, ctx: &GpuCtx, l: &Layouts) -> (u32, bool) {
        if let Some(row) = self.widget {
            return (row, false);
        }

        let row = self.rows.len() as u32;
        // identity matrix, white tint
        self.rows.push(Instance {
            color: [1.0; 4],
            ..Instance::placeholder()
        });
        self.translation.push([0.0; 3]);
        self.local_bounds.push(AABB::empty());
        self.world_bounds.push(AABB::empty());
        self.bounded_at.push(u32::MAX);
        // world position zero, relative to the origin
        let translation = match &self.last_origin {
            Some(origin) => anchored([0.0; 3], origin),
            None => [0.0; 4],
        };
        // append grows the buffers if needed
        let grew = self
            .buffer
            .append(ctx, std::slice::from_ref(&self.rows[row as usize]));
        let grew_t = self
            .translations
            .append(ctx, std::slice::from_ref(&translation));

        if grew || grew_t {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }

        self.widget = Some(row);
        (row, grew || grew_t)
    }
// --8<-- [end:inside]

// --8<-- [start:placement]
    /// Move one object; writes only its row.
    pub fn set_placement(&mut self, ctx: &GpuCtx, row: u32, place: &Xform) -> bool {
        let i = row as usize;
        let (Some(instance), Some(local)) = (self.rows.get_mut(i), self.local_bounds.get(i)) else {
            return false;
        };
        let hull = self.hulls.get(i).and_then(|h| h.as_deref());
        let (model, translation, world) = placed_row(local, hull, place);
        instance.model = model;
        instance.ao_radius = ambient_radius(&world);
        self.translation[i] = translation;
        self.world_bounds[i] = world;

        // keep the inside-test box in step
        if let Some(&at) = self.bounded_at.get(i)
            && let Some(b) = self.bounded.get_mut(at as usize)
        {
            let lo = world.min_point();
            let hi = world.max_point();
            b.lo = [lo[0], lo[1], lo[2]];
            b.hi = [hi[0], hi[1], hi[2]];
        }

        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        let instance = *instance;
        self.buffer
            .write_at(ctx, row, std::slice::from_ref(&instance));

        if let Some(origin) = &self.last_origin {
            let t = anchored(self.translation[i], origin);
            self.translations
                .write_at(ctx, row, std::slice::from_ref(&t));
        }

        true
    }

    // `pub(crate)` = visible anywhere in this crate and nowhere outside it.
    /// Set a row's box and spacing after its geometry changed.
    pub(crate) fn set_geometry_bounds(
        &mut self,
        ctx: &GpuCtx,
        row: u32,
        bounds: AABB,
        spacing: f32,
        place: &Xform,
    ) {
        self.local_bounds[row as usize] = bounds;
        self.set_hull(row, None);
        self.rows[row as usize].spacing = spacing;
        self.set_placement(ctx, row, place);
    }

    /// Join or leave the inside test with `world`, in O(1).
    fn set_bounded(&mut self, row: u32, on: bool, world: &AABB) {
        let at = self.bounded_at[row as usize];

        if on {
            let lo = world.min_point();
            let hi = world.max_point();
            let entry = BoundedRow {
                row,
                lo: [lo[0], lo[1], lo[2]],
                hi: [hi[0], hi[1], hi[2]],
            };

            if at == u32::MAX {
                self.bounded_at[row as usize] = self.bounded.len() as u32;
                self.bounded.push(entry);
            } else {
                self.bounded[at as usize] = entry;
            }

            return;
        }

        if at == u32::MAX {
            return;
        }

        self.bounded.swap_remove(at as usize);
        self.bounded_at[row as usize] = u32::MAX;

        // the last entry moved into the gap
        if let Some(moved) = self.bounded.get(at as usize) {
            self.bounded_at[moved.row as usize] = at;
        }
    }

    /// Keep `hull` for row `row`; the table grows only once some row has one.
    fn set_hull(&mut self, row: u32, hull: Option<Hull>) {
        let i = row as usize;

        if i >= self.hulls.len() {
            if hull.is_none() {
                return;
            }

            self.hulls.resize(i + 1, None);
        }

        self.hulls[i] = hull;
    }

    /// Write one row and its translation to the GPU.
    fn write_row(&mut self, ctx: &GpuCtx, row: u32) {
        let i = row as usize;
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.buffer
            .write_at(ctx, row, std::slice::from_ref(&self.rows[i]));

        if let Some(origin) = &self.last_origin {
            let t = anchored(self.translation[i], origin);
            self.translations
                .write_at(ctx, row, std::slice::from_ref(&t));
        }
    }

    /// Take a whole new row on an id that was free.
    pub fn set_row(&mut self, ctx: &GpuCtx, row: u32, r: &ObjectRow) {
        let i = row as usize;
        let (model, translation, world) = placed_row(&r.bounds, r.hull.as_deref(), &r.place);
        self.rows[i] = Instance {
            model,
            color: r.color,
            flags: r.flags,
            ao_radius: ambient_radius(&world),
            spacing: r.spacing,
            _pad: r.edge_color,
        };
        self.translation[i] = translation;
        self.local_bounds[i] = r.bounds;
        self.set_hull(row, r.hull.clone());
        self.world_bounds[i] = world;
        self.set_bounded(row, r.faces && world.is_valid(), &world);
        self.track(row, r.flags);
        self.write_row(ctx, row);
    }

    /// Keep the clipping plane and closed solid rows in step with row `row`'s new flags.
    fn track(&mut self, row: u32, flags: u32) {
        let live = flags & Instance::FLAG_DEAD == 0;
        track_in(
            &mut self.clipping,
            row,
            live && flags & Instance::FLAG_CLIPPING_PLANE != 0,
        );
        track_in(
            &mut self.closed,
            row,
            live && flags & Instance::FLAG_CLOSED != 0,
        );
    }

    /// Rows of clipping planes, hidden ones included.
    pub fn clipping_rows(&self) -> &[u32] {
        &self.clipping
    }

    /// Rows of verified closed solids, hidden ones included.
    pub fn closed_rows(&self) -> &[u32] {
        &self.closed
    }

    /// Row `row`'s placement as the GPU draws it, drag previews included.
    pub fn placement(&self, row: u32) -> Option<Xform> {
        let i = row as usize;
        let (instance, translation) = (self.rows.get(i)?, self.translation.get(i)?);
        let mut m = instance.model.map(f64::from);
        m[12] = translation[0];
        m[13] = translation[1];
        m[14] = translation[2];
        Some(Xform::from_matrix(m))
    }
// --8<-- [end:placement]

// --8<-- [start:bury]
    /// Take a redrawn row's box, spacing and drawing flags; selection, visibility and colors stay.
    pub fn update_geometry(&mut self, ctx: &GpuCtx, row: u32, r: &ObjectRow) {
        const KEEP: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_INSIDE
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        let i = row as usize;
        let (model, translation, world) = placed_row(&r.bounds, r.hull.as_deref(), &r.place);
        let before = self.rows[i];
        let instance = &mut self.rows[i];
        instance.model = model;
        instance.flags = (instance.flags & KEEP) | (r.flags & !KEEP);
        instance.ao_radius = ambient_radius(&world);
        instance.spacing = r.spacing;
        let bounded = r.faces && world.is_valid();
        let flags = self.rows[i].flags;
        self.track(row, flags);
        let same = bytemuck::bytes_of(&before) == bytemuck::bytes_of(&self.rows[i])
            && self.translation[i] == translation
            && same_box(&self.local_bounds[i], &r.bounds)
            && same_hull(self.hulls.get(i).and_then(Option::as_ref), r.hull.as_ref())
            && (self.bounded_at[i] != u32::MAX) == bounded;

        // a compaction walks every row again; most come back unchanged
        if same {
            return;
        }

        self.translation[i] = translation;
        self.local_bounds[i] = r.bounds;
        self.set_hull(row, r.hull.clone());
        self.world_bounds[i] = world;
        self.set_bounded(row, bounded, &world);
        self.write_row(ctx, row);
    }

    /// Hide rows for good and drop their boxes; contiguous rows share one write.
    pub fn retire_many(&mut self, ctx: &GpuCtx, rows: &[u32]) {
        if rows.is_empty() {
            return;
        }

        let mut sorted = rows.to_vec();
        sorted.sort_unstable();
        sorted.dedup();

        for &row in &sorted {
            let i = row as usize;
            let instance = &mut self.rows[i];
            instance.flags = Instance::FLAG_HIDDEN | Instance::FLAG_DEAD;
            instance.ao_radius = 0.0;
            self.track(row, Instance::FLAG_DEAD);
            self.local_bounds[i] = AABB::empty();
            self.set_hull(row, None);
            self.world_bounds[i] = AABB::empty();
            self.set_bounded(row, false, &AABB::empty());
        }

        self.geometry_revision = self.geometry_revision.wrapping_add(1);

        for run in sorted.chunk_by(|a, b| a + 1 == *b) {
            let first = run[0] as usize;
            self.buffer
                .write_at(ctx, run[0], &self.rows[first..first + run.len()]);
        }
    }

    // Undo never frees GPU memory: a deleted object's row is hidden and kept, so its undo is one write, not a new upload.
    /// Hide rows whose objects an undo may bring back; their drawing flags and own boxes stay for `unbury`.
    pub fn bury_many(&mut self, ctx: &GpuCtx, rows: &[u32]) {
        if rows.is_empty() {
            return;
        }

        let mut sorted = rows.to_vec();
        sorted.sort_unstable();
        sorted.dedup();

        for &row in &sorted {
            let i = row as usize;
            let instance = &mut self.rows[i];
            instance.flags = (instance.flags & !Instance::FLAG_SELECTED)
                | Instance::FLAG_HIDDEN
                | Instance::FLAG_DEAD;
            self.track(row, Instance::FLAG_DEAD);
            self.world_bounds[i] = AABB::empty();
            self.set_bounded(row, false, &AABB::empty());
        }

        self.geometry_revision = self.geometry_revision.wrapping_add(1);

        for run in sorted.chunk_by(|a, b| a + 1 == *b) {
            let first = run[0] as usize;
            self.buffer
                .write_at(ctx, run[0], &self.rows[first..first + run.len()]);
        }
    }

    /// Show a buried row again at `r`'s placement with its colors and hidden bit; the walk's flags and box stay.
    pub fn unbury(&mut self, ctx: &GpuCtx, row: u32, r: &ObjectRow) {
        const OWN: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        let i = row as usize;
        let hull = self.hulls.get(i).and_then(|h| h.as_deref());
        let (model, translation, world) = placed_row(&self.local_bounds[i], hull, &r.place);
        let instance = &mut self.rows[i];
        instance.model = model;
        instance.color = r.color;
        instance.flags = (instance.flags & !OWN) | (r.flags & OWN);
        instance.ao_radius = ambient_radius(&world);
        instance._pad = r.edge_color;
        let flags = instance.flags;
        self.translation[i] = translation;
        self.world_bounds[i] = world;
        let bounded = flags & Instance::FLAG_HAS_FACES != 0 && world.is_valid();
        self.set_bounded(row, bounded, &world);
        self.track(row, flags);
        self.write_row(ctx, row);
    }

    /// Grow a row's own box by `bounds`, placed at `place`.
    pub fn grow_local_bounds(&mut self, ctx: &GpuCtx, row: u32, bounds: &AABB, place: &Xform) {
        let i = row as usize;

        let Some(local) = self.local_bounds.get_mut(i) else {
            return;
        };

        local.union_with(bounds);
        let world = local.transformed(place);
        // the grown box outruns the extreme points
        self.set_hull(row, None);
        let world = if world.is_valid() {
            world
        } else {
            AABB::empty()
        };
        self.world_bounds[i] = world;
        self.rows[i].ao_radius = ambient_radius(&world);
        self.write_row(ctx, row);
    }
// --8<-- [end:bury]

// --8<-- [start:queries]
    /// World box of every row that is not dead.
    pub fn live_world_bounds(&self) -> AABB {
        let mut out = AABB::empty();

        for (row, bounds) in self.rows.iter().zip(&self.world_bounds) {
            if row.flags & Instance::FLAG_DEAD == 0 && bounds.is_valid() {
                out.union_with(bounds);
            }
        }

        out
    }

    /// True when row `row` already sits at `place`.
    pub fn placement_matches(&self, row: u32, place: &Xform) -> bool {
        let i = row as usize;
        let (Some(instance), Some(translation)) = (self.rows.get(i), self.translation.get(i))
        else {
            return false;
        };
        let mut model = place.to_f32();
        model[12] = 0.0;
        model[13] = 0.0;
        model[14] = 0.0;
        instance.model == model && *translation == [place.m[12], place.m[13], place.m[14]]
    }
// --8<-- [end:queries]

// --8<-- [start:colors]
    /// Set a row's face or edge color; None restores its own.
    pub fn set_color(&mut self, ctx: &GpuCtx, row: u32, edge: bool, color: Option<[u8; 3]>) {
        if let Some(r) = self.rows.get_mut(row as usize) {
            let flag = if edge {
                Instance::FLAG_EDGE_COLOR
            } else {
                Instance::FLAG_COLOR
            };
            r.flags &= !flag;

            if color.is_some() {
                r.flags |= flag;
            }

            // edge color is packed into the padding word
            if edge {
                r._pad = color
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], 255]))
                    .unwrap_or(0);
            } else {
                r.color = color
                    .map(|c| {
                        [
                            c[0] as f32 / 255.,
                            c[1] as f32 / 255.,
                            c[2] as f32 / 255.,
                            1.,
                        ]
                    })
                    .unwrap_or([1.; 4]);
            }

            self.buffer.write_at(ctx, row, std::slice::from_ref(r));
        }
    }

    /// Set or clear one flag bit on one row; a dead row stays hidden and unselected.
    pub fn set_flag(&mut self, ctx: &GpuCtx, row: u32, bit: u32, on: bool) {
        let Some(r) = self.rows.get_mut(row as usize) else {
            return;
        };
        let revive =
            (bit & Instance::FLAG_HIDDEN != 0 && !on) || (bit & Instance::FLAG_SELECTED != 0 && on);

        if r.flags & Instance::FLAG_DEAD != 0 && revive {
            return;
        }

        let was = r.flags & bit != 0;

        if was == on {
            return;
        }

        r.flags ^= bit;
        let flags = r.flags;

        if bit & Instance::FLAG_HIDDEN != 0 {
            self.geometry_revision = self.geometry_revision.wrapping_add(1);
        }

        self.buffer.write_at(ctx, row, std::slice::from_ref(r));

        if bit & Instance::FLAG_CLOSED != 0 {
            self.track(row, flags);
        }
    }

    /// Note that geometry changed without a row edit.
    pub(crate) fn geometry_changed(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
    }
// --8<-- [end:colors]

// --8<-- [start:forget]
    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.widget = None;
        self.rows.clear();
        self.translation.clear();
        self.local_bounds.clear();
        self.hulls.clear();
        self.bounded.clear();
        self.bounded_at.clear();
        self.world_bounds.clear();
        self.clipping.clear();
        self.closed.clear();
        self.buffer.reset();
        self.translations.reset();
        self.last_origin = None;
    }

    /// Forget every row and free the memory.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.local_bounds.shrink_to_fit();
        self.hulls = Vec::new();
        self.bounded.shrink_to_fit();
        self.bounded_at.shrink_to_fit();
        self.world_bounds.shrink_to_fit();
        self.rows.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }
// --8<-- [end:forget]

// --8<-- [start:lookups]
    /// Row `i` as uploaded.
    pub fn row(&self, i: u32) -> Option<&Instance> {
        self.rows.get(i as usize)
    }

    /// Row count.
    pub fn len(&self) -> u32 {
        self.rows.len() as u32
    }

    /// World box of a row, None when it has none.
    pub fn row_bounds(&self, row: u32) -> Option<AABB> {
        let b = *self.world_bounds.get(row as usize)?;
        b.is_valid().then_some(b)
    }

    /// Set the world box of a text row.
    pub fn set_text_bounds(&mut self, row: u32, bounds: AABB) {
        if let Some(target) = self.world_bounds.get_mut(row as usize) {
            *target = bounds;
        }
    }

    /// Scene origin in f64.
    pub fn anchor(&self) -> [f64; 3] {
        match &self.last_origin {
            Some(point) => [point[0], point[1], point[2]],
            None => [0.0; 3],
        }
    }

    /// Scene origin in f32; zero before the first frame.
    pub fn anchor_f32(&self) -> [f32; 3] {
        match &self.last_origin {
            Some(origin) => [origin[0] as f32, origin[1] as f32, origin[2] as f32],
            None => [0.0; 3],
        }
    }
}
// --8<-- [end:lookups]

// --8<-- [start:tests]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// The SSAO radius scales with the box, not with where it is.
    fn contact_radius_follows_object_size_not_position() {
        let small = AABB::new(0.0, 0.0, 0.0, 50.0, 50.0, 50.0);
        let moved = small.transformed(&Xform::translation(1.0e6, 0.0, 0.0));
        let large = small.transformed(&Xform::scale_xyz(100.0, 100.0, 100.0));
        assert_eq!(ambient_radius(&small), ambient_radius(&moved));
        assert!((ambient_radius(&large) / ambient_radius(&small) - 100.0).abs() < 1e-4);
        assert_eq!(ambient_radius(&AABB::empty()), 0.0);
    }

    /// A translated local box lands at the translated world position.
    #[test]
    fn world_box_translates() {
        let mut r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0), 0);
        r.bounds = AABB::new(0.5, 1.0, 1.5, 0.5, 1.0, 1.5);
        let b = world_box(&r);
        assert_eq!(b.min_point(), Point::new(10.0, 20.0, 30.0));
        assert_eq!(b.max_point(), Point::new(11.0, 22.0, 33.0));
    }

    /// A row with no local box stays empty, translated or not.
    #[test]
    fn world_box_empty_stays_empty() {
        let r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0), 0);
        assert!(!world_box(&r).is_valid());
    }

    /// A tiny move far from zero survives only relative to a near origin.
    #[test]
    fn the_anchor_is_what_keeps_a_small_move() {
        let world = 1.0e6_f64;
        let step = 1.0e-3_f64;
        assert_eq!((world + step) as f32, world as f32);

        let near = Point::new(world - 1.0e4, 0.0, 0.0);
        let before = anchored([world, 0.0, 0.0], &near)[0];
        let after = anchored([world + step, 0.0, 0.0], &near)[0];
        assert_ne!(after, before);
    }

    /// A move changes the translation and box, never the matrix.
    #[test]
    fn a_move_goes_into_the_translation_not_the_matrix() {
        let local = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let place = Xform::translation(10.0, 20.0, 30.0);
        let (model, translation, world) = placed_row(&local, None, &place);

        assert_eq!([model[12], model[13], model[14]], [0.0, 0.0, 0.0]);
        assert_eq!(translation, [10.0, 20.0, 30.0]);
        assert_eq!(world.min_point(), Point::new(9.0, 19.0, 29.0));
        assert_eq!(world.max_point(), Point::new(11.0, 21.0, 31.0));
    }

    /// One row moves, its neighbour stays, the widget row stays last.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn one_row_moves_and_its_neighbour_does_not() {
        use crate::engine::gpu::{Gpu, Upload};
        let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
        let mut upload = Upload::default();

        for x in [0.0, 100.0] {
            let mut row = ObjectRow::new(Xform::translation(x, 0.0, 0.0), 0);
            row.bounds = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
            row.faces = true;
            upload.obj.rows.push(row);
        }

        gpu.set_scene(&upload);
        assert_eq!(gpu.objects.len(), 2);
        let radius = gpu.objects.row(0).unwrap().ao_radius;

        let moved = Xform::translation(0.0, 50.0, 0.0);
        assert!(gpu.objects.set_placement(&gpu.ctx, 0, &moved));
        assert_eq!(gpu.objects.row(0).unwrap().ao_radius, radius);

        let first = gpu.objects.row_bounds(0).expect("row 0 has a box");
        let second = gpu.objects.row_bounds(1).expect("row 1 has a box");
        assert_eq!(first.min_point()[1], 49.0);
        assert_eq!(second.min_point()[0], 99.0);
        assert_eq!(second.min_point()[1], -1.0, "the neighbour did not move");

        gpu.objects
            .set_placement(&gpu.ctx, 0, &Xform::scale_xyz(2.0, 2.0, 2.0));
        assert_eq!(gpu.objects.row(0).unwrap().ao_radius, 2.0 * radius);
        assert_eq!(gpu.objects.row(1).unwrap().ao_radius, radius);

        // widget row comes after every object row
        let (widget, _) = gpu.objects.widget_row(&gpu.ctx, &gpu.layouts);
        assert_eq!(widget, 2);
        assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, widget);

        // a later upload drops and remints the widget row
        let mut more = Upload::default();
        more.obj
            .rows
            .push(ObjectRow::new(Xform::translation(200.0, 0.0, 0.0), 0));
        gpu.set_scene(&more);
        assert_eq!(gpu.objects.len(), 3, "two rows, one file's row, no widget");
        assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, 3);
    }

    /// A row with no box stays empty, never infinite.
    #[test]
    fn a_row_with_no_box_stays_empty() {
        let (_, _, world) = placed_row(&AABB::empty(), None, &Xform::translation(1.0, 0.0, 0.0));
        assert!(!world.is_valid());
    }

    /// A move written to the f32 value is lost at the next rebase.
    #[test]
    fn an_edit_written_past_the_base_does_not_survive_a_rebase() {
        let base = [1.0e4, 0.0, 0.0];
        let first = Point::new(0.0, 0.0, 0.0);
        let step = 0.25_f32;

        // edit the relative value only
        let edited = anchored(base, &first)[0] + step;
        assert_eq!(edited, 1.0e4 + 0.25);

        // the rebase never saw the edit
        let second = Point::new(1.0e3, 0.0, 0.0);
        assert_eq!(anchored(base, &second)[0], 9.0e3);
    }
}
// --8<-- [end:tests]

// --8<-- [start:table-lane]
impl super::lane::Lane for InstanceTable {
    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.reset();
    }

    fn on_release(&mut self, ctx: &GpuCtx, layouts: &Layouts) {
        self.release(ctx, layouts);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), 0)
    }
}
// --8<-- [end:table-lane]

// --8<-- [start:04b-ink-group]
// --8<-- [start:ink-group]
/// Textures and tiles the ink bind group reads.
pub struct InkScene<'a> {
    pub targets: &'a Targets,                            // depth and triangle id textures
}

/// Bind group 2 for ink lanes: rows, depth, triangle ids, tiles.
fn ink_instance_group(
    ctx: &GpuCtx,
    l: &Layouts,
    label: &str,
    buffers: [&wgpu::Buffer; 2],
    depths: [&wgpu::TextureView; 2],
    gradients: [&wgpu::TextureView; 2],
) -> wgpu::BindGroup {
    let view = wgpu::BindingResource::TextureView;
    let entries = [
        buffers[0].as_entire_binding(),
        buffers[1].as_entire_binding(),
        view(depths[0]),
        view(depths[1]),
        view(gradients[0]),
        view(gradients[1]),
    ];
    let entries: Vec<wgpu::BindGroupEntry> = entries
        .into_iter()
        .enumerate()
        .map(|(binding, resource)| wgpu::BindGroupEntry {
            binding: binding as u32,
            resource,
        })
        .collect();
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &l.ink_instance,
        entries: &entries,
    })
}

impl InstanceTable {
    /// Group 2 for ink lanes, with depth textures.
    pub fn ink_group(&self) -> &wgpu::BindGroup {
        self.ink_group
            .as_ref()
            .expect("the ink bind group is built with the GPU")
    }

    /// Rebuild the ink bind group.
    pub fn rebind_ink(&mut self, ctx: &GpuCtx, l: &Layouts, scene: &InkScene) {
        let t = scene.targets;
        self.ink_group = Some(ink_instance_group(
            ctx,
            l,
            "ink.instances.bind_group",
            [&self.buffer.buf, &self.translations.buf],
            [&t.depth_single, &t.depth_msaa],
            [&t.gradient_single, &t.gradient_msaa],
        ));
    }

    /// An ink bind group over the pick pass's own depth textures.
    pub fn pick_group(
        &self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        depths: [&wgpu::TextureView; 2],
        gradients: [&wgpu::TextureView; 2],
    ) -> wgpu::BindGroup {
        ink_instance_group(
            ctx,
            layouts,
            "pick.instances",
            [&self.buffer.buf, &self.translations.buf],
            depths,
            gradients,
        )
    }
}
// --8<-- [end:ink-group]
// --8<-- [end:04b-ink-group]
