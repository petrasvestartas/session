# session_io

File-format → Session (`.pb`) converters. **Every foreign file format the project reads lives
here and nowhere else** — the kernels (`session_py`, `session_rust`, `session_cpp`) speak `.pb`
and their own JSON, nothing more.

A format is then written and fixed once instead of three times in three languages, and the
kernels stop carrying dependencies a format drags in. `session_rust` in particular must stay pure
Rust so the viewer keeps building for `wasm32` — where there is no filesystem to read a file from
in the first place.

## Converters

| Binary | Reads | Produces |
|--------|-------|----------|
| `obj_import <file.obj> <out_stem> [--polylines]` | Wavefront OBJ | Mesh, or Polylines from `curv` runs |
| `ply_import <file.ply> <out_stem> [--points]` | PLY — ascii, binary LE and BE | Mesh if the file has faces, else PointCloud |
| `xyz_import <file.xyz> <out_stem>` | `x y z [r g b [a]]` | PointCloud |
| `pdf_import <file.pdf> <out_stem> [page]` | PDF via MuPDF | Lines, Polylines, NurbsCurves, Meshes, layer groups |

Each writes `<out_stem>.pb`, which any of the three kernels loads with `Session::pb_load`.

```bash
cargo build --release                      # obj/ply/xyz — seconds
cargo build --release --features pdf       # adds pdf_import; compiles MuPDF's C sources
```

`mupdf` is optional and off by default: it builds MuPDF from source, which no other converter
needs. `session_data/import_drawings.sh` passes `--features pdf`.

## Adding a format

Add `src/<fmt>.rs` with the parser, declare it in `src/lib.rs`, and add `src/bin/<fmt>_import.rs`
that builds a `Session` and calls `pb_dump`. New binaries under `src/bin/` are auto-discovered —
only `pdf_import` needs an explicit `[[bin]]`, because it carries `required-features`.

Colour convention: readers accept both 0–255 integer and 0–1 float channels and normalise to the
kernel's 0–1 `Color`, detected by whether any channel exceeds 1.
