const s={title:"34 · Polyline options, ordered input and a remembered view",html:`<h1 id="34-polyline-options-ordered-input-and-a-remembered-view">34 · Polyline options, ordered input and a remembered view<a class="anchor" href="#/course/34-polyline-options#34-polyline-options-ordered-input-and-a-remembered-view" aria-label="Link to this section">#</a></h1>
<p>The polyline command offers Points, Rectangle and Polygon, clicks and keys keep their order, and loading a scene never overrides a view you already chose.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/34-polyline-options#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p><code>Polyline</code> alone opens three options instead of running.</p>
<p><code>lessons/34/src/app/command.rs</code> · edit · type this</p>
<p>Replaces the line <code>&quot;polyline&quot; =&gt; &quot;Polyline points… · Example: Polyline 0,0,0…</code> in <code>lessons/33/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">polyline</span>&quot; =&gt; &quot;<span class="s">Polyline · click points, or choose Rectangle / Polygon · Enter finishes</span>&quot;,</code></pre></div>
<p>Added after the line <code>{</code> in <code>lessons/33/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">polyline</span>&quot; =&gt; &amp;[&quot;<span class="s">Polyline Points</span>&quot;, &quot;<span class="s">Polyline Rectangle</span>&quot;, &quot;<span class="s">Polyline Polygon</span>&quot;],</code></pre></div>
<p>Replaces the line <code>if (!options(text).is_empty() &amp;&amp; words.len() == 1)</code> in <code>lessons/33/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// a verb with options, or \`rotate &lt;axis&gt;\`, waits for more</span>
    <span class="k">if</span> (!options(text).is_empty() &amp;&amp; words.len() == <span class="s">1</span> &amp;&amp; !text.eq_ignore_ascii_case(&quot;<span class="s">polyline</span>&quot;))</code></pre></div>
<p>Added after the line <code>assert_eq!(accept(&quot;Lin&quot;), (&quot;Line&quot;.into(), true));</code> in <code>lessons/33/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(accept(&quot;<span class="s">Point</span>&quot;), (&quot;<span class="s">Point</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Polyline</span>&quot;), (&quot;<span class="s">Polyline</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Polyline rec</span>&quot;), (&quot;<span class="s">Polyline Rectangle</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Polyline pol</span>&quot;), (&quot;<span class="s">Polyline Polygon</span>&quot;.into(), <span class="s">true</span>));</code></pre></div>
<h2 id="step-2-srcstatedrawingrs">Step 2 · src/state/drawing.rs<a class="anchor" href="#/course/34-polyline-options#step-2-srcstatedrawingrs" aria-label="Link to this section">#</a></h2>
<p>Replace the whole file: the drawing state handles the three polyline modes.</p>
<p><code>lessons/34/src/state/drawing.rs</code> · replace the whole file · type this</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
<span class="k">use</span> <span class="k">crate</span>::app::{
    coords,
    cplane::CPlane,
    selection::Controls,
    snap::{<span class="k">self</span>, Snap, SnapKind},
};
<span class="k">use</span> session_rust::{Geometry, Point, Polyline, Vector};

<span class="c">/// A shape being drawn, not yet in the scene.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">struct</span> Draft {
    verb: String,               <span class="c">// point, line, polyline or curve</span>
    construction: String,       <span class="c">// points, rectangle or polygon</span>
    sides: usize,               <span class="c">// polygon side count</span>
    points: Vec&lt;Point&gt;,         <span class="c">// the points placed so far</span>
    plane: CPlane,              <span class="c">// the plane clicks land on</span>
    candidates: Vec&lt;Snap&gt;,      <span class="c">// scene points the cursor can snap to</span>
    hover: Option&lt;Point&gt;,       <span class="c">// where the cursor is now, in the scene</span>
    snapped: Option&lt;SnapKind&gt;,  <span class="c">// what the cursor snapped to</span>
}

<span class="k">impl</span> State {
    <span class="c">/// A command line entry while drawing; \`None\` if not one.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> drawing_command(&amp;<span class="k">mut</span> <span class="k">self</span>, text: &amp;str) -&gt; Option&lt;Result&lt;String, String&gt;&gt; {
        <span class="k">let</span> words: Vec&lt;_&gt; = text.split_whitespace().collect();
        <span class="k">let</span> verb = words.first().copied().unwrap_or(&quot;&quot;).to_ascii_lowercase();
        <span class="c">// Polyline Rectangle / Polygon / Points</span>
        <span class="k">let</span> construction = <span class="k">if</span> verb == &quot;<span class="s">polyline</span>&quot; {
            words
                .get(<span class="s">1</span>)
                .map(|word| word.to_ascii_lowercase())
                .filter(|word| matches!(word.as_str(), &quot;<span class="s">points</span>&quot; | &quot;<span class="s">rectangle</span>&quot; | &quot;<span class="s">polygon</span>&quot;))
        } <span class="k">else</span> {
            None
        };
        <span class="c">// a drawing verb with too few points starts a draft</span>
        <span class="k">if</span> matches!(verb.as_str(), &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; | &quot;<span class="s">curve</span>&quot;)
            &amp;&amp; (construction.is_some() || words.len() &lt; <span class="k">if</span> verb == &quot;<span class="s">point</span>&quot; { <span class="s">2</span> } <span class="k">else</span> { <span class="s">3</span> })
        {
            <span class="k">if</span> !<span class="k">self</span>.scene.streamed.is_empty() || !<span class="k">self</span>.scene.sheets.is_empty() {
                <span class="k">return</span> Some(Err(
                    &quot;<span class="s">geometry edits require a scene without streamed sources</span>&quot;.into(),
                ));
            }
            <span class="k">let</span> start = <span class="k">if</span> construction.is_some() { <span class="s">2</span> } <span class="k">else</span> { <span class="s">1</span> }; <span class="c">// where the points begin</span>
            <span class="k">self</span>.cancel_split();
            <span class="c">// draw on the plane the camera faces most</span>
            <span class="k">let</span> plane = CPlane::facing(&amp;<span class="k">self</span>.camera.orientation.rotate_vector(Vector::y_axis()));
            <span class="k">let</span> candidates = <span class="k">self</span>.drawing_candidates();
            <span class="k">self</span>.draft = Some(Draft {
                verb,
                construction: construction.unwrap_or_else(|| &quot;<span class="s">points</span>&quot;.into()),
                sides: <span class="s">6</span>,
                points: Vec::new(),
                plane,
                candidates,
                hover: None,
                snapped: None,
            });
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="c">// points typed on the same line count already</span>
            <span class="k">return</span> Some(<span class="k">if</span> words.len() &gt; start {
                <span class="k">self</span>.accept_coordinates(&amp;words[start..].join(&quot;<span class="s"> </span>&quot;))
            } <span class="k">else</span> {
                Ok(<span class="k">self</span>.drawing_prompt())
            });
        }
        <span class="k">self</span>.draft.as_ref()?; <span class="c">// not drawing: not ours</span>
        <span class="c">// Sides N sets the polygon side count</span>
        <span class="k">if</span> verb == &quot;<span class="s">sides</span>&quot; &amp;&amp; <span class="k">self</span>.draft.as_ref()?.construction == &quot;<span class="s">polygon</span>&quot; {
            <span class="k">return</span> Some(<span class="k">match</span> words.as_slice() {
                [_, count] =&gt; <span class="k">match</span> count.parse::&lt;usize&gt;() {
                    Ok(count) <span class="k">if</span> (<span class="s">3</span>..<span class="k">crate</span>::app::modeling::MAX_POINTS).contains(&amp;count) =&gt; {
                        <span class="k">self</span>.draft.as_mut().unwrap().sides = count;
                        Ok(<span class="k">self</span>.drawing_prompt())
                    }
                    _ =&gt; Err(&quot;<span class="s">Polygon sides must be between 3 and 4095</span>&quot;.into()),
                },
                _ =&gt; Err(&quot;<span class="s">Use Sides 6</span>&quot;.into()),
            });
        }
        <span class="c">// Enter alone finishes</span>
        <span class="k">if</span> text.trim().is_empty() {
            <span class="k">return</span> Some(<span class="k">self</span>.finish_drawing());
        }
        <span class="c">// a coordinate adds a point</span>
        <span class="k">if</span> coords::parse(words.first().copied().unwrap_or(&quot;&quot;)).is_some() {
            <span class="k">return</span> Some(<span class="k">self</span>.accept_coordinates(text));
        }
        None
    }

    <span class="c">/// Every scene point the cursor can snap to.</span>
    <span class="k">fn</span> drawing_candidates(&amp;<span class="k">self</span>) -&gt; Vec&lt;Snap&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
        <span class="c">// each document's object placements</span>
        <span class="k">let</span> placements: Vec&lt;_&gt; = <span class="k">self</span>
            .scene
            .docs
            .iter()
            .map(|doc| doc.session.world_xforms())
            .collect();
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.gpu.objects.len() {
            <span class="c">// skip hidden and display-only rows</span>
            <span class="k">if</span> !<span class="k">self</span>.scene.selectable(row)
                || <span class="k">self</span>.gpu.objects.row(row).is_some_and(|object| {
                    object.flags &amp; <span class="k">crate</span>::engine::gpu::Instance::FLAG_HIDDEN != <span class="s">0</span>
                })
            {
                <span class="k">continue</span>;
            }
            <span class="k">let</span> (Some(geometry), Some((doc, guid))) =
                (<span class="k">self</span>.scene.geometry(row), <span class="k">self</span>.scene.identity_of(row))
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> file = &amp;<span class="k">self</span>.scene.docs[doc];
            <span class="c">// the object's place in the world</span>
            <span class="k">let</span> place = placements[doc]
                .get(guid.as_ref())
                .map_or_else(|| file.place.clone(), |world| &amp;file.place * world);
            <span class="k">match</span> geometry {
                Geometry::Point(p) =&gt; out.push(Snap {
                    point: p.transformed(&amp;place),
                    kind: SnapKind::End,
                    owner: row,
                }),
                Geometry::Line(line) =&gt; snap::from_polyline(
                    &amp;[
                        line.start().transformed(&amp;place),
                        line.end().transformed(&amp;place),
                    ],
                    <span class="s">false</span>,
                    row,
                    &amp;<span class="k">mut</span> out,
                ),
                Geometry::Polyline(line) =&gt; {
                    <span class="k">let</span> points: Vec&lt;_&gt; = line
                        .get_points()
                        .iter()
                        .map(|p| p.transformed(&amp;place))
                        .collect();
                    snap::from_polyline(&amp;points, <span class="s">false</span>, row, &amp;<span class="k">mut</span> out);
                }
                Geometry::NurbsCurve(curve) =&gt; {
                    <span class="k">let</span> (a, b) = curve.domain();
                    <span class="k">for</span> t <span class="k">in</span> [a, b] {
                        out.push(Snap {
                            point: curve.point_at(t).transformed(&amp;place),
                            kind: SnapKind::End,
                            owner: row,
                        });
                    }
                }
                Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::NurbsSurface(_) =&gt; {
                    <span class="k">let</span> controls = Controls::from_geometry(geometry);
                    out.extend(controls.points.iter().map(|p| {
                        Snap {
                            point: Point::new(p.position[<span class="s">0</span>], p.position[<span class="s">1</span>], p.position[<span class="s">2</span>])
                                .transformed(&amp;place),
                            kind: SnapKind::Vertex,
                            owner: row,
                        }
                    }));
                }
                _ =&gt; {}
            }
        }
        out
    }

    <span class="c">/// Add typed coordinates to the draft.</span>
    <span class="k">fn</span> accept_coordinates(&amp;<span class="k">mut</span> <span class="k">self</span>, text: &amp;str) -&gt; Result&lt;String, String&gt; {
        <span class="c">// check every word before adding any</span>
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_ref().unwrap();
        <span class="k">let</span> <span class="k">mut</span> points = draft.points.clone();
        <span class="k">let</span> (x, y) = axes(draft.plane);
        <span class="k">for</span> word <span class="k">in</span> text.split_whitespace() {
            <span class="k">let</span> typed = coords::parse(word).ok_or(&quot;<span class="s">Use x,y,z, @dx,dy,dz, or distance&lt;angle</span>&quot;)?;
            <span class="c">// the direction from the last point to the cursor</span>
            <span class="k">let</span> along = points.last().zip(draft.hover.as_ref()).and_then(|(p, h)| {
                <span class="k">let</span> delta = Vector::new(h[<span class="s">0</span>] - p[<span class="s">0</span>], h[<span class="s">1</span>] - p[<span class="s">1</span>], h[<span class="s">2</span>] - p[<span class="s">2</span>]);
                <span class="k">let</span> length =
                    (delta[<span class="s">0</span>] * delta[<span class="s">0</span>] + delta[<span class="s">1</span>] * delta[<span class="s">1</span>] + delta[<span class="s">2</span>] * delta[<span class="s">2</span>]).sqrt();
                (length &gt; <span class="s">1</span>e-<span class="s">12</span>)
                    .then(|| Vector::new(delta[<span class="s">0</span>] / length, delta[<span class="s">1</span>] / length, delta[<span class="s">2</span>] / length))
            });
            <span class="k">let</span> p = coords::resolve(
                typed,
                &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                &amp;x,
                &amp;y,
                points.last(),
                along.as_ref(),
            )
            .ok_or(&quot;<span class="s">This coordinate needs a previous point or a cursor direction</span>&quot;)?;
            <span class="k">if</span> (<span class="s">0</span>..<span class="s">3</span>).any(|i| !p[i].is_finite() || p[i].abs() &gt; <span class="s">1</span>e<span class="s">12</span>) {
                <span class="k">return</span> Err(&quot;<span class="s">Coordinates must be finite and within ±1e12</span>&quot;.into());
            }
            points.push(p);
        }
        <span class="c">// how many points the verb takes</span>
        <span class="k">let</span> limit = <span class="k">match</span> draft.verb.as_str() {
            &quot;<span class="s">point</span>&quot; =&gt; <span class="s">1</span>,
            &quot;<span class="s">line</span>&quot; =&gt; <span class="s">2</span>,
            _ <span class="k">if</span> draft.construction != &quot;<span class="s">points</span>&quot; =&gt; <span class="s">2</span>,
            _ =&gt; <span class="k">crate</span>::app::modeling::MAX_POINTS,
        };
        <span class="k">if</span> points.len() &gt; limit {
            <span class="k">return</span> Err(format!(&quot;{}<span class="s"> accepts at most </span>{<span class="s">limit</span>}<span class="s"> points</span>&quot;, draft.verb));
        }
        <span class="k">let</span> previous = std::mem::replace(&amp;<span class="k">mut</span> <span class="k">self</span>.draft.as_mut().unwrap().points, points);
        <span class="k">let</span> result = <span class="k">self</span>.advance_drawing();
        <span class="c">// a failed finish keeps the old points</span>
        <span class="k">if</span> result.is_err() {
            <span class="k">self</span>.draft.as_mut().unwrap().points = previous;
        }
        result
    }

    <span class="c">/// Finish when the draft has all its points, else prompt for the next.</span>
    <span class="k">fn</span> advance_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_ref().unwrap();
        <span class="k">if</span> (draft.verb == &quot;<span class="s">point</span>&quot; &amp;&amp; draft.points.len() == <span class="s">1</span>)
            || ((draft.verb == &quot;<span class="s">line</span>&quot; || draft.construction != &quot;<span class="s">points</span>&quot;) &amp;&amp; draft.points.len() == <span class="s">2</span>)
        {
            <span class="k">self</span>.finish_drawing()
        } <span class="k">else</span> {
            Ok(<span class="k">self</span>.drawing_prompt())
        }
    }

    <span class="c">/// Turn the draft into a typed command and run it.</span>
    <span class="k">fn</span> finish_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> points = <span class="k">self</span>.draft.as_ref().unwrap().geometry_points()?;
        <span class="k">let</span> draft = <span class="k">self</span>.draft.take().unwrap();
        <span class="c">// &quot;line 0,0,0 1,1,1&quot;</span>
        <span class="k">let</span> <span class="k">mut</span> command = draft.verb.clone();
        <span class="k">for</span> p <span class="k">in</span> &amp;points {
            command.push_str(&amp;format!(&quot;<span class="s"> </span>{}<span class="s">,</span>{}<span class="s">,</span>{}&quot;, p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]));
        }
        <span class="c">// same path as a typed command: checks, selection, undo</span>
        <span class="k">let</span> result = <span class="k">crate</span>::app::command::parse(&amp;command).and_then(|_| <span class="k">self</span>.run_command(&amp;command));
        <span class="k">if</span> result.is_err() {
            <span class="k">self</span>.draft = Some(draft);
        }
        result
    }

    <span class="c">/// The draft as JSON, for the inspection tests.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_status(&amp;<span class="k">self</span>) -&gt; serde_json::Value {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> serde_json::Value::Null;
        };
        serde_json::json!({&quot;<span class="s">command</span>&quot;:draft.verb,&quot;<span class="s">construction</span>&quot;:draft.construction,&quot;<span class="s">sides</span>&quot;:draft.sides,&quot;<span class="s">points</span>&quot;:draft.points.iter().map(|p| [p[<span class="s">0</span>],p[<span class="s">1</span>],p[<span class="s">2</span>]]).collect::&lt;Vec&lt;_&gt;&gt;(),&quot;<span class="s">hover</span>&quot;:draft.hover.as_ref().map(|p| [p[<span class="s">0</span>],p[<span class="s">1</span>],p[<span class="s">2</span>]]),&quot;<span class="s">snap</span>&quot;:draft.snapped.map(|k| format!(&quot;{<span class="s">k:?</span>}&quot;))})
    }

    <span class="c">/// The verb being drawn, or empty.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_verb(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">self</span>.draft.as_ref().map_or(&quot;&quot;, |draft| draft.verb.as_str())
    }

    <span class="c">/// The status line text while drawing.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_prompt(&amp;<span class="k">self</span>) -&gt; String {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> String::new();
        };
        <span class="c">// rectangle and polygon ask for two special points</span>
        <span class="k">if</span> draft.construction != &quot;<span class="s">points</span>&quot; {
            <span class="k">let</span> point = <span class="k">match</span> (draft.construction.as_str(), draft.points.is_empty()) {
                (&quot;<span class="s">rectangle</span>&quot;, <span class="s">true</span>) =&gt; &quot;<span class="s">First corner</span>&quot;,
                (&quot;<span class="s">rectangle</span>&quot;, <span class="s">false</span>) =&gt; &quot;<span class="s">Opposite corner</span>&quot;,
                (_, <span class="s">true</span>) =&gt; &quot;<span class="s">Center</span>&quot;,
                (_, <span class="s">false</span>) =&gt; &quot;<span class="s">Radius point</span>&quot;,
            };
            <span class="k">let</span> sides = <span class="k">if</span> draft.construction == &quot;<span class="s">polygon</span>&quot; {
                format!(&quot;<span class="s"> · </span>{}<span class="s"> sides (type Sides N)</span>&quot;, draft.sides)
            } <span class="k">else</span> {
                String::new()
            };
            <span class="k">return</span> format!(
                &quot;<span class="s">Polyline </span>{}<span class="s">: </span>{<span class="s">point</span>}<span class="s"> · click or type x,y,z</span>{<span class="s">sides</span>}<span class="s"> · Esc cancels</span>&quot;,
                draft.construction
            );
        }
        <span class="k">let</span> point = <span class="k">if</span> draft.points.is_empty() {
            &quot;<span class="s">First point</span>&quot;
        } <span class="k">else</span> {
            &quot;<span class="s">Next point</span>&quot;
        };
        <span class="k">let</span> finish = <span class="k">if</span> matches!(draft.verb.as_str(), &quot;<span class="s">curve</span>&quot; | &quot;<span class="s">polyline</span>&quot;) {
            &quot;<span class="s"> · Enter finishes</span>&quot;
        } <span class="k">else</span> {
            &quot;&quot;
        };
        format!(
            &quot;{}<span class="s">: </span>{<span class="s">point</span>}<span class="s"> · click or type x,y,z · Snap </span>{}{<span class="s">finish</span>}<span class="s"> · Esc cancels</span>&quot;,
            draft.verb,
            <span class="k">if</span> <span class="k">self</span>.snap_enabled { &quot;<span class="s">On</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Off</span>&quot; }
        )
    }

    <span class="c">/// Move the cursor while drawing: snap or land on the plane.</span>
    <span class="k">pub</span> <span class="k">fn</span> hover_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="c">// the nearest snap point within 12 pixels</span>
        <span class="k">let</span> hit = <span class="k">if</span> <span class="k">self</span>.snap_enabled {
            <span class="k">let</span> origin = <span class="k">self</span>.camera.origin();
            <span class="k">let</span> matrix = <span class="k">self</span>.camera.view_proj_anchored(<span class="k">self</span>.aspect(), &amp;origin).m;
            <span class="k">let</span> (width, height) = <span class="k">self</span>.viewport();
            snap::best(&amp;draft.candidates, (x, y), <span class="s">12</span>.<span class="s">0</span> * <span class="k">self</span>.pixel_scale(), |p| {
                <span class="c">// scene point to screen pixel</span>
                <span class="k">let</span> v = [p[<span class="s">0</span>] - origin[<span class="s">0</span>], p[<span class="s">1</span>] - origin[<span class="s">1</span>], p[<span class="s">2</span>] - origin[<span class="s">2</span>]];
                <span class="k">let</span> clip: [f64; <span class="s">4</span>] = std::array::from_fn(|r| {
                    matrix[r] * v[<span class="s">0</span>] + matrix[r + <span class="s">4</span>] * v[<span class="s">1</span>] + matrix[r + <span class="s">8</span>] * v[<span class="s">2</span>] + matrix[r + <span class="s">12</span>]
                });
                (clip[<span class="s">3</span>] &gt; <span class="s">0</span>.<span class="s">0</span>).then(|| {
                    (
                        (clip[<span class="s">0</span>] / clip[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span> + <span class="s">0</span>.<span class="s">5</span>) * width,
                        (<span class="s">0</span>.<span class="s">5</span> - clip[<span class="s">1</span>] / clip[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span>) * height,
                    )
                })
            })
        } <span class="k">else</span> {
            None
        };
        <span class="c">// otherwise, where the cursor ray meets the plane</span>
        <span class="k">let</span> free = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()).and_then(|(p, d)| {
            draft.plane.hit(
                draft.points.last().unwrap_or(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)),
                &amp;p,
                &amp;d,
            )
        });
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_mut().unwrap();
        draft.snapped = hit.as_ref().map(|s| s.kind);
        draft.hover = hit.map(|s| s.point).or(free);
        <span class="s">true</span>
    }

    <span class="c">/// Click while drawing: place a point.</span>
    <span class="k">pub</span> <span class="k">fn</span> click_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">if</span> !<span class="k">self</span>.hover_drawing(x, y) {
            <span class="k">return</span> <span class="s">false</span>;
        }
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_mut().unwrap();
        <span class="k">let</span> Some(p) = draft.hover.clone() <span class="k">else</span> {
            <span class="k">self</span>.status(&quot;<span class="s">Point is outside the construction plane</span>&quot;);
            <span class="k">return</span> <span class="s">true</span>;
        };
        <span class="k">if</span> draft.points.len() &gt;= <span class="k">crate</span>::app::modeling::MAX_POINTS {
            <span class="k">self</span>.status(&quot;<span class="s">Too many points</span>&quot;);
            <span class="k">return</span> <span class="s">true</span>;
        }
        draft.points.push(p);
        <span class="k">let</span> message = <span class="k">self</span>.advance_drawing().unwrap_or_else(|e| {
            <span class="k">self</span>.draft.as_mut().unwrap().points.pop(); <span class="c">// a failed finish drops the point</span>
            e
        });
        <span class="k">self</span>.status(&amp;message);
        <span class="k">crate</span>::app::feedback::command_line(<span class="s">true</span>);
        <span class="s">true</span>
    }

    <span class="c">/// The draft as screen points for the preview, plus the snap name.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_overlay(&amp;<span class="k">self</span>) -&gt; (Vec&lt;(f64, f64)&gt;, String) {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> (Vec::new(), String::new());
        };
        <span class="c">// the placed points plus the cursor</span>
        <span class="k">let</span> <span class="k">mut</span> preview = draft.points.clone();
        preview.extend(draft.hover.iter().cloned());
        <span class="k">let</span> preview = construction_points(&amp;draft.construction, draft.plane, draft.sides, &amp;preview)
            .unwrap_or(preview);
        <span class="k">let</span> points = preview
            .iter()
            .filter_map(|p| <span class="k">self</span>.project([p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]))
            .collect();
        (
            points,
            draft.snapped.map(|k| format!(&quot;{<span class="s">k:?</span>}&quot;)).unwrap_or_default(),
        )
    }
}

<span class="c">/// The two axes of a construction plane.</span>
<span class="k">fn</span> axes(plane: CPlane) -&gt; (Vector, Vector) {
    <span class="k">match</span> plane {
        CPlane::Xy =&gt; (Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)),
        CPlane::Xz =&gt; (Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)),
        CPlane::Yz =&gt; (Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)),
    }
}

<span class="k">impl</span> Draft {
    <span class="c">/// The final polyline points.</span>
    <span class="k">fn</span> geometry_points(&amp;<span class="k">self</span>) -&gt; Result&lt;Vec&lt;Point&gt;, String&gt; {
        construction_points(&amp;<span class="k">self</span>.construction, <span class="k">self</span>.plane, <span class="k">self</span>.sides, &amp;<span class="k">self</span>.points)
    }
}

<span class="c">/// A rectangle or polygon from two points, or the points as they are.</span>
<span class="k">fn</span> construction_points(
    kind: &amp;str,
    plane: CPlane,
    sides: usize,
    points: &amp;[Point],
) -&gt; Result&lt;Vec&lt;Point&gt;, String&gt; {
    <span class="k">if</span> kind == &quot;<span class="s">points</span>&quot; {
        <span class="k">return</span> Ok(points.to_vec());
    }
    <span class="k">let</span> [a, b] = points <span class="k">else</span> {
        <span class="k">return</span> Err(&quot;<span class="s">Pick two points to complete this polyline</span>&quot;.into());
    };
    <span class="k">let</span> (x, y) = axes(plane);
    <span class="c">// a to b, measured along the plane axes</span>
    <span class="k">let</span> u: f64 = (<span class="s">0</span>..<span class="s">3</span>).map(|i| (b[i] - a[i]) * x[i]).sum();
    <span class="k">let</span> v: f64 = (<span class="s">0</span>..<span class="s">3</span>).map(|i| (b[i] - a[i]) * y[i]).sum();
    <span class="k">if</span> kind == &quot;<span class="s">rectangle</span>&quot; {
        <span class="k">if</span> u.abs() &lt;= <span class="s">1</span>e-<span class="s">12</span> || v.abs() &lt;= <span class="s">1</span>e-<span class="s">12</span> {
            <span class="k">return</span> Err(&quot;<span class="s">Rectangle corners must define a nonzero width and height</span>&quot;.into());
        }
        <span class="k">return</span> Ok(Polyline::rectangle(a, &amp;x, &amp;y, u, v, <span class="s">true</span>).get_points());
    }
    <span class="k">let</span> radius = u.hypot(v);
    <span class="k">if</span> radius &lt;= <span class="s">1</span>e-<span class="s">12</span> {
        <span class="k">return</span> Err(&quot;<span class="s">Polygon radius must be above zero</span>&quot;.into());
    }
    <span class="k">let</span> (cos, sin) = (u / radius, v / radius); <span class="c">// turn so a corner lands on b</span>
    Ok(Polyline::from_sides(sides, radius, <span class="s">true</span>)
        .get_points()
        .iter()
        .map(|p| {
            <span class="c">// rotate, then place on the plane</span>
            <span class="k">let</span> px = cos * p[<span class="s">0</span>] - sin * p[<span class="s">1</span>];
            <span class="k">let</span> py = sin * p[<span class="s">0</span>] + cos * p[<span class="s">1</span>];
            Point::new(
                a[<span class="s">0</span>] + px * x[<span class="s">0</span>] + py * y[<span class="s">0</span>],
                a[<span class="s">1</span>] + px * x[<span class="s">1</span>] + py * y[<span class="s">1</span>],
                a[<span class="s">2</span>] + px * x[<span class="s">2</span>] + py * y[<span class="s">2</span>],
            )
        })
        .collect())
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    #[test]
    <span class="k">fn</span> rectangle_and_polygon_follow_the_construction_plane() {
        <span class="k">let</span> points = [Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>), Point::new(<span class="s">14</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">33</span>.<span class="s">0</span>)];
        <span class="k">let</span> rectangle = construction_points(&quot;<span class="s">rectangle</span>&quot;, CPlane::Xz, <span class="s">6</span>, &amp;points).unwrap();
        assert_eq!(rectangle.len(), <span class="s">5</span>);
        assert_eq!(
            [rectangle[<span class="s">2</span>][<span class="s">0</span>], rectangle[<span class="s">2</span>][<span class="s">1</span>], rectangle[<span class="s">2</span>][<span class="s">2</span>]],
            [<span class="s">14</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">33</span>.<span class="s">0</span>]
        );
        <span class="k">let</span> polygon = construction_points(&quot;<span class="s">polygon</span>&quot;, CPlane::Xz, <span class="s">5</span>, &amp;points).unwrap();
        assert_eq!(polygon.len(), <span class="s">6</span>);
        <span class="k">for</span> p <span class="k">in</span> &amp;polygon {
            assert!(((p[<span class="s">0</span>] - <span class="s">10</span>.<span class="s">0</span>).hypot(p[<span class="s">2</span>] - <span class="s">30</span>.<span class="s">0</span>) - <span class="s">5</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
            assert_eq!(p[<span class="s">1</span>], <span class="s">20</span>.<span class="s">0</span>);
        }
        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            assert!((polygon[<span class="s">0</span>][i] - points[<span class="s">1</span>][i]).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
            assert!((polygon[<span class="s">0</span>][i] - polygon[<span class="s">5</span>][i]).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
        }
        assert!(construction_points(&quot;<span class="s">rectangle</span>&quot;, CPlane::Xy, <span class="s">6</span>, &amp;points).is_err());
        assert!(construction_points(&quot;<span class="s">polygon</span>&quot;, CPlane::Xy, <span class="s">6</span>, &amp;points[..<span class="s">1</span>]).is_err());
        assert!(
            construction_points(
                &quot;<span class="s">polygon</span>&quot;,
                CPlane::Xy,
                <span class="s">6</span>,
                &amp;[points[<span class="s">0</span>].clone(), points[<span class="s">0</span>].clone()]
            )
            .is_err()
        );
    }
}</code></pre></div>
<h2 id="step-3-srcappuirs">Step 3 · src/app/ui.rs<a class="anchor" href="#/course/34-polyline-options#step-3-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>The command panel shows the polyline options and runs events in the order they arrived.</p>
<p><code>lessons/34/src/app/ui.rs</code> · edit · type this</p>
<p>Added after the line <code>pub drawing_prompt: String,</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    drawing_command: String,                        <span class="c">// the drawing verb, e.g. polyline</span></code></pre></div>
<p>Added after the line <code>let mut consumed = response.consumed;</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> scene_rect = <span class="k">self</span>.scene_rect;
        scene_rect.max.y -= <span class="s">5</span>.<span class="s">0</span>;</code></pre></div>
<p>Replaces the 3 lines from <code>consumed = self.ui_drag</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                consumed =
                    <span class="k">self</span>.ui_drag || in_popup(<span class="k">self</span>.pointer) || !scene_rect.contains(<span class="k">self</span>.pointer);
            }
            WindowEvent::MouseInput { state, .. } =&gt; {
                <span class="k">if</span> *state == ElementState::Pressed {
                    <span class="k">self</span>.ui_drag = in_popup(<span class="k">self</span>.pointer) || !scene_rect.contains(<span class="k">self</span>.pointer);
                    <span class="c">// focus now so the first key is not lost</span></code></pre></div>
<p>Replaces the line <code>consumed = in_popup(self.pointer) || !self.scene_rect.con…</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                consumed = in_popup(<span class="k">self</span>.pointer) || !scene_rect.contains(<span class="k">self</span>.pointer)</code></pre></div>
<p>Replaces the 2 lines from <code>self.ui_drag =</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        <span class="k">self</span>.ui_drag = in_popup(<span class="k">self</span>.pointer) || !scene_rect.contains(<span class="k">self</span>.pointer);</code></pre></div>
<p>Replaces the line <code>MODEL.with_borrow_mut(|model| model.drawing_prompt = stat…</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        MODEL.with_borrow_mut(|model| {
            model.drawing_prompt = state.drawing_prompt();
            model.drawing_command = state.drawing_verb().to_owned();
        });
        <span class="k">let</span> drawing = state.drawing_overlay();
        <span class="c">// keep clicks and keys in arrival order</span>
        <span class="k">let</span> <span class="k">mut</span> batches = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> events = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> keyboard = None;
        <span class="k">for</span> event <span class="k">in</span> std::mem::take(&amp;<span class="k">mut</span> input.events) {
            <span class="k">let</span> kind = <span class="k">match</span> &amp;event {
                egui::Event::PointerButton { .. } =&gt; Some(<span class="s">false</span>),
                egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Paste(_) =&gt; {
                    Some(<span class="s">true</span>)
                }
                _ =&gt; None,
            };
            <span class="k">if</span> kind.is_some() &amp;&amp; keyboard.is_some() &amp;&amp; kind != keyboard {
                <span class="k">let</span> <span class="k">mut</span> batch = input.clone();
                batch.events = std::mem::take(&amp;<span class="k">mut</span> events);
                batches.push(batch);
            }
            <span class="k">if</span> kind.is_some() {
                keyboard = kind;
            }
            events.push(event);
        }
        input.events = events;
        batches.push(input);</code></pre></div>
<p>Replaces the 7 lines from <code>let mut output = if let Some(pointer) = pointer_input {</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> batches = batches.into_iter();
        <span class="k">let</span> <span class="k">mut</span> output = <span class="k">self</span>.context.run_ui(batches.next().unwrap(), &amp;<span class="k">mut</span> draw);
        <span class="k">for</span> batch <span class="k">in</span> batches {
            output.append(<span class="k">self</span>.context.run_ui(batch, &amp;<span class="k">mut</span> draw));
        }</code></pre></div>
<p>Added after the line <code>let previous_popup = model.completion_rect.take();</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> polyline_options = model.drawing_command == &quot;<span class="s">polyline</span>&quot;;
    <span class="k">let</span> panel = <span class="k">if</span> model.command_collapsed {
        egui::Panel::bottom(&quot;<span class="s">command-line-collapsed</span>&quot;).exact_size(<span class="k">if</span> polyline_options {
            <span class="s">62</span>.<span class="s">0</span>
        } <span class="k">else</span> {
            <span class="s">34</span>.<span class="s">0</span>
        })
    } <span class="k">else</span> {
        egui::Panel::bottom(&quot;<span class="s">command-line</span>&quot;)
            .default_size(<span class="s">104</span>.<span class="s">0</span>)
            .resizable(<span class="s">true</span>)
            .size_range(
                (<span class="k">if</span> polyline_options { <span class="s">92</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">64</span>.<span class="s">0</span> })
                    ..=(root.available_height() * <span class="s">0</span>.<span class="s">75</span>).max(<span class="s">104</span>.<span class="s">0</span>),
            )
    };
    <span class="k">let</span> panel_response = panel</code></pre></div>
<p>Added after the line <code>.show_inside(root, |ui| {</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ui.set_min_height(ui.max_rect().height());</code></pre></div>
<p>Replaces the 2 lines from <code>.min_scrolled_height(54.0)</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    .min_scrolled_height(<span class="s">0</span>.<span class="s">0</span>)
                    .max_height(
                        (ui.available_height() - <span class="k">if</span> polyline_options { <span class="s">66</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">38</span>.<span class="s">0</span> })
                            .max(<span class="s">0</span>.<span class="s">0</span>),
                    )</code></pre></div>
<p>Added after the line <code>);</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            }
            <span class="c">// Points / Rectangle / Polygon buttons while drawing a polyline</span>
            <span class="k">if</span> polyline_options {
                ui.horizontal_wrapped(|ui| {
                    <span class="k">for</span> (label, text) <span class="k">in</span> [
                        (&quot;<span class="s">Points</span>&quot;, &quot;<span class="s">Polyline Points</span>&quot;),
                        (&quot;<span class="s">Rectangle</span>&quot;, &quot;<span class="s">Polyline Rectangle</span>&quot;),
                        (&quot;<span class="s">Polygon</span>&quot;, &quot;<span class="s">Polyline Polygon</span>&quot;),
                        (&quot;<span class="s">Finish</span>&quot;, &quot;&quot;),
                    ] {
                        <span class="k">let</span> option = ui.button(label);
                        record(controls, &amp;format!(&quot;<span class="s">command/option/</span>{<span class="s">label</span>}&quot;), label, &amp;option);
                        <span class="k">if</span> option.clicked() {
                            *command = Some(text.into());
                            model.command.clear();
                            model.completion_visible = <span class="s">false</span>;
                            model.focus_command = <span class="s">true</span>;
                        }
                    }
                });</code></pre></div>
<p>Replaces the line <code>let inline_options = crate::app::command::options(&amp;model.…</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> option_prefix = <span class="k">if</span> model.inline_suffix {
                    &amp;model.completion_prefix
                } <span class="k">else</span> {
                    &amp;model.command
                };
                <span class="k">let</span> inline_options = <span class="k">if</span> option_prefix.contains('<span class="s"> </span>')
                    &amp;&amp; !option_prefix
                        .trim_start()
                        .to_ascii_lowercase()
                        .starts_with(&quot;<span class="s">polyline</span>&quot;)
                {
                    <span class="k">crate</span>::app::command::options(option_prefix)
                } <span class="k">else</span> {
                    &amp;[]
                };</code></pre></div>
<p>Replaces the line <code>model.command = format!(&quot;{name} &quot;);</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">let</span> (text, run) = <span class="k">crate</span>::app::command::accept(name);
                    <span class="k">if</span> run &amp;&amp; !tab {
                        *command = Some(text);
                        model.command.clear();
                    } <span class="k">else</span> {
                        model.command = <span class="k">if</span> run { format!(&quot;{<span class="s">text</span>}<span class="s"> </span>&quot;) } <span class="k">else</span> { text };
                    }
                    model.completion_visible = <span class="s">false</span>;</code></pre></div>
<p>Added after the line <code>});</code> in <code>lessons/33/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> <span class="k">let</span> Some(controls) = controls {
        <span class="k">let</span> rect = panel_response.response.rect;
        controls.push(Control {
            key: &quot;<span class="s">command/resize</span>&quot;.into(),
            label: &quot;<span class="s">Drag to resize command history</span>&quot;.into(),
            rect: [
                rect.left(),
                rect.top() - <span class="s">4</span>.<span class="s">0</span>,
                rect.right(),
                rect.top() + <span class="s">4</span>.<span class="s">0</span>,
            ],
        });
    }</code></pre></div>
<h2 id="step-4-srccamerars">Step 4 · src/camera.rs<a class="anchor" href="#/course/34-polyline-options#step-4-srccamerars" aria-label="Link to this section">#</a></h2>
<p>A <code>CameraPose</code> is the chosen view without the scene extent, so it can be compared.</p>
<p><code>lessons/34/src/camera.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/33/src/camera.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The part of the camera the user chose: where it looks from and at.</span>
#[derive(Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> CameraPose {
    target: [f64; <span class="s">3</span>],   <span class="c">// the point looked at</span>
    position: [f64; <span class="s">3</span>], <span class="c">// the eye</span>
    up: [f64; <span class="s">3</span>],       <span class="c">// the up direction</span>
    perspective: bool,  <span class="c">// false = orthographic</span>
}

<span class="k">impl</span> Camera {
    <span class="c">/// The current pose.</span>
    <span class="k">pub</span> <span class="k">fn</span> pose(&amp;<span class="k">self</span>) -&gt; CameraPose {
        CameraPose {
            target: <span class="k">self</span>.target,
            position: <span class="k">self</span>.position,
            up: <span class="k">self</span>.up,
            perspective: <span class="k">self</span>.perspective,
        }
    }

    <span class="c">/// Looks at the origin from 3 m, turned 30° and tilted 30° down.</span></code></pre></div>
<p>Added after the line <code>use super::*;</code> in <code>lessons/33/src/camera.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[test]
    <span class="k">fn</span> pose_tracks_navigation_without_tracking_scene_extent() {
        <span class="k">let</span> <span class="k">mut</span> camera = Camera::new();
        <span class="k">let</span> initial = camera.pose();
        camera.scene_extent = <span class="s">100</span>.<span class="s">0</span>;
        assert_eq!(camera.pose(), initial);
        camera.orbit(<span class="s">10</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>);
        assert_ne!(camera.pose(), initial);
        <span class="k">let</span> rotated = camera.pose();
        camera.pan(<span class="s">10</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>);
        assert_ne!(camera.pose(), rotated);
        <span class="k">let</span> panned = camera.pose();
        camera.zoom(<span class="s">1</span>.<span class="s">0</span>);
        assert_ne!(camera.pose(), panned);
        <span class="k">let</span> zoomed = camera.pose();
        camera.toggle_projection();
        assert_ne!(camera.pose(), zoomed);
    }

    <span class="c">/// The depth value of a point \`depth\` meters in front of the eye.</span></code></pre></div>
<h2 id="step-5-srcstaters">Step 5 · src/state.rs<a class="anchor" href="#/course/34-polyline-options#step-5-srcstaters" aria-label="Link to this section">#</a></h2>
<p>The pose at load time is remembered and <code>fit_loaded</code> only fits when it is unchanged.</p>
<p><code>lessons/34/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub camera: Camera,</code> in <code>lessons/33/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    load_camera: <span class="k">crate</span>::camera::CameraPose,                 <span class="c">// the view before loading started</span></code></pre></div>
<p>Added after the line <code>log::info!(&quot;gpu init {:.0} ms&quot;, now_ms() - t0);</code> in <code>lessons/33/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> camera = Camera::new();
        Ok(<span class="k">Self</span> {
            window,
            gpu,
            load_camera: camera.pose(),
            camera,</code></pre></div>
<p>Added after the line <code>pub fn clear(&amp;mut self) {</code> in <code>lessons/33/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.load_camera = <span class="k">self</span>.camera.pose(); <span class="c">// remember the view</span></code></pre></div>
<p>Added after the line <code>self.touch();</code> in <code>lessons/33/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Fit the camera, unless the user already moved it.</span>
    <span class="k">pub</span> <span class="k">fn</span> fit_loaded(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="c">// unchanged since loading started?</span>
        <span class="k">if</span> <span class="k">self</span>.camera.pose() == <span class="k">self</span>.load_camera {
            <span class="k">self</span>.fit_all();
        }</code></pre></div>
<h2 id="step-6-srclibrs">Step 6 · src/lib.rs<a class="anchor" href="#/course/34-polyline-options#step-6-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The loader calls <code>fit_loaded</code> instead of <code>fit_all</code>.</p>
<p><code>lessons/34/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>Msg::Fit =&gt; state.fit_all(),</code> in <code>lessons/33/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::Fit =&gt; state.fit_loaded(),</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/34-polyline-options#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/34/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Type <code>Polyline</code>, pick Rectangle, click two corners; then orbit and reload a scene: the view stays where you put it.</p>
<h2 id="next">Next<a class="anchor" href="#/course/34-polyline-options#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/35-attributes">35 · Attributes On|Off and a phone keyboard</a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcstatedrawingrs",text:"Step 2 · src/state/drawing.rs"},{level:2,id:"step-3-srcappuirs",text:"Step 3 · src/app/ui.rs"},{level:2,id:"step-4-srccamerars",text:"Step 4 · src/camera.rs"},{level:2,id:"step-5-srcstaters",text:"Step 5 · src/state.rs"},{level:2,id:"step-6-srclibrs",text:"Step 6 · src/lib.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"next",text:"Next"}]};export{s as default};
