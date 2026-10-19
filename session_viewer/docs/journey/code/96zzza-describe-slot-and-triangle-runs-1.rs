use crate::draw_slots::{OWN_ROW,Slot};
use std::{collections::HashMap,ops::Range};
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct Draw {
  pub faces: Range<u32>,
  pub pipes: Range<u32>,
  pub ribbons: Range<u32>,
  pub slots: Range<u32>,
}
fn headroom(arena: u32) -> u32 {
  arena / 64 + 1024
}
const SPARE_IDS: u32 = 4096;
fn grown_cap(len: u32, triangles: u32) -> u32 {
  len + len / 4 + (SPARE_IDS / triangles.max(1)).min(4)
}
pub struct Definition<'a> {
  pub key: u32,
  pub rows: &'a [u32],
  pub faces: Range<u32>,
  pub pipes: Range<u32>,
  pub ribbons: Range<u32>,
}
#[derive(Clone, Debug, PartialEq)]
struct Run {
  key: Option<u32>,
  start: u32,
  cap: u32,
  len: u32,
  ids: u32,
  span: u32,
  first: u32,
  triangles: u32,
  draw: Draw,
}
impl Run {
  fn free(start: u32, cap: u32, ids: u32, span: u32) -> Self {
    Self {
      key: None,
      start,
      cap,
      len: 0,
      ids,
      span,
      first: 0,
      triangles: 0,
      draw: Draw::default(),
    }
  }
  fn freed(&self) -> Self {
    Self::free(self.start, self.cap, self.ids, self.span)
  }
}
#[derive(Debug, Default, PartialEq)]
pub struct Change {
  pub all: bool,
  pub slots: Vec<Range<u32>>,
}
impl Change {
  fn all() -> Self {
    Self {
      all: true,
      slots: Vec::new(),
    }
  }
  #[cfg(test)]
  fn written(&self) -> Option<u32> {
    (!self.all).then(|| self.slots.iter().map(|range| range.len() as u32).sum())
  }
}
#[derive(Default)]
pub struct SlotSpace {
  rows: Vec<u32>,
  runs: Vec<Run>,
  base: u32,
  arena: u32,
  pub written: u32,
}
