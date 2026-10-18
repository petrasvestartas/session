range.len() as u32).sum())
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

impl SlotSpace {
  pub fn end(&self) -> u32 {
    self.runs.last().map_or(1, |run| run.start + run.cap)
  }
  pub fn triangle_count(&self) -> Option<u32> {
    let last = self.runs.last()?;
    let any = self.runs.iter().any(|run| run.key.is_some() && run.len > 0);
    any.then(|| self.base + last.ids + last.span)
  }
  pub fn draws(&self) -> Vec<Draw> {
    let mut held: Vec<&Run> = Vec::new();
    for run in self
      .runs
      .iter()
      .filter(|run| run.key.is_some() && run.len > 0)
    {
      held.insert(held.partition_point(|other| other.key < run.key), run);
    }
    held.iter()
      .map(|run| Draw {
        slots: run.start..run.start + run.len,
        ..run.draw.clone()
      })
      .collect()
  }
  fn run_at(&self, slot: u32) -> usize {
    self.runs.partition_point(|run| run.start + run.cap <= slot)
  }
  pub fn slot(&self, slot: u32) -> Slot {
    let run = &self.runs[self.run_at(slot)];
    let start = self.base + run.ids + (slot - run.start) * run.triangles;
    [self.rows[slot as usize], start.wrapping_sub(run.first)]
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {
    if slot == 0 {
      return [self.end() - 1, self.arena, 0, 0];
    }
    let run = &self.runs[self.run_at(slot)];
    let j = slot - run.start;
    let start = self.base + run.ids + j * run.triangles;
    let count = if run.key.is_some() && j < run.len {
      run.triangles
    } else {
      0
    };
    [self.rows[slot as usize], start, run.first, count]
  }
}
fn shared_draw(definition: &Definition) -> Draw {
  Draw {
    faces: definition.faces.clone(),
    pipes: definition.pipes.clone(),
    ribbons: definition.ribbons.clone(),
    slots: 0..0,
  }
}
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
  let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());
  for range in ranges {
    sorted.insert(sorted.partition_point(|r| r.start <= range.start), range);
  }
  let mut out: Vec<Range<u32>> = Vec::with_capacity(sorted.len());
  for range in sorted {
    let range = range.start.min(end)..range.end.min(end);
    if range.is_empty() {
      continue;
    }
    match out.last_mut() {
      Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
      _ => out.push(range),
    }
  }
  out
}
