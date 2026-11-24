use super::upload::drop_rows;
use serde::{Deserialize, Serialize};
use session_rust::{AABB, RenderVertex};
use std::any::Any;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

/// What the pixel under the cursor holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub row: u32, // object row
    pub sub: u32, // 0 = the object; else a tagged edge, face, dot or point id
}

/// What a pick may answer with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PickMode {
    #[default]
    Object, // whole objects
    Edge,      // edges
    Component, // edges first, else the visible face
    Controls {
        // control dots of one object
        parent: u32, // the object row
        cloud: bool, // true = pick cloud points instead
    },
}

/// Bit that marks a pick id as a face.
pub const FACE_TAG: u32 = 0x2000_0000;

/// Which object and face a triangle came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FaceSource {
    pub parent: u32, // object row
    pub face: usize, // face index in that object
}

/// Where one GPU vertex came from on its surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample {
    pub index: u32,   // GPU vertex index
    pub surface: u32, // surface index in the BRep
    pub uv: [f64; 2], // parameter on that surface
    pub sign: f32,    // +1 or -1 on the normal
}

/// Mesh rows of one upload, ready for the GPU.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,                 // one vertex per row
    pub vids: Vec<u32>,                           // object row of each vertex
    pub idx: Vec<u32>,                            // triangle indices of solid faces
    pub idx_print: Vec<u32>,                      // triangle indices of sheet fills
    pub idx_text: Vec<u32>,                       // triangle indices of sheet lettering
    pub face_ids: Vec<u32>,                       // source face of each solid triangle
    pub face_sources: Vec<FaceSource>,            // where each face came from
    pub surface_boundaries: Vec<(u32, [u32; 2])>, // pipe and sample range per surface edge
    pub surface_samples: Vec<Sample>,             // surface points for previews
}

impl ArenaRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_sources);
        drop_rows(&mut self.surface_samples);
        drop_rows(&mut self.surface_boundaries);
    }
}

/// One line segment, 40 bytes, as the shaders read it.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    pub p0: [f32; 3],     // start point
    pub radius: f32,      // 0 = pen width; > 0 = world mm
    pub p1: [f32; 3],     // end point
    pub instance_id: u32, // object row
    pub color: u32,       // packed rgba, red in the low byte
    pub facing: u32,      // packed normals of the two faces beside it
}

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// The sag of a pipe drawn only where it is a silhouette, where the two faces beside it face
/// opposite ways to the camera: the sign bit set on a zero slack; ribbon.wgsl reads the bit.
pub const SILHOUETTE_SAG: f32 = -0.0;

/// True for the sag of a pipe drawn only where it is a silhouette.
pub fn is_silhouette(sag: f32) -> bool {
    sag.is_sign_negative()
}

/// One batch of segments added to a sheet.
pub struct SegDraw {
    pub instance: u32, // object row of the sheet
    pub from: u32,     // first segment index within the sheet
    pub count: u32,    // segments in this batch
    pub first: u32,    // row of the first segment in the upload's sheet rows
}

/// Segment rows of one upload.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>,      // mesh and solid edges
    pub pipe_ids: Vec<u32>,               // source edge per pipe, or u32::MAX
    pub pipe_sags: Vec<f32>, // per pipe: how far it rises off the faces beside it, mm; 0 = on them
    pub pipe_chains: Vec<Range<u32>>, // runs of pipes that form one curve
    pub ribbon_chains: Vec<Range<u32>>, // runs of ribbons that form one curve
    pub ribbons: Vec<CylinderSegment>, // standalone lines and curves
    pub ribbon_ids: Vec<u32>, // source entity per ribbon, or u32::MAX
    pub ribbon_dashes: Vec<[f32; 3]>, // on length, off length, cumulative length before the segment
    pub ribbon_trims: Vec<[f32; 2]>, // visible point radii at the outer ends
    pub ribbon_heads: Vec<(u32, u32)>, // (ribbon, HEAD_END or HEAD_START): that end stops under an arrowhead
    pub sheet_rows: Vec<CylinderSegment>, // streamed sheet segments, kept apart from editable lines
    pub sheet_ids: Vec<u32>,           // source entity per sheet segment, or u32::MAX
    pub sheets: Vec<SegDraw>,          // sheet batches among the sheet rows
}

impl SegRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.pipe_ids);
        drop_rows(&mut self.pipe_sags);
        drop_rows(&mut self.pipe_chains);
        drop_rows(&mut self.ribbon_chains);
        drop_rows(&mut self.ribbons);
        drop_rows(&mut self.ribbon_ids);
        drop_rows(&mut self.ribbon_dashes);
        drop_rows(&mut self.ribbon_trims);
        drop_rows(&mut self.ribbon_heads);
        drop_rows(&mut self.sheet_rows);
        drop_rows(&mut self.sheet_ids);
        drop_rows(&mut self.sheets);
    }

    /// Move `other`'s segments after these.
    pub fn merge(&mut self, other: &mut SegRows) {
        let seg = self;
        let pipes = seg.pipes.len() as u32;
        let ribbons = seg.ribbons.len() as u32;
        let sheet_rows = seg.sheet_rows.len() as u32;
        seg.pipe_ids.resize(seg.pipes.len(), u32::MAX);
        seg.pipe_sags.resize(seg.pipes.len(), 0.0);
        seg.ribbon_ids.resize(seg.ribbons.len(), u32::MAX);
        seg.ribbon_dashes.resize(seg.ribbons.len(), [0.0; 3]);
        seg.ribbon_trims.resize(seg.ribbons.len(), [0.0; 2]);
        seg.sheet_ids.resize(seg.sheet_rows.len(), u32::MAX);
        other.pipe_ids.resize(other.pipes.len(), u32::MAX);
        other.pipe_sags.resize(other.pipes.len(), 0.0);
        other.ribbon_ids.resize(other.ribbons.len(), u32::MAX);
        other.ribbon_dashes.resize(other.ribbons.len(), [0.0; 3]);
        other.ribbon_trims.resize(other.ribbons.len(), [0.0; 2]);
        other.sheet_ids.resize(other.sheet_rows.len(), u32::MAX);
        seg.pipe_chains.extend(
            other
                .pipe_chains
                .iter()
                .map(|chain| chain.start + pipes..chain.end + pipes),
        );
        seg.ribbon_chains.extend(
            other
                .ribbon_chains
                .iter()
                .map(|chain| chain.start + ribbons..chain.end + ribbons),
        );
        seg.ribbon_heads.extend(
            other
                .ribbon_heads
                .iter()
                .map(|&(row, end)| (row + ribbons, end)),
        );

        for mut draw in other.sheets.drain(..) {
            draw.first += sheet_rows;
            seg.sheets.push(draw);
        }

        seg.pipes.append(&mut other.pipes);
        seg.pipe_ids.append(&mut other.pipe_ids);
        seg.pipe_sags.append(&mut other.pipe_sags);
        seg.ribbons.append(&mut other.ribbons);
        seg.ribbon_ids.append(&mut other.ribbon_ids);
        seg.ribbon_dashes.append(&mut other.ribbon_dashes);
        seg.ribbon_trims.append(&mut other.ribbon_trims);
        seg.sheet_rows.append(&mut other.sheet_rows);
        seg.sheet_ids.append(&mut other.sheet_ids);
    }
}

/// One marker or dot, 48 bytes, as the shaders read it.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],     // world position
    pub radius: f32,          // 0 = pen width; > 0 world mm; < 0 screen px
    pub color: [f32; 4],      // rgba
    pub instance_id: u32,     // object row
    pub facing: u32,          // packed normals of the faces around it
    pub facing_ext: [u32; 2], // more packed normals
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// Marker and dot rows of one upload.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>, // vertex markers, shaded
    pub dots: Vec<GlyphPoint>,    // flat dots
}

impl GlyphRows {
    /// Empty both tables and free their memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
    }
}

/// Marker for a cloud that has no normals.
pub const NO_NORMALS: u32 = u32::MAX;

/// One batch of points added to a cloud.
pub struct CloudDraw {
    pub instance: u32,   // object row of the cloud
    pub from: u32,       // first point index within the cloud
    pub count: u32,      // points in this batch
    pub first: u32,      // row of the first point in the upload
    pub spacing: f32,    // typical distance between points
    pub node_first: u32, // first octree node in the upload
    pub node_count: u32, // octree nodes; 0 = no octree
    pub nrm_first: u32,  // first normal row, or NO_NORMALS
}

/// One box of the cloud's octree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodNode {
    pub center: [f32; 3],   // box center
    pub size: f32,          // box edge length
    pub spacing: f32,       // point spacing inside the box
    pub first: u32,         // first point of the box, within the cloud
    pub count: u32,         // points in the box
    pub children: [i32; 8], // child node indices, -1 = none
}

/// Point rows of one upload.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>,         // x, y, z per point
    pub col: Vec<u32>,         // packed color per point
    pub nrm: Vec<u32>,         // packed normal per point
    pub draws: Vec<CloudDraw>, // point batches in this upload
    pub nodes: Vec<LodNode>,   // octree nodes in this upload
    pub expect: u32,           // points known to follow this upload
    pub expect_normals: u32,   // normals known to follow this upload
}

impl CloudRows {
    /// Points in this upload so far.
    pub fn point_count(&self) -> u32 {
        (self.pos.len() / 3) as u32
    }

    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pos);
        drop_rows(&mut self.col);
        drop_rows(&mut self.nrm);
        drop_rows(&mut self.draws);
        drop_rows(&mut self.nodes);
    }

    /// Move `other`'s cloud points and octree nodes after these.
    pub fn merge(&mut self, other: &mut CloudRows) {
        let cloud = self;
        let points = cloud.point_count();
        let nodes = cloud.nodes.len() as u32;
        let normals = cloud.nrm.len() as u32;

        for mut draw in other.draws.drain(..) {
            draw.first += points;
            draw.node_first += nodes;

            if draw.nrm_first != NO_NORMALS {
                draw.nrm_first += normals;
            }

            cloud.draws.push(draw);
        }

        cloud.pos.append(&mut other.pos);
        cloud.col.append(&mut other.col);
        cloud.nrm.append(&mut other.nrm);
        cloud.nodes.append(&mut other.nodes);
    }
}

/// One vector as the shader reads it: a head's tip lands on its point, a bare end is cut flat there.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VectorRow {
    // offset size
    pub start: [f32; 3],    //   0   12  start point, object space
    pub radius: f32,        //  12    4  0 = pen; > 0 world mm; < 0 pen multiplier
    pub end: [f32; 3],      //  16   12  end point, object space
    pub instance_id: u32,   //  28    4  object row
    pub color: u32,         //  32    4  packed rgba, red in the low byte
    pub head: f32,          //  36    4  arrowhead length in pens; 0 = default
    pub heads: u32,         //  40    4  HEAD_END | HEAD_START | HEAD_ONLY
    pub pad: u32,           //  44    4
    pub start_trim: f32,    //  48    4  visible point radius at the start
    pub end_trim: f32,      //  52    4  visible point radius at the end
    pub reserved: [u32; 2], //  56    8  -> 64, a multiple of 16
}

impl VectorRow {
    pub const HEAD_END: u32 = 1; // a head whose tip is `end`
    pub const HEAD_START: u32 = 2; // a head whose tip is `start`
    pub const HEAD_ONLY: u32 = 4; // no shaft: `start` only aims the heads, e.g. a curve's end tangent
}

const _: () = assert!(std::mem::size_of::<VectorRow>() == 64);

/// Vector rows of one upload; producers write them with `up.lanes.get_mut::<VectorRows>()`.
#[derive(Default)]
pub struct VectorRows {
    pub rows: Vec<VectorRow>, // one per vector
}

/// Half side of every plane's grid and length of its arrows, scene units, as f64 bits; `View Plane Size` sets it.
static PLANE_SIZE: AtomicU64 = AtomicU64::new(100.0_f64.to_bits());

/// Half side of every plane's grid, scene units.
pub fn plane_size() -> f64 {
    f64::from_bits(PLANE_SIZE.load(Ordering::Relaxed))
}

/// Set the half side of every plane's grid; the next frame draws it, no walk needed.
pub fn set_plane_size(size: f64) {
    PLANE_SIZE.store(size.to_bits(), Ordering::Relaxed);
}

/// One plane as the shader reads it: its frame; the grid and the arrows come from the vertex index.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlaneRow {
    // offset size
    pub origin: [f32; 3], //   0   12  object space
    pub instance_id: u32, //  12    4  object row
    pub x: [f32; 3],      //  16   12  unit x axis
    pub radius: f32,      //  28    4  pen: 0 = default; > 0 world mm; < 0 px
    pub y: [f32; 3],      //  32   12  unit y axis
    pub pad: u32,         //  44    4  -> 48, a multiple of 16
}

const _: () = assert!(std::mem::size_of::<PlaneRow>() == 48);

/// Plane rows of one upload; the walk writes them with `up.lanes.get_mut::<PlaneRows>()`.
#[derive(Default)]
pub struct PlaneRows {
    pub rows: Vec<PlaneRow>, // one per plane
}

/// Rows for registered lanes, one table per row type, so `Upload` needs no field per lane.
#[derive(Default)]
pub struct LaneRows {
    tables: Vec<Box<dyn Any>>, // one table per type
}

impl LaneRows {
    /// The table of type `T`, created empty on first use.
    pub fn get_mut<T: Default + 'static>(&mut self) -> &mut T {
        let at = match self.tables.iter().position(|t| t.is::<T>()) {
            Some(at) => at,
            None => {
                self.tables.push(Box::new(T::default()));
                self.tables.len() - 1
            }
        };
        self.tables[at]
            .downcast_mut::<T>()
            .expect("table has type T")
    }

    /// The table of type `T`; None when nothing was written.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.tables.iter().find_map(|t| t.downcast_ref::<T>())
    }

    /// Take the table of type `T` out; None when nothing was written.
    pub fn take<T: 'static>(&mut self) -> Option<T> {
        let at = self.tables.iter().position(|t| t.is::<T>())?;
        self.tables
            .remove(at)
            .downcast::<T>()
            .ok()
            .map(|table| *table)
    }

    /// Drop every table and free its memory.
    pub fn clear(&mut self) {
        self.tables = Vec::new();
    }
}

/// One clipping plane in world space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClipPlane {
    pub normal: [f64; 3], // unit normal toward the kept side
    pub offset: f64,      // kept where normal · p + offset >= 0
    pub origin: [f64; 3], // a point on the plane, where the hatch starts
    pub hatch: [f64; 3],  // unit direction across the hatch lines, in the plane
}

impl ClipPlane {
    /// Signed distance of world point `p`; below zero is cut away.
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        self.normal[0] * p[0] + self.normal[1] * p[1] + self.normal[2] * p[2] + self.offset
    }

    /// True when the plane passes through the box, or so near it, with the scene origin at
    /// `anchor`, that f32 rounding on the GPU may cut a face lying on the plane.
    pub fn crosses(&self, b: &AABB, anchor: [f64; 3]) -> bool {
        let reach =
            self.normal[0].abs() * b.hx + self.normal[1].abs() * b.hy + self.normal[2].abs() * b.hz;
        let center = [b.cx, b.cy, b.cz];
        let size = b.hx + b.hy + b.hz;
        let local: f64 = (0..3).map(|i| (center[i] - anchor[i]).abs()).sum();
        let world: f64 = center.iter().map(|v| v.abs()).sum();
        // 32 times CLIP_SLACK in clip.wgsl, plus the f32 rounding of the vertices themselves
        let slack =
            (local + size + self.distance(anchor).abs()) / 4096.0 + (world + size) / 1048576.0;
        self.distance(center).abs() <= reach + slack
    }
}

/// Most clipping planes cutting at once.
pub const MAX_PLANES: usize = 6;

/// One definition drawn once per instance: its shared rows and the slots of its instances.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotDraw {
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
    pub slots: Range<u32>,   // instance slots, never slot 0
}

/// Shared definition geometry and the instance rows that reference it.
pub struct Definition<'a> {
    pub key: u32,            // names the definition for as long as it lives
    pub rows: &'a [u32],     // instance rows; a member keeps its position until it leaves
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
}

// --8<-- [start:scene-text-placement]
/// Where a label sits and how it is sized.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TextPlacement {
    Screen {
        // fixed on screen, CSS px
        left: f32, // left edge
        top: f32,  // top edge
    },
    Anchor {
        // at a world point, screen-sized, hidden behind geometry
        world: [f64; 3],  // world point
        offset: [f32; 2], // shift from it, CSS px
    },
    Nameplate {
        // centered on a world point with a plate, always on top
        // world point
        world: [f64; 3],
        padding: [f32; 2], // space around the text, CSS px
        rounded: bool,     // rounded plate corners
    },
    WorldPlane {
        // lying on a plane in the world
        // top-left corner
        world: [f64; 3],
        right: [f64; 3],   // unit axis along the text
        up: [f64; 3],      // unit axis up the text
        world_height: f64, // em height, world units
        #[serde(default = "plated")]
        plate: bool, // a black plate behind the glyphs
    },
    WorldBillboard {
        // facing the camera, world-sized
        // world point
        world: [f64; 3],
        world_height: f64, // em height, world units
    },
}

// --8<-- [end:scene-text-placement]
/// World-plane texts have a plate unless they say otherwise.
fn plated() -> bool {
    true
}

/// The object a label belongs to, for picks and selection.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextObject {
    pub row: u32,       // object row
    pub selected: bool, // drawn as selected
}

/// One text label; sizes in CSS pixels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextLabel {
    pub id: u32,                    // unique per label
    pub object: Option<TextObject>, // owning object, if any
    pub text: String,               // the text
    pub font_size: f32,             // em size
    pub line_height: f32,           // distance between lines
    pub color: [u8; 4],             // rgba
    pub placement: TextPlacement,   // where it sits
    pub clip: Option<[f32; 4]>,     // screen box to cut it to: left, top, right, bottom
}

impl TextLabel {
    /// Ink color: black when selected, else the label's own.
    pub fn ink_color(&self) -> [u8; 4] {
        if self.object.is_some_and(|object| object.selected) {
            [0, 0, 0, 255]
        } else {
            self.color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_rows_by_type() {
        let mut rows = LaneRows::default();
        assert!(rows.get::<Vec<u32>>().is_none());
        rows.get_mut::<Vec<u32>>().push(7);
        rows.get_mut::<Vec<u32>>().push(8);
        rows.get_mut::<Vec<f32>>().push(1.0);
        assert_eq!(rows.get::<Vec<u32>>(), Some(&vec![7, 8]));
        assert_eq!(rows.get::<Vec<f32>>().map(Vec::len), Some(1));
        rows.clear();
        assert!(rows.get::<Vec<u32>>().is_none());
    }
}
