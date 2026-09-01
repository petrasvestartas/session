# Session datastructure plan — keep the architecture, fix the implementation

Verdict from the 2026-08-01 review: the composition (`objects` + `lookup` + `tree` + `graph`,
guid-tracked) is the right shape and stays. Four implementation fixes, phased so the cheap,
safe wins land first and nothing blocks the viewer lessons until each is needed.

## Measured baseline (Rust, release, 200k lines)

```
add_line x200k    3161 ms    RSS +2530 MB   (~12.7 KB per 100-byte object)
iterate lookup      27 ms
guid lookups        44 ms / 200k            (4.5M/s — fine)
pb_dumps           504 ms    (59.4 MB)
pb_loads          2819 ms
Session::clone    1503 ms
size_of::<Geometry>() = 8992 B   (Element inlines its caches; enum pays max variant)
```

Language asymmetry that drives the whole plan:

| | storage of one object | lookup ↔ objects | staleness risk |
|---|---|---|---|
| C++ (ground truth) | `std::variant<std::shared_ptr<T>…>` (16 B) | SAME pointer in both | none |
| Python | references | SAME object in both | none |
| Rust | 9 KB enum **by value** | **two independent clones** (+1 transient) | **yes — `pb_dumps` reads `objects`, viewer mutates `lookup`** |

So fixes 1/2/4 are Rust-parity work; fix 3 (ordering) is genuinely 3-language.

---

## P1 — Box the `Geometry` variants (Rust only, no API break) — ✅ DONE 2026-08-01

Measured after (same 200k-line bench, `cargo run --release --example bench_session`):

```
size_of Geometry     8992 B → 16 B
add_line x200k       3161 ms → 862 ms      RSS +2530 MB → +768 MB
pb_loads             2819 ms → 712 ms
Session::clone       1503 ms → 613 ms
minitest --rust      713/713 green; session_viewer compiles with ZERO edits (deref coercion)
```

Guards committed: `session_rust/tests/geometry_size.rs` (enum stays 16 B),
`session_rust/examples/bench_session.rs` (numbers one command away).
Remaining RSS is now dominated by the OBB ray cache (1848 B × N ≈ 370 MB → P5) and the
objects/lookup payload duplication (→ P2/P4). **P1b was DEFERRED**: boxing Element's four inline
caches touches ~70 call sites in element.rs for little gain now that Element already lives on
the heap behind the boxed variant.

**Problem.** Every `lookup` entry pays 8,992 B because the enum is as large as `Element`.

**Change.** In `session_rust/src/session.rs`, every variant becomes boxed — the Rust spelling of
C++'s `shared_ptr` variant (minus sharing, which is P4):

```rust
pub enum Geometry {
    OBB(Box<OBB>),
    BRep(Box<BRep>),
    Element(Box<Element>),
    Line(Box<Line>),
    Mesh(Box<Mesh>),
    NurbsCurve(Box<NurbsCurve>),
    NurbsSurface(Box<NurbsSurface>),
    Plane(Box<Plane>),
    Point(Box<Point>),
    PointCloud(Box<PointCloud>),
    Polyline(Box<Polyline>),
}
```

Box ALL variants, not just the fat ones — uniform construction sites, and small variants like
`Point` are cheap to box while keeping the enum at 16 B.

**Call-site impact — smaller than it looks:**
- **Read sites (viewer, tests): zero changes.** `Geometry::Mesh(m)` binds `m: &Box<Mesh>`;
  field access (`m.xform`) and function args (`push_mesh(m, …)` expecting `&Mesh`) work via
  deref coercion. The lesson-35 `Scene::build` compiles untouched (verify: `cargo check` in
  `session_viewer` with zero viewer edits).
- **Construction sites: mechanical.** `Geometry::Mesh(mesh.clone())` →
  `Geometry::Mesh(Box::new(mesh.clone()))` in `add_*`, `pb_loads`, `json` load, `Objects→lookup`
  rebuild (~40 sites, all inside `session.rs`).
- `Geometry::guid()`/match helpers: unchanged (auto-deref).

**P1b (optional, same PR).** `Element` is 8,992 B even on the heap because it inlines
`cached_aabb: Option<OBB>` (1,848 B), `cached_obb`, `cached_collision_mesh: Option<Mesh>`,
`cached_polylines`. Box those four fields → `Element` drops to a few hundred bytes. Element
counts are usually low; do it while in the file, skip if it fights serde.

**Acceptance.**
- `assert_eq!(std::mem::size_of::<Geometry>(), 16)` (new minitest, Rust-only file).
- 200k-line bench: RSS from 2.5 GB → target **< 600 MB**; `Session::clone` < 500 ms.
- `./bash/minitest.sh --rust --no-web` green; `session_viewer` compiles with no edits.

**Risk:** low. **Effort:** ~half a day including bench re-run.

---## P2 — Kill the staleness bug: `lookup` is declared THE truth — ✅ DONE 2026-08-01

Landed: field doc contracts; `objects_synced()` (lookup-truth refreshed into the objects vecs
by guid, insertion order kept) called by `jsondump` + `pb_dumps`; the transient AABB clone in
the 8 caching `add_*` fns removed (cache computed from the built `geometry` BEFORE insert);
new minitest ×3 "Lookup Mutation Roundtrip". Bench: add_line 862→705 ms; pb_dumps +~110 ms
(the save-time sync — save is rare, correctness is not).

**BONUS BUG FOUND & FIXED (C++):** the new test failed on C++ — `Objects::pb_loads`/`jsonload`
did `make_shared<T>(T::pb_loads(...))`, and the guid-REFRESHING copy constructor minted fresh
guids for every object on every load (silently orphaning tree/graph references keyed by guid).
Fixed with `keep_guid()` in `objects.cpp` (a LOAD is not a duplicate — restore the loaded
identity); cpp minitest 762/762.

### Original P2 spec (for reference)

**Problem.** `add_*` stores two independent copies; anyone mutating through `lookup` (the viewer
does; every future edit lesson will) leaves `objects` stale — and `pb_dumps`/`file_json_dump`
serialize from `objects`. Save would silently write pre-edit geometry. C++/py can't have this
bug (shared object).

**Change (stopgap that is also the contract):**
1. Doc-comment on both fields: `lookup` is authoritative between loads and saves; `objects` is
   the serialization/typed view, synced on save.
2. New private `fn sync_objects_from_lookup(&mut self)` — clears the `objects` vecs and refills
   them from `lookup` (typed by variant), preserving P3's canonical order once it exists.
3. Call it first thing in `pb_dumps`, `file_json_dump` (and any other objects-reading exporter).
4. While in `add_*`: drop the transient third clone — compute the AABB from `&mesh` BEFORE
   moving it into the box (`cache_geometry_aabb(&guid, …)` currently clones a whole `Geometry`
   just to measure it). This is fix 4 folded in; one clone remains (objects+lookup), P4 removes it.

**Cross-language lock-in test** (identical in all 3 languages, the minitest way):
add object → mutate it **through `lookup`** (e.g. `linecolor`) → `pb_dumps` → `pb_loads` →
assert the mutation survived. Passes trivially in C++/py (shared object), catches any Rust
regression forever.

**Acceptance:** new test green ×3 languages; no other test changes.
**Risk:** low. **Effort:** ~2 hours. **Deadline:** must land before lesson 39 (save) and 51 (undo).

---

## P3 — Deterministic iteration order (all 3 languages) — ✅ DONE 2026-08-01

Landed: `Session::order()` ×3 languages (objects vectors walked in the canonical type sequence
points→lines→planes→bboxes→polylines→pointclouds→meshes→nurbscurves→nurbssurfaces→breps→
elements; computed, no stored state, so `remove_object` needs nothing); minitest ×3 "Order"
(mixed-type insertion + pb-roundtrip stability — the roundtrip check is only possible because
P2's C++ guid fix landed); viewer `Scene::new` now filters `session.order()` (lesson 35 md +
`docs/35_scene_struct` snapshot updated). Suites: rust 715/715, py 748/748 (31/31 session),
cpp 762/762.

### Original P3 spec (for reference)

**Problem.** `lookup` iteration order is arbitrary (Rust `HashMap`, C++ `unordered_map`) and
differs per run and per language. The viewer already had to bolt `order` onto `Scene`;
reconcile (38) and any cross-language row comparison need one canonical order.

**Change.** Define **canonical session order** = insertion order, materialized as
`Session::order() -> Vec<String>` (or a maintained `insertion_order: Vec<String>` field,
serde-skipped) in all three languages:
- `add_*` appends the guid; `remove_*` (where it exists) removes it.
- On load (`pb_loads`/json), rebuild it by walking the `objects` vectors in one fixed type
  sequence (points, lines, planes, bboxes, polylines, pointclouds, meshes, nurbscurves,
  nurbssurfaces, breps, elements) — the vectors are already insertion-ordered per type, so this
  is deterministic across runs AND languages with zero schema change.
- Python: dict is already insertion-ordered — `order()` still added for API parity.

**Viewer follow-up (lesson 35 erratum, one paragraph):** `Scene::new` iterates
`session.order()` instead of `session.lookup`, and `Scene.order` becomes a plain copy (or goes
away). Update `docs/35_scene_struct` snapshot + the lesson's "HashMap is unordered" note.

**Acceptance:** load the same `.pb` twice → identical `order()`; load the shared fixture in all
3 languages → identical `order()` (add to session minitest); viewer rows stable across reloads.
**Risk:** low-medium (touch 3 languages + minitest parity). **Effort:** ~1 day ×3 languages.

---

## P4 — True single storage in Rust — ✅ DONE 2026-08-02 (Rc-shared, COW writes)

Design chosen (informed by a 5-agent census + 43-agent adversarial review): `Geometry` variants
and ALL `Objects` vectors hold `Rc<T>` — ONE allocation shared by both stores (the `shared_ptr`
mirror). Reads unchanged (deref coercion; viewer compiled with ZERO edits). Writes are
copy-on-write via `Rc::make_mut`; **DIRECTION CONTRACT: lookup wins** — mutate through `lookup`;
`objects_synced()` re-shares (Rc::ptr_eq-guarded) at save; `get_geometry` reads the synced view;
`compute_face_to_face` pulls lookup-truth in and re-points lookup after its cache fill. serde
gained the `rc` feature (JSON byte-identical). `session_viewer_archive` migrated (10 files).
Review-confirmed bugs fixed before landing: objects-side COW splits were silently dropped on
save (direction contract + synced get_geometry + test moved to the lookup path); ray_cast Mesh
arm now COWs ONLY when the triangle BVH is missing (`Mesh::has_triangle_bvh` +
`ray_cast_bvh_ready(&self)`).

Bench (200k lines): add 3161→274 ms, RSS +2530→+188 MB (13.4×), pb_loads 2819→575 ms,
clone 1503→63 ms (COW-shallow geometry).

### Original P4 spec (for reference)

P1+P2 leave one payload duplication (objects copy + lookup copy — matters mostly for meshes).
Two candidate endgames; **do not start until save (39) and undo (51) are implemented**, because
their access patterns decide the winner:

- **Option A — mirror C++ exactly:** `Geometry = enum of Rc<RefCell<T>>`, `objects` vectors hold
  the same `Rc`. Pros: literal ground-truth parity, one allocation, mutation visible everywhere.
  Cons: every read site grows `.borrow()`, and runtime `BorrowMut` panics become possible in
  viewer iteration patterns — the classic Rc<RefCell> hazard. Wasm is single-threaded so `Rc`
  is fine.
- **Option B — `lookup` owns, `objects` becomes a derived view:** keep boxed values in `lookup`;
  `objects` is filled only by `sync_objects_from_lookup()` (P2 already builds this muscle).
  Pros: idiomatic Rust, zero borrow panics, P2 becomes the permanent design. Cons: `objects` is
  no longer live between saves (C++/py's is) — parity is behavioral (same API results at save
  boundaries), not structural.

Leaning **Option B** (Rust stays value-oriented, no RefCell hazards), with the P2 sync as the
mechanism — but measure after P1: if RSS and clone times are already fine at real document
sizes, P4 may never be needed.

---

## P5 — AABB cache made LAZY — ✅ DONE 2026-08-02 (Rust + C++; Python already had no cache)

Not a type swap (SpatialBVH is OBB-native): a lifecycle fix. `cache_geometry_aabb` only sets
`bvh_cache_dirty`; `rebuild_ray_bvh_cache` recomputes ALL boxes from `order()` (deterministic
×languages, skips guids missing from lookup) into a LOCAL vec — the BVH copies boxes into its
nodes, so `cached_boxes` stays permanently empty (kept for API compat). Kills both the 370 MB
resident cache at 200k AND a latent staleness bug (mutated objects kept their add-time boxes).
`get_collisions` also iterates `order()` now (was unordered-map iteration, both languages).

Bug ledger closed with it: add_brep/add_element never set the dirty flag (objects invisible to
ray_cast; fixed rs+cpp); C++ remove_object relied on the removed len-mismatch net (dirty flag
added); C++ ray_cast pushed the SESSION guid into RayHit (now obj_guid) and operator[]'d the
lookup (now find+skip); Rust/py remove_object skipped elements (retain/filter added + a
resurrection roundtrip check in the "Remove Object" minitest ×3); Rust pb roundtrip now carries
nurbscurves/nurbssurfaces (was silently dropping them — C++/py already carried them); Rust
pb_loads now inserts Elements into lookup. Logged, NOT fixed (pre-existing, out of scope):
components 3-way divergence (py lookup has them, C++ separate map, Rust nowhere); get_geometry
never transforms Elements; dormant derived Serialize on Session bypasses objects_synced;
in-place lookup mutations still need a manual invalidate_bvh_cache() for the ray BVH.

### Original P5 spec (for reference)

`cached_boxes: Vec<OBB>` stores 1,848 B per object for what the ray-BVH consumes as an axis
box. At 200k objects that's ~370 MB — after P1 it becomes the single biggest allocation. Store
`[f64; 6]` min/max (48 B) instead; C++ has the same cache and gets the same fix. Do it when a
real document makes it hurt, not before.

---

## P6 — the wire is shaped like memory; reshape it for its reader — 📋 SPEC (2026-08-31)

**Problem.** The schema was derived from the in-memory representation, so the wire mirrors it:
`map<uint64, VertexData>` is a serialized `HashMap`, and `Color` is a sub-message carrying a
36-char guid `String` and a name. Three consequences, all measured:

- **Double materialization.** prost builds its maps, then `from_proto` builds the kernel's —
  714k SipHash inserts, then 714k more, for data that is really parallel arrays.
- **Per-object, not per-byte, cost.** ~500-800k `String` allocations per sheet load; wasm
  punishes it hardest — lion decodes at 28 ms/MB, a sheet at 95 ms/MB. Raw prost decode is
  byte-proportional and modest (213 ms native for 34.7 MB), so **the format is not the cost —
  the shape is**.
- **No streaming for anything but clouds.** Variable-length entries mean the length prefix
  gives bytes, not elements: buffers cannot be sized up front and a byte range can split an
  entry. `PointCloud.coords` is the only bulk field in the schema that is a packed
  fixed-width array, and it is the only type lesson 43 can stream. See
  `session_viewer/docs/43-streaming-cloud.md` § "Why this works for clouds and nothing else".

**Non-goal: replacing protobuf.** Its strengths — one schema, generated readers/writers in
three languages that provably agree, compatibility, buf tooling — are exactly what the
3-language parity rule needs. And since the kernel is f64 while the GPU wants f32, one
conversion pass over every coordinate is the floor; packed `repeated double` already reaches
it, so a zero-copy format (FlatBuffers / Cap'n Proto / custom container) would save only a
memcpy that has to happen anyway. Protobuf is the right *envelope*; it stops being right the
moment it is asked to model per-element structure for millions of elements. **Metadata →
messages. Bulk → packed arrays.**

**Migration discipline (applies to every item).** Additive: new tags alongside the old, readers
prefer new and fall back to old, writers emit only new, old fields marked deprecated and
`reserved` only after every asset is regenerated. This is what lets the three languages land
independently without a red CI window and keeps committed `.pb` assets loading. It also avoids
colliding with the in-flight lessons 45-51 authoring run.

### Ranked change set

1. **Bulk colours → packed `repeated float` (4 per colour).** `repeated Color pointcolors`
   appears in `Mesh`, `NurbsCurve` and `NurbsSurface`, and each `Color` carries guid + name:
   measured at **66 B per line** against 55 B for the line's actual geometry. Packed floats are
   memcpy-able and **bit-exact** — do NOT pack to RGBA8, it would break `to_proto`/`from_proto`
   round-trip equality in the minitests. Biggest win per unit of risk, and it touches no
   structure.
2. **Drop guid/name from sub-message positions.** `Line.start`/`end` are `Point`s carrying their
   own guid + name; a line's start point is not an identity. Keep guid/name on top-level
   objects only. (Line is 178 B/line on the wire for 48 B of data; 123 B of that is identity
   metadata.)
3. **`PointCloud.colors`: `repeated uint32` → `repeated fixed32`.** Varint is not memcpy-able,
   which is exactly why lesson 43 must fetch the 27 MB colour run whole and decode it
   element-by-element instead of slicing it. One-word change, makes colours stream like coords.
4. **`Mesh` maps → SoA.** `repeated uint64 vertex_keys`, `repeated double vertex_xyz` (3N,
   packed — length/24 gives the count), `repeated uint64 face_keys`, `repeated uint32
   face_offsets` (F+1 prefix sums), `repeated uint32 face_verts` (indices INTO `vertex_keys`,
   not raw keys — half the bytes and directly usable as GPU indices; `vertex_keys` is ascending
   so the reverse map is a binary search). Attributes go **columnar** — `repeated string
   attr_names` plus one packed `repeated double` per name — so the common case (measured: sheets
   have ZERO vertex attributes) costs nothing. A probe of just `map` → `repeated` measured mesh
   decode 160 → 100 ms; full SoA goes further and makes `walk_to_coords` generalise to meshes.
5. **Stop writing `halfedges`.** All three languages already tolerate its absence (lazy
   halfedge), yet the wire still carries megabytes of nested per-vertex maps that the reader
   throws away. Writer-side only.
6. **Skip `Graph.vertices` when there are no edges**, rebuilt on load from objects via ONE
   shared helper so `add_*` and rebuild cannot drift. Measured: file −22%, prost decode −19%,
   `pb_loads` −23%.

### What P6 does NOT fix

Loading, not residency. A `Line` still costs ~320 B and a face ~78 B, because that is the
**object model**, not the wire — a zero-copy format would not help either. Small memory needs
the other half: keep the transfer representation (SoA arrays) separate from the edit
representation (objects + halfedge) and build the second lazily, per object actually touched.
`display_only` and lazy-halfedge are the first two steps of that; they are independent of P6
and both are needed.

### Acceptance

- Round-trip minitests ×3 languages: `to_proto`/`from_proto` and `file_json_dump`/`load` equal
  on a mesh WITH vertex attributes, face attributes and holes (the columnar path), and on one
  without (the fast path). Old-file fallback proven by a checked-in pre-P6 `.pb` fixture.
- Cross-language: a file written by each language loads in the other two, bytes compared.
- `minitest.sh --py --rust --cpp` at current counts (735 / 767 / 783), run **twice**.
- Viewer goldens **pixel-identical** (`examples/selftest` PPM `cmp`), lion `189148`.
- `examples/bench_load.rs` and `examples/probe_mem.rs` before/after, double-run, in the plan.

**Order:** 1 → 3 → 5 → 6 → 2 → 4 (cheapest and least structural first; 4 is the big one).
**Risk:** 1/3/5/6 low, 2 medium (touches `Line`/`Point`), 4 high. **Effort:** ~a day for 1+3+5+6
per language, several for 4.

### RUST LANDED 2026-08-31 — items 1, 2, 5 (py/cpp ports NOT started)

Measured with `session_rust/examples/p6_probe.rs` (load a `.pb`, re-serialize, reload; second
arg writes the P6 file out). Rust minitests **738/738** after each step.

| file | wire | reload of P6 file vs load of pre-P6 |
|---|---|---|
| `draw_pd_treppenhaus04.pb` (90,015 objs) | 54,673,378 → 49,935,717 B (**−8.7%**) | 499 → **325 ms** |
| `floor_model.pb` | 2,800,017 → 2,524,968 B (**−9.8%**) | 19 → 13 ms |
| `colors_widths.pb` | 8,113 → 7,690 B (**−5.2%**) | — |

RENDER EQUIVALENCE, double-run, `examples/selftest` PPM `cmp`: `colors_widths` and
`floor_model` both **PIXEL-IDENTICAL** old vs P6-rewritten, and each run-to-run identical.

- **Item 1** — `Mesh` bulk colours → `pointcolors_rgba`/`facecolors_rgba`/`linecolors_rgba`
  (tags 18-20, packed float ×4).
- **Item 2** — `Line`: `coords` (tag 9, 6 packed doubles) replaces the two `Point`
  sub-messages, which were serialization-only wrappers each carrying a redundant
  `width: 1.0` fixed64; `linecolor_rgba` (tag 10) replaces the `Color` sub-message and its
  36-char guid String. This is the sheet win — a sheet's cost is Lines, not meshes.
- **Item 5** — `to_proto` no longer emits `halfedges`. It had been transiently *computing*
  them for meshes that carried none, so a re-save GREW the file 35%.

**ONE shared implementation:** `Color::pack` / `Color::unpack` live in `color.rs` and every
type calls them. Do NOT copy pack/unpack into a geometry module — see the simplification note
below.

**TRAP, hit and fixed:** `session_viewer/src/app/persistence.rs`'s `LeanMesh` is a hand-written
prost mirror, and an unlisted tag is skipped **silently**. Adding tags 18-20 to `mesh.proto`
without adding them to `LeanMesh` makes P6 meshes render with no colours and no error. Any new
tag must be added in both places — which is itself an argument for the note below.

### SIMPLIFICATION NOTE — the additive migration is the thing that hurts

The additive shape (legacy field + new field + writer branch + reader branch, ×3 languages ×
~8 types, plus a hand-mirrored `LeanMesh`) is *more* code, permanently, and it fails silently
when the mirror drifts. The end state should be ONE path:

1. Keep the P6 fields, DELETE the legacy ones (`reserved` their tags).
2. Put the fallback in a **one-shot migration binary** that reads pre-P6 and writes P6, run
   once over `session_viewer/assets/pb/` and `session_data`; then delete the binary.
3. Drop the "does any colour have a non-default name" branch by deciding bulk colours have no
   names (verify nothing sets one first) — that removes the `(legacy, packed)` tuple and makes
   `Color::pack` return a plain `Vec<f32>`.
4. Generate `LeanMesh` from the schema, or delete it once item 4's SoA makes the skip-list
   unnecessary.

That halves the serialization code instead of doubling it. It costs one coordinated landing
plus an asset regen — deliberately deferred here only because lessons 45-51 are mid-flight.

---

## Order of work and lesson coupling

```
P1  now            Rust-only, no API break            gates: size_of==16, bench, minitest --rust
P2  with P1        Rust-only + 1 new test ×3          MUST precede lesson 39 (save), 51 (undo)
P3  next           3 languages + viewer erratum        MUST precede lesson 38 (reconcile)
P4  deferred       decide Option A/B after 39+51 exist
P5  deferred       when a real file shows the cache in a profile
P6  spec'd         proto + 3 languages + asset regen  additive; sequence 1,3,5,6,2,4
```

Re-run the baseline bench after each phase (bench source lives in this plan's history; consider
committing it as `session_rust/examples/bench_session.rs` so the numbers are one command away).
