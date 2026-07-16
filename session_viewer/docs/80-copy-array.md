# 80 Copy, duplicate, array — daily-use editing, nearly free

> **Big picture.** *Phase 14.* Copying is the most-used editing operation in any CAD session — and
> on this architecture it costs almost nothing, because it's three existing rails composed:
> **clone** (kernel objects are `Clone`), **fresh identity** (`refresh_guid` — a kernel method this
> lesson added, because `set_guid` silently no-ops on a clone whose guid is already minted — a real
> trap), and **`AddGeometry`** (57 — so every copy is undoable for free). The lesson's real content
> is that identity trap and the Alt-drag wrinkle.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="copy clones the selection, refreshes guids so the clones are new identities, applies the from-to delta, and commits one AddGeometry command" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="30" width="110" height="34"/><rect x="150" y="30" width="130" height="34"/>
    <rect x="312" y="30" width="130" height="34"/><rect x="474" y="30" width="196" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="63" y="47">clone selection</text><text x="63" y="59" fill="#666" font-size="9">Geometry: Clone</text>
    <text x="215" y="47">refresh_guid()</text><text x="215" y="59" fill="#e05555" font-size="9">set_guid NO-OPS on clones!</text>
    <text x="377" y="47">apply from→to Δ</text><text x="377" y="59" fill="#666" font-size="9">54's apply_delta</text>
    <text x="572" y="47">AddGeometry (57)</text><text x="572" y="59" fill="#666" font-size="9">one Command → one undo</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3">
    <line x1="118" y1="47" x2="148" y2="47" marker-end="url(#ah80)"/>
    <line x1="280" y1="47" x2="310" y2="47" marker-end="url(#ah80)"/>
    <line x1="442" y1="47" x2="472" y2="47" marker-end="url(#ah80)"/>
  </g>
  <text x="340" y="100" fill="#888" text-anchor="middle">array = the same pipeline in a loop; Alt+gumball = the same pipeline on 54's release</text>
  <defs><marker id="ah80" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
    <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## The identity trap (why the kernel grew `refresh_guid`)

Guids are lazy `OnceLock`s: minted on first read, and **`Clone` copies the minted value**. So a
cloned wall *is* the original as far as `lookup`, the tree, undo snapshots, and reconcile are
concerned — inserting it would *overwrite* the original. And the existing `set_guid` is
`let _ = self.guid.set(g)` — a **silent no-op** once the lock is filled, i.e. on every clone.
This lesson added the honest primitive to the kernel, ×3 languages + a minitest:

```rust
    /// Clear the guid so a FRESH one mints lazily on next read — the duplicate/copy enabler.
    pub fn refresh_guid(&mut self) {
        self.guid = std::sync::OnceLock::new();
    }
```

The `Refresh Guid` minitest pins the contract: clone equals the original's guid; after refresh it
differs; the original is untouched.

## Files we touch

```
src/app/scene.rs      # clone_selection() → Vec<Geometry> with fresh guids
src/app/commands.rs   # `copy` (two-point Get-loop) and `array` verbs
src/state.rs          # Alt held at gumball-press → drag a COPY (54's skeleton, one branch)
```

## Step 1 — clone with fresh identities: `src/app/scene.rs`

```rust
    /// Deep-copies of the selection, each with a NEW guid (and the name suffixed so the
    /// tree reads sanely). The Geometry dispatch mirrors restore_geometry's arms (51).
    pub fn clone_selection(&self) -> Vec<Geometry> {
        let mut out = Vec::new();
        for g in &self.selected {
            let Some(geom) = self.session.lookup.get(g) else { continue };
            let mut c = geom.clone();
            match &mut c {
                Geometry::Mesh(m) => { m.refresh_guid(); m.name.push_str("_copy"); }
                Geometry::BRep(b) => { b.refresh_guid(); b.name.push_str("_copy"); }
                Geometry::Line(l) => { l.refresh_guid(); l.name.push_str("_copy"); }
                Geometry::Polyline(p) => { p.refresh_guid(); p.name.push_str("_copy"); }
                Geometry::Point(p) => { p.refresh_guid(); p.name.push_str("_copy"); }
                Geometry::NurbsCurve(nc) => { nc.refresh_guid(); nc.name.push_str("_copy"); }
                Geometry::NurbsSurface(ns) => { ns.refresh_guid(); ns.name.push_str("_copy"); }
                _ => {}
            }
            out.push(c);
        }
        out
    }
```

## Step 2 — the `copy` command: `src/app/commands.rs`

A two-point conversation (49's `ProbeCmd` shape): *from* anchor, *to* target; the delta is a plain
translation applied with 54's `apply_delta`; the whole batch commits as **one** `AddGeometry`:

```rust
    // CopyCmd::feed_point, second point:
    let d = Xform::translation(to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let mut clones = state.scene.clone_selection();
    for c in &mut clones {
        apply_delta(c, &d);                              // 54's per-variant compose/bake
    }
    let n = clones.len();
    state.commit(Box::new(AddGeometry::of_snapshots(clones)));
    CmdStep::Done(format!("copied {n} object(s)"))
```

(`AddGeometry::of_snapshots(Vec<Geometry>)` — the plural constructor 57's wrapper already supports
through `RemoveObjects::of_snapshots`. Snap (59) applies to both picks automatically, so
copy-from-corner-to-corner is *exact*.)

`array` is the same in a loop — count from the Get-loop's number parsing (49), one command total:

```rust
    // array N: after `copy`'s two points, repeat the delta N−1 more times, accumulating:
    let mut all = Vec::new();
    for k in 1..=n {
        let dk = Xform::translation(dx * k as f64, dy * k as f64, dz * k as f64);
        let mut batch = state.scene.clone_selection();
        for c in &mut batch { apply_delta(c, &dk); }
        all.extend(batch);
    }
    state.commit(Box::new(AddGeometry::of_snapshots(all)));
```

## Step 3 — Alt+gumball-drag = drag a copy: `src/state.rs`

One branch on 54's **release**, not its press: the drag runs exactly as 54 built it (live,
matrix-only, on the originals — cheapest possible preview), and Alt changes only what *commits*:

```rust
        // 54's release handler, first lines:
        if let Some(ctx) = self.gb_drag.take() {
            if self.alt_down {
                // originals go BACK to where they were (their kernel objects were never touched —
                // the live path only moved instance rows); the CLONES take the delta.
                for (_, row, base) in &ctx.base_models {
                    self.gpu.set_live_model(*row, base);          // snap originals home
                }
                let mut clones = self.scene.clone_selection();
                for c in &mut clones {
                    apply_delta(c, &ctx.final_delta);
                }
                self.commit(Box::new(AddGeometry::of_snapshots(clones)));
                self.gb_pressed = None;
                return;
            }
            // …54's normal commit continues unchanged…
        }
```

The subtlety that makes this clean: 54's live drag **never mutates the kernel objects** — so
"restore the originals" is just re-uploading their stashed models, and the copies are built from
pristine originals. That mid-drag discipline was designed in 54 for Esc-cancel; Alt-copy is its
second customer.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select a beam, `copy`, click its end, click the neighbouring column's corner (watch the `[End]`
  snaps) → an exact copy lands, selected state preserved, log `copied 1 object(s)`. **Ctrl+Z**
  removes *only the copy*. Click both — **different guids**, different tree rows.
- **Alt+drag** the X arrow → the original stays put, a copy follows the drag live; release → copy
  committed, one undo step. Without Alt → 54's normal move, unchanged.
- `array 5 …` → five more beams at even spacing, ONE undo step for all five.
- The trap test: comment out `refresh_guid` in `clone_selection` and `copy` once — the "copy"
  *replaces* the original in `lookup` and the scene loses an object. Restore the line. That's the
  bug the kernel method exists to prevent.

## Recap

```
Ch 79: files in and out.
Ch 80: DUPLICATION = three rails composed: Clone + refresh_guid + AddGeometry. The trap: guids are
       minted OnceLocks — Clone copies the VALUE and set_guid silently no-ops on filled locks, so a
       naive copy overwrites its original in lookup; refresh_guid (added to the kernel ×3 + minitest
       for this lesson) clears the lock so a fresh guid mints lazily. copy = two snapped points →
       translation → apply_delta per clone → ONE AddGeometry (undo removes the batch). array = the
       loop form, still one Command. Alt+gumball = 54's drag untouched until RELEASE: originals snap
       home (live path never touched kernel objects — 54's Esc discipline, second customer), clones
       take the final delta.
```

Edited: `app/scene.rs` (`clone_selection`), `app/commands.rs` (`copy`, `array`), `state.rs`
(Alt branch on 54's release). Kernel (done with this lesson): `refresh_guid` on 7 geometry types
×3 languages + `Refresh Guid` minitest ×3.

## Next

`81-layers.md` — organization users expect: layers, built on the tree groups the kernel already has.
