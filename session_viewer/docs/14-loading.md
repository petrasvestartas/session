# 14 · Load and replace complete scenes

**Start:** checkpoint 13. **Finish:** manifests and protobuf data load through the production path; invalid or stale replacements do not destroy the last valid scene.

## A manifest describes placement; protobuf carries geometry

The loader accepts the existing TOML/YAML/JSON scene descriptions and Session protobuf payloads. A manifest can refer to several geometry files with different placements and styles. Keep these concerns separate so placement-only changes can reuse immutable decoded geometry later.

```text
route → fetch manifest → validate items → fetch/decode source payload
                                                 ↓
                                    staged PendingDocument
                                                 ↓ generation still current?
                               Scene replacement / typed uploads
```

Validate before allocating from untrusted counts or indexing geometry. Check payload storage lengths, indices, transforms and required fields. A malformed document should report an error and preserve the last valid scene, not leave half a replacement installed.

## Two orders matter

The manifest establishes document order. Network responses can finish in another order. Stage results with their intended positions rather than letting response timing reorder source identities.

Request generations establish which replacement is newest. An older slow response arriving last must not replace a newer completed scene. Cancellation reduces wasted work, but generation checks are still required because a completion can race with cancellation.

## Callbacks have owners

Winit owns browser input registration. Fetch/deadline and live-source helpers retain their callback handles, abort controllers and registrations. Dropping the owner detaches or cancels them. Repeated `Closure::forget()` is not a lifecycle strategy for reloadable work.

An asynchronous completion posts a named application message and requests a redraw. Idle scenes do not continuously submit color frames merely to poll for potential work.

## Text belongs in the same scene model

At checkpoint 14, manifest text includes content, world position and height and creates fixed-plane text. Chapter 17 adds the `camera_facing: true` billboard option, along with stable source rows for authored text and generated document titles. Both orientations then share geometry's selection/hide lifecycle.

## Write the files

Follow [Complete file changes for 14](../lessons/14/index.md). Read the manifest records and validation, then the fetch/decode functions, staged loader and application message handling. Keep the provided local teaching manifest; personal remote credentials are not required.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. The local fixture loads through the real manifest/protobuf path. Verify selection and controls still work after loading.

The maintained loading test supplies a malformed replacement, deliberately reverses response completion order and alternates scenes repeatedly. The final visible scene must be the newest valid one, and equal replacement workloads must settle at equal owned GPU capacities. The browser allocator retaining WASM pages is not itself evidence that source documents are still alive.

**Before continuing:** explain the difference between canceling a request and rejecting a stale completion. Continue to [publication and reuse](15-publication.md).
