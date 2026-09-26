const s={title:"31 · Split curves and faces while keeping the shell joined",html:`<h1 id="31-split-curves-and-faces-while-keeping-the-shell-joined">31 · Split curves and faces while keeping the shell joined<a class="anchor" href="#/course/31-splitting#31-split-curves-and-faces-while-keeping-the-shell-joined" aria-label="Link to this section">#</a></h1>
<p>Splitting a curve keeps its cutter, and splitting a face keeps both regions in the joined shell.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/31-splitting#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>Add Split, its input checks and contextual syntax.</p>
<p><code>lessons/31/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Scale(f64),</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Split,      <span class="c">// cut a curve or face</span></code></pre></div>
<p>Added after the line <code>&quot;line&quot; =&gt; &quot;Line start end · Example: Line 0,0,0 100,0,0&quot;,</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">curve</span>&quot; =&gt; &quot;<span class="s">Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0</span>&quot;,</code></pre></div>
<p>Added after the line <code>&quot;explode&quot; =&gt; &quot;Select a polyline · Explode creates its ind…</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">split</span>&quot; =&gt; {
            &quot;<span class="s">Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels</span>&quot;
        }</code></pre></div>
<p>Replaces the 2 lines from <code>&quot;save&quot; | &quot;open&quot; | &quot;delete&quot; | &quot;del&quot; | &quot;undo&quot; | &quot;redo&quot; | &quot;h…</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">split</span>&quot; | &quot;<span class="s">save</span>&quot; | &quot;<span class="s">open</span>&quot; | &quot;<span class="s">delete</span>&quot; | &quot;<span class="s">del</span>&quot; | &quot;<span class="s">undo</span>&quot; | &quot;<span class="s">redo</span>&quot; | &quot;<span class="s">hide</span>&quot; | &quot;<span class="s">show</span>&quot;
        | &quot;<span class="s">fit</span>&quot; | &quot;<span class="s">escape</span>&quot; | &quot;<span class="s">esc</span>&quot; =&gt; Some(<span class="s">0</span>),</code></pre></div>
<p>Replaces the line <code>&quot;point&quot; | &quot;line&quot; | &quot;polyline&quot; | &quot;trim&quot; | &quot;extend&quot; | &quot;expl…</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; | &quot;<span class="s">curve</span>&quot; | &quot;<span class="s">trim</span>&quot; | &quot;<span class="s">extend</span>&quot; | &quot;<span class="s">explode</span>&quot; =&gt; {</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">split</span>&quot; =&gt; Ok(Command::Split),</code></pre></div>
<p>Replaces the line <code>&quot;point&quot; | &quot;line&quot; | &quot;polyline&quot; =&gt; {</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; | &quot;<span class="s">curve</span>&quot; =&gt; {</code></pre></div>
<p>Replaces the line <code>_ =&gt; Err(&quot;point needs one coordinate; line two; polyline …</code> in <code>lessons/30/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                (&quot;<span class="s">curve</span>&quot;, <span class="s">2</span>..) =&gt; Ok(Modeling::Curve(points)),
                _ =&gt; {
                    Err(&quot;<span class="s">point needs one coordinate; line two; polyline/curve at least two</span>&quot;.into())
                }</code></pre></div>
<h2 id="step-2-srcappinputrs">Step 2 · src/app/input.rs<a class="anchor" href="#/course/31-splitting#step-2-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Route Enter and Escape to the pending split before ordinary scene shortcuts.</p>
<p><code>lessons/31/src/app/input.rs</code> · edit · type this</p>
<p>Added after the line <code>Key::Named(NamedKey::Escape) =&gt; state.escape_selection(),</code> in <code>lessons/30/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Named(NamedKey::Enter) =&gt; state.confirm_split(),</code></pre></div>
<h2 id="step-3-srcappinspectionrs">Step 3 · src/app/inspection.rs<a class="anchor" href="#/course/31-splitting#step-3-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Expose the new selection and resource state to the browser inspection data.</p>
<p><code>lessons/31/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the line <code>snapshot[&quot;color_count&quot;] = serde_json::json!(state.scene.c…</code> in <code>lessons/30/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    snapshot[&quot;<span class="s">split</span>&quot;] = serde_json::json!(state.split_status());
    snapshot[&quot;<span class="s">source_faces</span>&quot;] =
        serde_json::json!(parent.and_then(|row| <span class="k">match</span> state.scene.geometry(row)? {
            session_rust::Geometry::BRep(brep) =&gt; Some(brep.face_count()),
            session_rust::Geometry::Element(element) =&gt; <span class="k">match</span> element.geometry() {
                session_rust::element::ElementGeometry::BRep(brep) =&gt; Some(brep.face_count()),
                _ =&gt; None,
            },
            _ =&gt; None,
        }));</code></pre></div>
<h2 id="step-4-srcappmodrs">Step 4 · src/app/mod.rs<a class="anchor" href="#/course/31-splitting#step-4-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/31/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod surface_preview;</code> in <code>lessons/30/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> splitting;</code></pre></div>
<h2 id="step-5-srcappmodelingrs">Step 5 · src/app/modeling.rs<a class="anchor" href="#/course/31-splitting#step-5-srcappmodelingrs" aria-label="Link to this section">#</a></h2>
<p>Connect curve creation and replacement to the split workflow.</p>
<p><code>lessons/31/src/app/modeling.rs</code> · edit · type this</p>
<p>Added after the line <code>Polyline(Vec&lt;[f64; 3]&gt;),</code> in <code>lessons/30/src/app/modeling.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Curve(Vec&lt;[f64; 3]&gt;),     <span class="c">// create a curve through control points</span></code></pre></div>
<p>Added after the line <code>self.create_geometry(Geometry::Polyline(Rc::new(Polyline:…</code> in <code>lessons/30/src/app/modeling.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Modeling::Curve(points) =&gt; {
                <span class="k">if</span> !(<span class="s">2</span>..=MAX_POINTS).contains(&amp;points.len()) {
                    <span class="k">return</span> Err(format!(&quot;<span class="s">curve needs 2–</span>{<span class="s">MAX_POINTS</span>}<span class="s"> control points</span>&quot;));
                }

                <span class="k">let</span> points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::&lt;Result&lt;Vec&lt;_&gt;, _&gt;&gt;()?;
                <span class="k">let</span> curve =
                    session_rust::NurbsCurve::create(<span class="s">false</span>, (points.len() - <span class="s">1</span>).min(<span class="s">3</span>), &amp;points);
                <span class="k">self</span>.create_geometry(Geometry::NurbsCurve(Rc::new(curve)))
            }</code></pre></div>
<p>Added after the line <code>debug_assert!(added.is_some());</code> in <code>lessons/30/src/app/modeling.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Geometry::NurbsCurve(curve) =&gt; {
                session.add_nurbscurve((*curve).clone(), None);
            }</code></pre></div>
<h2 id="step-6-srcappsceners">Step 6 · src/app/scene.rs<a class="anchor" href="#/course/31-splitting#step-6-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Expose the owning document identity to split operations.</p>
<p><code>lessons/31/src/app/scene.rs</code> · edit · type this</p>
<p>Replaces the line <code>if is_planar(&amp;self.tables, &amp;from, &amp;place) {</code> in <code>lessons/30/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a flat file is a drawing sheet, unless it was drawn here</span>
        <span class="k">if</span> <span class="k">self</span>.created_doc != Some(<span class="k">self</span>.docs.len()) &amp;&amp; is_planar(&amp;<span class="k">self</span>.tables, &amp;from, &amp;place) {</code></pre></div>
<h2 id="step-7-srcappsession_iors">Step 7 · src/app/session_io.rs<a class="anchor" href="#/course/31-splitting#step-7-srcappsession_iors" aria-label="Link to this section">#</a></h2>
<p>Preserve split results and visible curve pens when reopening a session.</p>
<p><code>lessons/31/src/app/session_io.rs</code> · edit · type this</p>
<p>Added after the line <code>struct Metadata {</code> in <code>lessons/30/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[serde(default)]
    created_doc: Option&lt;usize&gt;, <span class="c">// index of the \`Created\` document</span></code></pre></div>
<p>Added after the line <code>let metadata = Metadata {</code> in <code>lessons/30/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        created_doc: scene.created_doc,</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/30/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> metadata
        .created_doc
        .is_some_and(|index| index &gt;= metadata.documents.len())
    {
        <span class="k">return</span> Err(&quot;<span class="s">Created document index is outside the inventory</span>&quot;.into());
    }

    <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
    scene.created_doc = metadata.created_doc;</code></pre></div>
<p>Added after the line <code>#[test]</code> in <code>lessons/30/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A created line keeps its screen pen after reopening.</span>
    #[test]
    <span class="k">fn</span> created_curves_keep_visible_screen_pens_after_open() {
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene
            .model(&amp;<span class="k">crate</span>::app::modeling::Modeling::Line(
                [-<span class="s">3000</span>., -<span class="s">5000</span>., <span class="s">200</span>.],
                [-<span class="s">3000</span>., -<span class="s">1000</span>., <span class="s">200</span>.],
            ))
            .unwrap();
        <span class="k">let</span> restored = open(&amp;save(&amp;scene).unwrap()).unwrap();
        assert_eq!(restored.created_doc, Some(<span class="s">0</span>));
        assert_eq!(restored.tables.seg.ribbons.len(), <span class="s">1</span>);
        assert_eq!(restored.tables.seg.ribbons[<span class="s">0</span>].radius, <span class="s">0</span>.);
        assert_eq!(
            restored.tables.obj.rows[<span class="s">0</span>].flags &amp; <span class="k">crate</span>::engine::gpu::Instance::FLAG_SHEET,
            <span class="s">0</span>
        );
    }

    <span class="c">/// Edits, placements, hidden and colour state survive a save.</span></code></pre></div>
<h2 id="step-8-srcappsplittingrs">Step 8 · src/app/splitting.rs<a class="anchor" href="#/course/31-splitting#step-8-srcappsplittingrs" aria-label="Link to this section">#</a></h2>
<p>Convert cutters, split source curves or trimmed faces, and replace the result in one transaction.</p>
<p><code>lessons/31/src/app/splitting.rs</code> · 363 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Splitting cuts one curve or face with another, so the first pick is held here until the cutter arrives.</span>

<span class="k">use</span> super::scene::Scene;
<span class="k">use</span> session_rust::simple_split;
<span class="k">use</span> session_rust::{BRep, Geometry, NurbsCurve};
<span class="k">use</span> std::rc::Rc;

<span class="c">/// True for a geometry that can cut: a line, polyline or curve.</span>
<span class="k">pub</span> <span class="k">fn</span> is_cutter(geometry: &amp;Geometry) -&gt; bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

<span class="c">/// The face to split: None for a curve, the selected face for a BRep.</span>
<span class="k">pub</span> <span class="k">fn</span> face_index(geometry: &amp;Geometry, selected: Option&lt;usize&gt;) -&gt; Result&lt;Option&lt;usize&gt;, String&gt; {
    <span class="k">match</span> geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) =&gt; Ok(None),
        Geometry::BRep(brep) =&gt; brep_face(brep, selected).map(Some),
        Geometry::Element(element) =&gt; <span class="k">match</span> element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) =&gt; {
                brep_face(brep, selected).map(Some)
            }
            _ =&gt; Err(&quot;<span class="s">Split accepts curves and NURBS/BRep faces</span>&quot;.into()),
        },
        _ =&gt; Err(&quot;<span class="s">Split accepts lines, polylines, NURBS curves and surface faces</span>&quot;.into()),
    }
}

<span class="c">/// The selected face, or the only one.</span>
<span class="k">fn</span> brep_face(brep: &amp;BRep, selected: Option&lt;usize&gt;) -&gt; Result&lt;usize, String&gt; {
    selected
        .or((brep.face_count() == <span class="s">1</span>).then_some(<span class="s">0</span>))
        .filter(|face| *face &lt; brep.face_count())
        .ok_or_else(|| {
            &quot;<span class="s">Ctrl+Shift-select one BRep face before Split; the solid stays joined</span>&quot;.into()
        })
}

<span class="c">/// A cutter as a NURBS curve.</span>
<span class="k">fn</span> curve(geometry: &amp;Geometry) -&gt; Result&lt;NurbsCurve, String&gt; {
    <span class="k">match</span> geometry {
        Geometry::Line(line) =&gt; Ok(NurbsCurve::create(
            <span class="s">false</span>,
            <span class="s">1</span>,
            &amp;[line.point_at(<span class="s">0</span>.), line.point_at(<span class="s">1</span>.)],
        )),
        Geometry::Polyline(polyline) =&gt; Ok(NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;polyline.get_points())),
        Geometry::NurbsCurve(curve) =&gt; Ok((**curve).clone()),
        _ =&gt; Err(&quot;<span class="s">Choose a line, polyline or NURBS curve as cutter</span>&quot;.into()),
    }
}

<span class="k">impl</span> Scene {
    <span class="c">/// Split \`target\` by the cutters; returns how many pieces.</span>
    <span class="k">pub</span> <span class="k">fn</span> split_rows(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        target: u32,
        face: Option&lt;usize&gt;,
        cutters: &amp;[u32],
    ) -&gt; Result&lt;usize, String&gt; {
        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> Err(&quot;<span class="s">Splitting requires complete retained source documents</span>&quot;.into());
        }

        <span class="k">if</span> cutters.is_empty() || cutters.len() &gt; <span class="s">64</span> {
            <span class="k">return</span> Err(&quot;<span class="s">Choose 1–64 cutter curves</span>&quot;.into());
        }

        <span class="k">if</span> !<span class="k">self</span>.selectable(target) {
            <span class="k">return</span> Err(&quot;<span class="s">Unlock the target before splitting</span>&quot;.into());
        }

        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(target).ok_or(&quot;<span class="s">Target no longer exists</span>&quot;)?;
        <span class="k">let</span> file = <span class="k">self</span>.docs.get(doc).ok_or(&quot;<span class="s">Target has no source document</span>&quot;)?;

        <span class="k">if</span> file.display_only {
            <span class="k">return</span> Err(&quot;<span class="s">Target is display only</span>&quot;.into());
        }

        <span class="k">let</span> back = <span class="k">self</span>
            .placement_of(target)
            .ok_or(&quot;<span class="s">Target has no placement</span>&quot;)?
            .inverse()
            .ok_or(&quot;<span class="s">Target placement is singular</span>&quot;)?; <span class="c">// world into the target's frame</span>
        <span class="k">let</span> <span class="k">mut</span> tools = Vec::new(); <span class="c">// cutters in the target's frame</span>

        <span class="k">for</span> &amp;row <span class="k">in</span> cutters {
            <span class="k">if</span> row == target || !<span class="k">self</span>.selectable(row) {
                <span class="k">return</span> Err(&quot;<span class="s">Choose an unlocked cutter distinct from the target</span>&quot;.into());
            }

            <span class="k">let</span> <span class="k">mut</span> cutter = curve(<span class="k">self</span>.geometry(row).ok_or(&quot;<span class="s">Cutter no longer exists</span>&quot;)?)?;
            <span class="k">let</span> place = <span class="k">self</span>.placement_of(row).ok_or(&quot;<span class="s">Cutter has no placement</span>&quot;)?;

            <span class="k">if</span> place.inverse().is_none() {
                <span class="k">return</span> Err(&quot;<span class="s">Cannot transform the cutter into target coordinates</span>&quot;.into());
            }

            cutter.transform(&amp;(&amp;back * &amp;place));
            tools.push(cutter);
        }

        <span class="k">let</span> source = <span class="k">self</span>
            .geometry(target)
            .ok_or(&quot;<span class="s">Source geometry is unavailable</span>&quot;)?;
        <span class="k">let</span> face = face_index(source, face)?;
        <span class="k">let</span> tolerance = <span class="s">1</span>e-<span class="s">6</span>;
        <span class="c">// the new geometries and how many regions the cut made</span>
        <span class="k">let</span> (<span class="k">mut</span> pieces, regions) = <span class="k">match</span> source {
            Geometry::Line(line) =&gt; {
                <span class="k">let</span> pieces: Vec&lt;_&gt; = simple_split::split_line_by_curves(line, &amp;tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::Line(Rc::new(p)))
                    .collect();
                <span class="k">let</span> count = pieces.len();
                (pieces, count)
            }
            Geometry::Polyline(line) =&gt; {
                <span class="k">let</span> pieces: Vec&lt;_&gt; =
                    simple_split::split_polyline_by_curves(line, &amp;tools, tolerance)?
                        .into_iter()
                        .map(|p| Geometry::Polyline(Rc::new(p)))
                        .collect();
                <span class="k">let</span> count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsCurve(curve) =&gt; {
                <span class="k">let</span> pieces: Vec&lt;_&gt; = simple_split::split_curve_by_curves(curve, &amp;tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                    .collect();
                <span class="k">let</span> count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsSurface(surface) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> brep = simple_split::split_surface_by_curves(surface, &amp;tools, tolerance)?;
                brep.name = surface.name.clone();
                <span class="k">let</span> count = brep.face_count();
                (vec![Geometry::BRep(Rc::new(brep))], count)
            }
            Geometry::BRep(brep) =&gt; {
                <span class="k">let</span> next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or(&quot;<span class="s">Select a face</span>&quot;)?,
                    &amp;tools,
                    tolerance,
                )?;
                <span class="k">let</span> count = next.face_count() - brep.face_count() + <span class="s">1</span>;
                (vec![Geometry::BRep(Rc::new(next))], count)
            }
            Geometry::Element(element) =&gt; {
                <span class="k">let</span> session_rust::element::ElementGeometry::BRep(brep) = element.geometry() <span class="k">else</span> {
                    <span class="k">return</span> Err(&quot;<span class="s">Element has no BRep</span>&quot;.into());
                };
                <span class="k">let</span> next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or(&quot;<span class="s">Select a face</span>&quot;)?,
                    &amp;tools,
                    tolerance,
                )?;
                <span class="k">let</span> count = next.face_count() - brep.face_count() + <span class="s">1</span>;
                <span class="k">let</span> <span class="k">mut</span> result = (**element).clone();
                result.set_brep_geometry(next);
                (vec![Geometry::Element(Rc::new(result))], count)
            }
            _ =&gt; <span class="k">return</span> Err(&quot;<span class="s">Unsupported split target</span>&quot;.into()),
        };

        <span class="k">if</span> regions &lt; <span class="s">2</span> {
            <span class="k">return</span> Ok(<span class="s">1</span>); <span class="c">// nothing was cut</span>
        }

        <span class="c">// the pieces inherit the parent, placement and colours</span>
        <span class="k">let</span> parent_name = file
            .session
            .tree
            .get_node_by_name(&amp;guid)
            .and_then(|node| node.borrow().parent())
            .map(|node| node.borrow().name.clone());
        <span class="k">let</span> place = file.session.xform(&amp;guid);
        <span class="k">let</span> color = <span class="k">self</span>.colors.get(&amp;(doc, Rc::clone(&amp;guid))).copied();

        <span class="c">// name the pieces \`x (part 1)\`, \`x (part 2)\`...</span>
        <span class="k">if</span> pieces.len() &gt; <span class="s">1</span> {
            <span class="k">let</span> name = source.name();

            <span class="k">for</span> (index, piece) <span class="k">in</span> pieces.iter_mut().enumerate() {
                <span class="k">let</span> name = format!(&quot;{<span class="s">name</span>}<span class="s"> (part </span>{}<span class="s">)</span>&quot;, index + <span class="s">1</span>);

                <span class="k">match</span> piece {
                    Geometry::Line(p) =&gt; Rc::make_mut(p).name = name,
                    Geometry::Polyline(p) =&gt; Rc::make_mut(p).name = name,
                    Geometry::NurbsCurve(p) =&gt; Rc::make_mut(p).name = name,
                    _ =&gt; unreachable!(&quot;<span class="s">only curves create sibling objects</span>&quot;),
                }
            }
        }

        <span class="c">// the first piece replaces the target, the rest are added beside it</span>
        <span class="k">let</span> first = pieces.remove(<span class="s">0</span>);
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
        <span class="k">let</span> parent = parent_name.and_then(|name| session.tree.get_node_by_name(&amp;name));
        session.begin(&quot;<span class="s">split</span>&quot;);
        <span class="k">let</span> replaced = session.replace(&amp;guid, first);
        debug_assert!(replaced);

        <span class="k">for</span> piece <span class="k">in</span> pieces {
            <span class="k">let</span> node = <span class="k">match</span> piece {
                Geometry::Line(piece) =&gt; session.add_line((*piece).clone(), parent.as_ref()),
                Geometry::Polyline(piece) =&gt; session
                    .add_polyline((*piece).clone(), parent.as_ref())
                    .expect(&quot;<span class="s">valid split polyline</span>&quot;),
                Geometry::NurbsCurve(piece) =&gt; session
                    .add_nurbscurve((*piece).clone(), parent.as_ref())
                    .expect(&quot;<span class="s">valid split curve</span>&quot;),
                _ =&gt; unreachable!(&quot;<span class="s">only curves create sibling objects</span>&quot;),
            };
            <span class="k">let</span> id = node.borrow().name.clone();
            session.set_xform(&amp;id, place.clone());

            <span class="k">if</span> <span class="k">let</span> Some(color) = color {
                <span class="k">self</span>.colors.insert((doc, Rc::from(id)), color);
            }
        }

        session.commit();
        <span class="k">self</span>.last_edited = Some(doc);
        Ok(regions)
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::scene::FileDoc;
    <span class="k">use</span> session_rust::Xform;
    <span class="k">use</span> session_rust::{Line, Point, Session};

    <span class="c">/// Add one placed document.</span>
    <span class="k">fn</span> add(scene: &amp;<span class="k">mut</span> Scene, session: Rc&lt;Session&gt;, name: &amp;str, place: Xform) {
        scene.add_file(FileDoc {
            name: name.into(),
            session,
            place,
            point_px: <span class="s">0</span>.,
            display_only: <span class="s">false</span>,
        });
    }

    <span class="c">/// A split keeps the group, placement and other placements; undo reverses it.</span>
    #[test]
    <span class="k">fn</span> split_preserves_tree_placement_and_other_shared_documents_and_undo() {
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">shared</span>&quot;);
        <span class="k">let</span> group = session.add_group(&quot;<span class="s">parts</span>&quot;);
        session.set_xform(&amp;group.borrow().name, Xform::translation(<span class="s">10</span>., <span class="s">0</span>., <span class="s">0</span>.));
        <span class="k">let</span> target = session.add_line(
            Line::from_points(&amp;Point::new(-<span class="s">2</span>., <span class="s">0</span>., <span class="s">0</span>.), &amp;Point::new(<span class="s">2</span>., <span class="s">0</span>., <span class="s">0</span>.)),
            Some(&amp;group),
        );
        <span class="k">let</span> id = target.borrow().name.clone();
        <span class="k">let</span> shared = Rc::new(session);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        add(
            &amp;<span class="k">mut</span> scene,
            Rc::clone(&amp;shared),
            &quot;<span class="s">first</span>&quot;,
            Xform::translation(<span class="s">100</span>., <span class="s">0</span>., <span class="s">0</span>.),
        );
        add(
            &amp;<span class="k">mut</span> scene,
            Rc::clone(&amp;shared),
            &quot;<span class="s">second</span>&quot;,
            Xform::translation(<span class="s">200</span>., <span class="s">0</span>., <span class="s">0</span>.),
        );
        <span class="k">let</span> <span class="k">mut</span> cutters = Session::new(&quot;<span class="s">cutters</span>&quot;);
        cutters.add_line(
            Line::from_points(&amp;Point::new(<span class="s">110</span>., -<span class="s">2</span>., <span class="s">0</span>.), &amp;Point::new(<span class="s">110</span>., <span class="s">2</span>., <span class="s">0</span>.)),
            None,
        );
        add(&amp;<span class="k">mut</span> scene, Rc::new(cutters), &quot;<span class="s">cutters</span>&quot;, Xform::identity());
        scene
            .colors
            .insert(scene.identity_of(<span class="s">0</span>).unwrap(), [<span class="s">60</span>, <span class="s">170</span>, <span class="s">100</span>]);
        assert_eq!(scene.split_rows(<span class="s">0</span>, None, &amp;[<span class="s">2</span>]).unwrap(), <span class="s">2</span>);
        <span class="k">let</span> first = &amp;scene.docs[<span class="s">0</span>].session;
        assert_eq!(first.objects.lines.len(), <span class="s">2</span>);
        assert_eq!(scene.docs[<span class="s">1</span>].session.objects.lines.len(), <span class="s">1</span>);
        assert_eq!(shared.objects.lines.len(), <span class="s">1</span>);
        <span class="k">let</span> parent = first.tree.get_node_by_name(&quot;<span class="s">parts</span>&quot;).unwrap();
        assert_eq!(parent.borrow().children().len(), <span class="s">2</span>);
        assert_eq!(
            shared
                .tree
                .get_node_by_name(&quot;<span class="s">parts</span>&quot;)
                .unwrap()
                .borrow()
                .children()
                .len(),
            <span class="s">1</span>
        );
        assert!(first.lookup.contains_key(&amp;id));
        <span class="k">let</span> Geometry::Line(line) = &amp;first.lookup[&amp;id] <span class="k">else</span> {
            panic!()
        };
        assert!(line.point_at(<span class="s">1</span>.).distance(&amp;Point::new(<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>.), None) &lt; <span class="s">1</span>e-<span class="s">6</span>);
        assert_eq!(scene.colors.len(), <span class="s">2</span>);
        assert!(scene.undo());
        assert_eq!(scene.docs[<span class="s">0</span>].session.objects.lines.len(), <span class="s">1</span>);
        assert!(scene.redo());
        assert_eq!(scene.docs[<span class="s">0</span>].session.objects.lines.len(), <span class="s">2</span>);
        <span class="k">let</span> bytes = <span class="k">crate</span>::app::session_io::save(&amp;scene).unwrap();
        <span class="k">let</span> restored = <span class="k">crate</span>::app::session_io::open(&amp;bytes).unwrap();
        assert_eq!(restored.docs[<span class="s">0</span>].session.objects.lines.len(), <span class="s">2</span>);
    }

    <span class="c">/// A face split keeps the box solid; a bad cut changes nothing.</span>
    #[test]
    <span class="k">fn</span> split_face_keeps_solid_joined_and_invalid_cut_preserves_source() {
        <span class="k">let</span> brep = BRep::create_box(<span class="s">10</span>., <span class="s">10</span>., <span class="s">10</span>.);
        <span class="k">let</span> original_area = brep.face_meshes_q(Some((<span class="s">20</span>., <span class="s">0</span>.<span class="s">005</span>)))[<span class="s">0</span>].area();
        <span class="k">let</span> s = &amp;brep.m_surfaces[<span class="s">0</span>];
        <span class="k">let</span> a = s.get_cv(<span class="s">0</span>, <span class="s">0</span>).unwrap();
        <span class="k">let</span> u = s.get_cv(<span class="s">1</span>, <span class="s">0</span>).unwrap();
        <span class="k">let</span> v = s.get_cv(<span class="s">0</span>, <span class="s">1</span>).unwrap();
        <span class="k">let</span> p = |x: f64, y: f64| {
            Point::new(
                a[<span class="s">0</span>] + x * (u[<span class="s">0</span>] - a[<span class="s">0</span>]) + y * (v[<span class="s">0</span>] - a[<span class="s">0</span>]),
                a[<span class="s">1</span>] + x * (u[<span class="s">1</span>] - a[<span class="s">1</span>]) + y * (v[<span class="s">1</span>] - a[<span class="s">1</span>]),
                a[<span class="s">2</span>] + x * (u[<span class="s">2</span>] - a[<span class="s">2</span>]) + y * (v[<span class="s">2</span>] - a[<span class="s">2</span>]),
            )
        };
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">box</span>&quot;);
        <span class="k">let</span> node = session.add_brep(brep, None).unwrap();
        <span class="k">let</span> id = node.borrow().name.clone();
        session.add_line(Line::from_points(&amp;p(<span class="s">0</span>.<span class="s">5</span>, -<span class="s">1</span>.), &amp;p(<span class="s">0</span>.<span class="s">5</span>, <span class="s">2</span>.)), None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        add(&amp;<span class="k">mut</span> scene, Rc::new(session), &quot;<span class="s">box</span>&quot;, Xform::identity());
        <span class="k">let</span> row = (<span class="s">0</span>..scene.object_count() <span class="k">as</span> u32)
            .find(|&amp;row| matches!(scene.geometry(row), Some(Geometry::BRep(_))))
            .unwrap();
        <span class="k">let</span> cutter = (<span class="s">0</span>..scene.object_count() <span class="k">as</span> u32)
            .find(|&amp;row| matches!(scene.geometry(row), Some(Geometry::Line(_))))
            .unwrap();
        assert!(scene.split_rows(row, None, &amp;[cutter]).is_err());
        assert_eq!(scene.split_rows(row, Some(<span class="s">0</span>), &amp;[cutter]).unwrap(), <span class="s">2</span>);
        <span class="k">let</span> Geometry::BRep(result) = &amp;scene.docs[<span class="s">0</span>].session.lookup[&amp;id] <span class="k">else</span> {
            panic!()
        };
        assert_eq!(result.face_count(), <span class="s">7</span>);
        assert!(result.is_solid());
        <span class="k">let</span> meshes = result.face_meshes_q(Some((<span class="s">20</span>., <span class="s">0</span>.<span class="s">005</span>)));
        assert!(
            (meshes[<span class="s">0</span>].area() - original_area / <span class="s">2</span>.).abs() &lt; <span class="s">1</span>e-<span class="s">6</span>,
            &quot;<span class="s">first region area: </span>{}<span class="s"> of </span>{<span class="s">original_area</span>}&quot;,
            meshes[<span class="s">0</span>].area()
        );
        assert!(
            (meshes[<span class="s">6</span>].area() - original_area / <span class="s">2</span>.).abs() &lt; <span class="s">1</span>e-<span class="s">6</span>,
            &quot;<span class="s">second region area: </span>{}<span class="s"> of </span>{<span class="s">original_area</span>}&quot;,
            meshes[<span class="s">6</span>].area()
        );
        assert!(scene.undo());
        <span class="k">let</span> Geometry::BRep(original) = &amp;scene.docs[<span class="s">0</span>].session.lookup[&amp;id] <span class="k">else</span> {
            panic!()
        };
        assert_eq!(original.face_count(), <span class="s">6</span>);
        assert!(original.is_solid());
    }
}</code></pre></div>
<h2 id="step-9-srcappuirs">Step 9 · src/app/ui.rs<a class="anchor" href="#/course/31-splitting#step-9-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>Expose Split in the command and tool interface.</p>
<p><code>lessons/31/src/app/ui.rs</code> · edit · type this</p>
<p>Added after the line <code>model.command_open = false;</code> in <code>lessons/30/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    model.focus_command = <span class="s">false</span>;
                    response.surrender_focus();
                    close.surrender_focus();</code></pre></div>
<p>Added after the line <code>),</code> in <code>lessons/30/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    (
        &quot;<span class="s">Split</span>&quot;,
        &quot;<span class="s">Split selected curve or face with cutter curves</span>&quot;,
        &quot;<span class="s">split</span>&quot;,
    ),</code></pre></div>
<p>Added after the line <code>if response.clicked() {</code> in <code>lessons/30/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        response.surrender_focus();</code></pre></div>
<h2 id="step-10-srcshadersribbonwgsl">Step 10 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/31-splitting#step-10-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>Keep split curves visible with the same stroke filtering as the original curve.</p>
<p><code>lessons/31/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces the lines from <code>if (coverage(in) &lt; 0.5 || !ink_visible(in.pos.xy, ink_axi…</code> in <code>lessons/30/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Pick id: object row + 1 and tagged segment row + 1.</span>
<span class="k">fn</span> fs_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (coverage(in) &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the line <code>if (in.source_edge == 0xffffffffu || coverage(in) &lt; 0.5 |…</code> in <code>lessons/30/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (in.source_edge == <span class="s">0xffffffffu</span> || coverage(in) &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>)) {</code></pre></div>
<h2 id="step-11-srcstaters">Step 11 · src/state.rs<a class="anchor" href="#/course/31-splitting#step-11-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Store pending split state and route object picks to cutter collection.</p>
<p><code>lessons/31/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>mod sheet_query;</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> splitting;</code></pre></div>
<p>Added after the line <code>hierarchy: crate::app::hierarchy::Hierarchy,</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    pending_split: Option&lt;splitting::Pending&gt;,              <span class="c">// a split waiting for its cutter</span></code></pre></div>
<p>Added after the line <code>hierarchy: Default::default(),</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            pending_split: None,</code></pre></div>
<p>Added after the line <code>pub fn clear(&amp;mut self) {</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_split();</code></pre></div>
<p>Added after the line <code>pub fn select(&amp;mut self, row: Option&lt;u32&gt;) {</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_split();</code></pre></div>
<p>Added after the line <code>fn apply_pick(&amp;mut self, pick: Option&lt;Pick&gt;) {</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a split is waiting for its cutter</span>
        <span class="k">if</span> <span class="k">self</span>.pending_split.is_some() {
            <span class="k">if</span> <span class="k">let</span> Some(pick) = pick {
                <span class="k">self</span>.pick_split_cutter(pick.row);
            }

            <span class="k">return</span>;
        }</code></pre></div>
<p>Replaces the 2 lines from <code>let face = face || self.selection_tool == crate::app::sel…</code> in <code>lessons/30/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> splitting = <span class="k">self</span>.pending_split.is_some(); <span class="c">// a split wants a plain object</span>
        <span class="k">let</span> face = !splitting
            &amp;&amp; (face || <span class="k">self</span>.selection_tool == <span class="k">crate</span>::app::selection::SelectionTool::Face);
        <span class="k">let</span> edge = !splitting
            &amp;&amp; (edge || <span class="k">self</span>.selection_tool == <span class="k">crate</span>::app::selection::SelectionTool::Edge);
        <span class="k">self</span>.cancel_cloud_query();
        <span class="k">self</span>.gpu.pick.cancel();

        <span class="c">// a point cloud answers by its own query</span>
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        <span class="k">if</span> !splitting &amp;&amp; !edge &amp;&amp; <span class="k">self</span>.start_cloud_query(x, y) {
            <span class="k">return</span>;
        }

        <span class="c">// which kind of pick the id frame runs</span>
        <span class="k">let</span> mode = <span class="k">if</span> splitting {
            PickMode::Object
        } <span class="k">else</span> <span class="k">if</span> face {</code></pre></div>
<h2 id="step-12-srcstateeditrs">Step 12 · src/state/edit.rs<a class="anchor" href="#/course/31-splitting#step-12-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Dispatch Split and rebuild source selection after history changes.</p>
<p><code>lessons/31/src/state/edit.rs</code> · edit · type this</p>
<p>Replaces the line <code>fn after_history(&amp;mut self) {</code> in <code>lessons/30/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// After an undo, redo or delete: rebuild the rows, drop the selection.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> after_history(&amp;<span class="k">mut</span> <span class="k">self</span>) {</code></pre></div>
<p>Added after the line <code>let command = crate::app::command::parse(line)?;</code> in <code>lessons/30/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> command != Command::Split {
            <span class="k">self</span>.cancel_split();
        }</code></pre></div>
<p>Added after the line <code>match command {</code> in <code>lessons/30/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Split =&gt; <span class="k">self</span>.split_command(),</code></pre></div>
<p>Replaces the line <code>Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyl…</code> in <code>lessons/30/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    Modeling::Point(_)
                        | Modeling::Line(..)
                        | Modeling::Polyline(_)
                        | Modeling::Curve(_)</code></pre></div>
<p>Added after the line <code>Modeling::Line(..) =&gt; &quot;line&quot;,</code> in <code>lessons/30/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        Modeling::Curve(_) =&gt; &quot;<span class="s">NURBS curve</span>&quot;,</code></pre></div>
<h2 id="step-13-srcstatepanelrs">Step 13 · src/state/panel.rs<a class="anchor" href="#/course/31-splitting#step-13-srcstatepanelrs" aria-label="Link to this section">#</a></h2>
<p>Refresh the hierarchy when a split changes object membership.</p>
<p><code>lessons/31/src/state/panel.rs</code> · edit · type this</p>
<p>Added after the line <code>let rows = self.hierarchy.targets(index);</code> in <code>lessons/30/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="c">// while splitting, a click picks cutters</span>
                    <span class="k">if</span> <span class="k">self</span>.pending_split.is_some() {
                        <span class="k">for</span> row <span class="k">in</span> rows {
                            <span class="k">self</span>.pick_split_cutter(row);
                        }

                        <span class="k">return</span>;
                    }</code></pre></div>
<h2 id="step-14-srcstatesplittingrs">Step 14 · src/state/splitting.rs<a class="anchor" href="#/course/31-splitting#step-14-srcstatesplittingrs" aria-label="Link to this section">#</a></h2>
<p>Retain the target, selected face and cutter rows until confirmation.</p>
<p><code>lessons/31/src/state/splitting.rs</code> · 131 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
<span class="k">use</span> <span class="k">crate</span>::app::{feedback, splitting};

<span class="c">/// A split waiting for its cutters.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> Pending {
    <span class="k">pub</span> target: u32,         <span class="c">// the row being split</span>
    <span class="k">pub</span> face: Option&lt;usize&gt;, <span class="c">// the face of it, for a BRep</span>
    <span class="k">pub</span> cutters: Vec&lt;u32&gt;,   <span class="c">// the rows chosen as cutters</span>
}

<span class="k">impl</span> State {
    <span class="c">/// The pending split, for the inspection tests.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> split_status(&amp;<span class="k">self</span>) -&gt; Option&lt;(u32, Option&lt;usize&gt;, &amp;[u32])&gt; {
        <span class="k">self</span>.pending_split
            .as_ref()
            .map(|p| (p.target, p.face, p.cutters.as_slice()))
    }

    <span class="c">/// Split: start choosing cutters, or finish when they are chosen.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> split_command(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="c">// Split again finishes</span>
        <span class="k">if</span> <span class="k">self</span>.pending_split.is_some() {
            <span class="k">return</span> <span class="k">self</span>.finish_split();
        }

        <span class="k">let</span> row = <span class="k">self</span>
            .scene
            .selected
            .ok_or(&quot;<span class="s">Select a curve or Ctrl+Shift-select a face, then run Split</span>&quot;)?;
        <span class="c">// a selected face, else the whole curve</span>
        <span class="k">let</span> selected = <span class="k">match</span> <span class="k">self</span>.selection {
            <span class="k">crate</span>::app::selection::SelectionMode::Face { face, .. } =&gt; Some(face),
            _ =&gt; None,
        };
        <span class="k">let</span> face = splitting::face_index(
            <span class="k">self</span>.scene.geometry(row).ok_or(&quot;<span class="s">Source unavailable</span>&quot;)?,
            selected,
        )?;
        <span class="k">self</span>.pending_split = Some(Pending {
            target: row,
            face,
            cutters: vec![],
        });
        <span class="k">self</span>.place_gizmo(None);
        feedback::command_line(<span class="s">false</span>);
        feedback::focus_canvas();
        Ok(&quot;<span class="s">Split: select cutter lines, polylines or curves, then press Enter or tap Split again. Esc cancels. Faces require on-surface cutters.</span>&quot;.into())
    }

    <span class="c">/// Drop the pending split and its cutter highlights.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> cancel_split(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">let</span> Some(pending) = <span class="k">self</span>.pending_split.take() {
            <span class="k">for</span> row <span class="k">in</span> pending.cutters {
                <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
            }
        }
    }

    <span class="c">/// Add a clicked row to the cutters, or remove it again.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> pick_split_cutter(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) {
        <span class="k">let</span> Some(pending) = <span class="k">self</span>.pending_split.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="c">// only an unlocked curve, not the target</span>

        <span class="k">if</span> row == pending.target
            || !<span class="k">self</span>.scene.selectable(row)
            || !<span class="k">self</span>.scene.geometry(row).is_some_and(splitting::is_cutter)
        {
            <span class="k">self</span>.status(
                &quot;<span class="s">Choose an unlocked line, polyline or NURBS curve distinct from the target</span>&quot;,
            );
            <span class="k">return</span>;
        }

        <span class="k">if</span> <span class="k">let</span> Some(at) = pending.cutters.iter().position(|item| *item == row) {
            pending.cutters.remove(at);
            <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
        } <span class="k">else</span> <span class="k">if</span> pending.cutters.len() &lt; <span class="s">64</span> {
            pending.cutters.push(row);
            <span class="k">self</span>.gpu.set_selected(row, <span class="s">true</span>);
        }

        <span class="k">let</span> count = pending.cutters.len();
        <span class="k">self</span>.status(&amp;format!(
            &quot;<span class="s">Split: </span>{<span class="s">count</span>}<span class="s"> cutter curves selected. Enter or Split confirms; Esc cancels.</span>&quot;
        ));
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Enter: finish the split.</span>
    <span class="k">pub</span> <span class="k">fn</span> confirm_split(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.pending_split.is_some() {
            <span class="k">let</span> message = <span class="k">self</span>.finish_split().unwrap_or_else(|error| error);
            <span class="k">self</span>.status(&amp;message);
            <span class="k">self</span>.touch();
        }
    }

    <span class="c">/// Run the split with the chosen cutters.</span>
    <span class="k">fn</span> finish_split(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> pending = <span class="k">self</span>.pending_split.take().ok_or(&quot;<span class="s">Start Split first</span>&quot;)?;

        <span class="k">if</span> pending.cutters.is_empty() {
            <span class="k">self</span>.pending_split = Some(pending);
            <span class="k">return</span> Err(&quot;<span class="s">Select at least one cutter curve, then press Enter</span>&quot;.into());
        }

        <span class="k">for</span> &amp;row <span class="k">in</span> &amp;pending.cutters {
            <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
        }

        <span class="k">let</span> identity = <span class="k">self</span>.scene.identity_of(pending.target); <span class="c">// to find the row again after the rebuild</span>
        <span class="k">let</span> result = <span class="k">self</span>
            .scene
            .split_rows(pending.target, pending.face, &amp;pending.cutters);

        <span class="k">match</span> result {
            Ok(regions) <span class="k">if</span> regions &gt; <span class="s">1</span> =&gt; {
                <span class="k">self</span>.after_history();
                <span class="k">let</span> row = identity.and_then(|id| {
                    (<span class="s">0</span>..<span class="k">self</span>.scene.object_count() <span class="k">as</span> u32)
                        .find(|&amp;row| <span class="k">self</span>.scene.identity_of(row).as_ref() == Some(&amp;id))
                });
                <span class="k">self</span>.select(row);
                Ok(format!(
                    &quot;<span class="s">Split into </span>{<span class="s">regions</span>}<span class="s"> regions. The BRep stays joined; Undo restores the original. Cutters are retained.</span>&quot;
                ))
            }
            Ok(_) =&gt; {
                <span class="k">self</span>.place_gizmo(<span class="k">self</span>.scene.selected);
                Ok(&quot;<span class="s">No division: cutters must cross the curve or lie on the selected face.</span>&quot;.into())
            }
            Err(error) =&gt; {
                <span class="k">self</span>.place_gizmo(<span class="k">self</span>.scene.selected);
                Err(error)
            }
        }
    }
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/31-splitting#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/31/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: confirming the cutters creates curve pieces or joined face regions, and the status reports the split result.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-split-face.png" alt="Full viewer result for lesson 31" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The wrong face splits: a multi-face shell has no explicit selected face.</li>
<li>The cutter is rejected: it does not lie on the selected surface within tolerance.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/31-splitting#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/31/src/
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
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── deform.rs
│   ├── edit.rs
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs  ~
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── session_io.rs  ~
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── splitting.rs  +
│   ├── stream.rs
│   ├── surface_preview.rs
│   ├── touch.rs
│   ├── ui.rs  ~
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
│   │   ├── patch.rs
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
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl  ~
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
│   ├── panel.rs  ~
│   ├── sheet_query.rs
│   ├── splitting.rs  +
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: target and cutters → trimmed-region split → source transaction → refreshed selection.
Every file at this point: <code>lessons/31/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/31-splitting#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/32-colors-lighting">32 · Finish the command workspace and soft ambient lighting</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/31-splitting#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The shell has been divided into two face regions by an on-surface line. The cutter remains and the selected half is highlighted. The shell stays joined, and the editable session stores the updated source geometry. Use Undo to restore the original or Save to keep this result. See the <a href="/session/docs/course/docs/screenshots/extensions-split-phone.png">phone layout</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-split-face.png"><img src="/session/docs/course/docs/screenshots/extensions-split-face.png" alt="Full viewer result for lesson 31" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcappinputrs",text:"Step 2 · src/app/input.rs"},{level:2,id:"step-3-srcappinspectionrs",text:"Step 3 · src/app/inspection.rs"},{level:2,id:"step-4-srcappmodrs",text:"Step 4 · src/app/mod.rs"},{level:2,id:"step-5-srcappmodelingrs",text:"Step 5 · src/app/modeling.rs"},{level:2,id:"step-6-srcappsceners",text:"Step 6 · src/app/scene.rs"},{level:2,id:"step-7-srcappsession_iors",text:"Step 7 · src/app/session_io.rs"},{level:2,id:"step-8-srcappsplittingrs",text:"Step 8 · src/app/splitting.rs"},{level:2,id:"step-9-srcappuirs",text:"Step 9 · src/app/ui.rs"},{level:2,id:"step-10-srcshadersribbonwgsl",text:"Step 10 · src/shaders/ribbon.wgsl"},{level:2,id:"step-11-srcstaters",text:"Step 11 · src/state.rs"},{level:2,id:"step-12-srcstateeditrs",text:"Step 12 · src/state/edit.rs"},{level:2,id:"step-13-srcstatepanelrs",text:"Step 13 · src/state/panel.rs"},{level:2,id:"step-14-srcstatesplittingrs",text:"Step 14 · src/state/splitting.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
