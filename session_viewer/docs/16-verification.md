# 16 · Account for resources and keep useful tests

**Start:** checkpoint 15. **Finish:** the production ownership/accounting and complete verification harness are present. Two final lessons extend this checkpoint with source-face/text selection, silhouettes and finite triangle visibility.

## Measure quantities that mean different things

| Observation | What it measures | What it does not prove |
|---|---|---|
| Source payload | Known retained geometry fields, with shared documents deduplicated | Every allocation in the process |
| WASM capacity | Allocated linear-memory pages | Live source ownership or a leak |
| Owned GPU buffers | Actual exposed buffer capacities | Driver/swapchain/private-library allocations |
| Texture payload estimate | Dimensions × format bytes × samples | Exact VRAM consumption |
| rAF/submission timing | Browser scheduling or submitted work | Physical display latency |

```text
retained Rc<Session> ── weak identity cache ── known source payload
owned buffers ────────────────────────────── exposed GPU capacity
texture descriptors ──────────────────────── estimated payload
WASM memory pages ────────────────────────── linear-memory capacity
                             report separately
```

`SourceCache` keeps `Weak<Session>` identities. A weak reference recognizes a document without extending its lifetime. If the same immutable documents remain, the cache reuses its previous count; source replacement changes identity and triggers a new scan.

Future in-place editing must explicitly invalidate this cache or replace the `Rc` document. Mutating data behind an identity-only cache would make its measurements stale.

## Tests follow the production renderer

The native selftest creates an offscreen target and runs the same GPU modules. It is a test harness, not a second native application. Browser tests exercise actual input, asynchronous loading, canvas sizes and visible pixels. Geometry parity tests exercise independent producer implementations.

Keep these distinctions when reading results. A native shader test does not prove a browser adapter works. A browser screenshot does not establish CAD topology correctness. A source-hash comparison does not establish visual quality. Use each check for the claim it supports.

## Write the files

Follow [Complete file changes for 16](../lessons/16/index.md). Type/read the source-accounting cache and its inspection integration. Copy the complete native/browser fixture tools; their assertions are useful reading when diagnosing a future regression.

The file list explicitly removes the remaining obsolete teaching files. Do not retain a second renderer beside the final modules. Keep font licenses and intentional fixtures; those are dependencies of tests and documentation.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>, verify the local scene, then stop Trunk. Run the native and browser-target checks separately:

```sh
cargo fmt --package session_viewer -- --check
cargo clippy --locked --target wasm32-unknown-unknown --lib -- -D warnings
cargo clippy --locked --target x86_64-unknown-linux-gnu --all-targets -- -D warnings
cargo test --locked --target x86_64-unknown-linux-gnu -- --include-ignored --test-threads=1
python3 tests/publication.py
```

The explicit native target is the recorded Linux host target; use your installed native target on another platform. Ignored tests include real GPU tests, so a usable native adapter is required. WASM `--all-targets` is not the browser gate: native-only examples intentionally import the native selftest module.

Read [Verification tools](../tests/README.md) for exact scene, text, lifecycle and source-parity commands. Each tool says which external fixture or environment it requires. Missing hardware/data is unverified coverage, not a passing test.

## Before the final two lessons

Checkpoint 16 is the earlier production state. Do not run final convergence yet: the current viewer includes the later behavior in 17 and 18. The historical timing records in [Measurements](measurements.md) remain explicitly tied to the version measured.

**Before continuing:** explain why keeping WASM memory pages after scene replacement is different from retaining the old Session object. Continue to [source faces, text and outlines](17-source-presentation.md).
