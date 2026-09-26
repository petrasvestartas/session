const e={title:"How to use this course",html:`<h1 id="how-to-use-this-course">How to use this course<a class="anchor" href="#/course/how-to-learn#how-to-use-this-course" aria-label="Link to this section">#</a></h1>
<p>The black triangle in the top-right corner switches between the viewer and these docs.</p>
<h2 id="each-step">Each step<a class="anchor" href="#/course/how-to-learn#each-step" aria-label="Link to this section">#</a></h2>
<ol>
<li>Read the one or two lines above the code: what the file is for.</li>
<li>Type the code.</li>
<li>Run <code>cargo check --lib</code> where the lesson says. Until then the build may be red: a file is often written across several steps. Every finished lesson is a crate in <code>docs/lessons/&lt;id&gt;/</code> to diff against when you are lost.</li>
<li>At the end of the lesson, build with <code>trunk serve --port 8780</code> and compare the page with the screenshot.</li>
</ol>
<h2 id="code-block-labels">Code block labels<a class="anchor" href="#/course/how-to-learn#code-block-labels" aria-label="Link to this section">#</a></h2>
<p>Every code block is preceded by one line naming the lesson crate file, the line range shown and what to do with it:</p>
<ul>
<li><strong>type this, replace the whole file / append at the end of the file / new file</strong> — write the shown lines by hand at that place.</li>
<li><strong>replace lines A–B of <code>lessons/&lt;parent&gt;/…</code></strong> — the shown lines take the place of that range in the previous lesson&#39;s file.</li>
<li><strong>delete lines A–B of <code>lessons/&lt;parent&gt;/…</code></strong> — remove them.</li>
<li><strong>copy the file</strong> — a fixture or page; copy it from the lesson crate.</li>
</ul>
<p>The whole file as it stands at the end of the lesson is in <code>docs/lessons/&lt;id&gt;/</code>, for comparing with yours.</p>
<p>A Rust file enters the build only when a <code>mod</code> line names it; the bigger lessons create files first and declare them at the end.</p>
<h2 id="stuck">Stuck<a class="anchor" href="#/course/how-to-learn#stuck" aria-label="Link to this section">#</a></h2>
<ul>
<li>Read the error. <a href="#/course/debugging">Reading failures</a> covers the ones this course produces.</li>
<li>Compare your file with the one in <code>docs/lessons/&lt;id&gt;/</code>.</li>
<li>A word you do not know: <a href="#/course/words">Words before code</a>.</li>
</ul>
`,toc:[{level:2,id:"each-step",text:"Each step"},{level:2,id:"code-block-labels",text:"Code block labels"},{level:2,id:"stuck",text:"Stuck"}]};export{e as default};
