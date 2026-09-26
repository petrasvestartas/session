const s={title:"13 · Source controls",html:`<h1 id="13-source-controls">13 · Source controls<a class="anchor" href="#/course/13-controls#13-source-controls" aria-label="Link to this section">#</a></h1>
<p>F10 shows original curve and surface controls, and clicking a control highlights it.</p>
<p><img src="/session/docs/course/docs/illustrations/cloud-pick.svg" alt="A resident prefix is a fraction of the cloud, so a click walks every intersecting octree node whether or not it was downloaded, and accumulates the answer one bounded page at a time against the depth the frame already has." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcappselectionrs">Step 1 · src/app/selection.rs<a class="anchor" href="#/course/13-controls#step-1-srcappselectionrs" aria-label="Link to this section">#</a></h2>
<p>Add control points: a selection can now hold one object&#39;s controls, collected from its kernel geometry.</p>
<p><code>lessons/13/src/app/selection.rs</code> · edit · type this</p>
<p>Added at the top of <code>lessons/12/src/app/selection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The picked object lives in Scene; this says what inside that object is picked, if anything.</span>
<span class="k">use</span> session_rust::element::ElementGeometry;
<span class="k">use</span> session_rust::{Geometry, NurbsCurve, NurbsSurface, Point};

<span class="c">/// Which control point, in the kernel's own numbering: a vertex, or a CV (control vertex) of a curve or surface.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
<span class="k">pub</span> <span class="k">enum</span> ControlId {
    Vertex(usize), <span class="c">// mesh, polyline or BRep vertex</span>
    Curve { curve: usize, point: usize },
    Surface { surface: usize, u: usize, v: usize },
    Point(u32), <span class="c">// one point of a cloud</span>
}

<span class="c">/// \`#[default]\` marks the variant that \`Default::default()\` returns.</span></code></pre></div>
<p><code>lessons/13/src/app/selection.rs</code> · edit · type this</p>
<p>Replaces the 24 lines from <code>}</code> of <code>lessons/12/src/app/selection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Controls {
        parent: u32, <span class="c">// object row</span>
        selected: Option&lt;ControlId&gt;,
        cloud: bool, <span class="c">// parent is a point cloud</span>
    },
}

<span class="k">impl</span> SelectionMode {
    <span class="c">/// The object holding the sub-selection; None when the whole object is selected.</span>
    <span class="k">pub</span> <span class="k">fn</span> parent(&amp;<span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        <span class="k">match</span> <span class="k">self</span> {
            <span class="k">Self</span>::Object =&gt; None,
            <span class="k">Self</span>::Edge { parent, .. } | <span class="k">Self</span>::Controls { parent, .. } =&gt; Some(*parent),
        }
    }

    <span class="k">pub</span> <span class="k">fn</span> select_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, parent: u32, edge: u32) {
        *<span class="k">self</span> = <span class="k">Self</span>::Edge { parent, edge };
    }

    <span class="c">/// Show controls of \`parent\`; false when nothing changed.</span>
    <span class="k">pub</span> <span class="k">fn</span> enable_controls(&amp;<span class="k">mut</span> <span class="k">self</span>, parent: Option&lt;u32&gt;, cloud: bool) -&gt; bool {
        <span class="k">let</span> Some(parent) = parent <span class="k">else</span> { <span class="k">return</span> <span class="s">false</span> };

        <span class="c">// already showing this object's controls</span>
        <span class="k">if</span> matches!(<span class="k">self</span>, <span class="k">Self</span>::Controls { parent: active, .. } <span class="k">if</span> *active == parent) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        *<span class="k">self</span> = <span class="k">Self</span>::Controls {
            parent,
            selected: None,
            cloud,
        };
        <span class="s">true</span>
    }

    <span class="c">/// Back to object selection; returns the parent.</span>
    <span class="k">pub</span> <span class="k">fn</span> escape(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        <span class="k">let</span> parent = <span class="k">self</span>.parent();
        *<span class="k">self</span> = <span class="k">Self</span>::Object;
        parent
    }
}

<span class="c">/// One control point and where it is.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Control {
    <span class="k">pub</span> id: ControlId,
    <span class="k">pub</span> position: [f64; <span class="s">3</span>],
}

<span class="c">/// Every control of one object and the lines between them.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> Controls {
    <span class="k">pub</span> points: Vec&lt;Control&gt;,
    <span class="k">pub</span> links: Vec&lt;[usize; 2]&gt;, <span class="c">// lines of the control polygon or net, as index pairs into points</span>
    <span class="k">pub</span> cloud: bool, <span class="c">// a point cloud, controls stay on the GPU</span>
}

<span class="k">impl</span> Controls {
    <span class="c">/// Add one control; None when its position is not finite.</span>
    <span class="k">fn</span> push(&amp;<span class="k">mut</span> <span class="k">self</span>, id: ControlId, point: &amp;Point) -&gt; Option&lt;usize&gt; {
        <span class="k">let</span> position = [point[<span class="s">0</span>], point[<span class="s">1</span>], point[<span class="s">2</span>]];

        <span class="k">if</span> !position.into_iter().all(f64::is_finite) {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> index = <span class="k">self</span>.points.len();
        <span class="k">self</span>.points.push(Control { id, position });
        Some(index)
    }

    <span class="k">pub</span> <span class="k">fn</span> from_geometry(geometry: &amp;Geometry) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> controls = <span class="k">Self</span>::default();
        controls.append_geometry(geometry);
        controls
    }

    <span class="c">/// One arm per kind; a plane or a box has no controls.</span>
    <span class="k">fn</span> append_geometry(&amp;<span class="k">mut</span> <span class="k">self</span>, geometry: &amp;Geometry) {
        <span class="k">match</span> geometry {
            Geometry::Mesh(mesh) =&gt; <span class="k">self</span>.mesh(mesh),
            Geometry::Line(line) =&gt; {
                <span class="k">self</span>.push(ControlId::Vertex(<span class="s">0</span>), &amp;line.start());
                <span class="k">self</span>.push(ControlId::Vertex(<span class="s">1</span>), &amp;line.end());

                <span class="c">// link only when both ends were finite</span>
                <span class="k">if</span> <span class="k">self</span>.points.len() == <span class="s">2</span> {
                    <span class="k">self</span>.links.push([<span class="s">0</span>, <span class="s">1</span>]);
                }
            }
            Geometry::Polyline(polyline) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> previous = None;

                <span class="k">for</span> (index, coords) <span class="k">in</span> polyline.coords.chunks_exact(<span class="s">3</span>).enumerate() {
                    <span class="k">let</span> current = <span class="k">self</span>.push(
                        ControlId::Vertex(index),
                        &amp;Point::new(coords[<span class="s">0</span>], coords[<span class="s">1</span>], coords[<span class="s">2</span>]),
                    );

                    <span class="k">if</span> <span class="k">let</span> (Some(start), Some(end)) = (previous, current) {
                        <span class="k">self</span>.links.push([start, end]);
                    }

                    previous = current;
                }
            }
            Geometry::NurbsCurve(curve) =&gt; <span class="k">self</span>.curve(curve, <span class="s">0</span>),
            Geometry::NurbsSurface(surface) =&gt; <span class="k">self</span>.surface(surface, <span class="s">0</span>),
            Geometry::BRep(brep) =&gt; <span class="k">self</span>.brep(brep),
            Geometry::PointCloud(_) =&gt; <span class="k">self</span>.cloud = <span class="s">true</span>,
            Geometry::Point(point) =&gt; {
                <span class="k">self</span>.push(ControlId::Vertex(<span class="s">0</span>), point);
            }
            Geometry::Element(element) =&gt; <span class="k">match</span> element.geometry() {
                ElementGeometry::Mesh(mesh) =&gt; <span class="k">self</span>.mesh(mesh),
                ElementGeometry::BRep(brep) =&gt; <span class="k">self</span>.brep(brep),
                ElementGeometry::None =&gt; {}
            },
            Geometry::Plane(_) | Geometry::OBB(_) =&gt; {}
        }
    }

    <span class="c">/// Every mesh vertex.</span>
    <span class="k">fn</span> mesh(&amp;<span class="k">mut</span> <span class="k">self</span>, mesh: &amp;session_rust::Mesh) {
        <span class="k">for</span> key <span class="k">in</span> mesh.vertices() {
            <span class="k">if</span> <span class="k">let</span> Some(vertex) = mesh.vertex.get(&amp;key) {
                <span class="k">self</span>.push(
                    ControlId::Vertex(key),
                    &amp;Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }

    <span class="c">/// BRep vertices, then curve and surface nets.</span>
    <span class="k">fn</span> brep(&amp;<span class="k">mut</span> <span class="k">self</span>, brep: &amp;session_rust::BRep) {
        <span class="k">for</span> (index, vertex) <span class="k">in</span> brep.m_vertices.iter().enumerate() {
            <span class="k">self</span>.push(ControlId::Vertex(index), &amp;vertex.point);
        }

        <span class="k">for</span> (index, curve) <span class="k">in</span> brep.m_curves_3d.iter().enumerate() {
            <span class="k">self</span>.curve(curve, index);
        }

        <span class="k">for</span> (index, surface) <span class="k">in</span> brep.m_surfaces.iter().enumerate() {
            <span class="k">self</span>.surface(surface, index);
        }
    }

    <span class="c">/// The control polygon: the CVs joined in order; the curve follows it loosely.</span>
    <span class="k">fn</span> curve(&amp;<span class="k">mut</span> <span class="k">self</span>, curve: &amp;NurbsCurve, index: usize) {
        <span class="k">let</span> <span class="k">mut</span> previous = None;

        <span class="k">for</span> point <span class="k">in</span> <span class="s">0</span>..curve.cv_count() {
            <span class="k">let</span> current = <span class="k">match</span> curve.get_cv(point) {
                Some(position) =&gt; <span class="k">self</span>.push(
                    ControlId::Curve {
                        curve: index,
                        point,
                    },
                    &amp;position,
                ),
                None =&gt; None,
            };

            <span class="k">if</span> <span class="k">let</span> (Some(start), Some(end)) = (previous, current) {
                <span class="k">self</span>.links.push([start, end]);
            }

            previous = current;
        }
    }

    <span class="c">/// The control net: a grid of CVs, joined along u and along v.</span>
    <span class="k">fn</span> surface(&amp;<span class="k">mut</span> <span class="k">self</span>, surface: &amp;NurbsSurface, index: usize) {
        <span class="k">let</span> [width, height] = surface.m_cv_count;
        <span class="k">let</span> <span class="k">mut</span> previous = vec![None; height]; <span class="c">// previous[v] = the CV at (u - 1, v)</span>

        <span class="k">for</span> u <span class="k">in</span> <span class="s">0</span>..width {
            <span class="k">let</span> <span class="k">mut</span> last = None; <span class="c">// the CV at (u, v - 1)</span>

            <span class="k">for</span> (v, above) <span class="k">in</span> previous.iter_mut().enumerate() {
                <span class="k">let</span> current = <span class="k">match</span> surface.get_cv(u, v) {
                    Some(position) =&gt; <span class="k">self</span>.push(
                        ControlId::Surface {
                            surface: index,
                            u,
                            v,
                        },
                        &amp;position,
                    ),
                    None =&gt; None,
                };

                <span class="k">if</span> <span class="k">let</span> (Some(start), Some(end)) = (*above, current) {
                    <span class="k">self</span>.links.push([start, end]);
                }

                <span class="k">if</span> <span class="k">let</span> (Some(start), Some(end)) = (last, current) {
                    <span class="k">self</span>.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A new parent replaces the old selection; escape keeps the parent.</span>
    #[test]
    <span class="k">fn</span> switching_parent_replaces_specialized_selection_and_escape_retains_parent() {
        <span class="k">let</span> <span class="k">mut</span> mode = SelectionMode::default();
        mode.select_edge(<span class="s">4</span>, <span class="s">17</span>);
        assert!(mode.enable_controls(Some(<span class="s">4</span>), <span class="s">false</span>));
        assert!(!mode.enable_controls(Some(<span class="s">4</span>), <span class="s">false</span>));
        mode.select_edge(<span class="s">9</span>, <span class="s">3</span>);
        assert_eq!(mode, SelectionMode::Edge { parent: <span class="s">9</span>, edge: <span class="s">3</span> });
        assert_eq!(mode.escape(), Some(<span class="s">9</span>));
        assert_eq!(mode, SelectionMode::Object);
        assert!(!mode.enable_controls(None, <span class="s">false</span>));
    }

    <span class="c">/// A line's controls are its two ends.</span>
    #[test]
    <span class="k">fn</span> line_controls_use_original_endpoints() {
        <span class="k">let</span> geometry = Geometry::Line(session_rust::Line::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">6</span>.<span class="s">0</span>).into());
        <span class="k">let</span> controls = Controls::from_geometry(&amp;geometry);
        assert_eq!(controls.points.len(), <span class="s">2</span>);
        assert_eq!(controls.points[<span class="s">1</span>].position, [<span class="s">4</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">6</span>.<span class="s">0</span>]);
        assert_eq!(controls.points[<span class="s">1</span>].id, ControlId::Vertex(<span class="s">1</span>));
        assert_eq!(controls.links, [[<span class="s">0</span>, <span class="s">1</span>]]);
    }
}</code></pre></div>
<h2 id="step-2-srcappfetchrs">Step 2 · src/app/fetch.rs<a class="anchor" href="#/course/13-controls#step-2-srcappfetchrs" aria-label="Link to this section">#</a></h2>
<p>Fetch helpers retrieve source bytes and report the failing stage.</p>
<p><code>lessons/13/src/app/fetch.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! One HTTP request through the browser, returning status, body and ETag so an unchanged file can be skipped next time.</span>
<span class="k">use</span> wasm_bindgen::closure::Closure;
<span class="k">use</span> wasm_bindgen::{JsCast, JsValue};
<span class="k">use</span> wasm_bindgen_futures::JsFuture;
<span class="k">use</span> web_sys::{Headers, Request, RequestInit, RequestMode, Response};

<span class="c">/// The browser's message for a JS error value.</span>
<span class="k">fn</span> describe(e: JsValue) -&gt; String {
    <span class="k">match</span> e.as_string() {
        Some(message) =&gt; message,
        None =&gt; format!(&quot;{<span class="s">e:?</span>}&quot;),
    }
}

<span class="c">/// A JS error as a network error message.</span>
<span class="k">fn</span> network_error(error: JsValue) -&gt; String {
    format!(&quot;<span class="s">network error: </span>{}&quot;, describe(error))
}

<span class="c">/// What a GET came back with.</span>
<span class="k">pub</span> <span class="k">struct</span> Reply {
    <span class="k">pub</span> status: u16,
    <span class="k">pub</span> etag: Option&lt;String&gt;,
    <span class="k">pub</span> bytes: Vec&lt;u8&gt;, <span class="c">// the body, empty unless wanted</span>
}

<span class="c">/// Options for one GET.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> GetOpts {
    <span class="k">pub</span> no_store: bool, <span class="c">// skip the browser cache</span>
    <span class="k">pub</span> revalidate: bool, <span class="c">// ask the server if the cache is current</span>
    <span class="k">pub</span> if_none_match: Option&lt;String&gt;, <span class="c">// ETag for a conditional request</span>
    <span class="k">pub</span> range: Option&lt;(u64, u64)&gt;, <span class="c">// (start, length) of a byte range</span>
}</code></pre></div>
<p><code>lessons/13/src/app/fetch.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// GET \`url\`; any HTTP status is Ok, a network failure is Err.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> get(url: &amp;str, opts: &amp;GetOpts) -&gt; Result&lt;Reply, String&gt; {
    <span class="k">let</span> deadline = Deadline::new()?; <span class="c">// aborts after 90 s</span>
    <span class="k">let</span> init = RequestInit::new();
    init.set_signal(Some(&amp;deadline.controller.signal()));
    init.set_method(&quot;<span class="s">GET</span>&quot;);
    init.set_mode(RequestMode::Cors);

    <span class="k">if</span> opts.no_store {
        init.set_cache(web_sys::RequestCache::NoStore);
    } <span class="k">else</span> <span class="k">if</span> opts.revalidate {
        init.set_cache(web_sys::RequestCache::NoCache);
    }

    <span class="k">let</span> headers = Headers::new().map_err(describe)?;

    <span class="k">if</span> <span class="k">let</span> Some(tag) = &amp;opts.if_none_match {
        headers.set(&quot;<span class="s">If-None-Match</span>&quot;, tag).map_err(describe)?;
    }

    <span class="k">if</span> <span class="k">let</span> Some((start, len)) = opts.range {
        <span class="k">if</span> len == <span class="s">0</span> {
            <span class="k">return</span> Ok(Reply {
                status: <span class="s">206</span>,
                etag: None,
                bytes: Vec::new(),
            });
        }

        headers
            .set(
                &quot;<span class="s">Range</span>&quot;,
                &amp;format!(
                    &quot;<span class="s">bytes=</span>{}<span class="s">-</span>{}&quot;,
                    start,
                    start.checked_add(len - <span class="s">1</span>).ok_or(&quot;<span class="s">invalid byte range</span>&quot;)?
                ),
            )
            .map_err(describe)?;
    }

    init.set_headers(&amp;headers);
    <span class="k">let</span> request = Request::new_with_str_and_init(url, &amp;init).map_err(describe)?;
    <span class="k">let</span> window = web_sys::window().ok_or(&quot;<span class="s">no window</span>&quot;)?;
    <span class="k">let</span> resp: Response = JsFuture::from(window.fetch_with_request(&amp;request))
        .<span class="k">await</span>
        .map_err(network_error)?
        .dyn_into()
        .map_err(describe)?;
    <span class="k">let</span> etag = resp.headers().get(&quot;<span class="s">etag</span>&quot;).ok().flatten();
    <span class="k">let</span> status = resp.status();
    <span class="c">// read the body only when it is what was asked for</span>
    <span class="k">let</span> wanted = <span class="k">if</span> opts.range.is_some() {
        status == <span class="s">206</span>
    } <span class="k">else</span> {
        (<span class="s">200</span>..<span class="s">300</span>).contains(&amp;status)
    };

    <span class="k">if</span> !wanted {
        <span class="k">return</span> Ok(Reply {
            status,
            etag,
            bytes: Vec::new(),
        });
    }

    <span class="k">if</span> <span class="k">let</span> Ok(Some(length)) = resp.headers().get(&quot;<span class="s">Content-Length</span>&quot;)
        &amp;&amp; <span class="k">let</span> Ok(length) = length.parse::&lt;u64&gt;()
        &amp;&amp; length &gt; <span class="s">512</span> * <span class="s">1024</span> * <span class="s">1024</span>
    {
        <span class="k">return</span> Err(
            &quot;<span class="s">payload exceeds the 512 MiB whole-file limit; use cloud streaming</span>&quot;.to_string(),
        );
    }

    <span class="k">let</span> buf = JsFuture::from(resp.array_buffer().map_err(describe)?)
        .<span class="k">await</span>
        .map_err(describe)?;
    <span class="k">let</span> bytes = js_sys::Uint8Array::new(&amp;buf).to_vec();

    <span class="k">if</span> <span class="k">let</span> Some((_, length)) = opts.range
        &amp;&amp; bytes.len() <span class="k">as</span> u64 &gt; length
    {
        <span class="k">return</span> Err(&quot;<span class="s">range response exceeds requested bytes</span>&quot;.to_string());
    }

    Ok(Reply {
        status,
        etag,
        bytes,
    })
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/fetch.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A file's size from a HEAD request, if the server says.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> content_length(url: &amp;str) -&gt; Option&lt;u64&gt; {
    <span class="k">let</span> init = RequestInit::new();
    init.set_method(&quot;<span class="s">HEAD</span>&quot;);
    init.set_mode(RequestMode::Cors);
    <span class="k">let</span> request = Request::new_with_str_and_init(url, &amp;init).ok()?;
    <span class="k">let</span> window = web_sys::window()?;
    <span class="k">let</span> resp: Response = JsFuture::from(window.fetch_with_request(&amp;request))
        .<span class="k">await</span>
        .ok()?
        .dyn_into()
        .ok()?;
    resp.headers()
        .get(&quot;<span class="s">Content-Length</span>&quot;)
        .ok()
        .flatten()?
        .parse()
        .ok()
}

<span class="c">/// GET a whole file; a non-2xx status is an error.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> fetch_bytes(url: &amp;str) -&gt; Result&lt;Vec&lt;u8&gt;, String&gt; {
    <span class="k">let</span> r = get(
        url,
        &amp;GetOpts {
            revalidate: <span class="s">true</span>,
            ..GetOpts::default()
        },
    )
    .<span class="k">await</span>?;

    <span class="k">if</span> !(<span class="s">200</span>..<span class="s">300</span>).contains(&amp;r.status) {
        <span class="k">return</span> Err(format!(&quot;<span class="s">HTTP </span>{}<span class="s"> for </span>{<span class="s">url</span>}&quot;, r.status));
    }

    Ok(r.bytes)
}

<span class="c">/// GET a byte range; refuses a wrong length or a changed ETag.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> fetch_range(
    url: &amp;str,
    start: u64,
    len: u64,
    revision: &amp;Option&lt;String&gt;,
) -&gt; Result&lt;(Vec&lt;u8&gt;, Option&lt;String&gt;), String&gt; {
    <span class="k">let</span> reply = get(
        url,
        &amp;GetOpts {
            range: Some((start, len)),
            revalidate: <span class="s">true</span>,
            ..GetOpts::default()
        },
    )
    .<span class="k">await</span>?;

    <span class="k">if</span> reply.status != <span class="s">206</span> || reply.bytes.len() <span class="k">as</span> u64 != len {
        <span class="k">return</span> Err(format!(
            &quot;<span class="s">Range read failed for </span>{<span class="s">url</span>}<span class="s"> (HTTP </span>{}<span class="s">, </span>{}<span class="s"> of </span>{<span class="s">len</span>}<span class="s"> bytes)</span>&quot;,
            reply.status,
            reply.bytes.len()
        ));
    }

    <span class="k">if</span> revision.is_some() &amp;&amp; revision != &amp;reply.etag {
        <span class="k">return</span> Err(format!(&quot;{<span class="s">url</span>}<span class="s"> changed during the read; reload it</span>&quot;));
    }

    Ok((reply.bytes, reply.etag))
}

<span class="c">/// Call \`resolve\` after \`ms\` milliseconds.</span>
<span class="k">fn</span> schedule(resolve: js_sys::Function, ms: i32) {
    <span class="k">if</span> <span class="k">let</span> Some(w) = web_sys::window() {
        <span class="k">let</span> _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&amp;resolve, ms);
    }
}

<span class="c">/// Wait \`ms\` milliseconds.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> sleep_ms(ms: i32) {
    <span class="k">let</span> p = js_sys::Promise::new(&amp;<span class="k">mut</span> |resolve, _| schedule(resolve, ms));
    <span class="k">let</span> _ = JsFuture::from(p).<span class="k">await</span>;
}

<span class="c">/// Let the browser paint before continuing.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> next_tick() {
    sleep_ms(<span class="s">0</span>).<span class="k">await</span>;
}

<span class="c">/// A timer that aborts a fetch.</span>
<span class="k">struct</span> Deadline {
    controller: web_sys::AbortController,
    timer: i32, <span class="c">// the setTimeout handle</span>
    _callback: Closure&lt;<span class="k">dyn</span> FnMut()&gt;, <span class="c">// kept alive for the timer</span>
}

<span class="k">impl</span> Deadline {
    <span class="c">/// Abort after ninety seconds.</span>
    <span class="k">fn</span> new() -&gt; Result&lt;<span class="k">Self</span>, String&gt; {
        <span class="k">let</span> window = web_sys::window().ok_or(&quot;<span class="s">no window</span>&quot;)?;
        <span class="k">let</span> controller = web_sys::AbortController::new().map_err(describe)?;
        <span class="k">let</span> owned = controller.clone();
        <span class="k">let</span> callback = Closure::&lt;<span class="k">dyn</span> FnMut()&gt;::new(<span class="k">move</span> || abort_request(&amp;owned));
        <span class="k">let</span> timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                <span class="s">90_000</span>,
            )</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/fetch.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            .map_err(describe)?;
        Ok(<span class="k">Self</span> {
            controller,
            timer,
            _callback: callback,
        })
    }
}

<span class="k">impl</span> Drop <span class="k">for</span> Deadline {
    <span class="c">/// Clear the timer.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(<span class="k">self</span>.timer);
        }
    }
}

<span class="c">/// Abort the request.</span>
<span class="k">fn</span> abort_request(controller: &amp;web_sys::AbortController) {
    controller.abort();
}</code></pre></div>
<h2 id="step-3-srcappcloud_queryrs">Step 3 · src/app/cloud_query.rs<a class="anchor" href="#/course/13-controls#step-3-srcappcloud_queryrs" aria-label="Link to this section">#</a></h2>
<p>New file: a click on a streamed cloud reads only the pages under the cursor, including points not yet downloaded.</p>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! A click on a streamed cloud: read, page by page, only the points whose octree cube lies under the cursor.</span>
<span class="k">use</span> super::stream::{CloudFields, CloudLod};
<span class="k">use</span> session_rust::Xform;
<span class="k">use</span> std::cell::Cell;
<span class="k">use</span> std::ops::Range;
<span class="k">use</span> std::rc::Rc;

<span class="c">/// 65 536 points x 24 bytes = 1.5 MiB per range request.</span>
<span class="k">pub</span> <span class="k">const</span> PAGE_POINTS: u32 = <span class="s">65_536</span>;

<span class="c">/// Ids are stored as 4 little-endian bytes.</span>
<span class="k">pub</span> <span class="k">fn</span> original_id(raw: &amp;[u8]) -&gt; Result&lt;u32, String&gt; {
    <span class="k">let</span> Ok(bytes): Result&lt;[u8; 4], _&gt; = raw.try_into() <span class="k">else</span> { <span class="c">// any length but 4 fails here</span>
        <span class="k">return</span> Err(&quot;<span class="s">Original point ID range must contain exactly four bytes</span>&quot;.to_string());
    };
    Ok(u32::from_le_bytes(bytes))
}

<span class="c">/// The click and the camera it was made with.</span>
#[derive(Clone)]
<span class="k">pub</span> <span class="k">struct</span> QueryView {
    <span class="k">pub</span> matrix: Xform, <span class="c">// cloud space to clip space</span>
    <span class="k">pub</span> size: [f64; <span class="s">2</span>], <span class="c">// viewport in pixels</span>
    <span class="k">pub</span> at: [u32; <span class="s">2</span>], <span class="c">// click pixel</span>
    <span class="k">pub</span> radius: f64, <span class="c">// pick radius in pixels</span>
}

<span class="k">impl</span> QueryView {
    <span class="c">/// Clip space = x, y, z, w before the divide by w; x/w and y/w run -1..1 across the screen.</span>
    <span class="k">fn</span> clip(&amp;<span class="k">self</span>, point: [f64; <span class="s">3</span>]) -&gt; [f64; <span class="s">4</span>] {
        <span class="k">let</span> <span class="k">mut</span> clip = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>];

        <span class="k">for</span> (row, value) <span class="k">in</span> clip.iter_mut().enumerate() {
            <span class="c">// m is column-major: m[4 * column + row]</span>
            *value = <span class="k">self</span>.matrix.m[row] * point[<span class="s">0</span>]
                + <span class="k">self</span>.matrix.m[<span class="s">4</span> + row] * point[<span class="s">1</span>]
                + <span class="k">self</span>.matrix.m[<span class="s">8</span> + row] * point[<span class="s">2</span>]
                + <span class="k">self</span>.matrix.m[<span class="s">12</span> + row];
        }

        clip
    }

    <span class="c">/// A point as pixel x, y and depth; None when off screen.</span>
    <span class="k">pub</span> <span class="k">fn</span> project(&amp;<span class="k">self</span>, point: [f64; <span class="s">3</span>]) -&gt; Option&lt;[f64; 3]&gt; {
        <span class="k">let</span> p = <span class="k">self</span>.clip(point);

        <span class="c">// w &lt;= 0: behind the eye; z outside 0..w: beyond the far or near plane</span>
        <span class="k">if</span> invalid_clip(&amp;p) || p[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span> || p[<span class="s">2</span>] &lt; <span class="s">0</span>.<span class="s">0</span> || p[<span class="s">2</span>] &gt; p[<span class="s">3</span>] {
            <span class="k">return</span> None;
        }

        Some([
            (p[<span class="s">0</span>] / p[<span class="s">3</span>] + <span class="s">1</span>.<span class="s">0</span>) * <span class="k">self</span>.size[<span class="s">0</span>] * <span class="s">0</span>.<span class="s">5</span>,
            (<span class="s">1</span>.<span class="s">0</span> - p[<span class="s">1</span>] / p[<span class="s">3</span>]) * <span class="k">self</span>.size[<span class="s">1</span>] * <span class="s">0</span>.<span class="s">5</span>, <span class="c">// flipped: clip y points up, pixel y down</span>
            p[<span class="s">2</span>] / p[<span class="s">3</span>],
        ])
    }</code></pre></div>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Conservative: true when unsure, so a point is never missed, only read for nothing.</span>
    <span class="k">fn</span> intersects(&amp;<span class="k">self</span>, min: [f64; <span class="s">3</span>], size: f64) -&gt; bool {
        <span class="k">if</span> !size.is_finite() || size &lt; <span class="s">0</span>.<span class="s">0</span> || !min.into_iter().all(f64::is_finite) {
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> corners = [[<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>]; <span class="s">8</span>];

        <span class="k">for</span> (corner, clip) <span class="k">in</span> corners.iter_mut().enumerate() {
            *clip = <span class="k">self</span>.clip(cube_corner(min, size, corner));
        }

        <span class="k">if</span> corners.iter().any(invalid_clip) {
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="c">// all corners behind the eye: invisible; only some: the projection breaks, so say yes</span>
        <span class="k">if</span> corners.iter().all(behind_eye) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> corners.iter().any(behind_eye) {
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">if</span> corners.iter().all(beyond_far) || corners.iter().all(beyond_near) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="c">// the cube's box on screen against a square of +-radius around the click</span>
        <span class="k">let</span> <span class="k">mut</span> lo = [f64::INFINITY; <span class="s">2</span>];
        <span class="k">let</span> <span class="k">mut</span> hi = [f64::NEG_INFINITY; <span class="s">2</span>];

        <span class="k">for</span> p <span class="k">in</span> corners {
            <span class="k">let</span> xy = [
                (p[<span class="s">0</span>] / p[<span class="s">3</span>] + <span class="s">1</span>.<span class="s">0</span>) * <span class="k">self</span>.size[<span class="s">0</span>] * <span class="s">0</span>.<span class="s">5</span>,
                (<span class="s">1</span>.<span class="s">0</span> - p[<span class="s">1</span>] / p[<span class="s">3</span>]) * <span class="k">self</span>.size[<span class="s">1</span>] * <span class="s">0</span>.<span class="s">5</span>,
            ];
            grow_pixel_bounds(&amp;<span class="k">mut</span> lo, &amp;<span class="k">mut</span> hi, xy);
        }

        <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
            <span class="k">let</span> center = <span class="k">self</span>.at[axis] <span class="k">as</span> f64 + <span class="s">0</span>.<span class="s">5</span>; <span class="c">// the middle of the clicked pixel</span>

            <span class="k">if</span> lo[axis] &gt; center + <span class="k">self</span>.radius || hi[axis] &lt; center - <span class="k">self</span>.radius {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }

        <span class="s">true</span>
    }
}</code></pre></div>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> invalid_clip(point: &amp;[f64; <span class="s">4</span>]) -&gt; bool {
    !point.iter().copied().all(f64::is_finite)
}

<span class="k">fn</span> behind_eye(point: &amp;[f64; <span class="s">4</span>]) -&gt; bool {
    point[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span>
}

<span class="c">/// Reverse-Z: the far plane is z = 0 and the near plane z = w.</span>
<span class="k">fn</span> beyond_far(point: &amp;[f64; <span class="s">4</span>]) -&gt; bool {
    point[<span class="s">2</span>] &lt; <span class="s">0</span>.<span class="s">0</span>
}

<span class="k">fn</span> beyond_near(point: &amp;[f64; <span class="s">4</span>]) -&gt; bool {
    point[<span class="s">2</span>] &gt; point[<span class="s">3</span>]
}

<span class="c">/// Bits 0, 1, 2 of \`corner\` add \`size\` on x, y, z: 0..8 gives all eight.</span>
<span class="k">fn</span> cube_corner(min: [f64; <span class="s">3</span>], size: f64, corner: usize) -&gt; [f64; <span class="s">3</span>] {
    [
        min[<span class="s">0</span>] + <span class="k">if</span> corner &amp; <span class="s">1</span> == <span class="s">0</span> { <span class="s">0</span>.<span class="s">0</span> } <span class="k">else</span> { size },
        min[<span class="s">1</span>] + <span class="k">if</span> corner &amp; <span class="s">2</span> == <span class="s">0</span> { <span class="s">0</span>.<span class="s">0</span> } <span class="k">else</span> { size },
        min[<span class="s">2</span>] + <span class="k">if</span> corner &amp; <span class="s">4</span> == <span class="s">0</span> { <span class="s">0</span>.<span class="s">0</span> } <span class="k">else</span> { size },
    ]
}

<span class="c">/// Grow a pixel box to include a point.</span>
<span class="k">fn</span> grow_pixel_bounds(lo: &amp;<span class="k">mut</span> [f64; <span class="s">2</span>], hi: &amp;<span class="k">mut</span> [f64; <span class="s">2</span>], point: [f64; <span class="s">2</span>]) {
    <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        lo[axis] = lo[axis].min(point[axis]);
        hi[axis] = hi[axis].max(point[axis]);
    }
}</code></pre></div>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One point near the click.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Candidate {
    <span class="k">pub</span> local: u32, <span class="c">// index in the source file</span>
    <span class="k">pub</span> position: [f64; <span class="s">3</span>], <span class="c">// exact source position</span>
}

<span class="k">fn</span> source_range_order(left: &amp;Range&lt;u32&gt;, right: &amp;Range&lt;u32&gt;) -&gt; std::cmp::Ordering {
    left.start.cmp(&amp;right.start)
}

<span class="c">/// Sorted, then joined where they touch: [0..5, 3..9, 12..14] -&gt; [0..9, 12..14].</span>
<span class="k">fn</span> merge(<span class="k">mut</span> ranges: Vec&lt;Range&lt;u32&gt;&gt;) -&gt; Vec&lt;Range&lt;u32&gt;&gt; {
    ranges.sort_unstable_by(source_range_order);
    <span class="k">let</span> <span class="k">mut</span> out: Vec&lt;Range&lt;u32&gt;&gt; = Vec::new();

    <span class="k">for</span> range <span class="k">in</span> ranges {
        <span class="k">if</span> range.is_empty() {
            <span class="k">continue</span>;
        }

        <span class="k">if</span> <span class="k">let</span> Some(last) = out.last_mut()
            &amp;&amp; range.start &lt;= last.end
        {
            last.end = last.end.max(range.end);
        } <span class="k">else</span> {
            out.push(range);
        }
    }

    out
}

<span class="c">/// The fallback: scan the whole cloud as one range.</span>
<span class="k">fn</span> all_rows(total: u32) -&gt; Vec&lt;Range&lt;u32&gt;&gt; {
    std::iter::once(<span class="s">0</span>..total).collect()
}

<span class="c">/// The point ranges of every octree node near the click.</span>
<span class="k">pub</span> <span class="k">fn</span> eligible_ranges(lod: &amp;CloudLod, total: u32, view: &amp;QueryView) -&gt; Vec&lt;Range&lt;u32&gt;&gt; {
    <span class="k">let</span> n = lod.len();

    <span class="c">// a broken node table: scan everything rather than miss a point</span>
    <span class="k">if</span> n == <span class="s">0</span> || lod.first.len() &lt; n || lod.count.len() &lt; n || lod.min.len() &lt; n * <span class="s">3</span> {
        <span class="k">return</span> all_rows(total);
    }

    <span class="k">let</span> <span class="k">mut</span> coverage = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> eligible = Vec::new();

    <span class="k">for</span> node <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> (Ok(first), Ok(count)) = (
            u32::try_from(lod.first[node]),
            u32::try_from(lod.count[node]),
        ) <span class="k">else</span> {
            <span class="k">return</span> all_rows(total);
        };
        <span class="k">let</span> Some(end) = first.checked_add(count) <span class="k">else</span> {
            <span class="k">return</span> all_rows(total);
        };

        <span class="k">if</span> end &gt; total {
            <span class="k">return</span> all_rows(total);
        }

        coverage.push(first..end);
        <span class="k">let</span> at = node * <span class="s">3</span>;
        <span class="k">let</span> min = [lod.min[at], lod.min[at + <span class="s">1</span>], lod.min[at + <span class="s">2</span>]];

        <span class="k">if</span> view.intersects(min, lod.size[node]) {
            eligible.push(first..end);
        }
    }

    <span class="c">// points in no node would never be read: scan everything instead</span>
    <span class="k">if</span> merge(coverage) != all_rows(total) {
        <span class="k">return</span> all_rows(total);
    }

    merge(eligible)
}</code></pre></div>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One pick in a streamed cloud, page by page.</span>
<span class="k">pub</span> <span class="k">struct</span> Query {
    <span class="k">pub</span> id: u64, <span class="c">// pick number</span>
    <span class="k">pub</span> parent: u32, <span class="c">// the cloud's object row</span>
    <span class="k">pub</span> url: String, <span class="c">// the cloud file</span>
    <span class="k">pub</span> fields: CloudFields, <span class="c">// where the arrays are in the file</span>
    <span class="k">pub</span> view: QueryView, <span class="c">// the click</span>
    <span class="k">pub</span> cancelled: Rc&lt;Cell&lt;bool&gt;&gt;, <span class="c">// one flag shared with every read in flight; set when a newer pick replaces this</span>
    <span class="k">pub</span> revision: Option&lt;String&gt;, <span class="c">// ETag = the server's version tag; a changed file fails the read instead of mixing versions</span>
    <span class="k">pub</span> candidates: Vec&lt;Candidate&gt;, <span class="c">// points near the click so far</span>
    <span class="k">pub</span> best: Option&lt;u32&gt;, <span class="c">// winning candidate index so far</span>
    <span class="k">pub</span> checked: u32, <span class="c">// points examined so far</span>
    <span class="k">pub</span> total: u32, <span class="c">// points to examine</span>
    ranges: Vec&lt;Range&lt;u32&gt;&gt;, <span class="c">// pages still to read</span>
    next: usize,
    <span class="k">pub</span> awaiting_gpu: bool, <span class="c">// a page is on the GPU for ranking</span>
}

<span class="k">impl</span> Query {
    <span class="c">/// A new pick over the cloud's eligible ranges.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(id: u64, cloud: &amp;super::scene::StreamedCloud, view: QueryView) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> ranges = eligible_ranges(&amp;cloud.lod, cloud.total, &amp;view);
        <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>;

        <span class="k">for</span> range <span class="k">in</span> &amp;ranges {
            total += range.end - range.start;
        }

        <span class="k">Self</span> {
            id,
            parent: cloud.row,
            url: cloud.url.clone(),
            fields: cloud.fields.clone(),
            view,
            cancelled: Rc::new(Cell::new(<span class="s">false</span>)),
            revision: cloud.fields.revision.clone(),
            candidates: Vec::new(),
            best: None,
            checked: <span class="s">0</span>,
            total,
            ranges,
            next: <span class="s">0</span>,
            awaiting_gpu: <span class="s">false</span>,
        }
    }

    <span class="c">/// The next page to read, None when done.</span>
    <span class="k">pub</span> <span class="k">fn</span> next_page(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;Range&lt;u32&gt;&gt; {
        <span class="k">if</span> <span class="k">self</span>.cancelled.get() {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> range = <span class="k">self</span>.ranges.get_mut(<span class="k">self</span>.next)?;
        <span class="k">let</span> page = range.start..range.end.min(range.start.saturating_add(PAGE_POINTS)); <span class="c">// at most one page off the front</span>
        range.start = page.end;

        <span class="k">if</span> range.start &gt;= range.end {
            <span class="k">self</span>.next += <span class="s">1</span>;
        }

        Some(page)
    }
}</code></pre></div>
<p><code>lessons/13/src/app/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Drop <span class="k">for</span> Query {
    <span class="c">/// Drop runs when the Query is freed, so a replaced pick stops its own reads.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.cancelled.set(<span class="s">true</span>);
    }
}

<span class="c">/// One page's answer.</span>
<span class="k">pub</span> <span class="k">struct</span> Batch {
    <span class="k">pub</span> query: u64, <span class="c">// which pick</span>
    <span class="k">pub</span> count: u32, <span class="c">// points examined</span>
    <span class="k">pub</span> result: Result&lt;(Vec&lt;Candidate&gt;, Option&lt;String&gt;), String&gt;, <span class="c">// the candidates and the ETag the read saw</span>
}

<span class="c">/// The final answer of a pick.</span>
<span class="k">pub</span> <span class="k">struct</span> Resolved {
    <span class="k">pub</span> query: u64, <span class="c">// which pick</span>
    <span class="k">pub</span> result: Result&lt;(u32, [f64; 3]), String&gt;, <span class="c">// original id and position</span>
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/cloud_query.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// browser only: the reads go through fetch</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">mod</span> web {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::loader::post;

    <span class="k">use</span> <span class="k">crate</span>::app::fetch::fetch_range <span class="k">as</span> range;

    <span class="c">/// An owned copy for the async read, which may outlive the Query that started it.</span>
    <span class="k">struct</span> SourceRequest {
        query: u64,
        url: String,
        fields: CloudFields,
        cancelled: Rc&lt;Cell&lt;bool&gt;&gt;,
        revision: Option&lt;String&gt;,
    }

    <span class="k">fn</span> source_request(query: &amp;Query) -&gt; SourceRequest {
        SourceRequest {
            query: query.id,
            url: query.url.clone(),
            fields: query.fields.clone(),
            cancelled: query.cancelled.clone(),
            revision: query.revision.clone(),
        }
    }

    <span class="c">/// Start reading one page; the answer arrives as a message.</span>
    <span class="k">pub</span> <span class="k">fn</span> fetch_page(query: &amp;Query, page: Range&lt;u32&gt;) {
        <span class="c">// spawn_local runs the future on the browser's event loop; this call returns at once</span>
        wasm_bindgen_futures::spawn_local(post_page(
            source_request(query),
            query.view.clone(),
            page,
        ));
    }

    <span class="c">/// Read the page and post the answer unless cancelled.</span>
    <span class="k">async</span> <span class="k">fn</span> post_page(source: SourceRequest, view: QueryView, page: Range&lt;u32&gt;) {
        <span class="k">let</span> count = page.end - page.start;
        <span class="k">let</span> result = read_page(&amp;source, &amp;view, page).<span class="k">await</span>;

        <span class="k">if</span> !source.cancelled.get() {
            post(<span class="k">crate</span>::Msg::CloudQueryBatch(Batch {
                query: source.query,
                count,
                result,
            }));
        }
    }

    <span class="c">/// Read one page and keep the points near the click.</span>
    <span class="k">async</span> <span class="k">fn</span> read_page(
        source: &amp;SourceRequest,
        view: &amp;QueryView,
        page: Range&lt;u32&gt;,
    ) -&gt; Result&lt;(Vec&lt;Candidate&gt;, Option&lt;String&gt;), String&gt; {
        <span class="k">let</span> at = source.fields.coords_at + u64::from(page.start) * <span class="s">24</span>; <span class="c">// 3 f64 per point = 24 bytes</span>
        <span class="k">let</span> length = u64::from(page.end - page.start) * <span class="s">24</span>;
        <span class="k">let</span> (raw, revision) = range(&amp;source.url, at, length, &amp;source.revision).<span class="k">await</span>?;

        <span class="k">if</span> source.cancelled.get() {
            <span class="k">return</span> Err(&quot;<span class="s">Point query cancelled</span>&quot;.to_string());
        }

        Ok((page_candidates(&amp;raw, page.start, view)?, revision))
    }

    <span class="c">/// A point from its 24 stored bytes.</span>
    <span class="k">fn</span> source_position(raw: &amp;[u8]) -&gt; Result&lt;[f64; 3], String&gt; {
        <span class="k">if</span> raw.len() != <span class="s">24</span> {
            <span class="k">return</span> Err(&quot;<span class="s">Source coordinate range must contain exactly 24 bytes</span>&quot;.to_string());
        }

        <span class="k">let</span> <span class="k">mut</span> position = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

        <span class="k">for</span> (axis, value) <span class="k">in</span> position.iter_mut().enumerate() {
            <span class="k">let</span> bytes: [u8; <span class="s">8</span>] = raw[axis * <span class="s">8</span>..axis * <span class="s">8</span> + <span class="s">8</span>]
                .try_into()
                .expect(&quot;<span class="s">exact triple checked above</span>&quot;);
            *value = f64::from_le_bytes(bytes);

            <span class="c">// it must also fit f32, the GPU's precision</span>
            <span class="k">if</span> !value.is_finite() || !(*value <span class="k">as</span> f32).is_finite() {
                <span class="k">return</span> Err(
                    &quot;<span class="s">Source coordinates are nonfinite or outside the GPU coordinate range</span>&quot;
                        .to_string(),
                );
            }
        }

        Ok(position)
    }

    <span class="c">/// Points of one cloud chunk whose screen position is near the click.</span>
    <span class="k">fn</span> page_candidates(raw: &amp;[u8], first: u32, view: &amp;QueryView) -&gt; Result&lt;Vec&lt;Candidate&gt;, String&gt; {
        <span class="k">let</span> <span class="k">mut</span> candidates = Vec::new();

        <span class="k">for</span> (offset, xyz) <span class="k">in</span> raw.chunks_exact(<span class="s">24</span>).enumerate() {
            <span class="k">let</span> position = source_position(xyz)?;

            <span class="k">if</span> <span class="k">let</span> Some(point) = view.project(position) {
                <span class="k">let</span> distance = (point[<span class="s">0</span>] - view.at[<span class="s">0</span>] <span class="k">as</span> f64 - <span class="s">0</span>.<span class="s">5</span>).powi(<span class="s">2</span>)
                    + (point[<span class="s">1</span>] - view.at[<span class="s">1</span>] <span class="k">as</span> f64 - <span class="s">0</span>.<span class="s">5</span>).powi(<span class="s">2</span>);

                <span class="k">if</span> distance &lt;= view.radius.powi(<span class="s">2</span>) {
                    candidates.push(Candidate {
                        local: first + offset <span class="k">as</span> u32,
                        position,</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/cloud_query.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    });
                }
            }
        }

        Ok(candidates)
    }

    <span class="c">/// Read the winner's id and position.</span>
    <span class="k">pub</span> <span class="k">fn</span> resolve_id(query: &amp;Query, local: u32) {
        wasm_bindgen_futures::spawn_local(post_source(source_request(query), local));
    }

    <span class="c">/// Read and post the final answer unless cancelled.</span>
    <span class="k">async</span> <span class="k">fn</span> post_source(source: SourceRequest, local: u32) {
        <span class="k">let</span> result = read_source(&amp;source, local).<span class="k">await</span>;

        <span class="k">if</span> !source.cancelled.get() {
            post(<span class="k">crate</span>::Msg::CloudQueryResolved(Resolved {
                query: source.query,
                result,
            }));
        }
    }

    <span class="c">/// Read one point's position and id.</span>
    <span class="k">async</span> <span class="k">fn</span> read_source(source: &amp;SourceRequest, local: u32) -&gt; Result&lt;(u32, [f64; 3]), String&gt; {
        <span class="k">let</span> fields = &amp;source.fields;
        <span class="k">let</span> (coords, _) = range(
            &amp;source.url,
            fields.coords_at + u64::from(local) * <span class="s">24</span>,
            <span class="s">24</span>,
            &amp;source.revision,
        )
        .<span class="k">await</span>?;

        <span class="k">if</span> source.cancelled.get() {
            <span class="k">return</span> Err(&quot;<span class="s">Point query cancelled</span>&quot;.to_string());
        }

        <span class="k">let</span> position = source_position(&amp;coords)?;
        <span class="k">let</span> original = <span class="k">if</span> fields.ids_len == <span class="s">0</span> { <span class="c">// no id array: the index is the id</span>
            local
        } <span class="k">else</span> {
            <span class="k">if</span> fields.ids_len != u64::from(fields.count) * <span class="s">4</span> {
                <span class="k">return</span> Err(&quot;<span class="s">Original point ID count differs from source coordinates</span>&quot;.to_string());
            }

            <span class="k">let</span> (raw, _) = range(
                &amp;source.url,
                fields.ids_at + u64::from(local) * <span class="s">4</span>,
                <span class="s">4</span>,
                &amp;source.revision,
            )
            .<span class="k">await</span>?;
            original_id(&amp;raw)?
        };
        Ok((original, position))
    }
}
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">use</span> web::{fetch_page, resolve_id};

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A 100x100 view clicked at its centre.</span>
    <span class="k">fn</span> view() -&gt; QueryView {
        QueryView {
            matrix: Xform::identity(),
            size: [<span class="s">100</span>.<span class="s">0</span>; <span class="s">2</span>],
            at: [<span class="s">50</span>; <span class="s">2</span>],
            radius: <span class="s">6</span>.<span class="s">0</span>,
        }
    }

    <span class="c">/// An id keeps all 32 bits; a short read is refused.</span>
    #[test]
    <span class="k">fn</span> original_fixed32_ids_preserve_all_bits_and_reject_partial_ranges() {
        <span class="k">for</span> value <span class="k">in</span> [<span class="s">0</span>, <span class="s">127</span>, <span class="s">128</span>, <span class="s">65535</span>, u32::MAX, <span class="s">42</span>] {
            assert_eq!(original_id(&amp;value.to_le_bytes()).unwrap(), value);
        }

        assert!(original_id(&amp;[<span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>]).is_err());
        assert!(original_id(&amp;[<span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>, <span class="s">4</span>, <span class="s">5</span>]).is_err());
    }

    <span class="c">/// Nodes past six million points are still read; points in no node are scanned.</span>
    #[test]
    <span class="k">fn</span> nodes_after_six_million_remain_eligible_and_uncovered_rows_are_scanned() {
        <span class="k">let</span> lod = CloudLod {
            min: vec![<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">0</span>.<span class="s">01</span>, -<span class="s">0</span>.<span class="s">01</span>, <span class="s">0</span>.<span class="s">4</span>],
            size: vec![<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">02</span>],
            first: vec![<span class="s">0</span>, <span class="s">6_000_000</span>],
            count: vec![<span class="s">6_000_000</span>, <span class="s">100</span>],
            ..Default::default()
        };
        assert_eq!(
            eligible_ranges(&amp;lod, <span class="s">6_000_100</span>, &amp;view()),
            std::iter::once(<span class="s">6_000_000</span>..<span class="s">6_000_100</span>).collect::&lt;Vec&lt;_&gt;&gt;()
        );
        assert_eq!(
            eligible_ranges(&amp;lod, <span class="s">6_000_101</span>, &amp;view()),
            std::iter::once(<span class="s">0</span>..<span class="s">6_000_101</span>).collect::&lt;Vec&lt;_&gt;&gt;()
        );
    }

    <span class="c">/// Cubes and points project to the same pixel centre.</span>
    #[test]
    <span class="k">fn</span> cube_eligibility_uses_the_same_pixel_center_as_candidate_projection() {
        <span class="k">let</span> v = view();
        <span class="c">// 56.4 is within 6 of the pixel centre 50.5</span>
        assert!(v.intersects([<span class="s">0</span>.<span class="s">128</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">5</span>], <span class="s">0</span>.<span class="s">0</span>));
        assert!(!v.intersects([<span class="s">0</span>.<span class="s">132</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">5</span>], <span class="s">0</span>.<span class="s">0</span>));
    }

    <span class="c">/// A cube crossing the near plane stays eligible.</span>
    #[test]
    <span class="k">fn</span> near_plane_crossing_cube_is_conservative() {
        <span class="k">let</span> <span class="k">mut</span> v = view();
        v.matrix.m[<span class="s">3</span>] = <span class="s">1</span>.<span class="s">0</span>;
        v.matrix.m[<span class="s">15</span>] = <span class="s">0</span>.<span class="s">0</span>;
        assert!(v.intersects([-<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], <span class="s">2</span>.<span class="s">0</span>));
    }

    <span class="c">/// Pages cover every row; a cancel stops them.</span>
    #[test]
    <span class="k">fn</span> bounded_pages_visit_the_entire_range_and_cancellation_stops_advancement() {
        <span class="k">let</span> <span class="k">mut</span> query = Query {
            id: <span class="s">1</span>,
            parent: <span class="s">0</span>,
            url: String::new(),
            fields: CloudFields {
                end: <span class="s">0</span>,
                coords_at: <span class="s">0</span>,
                coords_len: <span class="s">0</span>,
                colors_at: <span class="s">0</span>,
                colors_len: <span class="s">0</span>,
                count: <span class="s">1</span>,
                ids_at: <span class="s">0</span>,
                ids_len: <span class="s">0</span>,
                revision: None,
            },
            view: view(),
            cancelled: Rc::new(Cell::new(<span class="s">false</span>)),
            revision: None,
            candidates: Vec::new(),
            best: None,
            checked: <span class="s">0</span>,
            total: PAGE_POINTS * <span class="s">2</span> + <span class="s">7</span>,
            ranges: std::iter::once(<span class="s">6_000_000</span>..<span class="s">6_000_000</span> + PAGE_POINTS * <span class="s">2</span> + <span class="s">7</span>).collect(),
            next: <span class="s">0</span>,
            awaiting_gpu: <span class="s">false</span>,
        };
        assert_eq!(query.next_page(), Some(<span class="s">6_000_000</span>..<span class="s">6_000_000</span> + PAGE_POINTS));
        assert_eq!(
            query.next_page(),
            Some(<span class="s">6_000_000</span> + PAGE_POINTS..<span class="s">6_000_000</span> + PAGE_POINTS * <span class="s">2</span>)
        );
        assert_eq!(
            query.next_page(),
            Some(<span class="s">6_000_000</span> + PAGE_POINTS * <span class="s">2</span>..<span class="s">6_000_000</span> + PAGE_POINTS * <span class="s">2</span> + <span class="s">7</span>)
        );
        assert_eq!(query.next_page(), None);
        query.next = <span class="s">0</span>;
        query.ranges = std::iter::once(<span class="s">0</span>..<span class="s">100</span>).collect();
        query.cancelled.set(<span class="s">true</span>);
        assert_eq!(query.next_page(), None);
        assert_eq!(query.ranges[<span class="s">0</span>], <span class="s">0</span>..<span class="s">100</span>);
    }

    <span class="c">/// Dropping a pick sets its cancel flag.</span>
    #[test]
    <span class="k">fn</span> cancellation_token_invalidates_inflight_work_on_drop() {
        <span class="k">let</span> token = Rc::new(Cell::new(<span class="s">false</span>));
        <span class="k">let</span> query = Query {
            id: <span class="s">1</span>,
            parent: <span class="s">0</span>,
            url: String::new(),
            fields: CloudFields {
                end: <span class="s">0</span>,
                coords_at: <span class="s">0</span>,
                coords_len: <span class="s">0</span>,
                colors_at: <span class="s">0</span>,
                colors_len: <span class="s">0</span>,
                count: <span class="s">1</span>,
                ids_at: <span class="s">0</span>,
                ids_len: <span class="s">0</span>,
                revision: None,
            },
            view: view(),
            cancelled: token.clone(),
            revision: None,
            candidates: Vec::new(),
            best: None,
            checked: <span class="s">0</span>,
            total: <span class="s">1</span>,
            ranges: std::iter::once(<span class="s">0</span>..<span class="s">1</span>).collect(),
            next: <span class="s">0</span>,
            awaiting_gpu: <span class="s">false</span>,
        };
        drop(query);
        assert!(token.get());
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/13/</code>.</p>
<h2 id="step-4-srcappstreamrs">Step 4 · src/app/stream.rs<a class="anchor" href="#/course/13-controls#step-4-srcappstreamrs" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/stream.rs</code> · edit · copy the file</p>
<p>Replaces <code>fn is_empty</code> in <code>lessons/12/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Number of nodes.</span>
    <span class="k">pub</span> <span class="k">fn</span> len(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.size.len()
    }

    <span class="c">/// Fill one array from a packed field; false when unknown.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_field(&amp;<span class="k">mut</span> <span class="k">self</span>, field: usize, raw: &amp;[u8]) -&gt; bool {
        <span class="k">if</span> (<span class="s">8</span>..=<span class="s">10</span>).contains(&amp;field) &amp;&amp; !raw.len().is_multiple_of(<span class="s">8</span>) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> (<span class="s">11</span>..=<span class="s">14</span>).contains(&amp;field) {
            <span class="k">let</span> <span class="k">mut</span> at = <span class="s">0</span>;

            <span class="k">while</span> at &lt; raw.len() {
                <span class="k">let</span> Some((_, bytes)) = varint(raw, at) <span class="k">else</span> {
                    <span class="k">return</span> <span class="s">false</span>;
                };
                at += bytes;
            }
        }

        <span class="k">match</span> field {
            <span class="s">8</span> =&gt; <span class="k">self</span>.min = packed_f64(raw),
            <span class="s">9</span> =&gt; <span class="k">self</span>.size = packed_f64(raw),
            <span class="s">10</span> =&gt; <span class="k">self</span>.spacing = packed_f64(raw),
            <span class="s">11</span> =&gt; <span class="k">self</span>.level = packed_i32(raw),
            <span class="s">12</span> =&gt; <span class="k">self</span>.first = packed_i32(raw),
            <span class="s">13</span> =&gt; <span class="k">self</span>.count = packed_i32(raw),
            <span class="s">14</span> =&gt; <span class="k">self</span>.children = packed_i32(raw),
            _ =&gt; <span class="k">return</span> <span class="s">false</span>,
        }

        <span class="s">true</span>
    }

    <span class="c">/// True when every node and child index is sound.</span>
    <span class="k">pub</span> <span class="k">fn</span> valid(&amp;<span class="k">self</span>, total: u32) -&gt; bool {
        <span class="k">let</span> n = <span class="k">self</span>.len();
        <span class="k">let</span> Some(triples) = n.checked_mul(<span class="s">3</span>) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(children) = n.checked_mul(<span class="s">8</span>) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> n == <span class="s">0</span>
            || <span class="k">self</span>.min.len() != triples
            || <span class="k">self</span>.spacing.len() != n
            || <span class="k">self</span>.level.len() != n
            || <span class="k">self</span>.first.len() != n
            || <span class="k">self</span>.count.len() != n
            || <span class="k">self</span>.children.len() != children
            || <span class="k">self</span>.level[<span class="s">0</span>] != <span class="s">0</span>
        {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> parents = vec![<span class="s">false</span>; n];

        <span class="k">for</span> node <span class="k">in</span> <span class="s">0</span>..n {
            <span class="k">if</span> !<span class="k">self</span>.valid_node(node, total) || !<span class="k">self</span>.valid_children(node, &amp;<span class="k">mut</span> parents) {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }

        !parents[<span class="s">1</span>..].contains(&amp;<span class="s">false</span>)
    }

    <span class="c">/// True when one node's values are sound.</span>
    <span class="k">fn</span> valid_node(&amp;<span class="k">self</span>, node: usize, total: u32) -&gt; bool {
        <span class="k">let</span> size = <span class="k">self</span>.size[node];
        <span class="k">let</span> spacing = <span class="k">self</span>.spacing[node];

        <span class="k">if</span> !finite_float(size)
            || size &lt; <span class="s">0</span>.<span class="s">0</span>
            || !finite_float(spacing)
            || spacing &lt; <span class="s">0</span>.<span class="s">0</span>
            || <span class="k">self</span>.level[node] &lt; <span class="s">0</span>
        {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">for</span> &amp;min <span class="k">in</span> &amp;<span class="k">self</span>.min[node * <span class="s">3</span>..node * <span class="s">3</span> + <span class="s">3</span>] {
            <span class="k">if</span> !finite_float(min) || !finite_float(min + size) {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }

        <span class="k">let</span> (Ok(first), Ok(count)) = (
            u32::try_from(<span class="k">self</span>.first[node]),
            u32::try_from(<span class="k">self</span>.count[node]),
        ) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(end) = first.checked_add(count) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        end &lt;= total
    }

    <span class="c">/// True when a node's children are one level down and unclaimed.</span>
    <span class="k">fn</span> valid_children(&amp;<span class="k">self</span>, node: usize, parents: &amp;<span class="k">mut</span> [bool]) -&gt; bool {
        <span class="k">for</span> &amp;child <span class="k">in</span> &amp;<span class="k">self</span>.children[node * <span class="s">8</span>..node * <span class="s">8</span> + <span class="s">8</span>] {
            <span class="k">if</span> child == -<span class="s">1</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> Ok(child) = usize::try_from(child) <span class="k">else</span> {
                <span class="k">return</span> <span class="s">false</span>;
            };

            <span class="k">if</span> child == <span class="s">0</span>
                || child &gt;= parents.len()
                || parents[child]
                || <span class="k">self</span>.level[node].checked_add(<span class="s">1</span>) != Some(<span class="k">self</span>.level[child])
            {
                <span class="k">return</span> <span class="s">false</span>;
            }

            parents[child] = <span class="s">true</span>;
        }

        <span class="s">true</span>
    }

    <span class="c">/// True when the file carried no octree.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_empty(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.size.is_empty()
    }
}

<span class="c">/// The varint at \`i\` and its byte length.</span>
<span class="k">pub</span> <span class="k">fn</span> varint(b: &amp;[u8], <span class="k">mut</span> i: usize) -&gt; Option&lt;(u64, usize)&gt; {
    <span class="k">let</span> (<span class="k">mut</span> v, <span class="k">mut</span> shift) = (<span class="s">0u64</span>, <span class="s">0u32</span>);
    <span class="k">let</span> start = i;

    <span class="k">loop</span> {
        <span class="k">let</span> byte = *b.get(i)?;

        <span class="k">if</span> shift == <span class="s">63</span> &amp;&amp; byte &gt; <span class="s">1</span> {
            <span class="k">return</span> None;
        }

        v |= ((byte &amp; <span class="s">0x7f</span>) <span class="k">as</span> u64) &lt;&lt; shift;
        i += <span class="s">1</span>;

        <span class="k">if</span> byte &amp; <span class="s">0x80</span> == <span class="s">0</span> {
            <span class="k">return</span> Some((v, i - start));
        }

        shift += <span class="s">7</span>;

        <span class="k">if</span> shift &gt; <span class="s">63</span> {
            <span class="k">return</span> None;
        }
    }
}

<span class="c">/// Byte length of a scalar field of wire type \`wire\`.</span>
<span class="k">fn</span> skip_scalar(b: &amp;[u8], i: usize, wire: u32) -&gt; Option&lt;usize&gt; {
    <span class="k">match</span> wire {
        <span class="s">0</span> =&gt; Some(varint(b, i)?.<span class="s">1</span>),
        <span class="s">1</span> =&gt; Some(<span class="s">8</span>),
        <span class="s">5</span> =&gt; Some(<span class="s">4</span>),
        _ =&gt; None,
    }
}

<span class="c">/// A skipped field longer than this is geometry, not a name.</span>
<span class="k">const</span> NAME_BYTES: u64 = <span class="s">256</span>;

<span class="c">/// Start and length of a single cloud's \`coords\`; None otherwise.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_to_coords(head: &amp;[u8]) -&gt; Option&lt;(u64, u64)&gt; {
    <span class="k">let</span> (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

<span class="c">/// Start, length of \`coords\` and end of the cloud message.</span>
<span class="k">fn</span> cloud_layout(head: &amp;[u8]) -&gt; Option&lt;(u64, u64, u64)&gt; {
    <span class="k">let</span> <span class="k">mut</span> at = <span class="s">0usize</span>;
    <span class="k">let</span> objects_end = descend_message(head, &amp;<span class="k">mut</span> at, None, <span class="s">3</span>)?;
    <span class="k">let</span> end = descend_message(head, &amp;<span class="k">mut</span> at, Some(objects_end), <span class="s">8</span>)?;

    <span class="k">while</span> (at <span class="k">as</span> u64) &lt; end {
        <span class="k">let</span> (tag, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        <span class="k">let</span> (field, wire) = (u32::try_from(tag &gt;&gt; <span class="s">3</span>).ok()?, (tag &amp; <span class="s">7</span>) <span class="k">as</span> u32);

        <span class="k">if</span> field == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">if</span> wire != <span class="s">2</span> {
            at = at.checked_add(skip_scalar(head, at, wire)?)?;

            <span class="k">if</span> at <span class="k">as</span> u64 &gt; end {
                <span class="k">return</span> None;
            }

            <span class="k">continue</span>;
        }

        <span class="k">let</span> (length, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        <span class="k">let</span> next = (at <span class="k">as</span> u64).checked_add(length)?;

        <span class="k">if</span> next &gt; end {
            <span class="k">return</span> None;
        }

        <span class="k">if</span> field == <span class="s">3</span> {
            <span class="k">return</span> Some((at <span class="k">as</span> u64, length, end));
        }

        <span class="k">if</span> field == <span class="s">4</span> || length &gt; NAME_BYTES {
            <span class="k">return</span> None;
        }

        at = usize::try_from(next).ok()?;
    }

    None
}

<span class="c">/// Enter field \`want\` of the message at \`at\`; returns its end.</span>
<span class="k">fn</span> descend_message(head: &amp;[u8], at: &amp;<span class="k">mut</span> usize, parent_end: Option&lt;u64&gt;, want: u32) -&gt; Option&lt;u64&gt; {
    <span class="k">loop</span> {
        <span class="k">let</span> (tag, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        <span class="k">let</span> (field, wire) = (u32::try_from(tag &gt;&gt; <span class="s">3</span>).ok()?, tag &amp; <span class="s">7</span>);

        <span class="k">if</span> field == <span class="s">0</span> || wire != <span class="s">2</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> (length, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        <span class="k">let</span> next = (*at <span class="k">as</span> u64).checked_add(length)?;

        <span class="k">if</span> <span class="k">let</span> Some(end) = parent_end
            &amp;&amp; next &gt; end
        {
            <span class="k">return</span> None;
        }

        <span class="k">if</span> field == want {
            <span class="k">if</span> want == <span class="s">8</span> &amp;&amp; parent_end != Some(next) {
                <span class="k">return</span> None;
            }

            <span class="k">return</span> Some(next);
        }

        <span class="k">if</span> length &gt; NAME_BYTES {
            <span class="k">return</span> None;
        }

        *at = usize::try_from(next).ok()?;
    }
}

<span class="c">/// True when the value fits an f32.</span>
<span class="k">fn</span> finite_float(value: f64) -&gt; bool {
    value.is_finite() &amp;&amp; (value <span class="k">as</span> f32).is_finite()
}

<span class="c">/// \`count\` xyz triples as f32; None when short or not finite.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">fn</span> checked_positions(raw: &amp;[u8], count: u32) -&gt; Option&lt;Vec&lt;f32&gt;&gt; {
    <span class="k">if</span> raw.len() <span class="k">as</span> u64 != u64::from(count).checked_mul(<span class="s">24</span>)? {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(raw.len() / <span class="s">8</span>);

    <span class="k">for</span> bytes <span class="k">in</span> raw.chunks_exact(<span class="s">8</span>) {
        <span class="k">let</span> value = f64::from_le_bytes(bytes.try_into().ok()?);

        <span class="k">if</span> !finite_float(value) {
            <span class="k">return</span> None;
        }

        out.push(value <span class="k">as</span> f32);
    }

    Some(out)
}

<span class="c">/// True when a range is small enough to read.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">fn</span> bounded_range(at: u64, length: u64) -&gt; bool {
    length &lt;= <span class="s">64</span> * <span class="s">1024</span> * <span class="s">1024</span> &amp;&amp; at.checked_add(length).is_some()
}

<span class="c">/// Checked body bounds, shared by metadata, position and color ranges.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="c">/// \`at + length\` when it stays within \`end\`.</span>
<span class="k">fn</span> body_end(at: u64, length: u64, end: u64) -&gt; Option&lt;u64&gt; {
    <span class="k">let</span> next = at.checked_add(length)?;

    <span class="k">if</span> next &lt;= end { Some(next) } <span class="k">else</span> { None }
}

<span class="c">/// A packed \`int32\` (varint) array in full.</span>
<span class="k">pub</span> <span class="k">fn</span> packed_i32(raw: &amp;[u8]) -&gt; Vec&lt;i32&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> i = <span class="s">0usize</span>;

    <span class="k">while</span> i &lt; raw.len() {
        <span class="k">let</span> Some((v, n)) = varint(raw, i) <span class="k">else</span> { <span class="k">break</span> };
        out.push(v <span class="k">as</span> i32);
        i += n;
    }

    out
}

<span class="c">/// A packed \`double\` array in full.</span>
<span class="k">pub</span> <span class="k">fn</span> packed_f64(raw: &amp;[u8]) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(raw.len() / <span class="s">8</span>);

    <span class="k">for</span> c <span class="k">in</span> raw.chunks_exact(<span class="s">8</span>) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()));
    }

    out
}

<span class="c">/// An already-fetched coords slice as f32 triples.</span>
<span class="k">pub</span> <span class="k">fn</span> positions_from(raw: &amp;[u8]) -&gt; Vec&lt;f32&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(raw.len() / <span class="s">8</span>);

    <span class="k">for</span> c <span class="k">in</span> raw.chunks_exact(<span class="s">8</span>) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) <span class="k">as</span> f32);
    }

    out
}

<span class="c">/// \`count\` colours from packed varints, and where they ended.</span>
<span class="k">pub</span> <span class="k">fn</span> colors_from(raw: &amp;[u8], count: u32) -&gt; Option&lt;(Vec&lt;u32&gt;, usize)&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(count <span class="k">as</span> usize);
    <span class="k">let</span> <span class="k">mut</span> i = <span class="s">0usize</span>;

    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..count {
        <span class="k">let</span> <span class="k">mut</span> rgba = [<span class="s">255u8</span>; <span class="s">4</span>];

        <span class="k">for</span> k <span class="k">in</span> &amp;<span class="k">mut</span> rgba {
            <span class="k">let</span> (v, n) = varint(raw, i)?;
            i += n;
            *k = (v &amp; <span class="s">255</span>) <span class="k">as</span> u8;
        }

        out.push(u32::from_le_bytes(rgba));
    }

    Some((out, i))
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">use</span> web::*;

<span class="c">/// The reads: find the arrays, then fetch slices by range.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">mod</span> web {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::fetch::{GetOpts, fetch_range, get};

    <span class="k">const</span> POINT_BYTES: u64 = <span class="s">24</span>; <span class="c">// three doubles</span>

    <span class="k">const</span> MAX_TABLE_BYTES: u64 = <span class="s">128</span> * <span class="s">1024</span> * <span class="s">1024</span>; <span class="c">// largest node table read</span>

    <span class="c">/// Read one range of the file.</span>
    <span class="k">async</span> <span class="k">fn</span> source_range(
        url: &amp;str,
        at: u64,
        length: u64,
        revision: &amp;Option&lt;String&gt;,
    ) -&gt; Option&lt;Vec&lt;u8&gt;&gt; {
        <span class="k">if</span> !bounded_range(at, length) {
            <span class="k">return</span> None;
        }

        <span class="k">if</span> length == <span class="s">0</span> {
            <span class="k">return</span> Some(Vec::new());
        }

        Some(fetch_range(url, at, length, revision).<span class="k">await</span>.ok()?.<span class="s">0</span>)
    }

    <span class="c">/// Find where a cloud's arrays are in the file.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> cloud_fields(url: &amp;str) -&gt; Option&lt;CloudFields&gt; {
        <span class="k">let</span> reply = get(
            url,
            &amp;GetOpts {
                range: Some((<span class="s">0</span>, <span class="s">8192</span>)),
                revalidate: <span class="s">true</span>,
                ..Default::default()
            },
        )
        .<span class="k">await</span>
        .ok()?;

        <span class="k">if</span> reply.status != <span class="s">206</span> || reply.bytes.len() &gt; <span class="s">8192</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> (coords_at, coords_len, end) = cloud_layout(&amp;reply.bytes)?;

        <span class="k">if</span> coords_len == <span class="s">0</span> || !coords_len.is_multiple_of(POINT_BYTES) {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> after = body_end(coords_at, coords_len, end)?;
        <span class="k">let</span> <span class="k">mut</span> colors = (after, <span class="s">0</span>);

        <span class="k">if</span> after &lt; end {
            <span class="k">let</span> header = source_range(url, after, <span class="s">16</span>.min(end - after), &amp;reply.etag).<span class="k">await</span>?;
            <span class="k">let</span> (tag, used) = varint(&amp;header, <span class="s">0</span>)?;

            <span class="k">if</span> tag &gt;&gt; <span class="s">3</span> == <span class="s">4</span> &amp;&amp; tag &amp; <span class="s">7</span> == <span class="s">2</span> {
                <span class="k">let</span> (length, extra) = varint(&amp;header, used)?;
                <span class="k">let</span> body = after.checked_add((used + extra) <span class="k">as</span> u64)?;
                body_end(body, length, end)?;
                colors = (body, length);
            }
        }

        Some(CloudFields {
            end,
            coords_at,
            coords_len,
            colors_at: colors.<span class="s">0</span>,
            colors_len: colors.<span class="s">1</span>,
            count: u32::try_from(coords_len / POINT_BYTES).ok()?,
            ids_at: <span class="s">0</span>,
            ids_len: <span class="s">0</span>,
            revision: reply.etag,
        })
    }

    <span class="c">/// Read the cloud's node table.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> cloud_lod(url: &amp;str, fields: &amp;<span class="k">mut</span> CloudFields) -&gt; Option&lt;CloudLod&gt; {
        <span class="k">let</span> <span class="k">mut</span> at = body_end(fields.colors_at, fields.colors_len, fields.end)?;
        <span class="k">let</span> <span class="k">mut</span> lod = CloudLod::default();
        <span class="k">let</span> <span class="k">mut</span> seen = [<span class="s">false</span>; <span class="s">7</span>];
        <span class="k">let</span> <span class="k">mut</span> table_bytes = <span class="s">0u64</span>;
        <span class="k">let</span> <span class="k">mut</span> ids = None;

        <span class="k">while</span> at &lt; fields.end {
            <span class="k">let</span> header = source_range(url, at, <span class="s">64</span>.min(fields.end - at), &amp;fields.revision).<span class="k">await</span>?;
            <span class="k">let</span> (tag, used) = varint(&amp;header, <span class="s">0</span>)?;
            <span class="k">let</span> (field, wire) = (usize::try_from(tag &gt;&gt; <span class="s">3</span>).ok()?, (tag &amp; <span class="s">7</span>) <span class="k">as</span> u32);

            <span class="k">if</span> field == <span class="s">0</span> {
                <span class="k">return</span> None;
            }

            <span class="k">if</span> wire != <span class="s">2</span> {
                <span class="k">if</span> (<span class="s">8</span>..=<span class="s">15</span>).contains(&amp;field) {
                    <span class="k">return</span> None;
                }

                <span class="k">let</span> skip = skip_scalar(&amp;header, used, wire)?;
                at = body_end(at, (used + skip) <span class="k">as</span> u64, fields.end)?;
                <span class="k">continue</span>;
            }

            <span class="k">let</span> (length, extra) = varint(&amp;header, used)?;
            <span class="k">let</span> body = body_end(at, (used + extra) <span class="k">as</span> u64, fields.end)?;
            <span class="k">let</span> next = body_end(body, length, fields.end)?;

            <span class="k">if</span> (<span class="s">8</span>..=<span class="s">14</span>).contains(&amp;field) {
                <span class="k">if</span> seen[field - <span class="s">8</span>] {
                    <span class="k">return</span> None;
                }

                table_bytes = table_bytes.checked_add(length)?;

                <span class="k">if</span> table_bytes &gt; MAX_TABLE_BYTES {
                    <span class="k">return</span> None;
                }

                <span class="k">let</span> raw = source_range(url, body, length, &amp;fields.revision).<span class="k">await</span>?;

                <span class="k">if</span> !lod.set_field(field, &amp;raw) {
                    <span class="k">return</span> None;
                }

                seen[field - <span class="s">8</span>] = <span class="s">true</span>;
            }

            <span class="k">if</span> field == <span class="s">15</span> {
                <span class="k">if</span> length != u64::from(fields.count) * <span class="s">4</span> || ids.is_some() {
                    <span class="k">return</span> None;
                }

                ids = Some((body, length));
            }

            at = next;
        }

        <span class="k">if</span> seen.contains(&amp;<span class="s">false</span>) || !lod.valid(fields.count) {
            <span class="k">return</span> None;
        }

        (fields.ids_at, fields.ids_len) = ids.unwrap_or((<span class="s">0</span>, <span class="s">0</span>));
        Some(lod)
    }

    <span class="c">/// Points \`[from, to)\` of the cloud.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> fetch_positions(
        url: &amp;str,
        fields: &amp;CloudFields,
        from: u32,
        to: u32,
    ) -&gt; Option&lt;Vec&lt;f32&gt;&gt; {
        <span class="k">if</span> from &gt; to || to &gt; fields.count {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> at = fields
            .coords_at
            .checked_add(u64::from(from).checked_mul(POINT_BYTES)?)?;
        <span class="k">let</span> length = u64::from(to - from).checked_mul(POINT_BYTES)?;
        body_end(
            at,
            length,
            body_end(fields.coords_at, fields.coords_len, fields.end)?,
        )?;
        <span class="k">let</span> raw = source_range(url, at, length, &amp;fields.revision).<span class="k">await</span>?;
        checked_positions(&amp;raw, to - from)
    }

    <span class="c">/// Colours of \`count\` points starting at byte \`at\`.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> fetch_colors(
        url: &amp;str,
        fields: &amp;CloudFields,
        at: u64,
        count: u32,
    ) -&gt; Option&lt;(Vec&lt;u32&gt;, u64)&gt; {
        <span class="k">let</span> end = body_end(fields.colors_at, fields.colors_len, fields.end)?;

        <span class="k">if</span> at &lt; fields.colors_at || at &gt; end || count &gt; fields.count {
            <span class="k">return</span> None;
        }

        <span class="c">// a colour is at most 8 bytes of varints</span>
        <span class="k">let</span> length = (u64::from(count) * <span class="s">8</span>).min(end - at);

        <span class="k">if</span> length == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> raw = source_range(url, at, length, &amp;fields.revision).<span class="k">await</span>?;
        <span class="k">let</span> (colors, used) = colors_from(&amp;raw, count)?;
        Some((colors, body_end(at, used <span class="k">as</span> u64, end)?))
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A two-node table.</span>
    <span class="k">fn</span> valid_lod() -&gt; CloudLod {
        CloudLod {
            min: vec![<span class="s">0</span>.<span class="s">0</span>; <span class="s">6</span>],
            size: vec![<span class="s">10</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
            spacing: vec![<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">5</span>],
            level: vec![<span class="s">0</span>, <span class="s">1</span>],
            first: vec![<span class="s">0</span>, <span class="s">1</span>],
            count: vec![<span class="s">1</span>, <span class="s">1</span>],
            children: [vec![<span class="s">1</span>], vec![-<span class="s">1</span>; <span class="s">15</span>]].concat(),
        }
    }

    <span class="c">/// Bad node tables are refused.</span>
    #[test]
    <span class="k">fn</span> lod_parallel_arrays_bounds_ranges_and_child_graph_are_validated() {
        assert!(valid_lod().valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.min.pop();
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.spacing[<span class="s">0</span>] = f64::NAN;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.size[<span class="s">0</span>] = f64::MAX;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.min[<span class="s">0</span>] = f32::MAX <span class="k">as</span> f64;
        lod.size[<span class="s">0</span>] = f32::MAX <span class="k">as</span> f64;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.first[<span class="s">1</span>] = <span class="s">2</span>;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.count[<span class="s">1</span>] = -<span class="s">1</span>;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.children[<span class="s">0</span>] = <span class="s">2</span>;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.children[<span class="s">8</span>] = <span class="s">0</span>;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.children[<span class="s">0</span>] = -<span class="s">1</span>;
        assert!(!lod.valid(<span class="s">2</span>));
        <span class="k">let</span> <span class="k">mut</span> lod = valid_lod();
        lod.children[<span class="s">1</span>] = <span class="s">1</span>;
        assert!(!lod.valid(<span class="s">2</span>));
    }

    <span class="c">/// Short or non-finite arrays are refused.</span>
    #[test]
    <span class="k">fn</span> packed_metadata_and_positions_reject_partial_or_nonfinite_values() {
        <span class="k">let</span> <span class="k">mut</span> lod = CloudLod::default();
        assert!(!lod.set_field(<span class="s">8</span>, &amp;[<span class="s">0</span>; <span class="s">7</span>]));
        assert!(!lod.set_field(<span class="s">14</span>, &amp;[<span class="s">0x80</span>]));
        <span class="k">let</span> raw: Vec&lt;_&gt; = [<span class="s">1</span>.<span class="s">0f64</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        assert_eq!(checked_positions(&amp;raw, <span class="s">1</span>).unwrap(), [<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>]);
        assert!(checked_positions(&amp;raw[..<span class="s">23</span>], <span class="s">1</span>).is_none());
        assert!(checked_positions(&amp;raw, <span class="s">2</span>).is_none());

        <span class="k">for</span> bad <span class="k">in</span> [f64::NAN, f64::INFINITY, f64::MAX] {
            <span class="k">let</span> raw: Vec&lt;_&gt; = [bad, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .collect();
            assert!(checked_positions(&amp;raw, <span class="s">1</span>).is_none());
        }
    }

    <span class="c">/// Ranges never overflow or leave the message.</span>
    #[test]
    <span class="k">fn</span> source_offsets_do_not_wrap_or_cross_the_enclosing_message() {
        assert!(bounded_range(<span class="s">0</span>, <span class="s">64</span> * <span class="s">1024</span> * <span class="s">1024</span>));
        assert!(!bounded_range(<span class="s">0</span>, <span class="s">64</span> * <span class="s">1024</span> * <span class="s">1024</span> + <span class="s">1</span>));
        assert!(!bounded_range(u64::MAX - <span class="s">1</span>, <span class="s">3</span>));
        assert_eq!(body_end(u64::MAX - <span class="s">1</span>, <span class="s">3</span>, u64::MAX), None);
        assert_eq!(body_end(<span class="s">50</span>, <span class="s">51</span>, <span class="s">100</span>), None);
        assert_eq!(body_end(<span class="s">50</span>, <span class="s">50</span>, <span class="s">100</span>), Some(<span class="s">100</span>));
        <span class="c">// a coords field longer than its message</span>
        assert_eq!(walk_to_coords(&amp;[<span class="s">0x1a</span>, <span class="s">4</span>, <span class="s">0x42</span>, <span class="s">2</span>, <span class="s">0x1a</span>, <span class="s">24</span>]), None);
    }

    <span class="c">/// Varints of one and two bytes read back.</span>
    #[test]
    <span class="k">fn</span> varint_reads_one_and_two_byte_values() {
        assert_eq!(varint(&amp;[<span class="s">0x05</span>], <span class="s">0</span>), Some((<span class="s">5</span>, <span class="s">1</span>)));
        assert_eq!(varint(&amp;[<span class="s">0xac</span>, <span class="s">0x02</span>], <span class="s">0</span>), Some((<span class="s">300</span>, <span class="s">2</span>)));
        assert_eq!(varint(&amp;[<span class="s">0x80</span>], <span class="s">0</span>), None);
        assert_eq!(
            varint(
                &amp;[<span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">1</span>],
                <span class="s">0</span>
            ),
            Some((u64::MAX, <span class="s">10</span>))
        );
        assert_eq!(
            varint(
                &amp;[<span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">0xff</span>, <span class="s">2</span>],
                <span class="s">0</span>
            ),
            None
        );
    }

    <span class="c">/// Colours decode and report where they end.</span>
    #[test]
    <span class="k">fn</span> colors_decode_and_report_their_end() {
        <span class="k">let</span> raw = [<span class="s">127u8</span>, <span class="s">0</span>, <span class="s">100</span>, <span class="s">127</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>, <span class="s">4</span>, <span class="s">9</span>, <span class="s">9</span>];
        <span class="k">let</span> (c, used) = colors_from(&amp;raw, <span class="s">2</span>).unwrap();
        assert_eq!(c, [<span class="s">0x7f64_007f</span>, <span class="s">0x0403_0201</span>]);
        assert_eq!(used, <span class="s">8</span>);
    }
}</code></pre></div>
<h2 id="step-5-srcenginegpupickrs">Step 5 · src/engine/gpu/pick.rs<a class="anchor" href="#/course/13-controls#step-5-srcenginegpupickrs" aria-label="Link to this section">#</a></h2>
<p>Picking reads an object and subobject ID asynchronously.</p>
<p><code>lessons/13/src/engine/gpu/pick.rs</code> · edit · type this</p>
<p>Added after the <code>Edge,</code> line in <code>enum PickMode</code> of <code>lessons/12/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Controls { <span class="c">// control dots of one object</span>
        parent: u32, <span class="c">// the object row</span>
        cloud: bool, <span class="c">// true = pick cloud points instead</span>
    },
}

<span class="c">/// Stage of a multi-page source point query.</span>
#[derive(Clone, Copy, PartialEq, Eq)]
<span class="k">enum</span> SourcePhase {
    Inactive, <span class="c">// no query running</span>
    FirstPage,
    MorePages, <span class="c">// later pages keep them</span></code></pre></div>
<p>Added after the <code>radius: u32,</code> line in <code>struct Picker</code> of <code>lessons/12/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    source_phase: SourcePhase, <span class="c">// stage of a source point query</span></code></pre></div>
<p>Added after the <code>radius: PICK_RADIUS,</code> line in <code>fn new</code> of <code>lessons/12/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            source_phase: SourcePhase::Inactive,</code></pre></div>
<p>Replaces the 2 lines from <code>self.generation = self.generation.wrapping_ad…</code> in <code>fn cancel</code> of <code>lessons/12/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_phase = SourcePhase::Inactive;
        <span class="k">self</span>.generation = <span class="k">self</span>.generation.wrapping_add(<span class="s">1</span>);
        <span class="k">self</span>.pending = None;
    }

    <span class="c">/// Start a source point query.</span>
    <span class="k">pub</span> <span class="k">fn</span> start_source_query(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.cancel();
        <span class="k">self</span>.source_phase = SourcePhase::FirstPage;
    }

    <span class="c">/// True while a source point query runs.</span>
    <span class="k">pub</span> <span class="k">fn</span> source_query(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.source_phase != SourcePhase::Inactive
    }

    <span class="c">/// True once the first page of the query was drawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> source_initialized(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.source_phase == SourcePhase::MorePages
    }

    <span class="c">/// Open a pass for one query page; the first page clears the ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_source&lt;'a&gt;(
        &amp;'a <span class="k">mut</span> <span class="k">self</span>,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> first = <span class="k">self</span>.source_phase == SourcePhase::FirstPage;
        <span class="k">self</span>.source_phase = SourcePhase::MorePages;
        <span class="k">let</span> target = <span class="k">self</span>
            .targets
            .as_ref()
            .expect(&quot;<span class="s">physical query pass initializes targets</span>&quot;);
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">source points</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: &amp;target.id_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: <span class="k">if</span> first {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    } <span class="k">else</span> {
                        wgpu::LoadOp::Load
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    <span class="c">/// The readback window around \`at\` at the current tolerance.</span></code></pre></div>
<h2 id="step-6-srcenginegpurenderrs">Step 6 · src/engine/gpu/render.rs<a class="anchor" href="#/course/13-controls#step-6-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/13/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the <code>draws += self.selection_outline.draw(pass);</code> line in <code>fn scene_list</code> of <code>lessons/12/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        draws += <span class="k">self</span>.control_net.draw_ribbons(pass, &amp;b);
        draws += <span class="k">self</span>.controls.draw_dots(pass, &amp;b);</code></pre></div>
<p>Added after the <code>};</code> line in <code>fn scene_list</code> of <code>lessons/12/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// source point query: faces and clouds, then the source dots</span>
        <span class="k">if</span> <span class="k">self</span>.pick.source_query() {
            <span class="k">if</span> !<span class="k">self</span>.pick.source_initialized() {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_pass(&amp;<span class="k">self</span>.ctx, encoder, size);

                <span class="k">if</span> <span class="k">let</span> Some(window) = window {
                    pass.set_scissor_rect(window.x, window.y, window.w, window.h);
                }

                <span class="k">self</span>.arena.draw_face_ids(&amp;<span class="k">mut</span> pass, &amp;basic);
                <span class="k">self</span>.splat.draw_ids(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.cloud_group);
            }

            {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_source(encoder);

                <span class="k">if</span> <span class="k">let</span> Some(window) = window {
                    pass.set_scissor_rect(window.x, window.y, window.w, window.h);
                }

                <span class="k">let</span> source = Binds {
                    mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
                    line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
                    instances: &amp;<span class="k">self</span>.objects.ink_group, <span class="c">// per-object rows plus depth</span>
                };
                <span class="k">self</span>.controls.draw_source_ids(&amp;<span class="k">mut</span> pass, &amp;source);
            }

            <span class="k">if</span> <span class="k">let</span> Some(at) = at {
                <span class="k">self</span>.pick.copy_window(&amp;<span class="k">self</span>.ctx, encoder, at);
            }

            <span class="k">return</span>;
        }</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn scene_list</code> of <code>lessons/12/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                PickMode::Controls { cloud: <span class="s">false</span>, .. } =&gt; {
                    <span class="k">self</span>.controls.draw_dot_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                }
                PickMode::Controls { cloud: <span class="s">true</span>, .. } =&gt; {}</code></pre></div>
<h2 id="step-7-srcstaters">Step 7 · src/state.rs<a class="anchor" href="#/course/13-controls#step-7-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from <code>use crate::app::selection::SelectionMode;</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::selection::{ControlId, Controls, SelectionMode};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::StreamRows;
<span class="k">use</span> <span class="k">crate</span>::app::walk::encode::FACING_UNKNOWN;
<span class="k">use</span> <span class="k">crate</span>::camera::Camera;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::pick::PickMode;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, GlyphPoint};</code></pre></div>
<p>Replaces the 4 lines from <code>requested: PickMode,</code> in <code>struct State</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    controls: Controls, <span class="c">// control points of the selected object</span>
    requested: PickMode, <span class="c">// what the pending pick looks for</span>
    <span class="k">pub</span> selection_radius_css: f64, <span class="c">// click tolerance in CSS pixels</span>
    scene_labels: Vec&lt;TextLabel&gt;, <span class="c">// Annotations are shaped when documents change.</span>
    show_selected_names: bool, <span class="c">// name label on the selection, T toggles</span>
    cloud_query: Option&lt;<span class="k">crate</span>::app::cloud_query::Query&gt;, <span class="c">// a point-cloud pick in flight</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    query_generation: u64, <span class="c">// counts cloud queries, old answers dropped</span></code></pre></div>
<p>Replaces the 4 lines from <code>requested: PickMode::Object,</code> in <code>fn new</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            controls: Controls::default(),
            requested: PickMode::Object,
            selection_radius_css: <span class="s">6</span>.<span class="s">0</span>,
            scene_labels: Vec::new(), <span class="c">// no text yet</span>
            show_selected_names: <span class="s">true</span>,
            cloud_query: None,
            #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
            query_generation: <span class="s">0</span>,</code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>self.selection = SelectionMode::Object;</code> line in <code>fn clear</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.controls = Controls::default();</code></pre></div>
<p>Added after the <code>self.gpu.logical_size = self.logical_size();</code> line in <code>fn resize</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.upload_controls();</code></pre></div>
<p>Added after the <code>pub fn touch(&amp;mut self) {</code> line in <code>impl State</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_cloud_query();</code></pre></div>
<p>Added after the <code>self.selection = SelectionMode::Object;</code> line in <code>fn select</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.controls = Controls::default();</code></pre></div>
<p>Added after the <code>self.scene.selected = row;</code> line in <code>fn select</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// T: show or hide the name label on the selection.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_selected_names(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.show_selected_names = !<span class="k">self</span>.show_selected_names;
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// H: hide the selection.</span></code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>fn apply_pick(&amp;mut self, pick: Option&lt;Pick&gt;) {</code> line in <code>impl State</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a point-cloud query takes the answer</span>
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        <span class="k">if</span> <span class="k">self</span>.cloud_query_awaiting_gpu() {
            <span class="k">self</span>.apply_cloud_query_pick(pick);
            <span class="k">return</span>;
        }</code></pre></div>
<p>Added after the <code>self.status(&amp;format!(&quot;Edge {edge} selected&quot;));</code> line in <code>fn apply_pick</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            PickMode::Controls { parent, cloud } =&gt; {
                <span class="k">if</span> <span class="k">let</span> Some(pick) = pick
                    &amp;&amp; pick.row == parent
                {
                    <span class="k">self</span>.apply_control(pick, cloud);
                }

                <span class="k">return</span>;
            }</code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>self.gpu.logical_size = logical;</code> line in <code>fn render</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.upload_controls();</code></pre></div>
<p>Added after the <code>crate::app::feedback::error(&amp;message);</code> line in <code>fn render</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.cancel_cloud_query();</code></pre></div>
<p>Replaces the 5 lines from <code>}</code> in <code>fn render</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        } <span class="k">else</span> <span class="k">if</span> <span class="k">self</span>.cloud_query_awaiting_gpu() &amp;&amp; !<span class="k">self</span>.gpu.pick.busy() {
            <span class="k">self</span>.cloud_query = None;
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.upload_controls();
            <span class="k">self</span>.status(&quot;<span class="s">Point query failed during GPU readback; click to retry</span>&quot;);
        }

        <span class="k">self</span>.needs_frame = <span class="s">false</span>;

        <span class="k">if</span> <span class="k">self</span>.gpu.view.spin {
            <span class="k">self</span>.cancel_cloud_query();</code></pre></div>
<p>Replaces the <code>if self.dirty {</code> line in <code>fn render</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">self</span>.dirty &amp;&amp; !<span class="k">self</span>.cloud_query_awaiting_gpu() {</code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Replaces the 5 lines from <code>self.gpu.pick.cancel();</code> in <code>fn request_selection</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cancel_cloud_query();
        <span class="k">self</span>.gpu.pick.cancel();

        <span class="c">// a point cloud answers by its own query</span>
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        <span class="k">if</span> !edge &amp;&amp; <span class="k">self</span>.start_cloud_query(x, y) {
            <span class="k">return</span>;
        }

        <span class="k">let</span> mode = <span class="k">if</span> edge {
            PickMode::Edge
        } <span class="k">else</span> {
            <span class="k">match</span> <span class="k">self</span>.selection {
                SelectionMode::Controls { parent, cloud, .. } =&gt; {
                    PickMode::Controls { parent, cloud }
                }
                _ =&gt; PickMode::Object,
            }</code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl State</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// F10: show the control points of the selected object.</span>
    <span class="k">pub</span> <span class="k">fn</span> enable_controls(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(parent) = <span class="k">self</span>.scene.selected <span class="k">else</span> {
            <span class="k">self</span>.status(&quot;<span class="s">Select one object before pressing F10</span>&quot;);
            <span class="k">return</span>;
        };

        <span class="c">// already on</span>
        <span class="k">if</span> matches!(<span class="k">self</span>.selection, SelectionMode::Controls { parent: active, .. } <span class="k">if</span> active == parent)
        {
            <span class="k">return</span>;
        }
        <span class="c">// the points come from the source geometry</span>
        <span class="k">let</span> controls = <span class="k">match</span> <span class="k">self</span>.scene.geometry(parent) {
            Some(geometry) =&gt; Controls::from_geometry(geometry),
            None <span class="k">if</span> <span class="k">self</span>.streamed_slot(parent).is_some() =&gt; Controls {
                cloud: <span class="s">true</span>,
                ..Controls::default()
            },
            None =&gt; {
                <span class="k">self</span>.status(&quot;<span class="s">Source controls are unavailable for this display-only object</span>&quot;);
                <span class="k">return</span>;
            }
        };

        <span class="k">if</span> !controls.cloud &amp;&amp; controls.points.is_empty() {
            <span class="k">self</span>.status(&quot;<span class="s">This object has no selectable source controls</span>&quot;);
            <span class="k">return</span>;
        }

        <span class="k">self</span>.selection.enable_controls(Some(parent), controls.cloud);
        <span class="k">self</span>.gpu.segments.set_edge(&amp;<span class="k">self</span>.gpu.ctx, None);
        <span class="k">self</span>.gpu.set_selected(parent, <span class="s">false</span>);
        <span class="k">self</span>.gpu
            .splat
            .set_controls(controls.cloud.then_some(parent));
        <span class="k">self</span>.controls = controls;
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.status(&quot;<span class="s">Control points: click to select; Esc to leave</span>&quot;);
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Esc: leave control points, keep the object selected.</span></code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl State</code> of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Upload the source controls once per F10.</span>
    <span class="k">fn</span> upload_controls(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.gpu.controls.reset();
        <span class="k">self</span>.gpu.control_net.reset();
        <span class="k">let</span> SelectionMode::Controls {
            parent, selected, ..
        } = <span class="k">self</span>.selection
        <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / <span class="k">self</span>.logical_size()[<span class="s">0</span>]; <span class="c">// device pixels per CSS pixel</span>
        <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();

        <span class="c">// one dot per control point</span>
        <span class="k">for</span> control <span class="k">in</span> &amp;<span class="k">self</span>.controls.points {
            <span class="k">let</span> color = <span class="k">if</span> Some(control.id) == selected {
                [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>] <span class="c">// yellow: selected</span>
            } <span class="k">else</span> {
                [<span class="s">0</span>.<span class="s">15</span>, <span class="s">0</span>.<span class="s">35</span>, <span class="s">0</span>.<span class="s">9</span>, <span class="s">1</span>.<span class="s">0</span>] <span class="c">// blue</span>
            };
            glyphs.dots.push(GlyphPoint {
                center: render_position(control.position),
                radius: -<span class="s">3</span>.<span class="s">5</span> * scale <span class="k">as</span> f32, <span class="c">// negative: a pixel size, not a world size</span>
                color,
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [FACING_UNKNOWN; <span class="s">2</span>],
            });
        }

        <span class="k">let</span> <span class="k">mut</span> segments = SegRows::default();

        <span class="c">// one thin line per link between control points</span>
        <span class="k">for</span> &amp;[start, end] <span class="k">in</span> &amp;<span class="k">self</span>.controls.links {
            segments.ribbons.push(CylinderSegment {
                p0: render_position(<span class="k">self</span>.controls.points[start].position),
                p1: render_position(<span class="k">self</span>.controls.points[end].position),
                radius: <span class="s">0</span>.<span class="s">0</span>,
                color: <span class="s">0xffcc8866</span>,
                instance_id: parent,
                facing: FACING_UNKNOWN,
            });
        }

        <span class="k">self</span>.gpu
            .controls
            .append(&amp;<span class="k">self</span>.gpu.ctx, &amp;<span class="k">self</span>.gpu.layouts, &amp;glyphs);
        <span class="k">self</span>.gpu
            .control_net
            .append(&amp;<span class="k">self</span>.gpu.ctx, &amp;<span class="k">self</span>.gpu.layouts, &amp;segments);
    }

    <span class="c">/// A control point was clicked: select it.</span>
    <span class="k">fn</span> apply_control(&amp;<span class="k">mut</span> <span class="k">self</span>, pick: Pick, cloud: bool) {
        <span class="k">let</span> SelectionMode::Controls { parent, .. } = <span class="k">self</span>.selection <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> pick.row != parent {
            <span class="k">return</span>;
        }

        <span class="k">let</span> id = <span class="k">if</span> cloud {
            <span class="c">// a cloud point: find its source id</span>
            <span class="k">let</span> Some((owner, local)) = <span class="k">self</span>.gpu.cloud.row_of(pick.sub) <span class="k">else</span> {
                <span class="k">return</span>;
            };

            <span class="k">if</span> owner != parent {
                <span class="k">return</span>;
            }

            <span class="k">self</span>.gpu.splat.set_point(Some(pick.sub));
            <span class="k">let</span> Some(source) = <span class="k">self</span>.scene.point_at(parent, local) <span class="k">else</span> {
                <span class="k">self</span>.status(&quot;<span class="s">Source point ID unavailable; no local display ID was substituted</span>&quot;);
                <span class="k">return</span>;
            };
            ControlId::Point(source.id)
        } <span class="k">else</span> {
            <span class="c">// a control dot: top two bits 01, index in the rest</span>
            <span class="k">if</span> pick.sub &amp; <span class="s">0xc000_0000</span> != <span class="s">0x4000_0000</span> {
                <span class="k">return</span>;
            }

            <span class="k">let</span> Some(control) = <span class="k">self</span>.controls.points.get((pick.sub &amp; <span class="s">0x3fff_ffff</span>) <span class="k">as</span> usize) <span class="k">else</span> {
                <span class="k">return</span>;
            };
            control.id
        };
        <span class="k">self</span>.selection = SelectionMode::Controls {
            parent,
            selected: Some(id),
            cloud,
        };
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Selected </span>{<span class="s">id:?</span>}&quot;));
        <span class="k">self</span>.touch();
    }

    <span class="c">/// A source query pauses color frames for readback.</span>
    <span class="k">fn</span> cloud_query_awaiting_gpu(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">match</span> &amp;<span class="k">self</span>.cloud_query {
            Some(query) =&gt; query.awaiting_gpu,
            None =&gt; <span class="s">false</span>,
        }
    }

    <span class="c">/// Locate the streamed descriptor belonging to the active source parent.</span>
    <span class="k">fn</span> streamed_slot(&amp;<span class="k">self</span>, parent: u32) -&gt; Option&lt;usize&gt; {
        <span class="k">for</span> (slot, cloud) <span class="k">in</span> <span class="k">self</span>.scene.streamed.iter().enumerate() {
            <span class="k">if</span> cloud.row == parent {
                <span class="k">return</span> Some(slot);
            }
        }

        None
    }

    <span class="c">/// Newer input cancels this query's callbacks.</span>
    <span class="k">fn</span> cancel_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.cloud_query.take().is_some() {
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.upload_controls();
            <span class="k">self</span>.status(&quot;<span class="s">Point query cancelled because the view or selection changed</span>&quot;);
        }
    }

    <span class="c">/// F10 queries the source, not what is displayed.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="c">/// Begin a source query at a pixel.</span>
    <span class="k">fn</span> start_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32) -&gt; bool {
        <span class="k">use</span> <span class="k">crate</span>::app::cloud_query::{Query, QueryView};
        <span class="k">let</span> SelectionMode::Controls {
            parent,
            cloud: <span class="s">true</span>,
            ..
        } = <span class="k">self</span>.selection
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(slot) = <span class="k">self</span>.streamed_slot(parent) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>.gpu.pick.cancel();
        <span class="k">self</span>.query_generation = <span class="k">self</span>.query_generation.wrapping_add(<span class="s">1</span>);
        <span class="k">let</span> cloud = &amp;<span class="k">self</span>.scene.streamed[slot];
        <span class="k">let</span> projection = <span class="k">self</span>
            .camera
            .view_proj_anchored(<span class="k">self</span>.aspect(), &amp;session_rust::Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / <span class="k">self</span>.logical_size()[<span class="s">0</span>];
        <span class="k">let</span> view = QueryView {
            matrix: &amp;projection * &amp;cloud.place,
            size: [
                f64::from(<span class="k">self</span>.gpu.config.width),
                f64::from(<span class="k">self</span>.gpu.config.height),
            ],
            at: [x, y],
            radius: (<span class="k">self</span>.selection_radius_css * scale).ceil().clamp(<span class="s">1</span>.<span class="s">0</span>, <span class="s">128</span>.<span class="s">0</span>) + <span class="s">3</span>.<span class="s">5</span> * scale,
        };
        <span class="k">self</span>.cloud_query = Some(Query::new(<span class="k">self</span>.query_generation, cloud, view));
        <span class="k">self</span>.gpu.pick.start_source_query();
        <span class="k">self</span>.advance_cloud_query();
        <span class="s">true</span>
    }

    <span class="c">/// Fetch one page; resolve when every page is done.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">fn</span> advance_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        query.awaiting_gpu = <span class="s">false</span>;
        query.candidates.clear();

        <span class="k">if</span> <span class="k">let</span> Some(page) = query.next_page() {
            <span class="k">let</span> progress = format!(
                &quot;<span class="s">Checking source points: </span>{}<span class="s"> / </span>{}<span class="s"> (display LOD remains bounded)</span>&quot;,
                query.checked, query.total
            );
            <span class="k">crate</span>::app::cloud_query::fetch_page(query, page);
            <span class="k">self</span>.status(&amp;progress);
        } <span class="k">else</span> <span class="k">if</span> <span class="k">let</span> Some(best) = query.best {
            <span class="k">crate</span>::app::cloud_query::resolve_id(query, best);
            <span class="k">self</span>.status(&quot;<span class="s">All eligible source points checked; resolving original point ID…</span>&quot;);
        } <span class="k">else</span> {
            <span class="k">self</span>.cloud_query = None;
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.status(&quot;<span class="s">No visible source point in the selection window</span>&quot;);
        }

        <span class="k">self</span>.upload_controls();
    }

    <span class="c">/// A range callback belongs to exactly one camera/scene/parent generation.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> cloud_query_batch(&amp;<span class="k">mut</span> <span class="k">self</span>, batch: <span class="k">crate</span>::app::cloud_query::Batch) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> query.id != batch.query || query.cancelled.get() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> (candidates, revision) = <span class="k">match</span> batch.result {
            Ok(result) =&gt; result,
            Err(error) =&gt; {
                <span class="k">self</span>.cloud_query = None;
                <span class="k">self</span>.gpu.pick.cancel();
                <span class="k">self</span>.upload_controls();
                <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Point query failed: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };
        query.checked += batch.count;
        query.revision = revision;

        <span class="k">if</span> candidates.is_empty() {
            <span class="k">self</span>.advance_cloud_query();
            <span class="k">return</span>;
        }

        query.candidates = candidates;
        query.awaiting_gpu = <span class="s">true</span>;
        <span class="k">let</span> parent = query.parent;
        <span class="k">let</span> at = query.view.at;
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / <span class="k">self</span>.logical_size()[<span class="s">0</span>];
        <span class="k">let</span> query = <span class="k">self</span>.cloud_query.as_ref().unwrap();
        <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();

        <span class="k">for</span> candidate <span class="k">in</span> &amp;query.candidates {
            glyphs.dots.push(GlyphPoint {
                center: render_position(candidate.position),
                radius: -<span class="s">3</span>.<span class="s">5</span> * scale <span class="k">as</span> f32,
                color: [<span class="s">0</span>.<span class="s">15</span>, <span class="s">0</span>.<span class="s">35</span>, <span class="s">0</span>.<span class="s">9</span>, <span class="s">1</span>.<span class="s">0</span>],
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [candidate.local, FACING_UNKNOWN],
            });
        }

        <span class="k">self</span>.gpu.controls.reset();
        <span class="k">self</span>.gpu
            .controls
            .append(&amp;<span class="k">self</span>.gpu.ctx, &amp;<span class="k">self</span>.gpu.layouts, &amp;glyphs);
        <span class="c">// ID targets only; never presented.</span>
        <span class="k">self</span>.requested = PickMode::Controls {
            parent,
            cloud: <span class="s">false</span>,
        };
        <span class="k">self</span>.gpu
            .pick
            .configure(<span class="k">self</span>.requested, <span class="k">self</span>.selection_radius_css, scale);
        <span class="k">self</span>.gpu.pick.request(at[<span class="s">0</span>], at[<span class="s">1</span>]);
        <span class="k">self</span>.needs_frame = <span class="s">true</span>;
    }

    <span class="c">/// Fold this page's answer, then the next node.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="c">/// Take the cloud query's answer.</span>
    <span class="k">fn</span> apply_cloud_query_pick(&amp;<span class="k">mut</span> <span class="k">self</span>, pick: Option&lt;Pick&gt;) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="c">// This is the winner of the accumulated physical source-point depth, including</span>
        query.best = <span class="k">match</span> pick {
            Some(pick) <span class="k">if</span> pick.row == query.parent &amp;&amp; pick.sub &lt; query.fields.count =&gt; {
                Some(pick.sub)
            }
            _ =&gt; None,
        };
        <span class="k">self</span>.advance_cloud_query();
    }

    <span class="c">/// Show the chosen source point even if not resident.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> cloud_query_resolved(&amp;<span class="k">mut</span> <span class="k">self</span>, resolved: <span class="k">crate</span>::app::cloud_query::Resolved) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_ref() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> query.id != resolved.query || query.cancelled.get() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> query = <span class="k">self</span>.cloud_query.take().unwrap();
        <span class="k">let</span> (source, position) = <span class="k">match</span> resolved.result {
            Ok(result) =&gt; result,
            Err(error) =&gt; {
                <span class="k">self</span>.gpu.pick.cancel();
                <span class="k">self</span>.upload_controls();
                <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Point query failed: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };
        <span class="k">let</span> Some(best) = query.best <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> id = ControlId::Point(source);
        <span class="k">self</span>.selection = SelectionMode::Controls {
            parent: query.parent,
            selected: Some(id),
            cloud: <span class="s">true</span>,
        };
        <span class="k">self</span>.controls.points = vec![<span class="k">crate</span>::app::selection::Control { id, position }];
        <span class="k">self</span>.gpu.splat.set_point(None);
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.status(&amp;format!(
            &quot;<span class="s">Selected source point </span>{<span class="s">source</span>}<span class="s"> (row </span>{}<span class="s">); all </span>{}<span class="s"> eligible points checked</span>&quot;,
            best, query.total
        ));
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Keep one readable source-document title above each loaded CAD group.</span></code></pre></div>
<p><code>lessons/13/src/state.rs</code> · edit · type this</p>
<p>Replaces <code>fn update_label</code> in <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Document text and the selected name, white.</span>
    <span class="k">fn</span> update_label(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> <span class="k">mut</span> labels = <span class="k">self</span>.scene_labels.clone();

        <span class="k">if</span> <span class="k">self</span>.show_selected_names
            &amp;&amp; !matches!(<span class="k">self</span>.selection, SelectionMode::Controls { .. })</code></pre></div>
<p>Replaces the <code>/// Center an annotation in the source object…</code> line of <code>lessons/12/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> State {
    <span class="c">/// The control points as JSON, for the inspection tests.</span>
    <span class="k">pub</span> <span class="k">fn</span> inspected_controls(&amp;<span class="k">self</span>) -&gt; Vec&lt;serde_json::Value&gt; {
        <span class="k">let</span> <span class="k">mut</span> points = Vec::with_capacity(<span class="k">self</span>.controls.points.len());

        <span class="k">for</span> point <span class="k">in</span> &amp;<span class="k">self</span>.controls.points {
            points.push(serde_json::json!({&quot;<span class="s">id</span>&quot;: point.id, &quot;<span class="s">position</span>&quot;: point.position}));
        }

        points
    }
}

<span class="c">/// A source position as a GPU control point.</span>
<span class="k">fn</span> render_position(position: [f64; <span class="s">3</span>]) -&gt; [f32; <span class="s">3</span>] {
    [position[<span class="s">0</span>] <span class="k">as</span> f32, position[<span class="s">1</span>] <span class="k">as</span> f32, position[<span class="s">2</span>] <span class="k">as</span> f32]
}

<span class="c">/// Center an annotation in the object's bounds.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// White-on-black annotations.</span></code></pre></div>
<h2 id="step-8-srcappinputrs">Step 8 · src/app/input.rs<a class="anchor" href="#/course/13-controls#step-8-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/13/src/app/input.rs</code> · edit · type this</p>
<p>Added after the <code>Key::Named(NamedKey::Escape) =&gt; state.escape_…</code> line in <code>fn key</code> of <code>lessons/12/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Named(NamedKey::F10) =&gt; state.enable_controls(),</code></pre></div>
<h2 id="step-9-srclibrs">Step 9 · src/lib.rs<a class="anchor" href="#/course/13-controls#step-9-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/13/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>CloudChunk(CloudChunk),</code> line in <code>enum Msg</code> of <code>lessons/12/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    CloudQueryBatch(app::cloud_query::Batch), <span class="c">// points asked for on click</span>
    CloudQueryResolved(app::cloud_query::Resolved), <span class="c">// those points answered</span></code></pre></div>
<p>Added after the <code>Msg::CloudChunk(c) =&gt; state.extend_streamed(c…</code> line in <code>fn user_event</code> of <code>lessons/12/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::CloudQueryBatch(batch) =&gt; state.cloud_query_batch(batch),
            Msg::CloudQueryResolved(resolved) =&gt; state.cloud_query_resolved(resolved),</code></pre></div>
<h2 id="step-10-srcappmodrs">Step 10 · src/app/mod.rs<a class="anchor" href="#/course/13-controls#step-10-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The application module connects source loading and interaction helpers.</p>
<p><code>lessons/13/src/app/mod.rs</code> · edit · type this</p>
<p>Added at the top of <code>lessons/12/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> cloud_query;</code></pre></div>
<p>Added after the <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> line of <code>lessons/12/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> fetch;</code></pre></div>
<h2 id="step-11-srcappinspectionrs">Step 11 · src/app/inspection.rs<a class="anchor" href="#/course/13-controls#step-11-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Copy the change: the snapshot now lists the controls on screen.</p>
<p><code>lessons/13/src/app/inspection.rs</code> · edit · copy the file</p>
<p>Replaces the <code>&quot;controls&quot;: Vec::&lt;serde_json::Value&gt;::new(),</code> line in <code>fn publish</code> of <code>lessons/12/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">controls</span>&quot;: state.inspected_controls(),</code></pre></div>
<h2 id="step-12-srcapploaderrs">Step 12 · src/app/loader.rs<a class="anchor" href="#/course/13-controls#step-12-srcapploaderrs" aria-label="Link to this section">#</a></h2>
<p>The loader stages manifest and geometry work before publishing it.</p>
<p><code>lessons/13/src/app/loader.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>use super::scene::{FileDoc, Scene};</code> of <code>lessons/12/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Loads a scene file and turns it into GPU rows a chunk at a time, so the page never freezes.</span>
<span class="k">use</span> super::scene::{FileDoc, Scene, StreamedInit};
<span class="k">use</span> super::stream::CloudFields;
<span class="k">use</span> super::walk::cloud::StreamRows;</code></pre></div>
<p>Added after the <code>async fn fixture() -&gt; Result&lt;(), String&gt; {</code> line of <code>lessons/12/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> super::route::query(&quot;<span class="s">scene</span>&quot;).as_deref() == Some(&quot;<span class="s">stream-test.yaml</span>&quot;) {
        <span class="k">return</span> streamed_fixture().<span class="k">await</span>;
    }</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/12/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Read a display prefix; keep every source row for F10.</span>
<span class="k">async</span> <span class="k">fn</span> streamed_fixture() -&gt; Result&lt;(), String&gt; {
    <span class="k">let</span> base = super::route::query(&quot;<span class="s">data</span>&quot;).ok_or(&quot;<span class="s">local source URL is missing</span>&quot;)?;

    <span class="k">if</span> !base.starts_with(&quot;<span class="s">http://127.0.0.1:</span>&quot;) &amp;&amp; !base.starts_with(&quot;<span class="s">http://localhost:</span>&quot;) {
        <span class="k">return</span> Err(&quot;<span class="s">The tutorial source fixture must use a local HTTP server</span>&quot;.to_string());
    }

    <span class="k">let</span> url = format!(&quot;{}<span class="s">/cloud.pb</span>&quot;, base.trim_end_matches('<span class="s">/</span>'));
    <span class="k">let</span> <span class="k">mut</span> fields = super::stream::cloud_fields(&amp;url)
        .<span class="k">await</span>
        .ok_or(&quot;<span class="s">invalid cloud envelope</span>&quot;)?;
    <span class="k">let</span> lod = super::stream::cloud_lod(&amp;url, &amp;<span class="k">mut</span> fields)
        .<span class="k">await</span>
        .ok_or(&quot;<span class="s">invalid source node table</span>&quot;)?;
    <span class="k">let</span> resident = fields.count.min(<span class="s">250_000</span>);
    <span class="k">let</span> positions = super::stream::fetch_positions(&amp;url, &amp;fields, <span class="s">0</span>, resident)
        .<span class="k">await</span>
        .ok_or(&quot;<span class="s">invalid source positions</span>&quot;)?;
    <span class="k">let</span> (colors, col_at) = <span class="k">if</span> fields.colors_len == <span class="s">0</span> {
        (Vec::new(), fields.colors_at)
    } <span class="k">else</span> {
        super::stream::fetch_colors(&amp;url, &amp;fields, fields.colors_at, resident)
            .<span class="k">await</span>
            .ok_or(&quot;<span class="s">invalid source colors</span>&quot;)?
    };
    post(Msg::StreamedCloud(Box::new(StreamedInit {
        name: &quot;<span class="s">Source query fixture</span>&quot;.to_string(), <span class="c">// the scene's title</span>
        url,
        place: Xform::identity(),
        rows: StreamRows { positions, colors },
        lod,
        fields,
        resident,
        point_px: <span class="s">3</span>.<span class="s">0</span>, <span class="c">// point size in CSS pixels</span>
        col_at,
    })));
    Ok(())
}

<span class="c">/// Where a cloud's streaming continues.</span></code></pre></div>
<h2 id="step-13-srcappsceners">Step 13 · src/app/scene.rs<a class="anchor" href="#/course/13-controls#step-13-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/13/src/app/scene.rs</code> · edit · copy the file</p>
<p>Added after the <code>}</code> line of <code>lessons/12/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// File placement times the object's own transform.</span>
<span class="k">fn</span> placement(world: &amp;HashMap&lt;String, Xform&gt;, place: &amp;Xform, guid: &amp;str) -&gt; Xform {
    <span class="k">match</span> world.get(guid) {
        Some(local) =&gt; place * local,
        None =&gt; place.clone(),
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::selection::Controls;
    <span class="k">use</span> session_rust::{BRep, Point};

    <span class="c">/// A document at the origin.</span>
    <span class="k">fn</span> file(name: &amp;str, session: Rc&lt;Session&gt;, display_only: bool) -&gt; FileDoc {
        FileDoc {
            name: name.into(),
            session,
            place: Xform::identity(),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only,
        }
    }

    <span class="c">/// The same guid in two documents stays two objects.</span>
    #[test]
    <span class="k">fn</span> duplicate_guids_keep_their_document_and_control_owners() {
        <span class="k">let</span> first = Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> guid = first.guid().to_string();
        <span class="k">let</span> <span class="k">mut</span> second = first.clone();
        second[<span class="s">0</span>] = <span class="s">20</span>.<span class="s">0</span>;
        assert_eq!(second.guid(), guid);
        <span class="k">let</span> <span class="k">mut</span> left = Session::new(&quot;<span class="s">left</span>&quot;);
        left.add_point(first, None);
        <span class="k">let</span> <span class="k">mut</span> right = Session::new(&quot;<span class="s">right</span>&quot;);
        right.add_point(second, None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">left</span>&quot;, Rc::new(left), <span class="s">false</span>));
        scene.add_file(file(&quot;<span class="s">right</span>&quot;, Rc::new(right), <span class="s">false</span>));
        assert_eq!(scene.object_count(), <span class="s">2</span>);
        assert_eq!(scene.document(<span class="s">0</span>).unwrap().name, &quot;<span class="s">left</span>&quot;);
        assert_eq!(scene.document(<span class="s">1</span>).unwrap().name, &quot;<span class="s">right</span>&quot;);
        <span class="k">let</span> a = Controls::from_geometry(scene.geometry(<span class="s">0</span>).unwrap());
        <span class="k">let</span> b = Controls::from_geometry(scene.geometry(<span class="s">1</span>).unwrap());
        assert_eq!(a.points[<span class="s">0</span>].position, [<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        assert_eq!(b.points[<span class="s">0</span>].position, [<span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        scene.hidden.insert(scene.identity_of(<span class="s">1</span>).unwrap());
        assert_eq!(scene.hidden_rows(), vec![<span class="s">1</span>]);
        assert!(scene.document(<span class="s">2</span>).is_none());
    }

    <span class="c">/// Two placements share one session until one is edited.</span>
    #[test]
    <span class="k">fn</span> an_edit_must_split_a_session_two_placements_share() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">twice</span>&quot;);
        source.add_point(Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> shared = Rc::new(source);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">first</span>&quot;, Rc::clone(&amp;shared), <span class="s">false</span>));
        scene.add_file(file(&quot;<span class="s">second</span>&quot;, Rc::clone(&amp;shared), <span class="s">false</span>));
        assert!(Rc::ptr_eq(&amp;scene.docs[<span class="s">0</span>].session, &amp;scene.docs[<span class="s">1</span>].session));

        Rc::make_mut(&amp;<span class="k">mut</span> scene.docs[<span class="s">0</span>].session).add_point(Point::new(<span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);

        assert!(!Rc::ptr_eq(&amp;scene.docs[<span class="s">0</span>].session, &amp;scene.docs[<span class="s">1</span>].session));
        assert_eq!(scene.docs[<span class="s">0</span>].session.lookup.len(), <span class="s">2</span>);
        assert_eq!(scene.docs[<span class="s">1</span>].session.lookup.len(), <span class="s">1</span>);
    }

    <span class="c">/// The old \`display_only\` flag keeps the controls.</span>
    #[test]
    <span class="k">fn</span> legacy_display_only_hint_retains_source_controls() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">retained CAD source</span>&quot;);
        source.add_brep(BRep::create_box(<span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>), None);
        <span class="k">let</span> source = Rc::new(source);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">display hint</span>&quot;, Rc::clone(&amp;source), <span class="s">true</span>));
        assert!(Rc::ptr_eq(&amp;source, &amp;scene.docs[<span class="s">0</span>].session));
        assert!(!scene.docs[<span class="s">0</span>].display_only);
        assert_eq!(scene.object_count(), <span class="s">1</span>);
        <span class="k">let</span> controls = Controls::from_geometry(scene.geometry(<span class="s">0</span>).unwrap());
        assert!(controls.points.len() &gt;= <span class="s">8</span>);
        assert!(!scene.tables.arena.idx.is_empty());
    }
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/13-controls#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/13/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: F10 shows original curve and surface controls, and clicking a control highlights it; status: <strong>Selected Surface { surface: 0, u: 0, v: 1 }</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/13-controls.png" alt="Checkpoint 13, left to right: F10 on the curve shows its three control points and control polygon; F10 on the surface shows the four corners of its control net; a clicked corner turns yellow and the status reads Selected Surface { surface: 0, u: 0, v: 1 }; F10 on the mesh shows its original vertices, not the tessellation." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Repeated F10 adds markers: enabling controls appends instead of replacing them.</li>
<li>The surface shows a dense grid: tessellation vertices replace source controls.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/13-controls#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/13/src/
├── app/
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
│   │   └── points.rs
│   ├── cloud_query.rs  +
│   ├── feedback.rs
│   ├── fetch.rs  +
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── loader.rs  ~
│   ├── mod.rs  ~
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── selection.rs  ~
│   ├── stream.rs  ~
│   └── touch.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs
│   │   ├── objects.rs
│   │   ├── pick.rs  ~
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── selection_outline.rs
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
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
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── selection_outline.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   └── triangle.wgsl
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/13/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/13-controls#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/14-loading">14 · Loading scenes</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/13-controls#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Select a curve or surface and press <strong>F10</strong>: its control points and control polygon appear.</p>
<p><a href="/session/docs/course/docs/screenshots/13.png"><img src="/session/docs/course/docs/screenshots/13.png" alt="Full viewer result for 13 controls" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappselectionrs",text:"Step 1 · src/app/selection.rs"},{level:2,id:"step-2-srcappfetchrs",text:"Step 2 · src/app/fetch.rs"},{level:2,id:"step-3-srcappcloud_queryrs",text:"Step 3 · src/app/cloud_query.rs"},{level:2,id:"step-4-srcappstreamrs",text:"Step 4 · src/app/stream.rs"},{level:2,id:"step-5-srcenginegpupickrs",text:"Step 5 · src/engine/gpu/pick.rs"},{level:2,id:"step-6-srcenginegpurenderrs",text:"Step 6 · src/engine/gpu/render.rs"},{level:2,id:"step-7-srcstaters",text:"Step 7 · src/state.rs"},{level:2,id:"step-8-srcappinputrs",text:"Step 8 · src/app/input.rs"},{level:2,id:"step-9-srclibrs",text:"Step 9 · src/lib.rs"},{level:2,id:"step-10-srcappmodrs",text:"Step 10 · src/app/mod.rs"},{level:2,id:"step-11-srcappinspectionrs",text:"Step 11 · src/app/inspection.rs"},{level:2,id:"step-12-srcapploaderrs",text:"Step 12 · src/app/loader.rs"},{level:2,id:"step-13-srcappsceners",text:"Step 13 · src/app/scene.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
