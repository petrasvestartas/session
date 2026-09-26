const e={title:"22 · Refresh diagnostics and resource checks",html:`<h1 id="22-refresh-diagnostics-and-resource-checks">22 · Refresh diagnostics and resource checks<a class="anchor" href="#/course/22-runtime-helpers#22-refresh-diagnostics-and-resource-checks" aria-label="Link to this section">#</a></h1>
<p>Continuing from checkpoint 21, the grid still orbits, pans and zooms while diagnostics retain the first GPU failure.</p>
<h2 id="step-1-srcenginegpudevicers">Step 1 · src/engine/gpu/device.rs<a class="anchor" href="#/course/22-runtime-helpers#step-1-srcenginegpudevicers" aria-label="Link to this section">#</a></h2>
<p>Two edits: keep the first device error when later submissions fail; test that the original message survives.</p>
<p><code>lessons/22/src/engine/gpu/device.rs</code> · edit · type this</p>
<p>Replaces the line <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> in <code>lessons/21/src/engine/gpu/device.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Store the first error message; later ones are ignored.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">fn</span> remember_failure(failure: &amp;std::sync::Mutex&lt;Option&lt;String&gt;&gt;, message: String) {
    <span class="k">if</span> <span class="k">let</span> Ok(<span class="k">mut</span> state) = failure.lock() {
        state.get_or_insert(message);</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/21/src/engine/gpu/device.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
#[test]
<span class="c">/// The first stored error stays when more arrive.</span>
<span class="k">fn</span> first_gpu_error_survives_follow_on_submission_errors() {
    <span class="k">let</span> failure = std::sync::Mutex::new(None);
    remember_failure(&amp;failure, &quot;<span class="s">texture allocation failed</span>&quot;.into());
    remember_failure(&amp;failure, &quot;<span class="s">invalid command buffer</span>&quot;.into());
    assert_eq!(
        failure.lock().unwrap().as_deref(),
        Some(&quot;<span class="s">texture allocation failed</span>&quot;)
    );
}</code></pre></div>
<h2 id="step-2-srcenginegpusurface_outliners">Step 2 · src/engine/gpu/surface_outline.rs<a class="anchor" href="#/course/22-runtime-helpers#step-2-srcenginegpusurface_outliners" aria-label="Link to this section">#</a></h2>
<p>Three edits: measure fractional silhouette coverage with selection disabled and enabled; check the accumulated coverage; require the same outline width in both states.</p>
<p><code>lessons/22/src/engine/gpu/surface_outline.rs</code> · edit · type this</p>
<p>Replaces the line <code>gpu.view.show_mesh_edges = false;</code> in <code>lessons/21/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        gpu.segments.set_selected(<span class="s">0</span>, <span class="s">false</span>);
                        <span class="k">let</span> silhouette = gpu.render_offscreen(&amp;input);
                        gpu.segments.set_selected(<span class="s">0</span>, <span class="s">true</span>);
                        <span class="k">let</span> edged = gpu.render_offscreen(&amp;input);
                        <span class="k">let</span> <span class="k">mut</span> coverage = <span class="s">0_u32</span>;

                        <span class="k">for</span> (plain, inked) <span class="k">in</span> silhouette.chunks_exact(<span class="s">4</span>).zip(edged.chunks_exact(<span class="s">4</span>))
                        {
                            <span class="k">let</span> lo = *plain[..<span class="s">3</span>].iter().min().unwrap();
                            <span class="k">let</span> hi = *plain[..<span class="s">3</span>].iter().max().unwrap();

                            <span class="k">if</span> hi - lo &lt;= <span class="s">2</span> {
                                coverage += u32::from(<span class="s">255</span> - hi);
                            }

                            <span class="k">if</span> hi &lt; <span class="s">8</span> {</code></pre></div>
<p>Replaces the 2 lines from <code>black &gt; 500,</code> in <code>lessons/21/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                            coverage &gt; gpu.config.height * <span class="s">255</span> / <span class="s">2</span>,
                            &quot;<span class="s">visible silhouette includes fractional coverage: </span>{<span class="s">coverage</span>}&quot;</code></pre></div>
<p>Replaces the 3 lines from <code>assert!(</code> in <code>lessons/21/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            assert_eq!(
                plain_black, black,
                &quot;<span class="s">ordinary and selected outlines have the same width</span>&quot;</code></pre></div>
<h2 id="step-3-srcenginegpuuploadrs">Step 3 · src/engine/gpu/upload.rs<a class="anchor" href="#/course/22-runtime-helpers#step-3-srcenginegpuuploadrs" aria-label="Link to this section">#</a></h2>
<p>Replace the upload vector with an empty vector to release its allocation.</p>
<p><code>lessons/22/src/engine/gpu/upload.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>v.clear();</code> in <code>lessons/21/src/engine/gpu/upload.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    *v = Vec::new();</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/22-runtime-helpers#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/22/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: the grid and world axes remain visible, orbit still works, and the status area has no error message.</p>
<p><img src="/session/docs/course/docs/screenshots/current-empty.png" alt="Full viewer result for lesson 22" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Later errors hide the allocation failure: the stored failure is overwritten.</li>
<li>Memory stays allocated after replacement: the upload vector is cleared but its capacity is retained.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/22-runtime-helpers#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/22/src/
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
│   ├── edit.rs
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
│   │   ├── device.rs  ~
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
│   │   ├── surface_outline.rs  ~
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── upload.rs  ~
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
│   ├── edit.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: GPU failure or scene replacement → diagnostics and released upload storage.
Every file at this point: <code>lessons/22/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/22-runtime-helpers#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/23-geometry-commands">23 · Create, trim, extend and explode</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/22-runtime-helpers#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>With an empty scene, the viewer shows the grid and world axes. Orbit, pan and zoom should work. Runtime diagnostics add no visible editing control. Captured in the maintained viewer with an empty manifest and no object selected.</p>
<p><a href="/session/docs/course/docs/screenshots/current-empty.png"><img src="/session/docs/course/docs/screenshots/current-empty.png" alt="Full viewer result for lesson 22" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpudevicers",text:"Step 1 · src/engine/gpu/device.rs"},{level:2,id:"step-2-srcenginegpusurface_outliners",text:"Step 2 · src/engine/gpu/surface_outline.rs"},{level:2,id:"step-3-srcenginegpuuploadrs",text:"Step 3 · src/engine/gpu/upload.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{e as default};
