# Bound large scenes and live updates

Display a bounded prefix of a large scene while keeping later reads, selection queries and scene replacement tied to the correct source.

## Prerequisites and ownership

Complete decoding, loading, document ownership and input. This application chapter teaches `app/stream.rs`, `range_gate.rs`, `cloud_query.rs`, `sheet_query.rs`, `live.rs`, `live_revision.rs` and their `state/cloud_query.rs` and `state/sheet_query.rs` adapters. The rendering subject owns resident cloud/sheet rows and their GPU lanes; this chapter connects remote ranges and source identities to those completed representations.

## Build

Connect bounded range readers and query messages to the existing loader, retaining source revision and cancellation state across awaits.

### A displayed prefix is not a complete editable document

`CloudFields` records byte positions and lengths of positions, colors, normals and original IDs in a protobuf file. `SheetFields` records segment endpoints, colors, widths, entity IDs and side-table metadata. Their `revision` field binds subsequent reads to the source ETag.

`stream::field_at` parses a bounded field header, checks field number and wire type, and computes the next position within the containing message. The array descriptors validate element sizes and counts before range reads address them. A field offset is not trusted simply because a varint parsed successfully; length and end arithmetic must also stay inside the message.

The loader first chooses a bounded prefix, then sends additional chunks while the scene's point/segment budget permits. It reserves budget before a fetch and rolls back the reservation when positions fail in the same generation. A stopped loop reports that it reached a ceiling or failed a range; it does not claim the prefix contains every source point.

The resident and stream adapters under `app/walk/cloud/` distinguish a complete kernel cloud from fetched point arrays. Sheet rows use the sheet adapter and source entity IDs. Neither display prefix automatically becomes a complete kernel document that arbitrary editing commands can safely rewrite.

### Bound concurrency with a future and a permit

`RangeGate` limits outstanding reads and queues later ones in FIFO order. `Enter` implements `Future`: polling either returns a permit, registers a waker and returns Pending, or returns None after the gate closes. A waker asks the executor to poll again; it is not a thread blocked inside the request.

`Permit::drop` hands the slot to the next queued turn. Dropping a waiting `Enter` removes that turn, or passes on a slot already assigned to it. Those two cancellation paths prevent a cancelled request from consuming capacity forever. Closing a scene's gate wakes waiting reads so they can observe closure instead of waiting indefinitely.

### A new scene invalidates old work

```rust
--8<-- "session_viewer/src/app/loader.rs:loading-generations"
```

The stream loop checks its saved generation before fetching and again before posting a chunk. The second check matters: clearing the scene while a request is in flight cannot cancel the elapsed network work, but it must prevent its result from entering the replacement scene. Resetting budgets and the range gate belongs to this same transition.

File ETags and scene generations answer different questions. An ETag helps prevent combining ranges from different file revisions. A generation prevents adopting work for an obsolete application scene. Both can be correct individually while the other has changed.

### Query source points beyond the resident prefix

`cloud_query.rs::QueryView` records the click, view, placement and clipping facts used by a point query. The query narrows candidate ranges using the source LOD bounds. Invalid or incomplete LOD coverage falls back conservatively to all source rows; an invalid index must not silently make points unselectable.

`Query::next_page` reads at most 65,536 points at a time. Candidate points near the click are sent for GPU ranking, with an `awaiting_gpu` state separating a posted batch from its answer. Original point IDs are read from their exact stored bytes rather than inferred from the current display-prefix index.

`state/cloud_query.rs` starts this work only in the appropriate cloud control-selection mode. A changed view or selection drops the query and cancels the GPU pick. `Query::drop` sets a shared cancellation flag so a future retaining its request data can see that it is obsolete. The final result must match the active query and parent before becoming a control selection.

### Resolve sheet identity without loading every record

Sheet geometry can carry an entity ID while its human-readable metadata remains in a side table. `sheet_query.rs` validates an `SHM1` header, the record count, fixed-size offset/length records and bounded JSON blobs. The table revision applies to later record reads too.

The state adapter selects the visible entity immediately, then fetches its name and kind when a metadata URL exists. Clicking the same entity again clears it. A newer selection advances the query generation, preventing a late metadata response from relabeling the current selection with the previous entity's name.

### Live notification is a reason to check, not replacement data

`live.rs` owns a notification connection and a polling fallback. The event callback marks that a check is needed; the loader still fetches and validates the actual source. The retained closure keeps the callback alive, and `Notify::drop` removes it and closes the event stream when its owner goes away.

`live_revision.rs::Revisions` retains HTTP validators separately from hashes of received bytes. A changed or missing ETag does not by itself require replacing an identical scene. Conversely, changed bytes must be noticed even when a server reuses a validator. The hash is an in-process change detector, not a cryptographic proof attached to published assets.

Replacement loading stages results and uses the load generation described in the loading chapter. Camera and UI state must not be reset merely because a poll returned unchanged content. A notification, successful GET and adopted new scene are three distinct events to observe.

### Adopt only a complete live replacement

A live-source tick runs after the polling interval or a change notification. It revalidates the manifest and mutable files, retaining decoded immutable revision URLs. A changed manifest or file marks a replacement pending; a failed read or an incomplete set of decoded documents keeps the last valid scene visible. A complete replacement reuses cached sessions where possible and drops files no longer listed. The notification relay connects after the first scene is ready. Notifications request an earlier check; they do not supply trusted geometry or bypass decoding.

### Convert a resident source slice into GPU rows

A streamed cloud slice appends positions and a bounded matching color column, padding missing colors with black. Normals are accepted only when the slice has one per point; otherwise the draw carries the no-normals sentinel. Only the first slice installs the full LOD node table. Each draw retains both its original source offset and its GPU offset, while spacing comes from the finest fully resident node or a fallback. This preserves the distinction between source IDs and compact resident storage.

### Validate streamed field boundaries

The stream parser separates a protobuf field's header, body and next offset using checked bounds. Length-delimited fields cannot advance past their containing message. Sheet coordinates use six doubles per segment; optional colours, widths and IDs must each contain four bytes per segment. The advertised segment count must agree, metadata must be bounded UTF-8, and known fields with the wrong wire type are rejected. Cloud LOD arrays validate packed-double alignment and complete varints before decoding. This metadata validation protects later range reads; a successfully parsed header does not prove that the remaining payload has arrived.

## Run and observe

Serve the cumulative application's `production.html` build. From `session_viewer`, run `node tests/streamed-controls.cjs` through the resource guard, with `VIEWER_URL` pointing to that server and `VIEWER_TEST_OUTPUT` naming a new capture directory. Set `CHROME_BIN`, `VIEWER_HEADLESS` and `VIEWER_CHROME_ARGS` for the recorded local WebGPU configuration; `NODE_PATH` locates Playwright.

The check supplies its own range-capable virtual source of 6,065,539 points without writing a full large file. Only 250,000 points become resident. It types `View Top`, selects the cloud and types `Controls`, then queries beyond the displayed prefix. The expected visible result is one yellow source point at `[-1,0,10]`, with original ID `0xfedcba98`; the inspector must still report 250,000 resident points. The held final page proves that an earlier candidate is not committed before all eligible source points have been checked.

Select a source point or sheet entity, then change the view or selection before its query finishes. The earlier answer must not replace the new selection. Record range responses, source revision and visible result together. A server returning the entire file for every range is not a successful streaming test.

Run `node tests/live-revision.cjs` with the same browser environment. Four polls alternate strong, weak and absent ETags while serving identical manifest bytes. The geometry downloads once and the scene is adopted once. The server then adds an authored text item: the next poll adopts that changed manifest, retains the camera and reuses the immutable geometry. Inspect the recorded text-label state to distinguish adoption from an unchanged poll; the small label's presence in inspection is not a pixel-legibility assertion for the floor view.

## Predict and try

Predict what happens to a queued range read when its scene is cleared. Pause responses in developer tools, request another scene, then release the old responses. Closing the gate wakes waiting requests and the generation checks reject old chunks. The visible replacement scene should remain authoritative even when the old network request completes later.

## Question and answer

Why retain both original point IDs and the displayed prefix index?

The prefix is a bounded representation that can grow or differ from source order. Its temporary row position is useful for rendering, while the original ID identifies the point in the complete source. Confusing them would select a different point when residency or LOD changes.

## Validation

The actual streamed-controls browser check passes at DPR 1 and 2: bounded residency and range reads, exact nonresident source ID, exhaustive eligible-page completion, a visible yellow point and cancellation of an asynchronous query. Before cancellation it creates a fresh control selection; clicking an already selected handle would exercise dragging. The earlier input/setup failures remain recorded. Live revision checking separately verifies four unchanged polls and one changed-manifest adoption, with one geometry download and an unchanged camera. Notification delivery, replacement geometry, sheet metadata delivery and physical-device acceptance remain separate requirements.
