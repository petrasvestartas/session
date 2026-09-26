const s={title:"Build the Session Viewer",html:`<h1 id="build-the-session-viewer">Build the Session Viewer<a class="anchor" href="#/course/readme#build-the-session-viewer" aria-label="Link to this section">#</a></h1>
<p>You type a CAD viewer in Rust, WebAssembly and WebGPU from an empty folder, one file at a time. Every lesson ends with something you can see in the browser.</p>
<h2 id="install-on-a-fresh-computer">Install on a fresh computer<a class="anchor" href="#/course/readme#install-on-a-fresh-computer" aria-label="Link to this section">#</a></h2>
<p>Linux, in a terminal:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c"># 1. compiler tools, curl, git</span>
sudo <span class="s">apt</span> <span class="s">install</span> <span class="s">build-essential</span> <span class="s">curl</span> <span class="s">git</span>

<span class="c"># 2. Rust, pinned to the version the course was verified with</span>
curl <span class="s">https://sh.rustup.rs</span> <span class="s">-sSf</span> | sh
rustup <span class="s">toolchain</span> <span class="s">install</span> <span class="s">1.97.1</span>
rustup <span class="s">default</span> <span class="s">1.97.1</span>
rustup <span class="s">target</span> <span class="s">add</span> <span class="s">wasm32-unknown-unknown</span>

<span class="c"># 3. Trunk: builds the page and serves it</span>
cargo <span class="s">install</span> <span class="s">trunk</span> <span class="s">--version</span> <span class="s">0.21.14</span> <span class="s">--locked</span>

<span class="c"># 4. the kernel the viewer draws</span>
git <span class="s">clone</span> <span class="s">--recurse-submodules</span> <span class="s">https://github.com/petrasvestartas/session.git</span></code></pre></div>
<ol start="5">
<li>Chrome, for WebGPU.</li>
</ol>
<p>Check:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>rustc <span class="s">--version</span>      <span class="c"># rustc 1.97.1</span>
trunk <span class="s">--version</span>      <span class="c"># trunk 0.21.14</span>
ls <span class="s">session</span>           <span class="c"># session_rust  session_cpp  session_py  session_proto ...</span></code></pre></div>
<h2 id="where-you-type">Where you type<a class="anchor" href="#/course/readme#where-you-type" aria-label="Link to this section">#</a></h2>
<p>Your crate goes inside <code>session</code>, next to <code>session_rust</code>. Name the folder <code>session_view</code>.</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>session/
├── session_rust/
├── session_cpp/  session_py/  session_proto/
└── session_view/        ← lesson 00 creates this; every command runs inside it</code></pre></div>
<h2 id="run">Run<a class="anchor" href="#/course/readme#run" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>cargo <span class="s">check</span> <span class="s">--lib</span>
trunk <span class="s">serve</span> <span class="s">--port</span> <span class="s">8780</span></code></pre></div>
<p>Open <a href="http://localhost:8780/" target="_blank" rel="noopener">http://localhost:8780/</a>. Stop with Ctrl+C.</p>
<h2 id="lessons">Lessons<a class="anchor" href="#/course/readme#lessons" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Lesson</th>
<th>You type</th>
<th>See</th>
</tr>
</thead>
<tbody><tr>
<td><a href="#/course/00-environment">00 · Empty project to WASM</a></td>
<td>Crate, wasm32 default, Trunk, a status message</td>
<td>a line of text</td>
</tr>
<tr>
<td><a href="#/course/01-first-frame">01 · First WebGPU frame</a></td>
<td>Adapter, device, surface, pipeline</td>
<td>one triangle</td>
</tr>
<tr>
<td><a href="#/course/02-camera">02 · Camera</a></td>
<td>Orbit, pan, cursor zoom, reversed depth</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/03-identity">03 · Object rows and identity</a></td>
<td><code>Instance</code> rows, storage bind group</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/04a-meshes">04a · Meshes on the GPU</a></td>
<td>Arena buffers, object table, <code>triangle.wgsl</code></td>
<td></td>
</tr>
<tr>
<td><a href="#/course/04b-strokes">04b · Strokes</a></td>
<td>Segment lane, screen-space ribbon shader</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/04c-markers">04c · Markers</a></td>
<td>Vertex markers, free dots</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/04d-clouds">04d · Point clouds</a></td>
<td>LOD nodes, splats</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/05-visibility">05 · Depth and visible ink</a></td>
<td>Reversed depth, ink visibility</td>
<td>camera + meshes</td>
</tr>
<tr>
<td><a href="#/course/06-cad-contract">06 · CAD face contract</a></td>
<td>Face meshes, UVs, normals</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/07-boundaries">07 · Shared boundaries</a></td>
<td>One chain per edge, constrained meshing</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/08-trimming">08 · Trims and seams</a></td>
<td>Holes, seams, poles</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/09-normals">09 · Normals and shading</a></td>
<td>Analytic normals, creases</td>
<td>CAD visualization</td>
</tr>
<tr>
<td><a href="#/course/10-text-layout">10 · Text shaping</a></td>
<td>Fonts, shaping, clusters</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/11-text-rendering">11 · Text rendering</a></td>
<td>Placement, coverage, plates</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/12-picking">12 · Production shell and picking</a></td>
<td>winit <code>App</code>, <code>State</code>, ID pass, selection</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/13-controls">13 · Source controls</a></td>
<td>F10 controls, cloud queries</td>
<td>picking + interaction</td>
</tr>
<tr>
<td><a href="#/course/14-loading">14 · Loading scenes</a></td>
<td>Manifests, validation</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/15-publication">15 · Publication and streamed reads</a></td>
<td>Revisions, metadata window</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/16-accounting">16 · Resource accounting</a></td>
<td>Source cache, capacity numbers</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/17-source-presentation">17 · Faces, text objects, silhouettes</a></td>
<td>Source faces, selectable text, outlines</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/18-finite-visibility">18 · Finite-triangle visibility</a></td>
<td>Projected triangles, tile lists</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/19-sheets">19 · Sheets</a></td>
<td>Batched drawings, ranged slices</td>
<td></td>
</tr>
<tr>
<td><a href="#/course/20-history">20 · The document</a></td>
<td>Transactions, undo and redo</td>
<td>full viewer</td>
</tr>
<tr>
<td><a href="#/course/21-editing">21 · Editing</a></td>
<td>Gumball, snapping, construction plane, command line, layers</td>
<td></td>
</tr>
</tbody></table>
<p>Do them in order. <a href="#/course/how-to-learn">How to use this course</a> explains the code-block labels; <a href="#/course/debugging">Reading failures</a> explains the errors.</p>
`,toc:[{level:2,id:"install-on-a-fresh-computer",text:"Install on a fresh computer"},{level:2,id:"where-you-type",text:"Where you type"},{level:2,id:"run",text:"Run"},{level:2,id:"lessons",text:"Lessons"}]};export{s as default};
