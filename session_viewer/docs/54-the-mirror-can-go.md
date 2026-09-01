# 54 The mirror can go

> The kernel's wire format changed underneath the viewer, and 218 lines of this crate existed
> only to work around the old one.
> Nothing you can see changes — but for the first time the tree compiles against the CURRENT
> kernel, which it did not before this lesson.

## 1. Why

Lesson [38](38-append-dont-rebuild.md) added `LeanMesh`: a hand-written mirror of `proto::Mesh`
with tag 5 left out, because prost decoded 208k halfedge entries on the bunny into a nested
`HashMap` only for `Mesh::from_proto` to throw them away. An unlisted tag is skipped with a
length jump instead. It was a real measurement and a real saving.

It was also a duplicate of someone else's type. Every field had to be repeated with prost's
exact annotations, and every change to `mesh.proto` was a change this file had to be told about
— by a human, correctly, or the decode silently produced the wrong shape.

The kernel's P6 wire reshape removed the halfedge map from the wire entirely. There is nothing
left to skip, so there is nothing left for the mirror to do, and `proto::Session` becomes the
only shape in this file again.

The same argument retires two more mirrors: `LeanObjects` and `LeanSessionProbe`, plus
`TreeOnlyProbe`, which skipped the tree on the first pass and then decoded the WHOLE buffer a
second time to get it back. That trade bought one cheap decode and paid for a second full parse.

## 2. The proof

This is the one lesson in the chain whose before-and-after is a compiler error rather than a
pixel count. Against the kernel the repo pins today, the end of lesson 53 does not build:

```
error[E0560]: struct `session_rust::proto::Mesh` has no field named `halfedges`
error[E0560]: struct `session_rust::proto::Mesh` has no field named `pointcolors`
error[E0560]: struct `session_rust::proto::Mesh` has no field named `facecolors`
error[E0560]: struct `session_rust::proto::Mesh` has no field named `linecolors`
```

Those are `LeanMesh`'s field list, and nothing else in the viewer names them. After this lesson
the same tree compiles with 0 errors and the warning count unchanged.

## 3. The steps

**3a.** The three mirrors go together. `**Remove**` takes them out from the first doc line **up
to** the doc comment of the function that follows.

**Remove** from `src/app/persistence.rs`, **up to** the decode function's own doc comment:

```rust
/// Wire-identical mirror of `proto::Mesh` with ONE field left out: `halfedges` (tag 5).
```

```rust
/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
```

**3b.** In their place, one note about what used to be there.

**Find** in `src/app/persistence.rs`:

```rust
/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
```

**Add above it:**

```rust
// The mesh lane used to arrive through a hand-written mirror of `proto::Mesh` that left out
// tag 5, because prost decoded 208k halfedge entries on the bunny only for `Mesh::from_proto`
// to throw them away. The wire no longer carries them, so there is nothing to skip and no
// mirror to keep in step - the kernel's own `proto::Session` is the only shape here again.
```

**3c.** The decode reads the kernel's own type.

**Find** in `src/app/persistence.rs`:

```rust
    let Ok(p) = LeanSessionProbe::decode(bytes) else { return Session::default() };
```

**Replace with:**

```rust
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
```

**3d.** The mesh lane joins the other ten. Its macro arm has no callers left.

**Find** in `src/app/persistence.rs`:

```rust
        chunk!(lean o.meshes, Mesh, Mesh, meshes);
```

**Replace with:**

```rust
        chunk!(o.meshes, Mesh, Mesh, meshes);
```

**Find** in `src/app/persistence.rs`:

```rust
        // the mesh lane arrives as LeanMesh (halfedges skipped); the kernel's from_proto still
        // does the building
        (lean $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x.into_proto()));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
```

**Delete**

**Find** in `src/app/persistence.rs`:

```rust
        };

        // from_proto -> Result for the nested types; a bad object is skipped, not fatal
```

**Replace with:**

```rust
        };
        // from_proto -> Result for the nested types; a bad object is skipped, not fatal
```

**3e.** And the tree comes out of the same decode as everything else.

**Find** in `src/app/persistence.rs`:

```rust
    // The tree is rebuilt ONLY to compose those transforms down the hierarchy - see
    // `Session::world_xforms`, which returns an empty map on the same test. A flat sheet or a
    // mesh file lands here with nothing to compose and pays neither the decode nor the 90k
    // Rc<RefCell<TreeNode>> allocations.
    if s.xforms.is_empty() {
        return s;
    }
    let p = match TreeOnlyProbe::decode(bytes) { Ok(t) => t, Err(_) => return s };
    if let Some(tp) = &p.tree{
```

**Replace with:**

```rust
    // The tree comes from the same decode as everything else. It used to be SKIPPED here and
    // then re-decoded by a second mirror struct, to avoid 90k `Rc<RefCell<TreeNode>>`
    // allocations on a file that composes nothing. That trade bought one decode and cost a
    // second parse of the whole buffer plus a mirror to keep in step; a Session that loads its
    // own tree is both simpler and honest about what it holds.
    if let Some(tp) = &p.tree {
```

## 4. The harness that proved the mirror was equivalent

`check_lean` existed to load a file both ways and compare the results field by field. With one
way left there is nothing to compare, and a harness that cannot fail is worse than no harness:
it reports success forever.

**Delete `examples/check_lean.rs`**.

**Find** in `Cargo.toml`:

```toml
[[example]]
name = "check_lean"
path = "examples/check_lean.rs"
```

**Delete**

**4b.** `bench_load` measured the mirror against the whole decode, and printed the halfedge
bytes it saved. Both numbers are gone; what remains is the packed-array shape P6 introduced.

**Find** in `examples/bench_load.rs`:

```rust
    let lean = session_viewer::app::persistence::LeanSessionProbe::decode(&bytes[..]).unwrap();
```

**Replace with:**

```rust
    let lean = session_rust::proto::Session::decode(&bytes[..]).unwrap();
```

**Find** in `examples/bench_load.rs`:

```rust
        println!("  sample line: encoded {} B | guid {:?} name {:?} dash {} start.guid {:?} start.name {:?} color {:?}",
            l.encoded_len(), l.guid, l.name, l.dash.len(),
            l.start.as_ref().map(|p| p.guid.clone()), l.start.as_ref().map(|p| p.name.clone()), l.linecolor.is_some());
        let tot: usize = o.lines.iter().map(|l| l.encoded_len()).sum();
        let guids: usize = o.lines.iter().map(|l| l.guid.len() + l.name.len()
            + l.start.as_ref().map_or(0, |p| p.guid.len()+p.name.len())
            + l.end.as_ref().map_or(0, |p| p.guid.len()+p.name.len())).sum();
```

**Replace with:**

```rust
        // P6: coords/linecolor_rgba are packed; the Point and Color sub-messages are gone.
        println!("  sample line: encoded {} B | guid {:?} name {:?} dash {} coords {} rgba {}",
            l.encoded_len(), l.guid, l.name, l.dash.len(), l.coords.len(), l.linecolor_rgba.len());
        let tot: usize = o.lines.iter().map(|l| l.encoded_len()).sum();
        let guids: usize = o.lines.iter().map(|l| l.guid.len() + l.name.len()).sum();
```

**Find** in `examples/bench_load.rs`:

```rust
        let he: usize = ms.iter().map(|m| m.halfedges.iter().map(|(k,v)| {
            let inner: usize = v.neighbors.len() * 4 + 2;
            let _ = k; inner + 6
        }).sum::<usize>()).sum();
        let he_entries: usize = ms.iter().map(|m| m.halfedges.values().map(|v| v.neighbors.len()).sum::<usize>()).sum();
        let verts: usize = ms.iter().map(|m| m.vertices.len()).sum();
        let attrs: usize = ms.iter().map(|m| m.vertices.values().map(|v| v.attributes.len()).sum::<usize>()).sum();
        let faces: usize = ms.iter().map(|m| m.faces.len()).sum();
        println!("  meshes: {:.1} MB encoded | {verts} verts ({attrs} attr entries) | {faces} faces | halfedge {he_entries} entries ~{:.1} MB",
            tot as f64/1.048576e6, he as f64/1.048576e6);
```

**Replace with:**

```rust
        let verts: usize = ms.iter().map(|m| m.vertices.len()).sum();
        let attrs: usize = ms.iter().map(|m| m.vertices.values().map(|v| v.attributes.len()).sum::<usize>()).sum();
        let faces: usize = ms.iter().map(|m| m.faces.len()).sum();
        println!("  meshes: {:.1} MB encoded | {verts} verts ({attrs} attr entries) | {faces} faces",
            tot as f64/1.048576e6);
```

## 5. Expected state

```
cargo check --target x86_64-unknown-linux-gnu --all-targets     0 errors, 15 warnings
cargo check --target wasm32-unknown-unknown                     0 errors
./docs/_gate.sh                                                 gate OK
```

`app/persistence.rs` 454 -> 310, and it names no type it does not own.

## Recap

A mirror of someone else's type is a maintenance debt that comes due every time they change.
This one paid for itself while the wire carried something the viewer did not want; the moment
the wire stopped, it was 218 lines of pure liability.

## Next

Lesson [56](56-nurbscurve.md) — NurbsCurve.
