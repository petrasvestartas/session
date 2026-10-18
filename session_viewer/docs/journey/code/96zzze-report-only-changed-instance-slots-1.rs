2 / 3 == run.triangles
      });
      if !same {
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
    }
    dirty
  }
}

impl SlotSpace {
  pub fn update(
    &mut self,
    definitions: &[Definition],
    touched: &[(u32, u32)],
    arena: u32,
  ) -> Change {
    if self.follow(arena).all {
      return self.place_all(definitions, self.base);
    }
    let mut members: Vec<u32> = Vec::new();
    let live: HashMap<u32, usize> = (0..definitions.len())
      .map(|i| (definitions[i].key, i))
      .collect();
    let mut dirty=self.retire(definitions,&live);
    let held: HashMap<u32, usize> = self
      .runs
      .iter()
      .filter_map(|run| Some((run.key?, run.start as usize)))
      .collect();
    for &(key, position) in touched {
      let (Some(&start), Some(&i)) = (held.get(&key), live.get(&key)) else {
        continue;
      };
      let run = &self.runs[self.run_at(start as u32)];
      let rows = definitions[i].rows;
      if position < run.cap && (position as usize) < rows.len() {
        let slot = run.start + position;
        self.rows[slot as usize] = rows[position as usize];
        members.push(slot);
      }
    }
    for definition in definitions {
      let len = definition.rows.len() as u32;
      let grow = held
        .get(&definition.key)
        .map(|&start| self.run_at(start as u32));
      if let Some(at) = grow {
        let run = &mut self.runs[at];
        if len <= run.cap {
          if len < run.len {
            dirty.push(run.start + len..run.start + run.len);
          }
          run.len = len;
          run.draw = shared_draw(definition);
          continue;
        }
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
      if len == 0 {
        continue;
      }
      let triangles = definition.faces.len() as u32 / 3;
      let cap = if grow.is_some() {
        grown_cap(len, triangles)
      } else {
        len
      };
      let at = self.allocate(cap, triangles);
      let run = self.hold(at, definition);
      dirty.push(run);
    }
    self.merge_free();
    let (mut free, mut used, mut free_ids, mut used_ids) = (0, 0, 0u64, 0u64);
    for run in &self.runs {
      let ids = u64::from(run.len) * u64::from(run.triangles);
      free += run.cap - run.len;
      used += run.len;
      free_ids += u64::from(run.span) - ids;
      used_ids += ids;
    }
    let spare_ids = u64::from(SPARE_IDS) * self.runs.len() as u64;
    if free > used + 256 || free_ids > used_ids / 2 + spare_ids {
      return self.place_all(definitions, self.base);
    }
    members.sort_unstable();
    members.dedup();
    dirty.extend(
      members
        .chunk_by(|a, b| a + 1 == *b)
        .map(|run| run[0]..run[0] + run.len() as u32),
    );
    Change {
      all: false,
      slots: coalesce(dirty, self.end()),
    }
  }
}
