const e={title:"Optional design reference · Section plane",html:`<h1 id="optional-design-reference-section-plane">Optional design reference · Section plane<a class="anchor" href="#/course/capstone#optional-design-reference-section-plane" aria-label="Link to this section">#</a></h1>
<p>This proposed feature is not implemented in the current viewer. This page gives its design answers and acceptance criteria, not a code tutorial. For complete runnable code, follow the course through to <a href="#/course/37-command-dock">lesson 37</a>.</p>
<p>The proposal is a <strong>section plane</strong>: a movable plane that cuts the scene, so faces, edges, markers and text on the far side disappear and a solid&#39;s interior becomes visible.</p>
<p><img src="/session/docs/course/docs/illustrations/capstone-01.svg" alt="Diagram: keyboard · pointer\\ which plane, where · state\\ the plane as data · uniform\\ plane reaches the GPU · every shader that draws\\ faces · ink · markers · text · pixels\\ cut away or kept · picking\\ does a cut object still answer?" loading="lazy" decoding="async"></p>
<h2 id="requirements">Requirements<a class="anchor" href="#/course/capstone#requirements" aria-label="Link to this section">#</a></h2>
<ul>
<li>One plane: a point and a normal, in the same anchored world frame the rest of the scene uses.</li>
<li>Everything on the negative side is gone: faces, edges, vertex markers, point clouds, text. Not faded — gone.</li>
<li>A key toggles it; the camera can move freely with it on.</li>
<li>Picking agrees with the picture: a cut-away part of an object cannot be clicked.</li>
<li>Turning it off costs nothing when it is off.</li>
</ul>
<h2 id="constraints">Constraints<a class="anchor" href="#/course/capstone#constraints" aria-label="Link to this section">#</a></h2>
<p>Respect these and the feature will fit; break them and you will feel the friction immediately.</p>
<ul>
<li>The scene contract (<code>src/shaders/scene.wgsl</code>) is declared once and every lane shader is compiled with it.</li>
<li>Lanes do not know about each other. Anything all lanes need belongs in the contract, not in six copies.</li>
<li>Pipelines are built once, in <code>build</code>, never per frame.</li>
<li>A view switch is a <em>view</em> change: no row is rewritten, no buffer is rebuilt, no geometry is walked again.</li>
<li>The id pass and the visible pass must see the same world, or picking lies.</li>
</ul>
<h2 id="design-questions">Design questions<a class="anchor" href="#/course/capstone#design-questions" aria-label="Link to this section">#</a></h2>
<p>The answers are written out directly below. No question must be solved to continue the course.</p>
<ol>
<li>Where does the plane live — in <code>Scene</code>, in <code>View</code>, in a lane? Which one survives a scene reload, and should it?</li>
<li>How does it reach the GPU? Which existing uniform block already goes everywhere, and what does adding four floats to it cost?</li>
<li>Which shader stage rejects a fragment on the far side — vertex or fragment? What breaks if you choose the other one?</li>
<li>A triangle straddling the plane: what happens at its cut edge, and what would a CAD user expect to see there instead?</li>
<li>Does the depth buffer need to change? Does the coverage mask that draws silhouettes?</li>
<li>Picking: is this one more shader change, or does it come for free? <em>Why</em> does it come for free?</li>
<li>When the plane is off, what code runs? What should run?</li>
</ol>
<p>Each is worked through below, then written out in full in the answer key.</p>
<h2 id="working-it-out">Working it out<a class="anchor" href="#/course/capstone#working-it-out" aria-label="Link to this section">#</a></h2>
<h3 id="1-where-the-plane-belongs">1 · Where the plane belongs<a class="anchor" href="#/course/capstone#1-where-the-plane-belongs" aria-label="Link to this section">#</a></h3>
<p><code>P</code> (x-ray) and <code>D</code> (headlight) are the analogue: no row, <code>View</code> state, carried in the block already bound to every lane. A section plane is the same — view state, not scene state — so it survives a reload, as a user expects.</p>
<h3 id="2-getting-four-floats-everywhere">2 · Getting four floats everywhere<a class="anchor" href="#/course/capstone#2-getting-four-floats-everywhere" aria-label="Link to this section">#</a></h3>
<p><code>LineUniform</code> is group 1 for every lane and is declared once in <code>scene.wgsl</code>. A plane is a <code>vec4</code>: <code>xyz</code> the unit normal, <code>w</code> the offset, so <code>dot(p, n) - w</code> is the signed distance. Adding a field touches the Rust struct, the offset assertions and the WGSL declaration — the three declarations of <a href="#/course/debugging">Habit 1</a>; the <code>instance.rs</code> mirror test catches a miss.</p>
<h3 id="3-which-stage-cuts">3 · Which stage cuts<a class="anchor" href="#/course/capstone#3-which-stage-cuts" aria-label="Link to this section">#</a></h3>
<p>A vertex-stage rejection removes only whole triangles, so the cut face keeps a ragged edge of whichever vertices survived. Fragment-stage <code>discard</code> cuts exactly at the plane, at pixel resolution — the reason x-ray discards in the fragment stage too. Its cost: a discarding fragment shader cannot be early-depth-tested, so compile it out when the plane is off.</p>
<h3 id="4-the-cut-face">4 · The cut face<a class="anchor" href="#/course/capstone#4-the-cut-face" aria-label="Link to this section">#</a></h3>
<p>Discarding alone leaves a hollow shell: you see the <em>inside</em> of the far wall, lit from the wrong side. A CAD viewer fills the cut with a cap. First version: paint back faces in a flat cap colour — the viewer already detects facing for the red back-face debug paint. Stencilling a true cap is a stretch goal, not a requirement.</p>
<p><img src="/session/docs/course/docs/illustrations/section-plane.svg" alt="A vertex-stage rejection can only drop whole triangles, so the cut face keeps a staircase of mesh edges; a fragment-stage discard cuts exactly on the plane, but discarding alone leaves a hollow shell until a cap fills it." loading="lazy" decoding="async"></p>
<h3 id="5-picking-for-free">5 · Picking for free<a class="anchor" href="#/course/capstone#5-picking-for-free" aria-label="Link to this section">#</a></h3>
<p>The id pass runs the <em>same</em> vertex and fragment entry points over the same rows, with the pick camera in the same uniform block. If the cut is a <code>discard</code> in a function both entry points call, a cut-away fragment writes no id, and picking agrees with the picture with no extra work.</p>
<h3 id="6-paying-nothing-when-off">6 · Paying nothing when off<a class="anchor" href="#/course/capstone#6-paying-nothing-when-off" aria-label="Link to this section">#</a></h3>
<p>Two ways, not equivalent. A uniform branch is predictable and uniform — cheap, one pipeline. A pipeline-overridable constant (<code>override</code>, as <code>SCENE_MSAA</code> does) compiles the test away entirely, at the cost of a second pipeline variant per lane. For a plane test the honest answer is usually the branch; measure before assuming otherwise.</p>
<h2 id="the-answer-key">The answer key<a class="anchor" href="#/course/capstone#the-answer-key" aria-label="Link to this section">#</a></h2>
<p><strong>A design that satisfies the constraints</strong></p>
<ul>
<li><strong>State</strong>: one <code>Option&lt;Plane&gt;</code> (or a <code>vec4</code> plus a <code>bool</code>) in <code>View</code>, beside <code>lit</code>, <code>opacity</code> and <code>show_outlines</code>. Toggled by a key in <code>input.rs</code> through a small <code>State</code> method that calls <code>touch</code>. No row, no buffer, no walk.</li>
<li><strong>Transport</strong>: a <code>vec4&lt;f32&gt;</code> appended to <code>LineUniform</code>, with its offset asserted on the Rust side. Off is a zero normal, so the test is <code>dot(n, p) - w &lt; 0.0 &amp;&amp; any(n != 0)</code> — or a separate <code>enabled</code> float if you prefer readability over packing. Both defensible; pick one and say why in a comment.</li>
<li><strong>Cut</strong>: one function in the scene contract, <code>fn clipped(world_pos: vec3&lt;f32&gt;) -&gt; bool</code>, called at the top of every fragment entry that draws world geometry — faces, ribbons, markers, splats, text planes — and in the id entries by virtue of being in the same functions. One declaration, every lane.</li>
<li><strong>Cap</strong>: back faces on the cut side painted flat. Correct for convex solids, visibly wrong for a solid with an internal void — document that limit rather than hiding it.</li>
<li><strong>Masks</strong>: the coverage mask must discard too, or the silhouette will outline the part you cut away. This is the step most people miss; the symptom is a black ring floating in space.</li>
<li><strong>Off</strong>: a uniform branch, one pipeline, nothing allocated. The plane&#39;s <code>w</code> changes per frame while dragging, and that is a <code>write_buffer</code> of one small block — the same cost as moving the camera.</li>
<li><strong>Tests</strong>: a native test that puts a known point on each side of a known plane and asserts the signed distance&#39;s sign; a <code>cargo xtest</code> shader parse to keep WGSL honest. Both run without a GPU.</li>
</ul>
<p>What is <em>not</em> in this design: no new lane, no new pipeline family, no trait, no per-object clipping state. A feature that fits the architecture adds one field and one function. If yours needed a new module, ask which constraint pushed you there.</p>
<h2 id="acceptance-criteria-for-the-proposed-feature">Acceptance criteria for the proposed feature<a class="anchor" href="#/course/capstone#acceptance-criteria-for-the-proposed-feature" aria-label="Link to this section">#</a></h2>
<p>This is a design reference for a future section plane, not a runnable implementation lesson. The current viewer does not implement it. Complete the course through <a href="#/course/37-command-dock">lesson 37</a> for the full supported implementation; no section-plane code is needed to finish that sequence. A future implementation must satisfy these checks:</p>
<ul>
<li><strong>It compiles at every step.</strong> Add the field and the assertion first, and check. Then the contract function, unused, and check. Then one lane. Then the rest.</li>
<li><strong>It fails visibly when wrong.</strong> Set the plane to cut through the middle of the fixture and orbit. A plane that moves with the camera means you tested in view space instead of world space.</li>
<li><strong>Picking agrees.</strong> Click where the cut removed geometry: nothing should be selected. If something is, your id entry point skipped the test.</li>
<li><strong>Silhouettes agree.</strong> Turn outlines on (<code>O</code>) with the plane on. A ring around missing geometry means the mask pass is not clipping.</li>
<li><strong>Off is off.</strong> Toggle it off and compare a screenshot with one from before your change. They must be identical, pixel for pixel.</li>
</ul>
<h2 id="if-you-want-more">If you want more<a class="anchor" href="#/course/capstone#if-you-want-more" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>More planes.</strong> Store a fixed-size plane array and an active count in the uniform. Test each active plane in a bounded loop. The count is shared by the draw, so loop iterations remain uniform; per-fragment discard results differ.</li>
<li><strong>A dimension primitive.</strong> Reuse stroke rows for the leader and arrow strokes, text placement for its label, and one source identity for their picks. Add a lane only if those existing representations cannot express the required drawing behavior.</li>
<li><strong>Contribute it.</strong> A clean section plane belongs in the viewer. Read <code>ARCHITECTURE.md</code>: changing source the course teaches means refolding the course.</li>
</ul>
`,toc:[{level:2,id:"requirements",text:"Requirements"},{level:2,id:"constraints",text:"Constraints"},{level:2,id:"design-questions",text:"Design questions"},{level:2,id:"working-it-out",text:"Working it out"},{level:3,id:"1-where-the-plane-belongs",text:"1 · Where the plane belongs"},{level:3,id:"2-getting-four-floats-everywhere",text:"2 · Getting four floats everywhere"},{level:3,id:"3-which-stage-cuts",text:"3 · Which stage cuts"},{level:3,id:"4-the-cut-face",text:"4 · The cut face"},{level:3,id:"5-picking-for-free",text:"5 · Picking for free"},{level:3,id:"6-paying-nothing-when-off",text:"6 · Paying nothing when off"},{level:2,id:"the-answer-key",text:"The answer key"},{level:2,id:"acceptance-criteria-for-the-proposed-feature",text:"Acceptance criteria for the proposed feature"},{level:2,id:"if-you-want-more",text:"If you want more"}]};export{e as default};
