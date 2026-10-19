# 20.7.8 · Read slot runs and lookup texels

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Read the row and triangle offset for a slot. Derive the four-word lookup texel from that same run, with zero triangles for unused room. Report draws in definition order and merge overlapping changed ranges.

## Type

Continue from [Describe slot and triangle runs](96zzza-describe-slot-and-triangle-runs.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
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
```

</details>

After that block, type:

```rust

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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzb-read-slot-runs-and-lookup-texels-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.8
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

![Native renderer output for read slot runs and lookup texels.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



CPU run → vertex slot and fragment lookup texel.



Why does an unused slot have zero triangles?

Reserved capacity is not a live member. Fragment lookup must not treat its spare triangle IDs as drawing.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Reuse the [run sketch](96zzza-describe-slot-and-triangle-runs.md). Read one occupied slot and one spare slot, then derive their lookup texels. The spare slot must describe zero live triangles.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.8
npm --prefix ../session_tests run course -- save 20.7.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This types the maintained CPU slot-allocation rules. GPU record ownership, slot and lookup uploads, and fragment consumers remain to be connected.



[Full validation scope](release.md).

</details>
