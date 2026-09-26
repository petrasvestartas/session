const s={title:"20 · The document: undo, redo and save",html:`<h1 id="20-the-document-undo-redo-and-save">20 · The document: undo, redo and save<a class="anchor" href="#/course/20-history#20-the-document-undo-redo-and-save" aria-label="Link to this section">#</a></h1>
<p>The sheet scene stays visible while document edits gain undo, redo and history-free saving.</p>
<p>A removal marks the object dead in place, so undo revives the same object without a copy; checkpoints purge what history can no longer reach.</p>
<p><img src="/session/docs/course/docs/illustrations/history.svg" alt="Edits group into transactions and a removal leaves a tombstone to restore from; the cursor moves back and forward through them, and a save purges the whole buffer because history never crosses pb or JSON." loading="lazy" decoding="async"></p>
<h2 id="step-1-session_rustsrchistoryrs">Step 1 · session_rust/src/history.rs<a class="anchor" href="#/course/20-history#step-1-session_rustsrchistoryrs" aria-label="Link to this section">#</a></h2>
<p>Read this source file from its link; the checkpoint already contains it.</p>
<details class="note"><summary><code>session_rust/src/history.rs</code> · read only</summary><p><a href="#/course/kernel/history">Open the full listing</a></p>
</details>
<h2 id="step-2-session_rustsrcsessionrs">Step 2 · session_rust/src/session.rs<a class="anchor" href="#/course/20-history#step-2-session_rustsrcsessionrs" aria-label="Link to this section">#</a></h2>
<p>Read this source file from its link; the checkpoint already contains it.</p>
<details class="note"><summary><code>session_rust/src/session.rs</code> · read only</summary><p><a href="#/course/kernel/session">Open the full listing</a></p>
</details>
<p>Run <code>cargo check</code> in <code>lessons/20/</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/20-history#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/20/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The sheet scene stays visible while document edits gain undo, redo and history-free saving; status: <strong>the status clears when loading finishes</strong>.</p>
<p><a href="/session/docs/course/docs/screenshots/19-sheets-overview.png"><img src="/session/docs/course/docs/screenshots/19-sheets-overview.png" alt="Full viewer result for 20 history" loading="lazy" decoding="async"></a></p>
<p>If it fails:</p>
<ul>
<li>Undo fails to restore a removed object: its geometry and tree position are not both recorded.</li>
<li>Saved history reappears: serialization includes the undo stack.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/20-history#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/20/session_rust/src/
├── bin/
│   ├── minitest.rs
│   └── pdf_import.rs
├── proto/
│   ├── .gitattributes
│   └── session_proto.rs
├── aabb.rs
├── boolean_polyline.rs
├── brep.rs
├── closest.rs
├── color.rs
├── convex_hull.rs
├── element.rs
├── file_encoders.rs
├── file_obj.rs
├── file_step.rs
├── graph.rs
├── guid_serde.rs
├── history.rs
├── instance_ref.rs
├── intersection.rs
├── io_xyz.rs
├── lib.rs
├── line.rs
├── main.rs
├── matrix.rs
├── mesh.rs
├── mesh_offset.rs
├── nurbscurve.rs
├── nurbsknot.rs
├── nurbssurface.rs
├── nurbssurface_trimmed.rs
├── obb.rs
├── objects.rs
├── pdf.rs
├── plane.rs
├── point.rs
├── pointcloud.rs
├── polyline.rs
├── primitives.rs
├── quaternion.rs
├── remesh_cdt.rs
├── remesh_nurbssurface_adaptive.rs
├── remesh_nurbssurface_grid.rs
├── render_mesh.rs
├── session.rs
├── session_config.rs
├── simple_split.rs
├── spatial_aabbtree.rs
├── spatial_bvh.rs
├── spatial_kdtree.rs
├── spatial_octree.rs
├── spatial_rtree.rs
├── tolerance.rs
├── tree.rs
├── vector.rs
└── xform.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/20/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/20-history#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/21-editing">21 · Editing: the gumball, the command line and the layers panel</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/20-history#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The picture is unchanged from lesson 19; undo, redo and save are the new behavior.</p>
<p><a href="/session/docs/course/docs/screenshots/19-sheets-overview.png"><img src="/session/docs/course/docs/screenshots/19-sheets-overview.png" alt="Full viewer result for 20 history" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-session_rustsrchistoryrs",text:"Step 1 · session_rust/src/history.rs"},{level:2,id:"step-2-session_rustsrcsessionrs",text:"Step 2 · session_rust/src/session.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
