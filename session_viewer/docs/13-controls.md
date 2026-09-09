# 13 · Select original controls, including streamed points

**Start:** checkpoint 12. **Finish:** F10 exposes original source controls for one parent; streamed clouds query all relevant source ranges independently of display LOD.

## A control point is not a display vertex

A curve may draw hundreds of segment endpoints from a few NURBS controls. F10 must show the original controls. The same distinction applies to mesh source vertices, BRep topology and surface control grids.

`ControlId` records the source kind and index. `Controls` retains original positions and any control-net connections. Uploading temporary marker rows preserves those IDs; reading back a marker must map to the source control, not its temporary GPU slot.

```text
Object selection → F10 → Controls { parent, selected source control }
      ↑                         │
      └──── Esc keeps parent ───┘
      └──── next Esc clears selection
```

Repeated F10 is idempotent: it does not append duplicate markers. Entering another parent's mode replaces the old one. Hiding/replacing a parent invalidates controls and pending answers.

## Display LOD is not a selection cutoff

A cloud may display a bounded subset of millions of points. Selecting source points must not silently ignore everything beyond that subset. The source-query coordinator traverses every relevant node range in bounded pages and tests candidate visibility.

The GPU accumulates the closest visible source answer across pages. A later page can contain a nearer occluder, so the application does not announce a final winner until all eligible pages finish. It then reads the original ID and exact source position for the winner.

```text
click + camera + source revision
          ↓ eligible source node ranges
          ↓ bounded page fetch
          ↓ candidate ID visibility (repeat for every page)
          ↓ final original ID / position
       selected control, even outside displayed LOD
```

Cancellation belongs to the query generation. A camera change retires both HTTP completions and GPU readback. Candidate pages are ID targets, not temporary color frames shown to the user.

## Keep source query and ordinary input understandable

In the final architecture, `state/cloud_query.rs` contains the page/query coordination methods; `app/cloud_query.rs` owns the source query data and range logic. The mechanical split appears with final convergence in chapter 18. State still owns the operation, so there is no second scene or selection controller.

## Write the files

Follow [Complete file changes for 13](../lessons/13/index.md). Read source control collection first, then the selection mode transitions and marker upload. Finally trace one cloud page from request to candidate visibility to original-ID resolution.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Select each family in the local fixture, press F10, click a source control, press F10 again and verify no duplication. Escape returns to the parent; the next Escape clears it. Switch parents and confirm old controls disappear.

The maintained streamed fixture selects original ID `0xfedcba98` beyond row six million while displaying 250,000 points. It also delays a page, changes input and verifies cancellation. The fixture is a virtual range server, so it does not need to write a huge cloud file to disk.

**Before continuing:** explain why the query cannot stop after its first apparently visible point. Continue to [loading](14-loading.md).
