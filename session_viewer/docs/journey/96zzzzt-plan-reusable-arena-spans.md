# 20.12.1 · Plan reusable arena spans

**Chapter 20 · Shared GPU rendering · Section 20.12 · Arena reclamation** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Prepare bounded hole reuse without moving any surviving geometry.

## Type

Continue from [Project live triangles through the lookup](96zzzzs-project-live-triangles-through-the-lookup.md). [Save or recover your work](recovery.md).

### 1. `src/arena_spans.rs`

Plan reusable spans before committing either geometry range, and reuse the existing coalescer for retirement.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-01.rs"
```

### 2. `src/lib.rs`

Declare the complete preparatory span helper.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod arena_rows;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod arena_spans;
mod arena_rows;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/arena_spans.rs`

Supply hole reuse, unchanged preflight state, overflow refusal, zero spans, coalescing and free-tail checks.

<details>
<summary>Locate the existing block</summary>

```text
use std::ops::Range;
pub struct Plan {pub range:Range<u32>,at:Option<usize>}
pub fn plan(free:&[Range<u32>],count:u32,end:u32)->Option<Plan> {
 if count>0 {if let Some(at)=free.iter().position(|span|span.end-span.start>=count){let first=free[at].start;return Some(Plan {range:first..first+count,at:Some(at)});}}
 Some(Plan {range:end..end.checked_add(count)?,at:None})
}
impl Plan {
 pub fn take(self,free:&mut Vec<Range<u32>>)->Range<u32> {
  if let Some(at)=self.at {free[at].start=self.range.end;if free[at].is_empty(){free.remove(at);}}
  self.range
 }
}
pub fn retire(free:&mut Vec<Range<u32>>,range:Range<u32>,mut end:u32)->u32 {
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
 *free=crate::slot_space::coalesce(std::mem::take(free),end);
 while free.last().is_some_and(|span|span.end==end){end=free.pop().unwrap().start;}end
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-03.rs" skip=854
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.12.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Reusable arena-span helpers compile; the application retains its existing drawing and allocation behavior.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Plan reusable arena spans; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Plan a vertex span and an index span before taking either. Prefer a suitable free interval; otherwise extend the checked tail. Retirement merges free intervals through the coalescer already written and trims only an unused tail. These helpers are preparatory; the application still appends geometry until the following connection.

Free intervals plus tail → bounded plan → take after both ranges validate.



Why separate planning from taking?

If either bounded request fails, neither free pool has changed.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select and clear an original, confirming unchanged drawing. Trace two free intervals and a request that cannot extend a u32 tail; neither original source nor the other interval should change.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.12.1
npm --prefix ../session_tests run course -- save 20.12.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Free-span planning is prepared. Arena retirement and GPU lease lifetime still need connection; current-viewer parity remains unfinished.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.12.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
