# 20.7.9 · Reuse and merge free slot ranges

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Find the first free run with enough slots and triangle IDs. Split its unused tail instead of discarding it. Fill the occupied run from its definition, then join neighbouring free runs and remove unused room at the end.

## Type

Continue from [Read slot runs and lookup texels](96zzzb-read-slot-runs-and-lookup-texels.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
    }
    match out.last_mut() {
      Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
      _ => out.push(range),
    }
  }
  out
}
```

</details>

After that block, type:

```rust

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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzc-reuse-and-merge-free-slot-ranges-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.9
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

![Native renderer output for reuse and merge free slot ranges.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



free range → fitting allocation → retained remainder.



Why check both slot capacity and triangle span?

Equal member counts can refer to definitions with different triangle counts. A hole must fit both resources.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Release two adjacent runs, merge their free intervals and allocate a smaller definition into the hole. Check both member capacity and triangle span; retain the unused remainder.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.9
npm --prefix ../session_tests run course -- save 20.7.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This types the maintained CPU slot-allocation rules. GPU record ownership, slot and lookup uploads, and fragment consumers remain to be connected.



[Full validation scope](release.md).

</details>
