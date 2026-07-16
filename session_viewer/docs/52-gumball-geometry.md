# 52 Gumball I — the widget appears

> **Big picture.** *Phase 9 — transform & draw (52–59).* Everything so far *inspects* the scene; this
> phase finally *changes* it. The gumball — the 3-axis move/rotate/scale gizmo every CAD app centers
> its editing on — arrives over four lessons: geometry (here), screen-constant scale + hit-testing
> (53), drag-to-translate with undo (54), rotate/scale (55). The payoff of Phases 4–8 shows
> immediately: the widget is built from the SAME `CylinderSegment`/`GlyphPoint` rows as the scene,
> picked with the same ray, and its edits will commit as 51's `Command`s.

<svg viewBox="0 0 680 190" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the gumball: three axis arrows with cone tips, three quarter-circle rotate arcs on the negative sides, four scale spheres, all centered on the selection centroid" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(200,105)">
    <line x1="0" y1="0" x2="130" y2="-20" stroke="#e05555" stroke-width="3"/><path d="M130,-20 l18,-3 l-14,-10 z" fill="#e05555"/>
    <line x1="0" y1="0" x2="-30" y2="-115" stroke="#4fae5c" stroke-width="3"/><path d="M-30,-115 l-5,-17 l-9,15 z" fill="#4fae5c"/>
    <line x1="0" y1="0" x2="-95" y2="55" stroke="#4f7dd0" stroke-width="3"/><path d="M-95,55 l-16,9 l16,7 z" fill="#4f7dd0"/>
    <path d="M -55,32 A 64,64 0 0 1 -18,-62" fill="none" stroke="#e05555" stroke-width="2.2"/>
    <path d="M 72,-11 A 64,64 0 0 1 -14,-63" fill="none" stroke="#4f7dd0" stroke-width="2.2" opacity="0.85"/>
    <circle cx="0" cy="0" r="7" fill="#dcdcdc"/>
    <circle cx="-58" cy="9" r="5.5" fill="#e05555"/><circle cx="14" cy="57" r="5.5" fill="#4fae5c"/><circle cx="47" cy="27" r="5.5" fill="#4f7dd0"/>
  </g>
  <g fill="#888">
    <text x="420" y="40">arrows → Translate X/Y/Z (shaft + cone tip)</text>
    <text x="420" y="64">arcs → Rotate, on the NEGATIVE axis side</text>
    <text x="420" y="88">axis spheres → Scale X/Y/Z (at −arc/2)</text>
    <text x="420" y="112">white center sphere → uniform Scale</text>
    <text x="420" y="144" fill="#666">all of it: CylinderSegment + GlyphPoint rows,</text>
    <text x="420" y="160" fill="#666">drawn in an overlay pass with CLEARED depth</text>
  </g>
</svg>

## Files we touch

```
src/engine/gumball.rs   # NEW — HandleKind, tuning consts, geometry gen (segments + glyphs)
src/engine/gpu/mod.rs   # gumball segment/glyph buffers + the overlay pass (depth cleared)
src/app/scene.rs        # selection_centroid() — where the widget sits
src/state.rs            # rebuild the gumball when the selection changes
```

## Step 1 — handles + tuning: `src/engine/gumball.rs` (NEW)

Every part of the widget belongs to exactly one **handle** — the id the hit-test (53) returns and the
drag math (54/55) switches on. The constants are the archive's tuned values, not guesses:

```rust
//! The gumball: pure geometry + ids. No wgpu here — it EMITS the same CylinderSegment/GlyphPoint
//! rows lessons 31/32 defined, and the gpu draws them in an overlay pass.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HandleKind {
    TranslateX, TranslateY, TranslateZ,
    RotateX, RotateY, RotateZ,
    ScaleX, ScaleY, ScaleZ,
    ScaleUniform,
}

pub const ARROW_LEN: f32 = 150.0;     // shaft length, gumball-local units (scaled per frame, 53)
pub const ARROW_CAP: f32 = 18.0;      // cone-tip length
pub const ARC_RADIUS: f32 = 150.0;    // rotate arcs sit at the arrow length
pub const SPHERE_R: f32 = 8.0;        // all four scale spheres
pub const ARC_SEGS: usize = 64;       // arc smoothness
pub const SHAFT_R: f32 = 2.5;         // world-radius override for segments (radius > 0, 31)

pub const AXIS_COLORS: [[f32; 4]; 3] = [
    [0.88, 0.33, 0.33, 1.0],   // X red
    [0.31, 0.68, 0.36, 1.0],   // Y green
    [0.31, 0.49, 0.82, 1.0],   // Z blue
];
```

## Step 2 — geometry generation: `src/engine/gumball.rs`

One function turns an origin + scale into rows. Arrow shafts are single segments; arcs are `ARC_SEGS`
short segments (the 31 path renders any polyline); spheres are glyphs. Each emitted row carries a
**handle tag** so 53 can map "what did the ray hit" back to a `HandleKind`:

```rust
pub struct GumballGeom {
    pub segments: Vec<(CylinderSegment, HandleKind)>,
    pub glyphs: Vec<(GlyphPoint, HandleKind)>,
}

/// Build the widget at `o`, scaled by `s` (53 computes s for constant screen size; use 1.0 for now).
/// `row` is a reserved instance row with an identity model — the gumball is world-anchored.
pub fn build(o: [f32; 3], s: f32, row: u32) -> GumballGeom {
    let axes = [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let t_kinds = [HandleKind::TranslateX, HandleKind::TranslateY, HandleKind::TranslateZ];
    let r_kinds = [HandleKind::RotateX, HandleKind::RotateY, HandleKind::RotateZ];
    let s_kinds = [HandleKind::ScaleX, HandleKind::ScaleY, HandleKind::ScaleZ];
    let mut g = GumballGeom { segments: Vec::new(), glyphs: Vec::new() };

    for i in 0..3 {
        let (a, c) = (axes[i], AXIS_COLORS[i]);
        let tip   = [o[0] + a[0]*ARROW_LEN*s, o[1] + a[1]*ARROW_LEN*s, o[2] + a[2]*ARROW_LEN*s];
        let neck  = [o[0] + a[0]*(ARROW_LEN-ARROW_CAP)*s, o[1] + a[1]*(ARROW_LEN-ARROW_CAP)*s, o[2] + a[2]*(ARROW_LEN-ARROW_CAP)*s];
        // shaft (the cone tip is a fattened final segment — see the note below)
        g.segments.push((CylinderSegment { p0: o, radius: SHAFT_R*s, p1: neck, instance_id: row, color: c }, t_kinds[i]));
        g.segments.push((CylinderSegment { p0: neck, radius: SHAFT_R*s*2.5, p1: tip, instance_id: row, color: c }, t_kinds[i]));

        // rotate arc: a quarter circle in the plane PERPENDICULAR to axis i, on the NEGATIVE side
        // (opposite the arrow, so arcs and arrows never overlap visually or in the hit-test).
        let (u, v) = (axes[(i + 1) % 3], axes[(i + 2) % 3]);
        let start = std::f32::consts::FRAC_PI_2;                     // archive convention
        let mut prev: Option<[f32; 3]> = None;
        for k in 0..=ARC_SEGS {
            let t = start + std::f32::consts::FRAC_PI_2 * (k as f32 / ARC_SEGS as f32);
            let p = [
                o[0] + (u[0]*t.cos() + v[0]*t.sin()) * ARC_RADIUS * s,
                o[1] + (u[1]*t.cos() + v[1]*t.sin()) * ARC_RADIUS * s,
                o[2] + (u[2]*t.cos() + v[2]*t.sin()) * ARC_RADIUS * s,
            ];
            if let Some(q) = prev {
                g.segments.push((CylinderSegment { p0: q, radius: SHAFT_R*s, p1: p, instance_id: row, color: c }, r_kinds[i]));
            }
            prev = Some(p);
        }

        // axis-scale sphere: on the negative axis at half the arc radius (archive placement)
        let sp = [o[0] - a[0]*ARC_RADIUS*0.5*s, o[1] - a[1]*ARC_RADIUS*0.5*s, o[2] - a[2]*ARC_RADIUS*0.5*s];
        g.glyphs.push((GlyphPoint { center: sp, radius: SPHERE_R*s, color: c, instance_id: row, _pad: [0; 3] }, s_kinds[i]));
    }
    // uniform-scale sphere: white, at the origin
    g.glyphs.push((GlyphPoint { center: o, radius: SPHERE_R*s, color: [0.86, 0.86, 0.86, 1.0], instance_id: row, _pad: [0; 3] }, HandleKind::ScaleUniform));
    g
}
```

> **Cone tips without a cone pipeline.** The archive ships a dedicated `cone.wgsl` (a cylinder shader
> whose radius tapers with `0.5 - local_z`). That's the polished look; the two-segment arrow above
> (thin shaft + a 2.5× fat cap segment) reads correctly at gumball size and costs zero new pipelines.
> If you want true cones later: copy `cylinder.wgsl`, replace the constant radius with
> `r * (0.5 - lp.z)` — one line.

Note the `radius: SHAFT_R*s` — a **positive** radius, which 31 defined as the world-mm override. The
gumball must *not* track the global thickness slider; its proportions are its identity.

## Step 3 — the overlay pass: `src/engine/gpu/mod.rs`

The gumball must float over geometry (you grab it even when the selection is buried), yet its own
parts must depth-test against *each other* (the far arc behind the near shaft). The standard trick:
draw it **last, in a pass that clears only the depth buffer** — color loads, depth restarts:

```rust
    // gumball tables — small, rebuilt whole whenever selection/camera changes (they're ~400 rows)
    pub gb_segments: Vec<CylinderSegment>,
    pub gb_glyphs: Vec<GlyphPoint>,
    // + gb_segment_buffer / gb_glyph_buffer / bind groups — same pattern as 31/32's, small fixed capacity
```

```rust
        // ---- gumball overlay pass (after the main passes, before egui) ----
        if self.gb_segment_count > 0 {
            let mut gp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gumball"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa_view, resolve_target: Some(&view), depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),   // ← CLEARED (reverse-Z far)
                    stencil_ops: None,
                }),
                occlusion_query_set: None, timestamp_writes: None, multiview_mask: None,
            });
            // same cylinder + sphere pipelines, bind groups 0-2 as the main pass, group 3 = gumball buffers
            gp.set_pipeline(&self.pipelines.cylinder);
            /* …bind groups, template buffers, draw_indexed over gb_segment_count… */
            gp.set_pipeline(&self.pipelines.sphere);
            /* …same, over gb_glyph_count… */
        }
```

(Reverse-Z (26): depth clears to `0.0`, the far plane. `upload_gumball(&GumballGeom)` strips the
handle tags, writes the two buffers, and keeps the tagged copy on `State` for 53's hit-test.)

## Step 4 — where it sits: `src/app/scene.rs` + `src/state.rs`

The widget anchors at the **selection centroid** — the average of the selected objects' world-box
centers (the archive's hard-won rule: use the *box center*, not the object's anchor, or a cylinder's
gumball sits at its base):

```rust
    /// World centroid of the selection — the gumball anchor. Box CENTERS, not object anchors.
    pub fn selection_centroid(&self) -> Option<[f32; 3]> {
        if self.selected.is_empty() { return None; }
        let (mut c, mut n) = ([0.0f64; 3], 0.0);
        for g in &self.selected {
            let (lo, hi) = self.world_aabb(g);                     // 37's helper
            for k in 0..3 { c[k] += (lo[k] + hi[k]) * 0.5; }
            n += 1.0;
        }
        Some([(c[0]/n) as f32, (c[1]/n) as f32, (c[2]/n) as f32])
    }
```

In `state.rs`, whenever the selection changes (the three gesture sites in 45 + hide in 46), rebuild:

```rust
    fn refresh_gumball(&mut self) {
        match self.scene.selection_centroid() {
            Some(o) => { let g = crate::engine::gumball::build(o, 1.0, GB_ROW); self.gb = Some(g); self.gpu.upload_gumball(self.gb.as_ref().unwrap()); }
            None => { self.gb = None; self.gpu.clear_gumball(); }
        }
    }
```

(`GB_ROW` is one reserved instance row with an identity model, created at startup — the gumball is
world-anchored and never rebased wrong because 33's rebase writes models, and identity + world coords
is exactly the convention lines already use.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select an object → the gumball appears at its **volume center** (select a tall cylinder — the
  widget must sit at mid-height, not the base). Select three objects → it sits at their common
  centroid. Deselect → gone.
- Axes read **X red, Y green, Z blue**, arcs on the *opposite* side of each arrow, four spheres
  (three colored + white center).
- Orbit until geometry passes *through* the widget → the gumball stays fully visible (the cleared
  depth), but its own near parts still occlude its far parts.
- It's tiny far away and huge up close — correct, and exactly what 53 fixes next.

## Recap

```
Ch 51: undo — every mutation is a Command. Phase 8 done.
Ch 52: GUMBALL GEOMETRY. HandleKind = the id everything keys on (4 translate/rotate/scale groups,
       10 handles). build(origin, scale, row) emits tagged CylinderSegment/GlyphPoint rows — NO new
       pipelines: shafts/arcs are 31 segments (positive radius = world-mm, immune to the thickness
       slider; fat last segment ≈ cone tip, real cone = one shader line later), spheres are 32
       glyphs. Archive constants: ARROW 150/18, ARC 150 on the NEGATIVE side (no overlap with
       arrows), spheres 8 at −arc/2 + white center. Drawn LAST in an overlay pass: color Load, depth
       CLEAR(0.0 — reverse-Z) → floats over the scene, self-occludes correctly. Anchor =
       selection_centroid from world-box CENTERS (archive bug: anchors put a cylinder's gumball at
       its base). Rebuilt on every selection change.
```

Edited: `engine/gumball.rs` (NEW — `HandleKind`, constants, `build`), `engine/gpu/mod.rs` (gumball
buffers + overlay pass), `app/scene.rs` (`selection_centroid`), `state.rs` (`refresh_gumball`).

## Next

`53-gumball-scale-hittest.md` — two fixes that make it usable: **constant screen size** (the scale
factor from view-space Z depth — the Euclidean-distance version breaks during orbit, a real archive
bug) and **hit-testing** (ray → nearest handle, tested BEFORE scene picking, with hover highlight).
