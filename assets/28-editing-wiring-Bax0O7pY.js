const s={title:"28 · Finish the shared editing wiring",html:`<h1 id="28-finish-the-shared-editing-wiring">28 · Finish the shared editing wiring<a class="anchor" href="#/course/28-editing-wiring#28-finish-the-shared-editing-wiring" aria-label="Link to this section">#</a></h1>
<p>The floating interface, nested panel and control editing work together before the workspace becomes docked.</p>
<h2 id="step-1-srcappeditrs">Step 1 · src/app/edit.rs<a class="anchor" href="#/course/28-editing-wiring#step-1-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Refuse unsupported controls and display-only documents, then cover both cases with tests.</p>
<p><code>lessons/28/src/app/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn delete_row(&amp;mut self, row: u32) -&gt; bool {</code> in <code>lessons/27/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> <span class="s">false</span>; <span class="c">// streamed scenes are not editable</span>
        }</code></pre></div>
<p>Delete the 15 lines from <code>fn a_kind_with_no_control_points_is_refused() {</code> in <code>lessons/27/src/app/edit.rs</code>.</p>
<p>Added after the line <code>}</code> in <code>lessons/27/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A point has no control points to edit.</span>
    #[test]
    <span class="k">fn</span> a_kind_with_no_control_points_is_refused() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        assert!(!scene.set_control_point(<span class="s">0</span>, <span class="s">0</span>, &amp;Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)));
    }

    <span class="c">/// A display-only document refuses edits.</span>
    #[test]
    <span class="k">fn</span> a_display_only_document_refuses_the_edit() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        scene.docs[<span class="s">0</span>].display_only = <span class="s">true</span>;
        assert!(
            scene
                .transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;)
                .is_none()
        );
    }</code></pre></div>
<h2 id="step-2-srcappinspectionrs">Step 2 · src/app/inspection.rs<a class="anchor" href="#/course/28-editing-wiring#step-2-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Expose the new selection and resource state to the browser inspection data.</p>
<p><code>lessons/28/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the line <code>&quot;draw_calls&quot;: state.gpu.performance.draws,</code> in <code>lessons/27/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">widget</span>&quot;: state.gpu.widget.placement,
        &quot;<span class="s">widget_highlight</span>&quot;: state.gpu.widget.active,
        &quot;<span class="s">widget_bytes</span>&quot;: state.gpu.widget.allocated_bytes(),</code></pre></div>
<p>Added after the line <code>&quot;gpu_texture_estimate_bytes&quot;: textures,</code> in <code>lessons/27/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">egui_private_gpu_capacity</span>&quot;: &quot;<span class="s">renderer buffers and font atlas are managed by egui; excluded from totals</span>&quot;,</code></pre></div>
<h2 id="step-3-srcappmodrs">Step 3 · src/app/mod.rs<a class="anchor" href="#/course/28-editing-wiring#step-3-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/28/src/app/mod.rs</code> · edit · type this</p>
<p>Delete the 2 lines from <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> in <code>lessons/27/src/app/mod.rs</code>.</p>
<p>Added after the line <code>pub mod inspection;</code> in <code>lessons/27/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> ui;</code></pre></div>
<h2 id="step-4-srcstateeditrs">Step 4 · src/state/edit.rs<a class="anchor" href="#/course/28-editing-wiring#step-4-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Reconnect command visibility and preserve streamed-source guards around Undo and Redo.</p>
<p><code>lessons/28/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn undo(&amp;mut self) {</code> in <code>lessons/27/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> !<span class="k">self</span>.scene.streamed.is_empty() || !<span class="k">self</span>.scene.sheets.is_empty() {
            <span class="k">self</span>.status(&quot;<span class="s">Undo requires a scene without streamed sources</span>&quot;);
            <span class="k">return</span>;
        }</code></pre></div>
<p>Added after the line <code>pub fn redo(&amp;mut self) {</code> in <code>lessons/27/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> !<span class="k">self</span>.scene.streamed.is_empty() || !<span class="k">self</span>.scene.sheets.is_empty() {
            <span class="k">self</span>.status(&quot;<span class="s">Redo requires a scene without streamed sources</span>&quot;);
            <span class="k">return</span>;
        }</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/28-editing-wiring#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/28/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: the egui windows and placed control edits work together, and a successful geometry command reports <strong>geometry updated</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-command-create.png" alt="Full viewer result for lesson 28" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Undo loses streamed data: a history rebuild accepts an incomplete source.</li>
<li>A new helper is unresolved: its module declaration is missing.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/28-editing-wiring#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/28/src/
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
│   ├── hierarchy.rs
│   ├── input.rs
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   ├── ui.rs
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
│   │   ├── ui.rs
│   │   ├── upload.rs
│   │   ├── view.rs
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
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
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── panel.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: UI action → shared edit helpers → source history → refreshed display.
Every file at this point: <code>lessons/28/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/28-editing-wiring#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/29-docked-workspace">29 · Dock the workspace, edit source geometry and save</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/28-editing-wiring#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 7 completes the original floating-window interface. The next chapter adds the docked workspace, source subobject edits, touch gumball and Save/Open. This is a maintained-viewer reference; its bottom command dock and toolbar are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-command-create.png"><img src="/session/docs/course/docs/screenshots/extensions-command-create.png" alt="Full viewer result for lesson 28" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappeditrs",text:"Step 1 · src/app/edit.rs"},{level:2,id:"step-2-srcappinspectionrs",text:"Step 2 · src/app/inspection.rs"},{level:2,id:"step-3-srcappmodrs",text:"Step 3 · src/app/mod.rs"},{level:2,id:"step-4-srcstateeditrs",text:"Step 4 · src/state/edit.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
