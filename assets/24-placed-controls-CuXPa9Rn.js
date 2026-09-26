const s={title:"24 · Make control dragging respect object placement",html:`<h1 id="24-make-control-dragging-respect-object-placement">24 · Make control dragging respect object placement<a class="anchor" href="#/course/24-placed-controls#24-make-control-dragging-respect-object-placement" aria-label="Link to this section">#</a></h1>
<p>Control points follow a placed object correctly during dragging and return to their source positions on cancellation.</p>
<h2 id="step-1-srcappeditrs">Step 1 · src/app/edit.rs<a class="anchor" href="#/course/24-placed-controls#step-1-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Convert the world-space control target through the inverse object placement, then test the result under translation and scale.</p>
<p><code>lessons/24/src/app/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn set_control_point(&amp;mut self, row: u32, index: usiz…</code> in <code>lessons/23/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> Some(back) = <span class="k">self</span>.placement_of(row).and_then(|place| place.inverse()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> local = to.transformed(&amp;back); <span class="c">// world point into the object's frame</span>
        <span class="k">let</span> to = &amp;local;</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/23/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A world point edit lands in local units under a placement.</span>
    #[test]
    <span class="k">fn</span> control_edit_converts_world_to_local_under_file_placement() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">placed</span>&quot;);
        assert!(
            source
                .add_polyline(
                    session_rust::Polyline::new(vec![
                        Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                        Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)
                    ]),
                    None
                )
                .is_some()
        );
        <span class="k">let</span> shared = Rc::new(source);
        <span class="k">let</span> <span class="k">mut</span> placed = file(&quot;<span class="s">placed</span>&quot;, Rc::clone(&amp;shared));
        placed.place = &amp;Xform::translation(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>) * &amp;Xform::scale_xyz(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(placed);
        scene.add_file(file(&quot;<span class="s">unmodified</span>&quot;, shared));
        assert!(scene.set_control_point(<span class="s">0</span>, <span class="s">1</span>, &amp;Point::new(<span class="s">120</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)));
        <span class="k">let</span> Geometry::Polyline(line) = scene.geometry(<span class="s">0</span>).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(line.get_point(<span class="s">1</span>).unwrap()[<span class="s">0</span>], <span class="s">2</span>.<span class="s">0</span>);
        assert_eq!(line.get_point(<span class="s">1</span>).unwrap()[<span class="s">1</span>], <span class="s">3</span>.<span class="s">0</span>);
        <span class="k">let</span> Geometry::Polyline(other) = scene.geometry(<span class="s">1</span>).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(other.get_point(<span class="s">1</span>).unwrap()[<span class="s">0</span>], <span class="s">1</span>.<span class="s">0</span>);
        assert!(scene.undo());
        <span class="k">let</span> Geometry::Polyline(line) = scene.geometry(<span class="s">0</span>).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(line.get_point(<span class="s">1</span>).unwrap()[<span class="s">0</span>], <span class="s">1</span>.<span class="s">0</span>);
    }</code></pre></div>
<h2 id="step-2-srcstaters">Step 2 · src/state.rs<a class="anchor" href="#/course/24-placed-controls#step-2-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Cancel an active gesture before clearing the scene or changing selection.</p>
<p><code>lessons/24/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn clear(&amp;mut self) {</code> in <code>lessons/23/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_gesture();</code></pre></div>
<p>Added after the line <code>pub fn select(&amp;mut self, row: Option&lt;u32&gt;) {</code> in <code>lessons/23/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_gesture();
        <span class="c">// back to plain object mode, no edge, face or controls</span></code></pre></div>
<h2 id="step-3-srcstateeditrs">Step 3 · src/state/edit.rs<a class="anchor" href="#/course/24-placed-controls#step-3-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Restore the grab placement before committing, restore controls on cancellation, and retain the control origin for placed previews.</p>
<p><code>lessons/24/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>};</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// put the preview back, the document applies the real move</span>
        <span class="k">self</span>.gpu
            .objects
            .set_placement(&amp;<span class="k">self</span>.gpu.ctx, active.row, &amp;active.base_place);
        <span class="k">self</span>.gpu.grew_bounds(active.row);
        <span class="k">self</span>.touch();</code></pre></div>
<p>Replaces the line <code>if self.control_drag.take().is_some() {</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(active) = <span class="k">self</span>.control_drag.take() {
            <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.scene.geometry(active.parent) {
                <span class="k">self</span>.controls = <span class="k">crate</span>::app::selection::Controls::from_geometry(geometry);
            }</code></pre></div>
<p>Added after the line <code>plane: CPlane,</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    origin: Point, <span class="c">// where the point was at the grab</span></code></pre></div>
<p>Replaces the line <code>let Some((sx, sy)) = self.project(at) else {</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> Some(place) = <span class="k">self</span>.scene.placement_of(parent) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> origin = Point::new(at[<span class="s">0</span>], at[<span class="s">1</span>], at[<span class="s">2</span>]).transformed(&amp;place);
        <span class="k">let</span> Some((sx, sy)) = <span class="k">self</span>.project([origin[<span class="s">0</span>], origin[<span class="s">1</span>], origin[<span class="s">2</span>]]) <span class="k">else</span> {</code></pre></div>
<p>Added after the line <code>plane: CPlane::facing(&amp;forward),</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            origin,</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> Some(back) = <span class="k">self</span>
            .scene
            .placement_of(active.parent)
            .and_then(|place| place.inverse())
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> point = point.transformed(&amp;back); <span class="c">// into the object's own frame</span></code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.scene.geometry(active.parent) {
            <span class="k">self</span>.controls = <span class="k">crate</span>::app::selection::Controls::from_geometry(geometry);
        }

        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.touch();</code></pre></div>
<p>Replaces the 5 lines from <code>let origin = {</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> free = active.plane.hit(&amp;active.origin, &amp;from, &amp;dir)?; <span class="c">// the plane point</span>
        <span class="k">let</span> place = <span class="k">self</span>.scene.placement_of(active.parent)?;</code></pre></div>
<p>Replaces the line <code>),</code> in <code>lessons/23/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                )
                .transformed(&amp;place),</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/24-placed-controls#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/24/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: a placed control moves in world space, Escape restores it, and the status area shows no error.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-controls.png" alt="Full viewer result for lesson 24" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A placed control jumps while dragging: the world target is stored as a local coordinate.</li>
<li>Escape leaves a displaced control: cancellation does not restore source-derived controls.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/24-placed-controls#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/24/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   ├── points.rs
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── edit.rs  ~
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── input.rs
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl
│   └── triangle_tiles.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: pointer target → inverse placement → local control → one source edit.
Every file at this point: <code>lessons/24/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/24-placed-controls#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/25-gumball">25 · Draw a solid, readable gumball</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/24-placed-controls#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The placed polyline remains in the scene with its source controls visible after a control has been dragged. Check the released control position and control polygon against the surrounding geometry. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>. At this checkpoint the gumball still uses strokes; the next chapter adds solid handles. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-controls.png"><img src="/session/docs/course/docs/screenshots/extensions-controls.png" alt="Full viewer result for lesson 24" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappeditrs",text:"Step 1 · src/app/edit.rs"},{level:2,id:"step-2-srcstaters",text:"Step 2 · src/state.rs"},{level:2,id:"step-3-srcstateeditrs",text:"Step 3 · src/state/edit.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
