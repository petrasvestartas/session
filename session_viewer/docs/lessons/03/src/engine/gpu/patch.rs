// --8<-- [start:lane-ids]
use super::Upload;
use super::lane::{REGISTERED, REGISTRY};

// An edit rewrites one object's rows where they already are; these types say which lane tables it touches and how many rows.
/// One row table of the editable lanes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) enum LaneId {
    Registered(u8), // a lane of lane::REGISTRY
}

/// The fixed lanes, in table order.
const FIXED: &[LaneId] = &[
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
            LaneId::Registered(i) => REGISTRY[i as usize].stride,
        }
    }
}
// --8<-- [end:lane-ids]

// --8<-- [start:counts]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Row counts per lane, for placing one object's rows.
pub(crate) struct Counts {
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
            lanes,
        }
    }

    /// Rows of one lane.
    pub fn get(&self, lane: LaneId) -> u32 {
        match lane {
            LaneId::Registered(i) => self.lanes[i as usize],
        }
    }

    /// Set the rows of one lane.
    pub fn set(&mut self, lane: LaneId, value: u32) {
        let slot = match lane {
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
