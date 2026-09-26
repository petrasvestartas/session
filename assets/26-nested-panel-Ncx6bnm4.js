const s={title:"26 · Build the nested session and graph panel",html:`<h1 id="26-build-the-nested-session-and-graph-panel">26 · Build the nested session and graph panel<a class="anchor" href="#/course/26-nested-panel#26-build-the-nested-session-and-graph-panel" aria-label="Link to this section">#</a></h1>
<p>The layer panel expands nested groups and selects or hides their descendant objects.</p>
<h2 id="step-1-indexhtml">Step 1 · index.html<a class="anchor" href="#/course/26-nested-panel#step-1-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/26/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>style=&quot;position: fixed; top: 12px; left: 12px; min-width:…</code> in <code>lessons/25/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>       style=&quot;position: fixed; top: 12px; left: 12px; width: min(320px,80vw); max-height: 70vh; overflow: auto; color: #fff; background: #222d; font: 13px/1.7 system-ui; padding: 6px 0; border-radius: 4px; outline: 1px solid #555;&quot;&gt;&lt;/div&gt;</code></pre></div>
<h2 id="step-2-srcappfeedbackrs">Step 2 · src/app/feedback.rs<a class="anchor" href="#/course/26-nested-panel#step-2-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Carry status and layer information from the scene into the interface.</p>
<p><code>lessons/26/src/app/feedback.rs</code> · edit · type this</p>
<p>Replaces the line <code>&quot;display:block;width:100%;text-align:left;border:0;backgr…</code> in <code>lessons/25/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:pre;overflow:hidden;text-overflow:ellipsis;opacity:1</span>&quot;,
        );

        <span class="k">if</span> row.hidden {
            <span class="k">let</span> _ = line.set_attribute(
                &quot;<span class="s">style</span>&quot;,
                &quot;<span class="s">display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:pre;overflow:hidden;text-overflow:ellipsis;opacity:0.45</span>&quot;,</code></pre></div>
<h2 id="step-3-srcapphierarchyrs">Step 3 · src/app/hierarchy.rs<a class="anchor" href="#/course/26-nested-panel#step-3-srcapphierarchyrs" aria-label="Link to this section">#</a></h2>
<p>Index document trees and graph endpoints into bounded sets of render rows.</p>
<p><code>lessons/26/src/app/hierarchy.rs</code> · 338 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The tree panel: rows grouped by document and kind, so one click can select or hide a whole branch.</span>

<span class="k">use</span> <span class="k">crate</span>::app::scene::Scene;
<span class="k">use</span> session_rust::Session;
<span class="k">use</span> std::collections::BTreeMap;
<span class="k">use</span> std::collections::HashMap;
<span class="k">use</span> std::collections::HashSet;
<span class="k">use</span> std::ops::Range;
<span class="k">use</span> std::rc::Rc;

<span class="k">const</span> MAX_NODES: usize = <span class="s">200_000</span>; <span class="c">// most tree nodes shown</span>

<span class="k">const</span> MAX_ROWS: usize = <span class="s">1_000_000</span>; <span class="c">// most object rows indexed</span>

<span class="k">pub</span> <span class="k">const</span> PAGE_SIZE: usize = <span class="s">128</span>; <span class="c">// nodes per panel page</span>

<span class="k">type</span> Lookup = HashMap&lt;usize, HashMap&lt;Rc&lt;str&gt;, u32&gt;&gt;; <span class="c">// document -&gt; guid -&gt; row</span>

<span class="c">/// One line of the tree panel.</span>
<span class="k">pub</span> <span class="k">struct</span> Node {
    <span class="k">pub</span> label: String,      <span class="c">// text shown</span>
    <span class="k">pub</span> depth: usize,       <span class="c">// indent level</span>
    <span class="k">pub</span> end: usize,         <span class="c">// index one past its last descendant</span>
    <span class="k">pub</span> rows: Range&lt;usize&gt;, <span class="c">// its object rows, as a slice of \`Hierarchy::rows\`</span>
}

<span class="c">/// The tree panel's flattened nodes.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> Hierarchy {
    <span class="k">pub</span> nodes: Vec&lt;Node&gt;,     <span class="c">// every node, parents before children</span>
    <span class="k">pub</span> rows: Vec&lt;u32&gt;,       <span class="c">// object rows the nodes point into</span>
    <span class="k">pub</span> open: HashSet&lt;usize&gt;, <span class="c">// expanded nodes</span>
    <span class="k">pub</span> page: usize,          <span class="c">// current panel page</span>
    <span class="k">pub</span> selected: Vec&lt;u32&gt;,   <span class="c">// rows highlighted</span>
    <span class="k">pub</span> truncated: bool,      <span class="c">// scene too large to show</span>
    revision: Option&lt;u64&gt;,    <span class="c">// scene revision this was built from</span>
}

<span class="k">impl</span> Hierarchy {
    <span class="c">/// Rebuild when the scene changed.</span>
    <span class="k">pub</span> <span class="k">fn</span> refresh(&amp;<span class="k">mut</span> <span class="k">self</span>, scene: &amp;Scene) {
        <span class="k">if</span> <span class="k">self</span>.revision != Some(scene.row_revision) {
            <span class="k">self</span>.rebuild(scene);
        }
    }

    <span class="c">/// Rebuild the nodes from every document's tree.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>, scene: &amp;Scene) {
        <span class="k">self</span>.revision = Some(scene.row_revision);
        <span class="k">self</span>.nodes.clear();
        <span class="k">self</span>.rows.clear();
        <span class="k">self</span>.truncated = scene.object_count() &gt; MAX_NODES;

        <span class="k">if</span> <span class="k">self</span>.truncated {
            <span class="k">return</span>;
        }

        <span class="c">// guid to row, per document</span>
        <span class="k">let</span> <span class="k">mut</span> lookup = Lookup::new();

        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
            <span class="k">if</span> <span class="k">let</span> Some(identity) = scene.identity_of(row) {
                lookup
                    .entry(identity.<span class="s">0</span>)
                    .or_default()
                    .insert(identity.<span class="s">1</span>, row);
            }
        }

        <span class="k">for</span> (doc, file) <span class="k">in</span> scene.docs.iter().enumerate() {
            <span class="k">let</span> start = <span class="k">self</span>.nodes.len();
            <span class="k">let</span> rows = <span class="k">self</span>.rows.len();

            <span class="k">if</span> !<span class="k">self</span>.tree(scene, doc, &amp;lookup)
                || !<span class="k">self</span>.graph(&amp;file.session, doc, &amp;file.name, &amp;lookup)
            {
                <span class="k">self</span>.nodes.truncate(start);
                <span class="k">self</span>.rows.truncate(rows);
                <span class="k">self</span>.truncated = <span class="s">true</span>;
                <span class="k">break</span>;
            }
        }

        <span class="k">self</span>.open.retain(|index| *index &lt; <span class="k">self</span>.nodes.len());
    }

    <span class="c">/// Add one document and its tree; false when a limit is hit.</span>
    <span class="k">fn</span> tree(&amp;<span class="k">mut</span> <span class="k">self</span>, scene: &amp;Scene, doc: usize, lookup: &amp;Lookup) -&gt; bool {
        <span class="k">let</span> file = &amp;scene.docs[doc];
        <span class="k">let</span> start = <span class="k">self</span>.nodes.len();

        <span class="k">if</span> !<span class="k">self</span>.push(&amp;file.name, <span class="s">0</span>) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> seen = HashSet::new(); <span class="c">// nodes visited</span>
        <span class="k">let</span> <span class="k">mut</span> stack = Vec::new(); <span class="c">// (node, depth, index to close)</span>

        <span class="k">if</span> <span class="k">let</span> Some(root) = file.session.tree.root() {
            stack.push((root, <span class="s">1</span>, None));
        }

        <span class="c">// depth first; a node is pushed again to close it after its children</span>
        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..MAX_NODES * <span class="s">2</span> {
            <span class="k">let</span> Some((node, depth, exit)) = stack.pop() <span class="k">else</span> {
                <span class="k">break</span>;
            };

            <span class="k">if</span> <span class="k">let</span> Some(index) = exit {
                <span class="k">self</span>.finish(index);
                <span class="k">continue</span>;
            }

            <span class="k">if</span> !seen.insert(Rc::as_ptr(&amp;node)) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> borrowed = node.borrow();
            <span class="k">let</span> row = row_of(lookup, doc, &amp;borrowed.name);
            <span class="k">let</span> label = row.map(|r| scene.object_name(r)).unwrap_or(&amp;borrowed.name);
            <span class="k">let</span> index = <span class="k">self</span>.nodes.len();

            <span class="k">if</span> !<span class="k">self</span>.push(label, depth) {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">if</span> <span class="k">let</span> Some(row) = row {
                <span class="k">self</span>.rows.push(row);
            }

            stack.push((Rc::clone(&amp;node), depth, Some(index)));
            <span class="k">let</span> children = borrowed.children();

            <span class="k">if</span> stack.len() + children.len() &gt; MAX_NODES {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">for</span> child <span class="k">in</span> children.into_iter().rev() {
                stack.push((child, depth + <span class="s">1</span>, None));
            }
        }

        <span class="k">if</span> !stack.is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.rows.len() == <span class="k">self</span>.nodes[start].rows.start {
            <span class="c">// objects outside the tree go under the document</span>
            <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
                <span class="k">if</span> scene
                    .identity_of(row)
                    .is_some_and(|(owner, _)| owner == doc)
                {
                    <span class="k">self</span>.rows.push(row);
                }
            }
        }

        <span class="k">self</span>.finish(start);
        <span class="k">self</span>.rows.len() &lt;= MAX_ROWS
    }

    <span class="k">fn</span> graph(&amp;<span class="k">mut</span> <span class="k">self</span>, session: &amp;Session, doc: usize, name: &amp;str, lookup: &amp;Lookup) -&gt; bool {
        <span class="k">let</span> vertices = session.graph.number_of_vertices();
        <span class="k">let</span> edges: usize = session.graph.edges.values().map(|edges| edges.len()).sum();
        <span class="k">let</span> remaining = MAX_ROWS.saturating_sub(<span class="k">self</span>.rows.len());

        <span class="k">if</span> vertices &gt; MAX_NODES || vertices &gt; remaining || edges &gt; remaining - vertices {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> groups: BTreeMap&lt;String, Vec&lt;u32&gt;&gt; = BTreeMap::new();

        <span class="k">for</span> vertex <span class="k">in</span> session.graph.get_vertices() {
            <span class="k">if</span> <span class="k">let</span> Some(row) = row_of(lookup, doc, &amp;vertex.name) {
                groups
                    .entry(format!(&quot;<span class="s">vertex: </span>{}&quot;, vertex.attribute))
                    .or_default()
                    .push(row);
            }
        }

        <span class="k">for</span> (from, edges) <span class="k">in</span> &amp;session.graph.edges {
            <span class="k">for</span> (to, edge) <span class="k">in</span> edges {
                <span class="k">if</span> from &gt; to {
                    <span class="k">continue</span>;
                }

                <span class="k">let</span> rows = groups
                    .entry(format!(&quot;<span class="s">edge: </span>{}&quot;, edge.attribute))
                    .or_default();

                <span class="k">for</span> guid <span class="k">in</span> [from, to] {
                    <span class="k">if</span> <span class="k">let</span> Some(row) = row_of(lookup, doc, guid) {
                        rows.push(row);
                    }

                    <span class="k">if</span> from == to {
                        <span class="k">break</span>;
                    }
                }
            }
        }

        <span class="k">for</span> (label, <span class="k">mut</span> rows) <span class="k">in</span> groups {
            rows.sort_unstable();
            rows.dedup();
            <span class="k">let</span> index = <span class="k">self</span>.nodes.len();

            <span class="k">if</span> !<span class="k">self</span>.push(&amp;format!(&quot;{<span class="s">name</span>}<span class="s"> / </span>{<span class="s">label</span>}&quot;), <span class="s">0</span>) {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">self</span>.rows.extend(rows);
            <span class="k">self</span>.finish(index);
        }

        <span class="s">true</span>
    }

    <span class="c">/// Add one open node; false at the limit.</span>
    <span class="k">fn</span> push(&amp;<span class="k">mut</span> <span class="k">self</span>, label: &amp;str, depth: usize) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.nodes.len() &gt;= MAX_NODES {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> start = <span class="k">self</span>.rows.len();
        <span class="k">let</span> label = label.chars().take(<span class="s">160</span>).collect();
        <span class="k">self</span>.nodes.push(Node {
            label,
            depth,
            end: <span class="s">0</span>,
            rows: start..start,
        });
        <span class="s">true</span>
    }

    <span class="c">/// Close node \`index\` at the current end.</span>
    <span class="k">fn</span> finish(&amp;<span class="k">mut</span> <span class="k">self</span>, index: usize) {
        <span class="k">self</span>.nodes[index].end = <span class="k">self</span>.nodes.len();
        <span class="k">self</span>.nodes[index].rows.end = <span class="k">self</span>.rows.len();
    }

    <span class="c">/// The nodes shown, skipping closed subtrees.</span>
    <span class="k">pub</span> <span class="k">fn</span> visible(&amp;<span class="k">self</span>) -&gt; Vec&lt;usize&gt; {
        <span class="k">let</span> <span class="k">mut</span> result = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> index = <span class="s">0</span>;

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.nodes.len() {
            <span class="k">let</span> Some(node) = <span class="k">self</span>.nodes.get(index) <span class="k">else</span> {
                <span class="k">break</span>;
            };
            result.push(index);
            index = <span class="k">if</span> <span class="k">self</span>.open.contains(&amp;index) {
                index + <span class="s">1</span>
            } <span class="k">else</span> {
                node.end
            };
        }

        result
    }

    <span class="c">/// The object rows under node \`index\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> targets(&amp;<span class="k">self</span>, index: usize) -&gt; Vec&lt;u32&gt; {
        <span class="k">let</span> Some(node) = <span class="k">self</span>.nodes.get(index) <span class="k">else</span> {
            <span class="k">return</span> Vec::new();
        };
        <span class="k">let</span> <span class="k">mut</span> rows = <span class="k">self</span>.rows[node.rows.clone()].to_vec();
        rows.sort_unstable();
        rows.dedup();
        rows
    }
}

<span class="c">/// The row of one guid in one document.</span>
<span class="k">fn</span> row_of(lookup: &amp;Lookup, doc: usize, guid: &amp;str) -&gt; Option&lt;u32&gt; {
    lookup.get(&amp;doc)?.get(guid).copied()
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::scene::FileDoc;
    <span class="k">use</span> session_rust::Point;
    <span class="k">use</span> session_rust::Session;
    <span class="k">use</span> session_rust::Xform;

    <span class="c">/// A full row table refuses the graph.</span>
    #[test]
    <span class="k">fn</span> graph_refuses_vertex_overflow_before_allocating_groups() {
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">budget</span>&quot;);
        session.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        session.add_point(Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> <span class="k">mut</span> index = Hierarchy::default();
        index.rows.resize(MAX_ROWS - <span class="s">1</span>, <span class="s">0</span>);
        assert!(!index.graph(&amp;session, <span class="s">0</span>, &quot;<span class="s">budget</span>&quot;, &amp;Lookup::new()));
        assert_eq!(index.rows.len(), MAX_ROWS - <span class="s">1</span>);
        assert!(index.nodes.is_empty());
    }

    <span class="c">/// Groups and edges name rows of their own document.</span>
    #[test]
    <span class="k">fn</span> nested_groups_and_graph_endpoints_are_document_scoped() {
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">test</span>&quot;);
        <span class="k">let</span> parent = session.add_group(&quot;<span class="s">parent</span>&quot;);
        <span class="k">let</span> child = session_rust::TreeNode::new(&quot;<span class="s">child</span>&quot;);
        session.add(&amp;child, Some(&amp;parent));
        <span class="k">let</span> a = session.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Some(&amp;child));
        <span class="k">let</span> b = session.add_point(Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Some(&amp;parent));
        session.add_edge(&amp;a.borrow().name, &amp;b.borrow().name, &quot;<span class="s">joint</span>&quot;);
        <span class="k">let</span> shared = Rc::new(session);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();

        <span class="k">for</span> name <span class="k">in</span> [&quot;<span class="s">first</span>&quot;, &quot;<span class="s">second</span>&quot;] {
            scene.add_file(FileDoc {
                name: name.into(),
                session: Rc::clone(&amp;shared),
                place: Xform::identity(),
                point_px: <span class="s">0</span>.<span class="s">0</span>,
                display_only: <span class="s">false</span>,
            });
        }

        <span class="k">let</span> <span class="k">mut</span> index = Hierarchy::default();
        index.rebuild(&amp;scene);
        <span class="k">let</span> parent = index
            .nodes
            .iter()
            .position(|node| node.label == &quot;<span class="s">parent</span>&quot;)
            .unwrap();
        <span class="k">let</span> child = index
            .nodes
            .iter()
            .position(|node| node.label == &quot;<span class="s">child</span>&quot;)
            .unwrap();
        assert_eq!(index.targets(parent), vec![<span class="s">0</span>, <span class="s">1</span>]);
        assert_eq!(index.targets(child), vec![<span class="s">0</span>]);
        <span class="k">let</span> edge = index
            .nodes
            .iter()
            .position(|node| node.label == &quot;<span class="s">first / edge: joint</span>&quot;)
            .unwrap();
        assert_eq!(index.targets(edge), vec![<span class="s">0</span>, <span class="s">1</span>]);
        assert!(!index.visible().contains(&amp;child));
        <span class="k">let</span> count = index.rows.len();
        index.rebuild(&amp;scene);
        assert_eq!(index.rows.len(), count);
        scene = Scene::new();
        index.rebuild(&amp;scene);
        assert!(index.rows.is_empty());
        assert!(index.nodes.is_empty());
    }
}</code></pre></div>
<h2 id="step-4-srcappinspectionrs">Step 4 · src/app/inspection.rs<a class="anchor" href="#/course/26-nested-panel#step-4-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Expose the new selection and resource state to the browser inspection data.</p>
<p><code>lessons/26/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the line <code>&quot;selected&quot;: parent,</code> in <code>lessons/25/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">selected_group_count</span>&quot;: state.selected_group_count(),
        &quot;<span class="s">hidden_count</span>&quot;: state.scene.hidden.len(),</code></pre></div>
<h2 id="step-5-srcapplayersrs">Step 5 · src/app/layers.rs<a class="anchor" href="#/course/26-nested-panel#step-5-srcapplayersrs" aria-label="Link to this section">#</a></h2>
<p>Count visible and hidden members from the layer membership sets.</p>
<p><code>lessons/26/src/app/layers.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn rows(scene: &amp;Scene) -&gt; Vec&lt;Row&gt; {</code> in <code>lessons/25/src/app/layers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The panel rows: documents, then the kinds present.</span>
<span class="k">pub</span> <span class="k">fn</span> rows(scene: &amp;Scene) -&gt; Vec&lt;Row&gt; {
    <span class="k">let</span> <span class="k">mut</span> documents = vec![(<span class="s">0</span>, <span class="s">0</span>); scene.docs.len()]; <span class="c">// (count, hidden) per document</span>
    <span class="k">let</span> <span class="k">mut</span> kinds = [(<span class="s">0</span>, <span class="s">0</span>); <span class="s">6</span>]; <span class="c">// (count, hidden) per kind</span>

    <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
        <span class="k">let</span> Some(identity) = scene.identity_of(row) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> hidden = usize::from(scene.hidden.contains(&amp;identity));

        <span class="k">if</span> <span class="k">let</span> Some(count) = documents.get_mut(identity.<span class="s">0</span>) {
            count.<span class="s">0</span> += <span class="s">1</span>;
            count.<span class="s">1</span> += hidden;
        }

        <span class="k">if</span> <span class="k">let</span> Some(geometry) = scene.geometry(row) {
            <span class="k">let</span> count = &amp;<span class="k">mut</span> kinds[Kind::of(geometry) <span class="k">as</span> usize];
            count.<span class="s">0</span> += <span class="s">1</span>;
            count.<span class="s">1</span> += hidden;
        }
    }

    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

    <span class="k">for</span> (index, &amp;(count, hidden)) <span class="k">in</span> documents.iter().enumerate() {
        <span class="k">if</span> count &gt; <span class="s">0</span> {
            out.push(Row {
                layer: Layer::Document(index),
                label: scene.docs[index].name.clone(),
                count,
                hidden: hidden == count,
            });
        }
    }

    <span class="k">for</span> kind <span class="k">in</span> [
        Kind::Solids,
        Kind::Surfaces,
        Kind::Meshes,
        Kind::Curves,
        Kind::Points,
        Kind::Clouds,
    ] {
        <span class="k">let</span> (count, hidden) = kinds[kind <span class="k">as</span> usize];

        <span class="k">if</span> count &gt; <span class="s">0</span> {
            out.push(Row {
                layer: Layer::Kind(kind),
                label: kind.label().into(),
                count,
                hidden: hidden == count,
            });
        }</code></pre></div>
<p>Delete the <code>fn all_hidden</code> block from <code>lessons/25/src/app/layers.rs</code>.</p>
<p>Added after the line <code>#[test]</code> in <code>lessons/25/src/app/layers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Row counts match the rows a layer controls.</span>
    #[test]
    <span class="k">fn</span> bucket_counts_match_membership_with_mixed_visibility() {
        <span class="k">let</span> <span class="k">mut</span> scene = scene_with_two_files();
        scene.hidden.insert(scene.identity_of(<span class="s">0</span>).unwrap());
        scene.hidden.insert(scene.identity_of(<span class="s">2</span>).unwrap());

        <span class="k">for</span> row <span class="k">in</span> rows(&amp;scene) {
            <span class="k">let</span> members = of_layer(&amp;scene, row.layer);
            assert_eq!(row.count, members.len());
            <span class="k">let</span> hidden = members
                .iter()
                .all(|row| scene.hidden.contains(&amp;scene.identity_of(*row).unwrap()));
            assert_eq!(row.hidden, hidden);
        }
    }

    <span class="c">/// A kind spans documents; a document is only its own.</span></code></pre></div>
<h2 id="step-6-srcappmodrs">Step 6 · src/app/mod.rs<a class="anchor" href="#/course/26-nested-panel#step-6-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/26/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod gizmo;</code> in <code>lessons/25/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> hierarchy;</code></pre></div>
<h2 id="step-7-srcappsceners">Step 7 · src/app/scene.rs<a class="anchor" href="#/course/26-nested-panel#step-7-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Map source trees and graph endpoints to row identities, and track revisions as documents change.</p>
<p><code>lessons/26/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the line <code>pub(crate) created_doc: Option&lt;usize&gt;,</code> in <code>lessons/25/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) row_revision: u64,                      <span class="c">// bumped when rows change</span></code></pre></div>
<p>Added after the line <code>created_doc: None,</code> in <code>lessons/25/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            row_revision: <span class="s">0</span>,</code></pre></div>
<p>Added after the line <code>fn reset_rows(&amp;mut self) {</code> in <code>lessons/25/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.row_revision = <span class="k">self</span>.row_revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<p>Added after the line <code>pub(super) fn push_row(&amp;mut self, owner: usize, guid: &amp;st…</code> in <code>lessons/25/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.row_revision = <span class="k">self</span>.row_revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<p>Added after the line <code>pub fn add_file(&amp;mut self, doc: FileDoc) {</code> in <code>lessons/25/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.row_revision = <span class="k">self</span>.row_revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<h2 id="step-8-srclibrs">Step 8 · src/lib.rs<a class="anchor" href="#/course/26-nested-panel#step-8-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Connect browser events, scene changes and drawing through the application state.</p>
<p><code>lessons/26/src/lib.rs</code> · edit · type this</p>
<p>Replaces the 3 lines from <code>if let Some(layer) = app::layers::Layer::from_key(&amp;key) {</code> in <code>lessons/25/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                state.panel_action(&amp;key);</code></pre></div>
<h2 id="step-9-srcstaters">Step 9 · src/state.rs<a class="anchor" href="#/course/26-nested-panel#step-9-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Own the hierarchy index, refresh its labels and keep group selection separate from one selected object.</p>
<p><code>lessons/26/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod edit;</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> panel;</code></pre></div>
<p>Added after the line <code>pub selection: SelectionMode,</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    hierarchy: <span class="k">crate</span>::app::hierarchy::Hierarchy,</code></pre></div>
<p>Added after the line <code>selection: SelectionMode::Object,</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            hierarchy: Default::default(),</code></pre></div>
<p>Added after the line <code>self.cancel_gesture();</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hierarchy = Default::default();</code></pre></div>
<p>Replaces the 2 lines from <code>self.cancel_gesture();</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// unhighlight the old selection</span>
        <span class="k">for</span> old <span class="k">in</span> <span class="k">self</span>.hierarchy.selected.drain(..) {
            <span class="k">self</span>.gpu.set_selected(old, <span class="s">false</span>);
        }

        <span class="c">// back to plain object mode, no edge, face or controls</span></code></pre></div>
<p>Added after the line <code>pub fn hide_selected(&amp;mut self) {</code> in <code>lessons/25/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// several rows selected</span>
        <span class="k">if</span> !<span class="k">self</span>.hierarchy.selected.is_empty() {
            <span class="k">let</span> rows = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.hierarchy.selected);

            <span class="k">for</span> row <span class="k">in</span> &amp;rows {
                <span class="k">self</span>.gpu.set_selected(*row, <span class="s">false</span>);
            }

            <span class="k">self</span>.set_rows_hidden(&amp;rows, <span class="s">true</span>);
            <span class="k">return</span>;
        }</code></pre></div>
<h2 id="step-10-srcstateeditrs">Step 10 · src/state/edit.rs<a class="anchor" href="#/course/26-nested-panel#step-10-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Apply visibility and deletion through the hierarchy, then refresh the affected rows.</p>
<p><code>lessons/26/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>if !self.scene.delete_row(row) {</code> in <code>lessons/25/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.status(&quot;<span class="s">This object cannot be deleted; streamed scenes cannot be rebuilt</span>&quot;);
            <span class="k">return</span>;
        }

        <span class="k">self</span>.after_history();</code></pre></div>
<p>Added after the line <code>fn after_history(&amp;mut self) {</code> in <code>lessons/25/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hierarchy.open.clear();
        <span class="k">self</span>.hierarchy.page = <span class="s">0</span>;</code></pre></div>
<p>Replaces the 3 lines from <code>let hidden: Vec&lt;bool&gt; = rows</code> in <code>lessons/25/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// anything still visible: hide the whole layer</span>
        <span class="k">let</span> hide = rows.iter().any(|&amp;row| {
            <span class="k">self</span>.scene
                .identity_of(row)
                .is_some_and(|id| !<span class="k">self</span>.scene.hidden.contains(&amp;id))
        });
        <span class="k">self</span>.set_rows_hidden(&amp;rows, hide);</code></pre></div>
<p>Replaces the line <code>let rows: Vec&lt;crate::app::feedback::LayerRow&gt; = layers::r…</code> in <code>lessons/25/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hierarchy.refresh(&amp;<span class="k">self</span>.scene);
        <span class="k">let</span> <span class="k">mut</span> rows: Vec&lt;<span class="k">crate</span>::app::feedback::LayerRow&gt; = layers::rows(&amp;<span class="k">self</span>.scene)</code></pre></div>
<p>Added after the line <code>.collect();</code> in <code>lessons/25/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hierarchy_labels(&amp;<span class="k">mut</span> rows);</code></pre></div>
<h2 id="step-11-srcstatepanelrs">Step 11 · src/state/panel.rs<a class="anchor" href="#/course/26-nested-panel#step-11-srcstatepanelrs" aria-label="Link to this section">#</a></h2>
<p>Turn panel actions into recursive selection and visibility updates.</p>
<p><code>lessons/26/src/state/panel.rs</code> · 183 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::layers::Layer;
<span class="k">use</span> <span class="k">crate</span>::state::State;

<span class="k">impl</span> State {
    <span class="c">/// How many rows are selected together.</span>
    <span class="k">pub</span> <span class="k">fn</span> selected_group_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.hierarchy.selected.len()
    }

    <span class="c">/// A click in the layers panel, by its key.</span>
    <span class="k">pub</span> <span class="k">fn</span> panel_action(&amp;<span class="k">mut</span> <span class="k">self</span>, key: &amp;str) {
        <span class="c">// a layer row: hide or show it</span>
        <span class="k">if</span> <span class="k">let</span> Some(layer) = Layer::from_key(key) {
            <span class="k">self</span>.toggle_layer(layer);
            <span class="k">return</span>;
        }

        <span class="c">// &lt;action&gt;/&lt;node index&gt;</span>
        <span class="k">let</span> Some((action, index)) = key.split_once('<span class="s">/</span>') <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> Ok(index) = index.parse::&lt;usize&gt;() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> action == &quot;<span class="s">page</span>&quot; {
            <span class="k">self</span>.hierarchy.page = index;
        } <span class="k">else</span> <span class="k">if</span> index &lt; <span class="k">self</span>.hierarchy.nodes.len() {
            <span class="k">match</span> action {
                &quot;<span class="s">open</span>&quot; =&gt; {
                    <span class="c">// fold or unfold the node</span>
                    <span class="k">if</span> !<span class="k">self</span>.hierarchy.open.remove(&amp;index) {
                        <span class="k">self</span>.hierarchy.open.insert(index);
                    }
                }
                &quot;<span class="s">select</span>&quot; =&gt; {
                    <span class="k">let</span> rows = <span class="k">self</span>.hierarchy.targets(index);
                    <span class="k">self</span>.select(None);

                    <span class="k">for</span> row <span class="k">in</span> rows {
                        <span class="k">if</span> <span class="k">self</span>
                            .scene
                            .identity_of(row)
                            .is_some_and(|id| !<span class="k">self</span>.scene.hidden.contains(&amp;id))
                        {
                            <span class="k">self</span>.gpu.set_selected(row, <span class="s">true</span>);
                            <span class="k">self</span>.hierarchy.selected.push(row);
                        }
                    }

                    <span class="k">if</span> <span class="k">self</span>.hierarchy.selected.len() == <span class="s">1</span> {
                        <span class="k">let</span> row = <span class="k">self</span>.hierarchy.selected[<span class="s">0</span>];
                        <span class="k">self</span>.select(Some(row));
                    }
                }
                &quot;<span class="s">hide</span>&quot; =&gt; {
                    <span class="k">let</span> rows = <span class="k">self</span>.hierarchy.targets(index);
                    <span class="c">// anything visible: hide all</span>
                    <span class="k">let</span> hide = rows.iter().any(|row| {
                        <span class="k">self</span>.scene
                            .identity_of(*row)
                            .is_some_and(|id| !<span class="k">self</span>.scene.hidden.contains(&amp;id))
                    });
                    <span class="k">self</span>.set_rows_hidden(&amp;rows, hide);
                    <span class="k">return</span>;
                }
                _ =&gt; <span class="k">return</span>,
            }
        }

        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Hide or show sorted rows.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> set_rows_hidden(&amp;<span class="k">mut</span> <span class="k">self</span>, rows: &amp;[u32], hide: bool) {
        <span class="c">// a hidden row cannot stay selected</span>
        <span class="k">if</span> hide
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
            <span class="k">let</span> Some(id) = <span class="k">self</span>.scene.identity_of(*row) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> changed = <span class="k">if</span> hide {
                <span class="k">self</span>.scene.hidden.insert(id)
            } <span class="k">else</span> {
                <span class="k">self</span>.scene.hidden.remove(&amp;id)
            };

            <span class="k">if</span> changed {
                <span class="k">self</span>.gpu.set_hidden(*row, hide);
            }
        }

        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// The rows of the layers panel for the current page.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> hierarchy_labels(&amp;<span class="k">mut</span> <span class="k">self</span>, rows: &amp;<span class="k">mut</span> Vec&lt;<span class="k">crate</span>::app::feedback::LayerRow&gt;) {
        <span class="k">use</span> <span class="k">crate</span>::app::feedback::LayerRow;
        <span class="k">use</span> <span class="k">crate</span>::app::hierarchy::PAGE_SIZE;
        <span class="k">let</span> visible = <span class="k">self</span>.hierarchy.visible(); <span class="c">// unfolded nodes</span>
        <span class="k">self</span>.hierarchy.page = <span class="k">self</span>
            .hierarchy
            .page
            .min(visible.len().saturating_sub(<span class="s">1</span>) / PAGE_SIZE); <span class="c">// keep the page in range</span>
        <span class="k">let</span> first = <span class="k">self</span>.hierarchy.page * PAGE_SIZE;

        <span class="c">// one panel row per visible node on this page</span>
        <span class="k">for</span> &amp;index <span class="k">in</span> visible.iter().skip(first).take(PAGE_SIZE) {
            <span class="k">let</span> node = &amp;<span class="k">self</span>.hierarchy.nodes[index];
            <span class="k">let</span> count = node.rows.len();
            <span class="c">// every row hidden?</span>
            <span class="k">let</span> hidden = <span class="k">self</span>.hierarchy.rows[node.rows.clone()].iter().all(|row| {
                <span class="k">self</span>.scene
                    .identity_of(*row)
                    .is_some_and(|id| <span class="k">self</span>.scene.hidden.contains(&amp;id))
            });
            <span class="k">let</span> indent = &quot;<span class="s">  </span>&quot;.repeat(node.depth.min(<span class="s">16</span>));

            <span class="k">if</span> node.end &gt; index + <span class="s">1</span> {
                <span class="k">let</span> mark = <span class="k">if</span> <span class="k">self</span>.hierarchy.open.contains(&amp;index) {
                    &quot;<span class="s">▾</span>&quot;
                } <span class="k">else</span> {
                    &quot;<span class="s">▸</span>&quot;
                };
                rows.push(LayerRow {
                    key: format!(&quot;<span class="s">open/</span>{<span class="s">index</span>}&quot;), <span class="c">// the open button key</span>
                    label: format!(&quot;{<span class="s">indent</span>}{<span class="s">mark</span>}<span class="s"> </span>{}&quot;, node.label), <span class="c">// node row text</span>
                    count,
                    hidden,
                });
            }

            rows.push(LayerRow {
                key: format!(&quot;<span class="s">select/</span>{<span class="s">index</span>}&quot;),
                label: format!(&quot;{<span class="s">indent</span>}<span class="s">Select </span>{}&quot;, node.label), <span class="c">// select row text</span>
                count,
                hidden,
            });
            rows.push(LayerRow {
                key: format!(&quot;<span class="s">hide/</span>{<span class="s">index</span>}&quot;), <span class="c">// the hide button key</span>
                label: format!(
                    &quot;{<span class="s">indent</span>}{}<span class="s"> </span>{}&quot;,
                    <span class="k">if</span> hidden { &quot;<span class="s">Show</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Hide</span>&quot; },
                    node.label
                ),
                count,
                hidden,
            });
        }

        <span class="c">// Previous and Next rows when there are more pages</span>
        <span class="k">for</span> (label, page) <span class="k">in</span> [
            (&quot;<span class="s">Previous</span>&quot;, <span class="k">self</span>.hierarchy.page.checked_sub(<span class="s">1</span>)),
            (
                &quot;<span class="s">Next</span>&quot;,
                (first + PAGE_SIZE &lt; visible.len()).then_some(<span class="k">self</span>.hierarchy.page + <span class="s">1</span>),
            ),
        ] {
            <span class="k">if</span> <span class="k">let</span> Some(page) = page {
                rows.push(LayerRow {
                    key: format!(&quot;<span class="s">page/</span>{<span class="s">page</span>}&quot;),
                    label: label.into(),
                    count: visible.len(),
                    hidden: <span class="s">false</span>,
                });
            }
        }

        <span class="k">if</span> <span class="k">self</span>.hierarchy.truncated {
            rows.push(LayerRow {
                key: String::new(),
                label: &quot;<span class="s">Tree exceeds panel capacity</span>&quot;.into(),
                count: <span class="s">0</span>,
                hidden: <span class="s">false</span>,
            });
        }
    }
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/26-nested-panel#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/26/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: expanding a group exposes its children, selecting it highlights its members, and the status area shows no error.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-panels.png" alt="Full viewer result for lesson 26" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A parent selects stale objects: the hierarchy was not refreshed after a revision.</li>
<li>An oversized tree loses members: index limits are not reported before truncation.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/26-nested-panel#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/26/src/
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
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs  +
│   ├── input.rs
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs  ~
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs  ~
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
│   ├── panel.rs  +
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source hierarchy → descendant row ranges → selection and visibility.
Every file at this point: <code>lessons/26/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/26-nested-panel#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/27-egui-interface">27 · Build the egui panel and command interface</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/26-nested-panel#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The expanded Session layers panel shows Assembly → Nested; selecting Nested highlights its two beams together. This maintained-viewer capture uses egui styling; this checkpoint uses DOM buttons with the same hierarchy and selection behavior. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-panels.png"><img src="/session/docs/course/docs/screenshots/extensions-panels.png" alt="Full viewer result for lesson 26" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-indexhtml",text:"Step 1 · index.html"},{level:2,id:"step-2-srcappfeedbackrs",text:"Step 2 · src/app/feedback.rs"},{level:2,id:"step-3-srcapphierarchyrs",text:"Step 3 · src/app/hierarchy.rs"},{level:2,id:"step-4-srcappinspectionrs",text:"Step 4 · src/app/inspection.rs"},{level:2,id:"step-5-srcapplayersrs",text:"Step 5 · src/app/layers.rs"},{level:2,id:"step-6-srcappmodrs",text:"Step 6 · src/app/mod.rs"},{level:2,id:"step-7-srcappsceners",text:"Step 7 · src/app/scene.rs"},{level:2,id:"step-8-srclibrs",text:"Step 8 · src/lib.rs"},{level:2,id:"step-9-srcstaters",text:"Step 9 · src/state.rs"},{level:2,id:"step-10-srcstateeditrs",text:"Step 10 · src/state/edit.rs"},{level:2,id:"step-11-srcstatepanelrs",text:"Step 11 · src/state/panel.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
