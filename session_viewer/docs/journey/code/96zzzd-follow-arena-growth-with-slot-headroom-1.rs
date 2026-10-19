   if merged.last().is_some_and(|run| run.key.is_none()) {
      merged.pop();
    }
    self.runs = merged;
    self.rows.truncate(self.end() as usize);
  }
}

impl SlotSpace {
  fn place_all(&mut self, definitions: &[Definition], base: u32) -> Change {
    self.runs.clear();
    self.rows.clear();
    self.rows.push(OWN_ROW);
    self.base = base;
    for definition in definitions {
      let len = definition.rows.len() as u32;
      if len > 0 {
        let at = self.allocate(len, definition.faces.len() as u32 / 3);
        self.hold(at, definition);
      }
    }
    Change::all()
  }
  pub fn reset(&mut self, definitions: &[Definition], arena: u32) -> Change {
    self.arena = arena;
    self.place_all(definitions, arena)
  }
  pub fn follow(&mut self, arena: u32) -> Change {
    if arena == self.arena {
      return Change::default();
    }
    self.arena = arena;
    if self.runs.is_empty() {
      self.base = arena;
      return Change::default();
    }
    if arena <= self.base && self.base - arena <= 2 * headroom(arena) {
      return Change {
        all: false,
        slots: vec![0..1],
      };
    }
    let held: Vec<(u32, Vec<u32>, Draw)> = self
      .runs
      .iter()
      .filter_map(|run| {
        let rows = self.rows[run.start as usize..(run.start + run.len) as usize].to_vec();
        Some((run.key?, rows, run.draw.clone()))
      })
      .collect();
    let definitions: Vec<Definition> = held
      .iter()
      .map(|(key, rows, draw)| Definition {
        key: *key,
        rows,
        faces: draw.faces.clone(),
        pipes: draw.pipes.clone(),
        ribbons: draw.ribbons.clone(),
      })
      .collect();
    self.place_all(&definitions, arena + headroom(arena))
  }
  fn retire(&mut self,definitions:&[Definition],live:&HashMap<u32,usize>)->Vec<Range<u32>> {
    let mut dirty=vec![0..1];
    for run in &mut self.runs {
      let Some(key) = run.key else {
        continue;
      };
      let same = live.get(&key).is_some_and(|&i| {
        let faces = &definitions[i].faces;
        faces.start / 3 == run.first && faces.len() as u32 / 3 == run.triangles
      });
      if !same {
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
    }
    dirty
  }
}
