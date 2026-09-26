use crate::engine::gpu::patch::{Counts, LaneId, Span};
use crate::engine::gpu::{ObjectRow, Upload};
use session_rust::{TreeNode, Xform, history};
use std::cell::RefCell;
use std::rc::Weak;

/// Owner of a text row.
pub(crate) const TEXT: usize = usize::MAX;

/// Owner of a row id nothing holds.
pub(crate) const FREE: usize = usize::MAX - 1;

/// Owner of the hidden row that dead lane rows point at.
pub(crate) const SINK: usize = usize::MAX - 2;

/// Owner of a deleted object's row whose lane rows stay resident, hidden, for an undo.
pub(crate) const TOMB: usize = usize::MAX - 3;

/// The note bits: what one edit did to one object.
pub(crate) const GEOMETRY: u8 = 1; // walk it again
pub(crate) const PLACE: u8 = 2; // compute its placement again
pub(crate) const PRESENCE: u8 = 4; // decide again whether it is drawn
pub(crate) const SUBTREE: u8 = 8; // every object below its tree node too

/// Where one object's rows sit; most objects use one lane.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) enum Footprint {
    #[default]
    None, // no lane rows: text, shell, sink, free or an empty object
    Cloud, // a document cloud: one CloudLane entry
    One {
        lane: LaneId,
        start: u32,
        count: u32,
    }, // point, line, polyline, curve, plane, box
    Many(u32), // index into `Spans`: mesh, BRep, surface, element
}

/// An allocation bigger than its content: after an in-place shrink, or with preview headroom.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cap {
    pub alloc: Counts, // rows owned per lane
    pub preview: bool, // made for a drag preview
}

/// Per-document viewer state, parallel to `Scene::docs`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DocState {
    pub sheet: Option<[f64; 2]>, // z band when the file was judged a flat sheet; never judged again
    pub nodes_from: usize, // `sync::tree_key` of the tree the node cache was filled from; 0 = refill
}

/// What one edit did to one object.
#[derive(Clone, Debug)]
pub(crate) struct Note {
    pub guid: std::rc::Rc<str>,                // the object or group
    pub what: u8,                              // GEOMETRY | PLACE | PRESENCE | SUBTREE
    pub node: Option<Weak<RefCell<TreeNode>>>, // its tree node, when the edit knows it
    pub parent: Option<(String, usize)>,       // (parent name, child index) of an added object
    pub tomb: Option<Weak<history::Tomb>>,     // the kernel tomb of an add or remove
}

impl Note {
    /// A note without a node.
    pub fn new(guid: &str, what: u8) -> Self {
        Self {
            guid: guid.into(),
            what,
            node: None,
            parent: None,
            tomb: None,
        }
    }
}

/// A deleted object's rows, left on the GPU and hidden while history can bring it back.
pub(crate) struct Tomb {
    pub row: u32,                    // its row id, kept from reuse
    pub foot: Footprint,             // its lane rows, untouched
    pub record: Weak<history::Tomb>, // the kernel tomb; gone when no undo reaches it
    pub born: u64,                   // burial order, oldest released first
    pub place: Option<Xform>,        // its placement in a sheet document, whose pens hang on it
    pub attributes: bool,            // whether element features were drawn
    pub points: u64,                 // cloud points it keeps on the GPU, 0 for other kinds
}

/// GPU work of one sync, O(changed); `Scene::upload_to` drains it in this order.
#[derive(Default)]
pub(crate) struct Staged {
    pub retire: Vec<u32>,                    // object rows to hide for good
    pub bury: Vec<u32>,                      // object rows hidden with their lane rows kept
    pub unbury: Vec<(u32, ObjectRow)>,       // buried rows shown again
    pub clouds: Vec<u32>,                    // object rows whose cloud entry is dropped
    pub kills: Vec<(LaneId, u32, u32)>,      // (lane, first, count) handed to the sink
    pub tails: Vec<(LaneId, u32, u32, u32)>, // (face, print or text lane, first, count, vertex) emptied
    pub patches: Vec<(Counts, Upload)>,      // one object's rows, written at these starts
    pub rows: Vec<(u32, ObjectRow)>,         // reused ids: the whole object row
    pub geometry: Vec<(u32, ObjectRow)>,     // redrawn rows: box, spacing and walk flags
    pub places: Vec<(u32, Xform)>,           // moved rows
}

/// The side table of footprints that span several lanes; slots are reused.
#[derive(Default)]
pub(crate) struct Spans {
    spans: Vec<Span>, // one per multi-lane footprint
    free: Vec<u32>,   // slots nothing holds
}

impl Spans {
    /// The footprint of `span`: nothing, one lane inline, or a slot here.
    pub fn foot(&mut self, span: Span) -> Footprint {
        let mut lanes = span.count.each().filter(|(_, rows)| *rows > 0);

        let Some((lane, count)) = lanes.next() else {
            return Footprint::None;
        };

        if lanes.next().is_none() {
            return Footprint::One {
                lane,
                start: span.start.get(lane),
                count,
            };
        }

        self.keep(span)
    }

    /// The footprint of `span` in a slot of its own, so every lane keeps its start.
    pub fn keep(&mut self, span: Span) -> Footprint {
        match self.free.pop() {
            Some(slot) => {
                self.spans[slot as usize] = span;
                Footprint::Many(slot)
            }
            None => {
                self.spans.push(span);
                Footprint::Many(self.spans.len() as u32 - 1)
            }
        }
    }

    /// The rows a footprint covers; empty for none and for a cloud.
    pub fn span(&self, foot: Footprint) -> Span {
        match foot {
            Footprint::None | Footprint::Cloud => Span::default(),
            Footprint::One { lane, start, count } => {
                let mut span = Span::default();
                span.start.set(lane, start);
                span.count.set(lane, count);
                span
            }
            Footprint::Many(slot) => self.spans[slot as usize],
        }
    }

    /// Give a footprint's slot back.
    pub fn release(&mut self, foot: Footprint) {
        if let Footprint::Many(slot) = foot {
            self.free.push(slot);
        }
    }

    /// Forget every slot.
    pub fn clear(&mut self) {
        self.spans = Vec::new();
        self.free = Vec::new();
    }

    /// Memory held.
    #[cfg(target_arch = "wasm32")]
    pub fn bytes(&self) -> usize {
        self.spans.capacity() * std::mem::size_of::<Span>() + self.free.capacity() * 4
    }
}

/// Row ids nothing holds, handed out last-freed first; an id freed in one sync waits for its end.
#[derive(Default)]
pub(crate) struct Ids {
    free: Vec<u32>,  // ready to reuse
    freed: Vec<u32>, // freed in the sync running now
}

impl Ids {
    /// An id to reuse.
    pub fn take(&mut self) -> Option<u32> {
        self.free.pop()
    }

    /// Give an id back; it is reused after this sync.
    pub fn give(&mut self, row: u32) {
        self.freed.push(row);
    }

    /// End of a sync: the ids freed in it become reusable.
    pub fn settle(&mut self) {
        self.free.append(&mut self.freed);
    }

    /// Ids nothing holds.
    pub fn len(&self) -> usize {
        self.free.len() + self.freed.len()
    }

    /// Forget every id.
    pub fn clear(&mut self) {
        self.free = Vec::new();
        self.freed = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::patch::Counts;

    /// A footprint is 12 bytes; one lane stays inline, several take a slot.
    #[test]
    fn footprint_is_12_bytes_and_counts_roundtrip() {
        assert_eq!(std::mem::size_of::<Footprint>(), 12);
        let mut spans = Spans::default();
        let mut one = Span::default();
        one.start.set(LaneId::Ribbons, 40);
        one.count.set(LaneId::Ribbons, 3);
        let foot = spans.foot(one);
        assert_eq!(
            foot,
            Footprint::One {
                lane: LaneId::Ribbons,
                start: 40,
                count: 3
            }
        );
        assert_eq!(spans.span(foot), one);
        let mut many = one;
        many.start.set(LaneId::Verts, 7);
        many.count.set(LaneId::Verts, 12);
        let foot = spans.foot(many);
        assert_eq!(foot, Footprint::Many(0));
        assert_eq!(spans.span(foot), many);
        spans.release(foot);
        assert_eq!(spans.foot(many), Footprint::Many(0), "the slot is reused");
        assert_eq!(spans.foot(Span::default()), Footprint::None);

        let mut up = Upload::default();
        up.lanes
            .get_mut::<crate::engine::gpu::vectors::VectorRows>()
            .rows
            .push(bytemuck::Zeroable::zeroed());
        let counts = Counts::of(&up);
        assert_eq!(counts.get(LaneId::Registered(0)), 1);
        let mut small = Counts::default();
        small.set(LaneId::Faces, 3);
        let mut big = small;
        big.set(LaneId::Faces, 6);
        big.set(LaneId::Dots, 1);
        assert!(small.fits(&big));
        assert!(!big.fits(&small));

        for lane in LaneId::all() {
            let mut counts = Counts::default();
            counts.set(lane, 9);
            assert_eq!(counts.get(lane), 9);
            assert_eq!(counts.bytes(), 9 * lane.stride());
        }
    }

    /// An id freed in one sync is reused only after it, last freed first.
    #[test]
    fn freed_ids_are_reused_lifo_after_the_sync() {
        let mut ids = Ids::default();
        ids.give(4);
        ids.give(9);
        assert_eq!(ids.take(), None, "not in the sync that freed them");
        assert_eq!(ids.len(), 2);
        ids.settle();
        assert_eq!(ids.take(), Some(9));
        assert_eq!(ids.take(), Some(4));
        assert_eq!(ids.take(), None);
    }
}
