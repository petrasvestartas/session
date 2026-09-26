const e={title:"Maintaining this site",html:`<h1 id="maintaining-this-site">Maintaining this site<a class="anchor" href="#/course/maintaining#maintaining-this-site" aria-label="Link to this section">#</a></h1>
<p>Not part of the course. Build and check:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>docs/serve.sh <span class="s">build</span>
docs/serve.sh</code></pre></div>
<ul>
<li>Each lesson is a runnable crate in <code>docs/lessons/&lt;id&gt;/</code>; <code>docs/lessons/SERIES.txt</code> lists every id, the id it builds on and the page that teaches it. Edit the crate and the page directly.</li>
<li>Lesson code never lives in the Markdown. A step names a file in <code>docs/lessons/&lt;id&gt;/</code> and a line range; the page includes those lines with a <code>--8&lt;--</code> snippet, so editing the lesson crate edits the page. When an edit moves lines, update the range in the page by hand.</li>
<li><code>docs/serve.sh build</code> fails on a snippet path that does not exist; that is the only site check. <code>cargo check</code> inside a lesson directory proves the lesson compiles.</li>
<li><code>diagrams.py</code> renders the D2 flowcharts; <code>check_svg.py</code> and <code>check_illustrations.cjs</code> check the illustrations; <code>theme.py</code> renders <code>theme.css</code>.</li>
</ul>
`,toc:[]};export{e as default};
