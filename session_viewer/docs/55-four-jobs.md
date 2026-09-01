# 55 Four jobs in one body

> `push_ink` is 262 lines, the largest function in the tree by a factor of two. It becomes 12.
> Nothing you can see changes.

## 1. Why

The `mark(..)` calls already in it name its own seams: a profiling shim, the pen-width policy,
the slot index, the topology walk, the edge pass, the vertex pass. Six jobs, thirty-odd shared
locals, one body.

Those locals are the giveaway. They ARE the shared state, so writing them down as a struct is
exactly what turns two halves of one body into two methods - nothing else has to move, because
nothing else was ever entangled. Notice how small the steps below are: the two loops never
change. They were never the problem.

The profiling shim is why this could not be done sooner. It declared `mark` TWICE, once real and
once empty, so every call site would typecheck on both targets - and a closure over a local
cannot leave the body that owns that local. Make it a value and the timings travel with the code
they measure.

## 2. The topology type gets a name

`prepare` stores it, so it can no longer stay anonymous.

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
use super::mesh_topology::{COPLANAR_DOT, mesh_topology};
```

**Replace with:**

```rust
use super::mesh_topology::{COPLANAR_DOT, MeshTopo, mesh_topology};
```

## 3. The shim becomes a value, and the locals become a struct

This is the whole change. `Marks` replaces the doubled closure; `Ink` holds what both passes read;
`prepare` is the first hundred lines, in the same order; `push_ink` becomes the twelve lines that
call them.

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
pub(crate) fn push_ink(
    m: &Mesh,
    ri: u32,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>,
) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    let prof = env_flag("VIEWER_PROFILE", &VIEWER_PROFILE);
    #[cfg(not(target_arch = "wasm32"))]
    let mut lap = std::time::Instant::now();
    #[cfg(not(target_arch = "wasm32"))]
    let mut mark = |name: &str, lap: &mut std::time::Instant| {
        if prof { eprintln!("  push_mesh {name:<20} {:?}", lap.elapsed()); *lap = std::time::Instant::now(); }
    };
    // Same signature on wasm so every `mark(..)` call site below stays identical.
    #[cfg(target_arch = "wasm32")]
    let mut lap = ();
    #[cfg(target_arch = "wasm32")]
    let mut mark = |_name: &str, _lap: &mut ()| {};
    let _ = &mut lap;

    // Edge width 0 = hidden wireframe, A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a triangulated PDF
    // fill (a letter, a pocket region) ask for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width broadcasts to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 {
            w[0]
        } else {
            w.get(i).copied().unwrap_or(1.0)
        }
    };

    let hidden = |i: usize| width_at(i) == 0.0;
```

**Replace with:**

```rust

/// The `VIEWER_PROFILE` stopwatch, as a value instead of a `#[cfg]`-doubled closure pair.
///
/// The old shim declared `mark` TWICE - once real, once empty - so every call site would
/// typecheck on both targets. A closure over a local cannot leave the body that owns the local,
/// which is precisely why the timings could not move when the function was split.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct Marks { on: bool, lap: std::time::Instant }
#[cfg(target_arch = "wasm32")]
pub(crate) struct Marks;

impl Marks {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self { Marks { on: env_flag("VIEWER_PROFILE", &VIEWER_PROFILE), lap: std::time::Instant::now() } }
    #[cfg(target_arch = "wasm32")]
    fn new() -> Self { Marks }

    #[cfg(not(target_arch = "wasm32"))]
    fn at(&mut self, name: &str) {
        if self.on { eprintln!("  push_mesh {name:<20} {:?}", self.lap.elapsed()); self.lap = std::time::Instant::now(); }
    }
    #[cfg(target_arch = "wasm32")]
    fn at(&mut self, _name: &str) {}
}

/// Everything both ink passes read, prepared once.
///
/// `push_ink` was 262 lines: the profiling shim, the pen-width policy, the slot index, the
/// topology walk, the edge pass and the vertex pass, sharing thirty-odd locals. Those locals ARE
/// the shared state, so writing them down is what turns two halves of one body into two methods.
struct Ink<'a> {
    m: &'a Mesh,
    ri: u32,
    keys: Vec<usize>,
    /// Vertex positions BY SLOT. f32 for the row; the f64 they came from decided the normals.
    vpos: Vec<[f32; 3]>,
    /// Key -> slot. A dense key space indexes a Vec; a mesh with holes falls back to the map.
    slot_vec: Vec<u32>,
    slot_map: std::collections::HashMap<usize, u32>,
    dense: bool,
    topo: MeshTopo,
    /// At or above WIREFRAME_BLACK_MIN edges the wireframe draws BLACK whatever the file says.
    black_wire: bool,
}

impl<'a> Ink<'a> {
    fn prepare(m: &'a Mesh, ri: u32) -> Self {
```

## 4. The first pass

`pipes` opens by re-making the three closures the old body called - `slot`, `width_at`, `hidden` -
so the loop after them is the loop that was always there, at the indentation it always had.
`black_wire` is prepared state now, so the pass reads it instead of recomputing it.

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    let edges = &topo.edges;
    let closed = topo.closed;
    mark("topology", &mut lap);
```

**Replace with:**

```rust
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN;
    Self { m, ri, keys, vpos, slot_vec, slot_map, dense, topo, black_wire }
    }

    #[inline]
    fn slot_of(&self, k: usize) -> usize {
        if self.dense { self.slot_vec[k] as usize } else { self.slot_map[&k] as usize }
    }

    /// Edge width 0 = hidden wireframe. A mesh only has explicit widths if someone called
    /// set_linecolors, so the 1.0 default leaves every ordinary mesh untouched - but a
    /// triangulated PDF fill asks for no wireframe at all, and without this every glyph would
    /// render outlined in tubes and dotted at each vertex. A single width broadcasts to every
    /// edge - one entry instead of one per edge.
    #[inline]
    fn width_of(&self, i: usize) -> f64 {
        let w = self.m.widths();
        if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
    }

    /// Edges -> cylinder segments: the wireframe pen.
    ///
    /// The bindings below re-make the closures the old body called, so the loop MOVED here
    /// unchanged - it is not retyped, and its indentation is the one it always had.
    fn pipes(&self, segments: &mut Vec<CylinderSegment>, mk: &mut Marks) {
    let (ri, topo, vpos, black_wire) = (self.ri, &self.topo, &self.vpos, self.black_wire);
    let edges = &topo.edges;
    let slot = |k: usize| self.slot_of(k);
    let width_at = |i: usize| self.width_of(i);
    let hidden = |i: usize| self.width_of(i) == 0.0;
```

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;
```

**Delete**

**Find** in `src/app/walk/mesh_ink.rs`:

```rust


    for (i, (a, b, col)) in edges.iter().enumerate(){
```

**Replace with:**

```rust

    for (i, (a, b, col)) in edges.iter().enumerate(){
```

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    mark("pipe loop", &mut lap);
```

**Replace with:**

```rust
    mk.at("pipe loop");
    }

    /// Vertices -> glyph points: the markers that sit where the pens meet.
    fn dots(&self, glyphs: &mut Vec<GlyphPoint>, mk: &mut Marks) {
    let (m, ri, topo, vpos, keys) = (self.m, self.ri, &self.topo, &self.vpos, &self.keys);
    let edges = &topo.edges;
    let edge_faces = &topo.edge_faces;
    let slot = |k: usize| self.slot_of(k);
    let width_at = |i: usize| self.width_of(i);
    let hidden = |i: usize| self.width_of(i) == 0.0;
```

## 5. The second pass

Same shape: bindings, then the untouched vertex walk.

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    mark("vbest+vedges", &mut lap);

    // VIEWER_NO_DOTS drops the per-vertex dots, so the harness can tell how much of a dense
    // wireframe's ink is dots and how much is edges.
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return closed }
```

**Replace with:**

```rust
    mk.at("vbest+vedges");

    // VIEWER_NO_DOTS drops the per-vertex dots, so the harness can tell how much of a dense
    // wireframe's ink is dots and how much is edges.
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return }
```

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    mark("dots loop", &mut lap);
    closed
}

```

**Replace with:**

```rust
    mk.at("dots loop");
    }
}

/// Both ink passes over one mesh. Returns whether the mesh is closed - the caller needs it for
/// FLAG_OPEN, and `prepare` learned it from the same topology walk the passes read.
pub(crate) fn push_ink(
    m: &Mesh,
    ri: u32,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>,
) -> bool {
    let mut mk = Marks::new();
    let ink = Ink::prepare(m, ri);
    mk.at("topology");
    ink.pipes(segments, &mut mk);
    ink.dots(glyphs, &mut mk);
    ink.topo.closed
}

```

## 6. Expected state

```
cargo check --target x86_64-unknown-linux-gnu --all-targets   0 errors, 14 warnings
cargo check --target wasm32-unknown-unknown                   0 errors
./docs/_gate.sh                                               gate OK
```

`push_ink` 262 -> 12 lines. The file's largest function is now `dots` at 121, and the tree's
largest is no longer in this file at all.

## Recap

A function that shares thirty locals between its halves is not one function. Writing the locals
down is the whole refactor; the halves fall out of it.

## Next

Lesson [58](58-nurbscurve.md) - NurbsCurve.
