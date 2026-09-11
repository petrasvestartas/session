# 13 · Source controls

## You are building

![Diagram: selected parent · Controls { points, links } from the source geometry · controls glyph lane · control_net segment lane · Pick { row = parent, sub = control index } · ControlId::Vertex / Curve / Surface / Point · selected marker turns yellow](illustrations/13-01.svg)

Streamed clouds display a bounded prefix, so a click must ask the source, not the screen:

![A resident prefix is a fraction of the cloud, so a click walks every intersecting octree node whether or not it was downloaded, and accumulates the answer one bounded page at a time against the depth the frame already has.](illustrations/cloud-pick.svg)

![Diagram: click · eligible source node ranges (octree ∩ click window) · fetch one bounded page (HTTP Range) · candidates within the window · GPU ID pass accumulates nearest visible point · range-read original fixed32 ID + exact position…](illustrations/13-02.svg)

![The screen draws a curve as chords and a surface as a grid; F10 shows the source controls, and a picked marker answers with a ControlId into the source.](illustrations/controls.svg)

## Starting point

- Checkpoint 12: clicks select objects and edges. `Gpu` already owns empty `controls` and `control_net` lanes.
- A display vertex is not a control: a curve draws hundreds of chords from a few control points. F10 must show the original ones.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1, 2 and 6; steps 3–5 fail and build again at step 6.

<!-- step-status: end -->

## Step 1 · Control identities

![Where this step sits in the viewer: Scene + walk, with 10 of 11 zones built so far.](illustrations/locator-7ccf7d4f74.svg){ .locator data-strip="illustrations/strip-6f8f40e8fe.svg" }

- `ControlId` names a control inside its parent's source geometry; the GPU slot it landed in is temporary.
- `enable_controls` is idempotent: pressing F10 on the same parent does nothing, so markers are never duplicated.

![Diagram: selected parent · Controls · ControlId](illustrations/13-03.svg)

<span class="zone-mark" data-strip="illustrations/strip-6f8f40e8fe.svg" data-zone="Scene + walk"></span>

<!-- file: 13 session_viewer/src/app/selection.rs type hunks=1 -->

- `Controls::from_geometry` reads source data: mesh vertex keys, BRep vertices, curve and surface control nets with links; never tessellation vertices.

<span class="zone-mark" data-strip="illustrations/strip-6f8f40e8fe.svg" data-zone="Scene + walk"></span>

<!-- file: 13 session_viewer/src/app/selection.rs type hunks=2 -->

## Step 2 · Fetching and source-query records

![Where this step sits in the viewer: Network, with 10 of 11 zones built so far.](illustrations/locator-f20b36578b.svg){ .locator data-strip="illustrations/strip-2c1e2b3b5e.svg" }

These two modules are new and undeclared, so the crate still builds after them.

- `fetch::get` refuses a `200` answer to a `Range` request: that would be the whole file.
- Every request owns a deadline; dropping it clears the timer.

![Diagram: click · QueryView · eligible_ranges · source page · Query token](illustrations/13-04.svg)

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/fetch.rs type lines=1-38 -->

- `get` treats any HTTP status as success and only a network failure as an error: a 304 or a 404 is the caller's decision.
- So one function serves the live source's conditional read and the loader's download.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/fetch.rs type lines=39-121 -->

- `content_length` is a HEAD request: a file's download size before a byte is fetched, so a scene can refuse what the device cannot hold.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/fetch.rs copy lines=122-228 -->

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/fetch.rs copy lines=229-248 -->

- `QueryView` freezes the click's projection; every page is tested against the same matrix and pixel window.
- A cube crossing the eye plane cannot be excluded, so `intersects` returns true for it.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=1-52 -->

- A source query may look at more nodes than it needed; it must never skip one that held the answer.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=53-92 -->

- A cube whose projection is not finite is kept and tested the slow way: the arithmetic that would reject it is the arithmetic that failed.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=93-124 -->

- `eligible_ranges` walks every octree node, resident or not; when the node table misses rows it falls back to a full bounded scan.
- `fetch_page` spawns one bounded range read per page and posts the result back; the token is checked at the callback, so a page that lands after you clicked elsewhere is discarded rather than folded in.
- `read_page` derives the byte range from the coordinate array's offset and reads it under the source revision, so a file republished mid-query is refused rather than mixed.
- `source_position` demands exactly 24 bytes and a value still finite after the cast to f32, which is all a GPU row can hold.
- `page_candidates` keeps only the points whose projection lands inside the frozen click window; ranking across pages stays on the GPU.
- `resolve_id` re-reads two small ranges for the winner alone: its original id and its exact position.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=125-195 -->

- `Query` owns the cancellation token; superseding input drops the query and every callback in flight checks the token before posting.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=196-251 -->

- Cancellation is an ownership property rather than a flag someone must remember to set.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs type lines=252-270 -->

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs copy lines=271-368 -->

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/cloud_query.rs copy lines=369-556 -->

<!-- check: 13 -->

## Step 3 · Wire parsing for streamed clouds

![Where this step sits in the viewer: Network, with 10 of 11 zones built so far.](illustrations/locator-f20b36578b.svg){ .locator data-strip="illustrations/strip-2c1e2b3b5e.svg" }

- `varint` and `skip_scalar` read the protobuf wire format by hand. The viewer wants one array's byte offset, not the message: decoding the message is the thing streaming exists to avoid.
- `descend_message` walks into the wanted field and requires every container to close exactly where its length says. A file that disagrees with itself is refused before a single range is requested.
- `walk_to_coords` and `cloud_layout` return `coords`' absolute offset and length from the first few kilobytes, so the point count is known before a byte of payload is fetched.
- `CloudLod::set_field` decodes one packed octree array per field number; `valid` then checks the whole table at once. After that the scene can index nodes and children without a bounds test per access.
- `bounded_range`, `body_end` and `checked_positions` are the range guards: a slice must be whole coordinate triples, land inside the file, and stay inside the array it belongs to.
- Long, and worth reading rather than typing: it is one wire-format reader and its guards, and nothing above this line in the course parses bytes.

![Diagram: cloud .pb header · CloudFields · point count](illustrations/13-05.svg)

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/stream.rs copy -->

## Step 4 · Picking controls

![Where this step sits in the viewer: GPU core, Lanes, with 10 of 11 zones built so far.](illustrations/locator-43ed20e7f8.svg){ .locator data-strip="illustrations/strip-187e4e26b4.svg" }

- `PickMode::Controls` restricts the ID pass to the temporary control markers of one parent.
- A source query keeps the physical depth and accumulates point IDs across pages: the first page clears the IDs, later pages load them.

![Diagram: PickMode::Controls · id_pass · control markers · pick · source page](illustrations/13-06.svg)

<span class="zone-mark" data-strip="illustrations/strip-ccdfd9e2ff.svg" data-zone="Lanes"></span>

<!-- file: 13 session_viewer/src/engine/gpu/pick.rs type -->

- Control markers draw after the silhouette; a source-query page draws only source-point IDs and returns before the ordinary ink IDs.

<span class="zone-mark" data-strip="illustrations/strip-68dea8ec67.svg" data-zone="GPU core"></span>

<!-- file: 13 session_viewer/src/engine/gpu/render.rs type -->

## Step 5 · State transitions

![Where this step sits in the viewer: State, with 10 of 11 zones built so far.](illustrations/locator-3fc75276ea.svg){ .locator data-strip="illustrations/strip-0fc6abc083.svg" }

- `controls` are the current parent's source controls; `cloud_query` is the in-flight page loop.

![Diagram: F10 · upload_controls · apply_control · selected control](illustrations/13-07.svg)

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=1-3 -->

- Clearing, resizing, touching and selecting all reset controls.
- Marker size follows the logical-to-physical scale, so a resize re-uploads them.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=4-8 -->

- A control answer is accepted only when its row is still the active parent.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=9-10 -->

- While a source-query page owns the GPU readback, no colour frame is presented: that page is an ID target, not a picture.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=11-13 -->

- A click in control mode picks controls; a click on a streamed cloud's controls starts the page loop instead.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=14-14 -->

- `enable_controls` reads the source geometry once; a display-only object without source reports that instead of inventing controls.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=15-15 -->

- `upload_controls` resets both lanes before appending, which is what makes repeated F10 idempotent.
- `apply_control` decodes the marker's sub-ID tag; a cloud control resolves through the cloud lane's row map.
- The page loop: `start_cloud_query` → `advance_cloud_query` → `cloud_query_batch` (upload candidates as ID targets, request a pick) → `apply_cloud_query_pick` (fold the winner) → next page → `cloud_query_resolved`.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=16-16 -->

- The selected name is hidden while controls are shown; the inspection snapshot lists the controls.

<span class="zone-mark" data-strip="illustrations/strip-0fc6abc083.svg" data-zone="State"></span>

<!-- file: 13 session_viewer/src/state.rs type hunks=17-19 -->

## Step 6 · Key, message and loader wiring

![Where this step sits in the viewer: Network, Scene + walk, Shell, Input, with 10 of 11 zones built so far.](illustrations/locator-2c5303abc3.svg){ .locator data-strip="illustrations/strip-7e4f59eba1.svg" }

![Diagram: F10 · Escape · Input · State · loader](illustrations/13-08.svg)

<span class="zone-mark" data-strip="illustrations/strip-25545ebdc0.svg" data-zone="Input"></span>

<!-- file: 13 session_viewer/src/app/input.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-56723afb3a.svg" data-zone="Shell"></span>

<!-- file: 13 session_viewer/src/lib.rs type -->

- A feature that talks to the network has this shape: one more `Msg` the event loop routes.

<span class="zone-mark" data-strip="illustrations/strip-56723afb3a.svg" data-zone="Shell"></span>

<!-- file: 13 session_viewer/src/app/mod.rs type -->

- `fetch` is the viewer's first code that talks to a server; loading stays local until lesson 14.

<span class="zone-mark" data-strip="illustrations/strip-56723afb3a.svg" data-zone="Shell"></span>

<!-- file: 13 session_viewer/src/app/inspection.rs copy -->

- The streamed fixture is opt-in (`?scene=stream-test.yaml`) and reads a bounded display prefix while keeping every source row reachable.

<span class="zone-mark" data-strip="illustrations/strip-2c1e2b3b5e.svg" data-zone="Network"></span>

<!-- file: 13 session_viewer/src/app/loader.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-6f8f40e8fe.svg" data-zone="Scene + walk"></span>

<!-- file: 13 session_viewer/src/app/scene.rs copy -->

## Check

<!-- checkpoint: 13 -->

Expected:

- Select the curve, press F10: its control points appear as blue markers joined by the control polygon.
- Click a marker: it turns yellow and the status names its `ControlId`.
- Press F10 again: nothing is added.
- Escape: markers disappear, the parent stays selected. Escape again: nothing selected.
- Select the surface and press F10: a control net, not the tessellation grid.

![Checkpoint 13, left to right: F10 on the curve shows its three control points and control polygon; F10 on the surface shows the four corners of its control net; a clicked corner turns yellow and the status reads `Selected Surface { surface: 0, u: 0, v: 1 }`; F10 on the mesh shows its original vertices, not the tessellation.](screenshots/13-controls.png)

## What changed

<!-- tree: 13 session_viewer/src -->

- `SelectionMode::Controls` joins `Object` and `Edge`; all three keep exactly one parent.
- Data flow: source geometry → `Controls` → temporary marker rows → ID pass → `ControlId`.
- Streamed clouds: click → eligible source ranges → bounded pages → GPU visibility per page → original fixed32 ID.

**Production equivalent:** `src/app/selection.rs`, `src/app/cloud_query.rs`, `src/app/fetch.rs`, `src/app/stream.rs`; production keeps the page-loop methods in `src/state/cloud_query.rs`.

## Try

- Select the polyline and press F10: every vertex is a control, so the markers sit on the display corners; select the line: two markers.
- With controls shown, drag the window edge to resize: the markers are re-uploaded at the new logical-to-physical scale and keep their size.
- Select a NURBS curve and press F10, then count the markers against the chords you can see: the markers are the control net the document carries, not the samples the tessellator chose.
- Press F10 on an object with no source geometry (a document title plate): the status says so instead of inventing controls.
- The streamed path needs a point cloud served over HTTP and this checkpoint has no fixture: `?scene=stream-test.yaml` wants a `?data=` base URL holding a large `cloud.pb`, which the course never supplies. Lesson 15 publishes real scenes — watch the page loop there in the network panel.
- Clear the selection while a page loop is running (Escape twice): the query token is dropped and no late page selects anything.

## Questions and answers

**A curve is drawn as hundreds of chords. Why can F10 not just show the vertices that were drawn?**

*How to work it out.* A display vertex is a sample the tessellator chose at this tolerance; change the tolerance and there is a different set. Now ask what the user would do with a marker on one — drag it? It corresponds to nothing in the document.

*The answer.* Display vertices are an approximation artefact with no identity and no meaning under editing. `Controls::from_geometry` reads the real control net from the source — a handful of points with links. Same rule as the tessellation seam in lesson 06: never let the display invent an identity.

**`fetch::get` refuses a `200` answer to a `Range` request. Why is that worth a check rather than a trusting read?**

*How to work it out.* `206` is the slice you asked for. `200` means the server ignored the range and is sending the whole file — possibly gigabytes to a device that asked for 64 KB. Nothing about that response is an error, so no other layer will object.

*The answer.* The failure is silent until memory runs out, and the check is one comparison. When you ask for less than everything, verify that you got less than everything.

**A streamed cloud displays a bounded prefix, so a click cannot be answered from the screen. What does the viewer do instead?**

*How to work it out.* The points you want may never have been downloaded, so the answer must come from the source. Paging takes time, and the camera may move meanwhile. Ask which camera each page should be tested against.

*The answer.* The click's projection is frozen into a `QueryView`, and every octree node — resident or not — is tested against that same matrix and pixel window while ids accumulate across pages. Freezing matters: with the live camera, later pages would be answering a different question than the first.

**`enable_controls` is idempotent. Name the bug that makes that worth stating explicitly.**

*How to work it out.* Ask what pressing F10 twice would do without it: run the upload again, adding a second set of markers on top of the first. Identical positions, so the screen looks almost the same.

*The answer.* Doubled ink, doubled pick answers and a marker count that grows until you select something else. Idempotence is cheap and the failure is silent — exactly when to write the invariant down.

**What you should be able to do now**

Describe the cancellation story and contrast it with lesson 12's generation counter. Correct: `Query` owns a cancellation token, superseding input drops the query, and every callback checks the token before posting — so a page that arrives after you clicked elsewhere is discarded at the callback. A generation *labels* answers so a stale one is recognised; a token *cancels* work still in flight. You need both: generations cannot stop a fetch, and a token cannot label an answer already on its way back.

## Next

[14 · Loading scenes](14-loading.md): manifests, protobuf documents, validation and safe replacement through the real loader.
