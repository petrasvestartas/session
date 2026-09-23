use super::lane::{REGISTERED, REGISTRY};
use super::Upload;

/// One row table of the editable lanes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) enum LaneId {
    Verts,          // mesh vertices and their object rows
    Faces,          // solid face indices
    Print,          // sheet fill indices
    Text,           // sheet lettering indices
    Sources,        // source faces
    Pipes,          // edge segments
    Ribbons,        // line segments
    Spheres,        // vertex markers
    Dots,           // flat dots
    Registered(u8), // a lane of lane::REGISTRY
}

/// The fixed lanes, in table order.
const FIXED: [LaneId; 9] = [
    LaneId::Verts,
    LaneId::Faces,
    LaneId::Print,
    LaneId::Text,
    LaneId::Sources,
    LaneId::Pipes,
    LaneId::Ribbons,
    LaneId::Spheres,
    LaneId::Dots,
];

impl LaneId {
    /// Every lane, fixed ones first.
    pub fn all() -> impl Iterator<Item = LaneId> {
        FIXED
            .into_iter()
            .chain((0..REGISTERED).map(|i| LaneId::Registered(i as u8)))
    }

    /// Bytes one row holds, on the GPU and in the CPU mirrors.
    pub fn stride(self) -> u64 {
        match self {
            LaneId::Verts => 44, // vertex plus its object row
            LaneId::Faces => 5,  // index plus a third of a face id
            LaneId::Print | LaneId::Text => 4,
            LaneId::Sources => 16,
            LaneId::Pipes => 60, // stroke, source id and edge source
            LaneId::Ribbons => 52,
            LaneId::Spheres | LaneId::Dots => 48,
            LaneId::Registered(i) => REGISTRY[i as usize].stride,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Row counts per lane, for placing one object's rows.
pub(crate) struct Counts {
    pub verts: u32,               // mesh vertices
    pub faces: u32,               // solid face indices
    pub print: u32,               // sheet fill indices
    pub text: u32,                // sheet lettering indices
    pub sources: u32,             // source faces
    pub pipes: u32,               // line segments drawn as pipes
    pub ribbons: u32,             // line segments drawn flat
    pub spheres: u32,             // vertex markers
    pub dots: u32,                // flat dots
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
            verts: up.arena.verts.len() as u32,
            faces: up.arena.idx.len() as u32,
            print: up.arena.idx_print.len() as u32,
            text: up.arena.idx_text.len() as u32,
            sources: up.arena.face_sources.len() as u32,
            pipes: up.seg.pipes.len() as u32,
            ribbons: up.seg.ribbons.len() as u32,
            spheres: up.glyph.spheres.len() as u32,
            dots: up.glyph.dots.len() as u32,
            lanes,
        }
    }

    /// Rows of one lane.
    pub fn get(&self, lane: LaneId) -> u32 {
        match lane {
            LaneId::Verts => self.verts,
            LaneId::Faces => self.faces,
            LaneId::Print => self.print,
            LaneId::Text => self.text,
            LaneId::Sources => self.sources,
            LaneId::Pipes => self.pipes,
            LaneId::Ribbons => self.ribbons,
            LaneId::Spheres => self.spheres,
            LaneId::Dots => self.dots,
            LaneId::Registered(i) => self.lanes[i as usize],
        }
    }

    /// Set the rows of one lane.
    pub fn set(&mut self, lane: LaneId, value: u32) {
        let slot = match lane {
            LaneId::Verts => &mut self.verts,
            LaneId::Faces => &mut self.faces,
            LaneId::Print => &mut self.print,
            LaneId::Text => &mut self.text,
            LaneId::Sources => &mut self.sources,
            LaneId::Pipes => &mut self.pipes,
            LaneId::Ribbons => &mut self.ribbons,
            LaneId::Spheres => &mut self.spheres,
            LaneId::Dots => &mut self.dots,
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

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Where one object's rows sit: first row and row count per lane.
pub(crate) struct Span {
    pub start: Counts, // first row per lane
    pub count: Counts, // rows per lane
}
