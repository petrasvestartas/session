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

## Order of work and lesson coupling

```
P1  now            Rust-only, no API break            gates: size_of==16, bench, minitest --rust
P2  with P1        Rust-only + 1 new test ×3          MUST precede lesson 39 (save), 51 (undo)
P3  next           3 languages + viewer erratum        MUST precede lesson 38 (reconcile)
P4  deferred       decide Option A/B after 39+51 exist
P5  deferred       when a real file shows the cache in a profile
```

Re-run the baseline bench after each phase (bench source lives in this plan's history; consider
committing it as `session_rust/examples/bench_session.rs` so the numbers are one command away).
