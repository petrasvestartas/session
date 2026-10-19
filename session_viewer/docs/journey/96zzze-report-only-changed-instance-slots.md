# 20.7.11 · Report only changed instance slots

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 28–55 minutes.** [Estimate](typing-load.md).

Connect the allocator helpers. Retire obsolete runs, patch touched members and reuse capacity for definitions that still fit. Move full runs into new room and compact excessive spare space. Return only the slot ranges that need GPU updates.

## Type

Continue from [Follow arena growth with slot headroom](96zzzd-follow-arena-growth-with-slot-headroom.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
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
```

</details>

After that block, type:

```rust

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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzze-report-only-changed-instance-slots-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/slot_space.rs`

Supply the five original allocator fixtures, retaining their allocation and changed-range assertions.

<details>
<summary>Locate the existing block</summary>

```text
a + 1 == *b)
        .map(|run| run[0]..run[0] + run.len() as u32),
    );
    Change {
      all: false,
      slots: coalesce(dirty, self.end()),
    }
  }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzze-report-only-changed-instance-slots-2.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for report only changed instance slots.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



touched members → reused runs → coalesced dirty ranges.



Why report changed ranges instead of every slot?

The GPU owner can update existing rows and texels in place. Unchanged definitions retain their allocations and uploads.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a member joining and leaving a definition. Identify the changed slots and any retained spare range. The existing drawing remains unchanged: the allocator is prepared here and connected to GPU ownership later.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.11
npm --prefix ../session_tests run course -- save 20.7.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This types the maintained CPU slot-allocation rules. GPU record ownership, slot and lookup uploads, and fragment consumers remain to be connected.



[Full validation scope](release.md).

</details>
