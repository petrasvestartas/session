# 45 Selection — highlight + marquee

> **Big picture.** *Phase 7.* Picking (42–44) answers a click; **selection is state** — the set of
> objects every later tool acts on: gumball (52) moves the selection, `delete` (51) removes it, the
> tree (70) mirrors it. This lesson stores that set, makes it *visible* (a tint via one instance-flag
> bit — the same mechanism as hidden/culled), and adds the second way to build it: the drag
> rectangle. The marquee has a genuinely pleasing trick — a rectangle on screen IS a smaller frustum,
> so 37's plane code does all the work.

<svg viewBox="0 0 680 160" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a drag rectangle on screen corresponds to a cropped sub-frustum in the scene; objects inside its planes get FLAG_SELECTED" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="20" y="26" width="150" height="100" fill="none" stroke="#4a4a4a"/>
  <rect x="60" y="50" width="70" height="45" fill="none" stroke="#6fb3ff" stroke-dasharray="4 3"/>
  <text x="95" y="140" fill="#888" text-anchor="middle">drag rect (screen)</text>
  <text x="205" y="80" fill="#6fb3ff" font-size="15">▶</text>
  <g stroke="#6fb3ff" fill="none" stroke-width="1.2">
    <path d="M260,76 L420,36 L420,116 Z" opacity="0.9"/>
    <path d="M260,76 L420,52 M260,76 L420,100" stroke-dasharray="3 3" opacity="0.6"/>
  </g>
  <text x="340" y="140" fill="#888" text-anchor="middle">= a cropped frustum (world)</text>
  <g transform="translate(460,0)">
    <text x="10" y="40" fill="#d7dae0">crop · view_proj → 37's</text>
    <text x="10" y="58" fill="#d7dae0">Frustum::from_view_proj</text>
    <text x="10" y="82" fill="#666">same 6 planes, same aabb test,</text>
    <text x="10" y="98" fill="#666">zero new plane math</text>
    <text x="10" y="126" fill="#6fb3ff">inside → FLAG_SELECTED</text>
  </g>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs   # FLAG_SELECTED; set_selected_rows (flip-tracking like 37's cull)
src/shaders/*.wgsl      # every instance-reading vs tints selected rows
src/camera.rs           # Frustum::cropped(view_proj, rect_ndc) — the marquee sub-frustum
src/app/scene.rs        # selected: HashSet<guid>; click/shift-click/marquee mutate it
src/state.rs            # mouse: click = replace, Shift+click = toggle, drag = marquee
```

## Step 1 — the flag + flip-tracked upload: `src/engine/gpu/mod.rs`

Bit 0 was reserved for exactly this (35):

```rust
impl Instance {
    pub const FLAG_SELECTED: u32 = 1 << 0;   // ← ADD above FLAG_HIDDEN (1<<1) / FLAG_CULLED (1<<7)
```

Selection changes a few rows out of 42k, so upload like 37's cull did — only rows whose membership
*flipped*:

```rust
    /// Set the selected bit on exactly `rows`; clear it everywhere else. Uploads only changed rows.
    pub fn set_selected_rows(&mut self, rows: &std::collections::HashSet<u32>) {
        for i in 0..self.instances.len() as u32 {
            let want = rows.contains(&i);
            let has = self.instances[i as usize].flags & Instance::FLAG_SELECTED != 0;
            if want != has {
                self.write_row(i, |inst| inst.flags ^= Instance::FLAG_SELECTED);
            }
        }
    }
```

## Step 2 — the tint: every instance-reading shader

Selected objects tint toward highlight-yellow in **every** pipeline the object owns — faces, edge
tubes, sphere glyphs, billboards — because they all read the same `instances[]` row. In each vs
(`mesh.wgsl`, `cylinder.wgsl`, `sphere.wgsl`, `point.wgsl`), right after the flags/hidden check from
37 Step 3:

```wgsl
const FLAG_SELECTED: u32 = 1u;

// after computing the output color:
if ((inst.flags & FLAG_SELECTED) != 0u) {
    o.color = mix(o.color, vec4<f32>(1.0, 0.85, 0.2, o.color.a), 0.6);
}
```

One bit, four pipelines, and the whole object lights up as a unit — the payoff of routing everything
through the instance table since 29.

## Step 3 — the marquee frustum: `src/camera.rs`

A drag rectangle covers `[x0,x1]×[y0,y1]` in NDC. Compose a **crop matrix** that stretches exactly
that window to the full `[-1,1]` cube, multiply it onto `view_proj`, and the cropped matrix *is* a
view-projection whose frustum is the marquee volume — feed it straight to 37's plane extractor:

```rust
    /// The 6 world-space planes of the sub-frustum under a screen rectangle (NDC coords).
    /// crop · view_proj remaps the rect to the full clip cube, so Gribb–Hartmann (37) needs
    /// no change.
    pub fn marquee_frustum(view_proj: &Xform, origin: &Point,
                           x0: f64, y0: f64, x1: f64, y1: f64) -> Frustum {
        let (sx, sy) = (2.0 / (x1 - x0), 2.0 / (y1 - y0));
        let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let crop = &Xform::translation(-sx * cx, -sy * cy, 0.0) * &Xform::scale_xyz(sx, sy, 1.0);
        Frustum::from_view_proj(&(&crop * view_proj).to_cols()).rebased_to_world(origin)
    }
```

(Convert cursor px → NDC with 41's two formulas; make sure `x0 < x1`, `y0 < y1` after sorting the
drag endpoints, and ignore drags under ~3 px — those are clicks.)

## Step 4 — selection state + the three gestures: `src/app/scene.rs` + `src/state.rs`

`Scene` gets the set and one apply function:

```rust
    pub selected: std::collections::HashSet<String>,   // ← ADD to Scene (init empty)

    /// Push the current selection to the GPU flags (call after any mutation below).
    pub fn apply_selection(&self, gpu: &mut Gpu) {
        let rows = self.selected.iter().filter_map(|g| self.guid_to_row.get(g).copied()).collect();
        gpu.set_selected_rows(&rows);
    }

    /// Marquee: everything whose world box the sub-frustum accepts. Crossing style —
    /// touching counts.
    pub fn select_marquee(&mut self, f: &Frustum, additive: bool) {
        if !additive { self.selected.clear(); }
        for guid in &self.order {
            let (lo, hi) = self.world_aabb(guid);
            if f.aabb_visible(lo, hi) { self.selected.insert(guid.clone()); }
        }
    }
```

(`world_aabb` is 37's helper; the scan is linear like 37's cull — per *release*, not per frame, so
even cheaper. The BVH broad-phase (36) is the at-scale upgrade if release-lag ever shows.)

In `state.rs`, the mouse gestures — press remembers the spot; release decides click vs drag:

```rust
        // release at ~press position → CLICK: replace (or Shift → toggle)
        match self.scene.pick_ray(&ray, tol) {
            Some(hit) if shift => {
                if !self.scene.selected.remove(&hit.guid) {
                    self.scene.selected.insert(hit.guid);
                }
            }
            Some(hit)          => {
                self.scene.selected.clear();
                self.scene.selected.insert(hit.guid);
            }
            None if !shift     => { self.scene.selected.clear(); }
            None               => {}
        }
        // release far from press → MARQUEE (Shift → additive)
        let f = Camera::marquee_frustum(&vp, &origin, x0, y0, x1, y1);
        self.scene.select_marquee(&f, shift);
        // either way:
        self.scene.apply_selection(&mut self.gpu);
```

(Drawing the rubber-band rectangle itself is two triangles in an overlay pass — or simply defer the
visual to 47's egui, which gives it for free; selection is correct either way.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Click** an object → it (faces *and* edges *and* glyphs) turns highlight-yellow; click another →
  the first reverts. Click empty space → everything deselects. **Shift+click** toggles one object in
  and out without touching the rest — Rhino's exact grammar.
- **Marquee** a region → exactly the objects touching the rectangle highlight, including ones only
  partially inside (crossing select). Orbit first, marquee at a steep angle — still correct: the test
  ran in world space against a real frustum, not in 2-D.
- **Stress gate** — marquee a large region of the PDF drawing: thousands select, release is instant,
  and the frame rate doesn't move afterwards (the flip-tracked upload wrote only the changed rows).

## Recap

```
Ch 44: thin picking — radius + mesh-wins-ties.
Ch 45: SELECTION. State = Scene.selected: HashSet<guid> — the set every later tool acts on. Visible
       via FLAG_SELECTED (bit 0, reserved since 35): one bit read by all four instance pipelines →
       the whole object tints as a unit; set_selected_rows uploads only flipped rows (37's pattern).
       Click = replace, Shift+click = toggle, empty click = clear. MARQUEE: the drag rect is
       remapped to the full clip cube by a crop matrix (translation · scale_xyz), so crop·view_proj
       fed to 37's Frustum::from_view_proj yields the sub-frustum's 6 world planes with zero new
       plane math; select_marquee = aabb_visible over world boxes (linear, per release), crossing
       style.
```

Edited: `engine/gpu/mod.rs` (`FLAG_SELECTED`, `set_selected_rows`), `shaders/*.wgsl` (selection tint),
`camera.rs` (`marquee_frustum` — crop · view_proj), `app/scene.rs` (`selected`, `apply_selection`,
`select_marquee`), `state.rs` (click / Shift+click / marquee gestures).

## Next

`46-hidden-filter.md` — visibility becomes real state. 35 parked a `hidden` set and a flag bit; now
`hide`/`show` actually work end to end: a hidden object doesn't draw (already), doesn't pick, doesn't
marquee — and the little state machine here is the groundwork for the first CLI verbs (48).
