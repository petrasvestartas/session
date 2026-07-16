# 46 Hidden objects — visibility is real state

> **Big picture.** *Phase 7 closes.* Hiding scaffolding to work on what's behind it is a daily CAD
> gesture, and it's the acid test of the state plumbing built since 35: one `hidden` set must be
> respected by **drawing** (37's shader collapse), **picking** (42/44), and **marquee** (45) — three
> different code paths agreeing about one fact. It's also the first *verb* — `hide`/`show` here become
> the first commands the CLI dispatches in 48.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one hidden set feeds three consumers: the shaders skip drawing, the pickers skip hits, the marquee skips membership" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="20" y="40" width="180" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="110" y="61" fill="#d7dae0" text-anchor="middle">Scene.hidden: HashSet&lt;guid&gt;</text>
  <g stroke="#6fb3ff" stroke-width="1.2">
    <line x1="200" y1="47" x2="300" y2="24" marker-end="url(#ah46)"/>
    <line x1="200" y1="57" x2="300" y2="57" marker-end="url(#ah46)"/>
    <line x1="200" y1="67" x2="300" y2="92" marker-end="url(#ah46)"/>
  </g>
  <g fill="none" stroke="#3a3a3a">
    <rect x="304" y="10" width="340" height="26"/><rect x="304" y="44" width="340" height="26"/><rect x="304" y="78" width="340" height="26"/>
  </g>
  <g fill="#d7dae0" font-size="10">
    <text x="314" y="27">draw — FLAG_HIDDEN → vs collapses to w=0 (37, already wired)</text>
    <text x="314" y="61">pick — 42/44 skip hidden guids (else you select what you can't see)</text>
    <text x="314" y="95">marquee — 45 skips hidden guids (else drags resurrect ghosts)</text>
  </g>
  <defs><marker id="ah46" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs   # set_flag_rows(flag, rows) — the generalization of 45's set_selected_rows
src/app/scene.rs        # hide_selected / show_all / apply_visibility; pickers + marquee skip hidden
src/state.rs            # H = hide selection, Ctrl+H = show all (keyboard until the CLI, 48)
```

## Step 1 — generalize the flag upload: `src/engine/gpu/mod.rs`

45's `set_selected_rows` and the visibility upload are the same function with a different bit. Extract
it — this is the moment the pattern earns a name:

```rust
    /// Set `flag` on exactly `rows`, clear it everywhere else; upload only rows that flipped.
    /// One function serves SELECTED (45) and HIDDEN (here) — CULLED (37) keeps its own per-frame
    /// path.
    pub fn set_flag_rows(&mut self, flag: u32, rows: &std::collections::HashSet<u32>) {
        for i in 0..self.instances.len() as u32 {
            let want = rows.contains(&i);
            let has = self.instances[i as usize].flags & flag != 0;
            if want != has {
                self.write_row(i, |inst| inst.flags ^= flag);
            }
        }
    }
```

and 45's function becomes a wrapper (find `set_selected_rows`, replace its body):

```rust
    pub fn set_selected_rows(&mut self, rows: &std::collections::HashSet<u32>) {
        self.set_flag_rows(Instance::FLAG_SELECTED, rows);
    }
```

The shader side needs **nothing**: 37 Step 3 already collapses any `FLAG_HIDDEN` row to a clipped
vertex in all four pipelines. Drawing was solved before hiding existed — that's the instance-flag
architecture paying out again.

## Step 2 — the verbs: `src/app/scene.rs`

`hidden` has sat on `Scene` since 35 (it fed `build()`'s initial flags); now it changes at runtime.
Hiding also drops the objects from the selection — you can't act on what you can't see (Rhino's rule),
and it prevents a hidden object riding along in a later gumball drag:

```rust
    /// Push `hidden` to the GPU flags — same flip-tracked upload as selection.
    pub fn apply_visibility(&self, gpu: &mut Gpu) {
        let rows = self.hidden.iter().filter_map(|g| self.guid_to_row.get(g).copied()).collect();
        gpu.set_flag_rows(Instance::FLAG_HIDDEN, &rows);
    }

    /// The first verb: hide whatever is selected. Hidden objects leave the selection.
    pub fn hide_selected(&mut self, gpu: &mut Gpu) {
        for g in self.selected.drain() { self.hidden.insert(g); }
        self.apply_visibility(gpu);
        self.apply_selection(gpu);
    }

    /// The inverse verb. (Per-object `show <name>` arrives with the CLI, 48.)
    pub fn show_all(&mut self, gpu: &mut Gpu) {
        self.hidden.clear();
        self.apply_visibility(gpu);
    }
```

## Step 3 — the pickers respect it: `src/app/scene.rs`

Three one-line guards, one per path:

```rust
    // pick_mesh (42) — inside the candidate loop, first line:
    if self.hidden.contains(&guid) { continue; }

    // pick_thin (44) — inside the hits loop, before the match:
    if self.hidden.contains(h.guid()) { continue; }

    // select_marquee (45) — inside the order loop, first line:
    if self.hidden.contains(guid) { continue; }
```

Without these, clicking where a hidden object *was* selects it invisibly — and the next `hide`
gesture acts on a ghost. The bug class this prevents is "the viewport and the state disagree", which
is exactly what Phase 7 exists to prevent.

## Step 4 — keys until the CLI: `src/state.rs`

In the keyboard handler (next to `F`):

```rust
                        Key::Character("h" | "H") if !ctrl =>
                            { state.scene.hide_selected(&mut state.gpu); }
                        Key::Character("h" | "H") if ctrl  =>
                            { state.scene.show_all(&mut state.gpu); }
```

(48 rebinds these to real `hide` / `show` commands on the bus; the `Scene` verbs don't change.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select a box in front of another, press **H** → it vanishes; the yellow tint vanishes with it (it
  left the selection). **Click where it was** → the object *behind* it picks — the ray flew through.
- **Marquee across the hidden object** → it stays unselected; Ctrl+H → it reappears, unselected.
- Perf HUD: hiding is a couple of `write_row` uploads, not a rebuild — the frame cost doesn't move.
- The trap this catches: comment out one of Step 3's guards and repeat — the hidden object silently
  joins the selection and reappears highlighted on Ctrl+H. Three consumers, one set, no exceptions.

## Recap

```
Ch 45: selection state + marquee.
Ch 46: VISIBILITY. Scene.hidden (parked since 35) becomes runtime state with three consumers that
       must agree: draw (FLAG_HIDDEN → w=0 collapse, wired in 37 — zero new shader work), pick
       (42/44 skip hidden guids), marquee (45 skips). set_flag_rows(flag, rows) generalizes 45's
       flip-tracked upload — selection and visibility are the same mechanism, different bit.
       hide_selected drains the selection into hidden (can't act on what you can't see); show_all
       clears. H / Ctrl+H until the CLI takes over (48). Phase 7 complete: ray → hit →
       sub-object → thin → selection → visibility.
```

Edited: `engine/gpu/mod.rs` (`set_flag_rows`, `set_selected_rows` → wrapper), `app/scene.rs`
(`apply_visibility`, `hide_selected`, `show_all`, 3 picker guards), `state.rs` (H / Ctrl+H).

## Next

`47-egui-hud.md` — Phase 8: the interface. An egui overlay lands on top of the 3-D pass — the perf HUD
graduates from the console to a real panel, checkboxes toggle grid/edges/projection, and a slider
drives 31's line thickness. The rule that keeps it sane: the UI *collects intent*, and `State` applies
it after the closure — never mutate mid-layout.
