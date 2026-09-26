const e={title:"Extension roadmap and historical designs",html:`<h1 id="extension-roadmap-and-historical-designs">Extension roadmap and historical designs<a class="anchor" href="#/course/extend-roadmap#extension-roadmap-and-historical-designs" aria-label="Link to this section">#</a></h1>
<p>Current implementation and independent teaching steps: <a href="#/course/22-runtime-helpers">Editing extensions</a>. The designs below are historical; use the supplement’s status table for supported operations and limits.</p>
<p>The frozen lesson-21 viewer reads, draws, picks, streams, moves, deletes and edits. The maintained viewer also creates points, lines and polylines through typed commands. <code>session_viewer_archive</code> (~11,000 lines of <code>src/</code>) created too, in a different architecture.</p>
<h2 id="three-rules-every-design-below-obeys">Three rules every design below obeys<a class="anchor" href="#/course/extend-roadmap#three-rules-every-design-below-obeys" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>A widget never touches the session.</strong> Events reach <code>src/app/input.rs</code>, which calls a named action on <code>State</code> (<code>escape_selection</code>, <code>enable_controls</code>, <code>hide_selected</code>, <code>fit_selected_or_all</code>, <code>set_cloud_size</code>, <code>request_selection</code>). New work is one more named action, never a reach into <code>self.scene.docs[..].session</code>.</li>
<li><strong>Undo lives in the kernel.</strong> <code>session_rust/src/history.rs</code>: transactions, tombstones, cursor; <code>Session::{begin, commit, undo, redo}</code>. A viewer stack is a second cursor that can disagree, and a save purges only one.</li>
<li><strong>Pixels come out of lanes.</strong> <code>Gpu</code> lists them by hand in <code>src/engine/gpu/mod.rs</code>: <code>backdrop, arena, segments, glyphs, controls, control_net, text, selection_outline, solid_outline, cloud, splat, pick</code>. New geometry reuses one or becomes one more.</li>
</ul>
<h2 id="the-four-guides">The four guides<a class="anchor" href="#/course/extend-roadmap#the-four-guides" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>Gumball</strong>, <strong>Command line</strong> and <strong>Tree</strong> were the designs; <a href="#/course/21-editing">lesson 21</a> is the
code they became. They are kept as the reasoning behind it, not as instructions to follow.</li>
<li><strong>This page</strong> preserves the original gap analysis; the linked implementation table records what remains.</li>
</ul>
<h2 id="built-in-lesson-21">Built, in lesson 21<a class="anchor" href="#/course/extend-roadmap#built-in-lesson-21" aria-label="Link to this section">#</a></h2>
<p>Nine of the gaps this page listed are closed. The code is the description now; a second one
here would drift from it.</p>
<table>
<thead>
<tr>
<th>was</th>
<th>now</th>
</tr>
</thead>
<tbody><tr>
<td>no screen ray</td>
<td><code>Camera::ray</code>, f64</td>
</tr>
<tr>
<td>no construction plane, no typed coordinates</td>
<td><code>app/cplane.rs</code>, <code>app/coords.rs</code></td>
</tr>
<tr>
<td>no object snapping</td>
<td><code>app/snap.rs</code>, ranked in screen space</td>
</tr>
<tr>
<td>nothing can be moved</td>
<td>the gumball, and <code>move</code> on the command line</td>
</tr>
<tr>
<td>no delete</td>
<td><code>Delete</code>, and <code>delete</code></td>
</tr>
<tr>
<td>no undo or redo</td>
<td>the kernel&#39;s history, per document</td>
</tr>
<tr>
<td>no sub-object editing that writes back</td>
<td>a control drag, through <code>Session::replace</code></td>
</tr>
<tr>
<td>no text entry anywhere on the page</td>
<td>the command line, opened with <code>:</code></td>
</tr>
<tr>
<td>no panel of rows beside the scene</td>
<td>the layers panel, opened with <code>L</code></td>
</tr>
</tbody></table>
<h3 id="5-interactive-drawing-remains">5 · Interactive drawing remains<a class="anchor" href="#/course/extend-roadmap#5-interactive-drawing-remains" aria-label="Link to this section">#</a></h3>
<ul>
<li>Click or typed coordinate adds a point, Enter finishes, <code>c</code> closes, <code>u</code> drops the last, Esc cancels, rubber band follows. Needs (1)–(4) plus a transient segment-lane region.</li>
<li>~400 lines against 564 (<code>tool_state.rs</code> 77, <code>state_tool.rs</code> 487): the preview is a lane write, the commit four kernel calls rather than four plus GPU add plus undo push plus label rebuild. (command line for verbs, this page for the loop)</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch — the state, owned by State, driven only by named actions</span>
<span class="k">enum</span> Tool { Idle, Point, Line, Polyline, Curve { degree: usize }, Move }

<span class="k">struct</span> Draft {
 tool: Tool,
 points: Vec&lt;Point&gt;, <span class="c">// f64 kernel points, never f32</span>
 cursor: Option&lt;SnapHit&gt;, <span class="c">// resolved once per pointer move</span>
 dirty: bool, <span class="c">// preview rebuilt in render, not per event</span>
}

<span class="k">impl</span> State {
 <span class="k">pub</span> <span class="k">fn</span> tool_begin(&amp;<span class="k">mut</span> <span class="k">self</span>, tool: Tool);
 <span class="k">pub</span> <span class="k">fn</span> tool_hover(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64);
 <span class="k">pub</span> <span class="k">fn</span> tool_point(&amp;<span class="k">mut</span> <span class="k">self</span>, p: Point);
 <span class="k">pub</span> <span class="k">fn</span> tool_text(&amp;<span class="k">mut</span> <span class="k">self</span>, line: &amp;str);
 <span class="k">pub</span> <span class="k">fn</span> tool_finish(&amp;<span class="k">mut</span> <span class="k">self</span>);
 <span class="k">pub</span> <span class="k">fn</span> tool_cancel(&amp;<span class="k">mut</span> <span class="k">self</span>);
}</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch — the only shape an edit may take in this viewer</span>
<span class="k">let</span> doc = &amp;<span class="k">mut</span> <span class="k">self</span>.scene.docs[owner];
<span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> doc.session); <span class="c">// FIRST: other placements share this Rc</span>
session.begin(&quot;<span class="s">Polyline</span>&quot;); <span class="c">// no begin -&gt; History::record is a silent no-op</span>
session.add_polyline(polyline, None);
session.commit; <span class="c">// no commit -&gt; nothing on the undo stack</span>
<span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu); <span class="c">// rows, ids, bounds, masks, hidden flags follow</span>
<span class="k">self</span>.touch;</code></pre></div>
<ul>
<li><strong>Bites.</strong> Without <code>begin</code>, <code>History::record</code> returns at once (<code>history.rs:249-255</code>) and <code>replace</code>/<code>set_xform</code>/<code>_add_object</code> snapshot only when <code>history.current.is_some</code>: the edit works and is silently unundoable.</li>
<li><strong>Bites.</strong> <code>Rc::make_mut</code> first — a manifest listing one file twice hands both documents the same <code>Rc</code> (<code>scene.rs</code>).</li>
<li><strong>Bites.</strong> <code>State::render</code> never requests the next frame: a rubber band that does not move is a missing <code>touch</code>.</li>
<li><strong>Bites.</strong> <code>Scene::rebuild</code>&#39;s only call site is <code>src/selftest/lifecycle.rs</code>, asserting a rebuilt scene renders pixel-identical to the incrementally loaded one. The commit path stands on that test.</li>
<li><strong>Bites.</strong> <code>rebuild</code> cannot restore a streamed cloud or sheet — no kernel object to walk — so a commit in such a scene must not take that path.</li>
</ul>
<h3 id="9-no-multi-selection-no-box-select">9 · No multi-selection, no box select<a class="anchor" href="#/course/extend-roadmap#9-no-multi-selection-no-box-select" aria-label="Link to this section">#</a></h3>
<ul>
<li>Depends on nothing, but everything after changes shape if it lands late. Archive <code>state_pick.rs</code> is 401 lines; here small and wide. (tree, which needs the same set)</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, src/app/scene.rs</span>
<span class="k">pub</span> selected: Vec&lt;u32&gt;, <span class="c">// was Option&lt;u32&gt;</span></code></pre></div>
<ul>
<li><strong>Touches.</strong> <code>State::select</code>, <code>hide_selected</code>, <code>fit_selected_or_all</code>, <code>enable_controls</code>, <code>apply_pick</code>, <code>update_label</code>, <code>Gpu::set_selected</code>, <code>selection_outline.set_selected</code>, the name annotations in <code>src/state/text.rs</code>.</li>
<li><strong>Bites.</strong> <code>Gpu::selection_revision</code> is in the coverage mask key (<code>MaskKey.selection</code>, <code>surface_outline.rs:30-40</code>): bump once per gesture, not per row.</li>
<li><strong>Not the archive&#39;s box test.</strong> <code>process_box_select</code> projects one centroid per object (<code>state_pick.rs:37-90</code>), so a long polyline crossing the box is missed. The picker reads ids in a window (<code>gpu/pick.rs</code>) and <code>Window::with_radius</code> goes to <code>MAX_RADIUS</code> 128: read the rectangle&#39;s ids, deduplicate rows, done.</li>
</ul>
<h3 id="11-no-greville-edit-points">11 · No Greville edit points<a class="anchor" href="#/course/extend-roadmap#11-no-greville-edit-points" aria-label="Link to this section">#</a></h3>
<ul>
<li>Handles <em>on</em> a curve or surface instead of in the control cage; dragging one refits the control points so the curve passes through the drag. Needs (10).</li>
<li>Archive <code>edit_points.rs</code>, 157 lines of which ~65 are its two tests: Greville abscissae ξᵢ = mean of <em>p</em> = order−1 consecutive knots; Eᵢ = C(ξᵢ); square rational collocation matrix Rⱼᵢ = wᵢNᵢ(ξⱼ)/Σwₖ Nₖ(ξⱼ); column <em>k</em> of R⁻¹ solved once at drag start against a unit vector, so each move is ΔPᵢ = col[i]·Δ — weights preserved, a circle stays a circle.</li>
<li><strong>The one file that ports unchanged</strong>: it imports <code>session_rust::{nurbsknot, Matrix, NurbsCurve, Point}</code> and nothing else. Its second test asserts to 1e-7 that dragging edit point <em>k</em> by Δ moves it by exactly Δ. (this page)</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, src/app/selection.rs — the only additions ControlId needs</span>
<span class="k">enum</span> ControlId {
 Vertex(usize), Curve { curve: usize, point: usize },
 Surface { surface: usize, u: usize, v: usize }, Point(u32),
 CurveEdit { curve: usize, index: usize }, <span class="c">// new</span>
 SurfaceEdit { surface: usize, index: usize }, <span class="c">// new</span>
}</code></pre></div>
<ul>
<li><strong>Bites.</strong> <code>Matrix::solve</code> returning <code>None</code> is the degenerate-curve guard: refuse the drag, do not unwrap.</li>
</ul>
<h3 id="12-no-live-deform-during-a-drag">12 · No live deform during a drag<a class="anchor" href="#/course/extend-roadmap#12-no-live-deform-during-a-drag" aria-label="Link to this section">#</a></h3>
<ul>
<li>Needs (10) and an in-place range rewrite meshes lack; archive ~350 lines plus the lane work. (gumball)</li>
<li><strong>The premise ports.</strong> A NURBS point is linear in its control points: freeze the tessellation at drag start, precompute each tessellation vertex&#39;s influence weight per moved control point, and every move is a multiply-add — no <code>point_at</code>, no <code>normal_at</code>, no re-tessellation. The adaptive rebuild runs once, on release.</li>
<li><strong>The lane gap.</strong> <code>Buffer::write_at</code> (<code>buffers.rs</code>) plus <code>Scene::ribbon_range(row)</code> means strokes can be rewritten in place today. Faces cannot: <code>ArenaLane</code>&#39;s <code>verts</code> is private (<code>arena.rs</code>), <code>Scene</code> keeps no per-row vertex range. Adding one is a real lane change — <code>Vec&lt;Option&lt;Range&lt;u32&gt;&gt;&gt;</code> beside <code>ribbon_ranges</code>, filled in <code>add_file</code>, plus <code>ArenaLane::write_verts</code>.</li>
<li><strong>Bites.</strong> An in-place vertex write does not bump <code>geometry_revision</code>, the key for the triangle tile index (<code>ProjectionKey { matrix, objects }</code>, <code>triangle_tiles.rs:178-181</code>) and the silhouette masks (<code>MaskKey.geometry</code>): visibility then culls against the pre-drag projection while the picture shows the deformed one. Bump per drag frame, paying a re-projection — the correct default — or prove the cheaper thing in a comment.</li>
<li><strong>Bites.</strong> <code>Instance::FLAG_SMOOTH</code> says a row is a tessellation; a deformed tessellation still is one.</li>
</ul>
<h3 id="13-no-edge-you-can-drag">13 · No edge you can drag<a class="anchor" href="#/course/extend-roadmap#13-no-edge-you-can-drag" aria-label="Link to this section">#</a></h3>
<ul>
<li>Ctrl+Shift+click an edge and move it, the highlight following the real curve, not its chorded control polygon. Needs (10). (gumball)</li>
<li><strong>Half exists</strong>: <code>PickMode::Edge</code>, <code>Scene::edge_at(pick)</code>, <code>SegRows.pipe_ids</code>, <code>SegmentLane::set_edge</code> (<code>src/state.rs</code>, <code>segments.rs</code>); every subdivision keeps its source edge id. The archive&#39;s 140 lines of edge selection are replaced; only the drag and write remain.</li>
<li><strong>Ports as a rule.</strong> The drag highlight is the frozen base polyline mapped by the delta: a whole-edge transform is affine, so nothing is re-evaluated per move.</li>
</ul>
<h3 id="14-no-brep-hole-collar-deform">14 · No BRep hole → collar deform<a class="anchor" href="#/course/extend-roadmap#14-no-brep-hole-collar-deform" aria-label="Link to this section">#</a></h3>
<ul>
<li>Pull a hole rim: the rim follows, the wall swells smoothly, the outer boundary holds, the trim is untouched. Archive ~300 lines — degree-elevate the face, inner radius from the rim, outer radius from the nearest outer-boundary control point, each control point moved by <code>g·(delta·P − P)</code> with <code>g</code> a smoothstep from 1 inside to 0 outside. Needs (10) and (12).</li>
<li><strong>A shape, not code:</strong> the viewer supplies a delta and two radii, the kernel does the geometry — which is why this does not belong in the viewer at all.</li>
<li><strong>Bites.</strong> The archive&#39;s write-back skips these faces: the flat overlay nodes are stale after a collar and writing them would undo it (<code>state_edit.rs:1387-1390</code>). Any deform moving control points the overlay does not know about needs the same exclusion.</li>
</ul>
<h3 id="15-no-selection-filters">15 · No selection filters<a class="anchor" href="#/course/extend-roadmap#15-no-selection-filters" aria-label="Link to this section">#</a></h3>
<ul>
<li>Not a CPU test after the pick: <code>Picker::configure(mode, radius_css, scale)</code> decides which lanes draw ids, so the filter is a mask carried into that decision and a filtered-out lane never writes an id.</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, src/engine/gpu/pick.rs</span>
<span class="k">struct</span> Filter(u32); <span class="c">// one bit per lane: faces, strokes, markers, clouds, text, sheets</span>
<span class="k">enum</span> PickMode { Object { filter: Filter }, Edge, Component, Controls { parent: u32, cloud: bool } }</code></pre></div>
<ul>
<li>Depends on nothing; under 100 lines (mask, per-lane draw guards in <code>gpu/pick.rs</code>, a setter). Cheapest item, and the one that makes the others usable. (tree)</li>
<li><strong>Bites.</strong> The archive filters by walking <code>session.lookup</code>, because its pick is a CPU raycast. Here that means picking twice, and it still picks wrong when the filtered-out object is in front, because the id pass already resolved depth against it.</li>
</ul>
<h3 id="16-no-tessellation-wireframe-toggle">16 · No tessellation wireframe toggle<a class="anchor" href="#/course/extend-roadmap#16-no-tessellation-wireframe-toggle" aria-label="Link to this section">#</a></h3>
<ul>
<li><strong>Keep the decision:</strong> 8 iso-curves per direction, adaptively polylined, not every triangle edge — ~25× fewer line vertices for the same answer. <strong>Not the implementation:</strong> it frees and reallocates every <code>&quot;__tess__&quot;</code>-prefixed arena slot per toggle and hand-allocates a tint instance to force black lines.</li>
<li><code>src/app/walk/mesh_ink.rs</code> decides seams at walk time, <code>VIEWER_SEAMS</code> (<code>src/app/knobs.rs</code>) is the launch-time switch; the toggle is a <code>View</code> field plus a walk variant, shaped like <code>Q</code>, <code>W</code>, <code>E</code>, <code>O</code>. Under 100 lines. (this page)</li>
</ul>
<h3 id="17-no-auto-naming-no-prompt-line">17 · No auto-naming, no prompt line<a class="anchor" href="#/course/extend-roadmap#17-no-auto-naming-no-prompt-line" aria-label="Link to this section">#</a></h3>
<ul>
<li><code>polyline_7</code>; <code>Polyline: next point (3) — Enter=finish, c=close, u=undo, Esc=cancel</code>.</li>
<li><code>app/feedback::status</code> writes one line into <code>#viewer-status</code> as <code>textContent</code>, never HTML. A prompt is a status line; the archive&#39;s log of the last 200 prompts is a panel, and belongs to the tree guide if wanted. Under 30 lines. (command line)</li>
</ul>
<h3 id="18-no-in-app-geometry-construction-and-it-stays-that-way">18 · No in-app geometry construction, and it stays that way<a class="anchor" href="#/course/extend-roadmap#18-no-in-app-geometry-construction-and-it-stays-that-way" aria-label="Link to this section">#</a></h3>
<ul>
<li><strong>Not a gap.</strong> Scenes come from manifests and <code>.pb</code> files (<code>src/app/manifest.rs</code>, <code>loader.rs</code>, <code>route.rs</code>, <code>live.rs</code>), validated before kernel constructors allocate from serialized counts (<code>src/app/validate.rs</code>). <code>ARCHITECTURE.md</code> §8 lists the archive&#39;s <code>demo.rs</code> (860 lines) as a structural defect — &quot;app data, not engine&quot;, <code>State::new</code> hardcoding <code>demo::active_scene</code>.</li>
<li><strong>Worth having:</strong> primitive <em>commands</em> (<code>box</code>, <code>sphere</code>) constructing through <code>Session::add_*</code> inside a transaction — item 5 with different verbs. (command line)</li>
</ul>
<h2 id="what-we-take-from-the-old-viewer-and-what-we-do-not">What we take from the old viewer and what we do not<a class="anchor" href="#/course/extend-roadmap#what-we-take-from-the-old-viewer-and-what-we-do-not" aria-label="Link to this section">#</a></h2>
<p><strong>Ports</strong> — <code>edit_points.rs</code> (157 lines), kernel-only and tested; <code>coord_parser.rs</code> (44 lines) at f64; <code>SnapKind</code>, its priority ladder and <code>SnapModes</code> (priority class first, pixel distance breaks ties); absolute snapshots on both sides of a recorded edit, and the accumulating-delta bug behind them; the live-deform premise (freeze, precompute weights, multiply-add); f64 through, f32 at the boundary, now in the write direction too; iso-curves, not triangle edges.</p>
<p><strong>Adapts</strong> — getpoint loop → a <code>Draft</code> driven by named actions with <code>touch</code> at every mutation; snap emission ports but the <code>session.lookup</code> sweep does not (ask what a row is); snap ranking stays CPU only for candidates no lane draws; invalidation → a revision counter; construction plane → forward-axis rule kept, world-origin placeholder replaced; <code>NodeAddr</code> → <code>ControlId</code> plus two Greville arms and the write direction; the two write hazards (mesh BVH, BRep 3D edges) as rules; bake-vs-matrix → the question, re-answered against the two-table transform; multi-selection yes, the centroid box test → a rectangle id read; the transient preview → an owned region of the segment and glyph lanes; <code>log_prompt</code> → <code>feedback::status</code>.</p>
<p><strong>Replaced</strong> — <code>undo_state.rs</code> + <code>state_undo.rs</code> (303 lines), because the kernel history is the cursor; <code>EditState</code>&#39;s GPU half, because the <code>controls</code>/<code>control_net</code> lanes are it; <code>build_overlay_data</code>, because <code>Controls::from_geometry</code> covers more types and is tested; <code>pick.rs</code> CPU raycasting and <code>closest_object_under_ray</code>, because selection is GPU ids with a halo, a generation and a mode — the kernel&#39;s <code>ray_cast</code> has never been called here and should not start; <code>selected_centroid</code> and its per-type arms, because <code>InstanceTable::row_bounds(row)</code> gives the box and a widget origin is its centre; <code>text.rs</code> (256 lines, an 8×16 atlas as screen-anchored quads), replaced by text objects with scene identity; <code>demo.rs</code> (860 lines), replaced by manifests, loader, routes and live reload; <code>rebuild_tess_wireframe</code> as written; <code>selftest.rs</code> (164 lines), replaced by the headless harness; Escape falling through to <code>event_loop.exit</code>, since Escape here is <code>escape_selection</code> with &quot;cancel the active tool&quot; in front of it.</p>
<ul>
<li>Also replaced: the <code>impl State</code>-at-crate-root arrangement (<code>state_tool.rs</code>, <code>state_edit.rs</code>, <code>state_undo.rs</code>, <code>state_interaction.rs</code>, <code>state_pick.rs</code>, <code>state_cmd.rs</code>, <code>state_ui.rs</code>, <code>state_render.rs</code>, <code>state_update.rs</code>). <code>commit_object_transform</code> alone reads and writes the session lookup, the xform table, four GPU pick-mesh caches, instance flags, colour overrides, the thickness uniform and the text labels. That coupling is why the gap list cannot be closed by copying files.</li>
</ul>
<h2 id="the-order-to-build-the-rest-in">The order to build the rest in<a class="anchor" href="#/course/extend-roadmap#the-order-to-build-the-rest-in" aria-label="Link to this section">#</a></h2>
<p>Each step compiles; each names what it must not break. Steps 1 to 8 of the original order are
lesson 21; what follows is what is left.</p>
<ol>
<li><strong>Filters</strong> (15) — picking: mask full is byte-identical, the headless harness proves it.</li>
<li><strong>Multi-selection and rectangle pick</strong> (9) — silhouettes (<code>selection_revision</code> once per gesture), sheets (a sheet row selects the sheet).</li>
<li><strong>Getpoint and the four tools</strong> (5), rubber band in a transient <code>segments</code> region — finite-triangle visibility (the preview is strokes), text identity (<code>register_text</code> on rebuild), one frame per demand (<code>touch</code>).</li>
<li><strong>Control write-back on Mesh and NurbsSurface</strong> — lesson 21 does Polyline and NurbsCurve; a mesh vertex and a surface CV need <code>invalidate_triangle_bvh</code> and the CAD contract re-derived by <code>app/walk/brep_edges.rs</code> on rebuild.</li>
<li><strong>Greville edit points</strong> (11) — nothing; the two <code>ControlId</code> arms are additive.</li>
<li><strong>Live deform</strong> (12) plus the per-row vertex range — the tile index and silhouettes (bump <code>geometry_revision</code>), <code>FLAG_SMOOTH</code>.</li>
<li><strong>Edge drag</strong> (13), <strong>wireframe toggle</strong> (16), <strong>naming and prompts</strong> (17) — the keyboard map, <code>feedback::status</code> staying <code>textContent</code>.</li>
<li><strong>BRep collar</strong> (14), if wanted at all, as a kernel operation the viewer parameterizes.</li>
</ol>
<h2 id="what-to-check-on-screen">What to check on screen<a class="anchor" href="#/course/extend-roadmap#what-to-check-on-screen" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>1.</strong> Strokes-only filter: a face click selects nothing, an edge click selects. Full filter: screenshot matches the old one, pixel for pixel.</li>
<li><strong>2–3.</strong> With <code>?inspect=1</code>, a click logs a world point that does not move when you orbit and click the same spot on the same object.</li>
<li><strong>4.</strong> Type in the field, press <code>F</code>: nothing fits. Escape, <code>F</code>: it fits.</li>
<li><strong>5.</strong> <code>point 0,0,0</code> marks the origin; reload and it is gone, because nothing was saved.</li>
<li><strong>6.</strong> Draw, undo, redo, undo — the status line names the transaction each time.</li>
<li><strong>7.</strong> Delete, undo: the object returns in the same tree position with the same name, not appended.</li>
<li><strong>8.</strong> Endpoint reads &quot;End&quot;, mid-edge &quot;Mid&quot;; a streamed cloud gives no snap, no freeze, no error.</li>
<li><strong>9.</strong> Five-point polyline from three clicks and two typed coordinates; rubber band at full frame rate; <code>u</code> drops, <code>c</code> closes, Esc leaves nothing.</li>
<li><strong>10.</strong> A rectangle across a long polyline whose middle is outside the box: selected.</li>
<li><strong>11.</strong> Move an object a kilometre out by one millimetre, orbit until it re-anchors: the move survives.</li>
<li><strong>12–13.</strong> F10, drag, release: the object deforms, the outline follows, a click picks the deformed surface.</li>
<li><strong>14.</strong> Drag an edit point on a circle: it follows exactly, and the circle is still a circle.</li>
<li><strong>15.</strong> Drag a surface control point across another object&#39;s silhouette: the two stay correctly ordered throughout the drag, not only after release.</li>
</ul>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/extend-roadmap#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The completed viewer has a command dock across the bottom, one right-hand layer tree with bulbs, selection locks and color swatches, and a left toolbar. Split keeps both face regions in the joined shell. The selected region has its gumball; Save/Open retains the edited geometry and layer settings. See the <a href="/session/docs/course/docs/screenshots/extensions-workspace-current-phone.png">phone layout</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-workspace-current-desktop.png"><img src="/session/docs/course/docs/screenshots/extensions-workspace-current-desktop.png" alt="Full viewer result for extend roadmap" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"three-rules-every-design-below-obeys",text:"Three rules every design below obeys"},{level:2,id:"the-four-guides",text:"The four guides"},{level:2,id:"built-in-lesson-21",text:"Built, in lesson 21"},{level:3,id:"5-interactive-drawing-remains",text:"5 · Interactive drawing remains"},{level:3,id:"9-no-multi-selection-no-box-select",text:"9 · No multi-selection, no box select"},{level:3,id:"11-no-greville-edit-points",text:"11 · No Greville edit points"},{level:3,id:"12-no-live-deform-during-a-drag",text:"12 · No live deform during a drag"},{level:3,id:"13-no-edge-you-can-drag",text:"13 · No edge you can drag"},{level:3,id:"14-no-brep-hole-collar-deform",text:"14 · No BRep hole → collar deform"},{level:3,id:"15-no-selection-filters",text:"15 · No selection filters"},{level:3,id:"16-no-tessellation-wireframe-toggle",text:"16 · No tessellation wireframe toggle"},{level:3,id:"17-no-auto-naming-no-prompt-line",text:"17 · No auto-naming, no prompt line"},{level:3,id:"18-no-in-app-geometry-construction-and-it-stays-that-way",text:"18 · No in-app geometry construction, and it stays that way"},{level:2,id:"what-we-take-from-the-old-viewer-and-what-we-do-not",text:"What we take from the old viewer and what we do not"},{level:2,id:"the-order-to-build-the-rest-in",text:"The order to build the rest in"},{level:2,id:"what-to-check-on-screen",text:"What to check on screen"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{e as default};
