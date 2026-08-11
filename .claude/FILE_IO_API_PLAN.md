# File I/O API plan — typed constructors, no `io` module

Decided 2026-08-11. **Blocked** until the `session_transformation` → `xform` refactor lands: every
file this touches (`mesh`, `pointcloud`, `polyline`, `session` + their tests) is dirty in all three
languages right now.

## The rule

There is no `io` / `io_xyz` / `file_obj` module and no free-function reader. **Loading a file is a
constructor on the type it produces.** A format that carries several kinds of geometry also gets a
`Session.from_*` helper that loads everything at once.

| Format | Types it yields | Entry points |
|--------|-----------------|--------------|
| **OBJ** | Mesh (`v`/`f`), Polylines (`curv`), points | `Mesh.from_obj` · `Polyline.from_obj` · `PointCloud.from_obj` · `Session.from_obj` |
| **PLY** | Mesh (with faces), PointCloud (without) | `Mesh.from_ply` · `Polyline.from_ply` · `PointCloud.from_ply` · `Session.from_ply` |
| **XYZ** | PointCloud only — single type, so **no** `Session.from_xyz` | `PointCloud.from_xyz` |
| **STEP** | Point, NurbsCurve, NurbsSurface, Trimmed, BRep | `Session.from_step` (C++ only, see below) |
| **PDF** | Lines, Polylines, NurbsCurves, Meshes, layer groups | `session_io` `pdf_import` → `.pb` |

Writers mirror the readers: `Mesh.to_obj(path)`, `PointCloud.to_xyz(path)`, `Session.to_obj(path)`.
String variants keep the existing suffix convention: `Mesh.from_obj_str(s)` / `Mesh.to_obj_str()`.

A `Type.from_fmt` returns **one** value (first/merged); `Session.from_fmt` returns everything the
file held, grouped. `Polyline.from_obj` returns the `curv` runs.

## Parsers are written once per language

Decided explicitly: OBJ/PLY/XYZ parse natively in Python, Rust and C++, kept at parity by minitest
— **not** delegated to `session_io`. `session_io` keeps only formats the kernels must not carry:
PDF today (MuPDF's C sources), and it is where a future heavy format lands.

## Work list

**Delete** in all three, one commit so parity never breaks:
- `session_py/src/session_py/{file_obj,file_obj_test,io,io_test}.py` + `__init__.py` exports
- `session_rust/src/{file_obj,file_obj_test,io,io_test}.rs` + `lib.rs` mod/`pub use` lines
- `session_cpp/src/{file_obj.h,file_obj.cpp,file_obj_test.cpp,io_xyz.h,io_xyz.cpp,io_xyz_test.cpp}`
  + CMakeLists `src/io_xyz.cpp` and the two `*_test.cpp` entries (impls are GLOBbed at line 221)

**Add** the constructors above to `mesh`, `pointcloud`, `polyline`, `session` in each language.
PLY is new to the kernels — port from `session_io/src/ply.rs` (ascii + binary LE/BE, unknown
properties skipped by width).

**Tests fold** out of the deleted classes and into the type that now owns them, keeping the
existing names as cases: `Read Bunny` → `mesh`/`pointcloud` `From Obj` / `From Xyz`, plus new
`From Ply`. Class count **47 → 44**; update the README Key API files table and re-derive parity.

**Callers to repoint:** `session_py/main.py:102`, `session_rust/src/main.rs:2,104`, and the three
frozen lesson snapshots `session_viewer/docs/{32b_point_clouds,33_camera_relative,34a_load_file}/src/engine/gpu.rs`
(they use `include_str!` + `read_xyz_from_str` → `PointCloud::from_xyz_str`). The **live viewer uses
none of this.** Four lesson `.md` files mention the old names: `32b-point-clouds.md`,
`34b-session-walk.md`, `79-import-export.md`, `_KERNEL_GAPS.md`.

## STEP stays in C++ for now

`file_step` is 4289 lines and **11 of the 13 C++ mains call `read_file_step`** in-process for the
boolean campaign — it cannot simply move. Reshape it as `Session.from_step` / `Session.to_step` in
C++ when convenient; a Rust/Python port is a separate project.
