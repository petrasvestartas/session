# 20.7.10 · Follow arena growth with slot headroom

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

A load or compaction places every definition again. Later arena growth should change only the lookup header while it fits before existing instance IDs. Renumber when the arena crosses that space, and retire runs whose definition or shared faces changed.

## Type

Continue from [Reuse and merge free slot ranges](96zzzc-reuse-and-merge-free-slot-ranges.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
    if merged.last().is_some_and(|run| run.key.is_none()) {
      merged.pop();
    }
    self.runs = merged;
    self.rows.truncate(self.end() as usize);
  }
}
```

</details>

After that block, type:

```rust

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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzd-follow-arena-growth-with-slot-headroom-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.10
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

![Native renderer output for follow arena growth with slot headroom.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



arena count → reserved headroom → header update or renumber.



Why avoid renumbering on every arena append?

Changing all instance IDs would also invalidate their lookup records. Headroom keeps small growth local.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Append geometry inside reserved headroom and compare every existing slot ID. Then cross the headroom boundary and trace the required growth. This preparation still leaves browser drawing unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.10
npm --prefix ../session_tests run course -- save 20.7.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This types the maintained CPU slot-allocation rules. GPU record ownership, slot and lookup uploads, and fragment consumers remain to be connected.



[Full validation scope](release.md).

</details>
