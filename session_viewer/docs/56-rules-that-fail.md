# 56 Rules that fail the build

> ARCHITECTURE.md lists five rules the code is supposed to obey. Nothing checks them.
> This lesson makes four of them tests, and the fourth one finds seven violations immediately -
> including the exact function where a real bug already happened.

## 1. Why prose is not enforcement

The five rules are true statements about this codebase and they were verified by hand, with
ad-hoc greps, by someone who wanted them to be true. That is not a check. A rule nobody runs is
a rule the next change breaks silently, and the next reader inherits as a lie.

`cargo xtest` already runs in CI. A rule written as a test is a rule the next change has to keep,
and - this matters more - a rule that has to be EDITED is a decision someone made on purpose.
Deliberate drift is the only kind worth having.

## 2. What the rules cost when they are not checked

Rule 5 says an option the caller must decide is a named FIELD, not another positional argument.
Written down, checked by eye, it read as satisfied. Written as a test, it failed on seven
signatures - and the worst of them was this:

`fn mk_splat_group1(device, layout, pos, col, nrm, sdepth, scolor)`.

Five consecutive `&wgpu::Buffer`. The only thing distinguishing them at a call site is position -
and the bind group it builds binds them in a DIFFERENT order than the parameter list reads:
`pos, col, sdepth, scolor, nrm`, normals last. The stream lane spent five lessons passing its
per-pixel buffers where its normals belonged. Nothing was ill-typed. Nothing looked wrong. The
frame was simply darker than it should have been, and no test could see it.

A named field can be written in the wrong ORDER and still be right. A positional argument cannot.

## 3. The two structs

**Find** in `src/engine/gpu/splat.rs`:

```rust
/// The two per-pixel buffers both lanes contest, and the group the resolve pass reads them with.
```

**Add below it:**

```rust
/// One point lane's three tables, by NAME.
///
/// They are three `&wgpu::Buffer` of the same type, and for five lessons the only thing telling
/// them apart at a call site was argument position - which is exactly how the stream lane came to
/// bind its per-pixel buffers where its normals belonged. Nothing in the type system could catch
/// it, and nothing in the frame looked wrong until someone measured the ink. A named field can be
/// written in the wrong ORDER and still be right; a positional argument cannot.
#[derive(Clone, Copy)]
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

/// What every lane binds identically: the frame's two uniforms and the object table.
#[derive(Clone, Copy)]
pub struct SharedBufs<'a> {
    pub mvp: &'a wgpu::Buffer,
    pub cloud: &'a wgpu::Buffer,
    pub instances: &'a wgpu::Buffer,
}

```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer, &PixelBufs),
    ) -> Self {
        let recs = zeroed_buffer(device, label, 16 + MAX_RECORDS * REC_WORDS * 4,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let (mvp, cloud, instances, pixels) = shared;
        let (pos, col, nrm) = points;
        Self {
            group0: mk_splat_group0(device, &layouts.splat_group0, mvp, cloud, instances, &recs),
            group1: mk_splat_group1(device, &layouts.splat_group1, pos, col, nrm, &pixels.depth, &pixels.color),
```

**Replace with:**

```rust
        points: PointBufs,
        shared: SharedBufs,
        pixels: &PixelBufs,
    ) -> Self {
        let recs = zeroed_buffer(device, label, 16 + MAX_RECORDS * REC_WORDS * 4,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        Self {
            group0: mk_splat_group0(device, &layouts.splat_group0, shared, &recs),
            group1: mk_splat_group1(device, &layouts.splat_group1, points, pixels),
```

## 4. The call sites become field literals

**Find** in `src/engine/gpu/splat.rs`:

```rust
        points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer, &PixelBufs),
    ) {
        let (mvp, cloud, instances, pixels) = shared;
        let (pos, col, nrm) = points;
        self.group0 = mk_splat_group0(device, &layouts.splat_group0, mvp, cloud, instances, &self.recs);
        self.group1 = mk_splat_group1(device, &layouts.splat_group1, pos, col, nrm, &pixels.depth, &pixels.color);
```

**Replace with:**

```rust
        points: PointBufs,
        shared: SharedBufs,
        pixels: &PixelBufs,
    ) {
        self.group0 = mk_splat_group0(device, &layouts.splat_group0, shared, &self.recs);
        self.group1 = mk_splat_group1(device, &layouts.splat_group1, points, pixels);
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    mvp: &wgpu::Buffer,
    cloud: &wgpu::Buffer,
    instances: &wgpu::Buffer,
    recs: &wgpu::Buffer
```

**Replace with:**

```rust
    b: SharedBufs,
    recs: &wgpu::Buffer,
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
            wgpu::BindGroupEntry{binding: 0, resource: mvp.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: cloud.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: instances.as_entire_binding()},
```

**Replace with:**

```rust
            wgpu::BindGroupEntry{binding: 0, resource: b.mvp.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: b.cloud.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: b.instances.as_entire_binding()},
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    pos: &wgpu::Buffer,
    col: &wgpu::Buffer,
    nrm: &wgpu::Buffer,
    sdepth: &wgpu::Buffer,
    scolor: &wgpu::Buffer,
```

**Replace with:**

```rust
    p: PointBufs,
    px: &PixelBufs,
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
            wgpu::BindGroupEntry{binding: 0, resource: pos.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: col.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: sdepth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: scolor.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 4, resource: nrm.as_entire_binding()},
```

**Replace with:**

```rust
            // NOTE the order: the shader binds the normals at 4, AFTER the two pixel buffers -
            // it does not match the order a reader would guess from the field list. That
            // mismatch is what the old five-positional-argument signature hid.
            wgpu::BindGroupEntry{binding: 0, resource: p.pos.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: p.col.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: px.depth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: px.color.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 4, resource: p.nrm.as_entire_binding()},
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        walked_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        stream_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) -> Self {
        let pixels = PixelBufs::new(device, layouts, width, height);
        let (mvp, cloud, instances) = shared;
        Self {
            walked: SplatSlot::new(device, layouts, "splat.rescales", walked_points, (mvp, cloud, instances, &pixels)),
            stream: SplatSlot::new(device, layouts, "splat.stream.recs", stream_points, (mvp, cloud, instances, &pixels)),
```

**Replace with:**

```rust
        walked_points: PointBufs,
        stream_points: PointBufs,
        shared: SharedBufs,
    ) -> Self {
        let pixels = PixelBufs::new(device, layouts, width, height);
        Self {
            walked: SplatSlot::new(device, layouts, "splat.rescales", walked_points, shared, &pixels),
            stream: SplatSlot::new(device, layouts, "splat.stream.recs", stream_points, shared, &pixels),
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        walked_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        stream_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) {
        let (mvp, cloud, instances) = shared;
        self.walked.rebind(device, layouts, walked_points, (mvp, cloud, instances, &self.pixels));
        self.stream.rebind(device, layouts, stream_points, (mvp, cloud, instances, &self.pixels));
```

**Replace with:**

```rust
        walked_points: PointBufs,
        stream_points: PointBufs,
        shared: SharedBufs,
    ) {
        self.walked.rebind(device, layouts, walked_points, shared, &self.pixels);
        self.stream.rebind(device, layouts, stream_points, shared, &self.pixels);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use splat::Splat;
```

**Replace with:**

```rust
use splat::{PointBufs, SharedBufs, Splat};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat = Splat::new(&device, &layouts, config.width, config.height,
            (&cloud.pos, &cloud.col, &cloud.nrm), (&stream.pos, &stream.col, &stream.nrm),
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));

```

**Replace with:**

```rust
        let splat = Splat::new(&device, &layouts, config.width, config.height,
            PointBufs { pos: &cloud.pos, col: &cloud.col, nrm: &cloud.nrm },
            PointBufs { pos: &stream.pos, col: &stream.col, nrm: &stream.nrm },
            SharedBufs { mvp: &frame.mvp_buffer, cloud: &frame.cloud_buffer, instances: &objects.buffer });

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            (&cloud.pos, &cloud.col, &cloud.nrm), (&stream.pos, &stream.col, &stream.nrm),
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));
```

**Replace with:**

```rust
            PointBufs { pos: &cloud.pos, col: &cloud.col, nrm: &cloud.nrm },
            PointBufs { pos: &stream.pos, col: &stream.col, nrm: &stream.nrm },
            SharedBufs { mvp: &frame.mvp_buffer, cloud: &frame.cloud_buffer, instances: &objects.buffer });
```

## 5. The rules

Four of the five. Rule 1 - a family may not build or renumber an object row - is the one rule
here that cannot be read off the source shape: `ObjectBase` rows are pushed by the walk, which
is allowed, and telling that apart from a family doing it needs to know who the caller is.

Note what the size test says about itself. It is a RATCHET, not a target. Line count does not
define good architecture - `splat.rs` is 419 lines of one coherent thing and splitting it would
make this code worse. What a budget catches is drift: outgrow it and the build fails, and someone
has to either split the file or raise the number ON PURPOSE, next to a reason. When this lesson
added the two structs above, `splat.rs` outgrew its budget and the test failed. The budget moved
to 470 with a note. That is the mechanism working, not a workaround.

**Create `src/architecture.rs`**:

```rust
//! `architecture.rs` — the five rules in ARCHITECTURE.md, as tests instead of prose.
//!
//! Every one of these was true when it was written, and every one was verified by hand with an
//! ad-hoc grep, which is another way of saying nobody was checking. Prose does not fail a build.
//! `cargo xtest` already runs in CI, so a rule written here is a rule the next change has to
//! keep - and a rule that has to be EDITED here is a decision someone made on purpose, which is
//! the only kind of architectural drift worth having.
//!
//! These read the crate's own source. That is deliberate: the shape of the code is the thing
//! under test, and no amount of runtime behaviour would catch a family reaching into another
//! family's buffers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn src() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("src") }

fn read(rel: &str) -> String {
    let p = src().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

/// Every `.rs` under `src/`, as (path-relative-to-src, contents).
fn all_rs() -> Vec<(String, String)> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());
        for e in entries {
            let p = e.path();
            if p.is_dir() { walk(&p, root, out); }
            else if p.extension().is_some_and(|x| x == "rs") {
                let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
                out.push((rel, std::fs::read_to_string(&p).unwrap()));
            }
        }
    }
    let root = src();
    let mut out = Vec::new();
    walk(&root, &root, &mut out);
    out
}

/// RULE 3 - the frame is an ordered LIST. `render.rs` says WHAT is drawn and in what order; it
/// never says HOW. The moment it names a buffer or a shader it has reached past a family into
/// that family's business, and the ordering stops being readable in one place.
#[test]
fn frame_names_no_buffer_or_shader() {
    let t = read("engine/gpu/render.rs");
    let bad: Vec<_> = t
        .lines()
        .enumerate()
        .filter(|(_, l)| l.contains("wgpu::Buffer") || l.contains(".wgsl"))
        .map(|(i, l)| format!("  render.rs:{}: {}", i + 1, l.trim()))
        .collect();
    assert!(bad.is_empty(), "render.rs must name no buffer and no shader:\n{}", bad.join("\n"));
}

/// RULE 2 - a module is defined by the ROW it owns. A family file may not name another family's
/// lane type: that is the difference between "the segment lane owns segments" and "everything
/// knows about everything". `mod.rs` is exempt because assembling the lanes is its whole job,
/// and so are the files that exist to span lanes.
#[test]
fn a_family_does_not_name_another_family() {
    const LANES: &[(&str, &str)] = &[
        ("arena.rs", "Arena"),
        ("segments.rs", "SegmentLane"),
        ("glyphs.rs", "GlyphLane"),
        ("cloud.rs", "CloudLane"),
        ("stream.rs", "StreamLane"),
    ];
    // These legitimately span families: they build, order or drive all of them.
    const SPANNING: &[&str] = &["mod.rs", "render.rs", "present.rs", "frame.rs", "device.rs"];
    let mut bad = Vec::new();
    for (file, _) in LANES {
        let t = read(&format!("engine/gpu/{file}"));
        for (other, ty) in LANES {
            if other == file || SPANNING.contains(file) { continue }
            if t.contains(ty) {
                bad.push(format!("  engine/gpu/{file} names {ty}, which belongs to {other}"));
            }
        }
    }
    assert!(bad.is_empty(), "a family may not name another family's lane:\n{}", bad.join("\n"));
}

/// Signatures already over the line when the rule was written, each with the reason it is still
/// here. This list may SHRINK freely; a new entry, or a bigger number on an old one, is a
/// deliberate act someone has to write down.
///
/// All three are the same shape underneath: values that travel together are being passed apart.
const KNOWN_WIDE: &[(&str, usize, &str)] = &[
    ("engine/gpu/buffers.rs:88", 6,
     "append_rows takes buf/count/cap loose - which is exactly `GrowBuf`. The struct EXISTS; the \
      lanes that call this (cloud.rs, objects.rs) just do not use it, keeping parallel fields \
      instead. Fixing the signature means converting those lanes first."),
    ("engine/gpu/splat.rs:122", 6,
     "SplatSlot::new - device, layouts, label, points, shared, pixels. The three buffer groups \
      are already named structs; what is left is genuinely six independent things."),
    ("engine/gpu/splat.rs:224", 7,
     "Splat::new - the same, plus width and height for the pixel buffers it allocates."),
];

/// RULE 5 - an option the caller must decide is a named FIELD, not another positional argument.
/// Six parameters is where a call site stops being readable and starts being a puzzle about
/// which `true` meant what. `MeshOpts` exists because `push_mesh` had eight.
#[test]
fn no_function_takes_more_than_five_parameters() {
    let mut bad = Vec::new();
    for (rel, t) in all_rs() {
        for (i, line) in t.lines().enumerate() {
            let l = line.trim_start();
            if !l.starts_with("fn ") && !l.starts_with("pub fn ") && !l.starts_with("pub(crate) fn ")
                && !l.starts_with("async fn ") && !l.starts_with("pub async fn ") { continue }
            // Only single-line signatures are counted here; a wrapped one is counted by the
            // block below, which walks to the closing paren.
            // Skip a generic block before the parameter list: `fn f<T: Trait>(a, b)` would
            // otherwise start counting at the `<` and score the TRAIT BOUNDS as parameters.
            let after_name = match l.find(|c: char| c == '<' || c == '(') { Some(k) => k, None => continue };
            let l = if l.as_bytes()[after_name] == b'<' {
                let mut d = 0i32;
                let mut end = after_name;
                for (k, ch) in l[after_name..].char_indices() {
                    match ch { '<' => d += 1, '>' => { d -= 1; if d == 0 { end = after_name + k + 1; break } }, _ => {} }
                }
                &l[end..]
            } else { l };
            let open = match l.find('(') { Some(k) => k, None => continue };
            let mut depth = 0i32;
            let mut params = String::new();
            let mut done = false;
            for ch in l[open..].chars() {
                match ch { '(' | '[' | '<' => depth += 1, ')' | ']' | '>' => { depth -= 1; if depth == 0 { done = true; break } }, _ => {} }
                params.push(ch);
            }
            if !done {
                // wrapped signature: gather following lines until depth returns to zero
                for cont in t.lines().skip(i + 1) {
                    for ch in cont.chars() {
                        match ch { '(' | '[' | '<' => depth += 1, ')' | ']' | '>' => { depth -= 1; if depth == 0 { done = true; break } }, _ => {} }
                        params.push(ch);
                    }
                    if done { break }
                }
            }
            // Count SEGMENTS, not commas: Rust allows a trailing comma after the last
            // parameter, and counting separators scored every wrapped signature one too high.
            let mut d = 0i32;
            let mut seg = String::new();
            let mut segs: Vec<String> = Vec::new();
            for ch in params.chars().skip(1) {
                match ch { '(' | '[' | '<' => d += 1, ')' | ']' | '>' => d -= 1, _ => {} }
                if ch == ',' && d == 0 { segs.push(std::mem::take(&mut seg)); } else { seg.push(ch); }
            }
            segs.push(seg);
            let count = segs.iter().filter(|s| !s.trim().is_empty()).count();
            // `&self` is not a decision the caller makes.
            let count = if params.contains("self") { count.saturating_sub(1) } else { count };
            if count > 5 {
                let here = format!("{rel}:{}", i + 1);
                match KNOWN_WIDE.iter().find(|(w, _, _)| *w == here) {
                    // Recorded debt may not GROW. Shrinking it means editing the number here,
                    // which is the point: the list can only get shorter on purpose.
                    Some((_, allowed, _)) if count <= *allowed => {}
                    Some((_, allowed, _)) => bad.push(format!(
                        "  {here}: {count} parameters, was {allowed} — it grew; fix it or say why")),
                    None => bad.push(format!(
                        "  {here}: {count} parameters — {}", l.split('(').next().unwrap().trim())),
                }
            }
        }
    }
    assert!(bad.is_empty(), "an option the caller decides is a named field, not a 6th argument:\n{}", bad.join("\n"));
}

/// A RATCHET, not a target. Line count does not define good architecture - `splat.rs` is 419
/// lines of one coherent thing and splitting it would make the code worse. What a budget is for
/// is drift: a file that grows past its entry here fails, and someone has to either split it or
/// raise the number ON PURPOSE. The number moving is the signal; its value is not a goal.
#[test]
fn no_file_grows_without_someone_deciding() {
    const BUDGET: &[(&str, usize)] = &[
        ("engine/gpu/splat.rs", 470), // +40: PointBufs/SharedBufs, so a bind group cannot be built positionally
        ("engine/gpu/mod.rs", 390),
        ("camera.rs", 370),
        ("app/walk/mesh_ink.rs", 360),
        ("engine/gpu/segments.rs", 350),
        ("engine/gpu/objects.rs", 350),
        ("engine/gpu/frame.rs", 340),
        ("app/persistence.rs", 330),
        ("lib.rs", 330),
        ("engine/gpu/arena.rs", 320),
    ];
    const DEFAULT: usize = 300;
    let budget: BTreeMap<_, _> = BUDGET.iter().copied().collect();
    let mut bad = Vec::new();
    for (rel, t) in all_rs() {
        let n = t.lines().count();
        let cap = budget.get(rel.as_str()).copied().unwrap_or(DEFAULT);
        if n > cap {
            bad.push(format!("  {rel}: {n} lines, budget {cap} — split it, or raise the budget here on purpose"));
        }
    }
    assert!(bad.is_empty(), "a file outgrew its budget:\n{}", bad.join("\n"));
}
```

**Find** in `src/lib.rs`:

```rust
mod state;
```

**Add below it:**

```rust
#[cfg(test)]
mod architecture; // the five rules in ARCHITECTURE.md, as tests
```

## 6. Expected state

```
cargo xtest                                                   8 passed, 0 failed
cargo check --target wasm32-unknown-unknown                   0 errors
./docs/_gate.sh                                               gate OK
```

Three signatures stay over the line, recorded in `KNOWN_WIDE` with the reason each is still
there. The most interesting is `append_rows`, which takes `buf`, `count` and `cap` loose - which
is exactly `GrowBuf`. That struct already EXISTS; `cloud.rs` and `objects.rs` simply do not use
it, keeping parallel fields instead. The wide signature is a symptom, and the rule is what made
it visible.

## Next

Lesson [58](58-nurbscurve.md) - NurbsCurve.
