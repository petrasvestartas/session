# 20.12.5 · Retire empty arena ranges after tail trimming

**Chapter 20 · Shared GPU rendering · Section 20.12 · Arena reclamation** · [Contents](contents.md)

**Typing: 1–1 minutes.** [Estimate](typing-load.md).

Release empty source records independently of the current arena extent.

## Type

Continue from [Reclaim ranges after the last GPU lease](96zzzzw-reclaim-ranges-after-the-last-gpu-lease.md). [Save or recover your work](recovery.md).

### 1. `src/arena_spans.rs`

Empty ranges reserve no bytes; a previous free-tail trim cannot invalidate their release.

<details>
<summary>Locate the existing block</summary>

```rust
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 if range.start==range.end{return end;}
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzx-retire-empty-arena-ranges-after-tail-trimming-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply both source retirement orders after tail trimming, exact empty endpoints, retained originals, subsequent zero-based reuse and unchanged rejection of reversed/out-of-bounds nonempty ranges.

<details>
<summary>Locate the existing block</summary>

```text
use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}

#[test]
fn retired_ranges_reuse_holes_without_pinning_or_moving_surviving_sources() {
 let a=source();let b=source();let history=Rc::clone(&a);let weak=Rc::downgrade(&a);let mut rows=crate::arena_rows::Rows::default();let first=rows.append(&a).unwrap();let survivor=rows.append(&b).unwrap();let held=rows.vertices[60..120].to_vec();
 assert!(!rows.retire(usize::MAX));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert!(weak.upgrade().is_some());assert_eq!(rows.source_count(),1);
 let c=Rc::new((*source()).clone().with_alphas(vec![0.5;3]).unwrap());let reused=rows.append(&c).unwrap();assert_eq!(reused,first);assert_eq!(rows.append(&b).unwrap(),survivor);assert_eq!(rows.vertices.len(),120);assert_eq!(&rows.vertices[..60],crate::packed_vertex::bytes(&c));assert_eq!(&rows.vertices[60..120],held);assert_eq!(rows.indices,[0,1,2,3,4,5]);
 let restored=rows.append(&history).unwrap();assert_eq!(restored.vertices,6..9);assert_eq!(restored.indices,6..9);assert_ne!(restored,reused);assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);assert_eq!(c.indices(),&[0,1,2]);
 assert!(rows.retire(Rc::as_ptr(&c)as usize));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert_eq!(rows.vertices.len(),120);assert_eq!(rows.append(&b).unwrap(),survivor);assert!(rows.retire(Rc::as_ptr(&b)as usize));assert!(rows.vertices.is_empty());assert!(rows.indices.is_empty());assert_eq!(rows.source_count(),0);assert_eq!(rows.append(&history).unwrap().vertices,0..3);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzx-retire-empty-arena-ranges-after-tail-trimming-02.rs" skip=3150
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.12.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Empty geometry retires safely in either order while preserving nonempty range bounds and hole reuse.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Retire empty arena ranges after tail trimming; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

An empty geometry can receive a zero-length range at the old tail. Retiring another source may shorten the tail before that empty record expires. Since the empty range owns no bytes, return the current end without inserting a hole or checking its old endpoint against the new extent. Keep all bounds checks for nonempty ranges.

Retire occupied tail → arena end shrinks → release empty source association → keep the new end.



Why can an empty range’s endpoint lie beyond the current arena end?

It was recorded before another retirement trimmed the tail. A zero-length range reserves no element, so its old endpoint does not constrain later release.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run Example CAD, Clipping Plane XY 0,0,0, Clipping Plane Solid and Fit. Run Select All, Delete, Undo and Select None repeatedly. Confirm exact restoration and no runtime error. The supplied optional fixture separately checks both empty/nonempty retirement orders and nonempty bounds.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.12.5
npm --prefix ../session_tests run course -- save 20.12.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Primitive lookup, incremental slots and arena reclamation remain connected, including empty source release. Camera precision/depth and the other completion items remain required.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.12.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
