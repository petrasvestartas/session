# 35 — end-state snapshot

A standalone copy of `session_viewer` as it stands at the **end of lesson 35**, both parts:

- **Part 1** — `Scene` owns the documents (`docs: Vec<Doc { name, place, session }>`) and the
  merged GPU tables; files append one at a time; the parse is sliced so frames render during it;
  the upload copies nothing.
- **Part 2** — the mesh edge lane: camera-facing quads at 2 triangles an edge instead of a 12
  triangle tube, a 40-byte segment row carrying its own face adjacency, and ink that **hugs** the
  surface it decorates instead of floating a guessed distance in front of it.

Use it to check what you typed:

```bash
diff -u docs/35_scene_struct/src/shaders/ribbon.wgsl session_viewer/src/shaders/ribbon.wgsl
```

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown     # verified clean at the time of the snapshot
trunk serve --release                            # NOT plain `trunk serve` - that is a debug wasm
```

`session_rust` is referenced at `../../../session_rust`, i.e. the kernel in this repo.

## The headless harness

`examples/selftest.rs` + `src/selftest.rs` render a frame without a window, which is how every
claim in Part 2 was checked. It is part of the lesson, not a side tool.

```bash
cargo build --release --example selftest --target x86_64-unknown-linux-gnu
VIEWER_W=900 VIEWER_H=750 VIEWER_ZOOM=19 VIEWER_ORBIT="10,-8" \
  ./target/x86_64-unknown-linux-gnu/release/examples/selftest out.ppm assets/scenes/bunny.json
```

| knob | what it does |
|---|---|
| `VIEWER_ORBIT="dx,dy"` | orbit before framing — most of these defects only appear at some angles |
| `VIEWER_ZOOM=n` | dolly after framing; negative zooms out |
| `VIEWER_LINE_STYLE=flat\|tubes` | which lane draws mesh edges |
| `VIEWER_NO_DEPTH=1` | forces the ink's depth compare to `Always` — the **oracle** for the acceptance test |
| a `.json` argument | resolved as a scene manifest, the way the browser does |

## The four checks that must keep passing

The mesh edge lane is easy to break and hard to eyeball. After any change to
`ribbon.wgsl` / `sphere.wgsl` / `cylinder.wgsl`, re-run all four:

| check | expected |
|---|---|
| ink depth on vs `VIEWER_NO_DEPTH=1`, zoom 19 | ~12 differing px of 675,000 |
| marker rim vs the same oracle | 394 = 394 |
| flat vs tube pen width on a box edge | 4 px = 4 px |
| a 2D drawing sheet | 52,244 ink px, unchanged |
