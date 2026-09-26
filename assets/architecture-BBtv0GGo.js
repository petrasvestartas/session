const e={title:"Session Viewer architecture",html:`<h1 id="session-viewer-architecture">Session Viewer architecture<a class="anchor" href="#/course/architecture#session-viewer-architecture" aria-label="Link to this section">#</a></h1>
<p>Reference for the finished viewer: one browser application, Rust compiled to WebAssembly, winit for input, wgpu over browser WebGPU. The <a href="#/course/readme">course</a> builds it from an empty crate; this page describes the result.</p>
<h2 id="module-graph">Module graph<a class="anchor" href="#/course/architecture#module-graph" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>%%{init: {&quot;flowchart&quot;: {&quot;useMaxWidth&quot;: false, &quot;wrappingWidth&quot;: 320}, &quot;themeVariables&quot;: {&quot;primaryColor&quot;: &quot;#ffffff&quot;, &quot;primaryTextColor&quot;: &quot;#111111&quot;, &quot;primaryBorderColor&quot;: &quot;#111111&quot;, &quot;lineColor&quot;: &quot;#ffffff&quot;, &quot;secondaryColor&quot;: &quot;#ffffff&quot;, &quot;tertiaryColor&quot;: &quot;#ffffff&quot;, &quot;edgeLabelBackground&quot;: &quot;#111111&quot;}}}%%
graph TD
    lib[&quot;lib.rs · App&lt;br/&gt;winit events, Msg handlers&quot;] --&gt; state[&quot;state.rs · State&lt;br/&gt;camera · selection · frame demand&quot;]
    state --&gt; camera[&quot;camera.rs&quot;]
    state --&gt; scene[&quot;app/scene.rs · Scene&lt;br/&gt;Rc&amp;lt;Session&amp;gt; documents, placements, row maps&quot;]
    state --&gt; gpu[&quot;engine/gpu/mod.rs · Gpu&lt;br/&gt;device, layouts, targets, lanes&quot;]
    state --&gt; text_state[&quot;state/text.rs · labels&quot;]
    state --&gt; cq[&quot;state/cloud_query.rs · source pages&quot;]
    lib --&gt; loader[&quot;app/loader.rs · fetch, validate, stage&quot;]
    loader --&gt; manifest[&quot;app/manifest.rs · validate.rs · decode.rs · stream.rs&quot;]
    scene --&gt; walk[&quot;app/walk/* · producers&lt;br/&gt;mesh · brep · curves · cloud · points · text&quot;]
    walk --&gt; upload[&quot;engine/gpu/upload.rs · Upload&lt;br/&gt;typed rows, no wgpu types&quot;]
    upload --&gt; gpu
    gpu --&gt; lanes[&quot;lanes: arena · segments · glyphs · cloud+splat · text · surface_outline · triangle_tiles&quot;]
    gpu --&gt; pick[&quot;gpu/pick.rs · Picker&quot;]
    gpu --&gt; render[&quot;gpu/render.rs · frame list&quot;]
    lanes --&gt; shaders[&quot;shaders/*.wgsl&quot;]
    input[&quot;app/input.rs · touch.rs&quot;] --&gt; state</code></pre></div>
<p>The one upward flow is a pick answer: <code>Picker</code> returns a row and sub-ID, <code>Scene</code> maps them to a source identity, <code>State</code> selects.</p>
<p>Higher layers drive lower ones, never the reverse: a shader knows an object row, <code>Scene</code> knows which source object that row is, and input asks <code>State</code> to select rather than touching a buffer.</p>
<h2 id="owners">Owners<a class="anchor" href="#/course/architecture#owners" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Owner</th>
<th>Responsibility</th>
</tr>
</thead>
<tbody><tr>
<td><code>lib.rs::App</code></td>
<td>Window and event loop, <code>Msg</code> handlers, the one place a redraw is requested</td>
</tr>
<tr>
<td><code>state.rs::State</code></td>
<td>Camera and selection transitions, <code>needs_frame</code> / <code>dirty</code>, pick requests</td>
</tr>
<tr>
<td><code>state/text.rs</code></td>
<td>Selected-object names, source text presentation</td>
</tr>
<tr>
<td><code>state/cloud_query.rs</code></td>
<td>Streamed-cloud source queries across pages, original-ID resolution</td>
</tr>
<tr>
<td><code>state/sheet_query.rs</code>, <code>app/sheet_query.rs</code></td>
<td>Sheet entity metadata: two ranged reads of the side table on selection</td>
</tr>
<tr>
<td><code>app/walk/sheet.rs</code></td>
<td>A sheet slice into ribbon segments with per-segment source ids</td>
</tr>
<tr>
<td><code>app/scene.rs::Scene</code></td>
<td>Retained <code>Rc&lt;Session&gt;</code> documents, placements, row → source maps</td>
</tr>
<tr>
<td><code>app/scene_text.rs</code></td>
<td>Stable rows for authored text and document titles</td>
</tr>
<tr>
<td><code>app/selection.rs</code></td>
<td><code>SelectionMode</code>, <code>ControlId</code>, <code>Controls</code></td>
</tr>
<tr>
<td><code>app/modeling.rs</code>, <code>app/edit.rs</code></td>
<td>Validated source transactions and placed control edits</td>
</tr>
<tr>
<td><code>app/hierarchy.rs</code>, <code>state/panel.rs</code></td>
<td>Bounded tree/graph index and shared select/hide actions</td>
</tr>
<tr>
<td><code>app/ui/</code>, <code>gpu/ui.rs</code></td>
<td>egui input/widget state, then GPU buffers and font textures</td>
</tr>
<tr>
<td><code>app/gizmo.rs</code>, <code>gpu/widget.rs</code></td>
<td>Handle hit tests and one reusable unlit mesh with a temporary antialiasing tile</td>
</tr>
<tr>
<td><code>app/walk/</code></td>
<td>Source geometry → typed <code>Upload</code> rows with source identity and bounds</td>
</tr>
<tr>
<td><code>engine/gpu/mod.rs::Gpu</code></td>
<td>One device and queue, layouts, frame uniforms, targets, every lane</td>
</tr>
<tr>
<td><code>gpu/arena.rs</code></td>
<td>Mesh vertex, index and object columns; <code>faces.rs</code> source-face rows; <code>triangle_tiles.rs</code> finite-visibility cache</td>
</tr>
<tr>
<td><code>gpu/segments.rs</code></td>
<td>Joined boundary pipes and standalone ribbons with source-edge IDs</td>
</tr>
<tr>
<td><code>gpu/glyphs.rs</code>, <code>cloud.rs</code>, <code>splat.rs</code></td>
<td>Markers and control dots; cloud LOD nodes; point splat prelude and resolve</td>
</tr>
<tr>
<td><code>gpu/surface_outline.rs</code></td>
<td>Ordinary and selected coverage masks rasterized in one pass, block maxima of each, one black compositor that skips every pixel no covered texel can reach; the masks are reused while camera, geometry and selection stand still</td>
</tr>
<tr>
<td><code>gpu/text.rs</code>, <code>text_plate.rs</code>, <code>text_plane.rs</code>, <code>text_outline.rs</code></td>
<td>Shaped glyph runs, plates, fixed-plane text, imported outlines</td>
</tr>
<tr>
<td><code>gpu/pick.rs::Picker</code></td>
<td>ID targets, bounded readback windows, generations and cancellation</td>
</tr>
<tr>
<td><code>app/loader.rs</code>, <code>fetch.rs</code>, <code>live.rs</code>, <code>stream.rs</code></td>
<td>Fetching, validation, staged replacement, bounded ranged reads</td>
</tr>
</tbody></table>
<p><img src="/session/docs/course/docs/illustrations/ownership.svg" alt="Source documents become display data; input and picking share the same scene identity." loading="lazy" decoding="async"></p>
<h2 id="three-representations-of-an-object">Three representations of an object<a class="anchor" href="#/course/architecture#three-representations-of-an-object" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Representation</th>
<th>Example</th>
<th>Lifetime and precision</th>
</tr>
</thead>
<tbody><tr>
<td>Source</td>
<td>One BRep face, its trim uses, original edge IDs</td>
<td>Retained <code>Session</code> data, f64</td>
</tr>
<tr>
<td>Display</td>
<td>Triangles, ordered boundary node chains, markers</td>
<td>Rebuilt on geometry change; source maps kept</td>
</tr>
<tr>
<td>GPU</td>
<td>Packed vertices, <code>Instance</code> rows, draw ranges</td>
<td>Object-relative f32 plus rebased translations</td>
</tr>
</tbody></table>
<p>Selecting a source face (Ctrl+Shift) selects the face, not a tessellation triangle. Every segment of one BRep edge carries the original edge ID. F10 shows original vertices or control points, not display subdivisions.</p>
<h2 id="a-frame">A frame<a class="anchor" href="#/course/architecture#a-frame" aria-label="Link to this section">#</a></h2>
<p><code>Gpu::encode_frame</code> in <code>gpu/render.rs</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>%%{init: {&quot;flowchart&quot;: {&quot;useMaxWidth&quot;: false, &quot;wrappingWidth&quot;: 320}, &quot;themeVariables&quot;: {&quot;primaryColor&quot;: &quot;#ffffff&quot;, &quot;primaryTextColor&quot;: &quot;#111111&quot;, &quot;primaryBorderColor&quot;: &quot;#111111&quot;, &quot;lineColor&quot;: &quot;#ffffff&quot;, &quot;secondaryColor&quot;: &quot;#ffffff&quot;, &quot;tertiaryColor&quot;: &quot;#ffffff&quot;, &quot;edgeLabelBackground&quot;: &quot;#111111&quot;}}}%%
flowchart TD
    A[&quot;triangle_tile_pass&lt;br/&gt;project triangles, bin into screen tiles&lt;br/&gt;(only when camera or geometry changed)&quot;] --&gt; B[&quot;point_pass&lt;br/&gt;cloud splat prelude&quot;]
    B --&gt; C[&quot;begin_faces: backdrop, grid, opaque faces, cloud resolve&lt;br/&gt;writes Depth32Float + Rgba16Float gradient/primitive metadata&quot;]
    C --&gt; D[&quot;selection mask · solid mask&lt;br/&gt;R8Unorm coverage against physical depth&quot;]
    D --&gt; E[&quot;begin_ink: face highlight, print geometry, unselected strokes,&lt;br/&gt;selected solid strokes, black silhouette, selected standalone curves,&lt;br/&gt;markers, text&quot;]
    E --&gt; F{&quot;pick pending?&quot;}
    F -- yes --&gt; G[&quot;id_pass: same lists, Rg32Uint IDs, scissored window, copy out&quot;]
    F -- no --&gt; H[&quot;submit · present&quot;]
    G --&gt; H</code></pre></div>
<p>Order matters twice in the ink pass: selected solid strokes go below the silhouette so their yellow fringe cannot narrow its black border, and selected standalone curves go above it so a coincident mesh edge cannot erase them. Both obey physical occlusion.</p>
<p>Section caps, ambient occlusion and the outline masks are passes: each owns its GPU state in one file, impls <code>Pass</code> (<code>gpu/pass.rs</code>: hooks before, in and after the face pass, the masks, the ink and the id pass) and is named once in <code>pass::PASSES</code>, which <code>render.rs</code> walks in order. Shared WGSL is included once: <code>pipelines::PRELUDE</code> ends every scene shader, and <code>build.rs</code> expands <code>#include &quot;file.wgsl&quot;</code> lines in the compute and point shaders.</p>
<p>The gumball tile is rendered and composited after scene ink; egui draws the final interface. Both overlays use their own layouts and avoid writing scene depth. Their sample counts do not depend on scene MSAA.</p>
<p><img src="/session/docs/course/docs/illustrations/frame.svg" alt="Physical surfaces, readable source ink, one black silhouette, then foreground annotations." loading="lazy" decoding="async"></p>
<h2 id="memory">Memory<a class="anchor" href="#/course/architecture#memory" aria-label="Link to this section">#</a></h2>
<p>Per-pixel attachments dominate: at 4x MSAA the colour, depth and <code>Rgba16Float</code> metadata targets cost 64 bytes per physical pixel, at 1x 12. <code>Targets::samples_for</code> chooses 4x only for solid geometry, inside the adapter&#39;s pixel budget (9 Mpx discrete, 2.5 Mpx integrated, 4.2 Mpx unknown), and below two physical pixels per CSS pixel, where the pixel density already halves the stair-steps; <code>?msaa=4</code> and <code>?msaa=1</code> force it. <code>?dpr=1.5</code> caps the device pixel ratio the canvas is rendered at, for people who prefer memory over crispness; nothing caps it by default. Every size-bound texture is an <code>Attachment</code>, whose drop calls <code>destroy()</code>: wgpu&#39;s WebGPU backend frees nothing on drop, so the old attachments of every resize would otherwise wait for the JavaScript garbage collector - a window drag (one resize a frame, 64 bytes per pixel each at 4x) lost the device on a scene of twelve objects. <code>replace_buffer</code> does the same for a table that grew or a pool that was resized, and <code>State::resize</code> applies at most one resize per 100 ms while a drag lasts. Thirty slow drag frames in a row, or the page a device loss reloaded into, set <code>view::reduce</code>: device scale 1, no antialiasing, in memory only. On a lost device the page reloads itself once with the browser&#39;s reason in <code>?recovered=</code>; <code>adopt_recovery</code> on the reloaded page reduces, takes the parameter back out of the address, and the status line names the reason; a second loss shows the error panel. <code>?inspect=1</code> publishes <code>gpu_texture_estimate_bytes</code> and <code>gpu_buffer_capacity_bytes</code>; these estimates exclude browser overhead and renderer-private allocations.</p>
<h2 id="depth-and-visible-ink">Depth and visible ink<a class="anchor" href="#/course/architecture#depth-and-visible-ink" aria-label="Link to this section">#</a></h2>
<ul>
<li>Depth is reversed: near is larger, the clear value is zero, opaque faces compare <code>Greater</code>.</li>
<li>A stroke covers samples beside its axis. The ink shader transfers the winning primitive&#39;s depth to the axis through the stored gradient before comparing, so a line on a surface is not hidden by the surface beside it.</li>
<li>A neighbouring triangle&#39;s plane can cross the axis outside the triangle. The finite fallback in <code>triangle_tiles.rs</code> then tests the actual triangles that intersect the axis&#39;s screen tile: projected records (six <code>vec4&lt;f32&gt;</code> each), per-tile counts, a prefix scan, and filled <code>(primitive, max depth)</code> lists. Overflowing or incomplete lists keep the conservative rejection. The list pool is sized for the scene, two references per tile plus eight per triangle, and the scan writes the words it needed into the first record; the CPU reads that back a frame later and grows the pool before the next projection, so a small scene never pays the 64 MB ceiling of a 262 144-tile grid.</li>
</ul>
<p><img src="/session/docs/course/docs/illustrations/finite-triangle.svg" alt="A triangle's plane extends beyond its footprint; the line is visible outside the actual triangle." loading="lazy" decoding="async"></p>
<h2 id="rust-wgsl-interfaces">Rust ↔ WGSL interfaces<a class="anchor" href="#/course/architecture#rust-wgsl-interfaces" aria-label="Link to this section">#</a></h2>
<p>Bind group scheme for every draw (<code>engine/pipelines/layouts.rs</code>):</p>
<table>
<thead>
<tr>
<th>Group</th>
<th>Binding</th>
<th>Rust layout</th>
<th>WGSL</th>
</tr>
</thead>
<tbody><tr>
<td>0</td>
<td>0</td>
<td><code>Layouts::mvp</code>, uniform</td>
<td><code>@group(0) @binding(0) var&lt;uniform&gt; mvp: mat4x4&lt;f32&gt;</code></td>
</tr>
<tr>
<td>1</td>
<td>0</td>
<td><code>Layouts::line</code>, uniform</td>
<td><code>@group(1) @binding(0) var&lt;uniform&gt; line: LineUniform</code></td>
</tr>
<tr>
<td>2</td>
<td>0</td>
<td><code>Layouts::instance</code>, storage</td>
<td><code>@group(2) @binding(0) var&lt;storage, read&gt; instances: array&lt;Instance&gt;</code></td>
</tr>
<tr>
<td>2</td>
<td>1</td>
<td>anchored translations, storage</td>
<td><code>@group(2) @binding(1) var&lt;storage, read&gt; translations: array&lt;vec4&lt;f32&gt;&gt;</code></td>
</tr>
<tr>
<td>2</td>
<td>2–5</td>
<td><code>Layouts::ink_instance</code>: physical depth and gradient, single and multisampled</td>
<td>depth and float textures sampled by the ink fragment stage</td>
</tr>
<tr>
<td>2</td>
<td>6–7</td>
<td>projected triangles and tile lists</td>
<td>finite-visibility storage read by <code>ink_visibility.wgsl</code></td>
</tr>
<tr>
<td>3</td>
<td>n</td>
<td>the lane&#39;s own rows (<code>ink_rows</code>, <code>segment_rows</code>, <code>points</code>)</td>
<td>e.g. <code>@group(3) @binding(0) var&lt;storage, read&gt; segments: array&lt;StrokeSegment&gt;</code></td>
</tr>
</tbody></table>
<p><code>Instance</code> (<code>gpu/instance.rs</code>) is one 96-byte row: <code>model: [f32;16]</code>, <code>color: [f32;4]</code>, <code>flags: u32</code>, <code>thickness</code>, <code>spacing</code>, <code>_pad</code>. The translation column is zero; the rebased translation is the 16-byte row at group 2 binding 1, so a re-anchor rewrites 16 bytes per object. Flag bits: selected, hidden, inside, print, open, sheet, smooth. Layout tests in <code>instance.rs</code> compare the Rust size and offsets with the WGSL declaration.</p>
<p>Meshes are vertex-pulled: <code>triangle.wgsl</code> reads <code>face_vertices</code>, <code>face_objects</code>, <code>face_indices</code> from storage at group 3 instead of a vertex buffer. Textures: color surface format, <code>Depth32Float</code> physical depth, <code>Rgba16Float</code> metadata (gradient in xy, packed primitive ID in zw), <code>R8Unorm</code> coverage masks, <code>Rg32Uint</code> pick IDs with their own <code>Depth32Float</code> and metadata.</p>
<h2 id="flows">Flows<a class="anchor" href="#/course/architecture#flows" aria-label="Link to this section">#</a></h2>
<p><strong>Camera.</strong> Pointer delta → <code>Input</code> → <code>Camera::orbit / pan / zoom_at</code> (f64, target + distance + quaternion) → <code>view_proj_anchored(aspect, anchor)</code> with reversed depth and a near plane at a fraction of the focus distance → <code>FrameUniforms</code> (<code>mvp</code>, eye, ortho half-height) → group 0.</p>
<p><strong>Geometry.</strong> <code>Msg::File</code> → <code>Scene</code> retains the document → <code>app/walk/*</code> produce <code>Upload</code> rows (arena, segments, glyphs, cloud, object rows, bounds) with source maps → <code>Gpu::set_scene</code> appends rows to lane buffers and rebinds → <code>render.rs</code> draws ranges.</p>
<p><strong>Picking.</strong> Pointer up without a drag → <code>State::request_selection</code> records mode, generation, camera → <code>id_pass</code> renders IDs into an attachment the size of the window around the cursor plus a three-texel halo, not the canvas: the pick pass sees the scene through the sub-frustum of that window (<code>PickView::clip_transform</code>), with the projection factors scaled so pens and markers keep their pixel size, and the visibility test addresses the canvas-wide tiles through <code>LineUniform::origin</code> → <code>Picker</code> maps a bounded copy asynchronously → row and sub-ID → <code>Scene::object_at / edge_at / face_at</code> → <code>SelectionMode</code> → <code>Instance::FLAG_SELECTED</code> uploaded → redraw. A camera, scene or mode change retires answers from an older generation. The ID, depth and metadata targets therefore cost a few kilobytes instead of 20 bytes per canvas pixel.</p>
<p><strong>Document history.</strong> The kernel <code>Session</code> keeps a <code>History</code> of transactions: a removal&#39;s record is the tombstone undo restores from, <code>replace</code> is the recorded edit, and every save purges the buffer. The viewer commits modeling operations through <code>Session</code> transactions and rebuilds display rows through <code>Scene::rebuild</code>; pointer movement previews GPU rows and commits once on release.</p>
<p><strong>Sheets.</strong> A drawing publishes as one <code>Sheet</code> message (<code>Objects.sheets</code>, field 17): packed fixed-width <code>coords</code>, <code>colors</code>, <code>widths</code> and <code>source_ids</code>, so <code>stream.rs</code> locates the arrays from the first kilobytes and the loader streams segments by byte range under a segment budget. The whole sheet is one object row and one ribbon draw; every segment carries its entity id through <code>SegRows.ribbon_ids</code>. A pick resolves row plus segment to the entity, and <code>sheet_query.rs</code> reads its GUID, name and kind from the <code>.meta</code> side table in two ranged reads, cached per sheet with the table&#39;s ETag. The kernel never decodes a sheet.</p>
<p><strong>Controls.</strong> F10 → <code>Controls</code> collects original vertices or control points of the selected parent → marker rows uploaded with <code>ControlId</code> → a control pick returns the original identity, not the marker slot. Streamed clouds query every eligible source page (<code>state/cloud_query.rs</code>) independently of display LOD and apply the final visible original ID.</p>
<p><strong>Text.</strong> String, font, size → <code>engine/text.rs</code> shapes glyph runs with the bundled Noto fonts → <code>TextPlacement</code> (Screen, Anchor, Nameplate, WorldPlane, WorldBillboard) → <code>TextLane::prepare</code> chooses the physical raster size once per DPR → coverage atlas → plate pass and glyph pass; fixed-plane text uses <code>text_plane.wgsl</code> with perspective-correct rounded plates that also serve the ID pass.</p>
<p><strong>Loading.</strong> Route → manifest (TOML, YAML or JSON) → <code>validate.rs</code> checks counts, indices, transforms → protobuf decode → <code>PendingDocument</code> staged with a request generation → <code>Msg::File</code> in manifest order. Stale generations are dropped; a malformed document keeps the last valid scene.</p>
<h2 id="lifecycles">Lifecycles<a class="anchor" href="#/course/architecture#lifecycles" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Change</th>
<th>Required work</th>
<th>Reused work</th>
</tr>
</thead>
<tbody><tr>
<td>Camera, DPR or resize</td>
<td>Uniforms, placement, finite-visibility projection, targets; cancel stale picks</td>
<td>Documents, tessellation, shaped text, the visibility pool&#39;s grown size</td>
</tr>
<tr>
<td>Selection or color</td>
<td>Object flags, selected labels, masks, redraw</td>
<td>Mesh buffers, projected-triangle cache</td>
</tr>
<tr>
<td>Hide or show</td>
<td>Visibility flags, projection invalidation, text visibility</td>
<td>Source geometry and identity</td>
</tr>
<tr>
<td>Document replacement</td>
<td>Source maps, display data, bounds, caches; cancel stale work</td>
<td>Device and pipelines</td>
</tr>
<tr>
<td>Text content or font</td>
<td>Reshape changed runs, raster, bounds</td>
<td>Unchanged glyph resources</td>
</tr>
<tr>
<td>Idle</td>
<td>Nothing submitted</td>
<td>Everything</td>
</tr>
</tbody></table>
<p><code>reset</code> keeps capacity for an edit rebuild; <code>release</code> returns scene-sized storage on replacement. <code>SourceCache</code> (<code>app/inspection/source_memory.rs</code>) holds <code>Weak&lt;Session&gt;</code> identities, so measuring retained source payload never extends a document&#39;s lifetime.</p>
<table>
<thead>
<tr>
<th>Retained data</th>
<th>Invalidate or release</th>
</tr>
</thead>
<tbody><tr>
<td>Hierarchy row index</td>
<td><code>row_revision</code> changes or scene clears; at most 200,000 nodes / 1,000,000 row references</td>
</tr>
<tr>
<td>Gumball mesh and uniform</td>
<td>Fixed 371,616 bytes until renderer destruction</td>
</tr>
<tr>
<td>Gumball tile</td>
<td>Replaced on tile-size change; destroyed on deselect; at most 36 MiB</td>
</tr>
<tr>
<td>egui command state</td>
<td>2,048 input characters, eight history entries; no source geometry</td>
</tr>
<tr>
<td>egui fonts and draw buffers</td>
<td>Process texture frees between frames; remaining textures free with renderer; private capacity is outside inspection counters</td>
</tr>
<tr>
<td>Source history</td>
<td>Kernel transactions retain undo data; no whole-document memory cap is claimed</td>
</tr>
</tbody></table>
<p>Loader routing state, live polling and the small UI model have one-page lifetimes. The application does not offer repeated mount/unmount in one page. Releasing allocations also does not shrink WebAssembly linear memory back to the operating system. Resource counters measure named allocations, not total browser memory.</p>
<h2 id="input">Input<a class="anchor" href="#/course/architecture#input" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Input</th>
<th>Result</th>
</tr>
</thead>
<tbody><tr>
<td>Left drag on handle / right drag / middle drag / wheel</td>
<td>Edit / orbit / pan / zoom toward the cursor</td>
</tr>
<tr>
<td>Left click</td>
<td>Select or toggle one source object</td>
</tr>
<tr>
<td>Left drag on an object</td>
<td>Move it or its selection on the ground, snapped when Snap is on; one undo step; Escape or a release over a panel puts it back</td>
</tr>
<tr>
<td>Left click on a gumball handle</td>
<td>A number box beside it: Move mm, Rotate deg or Scale factor; Enter applies one undo step, Escape closes</td>
</tr>
<tr>
<td>Ctrl + left click</td>
<td>Select an original mesh, BRep or NURBS edge</td>
</tr>
<tr>
<td>Ctrl + Shift + left click</td>
<td>Select an original face; a nearby eligible edge wins</td>
</tr>
<tr>
<td>F10 / Escape</td>
<td>Show the parent&#39;s original controls / leave the mode, then clear</td>
</tr>
<tr>
<td>1–7, C, F, Space</td>
<td>Standard views, reset, fit, projection toggle</td>
</tr>
<tr>
<td>Q, W, E, O, P, D, B</td>
<td>Points, lines, mesh edges, silhouettes, x-ray, lighting, back faces</td>
</tr>
<tr>
<td>H / S / T</td>
<td>Hide selection / show all / toggle selected names</td>
</tr>
</tbody></table>
<h2 id="editing-overlays">Editing overlays<a class="anchor" href="#/course/architecture#editing-overlays" aria-label="Link to this section">#</a></h2>
<p><code>app/ui/mod.rs</code> owns the egui context and translates winit input into panel actions and commands; each panel (number box, command line, layers) is one file under <code>app/ui/</code> that keeps its own state and implements <code>Panel</code> (fill from <code>State</code>, show, apply, keys, phone field, taps, test snapshot), named once in the <code>panels!</code> list; clicks and lines to run come back through one <code>Output</code>. <code>engine/gpu/ui.rs</code> owns its renderer and font textures and draws after the scene. Text input takes keyboard focus while the command window is open; scene shortcuts resume after closing it. The white/black visuals follow the archive customization. The old DOM command and layer listeners are removed.</p>
<p>The gumball owns one fixed mesh and 96-byte uniform, plus a selected-only antialiasing tile capped at 1024×1024. Its shader uses unlit colors; the tile uses 4× MSAA and 2× resolution before compositing. Deselect destroys the tile. Neither overlay owns document geometry. UI hit-box snapshots are opt-in with <code>?inspect=1</code>.</p>
<h2 id="adding-a-feature">Adding a feature<a class="anchor" href="#/course/architecture#adding-a-feature" aria-label="Link to this section">#</a></h2>
<p>For a new geometry family: a <code>walk/</code> producer that emits existing <code>Upload</code> rows with bounds and source identity; a new lane only when storage or drawing differs; one line in <code>render.rs</code>; then exercise select, hide, replace and release. For a new left-button tool: one file in <code>app/gesture/</code>, its <code>mod</code> line and one <code>// register:</code> line in <code>GESTURES</code>. For new per-feature state: one field line in <code>state/features.rs</code> (<code>Features</code>, reached as <code>state.features</code>), its per-frame work one line in <code>BEFORE_PICKS</code> or <code>AFTER_PICKS</code>. For a new panel: one file in <code>app/ui/</code> and one line in <code>panels!</code>. For a new command: one file in <code>app/command/verbs/</code> and one line in <code>verbs!</code>, which declares the module and adds its <code>SPEC</code> to <code>REGISTRY</code>. For a new optional pass: one <code>Pass</code> impl in <code>gpu/</code> and one <code>// register:</code> line in <code>pass::PASSES</code>; shared WGSL goes in one file, named in <code>PRELUDE</code> or <code>#include</code>d. For a shader change: read its Rust mirror, bindings, color and ID entry points, sample count and release path together, and check the layout test in <code>instance.rs</code>. Never mutate a vertex buffer behind <code>Scene</code>: it is the source of truth for picking, controls and undo.</p>
<p>The CAD geometry contract (shared boundaries, trims, pcurves, provenance) is in the <a href="#/course/cad-design">CAD design record</a>.</p>
`,toc:[{level:2,id:"module-graph",text:"Module graph"},{level:2,id:"owners",text:"Owners"},{level:2,id:"three-representations-of-an-object",text:"Three representations of an object"},{level:2,id:"a-frame",text:"A frame"},{level:2,id:"memory",text:"Memory"},{level:2,id:"depth-and-visible-ink",text:"Depth and visible ink"},{level:2,id:"rust-wgsl-interfaces",text:"Rust ↔ WGSL interfaces"},{level:2,id:"flows",text:"Flows"},{level:2,id:"lifecycles",text:"Lifecycles"},{level:2,id:"input",text:"Input"},{level:2,id:"editing-overlays",text:"Editing overlays"},{level:2,id:"adding-a-feature",text:"Adding a feature"}]};export{e as default};
