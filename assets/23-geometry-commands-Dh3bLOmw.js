const s={title:"23 · Create, trim, extend and explode",html:`<h1 id="23-create-trim-extend-and-explode">23 · Create, trim, extend and explode<a class="anchor" href="#/course/23-geometry-commands#23-create-trim-extend-and-explode" aria-label="Link to this section">#</a></h1>
<p>Typed commands create points and curves, then trim, extend or explode the selected source geometry.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/23-geometry-commands#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>Add the modeling command variant, parse its verbs and validate their arguments before dispatch.</p>
<p><code>lessons/23/src/app/command.rs</code> · edit · type this</p>
<p>Replaces the line <code>#[derive(Clone, Copy, Debug, PartialEq)]</code> in <code>lessons/22/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One parsed command line.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> Command {
    Model(<span class="k">crate</span>::app::modeling::Modeling), <span class="c">// create or edit geometry</span>
    Move([f64; <span class="s">3</span>]),                        <span class="c">// move the selection by mm</span>
    Rotate {
        axis: Axis,   <span class="c">// world axis</span>
        degrees: f64, <span class="c">// turn about the selection centre</span>
    },
    Scale(f64), <span class="c">// scale about the selection centre</span></code></pre></div>
<p>Added after the line <code>pub fn parse(line: &amp;str) -&gt; Result&lt;Command, String&gt; {</code> in <code>lessons/22/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> line.len() &gt; <span class="s">65536</span> {
        <span class="k">return</span> Err(&quot;<span class="s">command exceeds 64 KiB</span>&quot;.into());
    }</code></pre></div>
<p>Added after the line <code>let rest: Vec&lt;&amp;str&gt; = words.collect();</code> in <code>lessons/22/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// verbs with a fixed argument count</span>
    <span class="k">let</span> expected = <span class="k">match</span> verb.as_str() {
        &quot;<span class="s">scale</span>&quot; | &quot;<span class="s">s</span>&quot; =&gt; Some(<span class="s">1</span>),
        &quot;<span class="s">rotate</span>&quot; | &quot;<span class="s">rot</span>&quot; =&gt; Some(<span class="s">2</span>),
        &quot;<span class="s">delete</span>&quot; | &quot;<span class="s">del</span>&quot; | &quot;<span class="s">undo</span>&quot; | &quot;<span class="s">redo</span>&quot; | &quot;<span class="s">hide</span>&quot; | &quot;<span class="s">show</span>&quot; | &quot;<span class="s">fit</span>&quot; | &quot;<span class="s">escape</span>&quot; | &quot;<span class="s">esc</span>&quot; =&gt; Some(<span class="s">0</span>),
        _ =&gt; None,
    };

    <span class="k">if</span> expected.is_some_and(|count| rest.len() != count) {
        <span class="k">return</span> Err(format!(&quot;<span class="s">wrong number of arguments for \`</span>{<span class="s">verb</span>}<span class="s">\`</span>&quot;));
    }

    <span class="k">match</span> verb.as_str() {
        &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; | &quot;<span class="s">trim</span>&quot; | &quot;<span class="s">extend</span>&quot; | &quot;<span class="s">explode</span>&quot; =&gt; {
            model(&amp;verb, &amp;rest).map(Command::Model)
        }</code></pre></div>
<p>Added after the line <code>#[test]</code> in <code>lessons/22/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Modeling verbs check their point counts.</span>
    #[test]
    <span class="k">fn</span> modeling_commands_validate_arity_and_coordinates() {
        <span class="k">use</span> <span class="k">crate</span>::app::modeling::Modeling;
        assert_eq!(
            parse(&quot;<span class="s">point 1,2,3</span>&quot;),
            Ok(Command::Model(Modeling::Point([<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>])))
        );
        assert_eq!(
            parse(&quot;<span class="s">trim 0.2 0.8</span>&quot;),
            Ok(Command::Model(Modeling::Trim(<span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">8</span>)))
        );
        assert_eq!(parse(&quot;<span class="s">explode</span>&quot;), Ok(Command::Model(Modeling::Explode)));

        <span class="k">for</span> line <span class="k">in</span> [
            &quot;<span class="s">point @1,2,3</span>&quot;,
            &quot;<span class="s">line 0,0,0</span>&quot;,
            &quot;<span class="s">trim 0 1 extra</span>&quot;,
            &quot;<span class="s">explode extra</span>&quot;,
            &quot;<span class="s">scale 2 extra</span>&quot;,
            &quot;<span class="s">delete extra</span>&quot;,
        ] {
            assert!(parse(line).is_err(), &quot;{<span class="s">line</span>}&quot;);
        }

        assert!(parse(&amp;&quot;<span class="s">x</span>&quot;.repeat(<span class="s">65537</span>)).is_err());
    }

    <span class="c">/// Rotate carries its axis.</span></code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/22/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A modeling command from its verb and points.</span>
<span class="k">fn</span> model(verb: &amp;str, words: &amp;[&amp;str]) -&gt; Result&lt;<span class="k">crate</span>::app::modeling::Modeling, String&gt; {
    <span class="k">use</span> <span class="k">crate</span>::app::modeling::Modeling;

    <span class="k">match</span> verb {
        &quot;<span class="s">explode</span>&quot; <span class="k">if</span> words.is_empty() =&gt; Ok(Modeling::Explode),
        &quot;<span class="s">trim</span>&quot; | &quot;<span class="s">extend</span>&quot; <span class="k">if</span> words.len() == <span class="s">2</span> =&gt; {
            <span class="k">let</span> a = number(words.first().copied(), &quot;<span class="s">trim 0.2 0.8</span>&quot;)?;
            <span class="k">let</span> b = number(words.get(<span class="s">1</span>).copied(), &quot;<span class="s">trim 0.2 0.8</span>&quot;)?;
            Ok(<span class="k">if</span> verb == &quot;<span class="s">trim</span>&quot; {
                Modeling::Trim(a, b)
            } <span class="k">else</span> {
                Modeling::Extend(a, b)
            })
        }
        &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; =&gt; {
            <span class="k">let</span> <span class="k">mut</span> points = Vec::new();

            <span class="k">if</span> words.len() &gt; <span class="k">crate</span>::app::modeling::MAX_POINTS {
                <span class="k">return</span> Err(&quot;<span class="s">too many points</span>&quot;.into());
            }

            <span class="k">for</span> word <span class="k">in</span> words {
                <span class="k">let</span> Some(coords::Typed::Absolute { x, y, z }) = coords::parse(word) <span class="k">else</span> {
                    <span class="k">return</span> Err(&quot;<span class="s">use world coordinates x,y,z separated by spaces</span>&quot;.into());
                };
                points.push([x, y, z.unwrap_or(<span class="s">0</span>.<span class="s">0</span>)]);
            }

            <span class="k">match</span> (verb, points.len()) {
                (&quot;<span class="s">point</span>&quot;, <span class="s">1</span>) =&gt; Ok(Modeling::Point(points[<span class="s">0</span>])),
                (&quot;<span class="s">line</span>&quot;, <span class="s">2</span>) =&gt; Ok(Modeling::Line(points[<span class="s">0</span>], points[<span class="s">1</span>])),
                (&quot;<span class="s">polyline</span>&quot;, <span class="s">2</span>..) =&gt; Ok(Modeling::Polyline(points)),
                _ =&gt; Err(&quot;<span class="s">point needs one coordinate; line two; polyline at least two</span>&quot;.into()),
            }
        }
        _ =&gt; Err(&quot;<span class="s">try trim 0.2 0.8, extend -0.2 1.2, or explode</span>&quot;.into()),
    }
}</code></pre></div>
<h2 id="step-2-srcappmodrs">Step 2 · src/app/mod.rs<a class="anchor" href="#/course/23-geometry-commands#step-2-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/23/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod manifest;</code> in <code>lessons/22/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> modeling;</code></pre></div>
<h2 id="step-3-srcappmodelingrs">Step 3 · src/app/modeling.rs<a class="anchor" href="#/course/23-geometry-commands#step-3-srcappmodelingrs" aria-label="Link to this section">#</a></h2>
<p>Create points and curves, or trim, extend and explode selected geometry in a document transaction.</p>
<p><code>lessons/23/src/app/modeling.rs</code> · 315 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! A geometry command collects the points its verb needs, then builds one object and adds it to the scene.</span>

<span class="k">use</span> <span class="k">crate</span>::app::scene::FileDoc;
<span class="k">use</span> <span class="k">crate</span>::app::scene::Scene;
<span class="k">use</span> session_rust::Geometry;
<span class="k">use</span> session_rust::Line;
<span class="k">use</span> session_rust::Point;
<span class="k">use</span> session_rust::Polyline;
<span class="k">use</span> session_rust::Session;
<span class="k">use</span> session_rust::Xform;
<span class="k">use</span> std::rc::Rc;

<span class="c">/// Most points one command may create.</span>
<span class="k">pub</span> <span class="k">const</span> MAX_POINTS: usize = <span class="s">4096</span>;

<span class="c">/// One geometry command.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> Modeling {
    Point([f64; <span class="s">3</span>]),          <span class="c">// create a point</span>
    Line([f64; <span class="s">3</span>], [f64; <span class="s">3</span>]), <span class="c">// create a line</span>
    Polyline(Vec&lt;[f64; 3]&gt;),  <span class="c">// create a polyline</span>
    Trim(f64, f64),           <span class="c">// keep this part of the selected curve, 0..1</span>
    Extend(f64, f64),         <span class="c">// extend the selected curve to this range</span>
    Explode,                  <span class="c">// split the selected polyline into lines</span>
}

<span class="k">impl</span> Scene {
    <span class="c">/// Run one geometry command as one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> model(&amp;<span class="k">mut</span> <span class="k">self</span>, command: &amp;Modeling) -&gt; Result&lt;(), String&gt; {
        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> Err(&quot;<span class="s">geometry edits require a scene without streamed sources</span>&quot;.into());
        }

        <span class="k">match</span> command {
            Modeling::Point(p) =&gt; <span class="k">self</span>.create_geometry(Geometry::Point(Rc::new(point(*p)?))),
            Modeling::Line(a, b) =&gt; {
                <span class="k">let</span> line = Line::from_points(&amp;point(*a)?, &amp;point(*b)?);

                <span class="k">if</span> line.length() &lt;= <span class="s">1</span>e-<span class="s">12</span> {
                    <span class="k">return</span> Err(&quot;<span class="s">line endpoints must differ</span>&quot;.into());
                }

                <span class="k">self</span>.create_geometry(Geometry::Line(Rc::new(line)))
            }
            Modeling::Polyline(points) =&gt; {
                <span class="k">if</span> !(<span class="s">2</span>..=MAX_POINTS).contains(&amp;points.len()) {
                    <span class="k">return</span> Err(format!(&quot;<span class="s">polyline needs 2–</span>{<span class="s">MAX_POINTS</span>}<span class="s"> points</span>&quot;));
                }

                <span class="k">let</span> points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::&lt;Result&lt;Vec&lt;_&gt;, _&gt;&gt;()?;
                <span class="k">self</span>.create_geometry(Geometry::Polyline(Rc::new(Polyline::new(points))))
            }
            _ =&gt; <span class="k">self</span>.edit_geometry(command),
        }
    }

    <span class="c">/// Add a geometry to the \`Created\` document, making it if needed.</span>
    <span class="k">fn</span> create_geometry(&amp;<span class="k">mut</span> <span class="k">self</span>, geometry: Geometry) -&gt; Result&lt;(), String&gt; {
        <span class="k">let</span> index = <span class="k">self</span>.created_doc;
        <span class="k">let</span> doc = <span class="k">match</span> index {
            Some(index) =&gt; index,
            None =&gt; {
                <span class="k">self</span>.docs.push(FileDoc {
                    name: &quot;<span class="s">Created</span>&quot;.into(),
                    session: Rc::new(Session::new(&quot;<span class="s">Created</span>&quot;)),
                    place: Xform::identity(),
                    point_px: <span class="s">0</span>.<span class="s">0</span>,
                    display_only: <span class="s">false</span>,
                });
                <span class="k">self</span>.docs.len() - <span class="s">1</span>
            }
        };
        <span class="k">self</span>.created_doc = Some(doc);
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
        session.begin(&quot;<span class="s">create</span>&quot;);

        <span class="k">match</span> geometry {
            Geometry::Point(p) =&gt; {
                session.add_point((*p).clone(), None);
            }
            Geometry::Line(line) =&gt; {
                session.add_line((*line).clone(), None);
            }
            Geometry::Polyline(line) =&gt; {
                <span class="k">let</span> added = session.add_polyline((*line).clone(), None);
                debug_assert!(added.is_some());
            }
            _ =&gt; unreachable!(),
        }

        session.commit();
        <span class="k">self</span>.last_edited = Some(doc);
        Ok(())
    }

    <span class="c">/// Trim, extend or explode the selected object.</span>
    <span class="k">fn</span> edit_geometry(&amp;<span class="k">mut</span> <span class="k">self</span>, command: &amp;Modeling) -&gt; Result&lt;(), String&gt; {
        <span class="k">let</span> row = <span class="k">self</span>.selected.ok_or(&quot;<span class="s">select one object first</span>&quot;)?;
        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(row).ok_or(&quot;<span class="s">object no longer exists</span>&quot;)?;
        <span class="k">let</span> file = <span class="k">self</span>.docs.get(doc).ok_or(&quot;<span class="s">this object has no document</span>&quot;)?;

        <span class="k">if</span> file.display_only {
            <span class="k">return</span> Err(&quot;<span class="s">this document is display only</span>&quot;.into());
        }

        <span class="k">let</span> source = <span class="k">self</span>.geometry(row).ok_or(&quot;<span class="s">source geometry is unavailable</span>&quot;)?;

        <span class="k">if</span> matches!(command, Modeling::Explode) {
            <span class="k">let</span> Geometry::Polyline(line) = source <span class="k">else</span> {
                <span class="k">return</span> Err(&quot;<span class="s">explode currently accepts polylines</span>&quot;.into());
            };

            <span class="k">if</span> line.point_count() &gt; MAX_POINTS {
                <span class="k">return</span> Err(format!(&quot;<span class="s">explode is limited to </span>{<span class="s">MAX_POINTS</span>}<span class="s"> points</span>&quot;));
            }

            <span class="k">if</span> file
                .session
                .tree
                .get_node_by_name(&amp;guid)
                .is_some_and(|node| !node.borrow().is_leaf())
            {
                <span class="k">return</span> Err(&quot;<span class="s">explode requires an object without child geometry</span>&quot;.into());
            }

            <span class="k">let</span> points = line.get_points();
            <span class="k">let</span> width = line.width;
            <span class="k">let</span> dash = line.dash.clone();
            <span class="k">let</span> color = line.linecolor.clone();
            <span class="k">let</span> place = file.session.world_xform(&amp;guid); <span class="c">// the lines keep the placement</span>
            <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
            session.begin(&quot;<span class="s">explode</span>&quot;);

            <span class="k">if</span> !session.remove_object(&amp;guid) {
                session.commit();
                <span class="k">return</span> Err(&quot;<span class="s">object no longer exists</span>&quot;.into());
            }

            <span class="k">for</span> pair <span class="k">in</span> points.windows(<span class="s">2</span>) {
                <span class="k">let</span> <span class="k">mut</span> line = Line::from_points(&amp;pair[<span class="s">0</span>], &amp;pair[<span class="s">1</span>]);
                line.width = width;
                line.dash = dash.clone();
                line.linecolor = color.clone();
                <span class="k">let</span> node = session.add_line(line, None);
                session.set_xform(&amp;node.borrow().name, place.clone());
            }

            session.commit();
        } <span class="k">else</span> {
            <span class="k">let</span> next = edited(source, command)?;
            <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
            session.begin(&quot;<span class="s">edit geometry</span>&quot;);
            <span class="k">let</span> replaced = session.replace(&amp;guid, next);
            session.commit();

            <span class="k">if</span> !replaced {
                <span class="k">return</span> Err(&quot;<span class="s">object no longer exists</span>&quot;.into());
            }
        }

        <span class="k">self</span>.last_edited = Some(doc);
        Ok(())
    }
}

<span class="c">/// A point from finite, reasonable coordinates.</span>
<span class="k">fn</span> point(p: [f64; <span class="s">3</span>]) -&gt; Result&lt;Point, String&gt; {
    <span class="k">if</span> p.iter().any(|v| !v.is_finite() || v.abs() &gt; <span class="s">1</span>e<span class="s">12</span>) {
        <span class="k">return</span> Err(&quot;<span class="s">coordinates must be finite and within ±1e12</span>&quot;.into());
    }

    Ok(Point::new(p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]))
}

<span class="c">/// The source trimmed or extended to \`a..b\` of its length.</span>
<span class="k">fn</span> edited(source: &amp;Geometry, command: &amp;Modeling) -&gt; Result&lt;Geometry, String&gt; {
    <span class="k">let</span> (a, b, trim) = <span class="k">match</span> *command {
        Modeling::Trim(a, b) =&gt; (a, b, <span class="s">true</span>),
        Modeling::Extend(a, b) =&gt; (a, b, <span class="s">false</span>),
        _ =&gt; <span class="k">return</span> Err(&quot;<span class="s">expected trim or extend</span>&quot;.into()),
    };

    <span class="k">if</span> !a.is_finite() || !b.is_finite() || a &gt;= b || a.abs() &gt; <span class="s">1</span>e<span class="s">6</span> || b.abs() &gt; <span class="s">1</span>e<span class="s">6</span> {
        <span class="k">return</span> Err(&quot;<span class="s">parameters must be finite, increasing, and within ±1e6</span>&quot;.into());
    }

    <span class="k">if</span> (trim &amp;&amp; (a &lt; <span class="s">0</span>.<span class="s">0</span> || b &gt; <span class="s">1</span>.<span class="s">0</span>)) || (!trim &amp;&amp; (a &gt; <span class="s">0</span>.<span class="s">0</span> || b &lt; <span class="s">1</span>.<span class="s">0</span>)) {
        <span class="k">return</span> Err(&quot;<span class="s">trim keeps 0 ≤ a &lt; b ≤ 1; extend needs a ≤ 0 and b ≥ 1</span>&quot;.into());
    }

    <span class="k">match</span> source {
        Geometry::Line(line) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> next = Line::from_points(&amp;line.point_at(a), &amp;line.point_at(b));
            next.name = line.name.clone();
            next.linecolor = line.linecolor.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            Ok(Geometry::Line(Rc::new(next)))
        }
        Geometry::NurbsCurve(curve) =&gt; {
            <span class="k">if</span> curve.m_cv_count &gt; MAX_POINTS {
                <span class="k">return</span> Err(format!(&quot;<span class="s">curve edits are limited to </span>{<span class="s">MAX_POINTS</span>}<span class="s"> controls</span>&quot;));
            }

            <span class="k">let</span> <span class="k">mut</span> next = (**curve).clone();
            <span class="k">let</span> (lo, hi) = next.domain();
            <span class="k">let</span> a = lo + a * (hi - lo); <span class="c">// 0..1 into the curve domain</span>
            <span class="k">let</span> b = lo + b * (hi - lo);
            <span class="k">let</span> ok = <span class="k">if</span> trim {
                next.trim(a, b)
            } <span class="k">else</span> {
                next.extend(a, b)
            };

            <span class="k">if</span> !ok {
                <span class="k">return</span> Err(&quot;<span class="s">kernel refused this curve interval</span>&quot;.into());
            }

            Ok(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ =&gt; Err(&quot;<span class="s">trim and extend currently accept lines and NURBS curves</span>&quot;.into()),
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A scene with one selected polyline.</span>
    <span class="k">fn</span> scene(geometry: Polyline) -&gt; Scene {
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">test</span>&quot;);
        assert!(session.add_polyline(geometry, None).is_some());
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(FileDoc {
            name: &quot;<span class="s">test</span>&quot;.into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">false</span>,
        });
        scene.selected = Some(<span class="s">0</span>);
        scene
    }

    <span class="c">/// A created point undoes and redoes; NaN is refused.</span>
    #[test]
    <span class="k">fn</span> creation_undo_redo_and_invalid_input() {
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        assert!(scene.model(&amp;Modeling::Point([f64::NAN, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>])).is_err());
        assert!(scene.docs.is_empty());
        scene.model(&amp;Modeling::Point([<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>])).unwrap();
        assert_eq!(scene.docs[<span class="s">0</span>].session.lookup.len(), <span class="s">1</span>);
        assert!(scene.undo());
        assert!(scene.docs[<span class="s">0</span>].session.lookup.is_empty());
        assert!(scene.redo());
        assert_eq!(scene.docs[<span class="s">0</span>].session.lookup.len(), <span class="s">1</span>);
    }

    <span class="c">/// Trim and extend keep the pen and check their ranges.</span>
    #[test]
    <span class="k">fn</span> trim_and_extend_preserve_style_and_intervals() {
        <span class="k">let</span> <span class="k">mut</span> line = Line::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        line.width = <span class="s">3</span>.<span class="s">0</span>;
        <span class="k">let</span> source = Geometry::Line(Rc::new(line));
        <span class="k">let</span> Geometry::Line(trim) = edited(&amp;source, &amp;Modeling::Trim(<span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">8</span>)).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(trim.start()[<span class="s">0</span>], <span class="s">2</span>.<span class="s">0</span>);
        assert_eq!(trim.end()[<span class="s">0</span>], <span class="s">8</span>.<span class="s">0</span>);
        assert_eq!(trim.width, <span class="s">3</span>.<span class="s">0</span>);
        <span class="k">let</span> Geometry::Line(extend) = edited(&amp;source, &amp;Modeling::Extend(-<span class="s">0</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">5</span>)).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(extend.start()[<span class="s">0</span>], -<span class="s">5</span>.<span class="s">0</span>);
        assert_eq!(extend.end()[<span class="s">0</span>], <span class="s">15</span>.<span class="s">0</span>);
        assert!(edited(&amp;source, &amp;Modeling::Trim(-<span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">8</span>)).is_err());
        assert!(edited(&amp;source, &amp;Modeling::Extend(<span class="s">0</span>.<span class="s">1</span>, <span class="s">1</span>.<span class="s">5</span>)).is_err());
    }

    <span class="c">/// Curve trim takes 0..1 of the domain.</span>
    #[test]
    <span class="k">fn</span> curve_trim_uses_normalized_domain() {
        <span class="k">let</span> curve = session_rust::NurbsCurve::create(
            <span class="s">false</span>,
            <span class="s">1</span>,
            &amp;[Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)],
        );
        <span class="k">let</span> source = Geometry::NurbsCurve(Rc::new(curve));
        <span class="k">let</span> Geometry::NurbsCurve(curve) = edited(&amp;source, &amp;Modeling::Trim(<span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">8</span>)).unwrap()
        <span class="k">else</span> {
            panic!()
        };
        assert!((curve.point_at_start()[<span class="s">0</span>] - <span class="s">2</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
        assert!((curve.point_at_end()[<span class="s">0</span>] - <span class="s">8</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
    }

    <span class="c">/// Explode is one undo step and keeps the placement.</span>
    #[test]
    <span class="k">fn</span> explode_is_one_transaction_and_preserves_placement() {
        <span class="k">let</span> <span class="k">mut</span> scene = scene(Polyline::new(vec![
            Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        ]));
        <span class="k">let</span> (_, guid) = scene.identity_of(<span class="s">0</span>).unwrap();
        Rc::make_mut(&amp;<span class="k">mut</span> scene.docs[<span class="s">0</span>].session)
            .set_xform(&amp;guid, Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        scene.model(&amp;Modeling::Explode).unwrap();
        <span class="k">let</span> session = &amp;scene.docs[<span class="s">0</span>].session;
        assert_eq!(session.lookup.len(), <span class="s">2</span>);

        <span class="k">for</span> guid <span class="k">in</span> session.lookup.keys() {
            assert_eq!(session.world_xform(guid).m[<span class="s">12</span>], <span class="s">5</span>.<span class="s">0</span>);
        }

        assert!(scene.undo());
        assert_eq!(scene.docs[<span class="s">0</span>].session.lookup.len(), <span class="s">1</span>);
        assert!(matches!(
            scene.docs[<span class="s">0</span>].session.lookup.get(guid.as_ref()),
            Some(Geometry::Polyline(_))
        ));
        assert!(scene.redo());
        assert_eq!(scene.docs[<span class="s">0</span>].session.lookup.len(), <span class="s">2</span>);
    }
}</code></pre></div>
<h2 id="step-4-srcappsceners">Step 4 · src/app/scene.rs<a class="anchor" href="#/course/23-geometry-commands#step-4-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Make the scene helpers available to the modeling module.</p>
<p><code>lessons/23/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the line <code>pub last_edited: Option&lt;usize&gt;,</code> in <code>lessons/22/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) created_doc: Option&lt;usize&gt;,</code></pre></div>
<p>Added after the line <code>last_edited: None,</code> in <code>lessons/22/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            created_doc: None,
        }
    }

    <span class="c">/// Drop every document and its GPU rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">self</span>.created_doc = None;
        <span class="k">self</span>.last_edited = None;</code></pre></div>
<h2 id="step-5-srcstateeditrs">Step 5 · src/state/edit.rs<a class="anchor" href="#/course/23-geometry-commands#step-5-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Dispatch a modeling command, rebuild its display and report the result.</p>
<p><code>lessons/23/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn run_command(&amp;mut self, line: &amp;str) -&gt; Result&lt;Strin…</code> in <code>lessons/22/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_gesture();
        <span class="k">let</span> command = <span class="k">crate</span>::app::command::parse(line)?;

        <span class="k">if</span> matches!(command, Command::Delete | Command::Undo | Command::Redo)
            &amp;&amp; (!<span class="k">self</span>.scene.streamed.is_empty() || !<span class="k">self</span>.scene.sheets.is_empty())
        {
            <span class="k">return</span> Err(&quot;<span class="s">this command requires a scene without streamed sources</span>&quot;.into());
        }

        <span class="c">// commands that act on the selection</span></code></pre></div>
<p>Added after the line <code>match command {</code> in <code>lessons/22/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Model(command) =&gt; {
                <span class="k">self</span>.scene.model(&amp;command)?;
                <span class="k">self</span>.after_history();
                Ok(&quot;<span class="s">geometry updated</span>&quot;.into())
            }</code></pre></div>
<p>Replaces the line <code>self.delete_selected();</code> in <code>lessons/22/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> row = <span class="k">self</span>.scene.selected.ok_or(&quot;<span class="s">nothing is selected</span>&quot;)?;

                <span class="k">if</span> !<span class="k">self</span>.scene.delete_row(row) {
                    <span class="k">return</span> Err(&quot;<span class="s">this object cannot be deleted</span>&quot;.into());
                }

                <span class="k">self</span>.after_history();
                Ok(&quot;<span class="s">deleted</span>&quot;.into())
            }
            Command::Undo =&gt; {
                <span class="k">if</span> !<span class="k">self</span>.scene.undo() {
                    <span class="k">return</span> Err(&quot;<span class="s">nothing to undo</span>&quot;.into());
                }

                <span class="k">self</span>.after_history();
                Ok(&quot;<span class="s">undone</span>&quot;.into())
            }
            Command::Redo =&gt; {
                <span class="k">if</span> !<span class="k">self</span>.scene.redo() {
                    <span class="k">return</span> Err(&quot;<span class="s">nothing to redo</span>&quot;.into());
                }

                <span class="k">self</span>.after_history();</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/23-geometry-commands#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/23/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: a Point or Line command adds selected geometry and reports <strong>geometry updated</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-modeling.png" alt="Full viewer result for lesson 23" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A malformed command changes the scene: argument validation happens after mutation.</li>
<li>Explode needs several Undo actions: each piece is committed separately.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/23-geometry-commands#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/23/src/
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
│   ├── edit.rs
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── input.rs
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs  +
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
│   │   └── view.rs
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
│   └── triangle_tiles.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: command text → validated modeling operation → source transaction → existing draw paths.
Every file at this point: <code>lessons/23/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/23-geometry-commands#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/24-placed-controls">24 · Make control dragging respect object placement</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/23-geometry-commands#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>A newly created line has been trimmed to its middle 60% and selected. Compare its shortened extent with the other geometry in the full viewer. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>. At this checkpoint the command field still uses the original DOM interface, and the handles are drawn with strokes. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-modeling.png"><img src="/session/docs/course/docs/screenshots/extensions-modeling.png" alt="Full viewer result for lesson 23" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcappmodrs",text:"Step 2 · src/app/mod.rs"},{level:2,id:"step-3-srcappmodelingrs",text:"Step 3 · src/app/modeling.rs"},{level:2,id:"step-4-srcappsceners",text:"Step 4 · src/app/scene.rs"},{level:2,id:"step-5-srcstateeditrs",text:"Step 5 · src/state/edit.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
