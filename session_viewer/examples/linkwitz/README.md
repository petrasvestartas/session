# Linkwitz force-density presentation

This example teaches Klaus Linkwitz's force-density method with ordinary Session geometry, text, matrices, graph relationships, and transforms. Each reveal is serialized as a protobuf session. The existing `session_viewer` Rust application and WGSL shaders render it; no Manim, MathJax, SVG equation renderer, or second scene format is involved.

[Open the 36-slide presentation](https://brg-teaching.github.io/Teaching/CAS/website/session-viewer/?presentation=force-density/presentation.yaml). Use Left Arrow and Right Arrow. Drag the left region to orbit and use the wheel to zoom; the mathematics stays in slide space.

## Files

- `force_density.py`: editable source for the complete 36-reveal deck.
- `generated/presentation.yaml`: topic/reveal order, effects, links, and the protobuf references.
- `generated/steps/*.pb`: exactly the 36 referenced protobuf sessions.
- `one_slide.py`: the smallest complete authoring example, with one topic, one reveal, and one protobuf.
- `generated-one-slide/`: the tested one-slide export.
- `AUTHORING.md`: Session, text, matrix, reveal, morph, and publication examples.

The Teaching guide is [Create one presentation slide](https://github.com/BRG-teaching/Teaching/blob/main/CAS/website/session-viewer-guide/one-slide.md). Its source is also available beside the deployed guide as `one_slide.py`.

## Force-polygon repair

The first slide constructs each force from the 3D camera's orthographic right and screen-up axes. The authored vertical component is negated before the existing slide-plane reflection, so every polygon arrow has the same on-screen direction as its corresponding cable. The external load uses the same projection and closes the head-to-tail sum.

The morph works in local slide coordinates. It moves first without shortening the arrow, then turns and scales into the polygon. Its center is constrained with arrowhead clearance. Completion restores the authored target transform rather than leaving a nearly finished animation transform.

The browser tests inspect both the intended morph and the actual transformed line endpoints. They check normalized cable/polygon direction alignment of at least `0.999`, closure, and on-canvas trajectories. This prevents correct metadata from hiding a wrong rendered arrow.

Free nodes, solution annotations, and positive presentation-matrix cells use the website's pink `#dc4b8d` (`Color(220 / 255, 75 / 255, 141 / 255)`). Fixed nodes stay blue, the applied load stays green, negative matrix cells stay blue, and conventional red/green/blue coordinate accents remain unchanged. The positive presentation-matrix color is set in WGSL; the ordinary non-presentation matrix renderer is unchanged.

A separate animation-startup fix lets text-and-matrix-only decks finish their effects without waiting forever for a geometry frame. Topics with geometry still wait for the first geometry to be shown.

## Regenerate

Close the old local viewer first, especially on Windows. The exporter removes stale protobuf files before writing the new deck; an open file can interrupt that replacement and leave an incomplete deck.

From `session/session_py`, with the configured offline environment:

```bash
uv run --offline --no-sync python - <<'PY'
import runpy
from pathlib import Path
from session_py.viewer import serve

path = Path('../session_viewer/examples/linkwitz/force_density.py').resolve()
namespace = runpy.run_path(str(path))
viewer = serve(namespace['deck'], directory=path.with_name('generated'))
viewer.close()
print('Regenerated', path.with_name('generated'))
PY
```

Check that the manifest references exactly 36 unique files, that `generated/steps/` contains no extras, and that every file is nonempty before testing or publishing.

## Verify

Build from `session/session_viewer` only when the Rust or WGSL source changes:

```bash
bash ../../wood/tools/buildslot.sh viewer-open env CARGO_BUILD_JOBS=8 NO_COLOR=false trunk build
```

Run the real Chrome/WebGPU checks from the same directory:

```bash
NODE_PATH=target/presentation-tools/node_modules node tests/presentation.cjs
NODE_PATH=target/presentation-tools/node_modules node tests/presentation-quality.cjs
NODE_PATH=target/presentation-tools/node_modules node tests/presentation-one-slide.cjs
```

The first command covers all 36 reveals and navigation. The second checks four viewport/device-pixel-ratio cases. The third verifies the complete one-slide example, including its equation reveal and finished animation clock. Inspect `target/presentation/step-00.png`, `morph-mid.png`, and `one-slide.png` as well as the assertions.

## Publish

Deploy `dist/` as the contents of `CAS/website/session-viewer/`, and deploy `generated/` as its `force-density/` child. The manifest's `steps/` paths are relative to `force-density/presentation.yaml`; do not add another `generated` component.

Preserve `bridge.js` and its `<script src="bridge.js"></script>` tag. Remove superseded hashed runtime files, but do not publish the unrelated generated `pb/`, `text-quality.html`, or `view_local.yaml` from `dist`. Keep only the manifest-referenced protobufs. Commit source changes separately from the generated Teaching deployment.

The book is mirrored from the existing `force-density-linkwitz` folder on the public R2 bucket. Teaching's `CAS/website/tools/mirror_books.py` restores its catalogue entry when the upstream map omits it; private local study copies are not committed into Teaching.

## Book-concept review

The slide-by-slide review uses Linkwitz's Chapter 6, “Force density method: design of a timber shell,” and the accompanying matrix and reading notes. It checks equations, signs, units, rendered examples, and conceptual coverage independently of the animation repair. Review recommendations are not silently applied as a wholesale rewrite of this deck.

See [REVIEW.md](REVIEW.md) for all 36 completed slide reviews, the consolidated mathematical and presentation findings, and separate review prompts. Every slide had its own read-only review agent.
