  }
    match out.last_mut() {
      Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
      _ => out.push(range),
    }
  }
  out
}

impl SlotSpace {
  fn hold(&mut self, at: usize, definition: &Definition) -> Range<u32> {
    let run = &mut self.runs[at];
    let len = definition.rows.len();
    run.key = Some(definition.key);
    run.len = len as u32;
    run.first = definition.faces.start / 3;
    run.triangles = definition.faces.len() as u32 / 3;
    run.draw = shared_draw(definition);
    let (start, end) = (run.start as usize, (run.start + run.cap) as usize);
    if self.rows.len() < end {
      self.rows.resize(end, 0);
    }
    self.rows[start..start + len].copy_from_slice(definition.rows);
    start as u32..end as u32
  }
  fn allocate(&mut self, cap: u32, triangles: u32) -> usize {
    let span = cap * triangles;
    let fits = |run: &Run| run.key.is_none() && run.cap >= cap && run.span >= span;
    let Some(at) = self.runs.iter().position(fits) else {
      let (start, ids) = self
        .runs
        .last()
        .map_or((1, 0), |run| (run.start + run.cap, run.ids + run.span));
      self.runs.push(Run::free(start, cap, ids, span));
      return self.runs.len() - 1;
    };
    let hole = self.runs[at].clone();
    if hole.cap > cap {
      self.runs[at] = Run::free(hole.start, cap, hole.ids, span);
      let rest = Run::free(
        hole.start + cap,
        hole.cap - cap,
        hole.ids + span,
        hole.span - span,
      );
      self.runs.insert(at + 1, rest);
    }
    at
  }
  fn merge_free(&mut self) {
    let mut merged: Vec<Run> = Vec::with_capacity(self.runs.len());
    for run in self.runs.drain(..) {
      match merged.last_mut() {
        Some(last) if last.key.is_none() && run.key.is_none() => {
          last.cap += run.cap;
          last.span += run.span;
        }
        _ => merged.push(run),
      }
    }
    if merged.last().is_some_and(|run| run.key.is_none()) {
      merged.pop();
    }
    self.runs = merged;
    self.rows.truncate(self.end() as usize);
  }
}
