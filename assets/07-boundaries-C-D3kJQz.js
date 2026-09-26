const s={title:"07 · Shared boundaries",html:`<h1 id="07-shared-boundaries">07 · Shared boundaries<a class="anchor" href="#/course/07-boundaries#07-shared-boundaries" aria-label="Link to this section">#</a></h1>
<p>A cylinder and a block with a hole show continuous source boundary curves.</p>
<p><img src="/session/docs/course/docs/illustrations/shared-boundary.svg" alt="Before: face A, face B and the ink each chord the same edge differently. After: one canonical chain constrains both meshes and the ink is drawn from those nodes." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<h2 id="step-1-session_rustsrcnurbssurface_trimmedrs">Step 1 · session_rust/src/nurbssurface_trimmed.rs<a class="anchor" href="#/course/07-boundaries#step-1-session_rustsrcnurbssurface_trimmedrs" aria-label="Link to this section">#</a></h2>
<p>Read the kernel&#39;s trimmed surface; the lesson uses it as is.</p>
<details class="note"><summary><code>session_rust/src/nurbssurface_trimmed.rs</code> · read only</summary><p><a href="#/course/kernel/nurbssurface_trimmed">Open the full listing</a></p>
</details>
<h2 id="step-2-session_rustsrcbreprs">Step 2 · session_rust/src/brep.rs<a class="anchor" href="#/course/07-boundaries#step-2-session_rustsrcbreprs" aria-label="Link to this section">#</a></h2>
<p>Read the kernel&#39;s BRep; the lesson uses it as is.</p>
<details class="note"><summary><code>session_rust/src/brep.rs</code> · read only</summary><p><a href="#/course/kernel/brep">Open the full listing</a></p>
</details>
<p>Run <code>cargo check</code> in <code>lessons/07/</code>.</p>
<h2 id="step-3-srcappwalkbrep_edgesrs">Step 3 · src/app/walk/brep_edges.rs<a class="anchor" href="#/course/07-boundaries#step-3-srcappwalkbrep_edgesrs" aria-label="Link to this section">#</a></h2>
<p>Chains are keyed by edge so two faces share one boundary.</p>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::Mesh;
<span class="k">use</span> session_rust::brep::{BRep, BRepOrientation};

<span class="k">use</span> super::encode::{Pen, pack_facing};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::CylinderSegment;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> session_rust::AABB;

<span class="c">/// Order two floats, NaN counts as equal.</span>
<span class="k">fn</span> sample_order(a: &amp;f64, b: &amp;f64) -&gt; std::cmp::Ordering {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

<span class="c">/// Order by parameter, then by vertex key.</span>
<span class="k">fn</span> parameter_order(a: &amp;(f64, usize), b: &amp;(f64, usize)) -&gt; std::cmp::Ordering {
    sample_order(&amp;a.<span class="s">0</span>, &amp;b.<span class="s">0</span>).then(a.<span class="s">1</span>.cmp(&amp;b.<span class="s">1</span>))
}

<span class="c">/// Order by parameter with a total float order, then by key.</span>
<span class="k">fn</span> total_parameter_order(a: &amp;(f64, usize), b: &amp;(f64, usize)) -&gt; std::cmp::Ordering {
    a.<span class="s">0</span>.total_cmp(&amp;b.<span class="s">0</span>).then(a.<span class="s">1</span>.cmp(&amp;b.<span class="s">1</span>))</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="c">/// True when two samples share a parameter.</span>
<span class="k">fn</span> same_parameter(a: &amp;<span class="k">mut</span> (f64, usize), b: &amp;<span class="k">mut</span> (f64, usize)) -&gt; bool {
    a.<span class="s">0</span> == b.<span class="s">0</span>
}

<span class="c">/// Parse a sample index, None when not a number.</span>
<span class="k">fn</span> parse_sample_index(value: &amp;str) -&gt; Option&lt;usize&gt; {
    value.parse().ok()
}

<span class="c">/// A BRep edge is shared: each of the two faces meeting there uses the same edge, once from each side.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgeUse {
    <span class="k">pub</span> edge: usize,                  <span class="c">// edge index</span>
    <span class="k">pub</span> face: usize,                  <span class="c">// face index</span>
    <span class="k">pub</span> orientation: BRepOrientation, <span class="c">// which way the face runs it</span>
}

<span class="c">/// The two UV ends of a straight pcurve.</span>
<span class="k">fn</span> pcurve_ends(b: &amp;BRep, eu: &amp;EdgeUse) -&gt; Option&lt;([f64; 2], [f64; 2])&gt; {
    <span class="k">let</span> ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);

    <span class="k">if</span> ci &lt; <span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> c = b.m_curves_2d.get(ci <span class="k">as</span> usize)?;

    <span class="k">if</span> c.degree() != <span class="s">1</span> || c.is_rational() || c.cv_count() != <span class="s">2</span> {
        <span class="k">return</span> None; <span class="c">// not a straight line</span>
    }

    <span class="k">let</span> p0 = c.get_cv(<span class="s">0</span>)?;
    <span class="k">let</span> p1 = c.get_cv(c.cv_count().checked_sub(<span class="s">1</span>)?)?;
    Some(([p0[<span class="s">0</span>], p0[<span class="s">1</span>]], [p1[<span class="s">0</span>], p1[<span class="s">1</span>]]))
}

<span class="c">/// Sorted distinct values of one vertex attribute.</span>
<span class="k">fn</span> sample_values(fm: &amp;Mesh, name: &amp;str) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> <span class="k">mut</span> vals = Vec::new();

    <span class="k">for</span> vertex <span class="k">in</span> fm.vertex.values() {
        <span class="k">if</span> <span class="k">let</span> Some(value) = vertex.attributes.get(name) {
            vals.push(*value);
        }
    }

    vals.sort_by(sample_order);
    vals.dedup();
    vals
}

<span class="c">/// The sample value nearest \`target\`, also across a wrap.</span>
<span class="k">fn</span> nearest_sample(vals: &amp;[f64], target: f64, wrap: Option&lt;(f64, f64)&gt;) -&gt; Option&lt;f64&gt; {
    <span class="k">let</span> <span class="k">mut</span> best: Option&lt;(f64, f64)&gt; = None;

    <span class="k">for</span> &amp;v <span class="k">in</span> vals {
        <span class="k">let</span> <span class="k">mut</span> d = (v - target).abs();

        <span class="k">if</span> <span class="k">let</span> Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }

        <span class="k">if</span> <span class="k">match</span> best {
            Some((bd, _)) =&gt; d &lt; bd,
            None =&gt; <span class="s">true</span>,
        } {
            best = Some((d, v));
        }
    }

    Some(best?.<span class="s">1</span>)
}

<span class="c">/// Vertex keys along a grid edge, in parameter order.</span>
<span class="k">pub</span> <span class="k">fn</span> iso_chain(b: &amp;BRep, fm: &amp;Mesh, eu: &amp;EdgeUse) -&gt; Option&lt;Vec&lt;usize&gt;&gt; {
    <span class="k">let</span> e = b.m_edges.get(eu.edge)?;

    <span class="k">if</span> e.degenerated {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> (p0, p1) = pcurve_ends(b, eu)?;</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> !p0.into_iter().chain(p1).all(f64::is_finite) || (p0[<span class="s">0</span>] != p1[<span class="s">0</span>] &amp;&amp; p0[<span class="s">1</span>] != p1[<span class="s">1</span>]) {
        <span class="k">return</span> None;
    }

    <span class="c">// the parameter that does not change along the edge</span>
    <span class="k">let</span> fixed = <span class="k">if</span> (p1[<span class="s">0</span>] - p0[<span class="s">0</span>]).abs() &lt;= (p1[<span class="s">1</span>] - p0[<span class="s">1</span>]).abs() {
        <span class="s">0</span>
    } <span class="k">else</span> {
        <span class="s">1</span>
    };
    <span class="k">let</span> (fixed_name, free_name) = <span class="k">if</span> fixed == <span class="s">0</span> { (&quot;<span class="s">u</span>&quot;, &quot;<span class="s">v</span>&quot;) } <span class="k">else</span> { (&quot;<span class="s">v</span>&quot;, &quot;<span class="s">u</span>&quot;) };
    <span class="k">let</span> vals = sample_values(fm, fixed_name);

    <span class="k">if</span> vals.is_empty() {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> face = b.m_faces.get(eu.face)?;
    <span class="k">let</span> srf = b.m_surfaces.get(face.surface_index <span class="k">as</span> usize)?;
    <span class="k">let</span> wrap = <span class="k">if</span> srf.is_closed(fixed) {
        srf.domain(fixed) <span class="c">// closed: end equals start</span>
    } <span class="k">else</span> {
        None
    };
    <span class="k">let</span> at = nearest_sample(&amp;vals, p0[fixed], wrap)?;
    <span class="k">let</span> wrapped_start = <span class="k">match</span> wrap {
        Some((start, end)) =&gt; p0[fixed] - (end - start) == at,
        None =&gt; <span class="s">false</span>,
    };

    <span class="k">if</span> at != p0[fixed] &amp;&amp; !wrapped_start {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> <span class="k">mut</span> on_line: Vec&lt;(f64, usize)&gt; = Vec::new(); <span class="c">// (free parameter, key)</span>

    <span class="k">for</span> (&amp;key, vd) <span class="k">in</span> fm.vertex.iter() {
        <span class="k">let</span> (Some(&amp;f), Some(&amp;t)) = (vd.attributes.get(fixed_name), vd.attributes.get(free_name))
        <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> lo = p0[<span class="s">1</span> - fixed].min(p1[<span class="s">1</span> - fixed]);
        <span class="k">let</span> hi = p0[<span class="s">1</span> - fixed].max(p1[<span class="s">1</span> - fixed]);

        <span class="k">if</span> f == at &amp;&amp; t &gt;= lo &amp;&amp; t &lt;= hi {
            on_line.push((t, key));
        }
    }

    <span class="k">if</span> on_line.len() &lt; <span class="s">2</span> {
        <span class="k">return</span> None;
    }

    on_line.sort_by(parameter_order);
    <span class="k">let</span> <span class="k">mut</span> keys = Vec::with_capacity(on_line.len());

    <span class="k">for</span> (_, key) <span class="k">in</span> on_line {
        keys.push(key);
    }

    <span class="k">if</span> e.start_vertex == e.end_vertex {
        keys.push(keys[<span class="s">0</span>]); <span class="c">// closed edge</span>
    }

    Some(keys)
}

<span class="c">/// The mesh vertices one BRep edge runs along.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgeChain {
    <span class="k">pub</span> edge: usize,          <span class="c">// BRep edge index</span>
    <span class="k">pub</span> face: usize,          <span class="c">// face mesh the keys belong to</span>
    <span class="k">pub</span> keys: Vec&lt;usize&gt;,     <span class="c">// vertex keys along the edge</span>
    <span class="k">pub</span> other: Option&lt;usize&gt;, <span class="c">// the face on the other side</span>
}

<span class="c">/// Vertex keys along edge \`edge\` from the mesher's labels.</span>
<span class="k">fn</span> constrained_chain(fm: &amp;Mesh, edge: usize) -&gt; Option&lt;Vec&lt;usize&gt;&gt; {
    <span class="k">let</span> prefix = format!(&quot;<span class="s">brep_edge/</span>{<span class="s">edge</span>}<span class="s">/</span>&quot;);
    <span class="k">let</span> <span class="k">mut</span> uses =
        std::collections::BTreeMap::&lt;usize, std::collections::BTreeMap&lt;usize, usize&gt;&gt;::new(); <span class="c">// use id -&gt; sample -&gt; key</span>

    <span class="k">for</span> (&amp;key, vertex) <span class="k">in</span> &amp;fm.vertex {
        <span class="k">for</span> name <span class="k">in</span> vertex.attributes.keys() {
            <span class="k">let</span> Some(suffix) = name.strip_prefix(&amp;prefix) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> Some((use_id, sample)) = suffix.split_once('<span class="s">/</span>') <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> (Ok(use_id), Ok(sample)) = (use_id.parse::&lt;usize&gt;(), sample.parse::&lt;usize&gt;())
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> value = uses.entry(use_id).or_default().entry(sample).or_insert(key);
            *value = (*value).min(key); <span class="c">// smallest key wins</span>
        }
    }

    <span class="k">for</span> (&amp;use_id, samples) <span class="k">in</span> &amp;uses {
        <span class="k">if</span> samples.len() &lt; <span class="s">2</span> {
            <span class="k">continue</span>;
        }</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> keys = Vec::with_capacity(samples.len());

        <span class="k">for</span> (expected, (&amp;sample, &amp;key)) <span class="k">in</span> samples.iter().enumerate() {
            <span class="k">if</span> expected != sample {
                <span class="k">return</span> None; <span class="c">// a sample is missing</span>
            }

            keys.push(key);
        }

        <span class="c">// vertices inserted between samples carry a 0..1 position</span>
        <span class="k">let</span> interval_prefix = format!(&quot;<span class="s">brep_edge_interval/</span>{<span class="s">edge</span>}<span class="s">/</span>{<span class="s">use_id</span>}<span class="s">/</span>&quot;);
        <span class="k">let</span> <span class="k">mut</span> ordered = Vec::with_capacity(keys.len());

        <span class="k">for</span> (sample, key) <span class="k">in</span> keys.into_iter().enumerate() {
            ordered.push((sample <span class="k">as</span> f64, key));
        }

        <span class="k">for</span> (&amp;key, vertex) <span class="k">in</span> &amp;fm.vertex {
            <span class="k">for</span> name <span class="k">in</span> vertex.attributes.keys() {
                <span class="k">if</span> <span class="k">let</span> Some(sample) = name
                    .strip_prefix(&amp;interval_prefix)
                    .and_then(parse_sample_index)
                    &amp;&amp; <span class="k">let</span> Some(&amp;t) = vertex.attributes.get(name)
                    &amp;&amp; sample + <span class="s">1</span> &lt; samples.len()
                    &amp;&amp; t.is_finite()
                    &amp;&amp; t &gt; <span class="s">0</span>.<span class="s">0</span>
                    &amp;&amp; t &lt; <span class="s">1</span>.<span class="s">0</span>
                {
                    ordered.push((sample <span class="k">as</span> f64 + t, key));
                }
            }
        }

        ordered.sort_by(total_parameter_order);
        ordered.dedup_by(same_parameter);
        <span class="k">let</span> <span class="k">mut</span> keys = Vec::with_capacity(ordered.len());

        <span class="k">for</span> (_, key) <span class="k">in</span> ordered {
            keys.push(key);
        }

        <span class="k">return</span> Some(keys);
    }

    None
}

<span class="c">/// One chain per BRep edge, None when no mesh carries it.</span>
<span class="k">pub</span> <span class="k">fn</span> edge_chains(b: &amp;BRep, fms: &amp;[Mesh]) -&gt; Vec&lt;Option&lt;EdgeChain&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(b.m_edges.len());

    <span class="k">for</span> (ei, e) <span class="k">in</span> b.m_edges.iter().enumerate() {
        <span class="k">if</span> e.degenerated {
            out.push(None);
            <span class="k">continue</span>;
        }

        <span class="k">let</span> uses = b.edge_faces(ei);
        <span class="k">let</span> <span class="k">mut</span> found: Option&lt;EdgeChain&gt; = None;

        <span class="k">for</span> (k, u) <span class="k">in</span> uses.iter().enumerate() {
            <span class="k">let</span> eu = EdgeUse {
                edge: ei,
                face: u.index <span class="k">as</span> usize,
                orientation: u.orientation,
            };
            <span class="k">let</span> keys = <span class="k">match</span> constrained_chain(&amp;fms[eu.face], ei) {
                Some(keys) =&gt; keys,
                None =&gt; <span class="k">match</span> iso_chain(b, &amp;fms[eu.face], &amp;eu) {
                    Some(keys) =&gt; keys,
                    None =&gt; <span class="k">continue</span>,
                },
            };
            <span class="c">// the neighbouring face, if a different one</span>
            <span class="k">let</span> <span class="k">mut</span> other = None;

            <span class="k">for</span> (j, candidate) <span class="k">in</span> uses.iter().enumerate() {
                <span class="k">if</span> j != k &amp;&amp; candidate.index <span class="k">as</span> usize != eu.face {
                    other = Some(candidate.index <span class="k">as</span> usize);
                    <span class="k">break</span>;
                }
            }

            found = Some(EdgeChain {
                edge: ei,
                face: eu.face,
                keys,
                other,
            });
            <span class="k">break</span>;
        }

        out.push(found);
    }

    out
}

<span class="c">/// The average of two vertex normals.</span>
<span class="k">fn</span> mean_normal(a: Option&lt;[f64; 3]&gt;, b: Option&lt;[f64; 3]&gt;) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">let</span> (a, b) = (a?, b?);
    <span class="k">let</span> s = [a[<span class="s">0</span>] + b[<span class="s">0</span>], a[<span class="s">1</span>] + b[<span class="s">1</span>], a[<span class="s">2</span>] + b[<span class="s">2</span>]];
    <span class="k">let</span> l = (s[<span class="s">0</span>] * s[<span class="s">0</span>] + s[<span class="s">1</span>] * s[<span class="s">1</span>] + s[<span class="s">2</span>] * s[<span class="s">2</span>]).sqrt();

    <span class="k">if</span> l &gt; <span class="s">0</span>.<span class="s">0</span> {
        Some([s[<span class="s">0</span>] / l, s[<span class="s">1</span>] / l, s[<span class="s">2</span>] / l])
    } <span class="k">else</span> {
        None
    }
}</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The normal of the nearest face vertex.</span>
<span class="k">fn</span> nearest_normal(fm: &amp;Mesh, p: [f64; <span class="s">3</span>]) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">let</span> <span class="k">mut</span> best: Option&lt;(f64, usize, [f64; 3])&gt; = None;

    <span class="k">for</span> (&amp;key, vd) <span class="k">in</span> fm.vertex.iter() {
        <span class="k">let</span> d = (vd.x - p[<span class="s">0</span>]).powi(<span class="s">2</span>) + (vd.y - p[<span class="s">1</span>]).powi(<span class="s">2</span>) + (vd.z - p[<span class="s">2</span>]).powi(<span class="s">2</span>);
        <span class="k">let</span> closer = <span class="k">match</span> best {
            Some((bd, bk, _)) =&gt; d &lt; bd || (d == bd &amp;&amp; key &lt; bk),
            None =&gt; <span class="s">true</span>,
        };

        <span class="k">if</span> closer &amp;&amp; <span class="k">let</span> Some(n) = vd.normal() {
            best = Some((d, key, n));
        }
    }

    Some(best?.<span class="s">2</span>)
}

<span class="c">/// What every edge pipe of one BRep needs.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgePen&lt;'a&gt; {
    <span class="k">pub</span> fms: &amp;'a [Mesh],                                         <span class="c">// face meshes</span>
    <span class="k">pub</span> signs: &amp;'a [f64],                                        <span class="c">// +1 or -1 per face</span>
    <span class="k">pub</span> pen: Pen,                                                <span class="c">// row, width, colour</span>
}

<span class="c">/// A normal turned outward by its face's sign.</span>
<span class="k">fn</span> scaled_normal(normal: Option&lt;[f64; 3]&gt;, sign: f64) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">let</span> n = normal?;
    Some([n[<span class="s">0</span>] * sign, n[<span class="s">1</span>] * sign, n[<span class="s">2</span>] * sign])
}

<span class="c">/// One pipe per chain segment; returns how many were pushed.</span>
<span class="k">pub</span> <span class="k">fn</span> push_edge_pipes(
    seg: &amp;<span class="k">mut</span> SegRows,
    chain: &amp;EdgeChain,
    ep: &amp;EdgePen,
    bounds: &amp;<span class="k">mut</span> AABB,
) -&gt; usize {
    <span class="k">let</span> fm = &amp;ep.fms[chain.face];

    seg.pipes.reserve(chain.keys.len().saturating_sub(<span class="s">1</span>));
    <span class="k">let</span> <span class="k">mut</span> count = <span class="s">0</span>;

    <span class="k">for</span> w <span class="k">in</span> chain.keys.windows(<span class="s">2</span>) {
        <span class="k">let</span> (a, b) = (&amp;fm.vertex[&amp;w[<span class="s">0</span>]], &amp;fm.vertex[&amp;w[<span class="s">1</span>]]);
        <span class="k">let</span> p0 = [a.x, a.y, a.z];
        <span class="k">let</span> p1 = [b.x, b.y, b.z];
        <span class="k">let</span> n0 = scaled_normal(mean_normal(a.normal(), b.normal()), ep.signs[chain.face]);
        <span class="k">let</span> mid = [
            (p0[<span class="s">0</span>] + p1[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>,
            (p0[<span class="s">1</span>] + p1[<span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
            (p0[<span class="s">2</span>] + p1[<span class="s">2</span>]) * <span class="s">0</span>.<span class="s">5</span>,
        ];
        <span class="k">let</span> n1 = <span class="k">match</span> chain.other {
            Some(other) =&gt; scaled_normal(nearest_normal(&amp;ep.fms[other], mid), ep.signs[other]),
            None =&gt; n0,
        };
        <span class="k">let</span> p0f = super::curves::render_position(p0);
        <span class="k">let</span> p1f = super::curves::render_position(p1);

        <span class="c">// skip a segment that collapses in f32</span>
        <span class="k">if</span> p0f == p1f || !p0f.into_iter().chain(p1f).all(f32::is_finite) {
            <span class="k">continue</span>;
        }

        bounds.union_with_point(p0f[<span class="s">0</span>] <span class="k">as</span> f64, p0f[<span class="s">1</span>] <span class="k">as</span> f64, p0f[<span class="s">2</span>] <span class="k">as</span> f64);
        bounds.union_with_point(p1f[<span class="s">0</span>] <span class="k">as</span> f64, p1f[<span class="s">1</span>] <span class="k">as</span> f64, p1f[<span class="s">2</span>] <span class="k">as</span> f64);
        seg.pipes.push(CylinderSegment {
            p0: p0f,
            radius: ep.pen.radius,
            p1: p1f,
            instance_id: ep.pen.row,
            color: ep.pen.color,
            facing: pack_facing(n0.as_ref(), n1.as_ref()),
        });</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        seg.pipe_ids
            .push(u32::try_from(chain.edge).unwrap_or(u32::MAX)); <span class="c">// edge id for picking</span>
        count += <span class="s">1</span>;
    }

    count
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::brep::QUALITY;

    <span class="c">/// Labelled samples and inserted nodes form one ordered chain.</span>
    #[test]
    <span class="k">fn</span> constrained_chain_keeps_inserted_crease_nodes_and_source_identity() {
        <span class="k">let</span> <span class="k">mut</span> mesh = Mesh::new();

        <span class="k">for</span> (key, name, value) <span class="k">in</span> [
            (<span class="s">10</span>, &quot;<span class="s">brep_edge/7/2/0</span>&quot;, <span class="s">1</span>.<span class="s">0</span>),
            (<span class="s">20</span>, &quot;<span class="s">brep_edge_interval/7/2/0</span>&quot;, <span class="s">0</span>.<span class="s">25</span>),
            (<span class="s">21</span>, &quot;<span class="s">brep_edge_interval/7/2/0</span>&quot;, <span class="s">0</span>.<span class="s">25</span>),
            (<span class="s">30</span>, &quot;<span class="s">brep_edge/7/2/1</span>&quot;, <span class="s">1</span>.<span class="s">0</span>),
        ] {
            mesh.add_vertex(session_rust::Point::new(key <span class="k">as</span> f64, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Some(key));
            mesh.vertex</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                .get_mut(&amp;key)
                .unwrap()
                .attributes
                .insert(name.to_string(), value);
        }

        assert_eq!(constrained_chain(&amp;mesh, <span class="s">7</span>), Some(vec![<span class="s">10</span>, <span class="s">20</span>, <span class="s">30</span>]));
        assert_eq!(constrained_chain(&amp;mesh, <span class="s">8</span>), None);
        mesh.vertex
            .get_mut(&amp;<span class="s">30</span>)
            .unwrap()
            .attributes
            .remove(&quot;<span class="s">brep_edge/7/2/1</span>&quot;);
        assert_eq!(constrained_chain(&amp;mesh, <span class="s">7</span>), None);
    }

    <span class="c">/// First use of every edge.</span>
    <span class="k">fn</span> first_uses(b: &amp;BRep) -&gt; Vec&lt;EdgeUse&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">for</span> ei <span class="k">in</span> <span class="s">0</span>..b.m_edges.len() {
            <span class="k">let</span> uses = b.edge_faces(ei);
            <span class="k">let</span> Some(u) = uses.first() <span class="k">else</span> { <span class="k">continue</span> };
            out.push(EdgeUse {
                edge: ei,
                face: u.index <span class="k">as</span> usize,
                orientation: u.orientation,
            });
        }

        out
    }

    <span class="c">/// The chain starts exactly on BRep vertex \`vi\`.</span>
    <span class="k">fn</span> ends_on(b: &amp;BRep, fm: &amp;Mesh, keys: &amp;[usize], vi: usize) -&gt; bool {
        <span class="k">let</span> p = fm.vertex[&amp;keys[<span class="s">0</span>]].position();
        <span class="k">let</span> v = &amp;b.m_vertices[vi].point;
        p[<span class="s">0</span>] == v[<span class="s">0</span>] &amp;&amp; p[<span class="s">1</span>] == v[<span class="s">1</span>] &amp;&amp; p[<span class="s">2</span>] == v[<span class="s">2</span>]
    }

    <span class="c">/// A cylinder: two closed circles and a two-key seam.</span>
    #[test]
    <span class="k">fn</span> cylinder_chains_follow_the_grid() {
        <span class="k">let</span> b = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> uses = first_uses(&amp;b);
        <span class="k">let</span> chains: Vec&lt;Vec&lt;usize&gt;&gt; = uses
            .iter()
            .map(|u| iso_chain(&amp;b, &amp;fms[u.face], u).expect(&quot;<span class="s">grid use</span>&quot;))
            .collect();
        assert_eq!(chains.len(), <span class="s">3</span>);

        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
            assert_eq!(chains[k].first(), chains[k].last());
            assert!(chains[k].len() &gt; <span class="s">4</span>);
            assert!(ends_on(
                &amp;b,
                &amp;fms[uses[k].face],
                &amp;chains[k],
                b.m_edges[k].start_vertex <span class="k">as</span> usize
            ));
        }

        assert_eq!(chains[<span class="s">0</span>].len(), chains[<span class="s">1</span>].len());
        assert_eq!(chains[<span class="s">2</span>].len(), <span class="s">2</span>);
        assert!(ends_on(&amp;b, &amp;fms[uses[<span class="s">2</span>].face], &amp;chains[<span class="s">2</span>], <span class="s">0</span>));
        <span class="k">let</span> top = fms[uses[<span class="s">2</span>].face].vertex[chains[<span class="s">2</span>].last().unwrap()].position();
        assert_eq!(
            [top[<span class="s">0</span>], top[<span class="s">1</span>], top[<span class="s">2</span>]],
            [
                b.m_vertices[<span class="s">1</span>].point[<span class="s">0</span>],
                b.m_vertices[<span class="s">1</span>].point[<span class="s">1</span>],
                b.m_vertices[<span class="s">1</span>].point[<span class="s">2</span>]
            ]
        );
    }</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/07/src/app/walk/brep_edges.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A sphere's seam runs from pole to pole.</span>
    #[test]
    <span class="k">fn</span> sphere_seam_reaches_both_poles() {
        <span class="k">let</span> b = BRep::create_sphere(<span class="s">180</span>.<span class="s">0</span>);
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> uses = first_uses(&amp;b);
        <span class="k">let</span> seam = iso_chain(&amp;b, &amp;fms[<span class="s">0</span>], &amp;uses[<span class="s">0</span>]).expect(&quot;<span class="s">seam</span>&quot;);
        assert!(seam.len() &gt; <span class="s">10</span>);
        assert_ne!(seam.first(), seam.last());
        assert!(ends_on(&amp;b, &amp;fms[<span class="s">0</span>], &amp;seam, <span class="s">0</span>));
        <span class="k">let</span> north = fms[<span class="s">0</span>].vertex[seam.last().unwrap()].position();
        assert_eq!(north[<span class="s">2</span>], b.m_vertices[<span class="s">1</span>].point[<span class="s">2</span>]);
        assert!(iso_chain(&amp;b, &amp;fms[<span class="s">0</span>], &amp;uses[<span class="s">1</span>]).is_none());
        assert!(iso_chain(&amp;b, &amp;fms[<span class="s">0</span>], &amp;uses[<span class="s">2</span>]).is_none());
    }

    <span class="c">/// A torus has two closed seams; a cap has no iso chain.</span>
    #[test]
    <span class="k">fn</span> torus_seams_close_and_cdt_faces_decline() {
        <span class="k">let</span> b = BRep::create_torus(<span class="s">220</span>.<span class="s">0</span>, <span class="s">70</span>.<span class="s">0</span>);
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));

        <span class="k">for</span> u <span class="k">in</span> first_uses(&amp;b) {
            <span class="k">let</span> c = iso_chain(&amp;b, &amp;fms[u.face], &amp;u).expect(&quot;<span class="s">seam</span>&quot;);
            assert_eq!(c.first(), c.last());
            assert!(ends_on(&amp;b, &amp;fms[u.face], &amp;c, <span class="s">0</span>));
        }

        <span class="k">let</span> cyl = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> cfm = cyl.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> cap = cyl.edge_faces(<span class="s">0</span>)[<span class="s">1</span>];
        <span class="k">let</span> eu = EdgeUse {
            edge: <span class="s">0</span>,
            face: cap.index <span class="k">as</span> usize,
            orientation: cap.orientation,
        };
        assert!(iso_chain(&amp;cyl, &amp;cfm[eu.face], &amp;eu).is_none());
    }

    <span class="c">/// Every real edge of the mixed scene gets a chain.</span>
    #[test]
    <span class="k">fn</span> every_solid_edge_has_a_chain() {
        <span class="k">let</span> solids = [
            BRep::create_box(<span class="s">400</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">250</span>.<span class="s">0</span>),
            BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_cone(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_sphere(<span class="s">180</span>.<span class="s">0</span>),
            BRep::create_torus(<span class="s">220</span>.<span class="s">0</span>, <span class="s">70</span>.<span class="s">0</span>),
            BRep::create_block_with_hole(<span class="s">500</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>),
            BRep::create_pyramid(<span class="s">400</span>.<span class="s">0</span>, <span class="s">350</span>.<span class="s">0</span>),
        ];

        <span class="k">for</span> b <span class="k">in</span> &amp;solids {
            <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
            <span class="k">let</span> chains = edge_chains(b, &amp;fms);
            assert_eq!(chains.len(), b.m_edges.len());

            <span class="k">for</span> (ei, e) <span class="k">in</span> b.m_edges.iter().enumerate() {
                <span class="k">let</span> uses = b.edge_faces(ei);

                <span class="k">match</span> &amp;chains[ei] {
                    None =&gt; assert!(e.degenerated, &quot;{}<span class="s"> edge </span>{<span class="s">ei</span>}&quot;, b.name),
                    Some(c) =&gt; {
                        assert!(
                            uses.iter().any(|u| u.index <span class="k">as</span> usize == c.face),
                            &quot;{}<span class="s"> edge </span>{<span class="s">ei</span>}&quot;,
                            b.name
                        );
                        <span class="k">let</span> other = uses
                            .iter()
                            .find(|u| u.index <span class="k">as</span> usize != c.face)
                            .map(|u| u.index <span class="k">as</span> usize);
                        assert_eq!(c.other, other, &quot;{}<span class="s"> edge </span>{<span class="s">ei</span>}&quot;, b.name);
                    }
                }
            }
        }
    }

    <span class="c">/// Cylinder pipes carry both neighbouring face normals.</span>
    #[test]
    <span class="k">fn</span> pipes_face_both_adjacent_faces() {
        <span class="k">use</span> <span class="k">crate</span>::app::walk::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
        <span class="k">use</span> session_rust::AABB;
        <span class="k">let</span> b = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> chains = edge_chains(&amp;b, &amp;fms);
        <span class="k">let</span> signs = vec![<span class="s">1</span>.<span class="s">0</span>; fms.len()];
        <span class="k">let</span> ep = EdgePen {
            fms: &amp;fms,
            signs: &amp;signs,
            pen: Pen {
                row: <span class="s">3</span>,
                radius: encode_width(<span class="s">1</span>.<span class="s">0</span>),
                color: pack_rgba([<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
            },
        };
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
        <span class="k">let</span> circle = push_edge_pipes(&amp;<span class="k">mut</span> seg, chains[<span class="s">0</span>].as_ref().unwrap(), &amp;ep, &amp;<span class="k">mut</span> bounds);
        assert_eq!(circle, chains[<span class="s">0</span>].as_ref().unwrap().keys.len() - <span class="s">1</span>);

        <span class="k">for</span> p <span class="k">in</span> &amp;seg.pipes {
            assert_ne!(p.facing, FACING_UNKNOWN);
            assert_ne!(p.facing &amp; <span class="s">0xffff</span>, p.facing &gt;&gt; <span class="s">16</span>);
            assert_eq!(p.instance_id, <span class="s">3</span>);
        }

        <span class="k">let</span> before = seg.pipes.len();
        <span class="k">let</span> seam = push_edge_pipes(&amp;<span class="k">mut</span> seg, chains[<span class="s">2</span>].as_ref().unwrap(), &amp;ep, &amp;<span class="k">mut</span> bounds);
        assert_eq!(seam, <span class="s">1</span>);
        <span class="k">let</span> p = &amp;seg.pipes[before];
        assert_eq!(p.facing &amp; <span class="s">0xffff</span>, p.facing &gt;&gt; <span class="s">16</span>);
        assert!(seg.ribbons.is_empty());
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        assert!(seg.pipe_ids[..before].iter().all(|&amp;id| id == <span class="s">0</span>));
        assert_eq!(seg.pipe_ids[before], <span class="s">2</span>);
    }

    <span class="c">/// A segment that collapses in f32 pushes no pipe.</span>
    #[test]
    <span class="k">fn</span> collapsed_display_segments_do_not_create_pick_targets() {
        <span class="k">let</span> b = BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> chains = edge_chains(&amp;b, &amp;fms);
        <span class="k">let</span> chain = chains[<span class="s">0</span>].as_ref().unwrap();

        <span class="k">for</span> key <span class="k">in</span> &amp;chain.keys {
            <span class="k">let</span> vertex = fms[chain.face].vertex.get_mut(key).unwrap();
            vertex.x = <span class="s">1</span>e<span class="s">20</span>;
            vertex.y = <span class="s">1</span>e<span class="s">20</span>;
            vertex.z = <span class="s">1</span>e<span class="s">20</span>;
        }

        <span class="k">let</span> signs = vec![<span class="s">1</span>.<span class="s">0</span>; fms.len()];
        <span class="k">let</span> ep = EdgePen {
            fms: &amp;fms,
            signs: &amp;signs,
            pen: Pen {
                row: <span class="s">0</span>,
                radius: <span class="s">1</span>.<span class="s">0</span>,
                color: <span class="s">0</span>,
            },
        };
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        assert_eq!(push_edge_pipes(&amp;<span class="k">mut</span> seg, chain, &amp;ep, &amp;<span class="k">mut</span> AABB::empty()), <span class="s">0</span>);
        assert!(seg.pipes.is_empty());
        assert!(seg.pipe_ids.is_empty());
    }

    <span class="c">/// Pipe ids stay the BRep edge index after remeshing.</span>
    #[test]
    <span class="k">fn</span> remeshing_keeps_source_edge_identity() {
        <span class="k">let</span> b = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> coarse = b.face_meshes_q(Some((<span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">005</span>)));
        <span class="k">let</span> fine = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> coarse = edge_chains(&amp;b, &amp;coarse);
        <span class="k">let</span> fine = edge_chains(&amp;b, &amp;fine);

        <span class="k">for</span> (edge, (coarse, fine)) <span class="k">in</span> coarse.iter().zip(&amp;fine).enumerate() {
            assert_eq!(coarse.as_ref().unwrap().edge, edge);
            assert_eq!(fine.as_ref().unwrap().edge, edge);
        }

        assert!(fine[<span class="s">0</span>].as_ref().unwrap().keys.len() &gt; coarse[<span class="s">0</span>].as_ref().unwrap().keys.len());
    }

    <span class="c">/// A hole rim meshed by CDT still gets its edge chains.</span>
    #[test]
    <span class="k">fn</span> cdt_only_hole_boundaries_keep_real_edges() {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::create_block_with_hole(<span class="s">500</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>);
        b.m_faces = vec![b.m_faces[<span class="s">5</span>].clone()];
        b.m_shells.clear();
        b.m_solids.clear();
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        assert!(!fms[<span class="s">0</span>].face.is_empty());
        <span class="k">let</span> chains = edge_chains(&amp;b, &amp;fms);
        <span class="k">let</span> <span class="k">mut</span> attached = <span class="s">0</span>;

        <span class="k">for</span> (edge, chain) <span class="k">in</span> chains.iter().enumerate() {
            <span class="k">if</span> b.edge_faces(edge).is_empty() {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> chain = chain.as_ref().expect(&quot;<span class="s">trim constraint keeps its CAD edge</span>&quot;);
            assert_eq!(chain.edge, edge);
            assert_eq!(chain.face, <span class="s">0</span>);
            attached += <span class="s">1</span>;
        }

        assert_eq!(attached, <span class="s">5</span>);
    }

    <span class="c">/// A curved trim edge runs on the face's own vertices.</span>
    #[test]
    <span class="k">fn</span> curved_trim_edges_use_exact_face_samples() {
        <span class="k">use</span> session_rust::brep::BRepRef;
        <span class="k">use</span> session_rust::{NurbsCurve, Point, Primitives};
        <span class="k">let</span> surface = Primitives::wave_surface(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">5</span>);
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        <span class="k">let</span> surface_index = b.add_surface(&amp;surface);
        <span class="k">let</span> corners = [[<span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">1</span>], [<span class="s">0</span>.<span class="s">9</span>, <span class="s">0</span>.<span class="s">1</span>], [<span class="s">0</span>.<span class="s">9</span>, <span class="s">0</span>.<span class="s">9</span>], [<span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">9</span>]];

        <span class="k">for</span> uv <span class="k">in</span> corners {
            b.add_vertex(&amp;surface.point_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>]).unwrap(), <span class="s">0</span>.<span class="s">0</span>);
        }

        <span class="k">let</span> <span class="k">mut</span> edges = Vec::new();

        <span class="k">for</span> side <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            <span class="k">let</span> a = corners[side];
            <span class="k">let</span> z = corners[(side + <span class="s">1</span>) % <span class="s">4</span>];
            <span class="k">let</span> direction = <span class="k">if</span> a[<span class="s">0</span>] != z[<span class="s">0</span>] { <span class="s">0</span> } <span class="k">else</span> { <span class="s">1</span> };
            <span class="k">let</span> <span class="k">mut</span> curve = surface.iso_curve(direction, a[<span class="s">1</span> - direction]).unwrap();
            assert!(curve.trim(<span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">9</span>));

            <span class="k">if</span> a[direction] &gt; z[direction] {
                curve.reverse();
            }

            <span class="k">let</span> curve_index = b.add_curve_3d(&amp;curve);
            <span class="k">let</span> edge = b.add_edge(curve_index <span class="k">as</span> i32, side <span class="k">as</span> i32, ((side + <span class="s">1</span>) % <span class="s">4</span>) <span class="k">as</span> i32);
            <span class="k">let</span> pcurve = NurbsCurve::create(
                <span class="s">false</span>,
                <span class="s">1</span>,
                &amp;[Point::new(a[<span class="s">0</span>], a[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>), Point::new(z[<span class="s">0</span>], z[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>)],
            );
            <span class="k">let</span> pcurve_index = b.add_curve_2d(&amp;pcurve);
            b.add_pcurve(edge, surface_index, pcurve_index <span class="k">as</span> i32, -<span class="s">1</span>);
            edges.push(BRepRef::new(edge <span class="k">as</span> i32, BRepOrientation::Forward));
        }

        <span class="k">let</span> wire = b.add_wire(&amp;edges);
        b.add_face(
            surface_index <span class="k">as</span> i32,
            &amp;[BRepRef::new(wire <span class="k">as</span> i32, BRepOrientation::Forward)],
            <span class="s">0</span>.<span class="s">0</span>,
        );
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        assert!(!fms[<span class="s">0</span>].face.is_empty());
        <span class="k">let</span> chains = edge_chains(&amp;b, &amp;fms);

        <span class="k">for</span> (edge, chain) <span class="k">in</span> chains.iter().enumerate() {
            <span class="k">let</span> chain = chain.as_ref().expect(&quot;<span class="s">curved trimmed face edge</span>&quot;);
            assert_eq!(chain.edge, edge);
            assert!(chain.keys.len() &gt; <span class="s">4</span>);

            <span class="k">for</span> &amp;key <span class="k">in</span> &amp;chain.keys {
                <span class="k">let</span> vertex = &amp;fms[<span class="s">0</span>].vertex[&amp;key];
                <span class="k">let</span> u = *vertex.attributes.get(&quot;<span class="s">u</span>&quot;).unwrap();
                <span class="k">let</span> v = *vertex.attributes.get(&quot;<span class="s">v</span>&quot;).unwrap();
                <span class="k">let</span> p = surface.point_at(u, v).unwrap();
                assert_eq!([vertex.x, vertex.y, vertex.z], [p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]);
                <span class="k">let</span> n = vertex.normal().unwrap();
                assert!((n[<span class="s">0</span>] * n[<span class="s">0</span>] + n[<span class="s">1</span>] * n[<span class="s">1</span>] + n[<span class="s">2</span>] * n[<span class="s">2</span>] - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">10</span>);
            }
        }
    }
}</code></pre></div>
<h2 id="step-4-srcappwalkbrep_orientrs">Step 4 · src/app/walk/brep_orient.rs<a class="anchor" href="#/course/07-boundaries#step-4-srcappwalkbrep_orientrs" aria-label="Link to this section">#</a></h2>
<p>New file: give each face of a solid a sign, +1 or -1, so every normal points outward.</p>
<p><code>lessons/07/src/app/walk/brep_orient.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::brep_edges::EdgeChain;
<span class="k">use</span> session_rust::{BRep, Mesh};

<span class="c">/// Which way this face walks the edge s -&gt; n: Some(true) forwards, Some(false) backwards, None if unclear.</span>
<span class="k">fn</span> walks(fm: &amp;Mesh, s: usize, n: usize) -&gt; Option&lt;bool&gt; {
    <span class="k">let</span> fwd = occupied_halfedge(fm, s, n);
    <span class="k">let</span> back = occupied_halfedge(fm, n, s);

    <span class="k">match</span> (fwd, back) {
        (<span class="s">true</span>, <span class="s">false</span>) =&gt; Some(<span class="s">true</span>),
        (<span class="s">false</span>, <span class="s">true</span>) =&gt; Some(<span class="s">false</span>),
        _ =&gt; None,
    }
}

<span class="c">/// A halfedge is one direction of an edge; it is occupied when some face walks from -&gt; to.</span>
<span class="k">fn</span> occupied_halfedge(fm: &amp;Mesh, from: usize, to: usize) -&gt; bool {
    <span class="k">let</span> Some(neighbours) = fm.halfedge.get(&amp;from) <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    matches!(neighbours.get(&amp;to), Some(Some(_)))
}

<span class="k">fn</span> at(fm: &amp;Mesh, k: usize) -&gt; [f64; <span class="s">3</span>] {
    <span class="k">let</span> v = &amp;fm.vertex[&amp;k];
    [v.x, v.y, v.z]
}

<span class="c">/// Each face is meshed on its own, so the other face's copy of an edge is found by direction, not by key.</span>
<span class="k">fn</span> neighbour_along(fm: &amp;Mesh, s: usize, dir: [f64; <span class="s">3</span>]) -&gt; Option&lt;usize&gt; {
    <span class="k">let</span> p = at(fm, s);
    <span class="k">let</span> <span class="k">mut</span> best: Option&lt;(f64, usize)&gt; = None;

    <span class="k">for</span> &amp;w <span class="k">in</span> fm.halfedge.get(&amp;s)?.keys() {
        <span class="k">let</span> q = at(fm, w);
        <span class="k">let</span> d = [q[<span class="s">0</span>] - p[<span class="s">0</span>], q[<span class="s">1</span>] - p[<span class="s">1</span>], q[<span class="s">2</span>] - p[<span class="s">2</span>]];
        <span class="k">let</span> l = (d[<span class="s">0</span>] * d[<span class="s">0</span>] + d[<span class="s">1</span>] * d[<span class="s">1</span>] + d[<span class="s">2</span>] * d[<span class="s">2</span>]).sqrt();

        <span class="k">if</span> l == <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> c = (d[<span class="s">0</span>] * dir[<span class="s">0</span>] + d[<span class="s">1</span>] * dir[<span class="s">1</span>] + d[<span class="s">2</span>] * dir[<span class="s">2</span>]) / l; <span class="c">// cosine to \`dir\`</span>

        <span class="c">// a match as the if condition: true when this neighbour beats the best so far</span>
        <span class="k">if</span> <span class="k">match</span> best {
            Some((bc, bw)) =&gt; c &gt; bc || (c == bc &amp;&amp; w &lt; bw), <span class="c">// a tie goes to the smaller key, whatever the HashMap order</span>
            None =&gt; <span class="s">true</span>,
        } {
            best = Some((c, w));
        }
    }

    Some(best?.<span class="s">1</span>)
}</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_orient.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Nearest vertex; a tie goes to the smaller key.</span>
<span class="k">fn</span> vertex_at(fm: &amp;Mesh, p: [f64; <span class="s">3</span>]) -&gt; Option&lt;usize&gt; {
    <span class="k">let</span> <span class="k">mut</span> best: Option&lt;(f64, usize)&gt; = None;

    <span class="k">for</span> (&amp;k, v) <span class="k">in</span> fm.vertex.iter() {
        <span class="k">let</span> d = (v.x - p[<span class="s">0</span>]).powi(<span class="s">2</span>) + (v.y - p[<span class="s">1</span>]).powi(<span class="s">2</span>) + (v.z - p[<span class="s">2</span>]).powi(<span class="s">2</span>);

        <span class="k">if</span> <span class="k">match</span> best {
            Some((bd, bk)) =&gt; d &lt; bd || (d == bd &amp;&amp; k &lt; bk),
            None =&gt; <span class="s">true</span>,
        } {
            best = Some((d, k));
        }
    }

    Some(best?.<span class="s">1</span>)
}

<span class="c">/// Neighbouring faces agree on which side is outside when they walk their shared edge in opposite directions.</span>
<span class="k">fn</span> opposed(fms: &amp;[Mesh], c: &amp;EdgeChain) -&gt; Option&lt;bool&gt; {
    <span class="k">let</span> other = c.other?;
    <span class="k">let</span> (fa, fb) = (&amp;fms[c.face], &amp;fms[other]);
    <span class="k">let</span> (s, n) = (c.keys[<span class="s">0</span>], c.keys[<span class="s">1</span>]);</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_orient.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> away_a = walks(fa, s, n)?; <span class="c">// the owner face's way along s -&gt; n</span>
    <span class="k">let</span> (ps, pn) = (at(fa, s), at(fa, n));
    <span class="k">let</span> dir = [pn[<span class="s">0</span>] - ps[<span class="s">0</span>], pn[<span class="s">1</span>] - ps[<span class="s">1</span>], pn[<span class="s">2</span>] - ps[<span class="s">2</span>]];
    <span class="k">let</span> sb = vertex_at(fb, ps)?; <span class="c">// same start on the other face</span>
    <span class="k">let</span> nb = neighbour_along(fb, sb, dir)?; <span class="c">// same next point there</span>
    <span class="k">let</span> away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

<span class="c">/// a · (b × c) per triangle: six times the signed volume of the cone from the origin to the face.</span>
<span class="k">fn</span> six_volume(fm: &amp;Mesh) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> keys: Vec&lt;usize&gt; = fm.face.keys().copied().collect();
    keys.sort_unstable(); <span class="c">// the same sum, bit for bit, on every run</span>
    <span class="k">let</span> <span class="k">mut</span> v = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> k <span class="k">in</span> keys {
        <span class="k">let</span> verts = &amp;fm.face[&amp;k];

        <span class="k">if</span> verts.len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> (a, b, c) = (at(fm, verts[<span class="s">0</span>]), at(fm, verts[<span class="s">1</span>]), at(fm, verts[<span class="s">2</span>]));
        v += a[<span class="s">0</span>] * (b[<span class="s">1</span>] * c[<span class="s">2</span>] - b[<span class="s">2</span>] * c[<span class="s">1</span>]) - a[<span class="s">1</span>] * (b[<span class="s">0</span>] * c[<span class="s">2</span>] - b[<span class="s">2</span>] * c[<span class="s">0</span>])
            + a[<span class="s">2</span>] * (b[<span class="s">0</span>] * c[<span class="s">1</span>] - b[<span class="s">1</span>] * c[<span class="s">0</span>]);
    }

    v
}

<span class="c">/// +1 or -1 per face so every normal points outward.</span>
<span class="k">pub</span> <span class="k">fn</span> face_signs(b: &amp;BRep, fms: &amp;[Mesh], chains: &amp;[Option&lt;EdgeChain&gt;]) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> nf = fms.len();

    <span class="c">// an open shell keeps its authored normals</span>
    <span class="k">if</span> !b.is_solid() {
        <span class="k">return</span> vec![<span class="s">1</span>.<span class="s">0</span>; nf];
    }

    <span class="k">let</span> <span class="k">mut</span> adjacent: Vec&lt;Vec&lt;(usize, bool)&gt;&gt; = vec![Vec::new(); nf]; <span class="c">// per face: (neighbour, opposed)</span>

    <span class="k">for</span> c <span class="k">in</span> chains.iter().flatten() {
        <span class="k">if</span> <span class="k">let</span> (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));</code></pre></div>
<p><code>lessons/07/src/app/walk/brep_orient.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        }
    }

    <span class="k">let</span> <span class="k">mut</span> sign = vec![<span class="s">0</span>.<span class="s">0f64</span>; nf]; <span class="c">// 0 = not visited</span>

    <span class="k">for</span> start <span class="k">in</span> <span class="s">0</span>..nf {
        <span class="k">if</span> sign[start] != <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        sign[start] = <span class="s">1</span>.<span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> group = vec![start]; <span class="c">// faces reached from start; also the queue, read from \`head\`</span>
        <span class="k">let</span> <span class="k">mut</span> head = <span class="s">0</span>;

        <span class="c">// breadth first: a neighbour takes our sign if the pair agrees, else the opposite</span>
        <span class="k">while</span> head &lt; group.len() {
            <span class="k">let</span> f = group[head];
            head += <span class="s">1</span>;

            <span class="k">for</span> &amp;(g, opp) <span class="k">in</span> &amp;adjacent[f] {
                <span class="k">if</span> sign[g] == <span class="s">0</span>.<span class="s">0</span> {
                    sign[g] = <span class="k">if</span> opp { sign[f] } <span class="k">else</span> { -sign[f] };
                    group.push(g);
                }
            }
        }

        <span class="c">// negative volume: the whole group is inside out</span>
        <span class="k">let</span> <span class="k">mut</span> volume = <span class="s">0</span>.<span class="s">0</span>;

        <span class="k">for</span> &amp;face <span class="k">in</span> &amp;group {
            volume += sign[face] * six_volume(&amp;fms[face]);
        }

        <span class="k">if</span> volume &lt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">for</span> &amp;f <span class="k">in</span> &amp;group {
                sign[f] = -sign[f];
            }
        }
    }

    sign
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::brep::QUALITY;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::brep_edges::edge_chains;
    <span class="k">use</span> session_rust::brep::brep_reverse;

    <span class="c">/// Flip sign of every face of \`b\`.</span>
    <span class="k">fn</span> signs_of(b: &amp;BRep) -&gt; Vec&lt;f64&gt; {
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> chains = edge_chains(b, &amp;fms);
        face_signs(b, &amp;fms, &amp;chains)
    }

    <span class="c">/// Reverse the first two face uses.</span>
    <span class="k">fn</span> flipped(<span class="k">mut</span> b: BRep) -&gt; BRep {
        <span class="k">for</span> face <span class="k">in</span> b.m_shells[<span class="s">0</span>].faces.iter_mut().take(<span class="s">2</span>) {
            face.orientation = brep_reverse(face.orientation);
        }

        b
    }

    <span class="c">/// Kernel solids need no face flipped.</span>
    #[test]
    <span class="k">fn</span> kernel_solids_keep_their_normals() {
        <span class="k">for</span> b <span class="k">in</span> [
            BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_cone(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_box(<span class="s">400</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">250</span>.<span class="s">0</span>),
            BRep::create_block_with_hole(<span class="s">500</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>),
        ] {
            assert!(signs_of(&amp;b).iter().all(|&amp;s| s == <span class="s">1</span>.<span class="s">0</span>), &quot;{}&quot;, b.name);
        }
    }

    <span class="c">/// An open shell has no face flipped.</span>
    #[test]
    <span class="k">fn</span> open_shell_preserves_authored_orientation() {
        <span class="k">let</span> <span class="k">mut</span> b = flipped(BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>));</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/07/src/app/walk/brep_orient.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        b.m_solids.clear();
        assert!(signs_of(&amp;b).iter().all(|&amp;sign| sign == <span class="s">1</span>.<span class="s">0</span>));
    }

    <span class="c">/// A flipped face keeps the same outward normal.</span>
    #[test]
    <span class="k">fn</span> flipped_uses_change_no_outward_normal() {
        <span class="k">let</span> ok = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> fl = flipped(BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>));
        <span class="k">let</span> (fms_ok, fms_fl) = (
            ok.face_meshes_q(Some(QUALITY)),
            fl.face_meshes_q(Some(QUALITY)),
        );
        <span class="k">let</span> signs = face_signs(&amp;fl, &amp;fms_fl, &amp;edge_chains(&amp;fl, &amp;fms_fl));
        assert_eq!(signs[<span class="s">0</span>], -<span class="s">1</span>.<span class="s">0</span>);
        assert_eq!(signs[<span class="s">1</span>], -<span class="s">1</span>.<span class="s">0</span>);
        assert_eq!(signs[<span class="s">2</span>], <span class="s">1</span>.<span class="s">0</span>);

        <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">for</span> (key, vd) <span class="k">in</span> fms_ok[fi].vertex.iter() {
                <span class="k">let</span> n_ok = vd.normal().unwrap();
                <span class="k">let</span> n_fl = fms_fl[fi].vertex[key].normal().unwrap();
                assert_eq!(
                    [
                        n_fl[<span class="s">0</span>] * signs[fi],
                        n_fl[<span class="s">1</span>] * signs[fi],
                        n_fl[<span class="s">2</span>] * signs[fi]
                    ],
                    n_ok
                );
            }
        }
    }

    <span class="c">/// A flipped cylinder draws the same pipes.</span>
    #[test]
    <span class="k">fn</span> flipped_uses_change_no_pipe() {
        <span class="k">use</span> <span class="k">crate</span>::app::walk::WalkCx;
        <span class="k">use</span> <span class="k">crate</span>::app::walk::brep::walk_brep;
        <span class="k">use</span> <span class="k">crate</span>::app::walk::mesh_ink::Ink;
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
        <span class="k">let</span> <span class="k">mut</span> pipes = Vec::new();

        <span class="k">for</span> b <span class="k">in</span> [
            BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            flipped(BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>)),
        ] {
            <span class="k">let</span> <span class="k">mut</span> arena = ArenaRows::default();
            <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
            <span class="k">let</span> <span class="k">mut</span> glyph = GlyphRows::default();
            <span class="k">let</span> <span class="k">mut</span> ink = Ink {
                seg: &amp;<span class="k">mut</span> seg,
                glyph: &amp;<span class="k">mut</span> glyph,
            };
            walk_brep(
                &amp;<span class="k">mut</span> arena,
                &amp;<span class="k">mut</span> ink,
                &amp;b,
                &amp;WalkCx {
                    vert_base: <span class="s">0</span>,
                    cloud_px: <span class="s">0</span>.<span class="s">0</span>,
                    row: <span class="s">0</span>,
                },
            );
            pipes.push(
                seg.pipes
                    .iter()
                    .map(|p| (p.p0, p.p1, p.facing))
                    .collect::&lt;Vec&lt;_&gt;&gt;(),
            );
        }

        assert_eq!(pipes[<span class="s">0</span>], pipes[<span class="s">1</span>]);
    }
}</code></pre></div>
<h2 id="step-5-srcappwalkmodrs">Step 5 · src/app/walk/mod.rs<a class="anchor" href="#/course/07-boundaries#step-5-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>The walk passes orientation to the BRep walk.</p>
<p><code>lessons/07/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> brep_edges;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> brep_orient;</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/07/</code>.</p>
<h2 id="step-6-srcappwalkbreprs">Step 6 · src/app/walk/brep.rs<a class="anchor" href="#/course/07-boundaries#step-6-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>The BRep walk draws each shared edge once.</p>
<p><code>lessons/07/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces the line <code>use session_rust::{BRep, NurbsSurface, RenderMesh};</code> in <code>lessons/06/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::brep_edges::{EdgeChain, EdgePen, edge_chains, push_edge_pipes};
<span class="k">use</span> super::brep_orient::face_signs;
<span class="k">use</span> super::curves::{push_polyline, sample_nurbscurve};
<span class="k">use</span> super::encode::{Pen, encode_width, pack_rgba};
<span class="k">use</span> super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_mesh};
<span class="k">use</span> super::mesh_ink::Ink;
<span class="k">use</span> super::{Row, WalkCx};
<span class="k">use</span> <span class="k">crate</span>::app::knobs;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::Instance;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
<span class="k">use</span> session_rust::{BRep, Color, NurbsSurface, RenderMesh};</code></pre></div>
<p>Replaces the <code>fn walk_brep</code> lines in <code>lessons/06/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A BRep: every face meshed and uploaded, then its edges.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_brep(arena: &amp;<span class="k">mut</span> ArenaRows, ink: &amp;<span class="k">mut</span> Ink, b: &amp;BRep, cx: &amp;WalkCx) -&gt; Row {
    <span class="k">let</span> <span class="k">mut</span> fms = b.face_meshes_q(Some(QUALITY)); <span class="c">// one mesh per face</span>
    <span class="k">let</span> chains = edge_chains(b, &amp;fms); <span class="c">// edge polylines on the meshes</span>
    <span class="k">let</span> signs = face_signs(b, &amp;fms, &amp;chains); <span class="c">// +1 or -1 per face</span>
    <span class="k">let</span> <span class="k">mut</span> solid = Solid {
        pos: Vec::new(),
        tris: Vec::new(),
        bounds: AABB::empty(),
    };
    <span class="k">let</span> <span class="k">mut</span> verts = <span class="s">0</span>; <span class="c">// vertex total for spacing</span>

    <span class="k">for</span> (fi, fm) <span class="k">in</span> fms.iter_mut().enumerate() {
        fm.set_objectcolor(b.surfacecolor.clone());
        verts += fm.vertex.len();
        <span class="k">let</span> <span class="k">mut</span> rm = fm.to_render();

        <span class="c">// an inside-out face: flip normals and winding</span>
        <span class="k">if</span> signs[fi] &lt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">for</span> vertex <span class="k">in</span> &amp;<span class="k">mut</span> rm.vertices {
                <span class="k">for</span> component <span class="k">in</span> &amp;<span class="k">mut</span> vertex.normal {
                    *component = -*component;
                }
            }

            <span class="k">for</span> triangle <span class="k">in</span> rm.indices.chunks_exact_mut(<span class="s">3</span>) {
                triangle.swap(<span class="s">1</span>, <span class="s">2</span>);
            }
        }

        push_face(arena, &amp;rm, cx, &amp;<span class="k">mut</span> solid);
    }

    <span class="k">let</span> <span class="k">mut</span> flags = Instance::FLAG_SMOOTH; <span class="c">// vertices are samples</span>

    <span class="k">if</span> !b.is_solid() {
        flags |= Instance::FLAG_OPEN;
    }

    <span class="k">if</span> b.face_count() == <span class="s">1</span> {
        flags |= Instance::FLAG_SINGLE;
    }

    <span class="k">let</span> <span class="k">mut</span> row = Row {
        bounds: solid.bounds,
        spacing: mesh_spacing(&amp;solid.bounds, verts),
        flags,
        faces: <span class="s">true</span>,
    };

    <span class="k">if</span> !knobs::no_edges() {
        <span class="k">let</span> pen = Pen {
            row: cx.row,
            radius: encode_width(b.width),
            color: pack_rgba(Color::black().to_f32()),
        };
        <span class="k">let</span> ep = EdgePen {
            fms: &amp;fms,
            signs: &amp;signs,
            pen,
        };
        walk_brep_edges(ink, b, &amp;chains, (&amp;ep, &amp;<span class="k">mut</span> row.bounds));
    }

    row
}

<span class="c">/// Every BRep edge as pipes, or as a ribbon when no mesh owns it.</span>
<span class="k">fn</span> walk_brep_edges(
    ink: &amp;<span class="k">mut</span> Ink,
    b: &amp;BRep,
    chains: &amp;[Option&lt;EdgeChain&gt;],
    out: (&amp;EdgePen, &amp;<span class="k">mut</span> AABB),
) {
    <span class="k">let</span> (ep, bounds) = out;

    <span class="k">for</span> (ei, chain) <span class="k">in</span> chains.iter().enumerate() {
        <span class="k">match</span> chain {
            Some(c) =&gt; {
                push_edge_pipes(ink.seg, c, ep, bounds);
            }
            None =&gt; {
                <span class="k">if</span> !b.m_edges[ei].degenerated &amp;&amp; !b.edge_faces(ei).is_empty() {
                    log::warn!(
                        &quot;<span class="s">BREP </span>{<span class="s">:?</span>}<span class="s"> edge </span>{<span class="s">ei</span>}<span class="s">: missing tessellation boundary mapping; analytic display fallback is not a coherent CAD boundary</span>&quot;,
                        b.name
                    );
                }

                push_curve_ribbon(ink, b, ei, (&amp;ep.pen, bounds));
            }
        }
    }
}

<span class="c">/// An edge sampled from its 3D curve as a ribbon.</span>
<span class="k">fn</span> push_curve_ribbon(ink: &amp;<span class="k">mut</span> Ink, b: &amp;BRep, ei: usize, out: (&amp;Pen, &amp;<span class="k">mut</span> AABB)) {
    <span class="k">let</span> edge = &amp;b.m_edges[ei];

    <span class="k">if</span> edge.degenerated || edge.curve_3d_index &lt; <span class="s">0</span> {
        <span class="k">return</span>;
    }

    <span class="k">let</span> points: Vec&lt;[f32; 3]&gt; = sample_nurbscurve(&amp;b.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize])
        .into_iter()
        .map(super::curves::render_position)
        .collect();

    <span class="k">if</span> points.len() &lt; <span class="s">2</span> {
        <span class="k">return</span>;
    }

    push_polyline(ink.seg, &amp;points, out.<span class="s">0</span>, out.<span class="s">1</span>);</code></pre></div>
<h2 id="step-7-srcfixturers">Step 7 · src/fixture.rs<a class="anchor" href="#/course/07-boundaries#step-7-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: a block with a hole.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/07/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the line <code>use session_rust::{Color, Geometry, NurbsSurface, Point, …</code> in <code>lessons/06/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::{BRep, Color, Geometry, Xform};</code></pre></div>
<p>Replaces the <code>fn planar_surface</code> lines in <code>lessons/06/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Build the first source-face checkpoint entirely from local geometry.</span>
<span class="k">pub</span> <span class="k">fn</span> build() -&gt; CadFixture {
    <span class="k">let</span> <span class="k">mut</span> scene = CadFixture::new();
    <span class="k">let</span> <span class="k">mut</span> cylinder = BRep::create_cylinder(<span class="s">100</span>.<span class="s">0</span>, <span class="s">220</span>.<span class="s">0</span>);
    cylinder.surfacecolor = Color::grey();
    scene.add(
        Geometry::BRep(Rc::new(cylinder)),
        Xform::translation(-<span class="s">250</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
    );
    <span class="k">let</span> <span class="k">mut</span> hole = BRep::create_block_with_hole(<span class="s">320</span>.<span class="s">0</span>, <span class="s">240</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>, <span class="s">55</span>.<span class="s">0</span>);
    hole.surfacecolor = Color::grey();
    scene.add(
        Geometry::BRep(Rc::new(hole)),
        Xform::translation(<span class="s">220</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),</code></pre></div>
<h2 id="step-8-srclibrs">Step 8 · src/lib.rs<a class="anchor" href="#/course/07-boundaries#step-8-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The entry point picks the scene from the URL.</p>
<p><code>lessons/07/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>Ok(serde_json::json!({&quot;stage&quot;:6,&quot;objects&quot;:self.gpu.object…</code> in <code>lessons/06/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">7</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<h2 id="step-9-indexhtml">Step 9 · index.html<a class="anchor" href="#/course/07-boundaries#step-9-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status says checkpoint 07.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/07/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 06&lt;/title&gt;</code> in <code>lessons/06/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 07&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 06&lt;/output&gt;</code> in <code>lessons/06/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 07&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/06/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 07 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/07-boundaries#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/07/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A cylinder and a block with a hole show continuous source boundary curves; status: <strong>2 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/07.png" alt="Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A boundary floats or doubles: adjacent faces use different boundary samples.</li>
<li>A diagonal appears as an edge: mesh topology is substituted for the source edge chain.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/07-boundaries#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/07/src/app/walk/
├── bounds.rs
├── brep.rs  ~
├── brep_edges.rs  ~
├── brep_orient.rs  +
├── curves.rs
├── encode.rs
├── mesh.rs
├── mesh_ink.rs
├── mesh_topology.rs
└── mod.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/07/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/07-boundaries#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/08-trimming">08 · Trims, holes and periodic seams</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/07-boundaries#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.</p>
<p><a href="/session/docs/course/docs/screenshots/07.png"><img src="/session/docs/course/docs/screenshots/07.png" alt="Full viewer result for 07 boundaries" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-session_rustsrcnurbssurface_trimmedrs",text:"Step 1 · session_rust/src/nurbssurface_trimmed.rs"},{level:2,id:"step-2-session_rustsrcbreprs",text:"Step 2 · session_rust/src/brep.rs"},{level:2,id:"step-3-srcappwalkbrep_edgesrs",text:"Step 3 · src/app/walk/brep_edges.rs"},{level:2,id:"step-4-srcappwalkbrep_orientrs",text:"Step 4 · src/app/walk/brep_orient.rs"},{level:2,id:"step-5-srcappwalkmodrs",text:"Step 5 · src/app/walk/mod.rs"},{level:2,id:"step-6-srcappwalkbreprs",text:"Step 6 · src/app/walk/brep.rs"},{level:2,id:"step-7-srcfixturers",text:"Step 7 · src/fixture.rs"},{level:2,id:"step-8-srclibrs",text:"Step 8 · src/lib.rs"},{level:2,id:"step-9-indexhtml",text:"Step 9 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
