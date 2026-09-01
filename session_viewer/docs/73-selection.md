# 73 Selection — highlight + marquee

> **Big picture.** *Phase 7.* Picking (55–57) answers a click; **selection is state** — the set of
> objects every later tool acts on: gumball (65) moves the selection, `delete` (64) removes it, the
> tree (82) mirrors it. This lesson stores that set, makes it *visible* (a tint via one instance-flag
> bit — the same mechanism as hidden/culled), and adds the second way to build it: the drag
> rectangle. The marquee has a genuinely pleasing trick — a rectangle on screen IS a smaller frustum,
> so 41's plane code does all the work.

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
    <text x="10" y="40" fill="#d7dae0">crop · view_proj → 41's</text>
    <text x="10" y="58" fill="#d7dae0">Frustum::from_view_proj</text>
    <text x="10" y="82" fill="#666">same 6 planes, same aabb test,</text>
    <text x="10" y="98" fill="#666">zero new plane math</text>
    <text x="10" y="126" fill="#6fb3ff">inside → FLAG_SELECTED</text>
  </g>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs   # FLAG_SELECTED; write_row_flags; set_selected_rows (tables + live rows)
src/shaders/*.wgsl      # every instance-reading vs tints selected rows (triangle/cylinder/sphere)
src/camera.rs           # Camera::marquee_frustum(view_proj, rect_ndc) — the marquee sub-frustum
src/app/scene.rs        # selected: HashSet<u32> (ROWS — see Step 4); click/shift-click/marquee mutate it
src/state.rs            # on_left_release: click = replace, Shift+click = toggle, drag = marquee
src/lib.rs              # Left press stashes drag_start; release calls on_left_release; shift flag
```

## Step 1 — the flag + flip-tracked upload: `src/engine/gpu/mod.rs`

Bit 0 was reserved for exactly this (35):

```rust
impl Instance {
    pub const FLAG_SELECTED: u32 = 1 << 0;   // ← ADD above FLAG_HIDDEN (1<<1) / FLAG_CULLED (1<<7)
```

The write has two targets, and the order of importance matters. `scene.tables.objects[row].2` is
the **truth**: `set_scene` re-derives every instance from the tables, so a selection made while
later manifest files are still streaming in survives the next `Msg::File` append *because* it lives
there. The live instance is poked too, so the tint shows this frame instead of after the next
rebuild — through a tiny helper, because `Instance.flags` is private to the engine, which is why
the write lives in `gpu/mod.rs` and not in `Scene`:

```rust
    /// One row's flags → the live instance + an upload of just that row. FLAG_CULLED is
    /// per-frame engine state that never enters the tables, so preserve it on the way through.
    pub fn write_row_flags(&mut self, row: u32, flags: u32) {
        let keep = self.instances[row as usize].flags & Instance::FLAG_CULLED;
        self.instances[row as usize].flags = flags | keep;
        self.queue.write_buffer(&self.instance_buffer,
            (row as usize * std::mem::size_of::<Instance>()) as u64,
            bytemuck::bytes_of(&self.instances[row as usize]));
    }
```

Selection changes a few rows out of 42k, so upload like 41's cull did — only rows whose membership
*flipped*, tables first, live row second:

```rust
    /// Set the selected bit on exactly `rows`; clear it everywhere else. Writes the TABLES (the
    /// truth set_scene re-derives from) AND pokes each flipped row's live instance. Only changed
    /// rows hit the GPU.
    pub fn set_selected_rows(&mut self, tables: &mut ArenaUpload,
                             rows: &std::collections::HashSet<u32>) {
        for i in 0..tables.objects.len() as u32 {
            let want = rows.contains(&i);
            let has = tables.objects[i as usize].2 & Instance::FLAG_SELECTED != 0;
            if want != has {
                tables.objects[i as usize].2 ^= Instance::FLAG_SELECTED;
                let f = tables.objects[i as usize].2;
                self.write_row_flags(i, f);
            }
        }
    }
```

## Step 2 — the tint: every instance-reading shader

Selected objects tint toward highlight-yellow in **every** pipeline the object owns — faces, edge
tubes, sphere glyphs — because they all read the same `instances[]` row. The instance-reading
shaders with live tables are `triangle.wgsl`, `cylinder.wgsl`, `sphere.wgsl` — plus 34f's
`ribbon.wgsl`/`glyph.wgsl`, which get the cylinder/sphere edit respectively (their 34h tint
multiply line is the `o.color` anchor; `point.wgsl`'s cloud table stays empty until PointCloud is
wired). They differ in two ways that matter here: `triangle.wgsl` already has an `inst` local and its
`VsOut.color` is a **`vec3`**; `cylinder.wgsl`/`sphere.wgsl` only pull `.model` out of `instances[…]`
and their `VsOut.color` is a **`vec4`**. So the tint takes two forms.

In **`triangle.wgsl`**, find `o.color = in.color.rgb * inst.color.rgb;` (34h's line) and add right
after it (vec3, no alpha — `inst` is already in scope; the `const` goes at the top of the file):

```wgsl
const FLAG_SELECTED: u32 = 1u;
if ((inst.flags & FLAG_SELECTED) != 0u) {
    o.color = mix(o.color, vec3<f32>(1.0, 0.85, 0.2), 0.6);
}
```

In **`cylinder.wgsl`**, find 34h's tint line
`o.color = seg.color * instances[seg.instance_id].color;` — there is no `inst` local, so bind one
(the row is `seg.instance_id`); `o.color` is a `vec4`, so the mix keeps the alpha. Add after it
(the `const` goes at the top of the file):

```wgsl
const FLAG_SELECTED: u32 = 1u;
let inst = instances[seg.instance_id];
if ((inst.flags & FLAG_SELECTED) != 0u) {
    o.color = mix(o.color, vec4<f32>(1.0, 0.85, 0.2, o.color.a), 0.6);
}
```

The same block goes in **`ribbon.wgsl`** after ITS `o.color = seg.color * …;` line (34f's default
edges must tint too, or selection only shows in SOLID mode).

In **`sphere.wgsl`**, the same, but the glyph row is `g.instance_id`. Find
`o.color = g.color * instances[g.instance_id].color;` (34h) and add after it — and repeat in
**`glyph.wgsl`** at its matching line:

```wgsl
const FLAG_SELECTED: u32 = 1u;
let inst = instances[g.instance_id];
if ((inst.flags & FLAG_SELECTED) != 0u) {
    o.color = mix(o.color, vec4<f32>(1.0, 0.85, 0.2, o.color.a), 0.6);
}
```

One bit, three pipelines, and the whole object lights up as a unit — the payoff of routing everything
through the instance table since 29.

## Step 3 — the marquee frustum: `src/camera.rs`

A drag rectangle covers `[x0,x1]×[y0,y1]` in NDC. Compose a **crop matrix** that stretches exactly
that window to the full `[-1,1]` cube, multiply it onto `view_proj`, and the cropped matrix *is* a
view-projection whose frustum is the marquee volume — feed it straight to 41's plane extractor:

```rust
    /// The 6 world-space planes of the sub-frustum under a screen rectangle (NDC coords).
    /// crop · view_proj remaps the rect to the full clip cube, so Gribb–Hartmann (53) needs
    /// no change.
    pub fn marquee_frustum(view_proj: &Xform, origin: &Point,
                           x0: f64, y0: f64, x1: f64, y1: f64) -> Frustum {
        let (sx, sy) = (2.0 / (x1 - x0), 2.0 / (y1 - y0));
        let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let crop = &Xform::translation(-sx * cx, -sy * cy, 0.0) * &Xform::scale_xyz(sx, sy, 1.0);
        Frustum::from_view_proj(&(&crop * view_proj).to_cols()).rebased_to_world(origin)
    }
```

(Convert cursor px → NDC with 46's two formulas; make sure `x0 < x1`, `y0 < y1` after sorting the
drag endpoints, and ignore drags under ~3 px — those are clicks.)

## Step 4 — selection state + the three gestures: `src/app/scene.rs` + `src/state.rs`

`Scene` gets the set and one apply function:

```rust
    pub selected: std::collections::HashSet<u32>,   // ← ADD to Scene (init empty) — ROWS, not guids

    /// Push the current selection into the tables + GPU flags (call after any mutation below).
    pub fn apply_selection(&mut self, gpu: &mut Gpu) {
        gpu.set_selected_rows(&mut self.tables, &self.selected);
    }

    /// Marquee: everything whose world box the sub-frustum accepts. Crossing style —
    /// touching counts.
    pub fn select_marquee(&mut self, f: &Frustum, additive: bool) {
        if !additive { self.selected.clear(); }
        for row in 0..self.world_boxes.len() {
            let (lo, hi) = self.world_boxes[row];
            if f.aabb_visible(lo, hi) { self.selected.insert(row as u32); }
        }
    }
```

(`world_boxes` is 40's row-indexed extents cache — iterate rows directly;
the scan is linear like 41's cull — per *release*, not per frame, so even cheaper. The BVH
the broad-phase IS the at-scale path: run the marquee through 54's `shapecast` with a
three-way test — `Miss` outside the sub-frustum, `Contained` when the node box is fully
inside (the whole subtree is accepted with ZERO further plane tests — this is why a
full-scene marquee costs O(selected), not O(N)), `Intersects` otherwise. The plane math is
identical; only the classification gained a third answer.) One deliberate choice on the
key: the set stores **rows**, not guids. Guids *can* collide across docs — two files may carry the
same guid — so the row is the unambiguous identity, and keying by it also skips a `guid_to_row`
lookup and a `String` clone per hit. Where the UX speaks names (the tree, 75), translate at the
edge: a selected row's guid is just `order[row]`. One consequence to remember: anything that
reorders rows (46's reconcile) must remap or clear this set — 56's snapshots hit the same rule.

In `state.rs`, the mouse gestures — press remembers the spot; release decides click vs drag. The
two are **mutually exclusive**: a plain click is a zero-area rectangle, and `2.0 / (x1 - x0)` on a
zero-width window is a NaN frustum — worse, running the marquee afterward would *clear* the selection
the click just made. So the release branches on drag distance and runs exactly one of them:

<svg viewBox="0 0 560 180" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="on mouse release, distance from the press point decides: under 3 px is a click (replace or toggle), 3 px or more is a marquee; the two branches are mutually exclusive" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="70" y="30" fill="#888" text-anchor="middle">press ●</text>
  <circle cx="70" cy="50" r="4" fill="#6fb3ff"/>
  <text x="70" y="90" fill="#d7dae0" text-anchor="middle">release ●</text>
  <circle cx="70" cy="105" r="4" fill="#d7dae0"/>
  <line x1="70" y1="50" x2="70" y2="105" stroke="#555" stroke-dasharray="3 3"/>
  <text x="88" y="82" fill="#666">drag = |release − press|</text>
  <text x="270" y="58" fill="#888">drag &lt; 3 px</text>
  <line x1="150" y1="52" x2="255" y2="52" stroke="#6fb3ff"/>
  <rect x="345" y="36" width="200" height="34" fill="none" stroke="#5bbf87"/>
  <text x="445" y="52" fill="#5bbf87" text-anchor="middle">CLICK</text>
  <text x="445" y="65" fill="#666" text-anchor="middle" font-size="10">pick_ray → replace / toggle</text>
  <text x="265" y="128" fill="#888">drag ≥ 3 px</text>
  <line x1="150" y1="122" x2="255" y2="122" stroke="#6fb3ff"/>
  <rect x="345" y="106" width="200" height="34" fill="none" stroke="#5bbf87"/>
  <text x="445" y="122" fill="#5bbf87" text-anchor="middle">MARQUEE</text>
  <text x="445" y="135" fill="#666" text-anchor="middle" font-size="10">crop·view_proj → select_marquee</text>
  <text x="280" y="168" fill="#e06c6c" text-anchor="middle">never both — a click has x0==x1 → 2/(x1−x0) = NaN</text>
</svg>

This needs **two new `State` fields** first: add each to `struct State` **and** initialize it in
`State::new`, or it won't compile —

- `pub drag_start: (f64, f64)` — init `(0.0, 0.0)`
- `pub shift: bool` — init `false`; set from **lib.rs**'s `ModifiersChanged` arm, beside the
  `self.ctrl = …` line: `state.shift = mods.state().shift_key();`

Press and release split in **lib.rs** — replace 46's Left-button `MouseInput` arm with:

```rust
            WindowEvent::MouseInput { state: btn, button: MouseButton::Left, .. } => {
                if btn == ElementState::Pressed {
                    state.drag_start = state.cursor;    // press: remember the spot
                } else {
                    state.on_left_release();            // release: click vs marquee
                }
            }
```

and in `state.rs`, rename 47–49's `on_left_click` to `on_left_release`. Its body keeps the
`vp`/`origin`/`viewport`/`ray`/`tol` construction from 46/49, then the whole
pick-and-log block at the end is replaced with the gesture branch:

```rust
        // release: `self.cursor` is where we let go (46's px field); press stashed self.drag_start.
        let (px, py) = self.drag_start;                 // mouse-down, px
        let (mx, my) = self.cursor;                     // release, px
        let shift = self.shift;                         // Shift held? (tracked in ModifiersChanged, like ctrl)
        let drag = (mx - px).hypot(my - py);

        if drag < 3.0 {
            // CLICK: replace (or Shift → toggle). `ray`/`tol` as built at 47/49's pick site.
            match self.scene.pick_ray(&ray, tol) {
                Some(hit) if shift => {
                    if !self.scene.selected.remove(&hit.row) {
                        self.scene.selected.insert(hit.row);
                    }
                }
                Some(hit)          => {
                    self.scene.selected.clear();
                    self.scene.selected.insert(hit.row);
                }
                None if !shift     => { self.scene.selected.clear(); }
                None               => {}
            }
        } else {
            // MARQUEE (Shift → additive). px → NDC (46's two formulas), then sort so x0<x1, y0<y1.
            let (w, h) = (self.gpu.config.width as f64, self.gpu.config.height as f64);
            let ndc = |x: f64, y: f64| (2.0 * x / w - 1.0, 1.0 - 2.0 * y / h);
            let (mut x0, mut y0) = ndc(px, py);
            let (mut x1, mut y1) = ndc(mx, my);
            if x0 > x1 { std::mem::swap(&mut x0, &mut x1); }
            if y0 > y1 { std::mem::swap(&mut y0, &mut y1); }
            let vp = self.camera.view_proj(w / h);
            // the rebase point view_proj was anchored at (33/41) — NOT the eye position
            let origin = self.camera.origin();
            let f = Camera::marquee_frustum(&vp, &origin, x0, y0, x1, y1);
            self.scene.select_marquee(&f, shift);
        }
        self.scene.apply_selection(&mut self.gpu);       // push either result to the GPU flags
```

(Drawing the rubber-band rectangle itself lands with the UI layer: 52's egui overlay paints it from
`drag_start`/`cursor` as one `Painter` rect — see 52 Step 4. Selection is correct with or without
the visual.)

## Step 4b — marquee and clouds

Same contract as picking (55): streamed clouds have no BVH leaf, so no marquee can select
them, and that is correct — there is nothing a gumball or `delete` could do with a
`CloudSlot` today. Their rows DO carry the hidden flag though, so tree-side show/hide
(82) will work on them without any special casing.

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
Ch 49: thin picking — radius + mesh-wins-ties.
Ch 50: SELECTION. State = Scene.selected: HashSet<u32> of ROWS — the set every later tool acts on.
       Rows are the unambiguous key (guids can collide across docs) and save a guid_to_row lookup +
       a String clone per hit; the tree (82) translates via order[row]. Visible
       via FLAG_SELECTED (bit 0, reserved since 35): one bit read by all three instance pipelines
       (triangle/cylinder/sphere) → the whole object tints as a unit; set_selected_rows writes
       tables.objects[row].2 (the truth — set_scene re-derives instances from tables, so the
       selection survives later Msg::File appends) AND pokes flipped live rows via write_row_flags
       (gpu/mod.rs — Instance.flags is engine-private), only flipped rows upload (41's pattern).
       Release branches on drag distance: <3 px CLICK = replace,
       Shift+click = toggle, empty click = clear; >=3 px MARQUEE (never both — a zero-area rect is a
       NaN frustum). MARQUEE: the drag rect is
       remapped to the full clip cube by a crop matrix (translation · scale_xyz), so crop·view_proj
       fed to 41's Frustum::from_view_proj yields the sub-frustum's 6 world planes with zero new
       plane math; select_marquee = aabb_visible over world boxes (linear, per release), crossing
       style.
```

Edited: `engine/gpu/mod.rs` (`FLAG_SELECTED`, `write_row_flags`, `set_selected_rows`), `shaders/*.wgsl` (selection tint),
`camera.rs` (`marquee_frustum` — crop · view_proj), `app/scene.rs` (`selected`, `apply_selection`,
`select_marquee`), `state.rs` (click / Shift+click / marquee gestures).

## Next

`74-hidden-filter.md` — visibility becomes real state. 35 parked a `hidden` set and a flag bit; now
`hide`/`show` actually work end to end: a hidden object doesn't draw (already), doesn't pick, doesn't
marquee — and the little state machine here is the groundwork for the first CLI verbs (61).
