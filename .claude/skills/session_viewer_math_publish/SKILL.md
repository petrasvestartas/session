---
name: session_viewer_math_publish
description: Check, preview and publish a session viewer stage deck - the test that plays every step, local preview, uploading YAML and pictures to R2, and the live links. Use before saying a presentation is done, and when asked to publish it.
argument-hint: [presentation name]
---

Work from `session/session_viewer`. Heavy commands go through the build slot (superproject
`CLAUDE.md`).

## 1. Test: every step of every deck

```bash
bash ../../wood/tools/buildslot.sh --run stage env CARGO_BUILD_JOBS=8 timeout 590 \
  cargo test --target x86_64-pc-windows-msvc --lib stage::
```

It fails with the chapter and step on an unknown name, an unknown TeX command, or overlapping
blocks. A new presentation folder must be added to `DECKS` in `src/app/stage/tests.rs`.

## 2. Look at it

```bash
bash ../../wood/tools/buildslot.sh stage env CARGO_BUILD_JOBS=8 trunk serve
# http://127.0.0.1:8080/?stage=decks/<name>/index.yaml&data=off     add &chapter=3&step=2 to jump
```

Trunk copies `examples/stage` into the site as `decks/` and reloads on save. Step through every
slide you wrote or changed: a passing test does not show a label sitting on a line or a sentence
that reads badly. Check a phone-sized upright window too (about 400 x 800): the 3D side is on top.
A sentence longer than three lines at the top of the column runs into the equations: shorten it.

## 3. Publish the slides (only when asked)

The live site reads decks from R2, so a YAML change needs no viewer build:

```bash
E="--profile r2 --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com"
aws s3 sync examples/stage s3://session-viewer-data/stage/ --delete --exclude '*' --include '*.yaml' \
  --content-type application/yaml --cache-control no-cache $E
aws s3 sync examples/stage s3://session-viewer-data/stage/ --exclude '*' --include '*.png' \
  --content-type image/png --cache-control no-cache $E
```

Link: `https://petrasvestartas.github.io/session/?stage=<name>`.

## 4. When Rust changed

A new key or block works on the live site only after the viewer is pushed and the `viewer-pages`
workflow has deployed (about 20 minutes). Upload slides that use it **after** that deploy, or the
live deck shows an error. Push once and wait: each push cancels the deploy before it.

## Done means

- the test passes;
- you looked at the changed slides at desktop and phone size;
- the "next" pointers of neighbouring presentations and the list in
  `examples/stage/README.md` still name the right presentations;
- the link opens at its first slide with no red error line.
