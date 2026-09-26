// --8<-- [start:lane-ids]
use super::Upload;
use super::lane::{REGISTERED, REGISTRY};

// An edit rewrites one object's rows where they already are; these types say which lane tables it touches and how many rows.
/// One row table of the editable lanes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) enum LaneId {
    Verts,          // mesh vertices and their object rows; register:meshes
    Faces,          // solid face indices; register:meshes
    Print,          // sheet fill indices; register:meshes
    Text,           // sheet lettering indices; register:meshes
    Sources,        // source faces; register:meshes
    Pipes,          // edge segments; register:strokes
    Ribbons,        // line segments; register:strokes
    Spheres,        // vertex markers; register:markers
    Dots,           // flat dots; register:markers
    Registered(u8), // a lane of lane::REGISTRY
}

/// The fixed lanes, in table order.
const FIXED: &[LaneId] = &[
    LaneId::Verts,   // register:meshes
    LaneId::Faces,   // register:meshes
    LaneId::Print,   // register:meshes
    LaneId::Text,    // register:meshes
    LaneId::Sources, // register:meshes
    LaneId::Pipes,   // register:strokes
    LaneId::Ribbons, // register:strokes
    LaneId::Spheres, // register:markers
    LaneId::Dots,    // register:markers
];

impl LaneId {
    /// Every lane, fixed ones first.
    pub fn all() -> impl Iterator<Item = LaneId> {
        FIXED
            .iter()
            .copied()
            .chain((0..REGISTERED).map(|i| LaneId::Registered(i as u8)))
    }

    /// Bytes one row holds, on the GPU and in the CPU mirrors.
    pub fn stride(self) -> u64 {
        match self {
            LaneId::Verts => 44,               // vertex plus its object row; register:meshes
            LaneId::Faces => 5,                // index plus a third of a face id; register:meshes
            LaneId::Print | LaneId::Text => 4, // register:meshes
            LaneId::Sources => 16,             // register:meshes
            LaneId::Pipes => 60, // stroke, source id and edge source; register:strokes
            LaneId::Ribbons => 52, // register:strokes
            LaneId::Spheres | LaneId::Dots => 48, // register:markers
            LaneId::Registered(i) => REGISTRY[i as usize].stride,
        }
    }
}
// --8<-- [end:lane-ids]

// --8<-- [start:counts]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Row counts per lane, for placing one object's rows.
pub(crate) struct Counts {
    pub verts: u32,               // mesh vertices; register:meshes
    pub faces: u32,               // solid face indices; register:meshes
    pub print: u32,               // sheet fill indices; register:meshes
    pub text: u32,                // sheet lettering indices; register:meshes
    pub sources: u32,             // source faces; register:meshes
    pub pipes: u32,               // line segments drawn as pipes; register:strokes
    pub ribbons: u32,             // line segments drawn flat; register:strokes
    pub spheres: u32,             // vertex markers; register:markers
    pub dots: u32,                // flat dots; register:markers
    pub lanes: [u32; REGISTERED], // rows of each registered lane
}

impl Counts {
    /// Counts of one upload.
    pub fn of(up: &Upload) -> Self {
        let mut lanes = [0; REGISTERED];

        for (count, lane) in lanes.iter_mut().zip(REGISTRY) {
            *count = (lane.rows_in)(up);
        }

        Self {
            verts: up.arena.verts.len() as u32,          // register:meshes
            faces: up.arena.idx.len() as u32,            // register:meshes
            print: up.arena.idx_print.len() as u32,      // register:meshes
            text: up.arena.idx_text.len() as u32,        // register:meshes
            sources: up.arena.face_sources.len() as u32, // register:meshes
            pipes: up.seg.pipes.len() as u32,            // register:strokes
            ribbons: up.seg.ribbons.len() as u32,        // register:strokes
            spheres: up.glyph.spheres.len() as u32,      // register:markers
            dots: up.glyph.dots.len() as u32,            // register:markers
            lanes,
        }
    }

    /// Rows of one lane.
    pub fn get(&self, lane: LaneId) -> u32 {
        match lane {
            LaneId::Verts => self.verts,     // register:meshes
            LaneId::Faces => self.faces,     // register:meshes
            LaneId::Print => self.print,     // register:meshes
            LaneId::Text => self.text,       // register:meshes
            LaneId::Sources => self.sources, // register:meshes
            LaneId::Pipes => self.pipes,     // register:strokes
            LaneId::Ribbons => self.ribbons, // register:strokes
            LaneId::Spheres => self.spheres, // register:markers
            LaneId::Dots => self.dots,       // register:markers
            LaneId::Registered(i) => self.lanes[i as usize],
        }
    }

    /// Set the rows of one lane.
    pub fn set(&mut self, lane: LaneId, value: u32) {
        let slot = match lane {
            LaneId::Verts => &mut self.verts,     // register:meshes
            LaneId::Faces => &mut self.faces,     // register:meshes
            LaneId::Print => &mut self.print,     // register:meshes
            LaneId::Text => &mut self.text,       // register:meshes
            LaneId::Sources => &mut self.sources, // register:meshes
            LaneId::Pipes => &mut self.pipes,     // register:strokes
            LaneId::Ribbons => &mut self.ribbons, // register:strokes
            LaneId::Spheres => &mut self.spheres, // register:markers
            LaneId::Dots => &mut self.dots,       // register:markers
            LaneId::Registered(i) => &mut self.lanes[i as usize],
        };
        *slot = value;
    }

    /// Every lane with its rows.
    pub fn each(self) -> impl Iterator<Item = (LaneId, u32)> {
        LaneId::all().map(move |lane| (lane, self.get(lane)))
    }

    /// One lane at a time, `f` of both.
    fn zip(self, other: Self, f: impl Fn(u32, u32) -> u32) -> Self {
        let mut out = Self::default();

        for lane in LaneId::all() {
            out.set(lane, f(self.get(lane), other.get(lane)));
        }

        out
    }

    /// Add two counts.
    pub fn plus(self, other: Self) -> Self {
        self.zip(other, |a, b| a + b)
    }

    /// Subtract two counts.
    pub fn minus(self, other: Self) -> Self {
        self.zip(other, |a, b| a - b)
    }

    /// True when every lane fits in `cap`.
    pub fn fits(&self, cap: &Self) -> bool {
        LaneId::all().all(|lane| self.get(lane) <= cap.get(lane))
    }

    /// True when no lane has a row.
    pub fn is_empty(&self) -> bool {
        LaneId::all().all(|lane| self.get(lane) == 0)
    }

    /// Bytes these rows hold.
    pub fn bytes(&self) -> u64 {
        self.each()
            .map(|(lane, rows)| u64::from(rows) * lane.stride())
            .sum()
    }
}
// --8<-- [end:counts]

// --8<-- [start:span]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Where one object's rows sit: first row and row count per lane.
pub(crate) struct Span {
    pub start: Counts, // first row per lane
    pub count: Counts, // rows per lane
}
// --8<-- [end:span]
