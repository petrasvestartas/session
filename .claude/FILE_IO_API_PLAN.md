# File I/O — where parsers live, and the API they should expose

## Settled

**Every file parser lives in the kernel that uses it, written natively per language.** There is no
converter crate. `session_io` existed briefly and was deleted (2026-08-11): the idea was to parse
each format once and hand the kernels `.pb`, but measurement killed it —

| | bunny (2.5k verts) | tiled bunny (160k verts) |
|---|---|---|
| parse OBJ → Mesh | 14.3 ms | 757 ms |
| + serialize `.pb` | 20.7 ms | 1470 ms |
| + deserialize `.pb` | 19.7 ms | 1495 ms |
| **overhead of the hop** | **+282 %** | **+391 %** |

`.pb` also inflates 12.9 MB of OBJ into 83 MB. Parsing was never the cost; rebuilding the geometry
two extra times is. `.pb` stays a session format, not a bulk-geometry transport.

**PDF is the one exception, and it lives in `session_rust`, not a separate crate.**
`session_rust/src/pdf.rs` exposes `import_pdf(src, stem, page)`, with a `pdf_import` bin. Both sit
behind an optional `pdf` feature because `mupdf-sys` compiles MuPDF's C sources, which cannot build
for `wasm32` — the viewer's target. Default builds pull no MuPDF, so
`cargo check --target wasm32-unknown-unknown` is unaffected (verified). `import_drawings.sh` passes
`--features pdf`. The viewer loads sheets as `.pb`; it cannot run the importer itself — a browser
has no filesystem and MuPDF has no wasm target.

**STEP** stays C++-only: `file_step` is 4289 lines and 11 of the 13 C++ mains call `read_file_step`
in-process for the boolean campaign.

## Still open — the API shape

Today the kernels expose free functions in module-shaped files (`file_obj`, `io`/`io_xyz`), and
those show up as their own minitest classes. The user's rule: **`io` must never be a test class.**
Loading a file should be a constructor on the type it produces, and a format carrying several
geometry kinds also gets a `Session.from_*` helper:

| Format | Entry points |
|--------|--------------|
| **OBJ** | `Mesh.from_obj` · `Polyline.from_obj` · `PointCloud.from_obj` · `Session.from_obj` |
| **PLY** | `Mesh.from_ply` · `Polyline.from_ply` · `PointCloud.from_ply` · `Session.from_ply` |
| **XYZ** | `PointCloud.from_xyz` only — single type, so no `Session.from_xyz` |

Writers mirror them (`Mesh.to_obj`), string variants keep the existing suffix (`Mesh.from_obj_str`).
Since the parsers are in the kernels, these are ordinary methods — no orphan-rule or circular-import
problem.

Work when picked up, one commit so parity never breaks:
- Fold `file_obj`/`io`/`io_xyz` into `mesh`, `pointcloud`, `polyline`, `session` in all three
  languages; delete the modules, the `__init__.py`/`lib.rs` exports, and the CMakeLists entries
  (`src/io_xyz.cpp`, the two `*_test.cpp`; impls are GLOBbed at CMakeLists:221).
- Tests move to the owning class keeping their names as cases (`Read Bunny` → `From Obj`).
  Class count **47 → 44**; update the README Key API files table and re-derive parity.
- **PLY is new to all three kernels** — nothing exists today; write it per language (ascii +
  binary LE/BE, mesh when the file has faces else pointcloud, unknown properties skipped by width).
- Repoint `session_py/main.py:102`, `session_rust/src/main.rs:2,104`, and the three frozen lesson
  snapshots `session_viewer/docs/{32b_point_clouds,33_camera_relative,34a_load_file}/src/engine/gpu.rs`
  (they use `include_str!` + `read_xyz_from_str`). The **live viewer uses none of this.**
