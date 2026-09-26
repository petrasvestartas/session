const e={title:"Adding panels: tree, types and graph",html:`<h1 id="adding-panels-tree-types-and-graph">Adding panels: tree, types and graph<a class="anchor" href="#/course/extend-panels#adding-panels-tree-types-and-graph" aria-label="Link to this section">#</a></h1>
<p>Current implementation and independent teaching steps: <a href="#/course/22-runtime-helpers">Editing extensions</a>. The designs below are historical; use the supplement’s status table for supported operations and limits.</p>
<div class="note"><p class="note-title">Built in lesson 21, in part</p><p>The types panel is <code>src/app/layers.rs</code> and <code>L</code> opens it; <a href="#/course/21-editing">lesson 21</a> teaches
it. That frozen checkpoint has flat layers. The maintained viewer adds nested and graph
selection/visibility through <a href="#/course/26-nested-panel">the panel extension</a>, with the
<a href="#/course/27-egui-interface">egui interface</a>.</p>
</div>
<h2 id="one-visibility-set-three-filters">One visibility set, three filters<a class="anchor" href="#/course/extend-panels#one-visibility-set-three-filters" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>Scene.hidden: HashSet&lt;(usize, Rc&lt;str&gt;)&gt;</code> (<code>src/app/scene.rs</code>) is the only record of what is not drawn; written today only by <code>State::hide_selected</code> and <code>show_all</code> (<code>src/state.rs</code>).</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch, not code to type. src/state.rs</span>
<span class="k">struct</span> Filters {
 tree: Vec&lt;(usize, Vec&lt;u16&gt;)&gt;, <span class="c">// hidden subtrees: document index, child-index path</span>
 kinds: Vec&lt;Kind&gt;, <span class="c">// hidden type buckets</span>
 vertex: Vec&lt;String&gt;, <span class="c">// hidden graph vertex attributes</span>
 edge: Vec&lt;String&gt;, <span class="c">// hidden graph edge attributes</span>
 manual: HashSet&lt;(usize, Rc&lt;str&gt;)&gt;, <span class="c">// what H hid, one object at a time</span>
}</code></pre></div>
<ul>
<li>Axes, not a hidden set per panel; <code>Scene.hidden</code> is their union — else clearing the tree filter reveals rows the type filter still hides.</li>
<li><code>H</code> is a fourth axis, so <code>show_all</code> is one loop.</li>
<li>Axes store the FILTER, never rows: <code>Scene::rebuild</code> renumbers rows, paths and attribute strings survive.</li>
<li>So un-hiding is per axis, not per row. &quot;Hide all but this&quot; is <code>isolate</code>, the complement into <code>manual</code>.</li>
</ul>
<h2 id="what-each-filter-keys-on">What each filter keys on<a class="anchor" href="#/course/extend-panels#what-each-filter-keys-on" aria-label="Link to this section">#</a></h2>
<h3 id="by-element-type">By element type<a class="anchor" href="#/course/extend-panels#by-element-type" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch. src/app/filter.rs</span>
<span class="k">enum</span> Kind {
 Point, Line, Plane, OBB, Polyline, PointCloud, Mesh,
 NurbsCurve, NurbsSurface, BRep, Element, <span class="c">// the kernel's own order</span>
 StreamedCloud, Sheet, Text, <span class="c">// row kinds only this viewer has</span>
}
<span class="k">struct</span> TypeRanges { base: u32, ends: [u32; <span class="s">11</span>] }</code></pre></div>
<ul>
<li>The eleven are <code>Session::order</code>&#39;s sequence (<code>session_rust/src/session.rs</code>); <code>add_file</code> pushes rows in it, so each (document, kind) is a CONTIGUOUS run — eleven <code>u32</code>, 44 bytes per document, no per-row storage.</li>
<li>Two skips shorten a bucket, never reorder one: a guid missing from <code>session.lookup</code>, and <code>is_drawable</code> (<code>src/app/walk/mod.rs</code>) rejecting an <code>Element</code> with <code>None</code> geometry. Elements are last, so nothing after them moves.</li>
<li>Hiding a type is a RANGE write, not a scan.</li>
</ul>
<h3 id="by-tree-node">By tree node<a class="anchor" href="#/course/extend-panels#by-tree-node" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch. src/app/filter.rs</span>
<span class="k">struct</span> TreeIndex {
 nodes: Vec&lt;Node&gt;, <span class="c">// depth-first: a parent always precedes its children</span>
 row_node: HashMap&lt;u32, u32&gt;, <span class="c">// object row -&gt; index of the leaf node that names it</span>
}
<span class="k">struct</span> Node { parent: Option&lt;u32&gt;, child_index: u16, depth: u16, label: Rc&lt;str&gt;, rows: Vec&lt;u32&gt; }</code></pre></div>
<ul>
<li>Leaf test: <code>Scene::guid_to_row</code> contains <code>(document, node.name)</code> — <code>TreeNode.name</code> IS the geometry guid (<code>session_rust/src/tree.rs</code>) and <code>push_row</code> fills <code>guid_to_row</code>, so it cannot disagree with the rows.</li>
<li><code>Node.rows</code> is the group&#39;s whole subtree, resolved once: rows are what <code>set_hidden</code> takes, guids what survives a rebuild.</li>
<li><code>row_node</code> + <code>Node.parent</code> opens ancestors in O(depth); <code>src/app/validate.rs</code> caps hierarchies at 64 levels.</li>
<li>Key the filter and the open set on <code>(document, path)</code> — child indices from the root, <code>(0, [2, 1])</code>. Reason below.</li>
<li>Walk once, in <code>add_file</code>: <code>TreeNode::children</code> clones its <code>Rc</code> vector on every call.</li>
</ul>
<h3 id="by-graph-element">By graph element<a class="anchor" href="#/course/extend-panels#by-graph-element" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch. src/app/filter.rs</span>
<span class="k">struct</span> GraphIndex {
 vertex: HashMap&lt;Rc&lt;str&gt;, Vec&lt;u32&gt;&gt;, <span class="c">// vertex attribute -&gt; rows</span>
 edge: HashMap&lt;Rc&lt;str&gt;, Vec&lt;u32&gt;&gt;, <span class="c">// edge attribute -&gt; the rows of BOTH endpoints</span>
}
<span class="k">enum</span> Filter { Tree(usize, Vec&lt;u16&gt;), Kind(Kind), VertexAttribute(String), EdgeAttribute(String) }</code></pre></div>
<ul>
<li><code>Graph::add_node(key, attribute)</code> takes the geometry guid as key (<code>session_rust/src/graph.rs:290</code>): a <code>Vertex</code>&#39;s name is a guid, <code>attribute</code> the label to group on. <code>src/app/decode.rs</code> matches, so wasm and native agree. <code>src/app/decode.rs</code> matches, so wasm and native agree.</li>
<li>Edges have no rows, so an edge filter hides both endpoints — and the row says &quot;joint — 8 objects&quot;, not &quot;4 edges&quot;.</li>
<li>Read once per load: <code>get_vertices</code> clones every <code>Vertex</code>, <code>get_edges</code> de-duplicates into a new <code>Vec&lt;(String, String)&gt;</code> — ~114k allocations on the sheet fixture.</li>
<li><code>node_attribute</code> and <code>edge_attribute</code> need <code>&amp;mut self</code>, unreachable behind <code>Rc&lt;Session&gt;</code>; the index is the only read path.</li>
<li>Free later, all <code>&amp;self</code>: <code>neighbors</code>, <code>bfs</code>, <code>connected_components</code>, <code>shortest_path</code>, <code>cycle_basis</code>.</li>
</ul>
<h2 id="one-visibility-decision-and-the-path-it-takes-to-the-gpu">One visibility decision, and the path it takes to the GPU<a class="anchor" href="#/course/extend-panels#one-visibility-decision-and-the-path-it-takes-to-the-gpu" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch. src/state.rs</span>
<span class="k">pub</span> <span class="k">fn</span> set_filter(&amp;<span class="k">mut</span> <span class="k">self</span>, f: Filter, on: bool) <span class="c">// write one axis, then apply</span>
<span class="k">pub</span> <span class="k">fn</span> isolate(&amp;<span class="k">mut</span> <span class="k">self</span>, f: &amp;Filter) <span class="c">// complement into \`manual\`, then apply</span>
<span class="k">pub</span> <span class="k">fn</span> hide_rows(&amp;<span class="k">mut</span> <span class="k">self</span>, rows: &amp;[u32], on: bool) <span class="c">// what hide_selected becomes</span>
<span class="k">fn</span> apply_filters(&amp;<span class="k">mut</span> <span class="k">self</span>) <span class="c">// union -&gt; diff -&gt; runs -&gt; GPU</span></code></pre></div>
<ul>
<li><code>apply_filters</code>: union the indexes plus <code>manual</code>, diff against <code>Scene.hidden</code>, coalesce the FLIPPED rows into runs, write, replace.</li>
<li>Coalesce changed rows, not targeted rows — <code>set_flag</code> early-returns on a matching bit, so a naive run write makes a redundant toggle cost megabytes.</li>
<li><code>Scene.hidden</code> stays <code>(document, guid)</code>: <code>add_file</code> re-applies that form while pushing rows, and guids collide between documents (<code>Scene::identity_of</code>).</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Sketch. src/engine/gpu/objects.rs and src/engine/gpu/mod.rs</span>
<span class="k">impl</span> InstanceTable { <span class="k">pub</span> <span class="k">fn</span> set_flags_run(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, rows: Range&lt;u32&gt;, bit: u32, on: bool) }
<span class="k">impl</span> Gpu { <span class="k">pub</span> <span class="k">fn</span> set_hidden_rows(&amp;<span class="k">mut</span> <span class="k">self</span>, runs: &amp;[Range&lt;u32&gt;], on: bool) }</code></pre></div>
<ul>
<li><code>set_flags_run</code>: flip the CPU mirror slice, ONE <code>GrowBuf::write_at</code> (it already takes a slice and multiplies by the stride).</li>
<li><code>set_hidden_rows</code>: that per run, then <code>splat.invalidate</code> ONCE per batch — the rebuild it forces walks every cloud.</li>
<li>Both beside the single-row <code>set_flag</code>/<code>set_hidden</code>; <code>InstanceTable</code> owns the rows, so the stride stays there.</li>
</ul>
<h2 id="what-a-hide-costs">What a hide costs<a class="anchor" href="#/course/extend-panels#what-a-hide-costs" aria-label="Link to this section">#</a></h2>
<ul>
<li>One row: 96 bytes (<code>Instance</code> is the 96 B storage stride, asserted in <code>src/engine/gpu/instance.rs</code>), a <code>geometry_revision</code> bump, one cloud-record invalidation.</li>
<li>One bucket on <code>assets/pb/view_local_sheet_querschnitt.pb</code>: 56,889 line rows, one run, 96 × 56,889 ≈ 5.5 MB in a single <code>queue.write_buffer</code> against 56,889 writes — why <code>set_flags_run</code> exists.</li>
<li>Untouched: the model matrix, the 16-byte anchored translation row (<code>src/engine/gpu/objects.rs</code>). No re-anchor, no rebuild, no camera work.</li>
<li>Free elsewhere: every vertex stage parks hidden rows outside the clip volume (<code>shaders/triangle.wgsl</code>, <code>ribbon.wgsl</code>, <code>glyph.wgsl</code>, <code>sphere.wgsl</code>) and the id pass reuses those stages, so drawing and picking stop together. A &quot;do not pick the hidden&quot; list would be a second truth.</li>
<li>One CPU consumer: <code>src/engine/gpu/splat.rs</code> drops hidden rows while rebuilding cloud records — what <code>invalidate</code> is for.</li>
<li>A REPARENT is a full <code>Scene::rebuild</code>, not a hide: <code>session.world_xforms</code> composes placements at walk time in <code>add_file</code>. Say so in the UI.</li>
</ul>
<h2 id="where-the-indexes-are-built">Where the indexes are built<a class="anchor" href="#/course/extend-panels#where-the-indexes-are-built" aria-label="Link to this section">#</a></h2>
<ul>
<li>All three in <code>Scene::add_file</code> (<code>src/app/scene.rs</code>): eleven counters in the existing <code>push_row</code> loop, one tree DFS, one <code>get_vertices</code> pass.</li>
<li>Not <code>src/app/decode.rs</code> — wasm-only, and the native harness (<code>examples/check_determinism.rs</code>, <code>src/selftest/lifecycle.rs</code>) hand-builds a <code>FileDoc</code>, so no native test would see the index.</li>
<li>Not per frame — <code>order</code>, <code>owners</code> and <code>guid_to_row</code> are private to <code>scene.rs</code>, and <code>add_file</code> already knows each guid&#39;s row.</li>
<li>Clear in <code>reset_rows</code>, so <code>Scene::clear</code> and <code>rebuild</code> need no new field names.</li>
<li><code>add_streamed_cloud</code>/<code>add_sheet</code>: one <code>Kind</code> entry each, and empty — not absent — tree and graph entries, because those vectors are indexed alongside <code>Scene.docs</code>.</li>
</ul>
<h2 id="the-panels-rows">The panel&#39;s rows<a class="anchor" href="#/course/extend-panels#the-panels-rows" aria-label="Link to this section">#</a></h2>
<ul>
<li>Five row kinds, none an object row: document header, tree group, tree leaf, type bucket, attribute bucket.</li>
<li><code>src/app/panel.rs</code> (new, beside <code>scene.rs</code>) owns the flat list, open set, scroll and hit test; it names no wgpu type, so it unit-tests with no device — <code>src/app/mod.rs</code>&#39;s rule for that directory.</li>
<li>Flat list rebuilt on open-set, search or scene change, never per frame: an open group costs its whole visible subtree, paid on the click that opened it.</li>
<li>Window: <code>first = floor(scroll / ROW_H)</code>, <code>count = ceil(panel_height / ROW_H) + 1</code> — the <code>+ 1</code> is the partly visible bottom row, submitted so the clip cuts it, or rows pop at the boundary.</li>
<li><code>ROW_H = 18.0</code> CSS px: <code>validate_label</code> (<code>src/engine/text.rs</code>) needs <code>line_height &gt;= font_size</code>; 13 px text leaves 5 px of leading and squares the 18 × 18 icon cells.</li>
<li><code>INDENT_W = 14.0</code>, <code>ARROW_W = 12.0</code>: group <code>depth * INDENT_W</code>, leaf <code>+ ARROW_W</code>, so leaf text starts under parent text — the group&#39;s triangle costs <code>ARROW_W</code> and the leaf has none to spend.</li>
<li><code>PANEL_W = 280.0</code> CSS px: the clip rectangle cuts long names there instead of spilling them across the scene.</li>
<li>The panel OVERLAYS the canvas (<code>State::logical_size</code> reads the canvas element); shrinking the viewport would retarget every pass for a UI strip. Cost: that 280 px strip is unpickable while open.</li>
<li>Bucket rows make 56,889 objects readable: one per non-empty kind per document, with a count. Querschnitt&#39;s graph panel collapses to ONE row — <code>line_my_line</code>, 56,889 objects — which is why the type panel exists beside it.</li>
</ul>
<h2 id="drawing-the-panel">Drawing the panel<a class="anchor" href="#/course/extend-panels#drawing-the-panel" aria-label="Link to this section">#</a></h2>
<ul>
<li>Row TEXT via the text lane: one <code>TextLabel</code> per visible row, <code>TextPlacement::Screen { left, top }</code> in CSS px, <code>clip: Some([l, t, r, b])</code>, <code>object: None</code>.</li>
<li><code>object: None</code> because a plate writes <code>object.row + 1</code> into the ID target (<code>text_plate.rs</code>) and would answer scene picks with an unrelated row, and <code>text_rectangle</code> (<code>text.rs</code>) would give it a rounded pill, not a row band.</li>
<li>One <code>gpu.text.set_labels(...)</code> for panel and scene together: it REPLACES the document, so a second call erases the nameplate and every source text. Extend <code>State::update_label</code> (<code>src/state/text.rs</code>).</li>
<li>Ids unique per submission: objects use <code>row + 1</code>, the nameplate <code>0</code>, panel rows <code>PANEL_LABEL_ID = 1 &lt;&lt; 31</code> plus the flat-list index — object rows never reach 2^31.</li>
<li>The id is also the shaping-reuse key (<code>same_layout</code>: text, font size, line height), so scrolling moves <code>Screen { left, top }</code> and reshapes NOTHING — pinned by <code>placement_and_color_do_not_reshape_but_font_reload_does</code> (<code>src/engine/text.rs</code>). Key on row identity, never a screen slot.</li>
<li><code>include_text_bounds</code> skips <code>Screen</code>: panel rows never grow the scene bounds, and <code>F</code> still frames the model.</li>
<li>256 KiB of text per submission (<code>MAX_TEXT_BYTES</code>) — another reason to submit only the window.</li>
<li>Row CHROME is its own lane: <code>src/engine/gpu/panel.rs</code>, a field on <code>Gpu</code> beside <code>text</code>, drawn in <code>Renderer::scene_list</code> just before <code>self.text.draw(pass)</code> so bands sit under their glyphs.</li>
<li>One <code>GrowBuf</code> of CSS-px quads converted to NDC on the CPU, one pipeline, alpha blend, <code>depth_write_enabled: false</code>, <code>depth_compare: GreaterEqual</code> at 1.0 — copy <code>src/engine/gpu/text_plate.rs</code>, already that shape for that reason.</li>
<li>No id pipeline, not in <code>Renderer::id_pass</code>: a lane that draws no ids cannot be mis-picked.</li>
<li>Not <code>Plates</code> widened: it is <code>pub(super)</code>, its vertex carries an object row, and <code>text_rectangle</code> only emits nameplate and object-bearing rectangles.</li>
</ul>
<h2 id="hit-testing-on-the-cpu-deliberately">Hit testing: on the CPU, deliberately<a class="anchor" href="#/course/extend-panels#hit-testing-on-the-cpu-deliberately" aria-label="Link to this section">#</a></h2>
<ul>
<li>The GPU could do it with no new <code>PickMode</code>: <code>TextLane::draw_ids</code> runs outside the <code>match mode</code> in <code>Renderer::id_pass</code>, so a screen label with an <code>object</code> is pickable today.</li>
<li>Don&#39;t: the id path answers occlusion, and costs a pass, a <code>copy_window</code> readback, a generation check and a frame of lag (<code>Picker::poll</code>, applied atop <code>State::render</code>) — hover cannot survive that. A row is an axis-aligned CSS-px box computed one call earlier: four comparisons, no lag, no stale-generation drop.</li>
<li>The CPU test must add precedence: run it in <code>Input::left</code> (<code>src/app/input.rs</code>) BEFORE <code>state.request_selection(...)</code> and return on a hit.</li>
<li>Units bite: <code>Input.last_cursor</code> is surface px (<code>CursorMoved</code> multiplies by <code>surface_per_physical</code>), <code>Screen</code> is CSS px. Divide by <code>device_pixel_ratio</code> (<code>src/engine/gpu/view.rs</code>) and lay out against <code>State::logical_size</code>.</li>
<li>Hover on <code>CursorMoved</code> when not dragging; return <code>true</code> only when the hovered row CHANGED, else every mouse motion redraws the scene.</li>
<li><code>l</code> toggles the panel in <code>Input::key</code> — taken are c, f, q, w, e, o, d, h, s, t, b, p, 1–7, <code>[</code>, <code>]</code>, Space, Escape, F10.</li>
</ul>
<h2 id="expand-collapse-scroll-search">Expand, collapse, scroll, search<a class="anchor" href="#/course/extend-panels#expand-collapse-scroll-search" aria-label="Link to this section">#</a></h2>
<ul>
<li>Open set <code>HashSet&lt;(usize, Vec&lt;u16&gt;)&gt;</code>: document index plus child-index path. A path survives because both loaders preserve child order.</li>
<li>Never <code>TreeNode::guid</code>: minted lazily from a per-process <code>OnceLock</code> and restored by neither loader (<code>build_tree</code> in <code>src/app/decode.rs</code> and the kernel&#39;s <code>proto_to_treenode</code> both call <code>TreeNode::new(&amp;proto.name)</code>). Keyed on it, groups collapse and tree filters orphan on every reload.</li>
<li>Default closed.</li>
<li>Search filters the flat list, no auto-expand: that is 56,889 entries rebuilt per keystroke, and an unreadable tree. Matches flat, capped, counted, group as trailing context.</li>
<li>Match <code>Scene::object_name(row)</code>, which falls back to the TYPE — &quot;Line&quot;, &quot;Mesh&quot;. Never the raw guid: 56,889 UUIDs are unreadable.</li>
</ul>
<h2 id="panel-selection-and-viewport-selection-in-step">Panel selection and viewport selection, in step<a class="anchor" href="#/course/extend-panels#panel-selection-and-viewport-selection-in-step" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>State::select(Option&lt;u32&gt;)</code> is the ONE place the highlight moves; a leaf click calls it, and the panel touches neither <code>Scene</code> nor <code>Gpu</code>.</li>
<li><code>Scene.selected</code> is one <code>Option&lt;u32&gt;</code>, so a GROUP row cannot select its subtree — multi-selection also changes <code>fit_selected_or_all</code>, the nameplate and <code>State::enable_controls</code>.</li>
<li>A group row&#39;s targets: its triangle, its filter dot, and its label, which expands — the generous hit area.</li>
<li>Reveal-on-pick inside <code>State::select</code>: <code>row_node[row]</code> up <code>Node.parent</code> (O(depth)), open ancestors, scroll into view. No cross-app flag, no picking change.</li>
<li>Scroll only when the row is outside the window, else it fights the user&#39;s own scrolling.</li>
<li>The filter dot is DERIVED from <code>Filters.tree</code>; a stored flag drifts the moment <code>H</code>, <code>show_all</code> or another axis touches those rows.</li>
</ul>
<h2 id="reload-rebuild-and-what-survives">Reload, rebuild, and what survives<a class="anchor" href="#/course/extend-panels#reload-rebuild-and-what-survives" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>Scene::rebuild</code> keeps <code>docs</code> and <code>hidden</code> and re-runs <code>add_file</code>, so the indexes return with the rows — provided <code>reset_rows</code> cleared them, or they double.</li>
<li><code>Scene::clear</code> drops documents and <code>hidden</code>; <code>State::clear</code> must clear <code>Filters</code> too, or a new scene starts half-hidden.</li>
<li>Open state and scroll are path-keyed: they survive a rebuild of the same documents and die with a <code>clear</code>.</li>
<li>Streamed clouds and sheets cannot return after a rebuild: there is no kernel object to re-walk, and <code>reset_rows</code> empties <code>Scene.streamed</code> and <code>Scene.sheets</code> before the first <code>add_file</code>, so the documents survive in <code>docs</code> as empty shells that walk to no rows. Refuse a reparent or delete while either is non-empty, say why on the row, and drop their rows if a rebuild happens anyway.</li>
<li>Nothing prunes panel state: rebuild the flat list after every <code>add_file</code>, <code>rebuild</code> and <code>clear</code>, and drop entries whose document index is gone.</li>
</ul>
<h2 id="the-awkward-rows">The awkward rows<a class="anchor" href="#/course/extend-panels#the-awkward-rows" aria-label="Link to this section">#</a></h2>
<ul>
<li>STREAMED CLOUD: one row, guid <code>stream:{url}</code>, points not all resident (<code>done_to</code>, <code>total</code>). A single leaf under its own document; hiding is one flag write however much has arrived.</li>
<li>STREAMED SHEET: one row, guid <code>sheet:{url}</code>, an EMPTY <code>Session::new(&amp;name)</code> — no tree, graph or lookup. Hence the panel&#39;s ROOT is the document list, with session trees nested under it.</li>
<li>LOCAL SHEET: <code>assets/view_local.yaml</code> loads <code>pb/view_local_sheet_querschnitt.pb</code> through <code>add_file</code> — 56,889 rows, each with a tree node and a graph vertex. Every cost claim here must survive that file.</li>
<li>TEXT OBJECTS: rows with owner <code>usize::MAX</code>, keys like <code>document-title/0</code>. <code>Scene::visible_texts</code> and <code>restore_text_visibility</code> both recompute from <code>hidden</code>, so a filter calling <code>Gpu::set_hidden</code> alone is silently un-hidden by the next text replacement — write <code>Scene.hidden</code>.</li>
<li>Text rows belong to no document: give them their own section.</li>
</ul>
<h2 id="edits-and-the-one-undo">Edits, and the one undo<a class="anchor" href="#/course/extend-panels#edits-and-the-one-undo" aria-label="Link to this section">#</a></h2>
<ul>
<li>A hide records NOTHING: <code>Op</code> (<code>session_rust/src/history.rs</code>) is <code>Add | Remove | Replace | Xform</code>, and hiding is view state dropped by <code>Scene::clear</code>. Recording it would make Ctrl+Z un-hide.</li>
<li>No viewer-side undo stack: transactions, tombstones and the cursor are the kernel&#39;s.</li>
<li>Transactions only for document changes (<code>History::begin</code>, <code>record</code>, <code>commit</code>): reparent and delete, not hide, expand, scroll or filter.</li>
<li>No reparent op: <code>Tombstone</code> carries <code>parent_guid</code>, <code>index</code>, the detached <code>node</code> subtree, the graph <code>attribute</code> and incident <code>edges</code>, so a reparent round-trips as Remove + Add in ONE transaction. Otherwise add a kernel op — say which.</li>
<li><code>&amp;mut Session</code>: <code>FileDoc.session</code> is <code>Rc&lt;Session&gt;</code>; the only precedent is <code>Rc::make_mut(&amp;mut scene.docs[0].session)</code> in <code>scene.rs</code>&#39;s tests — in place with a sole handle, a full clone otherwise. Check before editing 56,889 objects.</li>
</ul>
<h2 id="the-order-that-compiles">The order that compiles<a class="anchor" href="#/course/extend-panels#the-order-that-compiles" aria-label="Link to this section">#</a></h2>
<ol>
<li><code>src/app/filter.rs</code>, registered in <code>src/app/mod.rs</code>: <code>Kind</code>, <code>TypeRanges</code>, <code>TreeIndex</code>, <code>GraphIndex</code>, <code>Filter</code> — pure data, no <code>Scene</code>, no <code>Gpu</code>. Tests: a path key round-trips, a range is half-open, an empty index answers with no rows.</li>
<li><code>Scene</code> grows <code>types</code>, <code>trees</code>, <code>graphs</code> — filled in <code>add_file</code>, cleared in <code>reset_rows</code>, synthetic entries from <code>add_streamed_cloud</code>/<code>add_sheet</code>. Check: existing <code>scene.rs</code> tests unchanged.</li>
<li><code>Scene::filter_rows(&amp;Filter) -&gt; Vec&lt;u32&gt;</code>, tested in <code>scene.rs</code>: two documents sharing a guid (the existing collision test), a point/line/mesh document proving contiguity, an <code>Element</code> with no geometry shortening only its own block.</li>
<li><code>InstanceTable::set_flags_run</code> and <code>Gpu::set_hidden_rows</code> — no callers, headless GPU tests still pass.</li>
<li><code>State::hide_rows</code>; <code>hide_selected</code> its one-row case, <code>show_all</code> its clear-all. <code>hidden_rows</code> iterates a <code>HashSet</code>, so sort before coalescing.</li>
<li><code>set_filter</code>/<code>isolate</code>/<code>apply_filters</code>, <code>Filters</code> on <code>State</code>, cleared in <code>State::clear</code>. Native example before any UI: load querschnitt, hide <code>Kind::Line</code>, assert the hidden count and ONE run.</li>
<li><code>src/app/panel.rs</code> — rows, open set, scroll, window, <code>row_at(css_x, css_y)</code>; pure CSS-px geometry, no device.</li>
<li><code>src/engine/gpu/panel.rs</code> — the chrome lane before <code>self.text.draw(pass)</code>: empty quad list, then one fixed rectangle, then the row model.</li>
<li>Panel labels inside <code>State::update_label</code>, visible window only; <code>?inspect=1</code> already reports <code>text_labels</code>.</li>
<li><code>src/app/input.rs</code> — <code>l</code>, the hit test ahead of <code>request_selection</code>, hover on <code>CursorMoved</code>. First end-to-end step.</li>
<li>Reveal-on-pick inside <code>State::select</code>.</li>
<li><code>?inspect=1</code> gains <code>filters</code>, <code>panel_rows</code>, <code>panel_rect</code> and a <code>hidden</code> count (<code>src/app/inspection.rs</code>), plus one <code>tests/*.cjs</code> beside <code>interaction.cjs</code>: a type bucket hides the right number of rows, a row click selects without firing a scene pick.</li>
</ol>
<h2 id="what-we-take-from-the-old-viewer-and-what-we-do-not">What we take from the old viewer and what we do not<a class="anchor" href="#/course/extend-panels#what-we-take-from-the-old-viewer-and-what-we-do-not" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>PORTS — the leaf rule</strong>, a kernel property, against ONE guid set (<code>Scene::guid_to_row</code>) instead of that viewer&#39;s three disagreeing ones.</li>
<li><strong>PORTS — the subtree leaf DFS, precomputed</strong>; per frame it is O(subtree) per group row. The result here is <code>Vec&lt;u32&gt;</code> of ROWS, the write address.</li>
<li><strong>PORTS — the indent arithmetic</strong>: <code>depth * INDENT_W</code> for a group, <code>+ ARROW_W</code> for a leaf.</li>
<li><strong>PORTS — derived group state</strong>: the dot is computed, never stored, so it cannot drift from the viewport.</li>
<li><strong>PORTS — claim the icon cells first, let the name shrink</strong>; the rule ports, the widget calls do not.</li>
<li><strong>ADAPTS — reveal-on-pick</strong>: a flag the picking code reached across the app to set becomes a step inside <code>State::select</code>.</li>
<li><strong>ADAPTS — panel click to selection</strong>: a row click calls <code>State::select</code>, and a group row does not select at all.</li>
<li><strong>ADAPTS — visibility</strong>: a UI-written guid set becomes <code>Scene.hidden</code>, keyed <code>(document, guid)</code> because guids collide between documents, re-applied by <code>add_file</code>, written through one action.</li>
<li><strong>ADAPTS — search</strong>: the substring predicate ports, the auto-expand does not — one keystroke would rebuild 56,889 entries.</li>
<li><strong>REPLACED — egui and every widget call</strong>: none in <code>Cargo.toml</code>, no DOM panel in <code>index.html</code>. Rows are text-lane labels plus one quad lane.</li>
<li><strong>REPLACED — the hit test</strong>: <code>Response</code>-based becomes a four-comparison CPU box test.</li>
<li><strong>REPLACED — the per-frame snapshot</strong>: a guid→label map, a cloned leaf cache and a re-listed edge set are six figures of allocation per frame here. Indexes are built once, in <code>add_file</code>.</li>
<li><strong>REPLACED — the god state and its change vectors</strong>: they escaped a borrow conflict inside a UI closure; there is no closure — <code>src/app/input.rs</code> calls a named action on <code>&amp;mut State</code>.</li>
<li><strong>REPLACED — the viewer-side undo stack</strong>: the history is <code>session_rust/src/history.rs</code>.</li>
<li><strong>REPLACED — nothing, for the colour columns</strong>: per-object override does not exist, <code>Instance.color</code> is written by the walk. Two pickers per row means an override system, a re-apply path after every rebuild, and a double write under group and leaf keys. Layer colour&#39;s honest source is <code>TreeNode.color</code> — a kernel change, three encode/decode gaps first.</li>
<li><strong>REPLACED — nothing, for the separate web app</strong>: its one transferable idea, labelling a graph node by its <code>attribute</code>, arrives via <code>Vertex.attribute</code>.</li>
<li><strong>REJECTED — the group selection lock</strong>: it makes one pick a multi-object selection this viewer cannot represent.</li>
<li><strong>REJECTED — the per-leaf transform lock</strong>: its only consumer there was a transform gizmo, which this viewer does not have.</li>
<li><strong>REJECTED — behaviour keyed on a literal group name</strong>: auto-hiding groups called &quot;FloorModel&quot; gives a different file different behaviour with nothing on screen saying so.</li>
<li><strong>REJECTED — <code>TreeNode::guid</code> as any key</strong>: minted per process, restored by neither loader.</li>
</ul>
<h2 id="what-to-check-on-screen">What to check on screen<a class="anchor" href="#/course/extend-panels#what-to-check-on-screen" aria-label="Link to this section">#</a></h2>
<ul>
<li>Step 3: nothing visible; <code>cargo test</code> passes, new <code>filter_rows</code> tests and every pre-existing <code>scene.rs</code> test.</li>
<li>Step 5: <code>H</code> still hides the selection, <code>S</code> still shows everything, on a local file and a streamed sheet. Nothing on screen changed — that is the check.</li>
<li>Step 6: hiding <code>Kind::Line</code> on querschnitt leaves the page blank of linework and reports one run; <code>show_all</code> restores it, frame time unchanged.</li>
<li>Step 8: one fixed rectangle over the scene at the panel&#39;s position, in front of geometry, unmoved by orbiting.</li>
<li>Step 9: rows read at the top-left, clipped at the right edge, scrolling without flicker; <code>?inspect=1</code> shows <code>text_labels</code> rising by the window count, not the row count.</li>
<li>Step 10: <code>l</code> toggles the panel; hover highlights only that row; a leaf click selects that object and shows its nameplate; clicks inside the panel never change the 3D selection; clicks outside still pick.</li>
<li>Step 11: picking an object opens its groups and scrolls its row into view exactly once, landing on the first matching row.</li>
<li>Step 12: a type bucket on querschnitt removes exactly that bucket&#39;s objects; the graph attribute row removes the same set; both on, then one off, leaves the other&#39;s rows hidden.</li>
</ul>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/extend-panels#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Reference result from the supported <a href="#/course/22-runtime-helpers">implementation tutorials</a>. The expanded Session layers panel shows Assembly → Nested; selecting Nested highlights its two beams together. This maintained-viewer capture uses egui styling; the independent panel lesson uses DOM buttons with the same hierarchy and selection behavior. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-panels.png"><img src="/session/docs/course/docs/screenshots/extensions-panels.png" alt="Full viewer result for extend panels" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"one-visibility-set-three-filters",text:"One visibility set, three filters"},{level:2,id:"what-each-filter-keys-on",text:"What each filter keys on"},{level:3,id:"by-element-type",text:"By element type"},{level:3,id:"by-tree-node",text:"By tree node"},{level:3,id:"by-graph-element",text:"By graph element"},{level:2,id:"one-visibility-decision-and-the-path-it-takes-to-the-gpu",text:"One visibility decision, and the path it takes to the GPU"},{level:2,id:"what-a-hide-costs",text:"What a hide costs"},{level:2,id:"where-the-indexes-are-built",text:"Where the indexes are built"},{level:2,id:"the-panels-rows",text:"The panel's rows"},{level:2,id:"drawing-the-panel",text:"Drawing the panel"},{level:2,id:"hit-testing-on-the-cpu-deliberately",text:"Hit testing: on the CPU, deliberately"},{level:2,id:"expand-collapse-scroll-search",text:"Expand, collapse, scroll, search"},{level:2,id:"panel-selection-and-viewport-selection-in-step",text:"Panel selection and viewport selection, in step"},{level:2,id:"reload-rebuild-and-what-survives",text:"Reload, rebuild, and what survives"},{level:2,id:"the-awkward-rows",text:"The awkward rows"},{level:2,id:"edits-and-the-one-undo",text:"Edits, and the one undo"},{level:2,id:"the-order-that-compiles",text:"The order that compiles"},{level:2,id:"what-we-take-from-the-old-viewer-and-what-we-do-not",text:"What we take from the old viewer and what we do not"},{level:2,id:"what-to-check-on-screen",text:"What to check on screen"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{e as default};
