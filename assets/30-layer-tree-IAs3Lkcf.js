const s={title:"30 · Keep source dragging live and build one layer tree",html:`<h1 id="30-keep-source-dragging-live-and-build-one-layer-tree">30 · Keep source dragging live and build one layer tree<a class="anchor" href="#/course/30-layer-tree#30-keep-source-dragging-live-and-build-one-layer-tree" aria-label="Link to this section">#</a></h1>
<p>One layer tree controls visibility, locking and color while shell dragging updates retained surface samples.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/30-layer-tree#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>Show command syntax and a usable example while the reader types.</p>
<p><code>lessons/30/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Escape,</code> in <code>lessons/29/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Help text for the verb being typed.</span>
<span class="k">pub</span> <span class="k">fn</span> hint(line: &amp;str) -&gt; &amp;'static str {
    <span class="k">match</span> line
        .split_whitespace()
        .next()
        .unwrap_or(&quot;&quot;)
        .to_ascii_lowercase()
        .as_str()
    {
        &quot;<span class="s">point</span>&quot; =&gt; &quot;<span class="s">Point x,y,z · Example: Point 0,0,0 · Enter creates the point</span>&quot;,
        &quot;<span class="s">line</span>&quot; =&gt; &quot;<span class="s">Line start end · Example: Line 0,0,0 100,0,0</span>&quot;,
        &quot;<span class="s">polyline</span>&quot; =&gt; &quot;<span class="s">Polyline points… · Example: Polyline 0,0,0 100,0,0 100,100,0</span>&quot;,
        &quot;<span class="s">move</span>&quot; | &quot;<span class="s">m</span>&quot; =&gt; &quot;<span class="s">Select an object, then Move dx,dy,dz · Example: Move 10,0,0</span>&quot;,
        &quot;<span class="s">rotate</span>&quot; | &quot;<span class="s">rot</span>&quot; =&gt; &quot;<span class="s">Select an object, then Rotate axis degrees · Example: Rotate z 45</span>&quot;,
        &quot;<span class="s">scale</span>&quot; | &quot;<span class="s">s</span>&quot; =&gt; &quot;<span class="s">Select an object, then Scale factor · Example: Scale 2</span>&quot;,
        &quot;<span class="s">trim</span>&quot; =&gt; &quot;<span class="s">Select a line or curve · Trim 0.2 0.8 keeps that part of its length/domain</span>&quot;,
        &quot;<span class="s">extend</span>&quot; =&gt; &quot;<span class="s">Select a line or curve · Extend -0.2 1.2 extends its domain at both ends</span>&quot;,
        &quot;<span class="s">explode</span>&quot; =&gt; &quot;<span class="s">Select a polyline · Explode creates its individual line segments</span>&quot;,
        &quot;<span class="s">save</span>&quot; =&gt; &quot;<span class="s">Save downloads the complete editable scene as a .session file</span>&quot;,
        &quot;<span class="s">open</span>&quot; =&gt; &quot;<span class="s">Open restores a saved .session file</span>&quot;,
        &quot;<span class="s">fit</span>&quot; =&gt; &quot;<span class="s">Fit zooms to the selection, or the whole scene when nothing is selected</span>&quot;,
        _ =&gt; &quot;<span class="s">Try Point 0,0,0 · Line 0,0,0 100,0,0 · Fit · Undo · Save · Enter or Run executes</span>&quot;,
    }
}

<span class="c">/// Parse one line; the error is the message to show.</span></code></pre></div>
<h2 id="step-2-srcappdeformrs">Step 2 · src/app/deform.rs<a class="anchor" href="#/course/30-layer-tree#step-2-srcappdeformrs" aria-label="Link to this section">#</a></h2>
<p>Validate changed surface boundaries before accepting a shell deformation.</p>
<p><code>lessons/30/src/app/deform.rs</code> · edit · type this</p>
<p>Replaces the line <code>for curve in &amp;mut next.m_curves_3d {</code> in <code>lessons/29/src/app/deform.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> <span class="k">mut</span> changed_curves = HashSet::new(); <span class="c">// curves touched</span>
            <span class="k">let</span> <span class="k">mut</span> changed_surfaces = HashSet::new(); <span class="c">// surfaces touched</span>

            <span class="k">for</span> (curve_index, curve) <span class="k">in</span> next.m_curves_3d.iter_mut().enumerate() {
                <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..curve.cv_count() {
                    <span class="k">if</span> <span class="k">let</span> Some(p) = curve.get_cv(i)
                        &amp;&amp; matches(&amp;p)
                    {
                        curve.set_cv_point(i, &amp;p.transformed(delta));
                        changed_curves.insert(curve_index);
                    }
                }
            }

            <span class="k">for</span> (surface_index, surface) <span class="k">in</span> next.m_surfaces.iter_mut().enumerate() {
                <span class="k">for</span> u <span class="k">in</span> <span class="s">0</span>..surface.m_cv_count[<span class="s">0</span>] {
                    <span class="k">for</span> v <span class="k">in</span> <span class="s">0</span>..surface.m_cv_count[<span class="s">1</span>] {
                        <span class="k">if</span> <span class="k">let</span> Some(p) = surface.get_cv(u, v)
                            &amp;&amp; matches(&amp;p)
                        {
                            surface.set_cv(u, v, &amp;p.transformed(delta));
                            changed_surfaces.insert(surface_index);
                        }
                    }
                }

                <span class="k">if</span> changed_surfaces.contains(&amp;surface_index) {
                    surface.m_mesh = None; <span class="c">// drop the cached mesh</span>
                }
            }

            validate_boundaries(&amp;next, &amp;changed_curves, &amp;changed_surfaces)?;</code></pre></div>
<p>Replaces the line <code>fn validate_boundaries(brep: &amp;session_rust::BRep) -&gt; Resu…</code> in <code>lessons/29/src/app/deform.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Refuse an edit whose edges no longer lie on their surfaces.</span>
<span class="k">fn</span> validate_boundaries(
    brep: &amp;session_rust::BRep,
    changed_curves: &amp;HashSet&lt;usize&gt;,
    changed_surfaces: &amp;HashSet&lt;usize&gt;,
) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> !brep.is_valid() {
        <span class="k">return</span> Err(&quot;<span class="s">Edit would invalidate BRep topology</span>&quot;.into());
    }

    <span class="k">for</span> edge <span class="k">in</span> &amp;brep.m_edges {
        <span class="k">if</span> edge.degenerated
            || (!changed_curves.contains(&amp;(edge.curve_3d_index <span class="k">as</span> usize))
                &amp;&amp; !edge
                    .pcurves
                    .iter()
                    .any(|pc| changed_surfaces.contains(&amp;(pc.surface_index <span class="k">as</span> usize))))
        {</code></pre></div>
<p>Replaces the line <code>validate_boundaries(&amp;next).unwrap();</code> in <code>lessons/29/src/app/deform.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        validate_boundaries(
            &amp;next,
            &amp;(<span class="s">0</span>..next.m_curves_3d.len()).collect(),
            &amp;(<span class="s">0</span>..next.m_surfaces.len()).collect(),
        )
        .unwrap();</code></pre></div>
<h2 id="step-3-srcappeditrs">Step 3 · src/app/edit.rs<a class="anchor" href="#/course/30-layer-tree#step-3-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Try an in-place source preview before falling back to a complete geometry rebuild.</p>
<p><code>lessons/30/src/app/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>let (doc, guid) = self.writable(row).ok_or(&quot;Source is not…</code> in <code>lessons/29/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">self</span>.patch_preview(row, &amp;geometry, gpu) {
            <span class="k">return</span> Ok(()); <span class="c">// fast path: only vertices moved</span>
        }

        <span class="c">// swap in, rebuild the rows, swap back</span></code></pre></div>
<h2 id="step-4-srcappfeedbackrs">Step 4 · src/app/feedback.rs<a class="anchor" href="#/course/30-layer-tree#step-4-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Carry status and layer information from the scene into the interface.</p>
<p><code>lessons/30/src/app/feedback.rs</code> · edit · type this</p>
<p>Replaces the line <code>#[derive(Clone)]</code> in <code>lessons/29/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One row of the layers panel, as the panel needs it.</span>
#[derive(Clone, Default)]
<span class="k">pub</span> <span class="k">struct</span> LayerRow {
    <span class="k">pub</span> key: String,                 <span class="c">// unique id of the row</span>
    <span class="k">pub</span> label: String,               <span class="c">// text shown</span>
    <span class="k">pub</span> count: usize,                <span class="c">// objects under it</span>
    <span class="k">pub</span> hidden: bool,                <span class="c">// eye toggled off</span>
    <span class="k">pub</span> locked: bool,                <span class="c">// not editable</span>
    <span class="k">pub</span> color: Option&lt;[u8; 3]&gt;,      <span class="c">// face colour swatch</span>
    <span class="k">pub</span> depth: usize,                <span class="c">// indent level</span>
    <span class="k">pub</span> expanded: Option&lt;bool&gt;,      <span class="c">// open, closed or no children</span></code></pre></div>
<h2 id="step-5-srcapphierarchyrs">Step 5 · src/app/hierarchy.rs<a class="anchor" href="#/course/30-layer-tree#step-5-srcapphierarchyrs" aria-label="Link to this section">#</a></h2>
<p>Index document trees and graph endpoints into bounded sets of render rows.</p>
<p><code>lessons/30/src/app/hierarchy.rs</code> · edit · type this</p>
<p>Added after the line <code>use crate::app::scene::Scene;</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">use</span> session_rust::Session;
#[cfg(test)]</code></pre></div>
<p>Replaces the line <code>for (doc, file) in scene.docs.iter().enumerate() {</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">for</span> doc <span class="k">in</span> <span class="s">0</span>..scene.docs.len() {
            <span class="k">let</span> start = <span class="k">self</span>.nodes.len();
            <span class="k">let</span> rows = <span class="k">self</span>.rows.len();

            <span class="k">if</span> !<span class="k">self</span>.tree(scene, doc, &amp;lookup) {</code></pre></div>
<p>Added after the line <code>let mut seen = HashSet::new();</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> seen_rows = HashSet::new(); <span class="c">// rows placed</span>
        <span class="k">let</span> <span class="k">mut</span> stack = Vec::new(); <span class="c">// (node, depth, index to close)</span>

        <span class="k">if</span> <span class="k">let</span> Some(root) = file.session.tree.root() {
            <span class="k">if</span> root.borrow().name == file.name {
                <span class="c">// the document line stands for the root</span>
                <span class="k">for</span> child <span class="k">in</span> root.borrow().children().into_iter().rev() {
                    stack.push((child, <span class="s">1</span>, None));
                }
            } <span class="k">else</span> {
                stack.push((root, <span class="s">1</span>, None));
            }</code></pre></div>
<p>Replaces the line <code>if let Some(row) = row {</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(row) = row
                &amp;&amp; seen_rows.insert(row)
            {</code></pre></div>
<p>Replaces the lines from <code>if self.rows.len() == self.nodes[start].rows.start {</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// objects outside the tree go under the document</span>
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
            <span class="k">if</span> scene
                .identity_of(row)
                .is_some_and(|(owner, _)| owner == doc)
                &amp;&amp; seen_rows.insert(row)
            {
                <span class="k">let</span> index = <span class="k">self</span>.nodes.len();

                <span class="k">if</span> !<span class="k">self</span>.push(scene.object_name(row), <span class="s">1</span>) {
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">self</span>.rows.push(row);
                <span class="k">self</span>.finish(index);</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Add the graph's vertices and edges as groups.</span>
    #[cfg(test)]</code></pre></div>
<p>Added after the line <code>use session_rust::Point;</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(test)]</code></pre></div>
<p>Added after the line <code>assert_eq!(index.targets(child), vec![0]);</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> tree_count = index.rows.len();
        <span class="k">let</span> <span class="k">mut</span> lookup = Lookup::new();

        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
            <span class="k">let</span> (doc, id) = scene.identity_of(row).unwrap();
            lookup.entry(doc).or_default().insert(id, row);
        }

        assert!(index.graph(&amp;shared, <span class="s">0</span>, &quot;<span class="s">first</span>&quot;, &amp;lookup));</code></pre></div>
<p>Replaces the line <code>assert_eq!(index.rows.len(), count);</code> in <code>lessons/29/src/app/hierarchy.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(index.rows.len(), tree_count);</code></pre></div>
<h2 id="step-6-srcappinspectionrs">Step 6 · src/app/inspection.rs<a class="anchor" href="#/course/30-layer-tree#step-6-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Expose the new selection and resource state to the browser inspection data.</p>
<p><code>lessons/30/src/app/inspection.rs</code> · edit · type this</p>
<p>Replaces the line <code>let snapshot = serde_json::json!({</code> in <code>lessons/29/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> snapshot = serde_json::json!({</code></pre></div>
<p>Added after the line <code>});</code> in <code>lessons/29/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    snapshot[&quot;<span class="s">locked_count</span>&quot;] = serde_json::json!(state.scene.locked.len());
    snapshot[&quot;<span class="s">color_count</span>&quot;] = serde_json::json!(state.scene.colors.len());
    snapshot[&quot;<span class="s">scene_revision</span>&quot;] = serde_json::json!(state.scene.row_revision);
    snapshot[&quot;<span class="s">preview_cache_bytes</span>&quot;] = serde_json::json!(state.scene.preview_cache_bytes());</code></pre></div>
<h2 id="step-7-srcappmodrs">Step 7 · src/app/mod.rs<a class="anchor" href="#/course/30-layer-tree#step-7-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/30/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod ui;</code> in <code>lessons/29/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> surface_preview;</code></pre></div>
<h2 id="step-8-srcappsceners">Step 8 · src/app/scene.rs<a class="anchor" href="#/course/30-layer-tree#step-8-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Retain per-object upload ranges, surface preview samples, locks and color overrides.</p>
<p><code>lessons/30/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the line <code>pub hidden: HashSet&lt;(usize, Rc&lt;str&gt;)&gt;,</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> locked: HashSet&lt;(usize, Rc&lt;str&gt;)&gt;,             <span class="c">// (document, guid) not selectable</span>
    <span class="k">pub</span> colors: HashMap&lt;(usize, Rc&lt;str&gt;), [u8; 3]&gt;,    <span class="c">// face colour overrides</span></code></pre></div>
<p>Added after the line <code>bases: Bases,</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    uploaded: <span class="k">crate</span>::engine::gpu::patch::Counts,       <span class="c">// GPU row counts so far</span>
    surface_previews: Vec&lt;Option&lt;<span class="k">crate</span>::app::surface_preview::SurfacePreview&gt;&gt;, <span class="c">// per row, for live surface edits</span>
    <span class="k">pub</span>(super) preview_spans: Vec&lt;Option&lt;<span class="k">crate</span>::engine::gpu::patch::Span&gt;&gt;, <span class="c">// per row, its GPU range</span></code></pre></div>
<p>Added after the line <code>impl Scene {</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// True when the row is not locked.</span>
    <span class="k">pub</span> <span class="k">fn</span> selectable(&amp;<span class="k">self</span>, row: u32) -&gt; bool {
        <span class="k">self</span>.identity_of(row)
            .is_some_and(|id| !<span class="k">self</span>.locked.contains(&amp;id))
    }

    <span class="c">/// Empty: no documents, no rows.</span></code></pre></div>
<p>Added after the line <code>hidden: HashSet::new(),</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            locked: HashSet::new(),
            colors: HashMap::new(),</code></pre></div>
<p>Added after the line <code>bases: Bases::default(),</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            uploaded: Default::default(),
            preview_spans: Vec::new(),
            surface_previews: Vec::new(),</code></pre></div>
<p>Added after the line <code>self.hidden.clear();</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.locked.clear();
        <span class="k">self</span>.colors.clear();</code></pre></div>
<p>Added after the line <code>self.bases = Bases::default();</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.uploaded = Default::default();
        <span class="k">self</span>.preview_spans.clear();
        <span class="k">self</span>.surface_previews.clear();</code></pre></div>
<p>Added after the line <code>self.bases.ribbon += self.tables.seg.ribbons.len() as u32;</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.uploaded = <span class="k">self</span>
            .uploaded
            .plus(<span class="k">crate</span>::engine::gpu::patch::Counts::of(&amp;<span class="k">self</span>.tables));</code></pre></div>
<p>Added after the line <code>self.tables.obj.rows.push(ObjectRow::new(place, flags));</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(color) = <span class="k">self</span>.colors.get(&amp;(owner, Rc::from(guid))) {
            <span class="k">let</span> object = <span class="k">self</span>.tables.obj.rows.last_mut().expect(&quot;<span class="s">row just appended</span>&quot;);
            object.color = [
                color[<span class="s">0</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                color[<span class="s">1</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                color[<span class="s">2</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                <span class="s">1</span>.,
            ];
            object.flags |= Instance::FLAG_COLOR;
        }

        <span class="k">let</span> guid: Rc&lt;str&gt; = Rc::from(guid);
        <span class="k">self</span>.guid_to_row.insert((owner, Rc::clone(&amp;guid)), row);
        <span class="k">self</span>.order.push(guid);
        <span class="k">self</span>.owners.push(owner);
        <span class="k">self</span>.ribbon_ranges.push(None);
        <span class="k">self</span>.preview_spans.push(None);
        <span class="k">self</span>.surface_previews.push(None);</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> start = <span class="k">self</span>
                .uploaded
                .plus(<span class="k">crate</span>::engine::gpu::patch::Counts::of(&amp;<span class="k">self</span>.tables));
            <span class="k">let</span> r = walk_geometry(&amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> <span class="k">self</span>.tables), &amp;cx, geom);
            <span class="k">let</span> end = <span class="k">self</span>
                .uploaded
                .plus(<span class="k">crate</span>::engine::gpu::patch::Counts::of(&amp;<span class="k">self</span>.tables));
            <span class="k">let</span> span = <span class="k">crate</span>::engine::gpu::patch::Span {
                start,
                count: end.minus(start),
            };
            <span class="k">self</span>.preview_spans[row <span class="k">as</span> usize] = Some(span);
            <span class="k">self</span>.surface_previews[row <span class="k">as</span> usize] =
                <span class="k">crate</span>::app::surface_preview::SurfacePreview::capture(
                    &amp;<span class="k">self</span>.tables,
                    span,
                    start.minus(<span class="k">self</span>.uploaded),
                    geom,
                );</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/29/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Scene {
    <span class="c">/// Rewrite one object's GPU rows in place; false when they no longer fit.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch_preview(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, geometry: &amp;Geometry, gpu: &amp;<span class="k">mut</span> Gpu) -&gt; bool {
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::patch::Counts;

        <span class="k">if</span> matches!(geometry, Geometry::PointCloud(_)) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> Some(span) = <span class="k">self</span>.preview_spans.get(row <span class="k">as</span> usize).copied().flatten() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(place) = <span class="k">self</span>.placement_of(row) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> <span class="k">let</span> Some(preview) = <span class="k">self</span>
            .surface_previews
            .get(row <span class="k">as</span> usize)
            .and_then(Option::as_ref)
            &amp;&amp; <span class="k">let</span> Some((vertices, pipes, bounds)) = preview.evaluate(geometry)
        {
            gpu.arena
                .patch_vertices(&amp;gpu.ctx, span.start.verts, &amp;vertices);
            gpu.segments.patch_pipes(&amp;gpu.ctx, span.start.pipes, &amp;pipes);
            gpu.objects
                .set_geometry_bounds(&amp;gpu.ctx, row, bounds, <span class="s">0</span>.<span class="s">0</span>, &amp;place);
            gpu.grew_bounds(row);
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> up = Upload::default();
        <span class="k">let</span> cx = WalkCx {
            vert_base: span.start.verts,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row,
        };
        <span class="k">let</span> result = walk_geometry(&amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> up), &amp;cx, geometry);

        <span class="k">if</span> Counts::of(&amp;up) != span.count {
            <span class="k">return</span> <span class="s">false</span>;
        }

        gpu.arena.patch(&amp;gpu.ctx, span.start, &amp;up.arena);
        gpu.segments.patch(&amp;gpu.ctx, span.start, &amp;up.seg);
        gpu.glyphs.patch(&amp;gpu.ctx, span.start, &amp;up.glyph);
        gpu.objects
            .set_geometry_bounds(&amp;gpu.ctx, row, result.bounds, result.spacing, &amp;place);

        <span class="k">for</span> (i, pipe) <span class="k">in</span> up.seg.pipes.iter().enumerate() {
            <span class="k">self</span>.edge_sources[span.start.pipes <span class="k">as</span> usize + i] = (
                pipe.instance_id,
                up.seg.pipe_ids.get(i).copied().unwrap_or(u32::MAX),
            );
        }

        gpu.grew_bounds(row);
        <span class="s">true</span>
    }
}

<span class="k">impl</span> Scene {
    <span class="c">/// Memory held by the edit previews.</span>
    <span class="k">pub</span> <span class="k">fn</span> preview_cache_bytes(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.surface_previews
            .iter()
            .flatten()
            .map(|p| p.allocated_bytes())
            .sum::&lt;usize&gt;()
            + <span class="k">self</span>.preview_spans.capacity()
                * std::mem::size_of::&lt;Option&lt;<span class="k">crate</span>::engine::gpu::patch::Span&gt;&gt;()
    }
}</code></pre></div>
<h2 id="step-9-srcappsession_iors">Step 9 · src/app/session_io.rs<a class="anchor" href="#/course/30-layer-tree#step-9-srcappsession_iors" aria-label="Link to this section">#</a></h2>
<p>Store visibility, locks and color overrides with each saved document.</p>
<p><code>lessons/30/src/app/session_io.rs</code> · edit · type this</p>
<p>Added after the line <code>hidden: Vec&lt;(usize, String)&gt;,</code> in <code>lessons/29/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[serde(default)]
    locked: Vec&lt;(usize, String)&gt;, <span class="c">// (document, guid) locked</span>
    #[serde(default)]
    colors: Vec&lt;(usize, String, [u8; 3])&gt;, <span class="c">// face colour overrides</span></code></pre></div>
<p>Added after the line <code>hidden.sort();</code> in <code>lessons/29/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> locked: Vec&lt;_&gt; = scene
        .locked
        .iter()
        .map(|(doc, id)| (*doc, id.to_string()))
        .collect();
    locked.sort();
    <span class="k">let</span> <span class="k">mut</span> colors: Vec&lt;_&gt; = scene
        .colors
        .iter()
        .map(|((doc, id), color)| (*doc, id.to_string(), *color))
        .collect();
    colors.sort();</code></pre></div>
<p>Added after the line <code>hidden,</code> in <code>lessons/29/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        locked,
        colors,</code></pre></div>
<p>Added after the line <code>.map(|(doc, id)| (doc, Rc::from(id)))</code> in <code>lessons/29/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    scene.locked = metadata
        .locked
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
    scene.colors = metadata
        .colors
        .into_iter()
        .map(|(doc, id, color)| ((doc, Rc::from(id)), color))
        .collect();</code></pre></div>
<p>Added after the line <code>scene.hidden.insert(scene.identity_of(1).unwrap());</code> in <code>lessons/29/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        scene.locked.insert(scene.identity_of(<span class="s">0</span>).unwrap());
        scene
            .colors
            .insert(scene.identity_of(<span class="s">1</span>).unwrap(), [<span class="s">240</span>, <span class="s">80</span>, <span class="s">30</span>]);
        <span class="k">let</span> bytes = save(&amp;scene).unwrap();
        assert!(scene.undo(), &quot;<span class="s">saving leaves live undo available</span>&quot;);
        <span class="k">let</span> restored = open(&amp;bytes).unwrap();
        assert_eq!(restored.docs.len(), <span class="s">2</span>);
        assert_eq!(restored.hidden.len(), <span class="s">1</span>);
        assert_eq!(restored.locked, scene.locked);
        assert_eq!(restored.colors, scene.colors);
        assert!(!restored.selectable(<span class="s">0</span>));
        assert!(restored.selectable(<span class="s">1</span>));</code></pre></div>
<h2 id="step-10-srcappsurface_previewrs">Step 10 · src/app/surface_preview.rs<a class="anchor" href="#/course/30-layer-tree#step-10-srcappsurface_previewrs" aria-label="Link to this section">#</a></h2>
<p>Retain UV samples and boundary endpoints, reevaluate changed surfaces, then patch their existing ranges.</p>
<p><code>lessons/30/src/app/surface_preview.rs</code> · 268 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::patch::Span;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, Upload};
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::{Geometry, NurbsSurface, RenderVertex};
<span class="k">use</span> std::collections::HashMap;

<span class="c">/// Where one GPU vertex came from on its surface.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> Sample {
    <span class="k">pub</span> index: u32,   <span class="c">// GPU vertex index</span>
    <span class="k">pub</span> surface: u32, <span class="c">// surface index in the BRep</span>
    <span class="k">pub</span> uv: [f64; <span class="s">2</span>], <span class="c">// parameter on that surface</span>
    <span class="k">pub</span> sign: f32,    <span class="c">// +1 or -1 on the normal</span>
}

<span class="c">/// Enough of a surface upload to re-evaluate it after a control moves.</span>
<span class="k">pub</span> <span class="k">struct</span> SurfacePreview {
    samples: Vec&lt;Sample&gt;,              <span class="c">// uv of every vertex</span>
    vertices: Vec&lt;RenderVertex&gt;,       <span class="c">// vertices as uploaded</span>
    controls: Vec&lt;Vec&lt;f64&gt;&gt;,           <span class="c">// control points at capture, per surface</span>
    pipes: Vec&lt;CylinderSegment&gt;,       <span class="c">// edge pipes as uploaded</span>
    pipe_vertices: Vec&lt;[usize; 2]&gt;,    <span class="c">// vertex index at each pipe end</span>
    pipe_normals: Vec&lt;[usize; 2]&gt;,     <span class="c">// vertex whose normal each pipe side uses</span>
    chains: Vec&lt;std::ops::Range&lt;u32&gt;&gt;, <span class="c">// joined pipe runs</span>
}

<span class="k">impl</span> SurfacePreview {
    <span class="c">/// Remember the upload of one surface or BRep.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> capture(
        up: &amp;Upload,
        span: Span,
        local: <span class="k">crate</span>::engine::gpu::patch::Counts,
        geometry: &amp;Geometry,
    ) -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">if</span> span.count.verts == <span class="s">0</span>
            || span.count.ribbons != <span class="s">0</span>
            || span.count.spheres != <span class="s">0</span>
            || span.count.dots != <span class="s">0</span>
        {
            <span class="k">return</span> None; <span class="c">// not a surface upload</span>
        }

        <span class="k">let</span> first = local.verts <span class="k">as</span> usize; <span class="c">// first vertex in the upload</span>
        <span class="k">let</span> end = first + span.count.verts <span class="k">as</span> usize;
        <span class="k">let</span> samples: Vec&lt;_&gt; = up
            .arena
            .surface_samples
            .iter()
            .filter(|sample| (first..end).contains(&amp;(sample.index <span class="k">as</span> usize)))
            .copied()
            .collect();

        <span class="k">if</span> samples.len() != span.count.verts <span class="k">as</span> usize {
            <span class="k">return</span> None; <span class="c">// a vertex without uv</span>
        }

        <span class="c">// vertices at each position</span>
        <span class="k">let</span> <span class="k">mut</span> positions = HashMap::&lt;[u32; 3], Vec&lt;usize&gt;&gt;::new();

        <span class="k">for</span> (i, vertex) <span class="k">in</span> up.arena.verts[first..end].iter().enumerate() {
            positions
                .entry(vertex.position.map(f32::to_bits))
                .or_default()
                .push(i);
        }

        <span class="k">let</span> start_pipe = local.pipes <span class="k">as</span> usize;
        <span class="k">let</span> end_pipe = start_pipe + span.count.pipes <span class="k">as</span> usize;
        <span class="k">let</span> pipes = up.seg.pipes[start_pipe..end_pipe].to_vec();
        <span class="k">let</span> pipe_vertices = pipes
            .iter()
            .map(|p| {
                Some([
                    *positions.get(&amp;p.p0.map(f32::to_bits))?.first()?,
                    *positions.get(&amp;p.p1.map(f32::to_bits))?.first()?,
                ])
            })
            .collect::&lt;Option&lt;Vec&lt;_&gt;&gt;&gt;()?;
        <span class="c">// a vertex on each side of the pipe, for its normals</span>
        <span class="k">let</span> pipe_normals = pipes
            .iter()
            .map(|p| {
                <span class="k">let</span> candidates = positions.get(&amp;p.p0.map(f32::to_bits))?;
                <span class="k">let</span> first = *candidates.first()?;
                <span class="k">let</span> other = candidates
                    .iter()
                    .copied()
                    .find(|&amp;i| samples[i].surface != samples[first].surface)
                    .unwrap_or(first);
                Some([first, other])
            })
            .collect::&lt;Option&lt;Vec&lt;_&gt;&gt;&gt;()?;
        <span class="k">let</span> chains = up
            .seg
            .pipe_chains
            .iter()
            .filter(|r| r.start &gt;= local.pipes &amp;&amp; r.end &lt;= local.pipes + span.count.pipes)
            .map(|r| r.start - local.pipes..r.end - local.pipes)
            .collect();
        Some(<span class="k">Self</span> {
            samples,
            vertices: up.arena.verts[first..end].to_vec(),
            controls: surfaces(geometry)?.iter().map(|s| s.m_cv.clone()).collect(),
            pipes,
            pipe_vertices,
            pipe_normals,
            chains,
        })
    }

    <span class="c">/// Re-evaluate the vertices and pipes for the edited geometry.</span>
    <span class="k">pub</span> <span class="k">fn</span> evaluate(&amp;<span class="k">self</span>, geometry: &amp;Geometry) -&gt; Option&lt;(Vec&lt;RenderVertex&gt;, SegRows, AABB)&gt; {
        <span class="k">let</span> surfaces = surfaces(geometry)?;
        <span class="c">// surfaces whose controls moved</span>
        <span class="k">let</span> changed: Vec&lt;_&gt; = surfaces
            .iter()
            .enumerate()
            .map(|(i, s)| <span class="k">self</span>.controls.get(i) != Some(&amp;s.m_cv))
            .collect();
        <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
        <span class="k">let</span> <span class="k">mut</span> vertices = Vec::with_capacity(<span class="k">self</span>.samples.len());

        <span class="k">for</span> (sample, baseline) <span class="k">in</span> <span class="k">self</span>.samples.iter().zip(&amp;<span class="k">self</span>.vertices) {
            <span class="k">let</span> surface = surfaces.get(sample.surface <span class="k">as</span> usize)?;

            <span class="c">// unchanged surface: keep the vertex</span>
            <span class="k">if</span> !changed[sample.surface <span class="k">as</span> usize] {
                bounds.union_with_point(
                    baseline.position[<span class="s">0</span>] <span class="k">as</span> f64,
                    baseline.position[<span class="s">1</span>] <span class="k">as</span> f64,
                    baseline.position[<span class="s">2</span>] <span class="k">as</span> f64,
                );
                vertices.push(*baseline);
                <span class="k">continue</span>;
            }

            <span class="k">let</span> point = surface.point_at(sample.uv[<span class="s">0</span>], sample.uv[<span class="s">1</span>])?;
            <span class="k">let</span> normal = surface.normal_at(sample.uv[<span class="s">0</span>], sample.uv[<span class="s">1</span>]);
            <span class="k">let</span> position = point.to_f32();

            <span class="k">if</span> position.iter().any(|p| !p.is_finite()) {
                <span class="k">return</span> None;
            }

            bounds.union_with_point(position[<span class="s">0</span>] <span class="k">as</span> f64, position[<span class="s">1</span>] <span class="k">as</span> f64, position[<span class="s">2</span>] <span class="k">as</span> f64);
            vertices.push(RenderVertex {
                position,
                normal: [
                    normal[<span class="s">0</span>] <span class="k">as</span> f32 * sample.sign,
                    normal[<span class="s">1</span>] <span class="k">as</span> f32 * sample.sign,
                    normal[<span class="s">2</span>] <span class="k">as</span> f32 * sample.sign,
                ],
                color: baseline.color,
            });
        }

        <span class="k">let</span> <span class="k">mut</span> segments = SegRows {
            pipe_chains: <span class="k">self</span>.chains.clone(),
            ..Default::default()
        };

        <span class="k">for</span> ((pipe, ends), normals) <span class="k">in</span> <span class="k">self</span>
            .pipes
            .iter()
            .zip(&amp;<span class="k">self</span>.pipe_vertices)
            .zip(&amp;<span class="k">self</span>.pipe_normals)
        {
            <span class="k">let</span> <span class="k">mut</span> pipe = *pipe;
            pipe.p0 = vertices[ends[<span class="s">0</span>]].position;
            pipe.p1 = vertices[ends[<span class="s">1</span>]].position;
            <span class="k">let</span> a = vertices[normals[<span class="s">0</span>]].normal.map(f64::from);
            <span class="k">let</span> b = vertices[normals[<span class="s">1</span>]].normal.map(f64::from);
            pipe.facing = super::walk::encode::pack_facing(Some(&amp;a), Some(&amp;b));
            segments.pipes.push(pipe);
        }

        Some((vertices, segments, bounds))
    }

    <span class="c">/// Memory held by the preview.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.samples.capacity() * std::mem::size_of::&lt;Sample&gt;()
            + <span class="k">self</span>.vertices.capacity() * std::mem::size_of::&lt;RenderVertex&gt;()
            + <span class="k">self</span>.controls.capacity() * std::mem::size_of::&lt;Vec&lt;f64&gt;&gt;()
            + <span class="k">self</span>
                .controls
                .iter()
                .map(|v| v.capacity() * <span class="s">8</span>)
                .sum::&lt;usize&gt;()
            + <span class="k">self</span>.pipes.capacity() * std::mem::size_of::&lt;CylinderSegment&gt;()
            + (<span class="k">self</span>.pipe_vertices.capacity() + <span class="k">self</span>.pipe_normals.capacity())
                * std::mem::size_of::&lt;[usize; <span class="s">2</span>]&gt;()
            + <span class="k">self</span>.chains.capacity() * std::mem::size_of::&lt;std::ops::Range&lt;u32&gt;&gt;()
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::deform::{<span class="k">self</span>, Target};
    <span class="k">use</span> <span class="k">crate</span>::app::walk::{Walk, WalkCx, walk_geometry};
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::patch::Counts;
    <span class="k">use</span> session_rust::{BRep, Xform};
    <span class="k">use</span> std::rc::Rc;

    <span class="c">/// Vertices at one point keep their own uv when pulled apart.</span>
    #[test]
    <span class="k">fn</span> joined_shell_preview_moves_source_samples_and_cancel_restores_them() {
        <span class="k">let</span> source = Geometry::BRep(Rc::new(BRep::create_box(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>)));
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">0</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">3</span>,
        };
        walk_geometry(&amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> upload), &amp;cx, &amp;source);
        <span class="k">let</span> span = Span {
            start: Counts::default(),
            count: Counts::of(&amp;upload),
        };
        <span class="k">let</span> preview = SurfacePreview::capture(&amp;upload, span, Counts::default(), &amp;source)
            .expect(&quot;<span class="s">joined box has parameter provenance</span>&quot;);
        <span class="k">let</span> (before, _, _) = preview.evaluate(&amp;source).unwrap();
        assert_eq!(before.len(), upload.arena.verts.len());

        <span class="k">for</span> (a, b) <span class="k">in</span> before.iter().zip(&amp;upload.arena.verts) {
            assert_eq!(
                a.position, b.position,
                &quot;<span class="s">initial sample is the drawn source position</span>&quot;
            );
        }

        <span class="k">let</span> changed =
            deform::transform(&amp;source, Target::Face(<span class="s">0</span>), &amp;Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>))
                .unwrap();
        <span class="k">let</span> Geometry::BRep(brep) = &amp;changed <span class="k">else</span> {
            unreachable!()
        };
        assert!(brep.is_solid(), &quot;<span class="s">source shell remains joined</span>&quot;);
        <span class="k">let</span> (after, pipes, _) = preview.evaluate(&amp;changed).unwrap();
        assert!(
            before
                .iter()
                .zip(&amp;after)
                .any(|(a, b)| a.position != b.position)
        );
        assert!(
            before
                .iter()
                .zip(&amp;after)
                .any(|(a, b)| a.position == b.position)
        );

        <span class="k">for</span> pipe <span class="k">in</span> pipes.pipes {
            assert!(after.iter().any(|v| v.position == pipe.p0));
            assert!(after.iter().any(|v| v.position == pipe.p1));
        }

        <span class="k">let</span> (cancelled, _, _) = preview.evaluate(&amp;source).unwrap();

        <span class="k">for</span> (a, b) <span class="k">in</span> before.iter().zip(&amp;cancelled) {
            assert_eq!(a.position, b.position);
        }
    }
}

<span class="c">/// The surfaces inside a geometry.</span>
<span class="k">fn</span> surfaces(geometry: &amp;Geometry) -&gt; Option&lt;&amp;[NurbsSurface]&gt; {
    <span class="k">match</span> geometry {
        Geometry::BRep(b) =&gt; Some(&amp;b.m_surfaces),
        Geometry::NurbsSurface(s) =&gt; Some(std::slice::from_ref(s)),
        Geometry::Element(e) =&gt; <span class="k">match</span> e.geometry() {
            session_rust::element::ElementGeometry::BRep(b) =&gt; Some(&amp;b.m_surfaces),
            _ =&gt; None,
        },
        _ =&gt; None,
    }
}</code></pre></div>
<h2 id="step-11-srcappuirs">Step 11 · src/app/ui.rs<a class="anchor" href="#/course/30-layer-tree#step-11-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>Draw one layer tree with bulbs, locks, color controls and command hints.</p>
<p><code>lessons/30/src/app/ui.rs</code> · edit · type this</p>
<p>Replaces the line <code>let snapshot = MODEL.with_borrow(|model| serde_json::json…</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> snapshot = MODEL.with_borrow(|model| serde_json::json!({&quot;<span class="s">framework</span>&quot;: &quot;<span class="s">egui 0.34.3</span>&quot;, &quot;<span class="s">controls</span>&quot;: <span class="k">self</span>.controls, &quot;<span class="s">command_open</span>&quot;: model.command_open, &quot;<span class="s">layers_open</span>&quot;: model.layers_open, &quot;<span class="s">command</span>&quot;: model.command, &quot;<span class="s">history</span>&quot;: model.history, &quot;<span class="s">hint</span>&quot;: <span class="k">crate</span>::app::command::hint(&amp;model.command)}));</code></pre></div>
<p>Replaces the 8 lines from <code>let mut at = 0;</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">for</span> row <span class="k">in</span> &amp;model.rows {
                        ui.horizontal(|ui| {
                            <span class="k">if</span> <span class="k">let</span> Some((_, index)) = row.key.split_once('<span class="s">/</span>')
                                &amp;&amp; row.key.starts_with(&quot;<span class="s">select/</span>&quot;)
                            {
                                ui.spacing_mut().item_spacing.x = <span class="s">2</span>.;
                                ui.add_space(row.depth.min(<span class="s">8</span>) <span class="k">as</span> f32 * <span class="s">10</span>.);
                                <span class="k">let</span> response = layer_icon(ui, &quot;<span class="s">open</span>&quot;, row);
                                record(controls, &amp;format!(&quot;<span class="s">open/</span>{<span class="s">index</span>}&quot;), &amp;row.label, &amp;response);

                                <span class="k">if</span> response.clicked() &amp;&amp; row.expanded.is_some() {
                                    *action = Some(format!(&quot;<span class="s">open/</span>{<span class="s">index</span>}&quot;));
                                }

                                <span class="k">let</span> width = (ui.available_width() - <span class="s">86</span>.).max(<span class="s">24</span>.);
                                <span class="k">let</span> response = ui
                                    .add_sized(
                                        [width, <span class="s">28</span>.],
                                        egui::Button::new(&amp;row.label).frame(<span class="s">false</span>).truncate(),
                                    )
                                    .on_hover_text(format!(
                                        &quot;{}<span class="s"> · </span>{}<span class="s"> objects</span>&quot;,
                                        row.label, row.count
                                    ));
                                record(
                                    controls,
                                    &amp;row.key,
                                    &amp;format!(&quot;<span class="s">Select </span>{}&quot;, row.label),
                                    &amp;response,
                                );

                                <span class="k">if</span> response.clicked() {
                                    *action = Some(row.key.clone());
                                }

                                <span class="k">for</span> kind <span class="k">in</span> [&quot;<span class="s">hide</span>&quot;, &quot;<span class="s">lock</span>&quot;] {
                                    <span class="k">let</span> response = layer_icon(ui, kind, row);
                                    record(
                                        controls,
                                        &amp;format!(&quot;{<span class="s">kind</span>}<span class="s">/</span>{<span class="s">index</span>}&quot;),
                                        &amp;format!(
                                            &quot;{}<span class="s"> </span>{}&quot;,
                                            <span class="k">if</span> kind == &quot;<span class="s">hide</span>&quot; {
                                                <span class="k">if</span> row.hidden { &quot;<span class="s">Show</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Hide</span>&quot; }
                                            } <span class="k">else</span> <span class="k">if</span> row.locked {
                                                &quot;<span class="s">Unlock</span>&quot;
                                            } <span class="k">else</span> {
                                                &quot;<span class="s">Lock</span>&quot;
                                            },
                                            row.label
                                        ),
                                        &amp;response,
                                    );

                                    <span class="k">if</span> response.clicked() {
                                        *action = Some(format!(&quot;{<span class="s">kind</span>}<span class="s">/</span>{<span class="s">index</span>}&quot;));
                                    }
                                }

                                layer_color(ui, row, index, controls, action);
                            } <span class="k">else</span> {
                                <span class="k">let</span> response = ui.button(&amp;row.label);</code></pre></div>
<p>Replaces the 4 lines from <code>fn layer_button(ui: &amp;mut egui::Ui, row: &amp;LayerRow) -&gt; egu…</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One icon of a layer row: eye, lock or arrow.</span>
<span class="k">fn</span> layer_icon(ui: &amp;<span class="k">mut</span> egui::Ui, kind: &amp;str, row: &amp;LayerRow) -&gt; egui::Response {
    <span class="k">let</span> (rect, response) = ui.allocate_exact_size(egui::vec2(<span class="s">26</span>., <span class="s">28</span>.), egui::Sense::click());
    <span class="k">let</span> c = rect.center();
    <span class="k">let</span> ink = ui.visuals().text_color();
    <span class="k">let</span> stroke = egui::Stroke::new(<span class="s">1</span>.<span class="s">4_f32</span>, ink);

    <span class="k">if</span> response.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(<span class="s">1</span>.), <span class="s">3</span>., ui.visuals().widgets.hovered.bg_fill);
    }

    <span class="k">match</span> kind {
        &quot;<span class="s">open</span>&quot; =&gt; {
            <span class="k">if</span> <span class="k">let</span> Some(open) = row.expanded {
                <span class="k">let</span> points = <span class="k">if</span> open {
                    vec![
                        c + egui::vec2(-<span class="s">4</span>., -<span class="s">2</span>.),
                        c + egui::vec2(<span class="s">4</span>., -<span class="s">2</span>.),
                        c + egui::vec2(<span class="s">0</span>., <span class="s">3</span>.),
                    ]
                } <span class="k">else</span> {
                    vec![
                        c + egui::vec2(-<span class="s">2</span>., -<span class="s">4</span>.),
                        c + egui::vec2(-<span class="s">2</span>., <span class="s">4</span>.),
                        c + egui::vec2(<span class="s">3</span>., <span class="s">0</span>.),
                    ]
                };
                ui.painter()
                    .add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
            }

            response.on_hover_text(&quot;<span class="s">Expand or collapse</span>&quot;)
        }
        &quot;<span class="s">hide</span>&quot; =&gt; {
            <span class="k">let</span> fill = <span class="k">if</span> row.hidden {
                egui::Color32::TRANSPARENT
            } <span class="k">else</span> {
                egui::Color32::from_rgb(<span class="s">255</span>, <span class="s">216</span>, <span class="s">80</span>)
            };
            ui.painter()
                .circle(c + egui::vec2(<span class="s">0</span>., -<span class="s">3</span>.), <span class="s">5</span>., fill, stroke);

            <span class="k">for</span> y <span class="k">in</span> [<span class="s">3</span>., <span class="s">6</span>.] {
                ui.painter()
                    .line_segment([c + egui::vec2(-<span class="s">3</span>., y), c + egui::vec2(<span class="s">3</span>., y)], stroke);
            }

            <span class="k">if</span> row.hidden {
                ui.painter()
                    .line_segment([c + egui::vec2(-<span class="s">7</span>., <span class="s">8</span>.), c + egui::vec2(<span class="s">7</span>., -<span class="s">9</span>.)], stroke);
            }

            response.on_hover_text(<span class="k">if</span> row.hidden {
                &quot;<span class="s">Show object and children</span>&quot;
            } <span class="k">else</span> {
                &quot;<span class="s">Hide object and children</span>&quot;
            })
        }
        _ =&gt; {
            ui.painter().rect(
                egui::Rect::from_center_size(c + egui::vec2(<span class="s">0</span>., <span class="s">3</span>.), egui::vec2(<span class="s">11</span>., <span class="s">9</span>.)),
                <span class="s">1</span>.,
                <span class="k">if</span> row.locked {
                    egui::Color32::from_rgb(<span class="s">225</span>, <span class="s">180</span>, <span class="s">90</span>)
                } <span class="k">else</span> {
                    egui::Color32::TRANSPARENT
                },
                stroke,
                egui::StrokeKind::Inside,
            );
            <span class="k">let</span> x = <span class="k">if</span> row.locked { <span class="s">0</span>. } <span class="k">else</span> { <span class="s">3</span>. };
            ui.painter().add(egui::Shape::line(
                vec![
                    c + egui::vec2(-<span class="s">3</span>. + x, -<span class="s">1</span>.),
                    c + egui::vec2(-<span class="s">3</span>. + x, -<span class="s">6</span>.),
                    c + egui::vec2(<span class="s">3</span>. + x, -<span class="s">6</span>.),
                    c + egui::vec2(<span class="s">3</span>. + x, -<span class="s">1</span>.),
                ],
                stroke,
            ));
            response.on_hover_text(<span class="k">if</span> row.locked {
                &quot;<span class="s">Unlock object and children</span>&quot;
            } <span class="k">else</span> {
                &quot;<span class="s">Lock selection of object and children</span>&quot;
            })
        }
    }
}

<span class="c">/// The colour swatches of a layer row.</span>
<span class="k">fn</span> layer_color(
    ui: &amp;<span class="k">mut</span> egui::Ui,
    row: &amp;LayerRow,
    index: &amp;str,
    controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;,
    action: &amp;<span class="k">mut</span> Option&lt;String&gt;,
) {
    <span class="k">let</span> <span class="k">mut</span> color = row.color.unwrap_or([<span class="s">180</span>, <span class="s">180</span>, <span class="s">180</span>]);
    <span class="k">let</span> response = ui
        .menu_button(
            egui::RichText::new(&quot;<span class="s">■</span>&quot;).color(egui::Color32::from_rgb(color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>])),
            |ui| {
                ui.label(&quot;<span class="s">Object and child colors</span>&quot;);
                <span class="k">for</span> colors <span class="k">in</span> [
                    [
                        (&quot;<span class="s">Red</span>&quot;, [<span class="s">230</span>, <span class="s">65</span>, <span class="s">55</span>]),
                        (&quot;<span class="s">Orange</span>&quot;, [<span class="s">240</span>, <span class="s">145</span>, <span class="s">45</span>]),
                        (&quot;<span class="s">Yellow</span>&quot;, [<span class="s">240</span>, <span class="s">210</span>, <span class="s">60</span>]),
                    ],
                    [
                        (&quot;<span class="s">Green</span>&quot;, [<span class="s">60</span>, <span class="s">170</span>, <span class="s">100</span>]),
                        (&quot;<span class="s">Blue</span>&quot;, [<span class="s">65</span>, <span class="s">130</span>, <span class="s">225</span>]),
                        (&quot;<span class="s">Violet</span>&quot;, [<span class="s">160</span>, <span class="s">85</span>, <span class="s">210</span>]),
                    ],
                    [
                        (&quot;<span class="s">White</span>&quot;, [<span class="s">245</span>, <span class="s">245</span>, <span class="s">245</span>]),
                        (&quot;<span class="s">Gray</span>&quot;, [<span class="s">150</span>, <span class="s">150</span>, <span class="s">150</span>]),
                        (&quot;<span class="s">Black</span>&quot;, [<span class="s">35</span>, <span class="s">35</span>, <span class="s">35</span>]),
                    ],
                ] {
                    ui.horizontal(|ui| {
                        <span class="k">for</span> (name, rgb) <span class="k">in</span> colors {
                            <span class="k">let</span> response = ui
                                .add_sized(
                                    [<span class="s">48</span>., <span class="s">28</span>.],
                                    egui::Button::new(
                                        egui::RichText::new(&quot;<span class="s">■</span>&quot;)
                                            .color(egui::Color32::from_rgb(rgb[<span class="s">0</span>], rgb[<span class="s">1</span>], rgb[<span class="s">2</span>])),
                                    ),
                                )
                                .on_hover_text(name);
                            <span class="k">let</span> key =
                                format!(&quot;<span class="s">color/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">:02x</span>}{<span class="s">:02x</span>}{<span class="s">:02x</span>}&quot;, rgb[<span class="s">0</span>], rgb[<span class="s">1</span>], rgb[<span class="s">2</span>]);
                            record(controls, &amp;key, name, &amp;response);

                            <span class="k">if</span> response.clicked() {
                                *action = Some(key);
                                ui.close();
                            }
                        }
                    });
                }
                ui.separator();
                <span class="k">let</span> <span class="k">mut</span> changed = <span class="s">false</span>;

                <span class="k">for</span> (channel, value) <span class="k">in</span> [&quot;<span class="s">R</span>&quot;, &quot;<span class="s">G</span>&quot;, &quot;<span class="s">B</span>&quot;].into_iter().zip(color.iter_mut()) {
                    changed |= ui
                        .add(egui::Slider::new(value, <span class="s">0</span>..=<span class="s">255</span>).text(channel))
                        .changed();
                }

                <span class="k">if</span> changed {
                    *action = Some(format!(
                        &quot;<span class="s">color/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">:02x</span>}{<span class="s">:02x</span>}{<span class="s">:02x</span>}&quot;,
                        color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>]
                    ));
                }
            },
        )
        .response
        .on_hover_text(&quot;<span class="s">Change object and child colors</span>&quot;);
    record(
        controls,
        &amp;format!(&quot;<span class="s">color/</span>{<span class="s">index</span>}&quot;),
        &amp;format!(&quot;<span class="s">Color </span>{}&quot;, row.label),
        &amp;response,
    );</code></pre></div>
<p>Replaces the line <code>let height = if model.command_open { 160.0 } else { 76.0 };</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> height = <span class="k">if</span> model.command_open { <span class="s">160</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">104</span>.<span class="s">0</span> };
    egui::Panel::bottom(&quot;<span class="s">command-line</span>&quot;)
        .default_size(height)
        .size_range(<span class="s">104</span>.<span class="s">0</span>..=<span class="s">260</span>.<span class="s">0</span>)
        .resizable(<span class="s">true</span>)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(&quot;<span class="s">command-history</span>&quot;)
                .stick_to_bottom(<span class="s">true</span>)
                .max_height((ui.available_height() - <span class="s">68</span>.<span class="s">0</span>).max(<span class="s">20</span>.<span class="s">0</span>))</code></pre></div>
<p>Added after the line <code>ui.separator();</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ui.small(<span class="k">crate</span>::app::command::hint(&amp;model.command));</code></pre></div>
<p>Replaces the line <code>.hint_text(&quot;Type a command&quot;),</code> in <code>lessons/29/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        .hint_text(&quot;<span class="s">Point 0,0,0</span>&quot;),</code></pre></div>
<h2 id="step-12-srcappwalkbreprs">Step 12 · src/app/walk/brep.rs<a class="anchor" href="#/course/30-layer-tree#step-12-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>Record surface parameters alongside tessellated vertices for later preview updates.</p>
<p><code>lessons/30/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/29/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> surface_index = b.m_faces[fi].surface_index <span class="k">as</span> usize;
        cache_samples(arena, fm, &amp;rm, &amp;b.m_surfaces[surface_index], surface_index); <span class="c">// uv per vertex for live edits</span></code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/29/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Remember each vertex's uv on its surface.</span>
<span class="k">fn</span> cache_samples(
    arena: &amp;<span class="k">mut</span> ArenaRows,
    mesh: &amp;Mesh,
    render: &amp;RenderMesh,
    surface: &amp;NurbsSurface,
    index: usize,
) {
    <span class="k">let</span> <span class="k">mut</span> rows: Vec&lt;_&gt; = mesh.vertex.iter().collect();
    rows.sort_unstable_by_key(|&amp;(key, _)| *key);

    <span class="k">if</span> rows.len() != render.vertices.len() {
        <span class="k">return</span>; <span class="c">// vertices were split, no mapping</span>
    }

    <span class="k">for</span> (offset, ((_, vertex), rendered)) <span class="k">in</span> rows.into_iter().zip(&amp;render.vertices).enumerate() {
        <span class="k">let</span> (Some(&amp;u), Some(&amp;v)) = (vertex.attributes.get(&quot;<span class="s">u</span>&quot;), vertex.attributes.get(&quot;<span class="s">v</span>&quot;)) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> normal = surface.normal_at(u, v);
        <span class="c">// sign flips when the rendered normal was reversed</span>
        <span class="k">let</span> dot = (<span class="s">0</span>..<span class="s">3</span>)
            .map(|d| normal[d] * rendered.normal[d] <span class="k">as</span> f64)
            .sum::&lt;f64&gt;();
        arena
            .surface_samples
            .push(<span class="k">crate</span>::app::surface_preview::Sample {
                index: arena.verts.len() <span class="k">as</span> u32 + offset <span class="k">as</span> u32,
                surface: index <span class="k">as</span> u32,
                uv: [u, v],
                sign: <span class="k">if</span> dot &lt; <span class="s">0</span>.<span class="s">0</span> { -<span class="s">1</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">1</span>.<span class="s">0</span> },
            });
    }
}</code></pre></div>
<h2 id="step-13-srcappwalkpointsrs">Step 13 · src/app/walk/points.rs<a class="anchor" href="#/course/30-layer-tree#step-13-srcappwalkpointsrs" aria-label="Link to this section">#</a></h2>
<p>Give a newly created point a visible screen-size marker.</p>
<p><code>lessons/30/src/app/walk/points.rs</code> · edit · type this</p>
<p>Added after the line <code>let center = p.to_f32();</code> in <code>lessons/29/src/app/walk/points.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> radius = encode_width(p.width); <span class="c">// negative = pixels</span>
    glyph.dots.push(GlyphPoint {
        center,
        radius: <span class="k">if</span> radius == <span class="s">0</span>.<span class="s">0</span> { -<span class="s">3</span>.<span class="s">0</span> } <span class="k">else</span> { radius }, <span class="c">// no pen: 3 px dot</span></code></pre></div>
<h2 id="step-14-srcenginegpuarenars">Step 14 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/30-layer-tree#step-14-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>Patch the existing face and vertex ranges during a preview.</p>
<p><code>lessons/30/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Added after the line <code>pub face_sources: Vec&lt;super::faces::FaceSource&gt;,</code> in <code>lessons/29/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> surface_samples: Vec&lt;<span class="k">crate</span>::app::surface_preview::Sample&gt;,</code></pre></div>
<p>Added after the line <code>drop_rows(&amp;mut self.face_sources);</code> in <code>lessons/29/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.surface_samples);</code></pre></div>
<p>Added after the line <code>.append(ctx, up, [&amp;self.verts.buf, &amp;self.vids.buf, &amp;self.…</code> in <code>lessons/29/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Overwrite vertices starting at row \`first\`.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch_vertices(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, first: u32, vertices: &amp;[RenderVertex]) {
        <span class="k">self</span>.tiles.invalidate();
        <span class="k">self</span>.verts.write_at(ctx, first, vertices);
    }

    <span class="c">/// Overwrite one object's rows in place.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, at: super::patch::Counts, up: &amp;ArenaRows) {
        <span class="k">self</span>.tiles.invalidate();
        <span class="k">self</span>.verts.write_at(ctx, at.verts, &amp;up.verts);
        <span class="k">self</span>.vids.write_at(ctx, at.verts, &amp;up.vids);
        <span class="k">self</span>.faces.write_at(ctx, at.faces, &amp;up.idx);
        <span class="k">self</span>.print.write_at(ctx, at.print, &amp;up.idx_print);
        <span class="k">self</span>.text.write_at(ctx, at.text, &amp;up.idx_text);
        <span class="k">self</span>.source_faces.patch(ctx, at, up);
    }

    <span class="c">/// Draw the solid faces.</span></code></pre></div>
<h2 id="step-15-srcenginegpubuffersrs">Step 15 · src/engine/gpu/buffers.rs<a class="anchor" href="#/course/30-layer-tree#step-15-srcenginegpubuffersrs" aria-label="Link to this section">#</a></h2>
<p>Allow existing GPU buffers to receive geometry patches.</p>
<p><code>lessons/30/src/engine/gpu/buffers.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn write_at&lt;T: Pod&gt;(&amp;self, ctx: &amp;GpuCtx, at: u32, dat…</code> in <code>lessons/29/src/engine/gpu/buffers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> data.is_empty() {
            <span class="k">return</span>;
        }</code></pre></div>
<h2 id="step-16-srcenginegpufacesrs">Step 16 · src/engine/gpu/faces.rs<a class="anchor" href="#/course/30-layer-tree#step-16-srcenginegpufacesrs" aria-label="Link to this section">#</a></h2>
<p>Keep source-face identities attached to the updated triangle ranges.</p>
<p><code>lessons/30/src/engine/gpu/faces.rs</code> · edit · type this</p>
<p>Added after the line <code>}));</code> in <code>lessons/29/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Overwrite one object's faces in place.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        at: super::patch::Counts,
        up: &amp;super::arena::ArenaRows,
    ) {
        <span class="k">let</span> first = at.sources <span class="k">as</span> usize;
        <span class="k">self</span>.sources[first..first + up.face_sources.len()].copy_from_slice(&amp;up.face_sources);
        <span class="k">let</span> ids: Vec&lt;u32&gt; = (<span class="s">0</span>..up.idx.len() / <span class="s">3</span>)
            .map(|i| <span class="k">match</span> up.face_ids.get(i) {
                Some(&amp;id) <span class="k">if</span> id != u32::MAX =&gt; at.sources + id,
                _ =&gt; u32::MAX,
            })
            .collect();
        <span class="k">self</span>.ids.write_at(ctx, at.faces / <span class="s">3</span>, &amp;ids);
        <span class="k">self</span>.revision = <span class="k">self</span>.revision.wrapping_add(<span class="s">1</span>);
    }

    <span class="c">/// Face behind a pick id, if it belongs to object \`row\`.</span></code></pre></div>
<h2 id="step-17-srcenginegpuglyphsrs">Step 17 · src/engine/gpu/glyphs.rs<a class="anchor" href="#/course/30-layer-tree#step-17-srcenginegpuglyphsrs" aria-label="Link to this section">#</a></h2>
<p>Patch existing marker ranges during a source preview.</p>
<p><code>lessons/30/src/engine/gpu/glyphs.rs</code> · edit · type this</p>
<p>Added after the line <code>impl GlyphLane {</code> in <code>lessons/29/src/engine/gpu/glyphs.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Overwrite one object's rows in place.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, at: super::patch::Counts, up: &amp;GlyphRows) {
        <span class="k">self</span>.spheres.buf.write_at(ctx, at.spheres, &amp;up.spheres);
        <span class="k">self</span>.dots.buf.write_at(ctx, at.dots, &amp;up.dots);
    }</code></pre></div>
<h2 id="step-18-srcenginegpuinstancers">Step 18 · src/engine/gpu/instance.rs<a class="anchor" href="#/course/30-layer-tree#step-18-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>Add a flag for display color overrides to the instance row.</p>
<p><code>lessons/30/src/engine/gpu/instance.rs</code> · edit · type this</p>
<p>Added after the line <code>pub const FLAG_SINGLE: u32 = 1 &lt;&lt; 7;</code> in <code>lessons/29/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Use the layer color instead of the object's.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_COLOR: u32 = <span class="s">1</span> &lt;&lt; <span class="s">8</span>;

    <span class="c">/// The one row an empty scene binds: identity, grey, no flags.</span></code></pre></div>
<h2 id="step-19-srcenginegpumodrs">Step 19 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/30-layer-tree#step-19-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the new GPU resources, initialize them and include their allocations in the counters.</p>
<p><code>lessons/30/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod objects;</code> in <code>lessons/29/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span>(<span class="k">crate</span>) <span class="k">mod</span> patch;</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/29/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> <span class="k">fn</span> set_object_color(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, color: [u8; <span class="s">3</span>]) {
        <span class="k">self</span>.objects.set_color(&amp;<span class="k">self</span>.ctx, row, color);
        <span class="k">self</span>.splat.invalidate();
    }

    <span class="c">/// Hide or show object \`row\`.</span></code></pre></div>
<h2 id="step-20-srcenginegpuobjectsrs">Step 20 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/30-layer-tree#step-20-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>Update geometry bounds and display colors in the existing object row.</p>
<p><code>lessons/30/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/29/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Set a row's box and spacing after its geometry changed.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> set_geometry_bounds(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        row: u32,
        bounds: AABB,
        spacing: f32,
        place: &amp;Xform,
    ) {
        <span class="k">self</span>.local_bounds[row <span class="k">as</span> usize] = bounds;
        <span class="k">self</span>.rows[row <span class="k">as</span> usize].spacing = spacing;
        <span class="k">self</span>.set_placement(ctx, row, place);
    }

    <span class="c">/// Change display color without rebuilding geometry or placement.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_color(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, color: [u8; <span class="s">3</span>]) {
        <span class="k">if</span> <span class="k">let</span> Some(r) = <span class="k">self</span>.rows.get_mut(row <span class="k">as</span> usize) {
            r.color = [
                color[<span class="s">0</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                color[<span class="s">1</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                color[<span class="s">2</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                <span class="s">1</span>.,
            ];
            r.flags |= Instance::FLAG_COLOR;
            <span class="k">self</span>.buffer.write_at(ctx, row, std::slice::from_ref(r));
        }
    }

    <span class="c">/// Set or clear one flag bit on one row.</span></code></pre></div>
<h2 id="step-21-srcenginegpupatchrs">Step 21 · src/engine/gpu/patch.rs<a class="anchor" href="#/course/30-layer-tree#step-21-srcenginegpupatchrs" aria-label="Link to this section">#</a></h2>
<p>Count each upload table and record the ranges owned by an object.</p>
<p><code>lessons/30/src/engine/gpu/patch.rs</code> · 64 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::Upload;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
<span class="c">/// Row counts per lane, for placing one object's rows.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">struct</span> Counts {
    <span class="k">pub</span> verts: u32, <span class="c">// mesh vertices</span>
    <span class="k">pub</span> faces: u32, <span class="c">// solid face indices</span>
    <span class="k">pub</span> print: u32, <span class="c">// sheet fill indices</span>
    <span class="k">pub</span> text: u32, <span class="c">// sheet lettering indices</span>
    <span class="k">pub</span> sources: u32, <span class="c">// source faces</span>
    <span class="k">pub</span> pipes: u32, <span class="c">// line segments drawn as pipes</span>
    <span class="k">pub</span> ribbons: u32, <span class="c">// line segments drawn flat</span>
    <span class="k">pub</span> spheres: u32, <span class="c">// vertex markers</span>
    <span class="k">pub</span> dots: u32, <span class="c">// flat dots</span>
}

<span class="k">impl</span> Counts {
    <span class="c">/// Counts of one upload.</span>
    <span class="k">pub</span> <span class="k">fn</span> of(up: &amp;Upload) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            verts: up.arena.verts.len() <span class="k">as</span> u32,
            faces: up.arena.idx.len() <span class="k">as</span> u32,
            print: up.arena.idx_print.len() <span class="k">as</span> u32,
            text: up.arena.idx_text.len() <span class="k">as</span> u32,
            sources: up.arena.face_sources.len() <span class="k">as</span> u32,
            pipes: up.seg.pipes.len() <span class="k">as</span> u32,
            ribbons: up.seg.ribbons.len() <span class="k">as</span> u32,
            spheres: up.glyph.spheres.len() <span class="k">as</span> u32,
            dots: up.glyph.dots.len() <span class="k">as</span> u32,
        }
    }

    <span class="c">/// Add two counts.</span>
    <span class="k">pub</span> <span class="k">fn</span> plus(<span class="k">self</span>, other: <span class="k">Self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            verts: <span class="k">self</span>.verts + other.verts,
            faces: <span class="k">self</span>.faces + other.faces,
            print: <span class="k">self</span>.print + other.print,
            text: <span class="k">self</span>.text + other.text,
            sources: <span class="k">self</span>.sources + other.sources,
            pipes: <span class="k">self</span>.pipes + other.pipes,
            ribbons: <span class="k">self</span>.ribbons + other.ribbons,
            spheres: <span class="k">self</span>.spheres + other.spheres,
            dots: <span class="k">self</span>.dots + other.dots,
        }
    }

    <span class="c">/// Subtract two counts.</span>
    <span class="k">pub</span> <span class="k">fn</span> minus(<span class="k">self</span>, other: <span class="k">Self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            verts: <span class="k">self</span>.verts - other.verts,
            faces: <span class="k">self</span>.faces - other.faces,
            print: <span class="k">self</span>.print - other.print,
            text: <span class="k">self</span>.text - other.text,
            sources: <span class="k">self</span>.sources - other.sources,
            pipes: <span class="k">self</span>.pipes - other.pipes,
            ribbons: <span class="k">self</span>.ribbons - other.ribbons,
            spheres: <span class="k">self</span>.spheres - other.spheres,
            dots: <span class="k">self</span>.dots - other.dots,
        }
    }
}

#[derive(Clone, Copy)]
<span class="c">/// Where one object's rows sit: first row and row count per lane.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">struct</span> Span {
    <span class="k">pub</span> start: Counts, <span class="c">// first row per lane</span>
    <span class="k">pub</span> count: Counts, <span class="c">// rows per lane</span>
}</code></pre></div>
<h2 id="step-22-srcenginegpusegmentsrs">Step 22 · src/engine/gpu/segments.rs<a class="anchor" href="#/course/30-layer-tree#step-22-srcenginegpusegmentsrs" aria-label="Link to this section">#</a></h2>
<p>Patch boundary pipes and other stroke ranges in place.</p>
<p><code>lessons/30/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/29/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Overwrite pipe rows starting at \`first\`.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch_pipes(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, first: u32, up: &amp;SegRows) {
        <span class="k">let</span> pipes = joined_rows(&amp;up.pipes, &amp;up.pipe_chains, first);
        <span class="k">self</span>.pipes.buf.write_at(ctx, first, &amp;pipes);
    }

    <span class="c">/// Overwrite one object's rows in place.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, at: super::patch::Counts, up: &amp;SegRows) {
        <span class="k">let</span> pipes = joined_rows(&amp;up.pipes, &amp;up.pipe_chains, at.pipes);
        <span class="k">self</span>.pipes.buf.write_at(ctx, at.pipes, &amp;pipes);
        <span class="k">let</span> <span class="k">mut</span> ids = up.pipe_ids.clone();
        ids.resize(up.pipes.len(), u32::MAX);
        <span class="k">self</span>.pipes.ids.write_at(ctx, at.pipes, &amp;ids);
        <span class="k">let</span> ribbons = joined_rows(&amp;up.ribbons, &amp;up.ribbon_chains, at.ribbons);
        <span class="k">self</span>.ribbons.buf.write_at(ctx, at.ribbons, &amp;ribbons);
        <span class="k">let</span> <span class="k">mut</span> ids = up.ribbon_ids.clone();
        ids.resize(up.ribbons.len(), u32::MAX);
        <span class="k">self</span>.ribbons.ids.write_at(ctx, at.ribbons, &amp;ids);
    }

    <span class="c">/// Sheet of a GPU ribbon row: (object row, segment index).</span></code></pre></div>
<h2 id="step-23-srcshadersglyphwgsl">Step 23 · src/shaders/glyph.wgsl<a class="anchor" href="#/course/30-layer-tree#step-23-srcshadersglyphwgsl" aria-label="Link to this section">#</a></h2>
<p>Apply object color overrides to marker colors.</p>
<p><code>lessons/30/src/shaders/glyph.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = g.color * inst.color;</code> in <code>lessons/29/src/shaders/glyph.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = object_color(g.color, inst);</code></pre></div>
<h2 id="step-24-srcshadersribbonwgsl">Step 24 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/30-layer-tree#step-24-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>Apply object color overrides to strokes.</p>
<p><code>lessons/30/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = unpack4x8unorm(seg.color) * inst.color;</code> in <code>lessons/29/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = object_color(unpack4x8unorm(seg.color), inst);</code></pre></div>
<h2 id="step-25-srcshadersscenewgsl">Step 25 · src/shaders/scene.wgsl<a class="anchor" href="#/course/30-layer-tree#step-25-srcshadersscenewgsl" aria-label="Link to this section">#</a></h2>
<p>Choose between the authored color and the object override using the instance flag.</p>
<p><code>lessons/30/src/shaders/scene.wgsl</code> · edit · type this</p>
<p>Added after the line <code>const FLAG_SINGLE: u32 = 128u;</code> in <code>lessons/29/src/shaders/scene.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">const</span> FLAG_COLOR: <span class="k">u32</span> = <span class="s">256u</span>;<span class="c"> // use the layer color</span>

<span class="c">// layer colour if FLAG_COLOR, else the authored one</span>
<span class="k">fn</span> object_color(authored: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, inst: Instance) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> select(authored * inst.color, <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(inst.color.rgb, authored.a), (inst.flags &amp; FLAG_COLOR) != <span class="s">0u</span>);
}</code></pre></div>
<h2 id="step-26-srcshadersspherewgsl">Step 26 · src/shaders/sphere.wgsl<a class="anchor" href="#/course/30-layer-tree#step-26-srcshadersspherewgsl" aria-label="Link to this section">#</a></h2>
<p>Apply object color overrides to sphere markers.</p>
<p><code>lessons/30/src/shaders/sphere.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = g.color * inst.color;</code> in <code>lessons/29/src/shaders/sphere.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = object_color(g.color, inst);</code></pre></div>
<h2 id="step-27-srcshaderssplatwgsl">Step 27 · src/shaders/splat.wgsl<a class="anchor" href="#/course/30-layer-tree#step-27-srcshaderssplatwgsl" aria-label="Link to this section">#</a></h2>
<p>Apply object color overrides to cloud splats.</p>
<p><code>lessons/30/src/shaders/splat.wgsl</code> · edit · type this</p>
<p>Added after the line <code>var rgba = unpack4x8unorm(colors[i]) * tint;</code> in <code>lessons/29/src/shaders/splat.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // FLAG_COLOR: the layer color replaces the point's</span>
    <span class="k">if</span> ((table[base + <span class="s">38u</span>] &amp; <span class="s">256u</span>) != <span class="s">0u</span>) {
        rgba = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(tint.rgb, rgba.a);
    }</code></pre></div>
<h2 id="step-28-srcshaderstrianglewgsl">Step 28 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/30-layer-tree#step-28-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>Apply object color overrides to triangle surfaces.</p>
<p><code>lessons/30/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = in.color.rgb * inst.color.rgb;</code> in <code>lessons/29/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = object_color(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color.rgb, <span class="s">1.0</span>), inst).rgb;</code></pre></div>
<h2 id="step-29-srcstaters">Step 29 · src/state.rs<a class="anchor" href="#/course/30-layer-tree#step-29-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Exclude locked rows from picking and clear stale preview state after scene changes.</p>
<p><code>lessons/30/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn select(&amp;mut self, row: Option&lt;u32&gt;) {</code> in <code>lessons/29/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> row = row.filter(|row| <span class="k">self</span>.scene.selectable(*row));</code></pre></div>
<p>Added after the line <code>fn apply_pick(&amp;mut self, pick: Option&lt;Pick&gt;) {</code> in <code>lessons/29/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> pick = pick.filter(|pick| <span class="k">self</span>.scene.selectable(pick.row));

        <span class="c">// a point-cloud query takes the answer</span></code></pre></div>
<h2 id="step-30-srcstateeditrs">Step 30 · src/state/edit.rs<a class="anchor" href="#/course/30-layer-tree#step-30-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Update cached surface samples during dragging and restore source rendering on cancellation.</p>
<p><code>lessons/30/src/state/edit.rs</code> · edit · type this</p>
<p>Delete the 4 lines from <code>if active.target.is_some() {</code> in <code>lessons/29/src/state/edit.rs</code>.</p>
<p>Replaces the line <code>self.scene.rebuild(&amp;mut self.gpu);</code> in <code>lessons/29/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.restore_source_render(active.row);</code></pre></div>
<p>Replaces the line <code>self.scene.rebuild(&amp;mut self.gpu);</code> in <code>lessons/29/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">self</span>.restore_source_render(active.row);</code></pre></div>
<p>Added after the line <code>Command::Model(command) =&gt; {</code> in <code>lessons/29/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">use</span> <span class="k">crate</span>::app::modeling::Modeling;
                <span class="c">// a new object, as opposed to an edit</span>
                <span class="k">let</span> created = matches!(
                    command,
                    Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyline(_)
                );
                <span class="k">self</span>.scene.model(&amp;command)?;
                <span class="k">self</span>.after_history();

                <span class="k">if</span> created {
                    <span class="c">// select the new object: the last row of its document</span>
                    <span class="k">if</span> <span class="k">let</span> Some(doc) = <span class="k">self</span>.scene.created_doc {
                        <span class="k">let</span> row = (<span class="s">0</span>..<span class="k">self</span>.gpu.objects.len())
                            .rev()
                            .find(|&amp;row| <span class="k">self</span>.scene.identity_of(row).is_some_and(|id| id.<span class="s">0</span> == doc));
                        <span class="k">self</span>.select(row);
                    }

                    <span class="k">let</span> name = <span class="k">match</span> command {
                        Modeling::Point(_) =&gt; &quot;<span class="s">point</span>&quot;,
                        Modeling::Line(..) =&gt; &quot;<span class="s">line</span>&quot;,
                        _ =&gt; &quot;<span class="s">polyline</span>&quot;,
                    };
                    Ok(format!(
                        &quot;<span class="s">Created and selected </span>{<span class="s">name</span>}<span class="s">. Type Fit to locate it; Undo to remove it.</span>&quot;
                    ))
                } <span class="k">else</span> {
                    Ok(&quot;<span class="s">geometry updated</span>&quot;.into())
                }</code></pre></div>
<p>Replaces the 9 lines from <code>let mut rows: Vec&lt;crate::app::feedback::LayerRow&gt; = layer…</code> in <code>lessons/29/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> rows = Vec::new();</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/29/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> State {
    <span class="c">/// Draw \`row\` from its document geometry again, dropping any preview.</span>
    <span class="k">fn</span> restore_source_render(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) {
        <span class="k">let</span> geometry = <span class="k">self</span>.scene.geometry(row).cloned();
        <span class="c">// patch in place when possible, else rebuild everything</span>

        <span class="k">if</span> !geometry.is_some_and(|geometry| <span class="k">self</span>.scene.patch_preview(row, &amp;geometry, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu))
        {
            <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        }
    }
}</code></pre></div>
<h2 id="step-31-srcstatepanelrs">Step 31 · src/state/panel.rs<a class="anchor" href="#/course/30-layer-tree#step-31-srcstatepanelrs" aria-label="Link to this section">#</a></h2>
<p>Route bulbs, locks and swatches through the selected tree node and its descendants.</p>
<p><code>lessons/30/src/state/panel.rs</code> · edit · type this</p>
<p>Added after the line <code>self.toggle_layer(layer);</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// color/&lt;node&gt;/&lt;face|edge&gt;/&lt;hex or original&gt;</span>
        <span class="k">if</span> <span class="k">let</span> Some(value) = key.strip_prefix(&quot;<span class="s">color/</span>&quot;) {
            <span class="k">if</span> <span class="k">let</span> Some((index, hex)) = value.split_once('<span class="s">/</span>')
                &amp;&amp; <span class="k">let</span> (Ok(index), Ok(rgb)) = (index.parse::&lt;usize&gt;(), u32::from_str_radix(hex, <span class="s">16</span>))
                &amp;&amp; index &lt; <span class="k">self</span>.hierarchy.nodes.len()
            {
                <span class="k">let</span> color = [(rgb &gt;&gt; <span class="s">16</span>) <span class="k">as</span> u8, (rgb &gt;&gt; <span class="s">8</span>) <span class="k">as</span> u8, rgb <span class="k">as</span> u8];

                <span class="k">for</span> row <span class="k">in</span> <span class="k">self</span>.hierarchy.targets(index) {
                    <span class="k">if</span> <span class="k">let</span> Some(id) = <span class="k">self</span>.scene.identity_of(row) {
                        <span class="k">self</span>.scene.colors.insert(id, color);
                        <span class="k">self</span>.gpu.set_object_color(row, color);
                    }
                }

                <span class="k">self</span>.refresh_layers();
                <span class="k">self</span>.touch();
            }

            <span class="k">return</span>;
        }

        <span class="c">// &lt;action&gt;/&lt;node index&gt;</span></code></pre></div>
<p>Replaces the 5 lines from <code>if self</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        <span class="k">if</span> <span class="k">self</span>.scene.identity_of(row).is_some_and(|id| {
                            !<span class="k">self</span>.scene.hidden.contains(&amp;id) &amp;&amp; !<span class="k">self</span>.scene.locked.contains(&amp;id)
                        }) {</code></pre></div>
<p>Added after the line <code>self.select(Some(row));</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                &quot;<span class="s">lock</span>&quot; =&gt; {
                    <span class="k">let</span> rows = <span class="k">self</span>.hierarchy.targets(index);
                    <span class="k">let</span> lock = rows.iter().any(|row| <span class="k">self</span>.scene.selectable(*row)); <span class="c">// anything unlocked: lock all</span>

                    <span class="c">// a locked row cannot stay selected</span>
                    <span class="k">if</span> lock
                        &amp;&amp; (<span class="k">self</span>
                            .scene
                            .selected
                            .is_some_and(|row| rows.binary_search(&amp;row).is_ok())
                            || <span class="k">self</span>
                                .hierarchy
                                .selected
                                .iter()
                                .any(|row| rows.binary_search(row).is_ok()))
                    {
                        <span class="k">self</span>.select(None);
                    }

                    <span class="k">for</span> row <span class="k">in</span> rows {
                        <span class="k">if</span> <span class="k">let</span> Some(id) = <span class="k">self</span>.scene.identity_of(row) {
                            <span class="k">if</span> lock {
                                <span class="k">self</span>.scene.locked.insert(id);
                            } <span class="k">else</span> {
                                <span class="k">self</span>.scene.locked.remove(&amp;id);
                            }
                        }
                    }
                }</code></pre></div>
<p>Replaces the 7 lines from <code>let indent = &quot;  &quot;.repeat(node.depth.min(16));</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> targets = &amp;<span class="k">self</span>.hierarchy.rows[node.rows.clone()];
            <span class="k">let</span> locked = targets.iter().all(|row| !<span class="k">self</span>.scene.selectable(*row)); <span class="c">// every row locked?</span>
            <span class="c">// the shared color, when every row has the same one</span>
            <span class="k">let</span> first_color = targets
                .first()
                .and_then(|row| <span class="k">self</span>.scene.identity_of(*row))
                .and_then(|id| <span class="k">self</span>.scene.colors.get(&amp;id).copied());
            <span class="k">let</span> color = first_color.filter(|first| {
                targets.iter().all(|row| {
                    <span class="k">self</span>.scene
                        .identity_of(*row)
                        .is_some_and(|id| <span class="k">self</span>.scene.colors.get(&amp;id) == Some(first))
                })
            });
            rows.push(LayerRow {
                key: format!(&quot;<span class="s">select/</span>{<span class="s">index</span>}&quot;),
                label: node.label.clone(),
                count,
                hidden,
                locked,
                color,
                depth: node.depth,
                expanded: (node.end &gt; index + <span class="s">1</span>).then(|| <span class="k">self</span>.hierarchy.open.contains(&amp;index)),</code></pre></div>
<p>Added after the line <code>hidden: false,</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    ..Default::default()</code></pre></div>
<p>Added after the line <code>hidden: false,</code> in <code>lessons/29/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                ..Default::default()</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/30-layer-tree#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/30/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: a point is visible after Fit, layer locks prevent selection, and a successful modeling command reports <strong>geometry updated</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-layers-desktop.png" alt="Full viewer result for lesson 30" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The entire scene rebuilds while dragging: cached UV samples are not used.</li>
<li>Locks vanish after Open: only visibility is serialized.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/30-layer-tree#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/30/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
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
│   │   ├── points.rs  ~
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── deform.rs  ~
│   ├── edit.rs  ~
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs  ~
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
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── session_io.rs  ~
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── surface_preview.rs  +
│   ├── touch.rs
│   ├── ui.rs  ~
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs
│   │   ├── buffers.rs  ~
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs  ~
│   │   ├── frame.rs
│   │   ├── glyphs.rs  ~
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── patch.rs  +
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs
│   │   ├── segments.rs  ~
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
│   ├── glyph.wgsl  ~
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl  ~
│   ├── sphere.wgsl  ~
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl  ~
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── panel.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source surface samples → preview ranges → GPU patches; layer actions → retained settings.
Every file at this point: <code>lessons/30/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/30-layer-tree#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/31-splitting">31 · Split curves and faces while keeping the shell joined</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/30-layer-tree#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The completed viewer has one right-hand layer tree. Bulbs control visibility, locks prevent selection, and swatches change object and child colors. The command dock spans the bottom and displays syntax hints. Source-shell dragging updates the existing preview throughout the gesture; Save/Open retains geometry, visibility, locks and colors.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-layers-desktop.png"><img src="/session/docs/course/docs/screenshots/extensions-layers-desktop.png" alt="Full viewer result for lesson 30" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcappdeformrs",text:"Step 2 · src/app/deform.rs"},{level:2,id:"step-3-srcappeditrs",text:"Step 3 · src/app/edit.rs"},{level:2,id:"step-4-srcappfeedbackrs",text:"Step 4 · src/app/feedback.rs"},{level:2,id:"step-5-srcapphierarchyrs",text:"Step 5 · src/app/hierarchy.rs"},{level:2,id:"step-6-srcappinspectionrs",text:"Step 6 · src/app/inspection.rs"},{level:2,id:"step-7-srcappmodrs",text:"Step 7 · src/app/mod.rs"},{level:2,id:"step-8-srcappsceners",text:"Step 8 · src/app/scene.rs"},{level:2,id:"step-9-srcappsession_iors",text:"Step 9 · src/app/session_io.rs"},{level:2,id:"step-10-srcappsurface_previewrs",text:"Step 10 · src/app/surface_preview.rs"},{level:2,id:"step-11-srcappuirs",text:"Step 11 · src/app/ui.rs"},{level:2,id:"step-12-srcappwalkbreprs",text:"Step 12 · src/app/walk/brep.rs"},{level:2,id:"step-13-srcappwalkpointsrs",text:"Step 13 · src/app/walk/points.rs"},{level:2,id:"step-14-srcenginegpuarenars",text:"Step 14 · src/engine/gpu/arena.rs"},{level:2,id:"step-15-srcenginegpubuffersrs",text:"Step 15 · src/engine/gpu/buffers.rs"},{level:2,id:"step-16-srcenginegpufacesrs",text:"Step 16 · src/engine/gpu/faces.rs"},{level:2,id:"step-17-srcenginegpuglyphsrs",text:"Step 17 · src/engine/gpu/glyphs.rs"},{level:2,id:"step-18-srcenginegpuinstancers",text:"Step 18 · src/engine/gpu/instance.rs"},{level:2,id:"step-19-srcenginegpumodrs",text:"Step 19 · src/engine/gpu/mod.rs"},{level:2,id:"step-20-srcenginegpuobjectsrs",text:"Step 20 · src/engine/gpu/objects.rs"},{level:2,id:"step-21-srcenginegpupatchrs",text:"Step 21 · src/engine/gpu/patch.rs"},{level:2,id:"step-22-srcenginegpusegmentsrs",text:"Step 22 · src/engine/gpu/segments.rs"},{level:2,id:"step-23-srcshadersglyphwgsl",text:"Step 23 · src/shaders/glyph.wgsl"},{level:2,id:"step-24-srcshadersribbonwgsl",text:"Step 24 · src/shaders/ribbon.wgsl"},{level:2,id:"step-25-srcshadersscenewgsl",text:"Step 25 · src/shaders/scene.wgsl"},{level:2,id:"step-26-srcshadersspherewgsl",text:"Step 26 · src/shaders/sphere.wgsl"},{level:2,id:"step-27-srcshaderssplatwgsl",text:"Step 27 · src/shaders/splat.wgsl"},{level:2,id:"step-28-srcshaderstrianglewgsl",text:"Step 28 · src/shaders/triangle.wgsl"},{level:2,id:"step-29-srcstaters",text:"Step 29 · src/state.rs"},{level:2,id:"step-30-srcstateeditrs",text:"Step 30 · src/state/edit.rs"},{level:2,id:"step-31-srcstatepanelrs",text:"Step 31 · src/state/panel.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
