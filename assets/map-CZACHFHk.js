const e={title:"The map",html:`<h1 id="the-map">The map<a class="anchor" href="#/course/map#the-map" aria-label="Link to this section">#</a></h1>
<p>Learn this one picture and the rest of the course has an address.</p>
<p><img src="/session/docs/course/docs/illustrations/map.svg" alt="The whole viewer as one map: the top row is how documents come in, the bottom row is how a frame is drawn, and a pick answer travels back up." loading="lazy" decoding="async"></p>
<p>From lesson 01 on, every step that touches a file opens with this map, one box filled pink: where the code on the page lives. Lesson 00 builds only the first box, so it shows no map.</p>
<p>A compressed copy stays <strong>pinned at the top of the page</strong>, following you from step to step and from code block to code block.</p>
<p>Three fills, and nothing else on the map ever moves:</p>
<ul>
<li><strong>pink</strong> — the code on this page lives here</li>
<li><strong>light slate</strong> — you have already built it</li>
<li><strong>near-black</strong> — still ahead of you</li>
</ul>
<h2 id="the-two-paths">The two paths<a class="anchor" href="#/course/map#the-two-paths" aria-label="Link to this section">#</a></h2>
<p>Every file serves one of two journeys.</p>
<p><strong>Documents come in along the top row.</strong> A file arrives over the network, decodes into kernel documents holding exact f64 geometry, and the walk turns those into rows — plain arrays with no wgpu types. It runs when a scene loads, then stops.</p>
<p><strong>A frame is drawn along the bottom row.</strong> The browser asks → the shell routes → input may have moved the camera → state decides what shows → the GPU core writes the per-frame uniforms and hands each lane its turn → lanes record draw calls → shaders make pixels. On demand, without tessellating source geometry again.</p>
<p>They meet once: the walk&#39;s rows are uploaded, and the frame path reads only those. That junction keeps the frame path separate from geometry reconstruction; changing a camera does not re-walk the source.</p>
<p><strong>One arrow goes backwards.</strong> A pick: the lanes draw object ids into a small offscreen window, the answer is read back, and <code>Scene</code> turns a row number into the document object it came from. Two flows go further than the arrow shows — picking a streamed cloud point or a sheet entity continues left into the network, because the identity was never on this machine; and the tile pool reads its own size report back a frame later, never reaching the scene. Everything else points downward.</p>
<h2 id="the-zones-one-line-each">The zones, one line each<a class="anchor" href="#/course/map#the-zones-one-line-each" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Zone</th>
<th>What lives there</th>
<th>Why it is separate</th>
</tr>
</thead>
<tbody><tr>
<td><strong>Page</strong></td>
<td><code>index.html</code>, <code>Trunk.toml</code>, <code>Cargo.toml</code></td>
<td>The browser&#39;s side of the contract: what gets loaded before any Rust runs.</td>
</tr>
<tr>
<td><strong>Network</strong></td>
<td><code>fetch</code>, <code>manifest</code>, <code>validate</code>, <code>decode</code>, <code>stream</code>, <code>live</code>, <code>route</code>, <code>loader</code>, <code>cloud_query</code>, <code>sheet_query</code></td>
<td>The only code that touches bytes you did not create. Incoming file and manifest validation happens here.</td>
</tr>
<tr>
<td><strong>Kernel</strong></td>
<td><code>session_rust</code></td>
<td>Shared with the C++ and Python kernels: exact f64 geometry and identity, plus the one shared display type, <code>RenderVertex</code>. It links wgpu for that and for the GPU buffers a <code>Mesh</code> caches, and decides nothing about how the viewer draws.</td>
</tr>
<tr>
<td><strong>Scene + walk</strong></td>
<td><code>app/scene.rs</code>, <code>app/scene_text.rs</code>, <code>app/selection.rs</code>, <code>app/walk/*</code>, <code>engine/text.rs</code></td>
<td>Turns one document into rows and names what can be selected. No producer in <code>walk/</code> knows about files, selection or the camera; <code>Scene</code> holds the documents and their placements and hands finished rows to the GPU.</td>
</tr>
<tr>
<td><strong>Shell</strong></td>
<td><code>lib.rs</code>, <code>app/mod.rs</code>, <code>app/feedback.rs</code>, <code>app/inspection*</code>, <code>app/knobs.rs</code>, <code>selftest*</code>, <code>text_quality.rs</code>, <code>engine/performance.rs</code></td>
<td>The window, the event loop, the one place a redraw is asked for, and the measurements that observe a frame without changing it.</td>
</tr>
<tr>
<td><strong>Input</strong></td>
<td><code>app/input.rs</code>, <code>app/touch.rs</code></td>
<td>Gestures become intentions. It never touches a buffer or names a wgpu type: scene changes go through <code>State</code>. It does flip the view knobs (<code>view.lit</code>, <code>show_outlines</code>) and read the device scale directly — per-frame view state, not scene state.</td>
</tr>
<tr>
<td><strong>State</strong></td>
<td><code>state.rs</code>, <code>camera.rs</code></td>
<td>Camera and selection transitions, and the demand for the next frame.</td>
</tr>
<tr>
<td><strong>GPU core</strong></td>
<td><code>device</code>, <code>present</code>, <code>render</code>, <code>frame</code>, <code>objects</code>, <code>targets</code>, <code>buffers</code>, <code>instance</code>, <code>upload</code>, <code>view</code>, <code>pipelines/</code></td>
<td>One device, one set of uniforms, one list of passes, one row type, one growable buffer. Everything a lane needs but no lane should own.</td>
</tr>
<tr>
<td><strong>Lanes</strong></td>
<td><code>arena</code>, <code>faces</code>, <code>segments</code>, <code>glyphs</code>, <code>cloud</code>, <code>splat</code>, <code>lod</code>, <code>text*</code>, <code>surface_outline</code>, <code>backdrop</code>, <code>triangle_tiles</code>, <code>pick</code></td>
<td>One per kind of thing drawn, so a new primitive is an addition rather than an edit. Most lanes ignore each other; the exceptions are deliberate and few — outline text borrows the arena&#39;s buffers rather than copying the geometry, the splat lane reads the cloud tables and the LOD walk it draws from, and the ink shader reads the projected triangles and the tile pool the arena&#39;s tile lane fills.</td>
</tr>
<tr>
<td><strong>Shaders</strong></td>
<td><code>src/shaders/*.wgsl</code></td>
<td>The code that runs on the GPU, compiled against the scene contract in <code>scene.wgsl</code>.</td>
</tr>
<tr>
<td><strong>Pixels</strong></td>
<td>the canvas</td>
<td>Where it all ends up.</td>
</tr>
</tbody></table>
<h2 id="how-to-use-it-while-you-type">How to use it while you type<a class="anchor" href="#/course/map#how-to-use-it-while-you-type" aria-label="Link to this section">#</a></h2>
<p><strong>Lanes</strong> lit: you are adding a way to draw something. Ask what rows it reads and which shader it feeds.</p>
<p><strong>GPU core</strong> lit: you are changing something <em>every</em> lane sees — a uniform field, a pass, a pipeline rule. These need the three declarations to agree; expect a validation error if you miss one.</p>
<p><strong>Scene + walk</strong> lit: you are deciding what a document <em>becomes</em>. Nothing here can see the camera; wanting to is the design telling you the work belongs one row down.</p>
<p>Two zones lit: the change crosses a boundary — a file written across several steps, with the build red in between; the lesson says where to run <code>cargo check</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/map#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/00-environment">00 · Empty project to a WASM message</a>: the first box on the map, and the only one you can build without a GPU.</p>
`,toc:[{level:2,id:"the-two-paths",text:"The two paths"},{level:2,id:"the-zones-one-line-each",text:"The zones, one line each"},{level:2,id:"how-to-use-it-while-you-type",text:"How to use it while you type"},{level:2,id:"next",text:"Next"}]};export{e as default};
