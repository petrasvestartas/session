const e={title:"Small code, bounded work",html:`<h1 id="small-code-bounded-work">Small code, bounded work<a class="anchor" href="#/course/performance-patterns#small-code-bounded-work" aria-label="Link to this section">#</a></h1>
<h2 id="you-are-building">You are building<a class="anchor" href="#/course/performance-patterns#you-are-building" aria-label="Link to this section">#</a></h2>
<p>Use the same design for each extension:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>input → named State action → source or view state → changed GPU rows → frame</code></pre></div>
<p>A shorter function is useful when it also removes work or makes ownership clearer. Moving the same work into a generic framework is not a performance improvement.</p>
<h2 id="starting-point">Starting point<a class="anchor" href="#/course/performance-patterns#starting-point" aria-label="Link to this section">#</a></h2>
<p>Every lesson ends at a <strong>checkpoint</strong>, a complete crate under <code>docs/lessons/&lt;id&gt;/</code>; the code blocks are included from those crates. The course runs 00 to 37, from an empty project to the maintained viewer.</p>
<h2 id="step-1-name-when-the-work-runs">Step 1 · Name when the work runs<a class="anchor" href="#/course/performance-patterns#step-1-name-when-the-work-runs" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>When</th>
<th>Existing pattern</th>
<th>Keep out of this path</th>
</tr>
</thead>
<tbody><tr>
<td>Load or geometry edit</td>
<td><code>Scene</code> walks source once and uploads rows</td>
<td>Camera-dependent document copies</td>
</tr>
<tr>
<td>Scene structure changes</td>
<td><code>Hierarchy::refresh</code> checks <code>row_revision</code></td>
<td>Rebuilding a tree on expand or hide</td>
</tr>
<tr>
<td>Selection or visibility changes</td>
<td>One named action changes flags</td>
<td>A second hide set or undo cursor</td>
</tr>
<tr>
<td>Pointer moves</td>
<td>Preview the affected object/control</td>
<td>Kernel history transactions per event</td>
</tr>
<tr>
<td>Frame</td>
<td>Uniform writes, cached preparation, draw calls</td>
<td>Source tessellation or hierarchy construction</td>
</tr>
<tr>
<td>Scene release</td>
<td>Drop source handles and upload staging</td>
<td>Strong references held by observer caches</td>
</tr>
</tbody></table>
<p><strong>READ ONLY — <code>src/app/layers.rs</code>, <code>rows</code>.</strong> One walk counts all document and type buckets. For N objects and D documents, work is O(N + D), with D counters and six fixed type counters. Building each document&#39;s member list merely to count it costs O(D × N) and allocates temporary row vectors.</p>
<p><strong>READ ONLY — <code>src/app/hierarchy.rs</code>, <code>row_of</code>.</strong> The outer map chooses the document; the inner map accepts a borrowed <code>&amp;str</code>. Looking up a GUID does not need a fresh <code>Rc&lt;str&gt;</code>. Keep document identity in the key: different placements can share GUIDs.</p>
<h2 id="step-2-keep-one-mutation-path">Step 2 · Keep one mutation path<a class="anchor" href="#/course/performance-patterns#step-2-keep-one-mutation-path" aria-label="Link to this section">#</a></h2>
<p><strong>READ ONLY — <code>src/state/edit.rs</code>, <code>toggle_layer</code>, and <code>src/state/panel.rs</code>, <code>set_rows_hidden</code>.</strong> Both document/type controls and hierarchy controls use the same visibility writer. It updates <code>Scene.hidden</code>, writes only changed GPU flags, and clears selection only when hiding selected rows.</p>
<p>The target rows are sorted by <code>layers::of_layer</code> or <code>Hierarchy::targets</code>. That permits binary-search membership checks; a nested linear search can turn a large group action into quadratic work. If another caller supplies rows, preserve this contract.</p>
<p>Do not add an index merely to avoid a short scan. Add one when it avoids repeated expensive work, and name the event that invalidates it.</p>
<h2 id="step-3-decide-whether-capacity-stays">Step 3 · Decide whether capacity stays<a class="anchor" href="#/course/performance-patterns#step-3-decide-whether-capacity-stays" aria-label="Link to this section">#</a></h2>
<p><strong>READ ONLY — <code>src/engine/gpu/upload.rs</code>, <code>drop_rows</code>.</strong></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>*v = Vec::new();</code></pre></div>
<p>Assignment drops the old vector and its elements, then leaves an empty vector with no allocation. Upload staging is finished, so retaining capacity has no purpose here.</p>
<p>Use <code>clear()</code> for bounded scratch space that will be filled again. It keeps capacity. Drop or replace a vector when the owning scene or operation ends. Do not call <code>shrink_to_fit</code> every frame: that turns reuse into allocator work.</p>
<p>For a graph budget, check vertices against the remaining capacity <strong>before</strong> subtracting them to check edges. Saturating subtraction alone does not reject too many vertices when the edge count is zero. If indexing cannot finish a document, omit its entire index and report the limit; a partial group must never pretend to cover all descendants.</p>
<h2 id="step-4-change-the-teaching-source-once">Step 4 · Change the teaching source once<a class="anchor" href="#/course/performance-patterns#step-4-change-the-teaching-source-once" aria-label="Link to this section">#</a></h2>
<ul>
<li>Explain the principle once, then link later lessons to it.</li>
<li>Use the existing <strong>You are building → Starting point → Steps → Check → What changed → Try → Questions and answers</strong> structure.</li>
<li>A new lesson needs the complete wiring, its lifecycle cleanup and an observable check. “Copy the relevant methods” is an orientation guide, not a replayable lesson.</li>
<li>A lesson edit changes the crate in <code>docs/lessons/&lt;id&gt;/</code> and the line ranges its page references. A production-only improvement belongs in a clearly labelled supplement until its lesson crate carries it.</li>
<li>Keep limitations beside commands: supported geometry, coordinate units, selection scope and memory limits.</li>
</ul>
<h2 id="check">Check<a class="anchor" href="#/course/performance-patterns#check" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>cargo <span class="s">xtest</span> <span class="s">-j4</span> <span class="s">--lib</span>
cargo <span class="s">clippy</span> <span class="s">-j4</span> <span class="s">--lib</span> <span class="s">--</span> <span class="s">-D</span> <span class="s">warnings</span>
docs/serve.sh <span class="s">build</span> <span class="s">--quiet</span></code></pre></div>
<p><code>docs/serve.sh build</code> fails on a lesson snippet that points at a missing file; <code>cargo check</code> inside <code>docs/lessons/&lt;id&gt;/</code> proves that lesson compiles. Neither runs the browser. Native tests do not prove browser interaction; browser checks need a usable WebGPU adapter.</p>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/performance-patterns#what-changed" aria-label="Link to this section">#</a></h2>
<p>Review scope: course structure and runtime source, with focused review of ownership, upload release, frame preparation, editing, selection and panels. The integrated lesson replay matches all 114 maintained runtime/build files. This is not a proof of every shader or geometry algorithm.</p>
<p>Applied: one-pass bucket counts, borrowed hierarchy lookups, one visibility writer, corrected graph capacity checks, and direct staging-vector release. These remove identifiable loops or allocations; no frame-rate improvement is claimed without a benchmark.</p>
<p>Further work needs separate measurements: batch large flag updates into GPU writes and bound retained undo snapshots. The optional editing lessons are now individually replayable. Keep the existing lane and checkpoint machinery rather than introducing another framework.</p>
<h2 id="course-site-weight">Course site weight<a class="anchor" href="#/course/performance-patterns#course-site-weight" aria-label="Link to this section">#</a></h2>
<p>Most of the course is highlighted code, so a page weighs what the markup around each token weighs. Each change below was measured on the built site, in the order listed:</p>
<table>
<thead>
<tr>
<th>Change</th>
<th>Where</th>
<th>Before → after</th>
</tr>
</thead>
<tbody><tr>
<td>No per-line anchors or line spans</td>
<td><code>pymdownx.highlight</code> in <code>mkdocs.yml</code></td>
<td>all pages 23.95 → 17.06 MB of HTML</td>
</tr>
<tr>
<td>Whole kernel files on their own pages</td>
<td><code>docs/kernel/*.md</code></td>
<td><code>07-boundaries</code> 2.01 → 0.35 MB, search index 3.04 → 2.54 MB</td>
</tr>
<tr>
<td><code>.w</code> and <code>.n</code> tokens unwrapped between tags</td>
<td><code>docs/hooks/lean_html.py</code></td>
<td>all pages 16.94 → 12.53 MB</td>
</tr>
<tr>
<td>PNG screenshots sized, lazy unless first with no code above</td>
<td><code>docs/hooks/lean_html.py</code></td>
<td><code>command-line-walkthrough</code> fetches 450 → 133 KB of images before scrolling; lessons 22-32 no longer fetch their end-of-page screenshot (18-91 KB)</td>
</tr>
<tr>
<td>Build tools and sources not published</td>
<td><code>exclude_docs</code> in <code>mkdocs.yml</code></td>
<td>226 fewer files, 687 → 461</td>
</tr>
</tbody></table>
<p>With the CPU slowed 4x in Chrome, <code>12-picking</code> (47,534 → 23,848 elements) reaches DOMContentLoaded in 790 ms instead of 1,862 ms, and <code>07-boundaries</code> in 332 ms instead of 1,698 ms. The rendered pages stayed pixel-identical and the copy button copies the same text.</p>
<ul>
<li>A whole kernel file is a page under <code>docs/kernel/</code> with <code>search: exclude: true</code>, linked from the lesson; never include it inline.</li>
<li>The hook unwraps <code>.n</code> only because Material paints it in the plain code colour: if <code>--md-code-hl-name-color</code> is ever themed, stop unwrapping <code>.n</code>. A token next to plain text keeps its span, because one merged text run moves the glyphs after it by 1/64 px.</li>
<li>SVGs keep no size attribute and load eagerly: a size on a scaled SVG changes how Chrome rasterises it, and an unsized lazy image would move an anchor target.</li>
<li>Measured and left out: <code>navigation.instant</code> (the largest pages show their heading later, and a local build downloads every stylesheet twice) and a Roboto preload (first paint 0.1-0.2 s later on a slow connection).</li>
</ul>
<h2 id="try">Try<a class="anchor" href="#/course/performance-patterns#try" aria-label="Link to this section">#</a></h2>
<p>On a large local scene, open the panel and repeatedly hide/show a group. Index storage should stay stable until the scene structure changes. Compare a document with many small groups against one flat group; total bucket-counting work should depend on object count, not object count multiplied by group count.</p>
<h2 id="questions-and-answers">Questions and answers<a class="anchor" href="#/course/performance-patterns#questions-and-answers" aria-label="Link to this section">#</a></h2>
<p><strong>Why keep explicit loops?</strong> Their bounds and allocations are visible. Use an iterator when it avoids a temporary collection or expresses the same operation more clearly.</p>
<p><strong>Why not rewrite every old lesson?</strong> Its checkpoint is reproducible. Change a lesson when teaching or measured behavior improves, then regenerate and verify that checkpoint.</p>
<p><strong>Does a memory limit prevent every crash?</strong> No. It bounds the resource it names. Source geometry, history, GPU allocations and browser overhead need separate accounting.</p>
`,toc:[{level:2,id:"you-are-building",text:"You are building"},{level:2,id:"starting-point",text:"Starting point"},{level:2,id:"step-1-name-when-the-work-runs",text:"Step 1 · Name when the work runs"},{level:2,id:"step-2-keep-one-mutation-path",text:"Step 2 · Keep one mutation path"},{level:2,id:"step-3-decide-whether-capacity-stays",text:"Step 3 · Decide whether capacity stays"},{level:2,id:"step-4-change-the-teaching-source-once",text:"Step 4 · Change the teaching source once"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"course-site-weight",text:"Course site weight"},{level:2,id:"try",text:"Try"},{level:2,id:"questions-and-answers",text:"Questions and answers"}]};export{e as default};
