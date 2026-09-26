const s={title:"09 · Normals and shading",html:`<h1 id="09-normals-and-shading">09 · Normals and shading<a class="anchor" href="#/course/09-normals#09-normals-and-shading" aria-label="Link to this section">#</a></h1>
<p>The sphere shades smoothly and sharp rims retain separate normals under transformed placements.</p>
<p><img src="/session/docs/course/docs/illustrations/normals.svg" alt="Analytic normal or finite fallback at a pole; two shading normals at a C0 crease; the cofactor transform keeps a normal perpendicular under nonuniform scale." loading="lazy" decoding="async"></p>
<h2 id="step-1-session_rustsrcnurbssurface_trimmedrs">Step 1 · session_rust/src/nurbssurface_trimmed.rs<a class="anchor" href="#/course/09-normals#step-1-session_rustsrcnurbssurface_trimmedrs" aria-label="Link to this section">#</a></h2>
<p>Read the kernel&#39;s trimmed surface normals; used as is.</p>
<details class="note"><summary><code>session_rust/src/nurbssurface_trimmed.rs</code> · read only</summary><p><a href="#/course/kernel/nurbssurface_trimmed">Open the full listing</a></p>
</details>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/09/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/09/assets/pb/view_mixed_teapot.pb</code> (binary)</li>
</ul>
<p>Run <code>cargo check</code> in <code>lessons/09/</code>.</p>
<h2 id="step-2-srcshadersnormalswgsl">Step 2 · src/shaders/normals.wgsl<a class="anchor" href="#/course/09-normals#step-2-srcshadersnormalswgsl" aria-label="Link to this section">#</a></h2>
<p>Normals are rotated with the inverse transpose of the model matrix.</p>
<p><code>lessons/09/src/shaders/normals.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>let transformed = model * normal;</code> in <code>lessons/08/src/shaders/normals.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> x = model[<span class="s">0</span>];
    <span class="k">let</span> y = model[<span class="s">1</span>];
    <span class="k">let</span> z = model[<span class="s">2</span>];
<span class="c">    // determinant: sign tells a mirrored matrix</span>
    <span class="k">let</span> det = dot(x, cross(y, z));
    <span class="k">let</span> scale = length(x) * length(y) * length(z);

    <span class="k">if</span> (scale == <span class="s">0.0</span> || abs(det) &lt;= scale * 1e-<span class="s">12</span> || dot(normal, normal) == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

<span class="c">    // inverse transpose times the normal</span>
    <span class="k">let</span> transformed = <span class="k">mat3x3</span>&lt;<span class="k">f32</span>&gt;(cross(y, z), cross(z, x), cross(x, y)) * normal * sign(det);</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> face_normal(model: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;, normal: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code></code></pre></div>
<h2 id="step-3-srcshaderstrianglewgsl">Step 3 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/09-normals#step-3-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>The mesh shader shades with the rotated normal.</p>
<p><code>lessons/09/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>o.normal = vec3&lt;f32&gt;(0.0);</code> in <code>lessons/08/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.normal = face_normal(inst.model, in.normal);</code></pre></div>
<h2 id="step-4-srcappwalkbrep_edgesrs">Step 4 · src/app/walk/brep_edges.rs<a class="anchor" href="#/course/09-normals#step-4-srcappwalkbrep_edgesrs" aria-label="Link to this section">#</a></h2>
<p>Edge pipes take their facing from the two face normals.</p>
<p><code>lessons/09/src/app/walk/brep_edges.rs</code> · edit · type this</p>
<p>Replaces the <code>fn mean_normal</code> lines in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A triangle edge keyed by its two end positions.</span>
<span class="k">type</span> FacetEdge = [[u64; <span class="s">3</span>]; <span class="s">2</span>];

<span class="c">/// The triangles meeting at one edge.</span>
#[derive(Default)]
<span class="k">struct</span> FacetPair {
    normals: [Option&lt;[f64; 3]&gt;; <span class="s">2</span>], <span class="c">// first two triangle normals</span>
    count: usize,                   <span class="c">// how many triangles in total</span>
}

<span class="c">/// Floats are compared as raw bits here, because two vertices at the same spot must hash to the same key.</span>
<span class="k">fn</span> position_bits(position: [f64; <span class="s">3</span>]) -&gt; [u64; <span class="s">3</span>] {
    <span class="k">let</span> <span class="k">mut</span> bits = [<span class="s">0</span>; <span class="s">3</span>];

    <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
        bits[axis] = <span class="k">if</span> position[axis] == <span class="s">0</span>.<span class="s">0</span> {
            <span class="s">0</span>
        } <span class="k">else</span> {
            position[axis].to_bits()
        };
    }

    bits
}

<span class="c">/// Edge key for two positions, in either order.</span>
<span class="k">fn</span> facet_edge(a: [f64; <span class="s">3</span>], b: [f64; <span class="s">3</span>]) -&gt; FacetEdge {
    <span class="k">let</span> a = position_bits(a);
    <span class="k">let</span> b = position_bits(b);

    <span class="k">if</span> a &lt;= b { [a, b] } <span class="k">else</span> { [b, a] }
}

<span class="c">/// A normal is the direction a surface faces; comparing the two normals at an edge says whether it is a crease or flat.</span>
<span class="k">fn</span> face_facets(mesh: &amp;Mesh) -&gt; std::collections::HashMap&lt;FacetEdge, FacetPair&gt; {
    <span class="k">let</span> <span class="k">mut</span> result = std::collections::HashMap::&lt;FacetEdge, FacetPair&gt;::new();
    <span class="k">let</span> <span class="k">mut</span> faces: Vec&lt;_&gt; = mesh.face.keys().copied().collect();
    faces.sort_unstable();

    <span class="k">for</span> key <span class="k">in</span> faces {
        <span class="k">let</span> vertices = &amp;mesh.face[&amp;key];

        <span class="k">if</span> vertices.len() != <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> p = [[<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>]; <span class="s">3</span>];

        <span class="k">for</span> corner <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> v = &amp;mesh.vertex[&amp;vertices[corner]];
            p[corner] = [v.x, v.y, v.z];
        }

        <span class="k">let</span> <span class="k">mut</span> a = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];
        <span class="k">let</span> <span class="k">mut</span> b = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

        <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            a[axis] = p[<span class="s">1</span>][axis] - p[<span class="s">0</span>][axis];
            b[axis] = p[<span class="s">2</span>][axis] - p[<span class="s">0</span>][axis];
        }

        <span class="k">let</span> <span class="k">mut</span> normal = [
            a[<span class="s">1</span>] * b[<span class="s">2</span>] - a[<span class="s">2</span>] * b[<span class="s">1</span>],
            a[<span class="s">2</span>] * b[<span class="s">0</span>] - a[<span class="s">0</span>] * b[<span class="s">2</span>],
            a[<span class="s">0</span>] * b[<span class="s">1</span>] - a[<span class="s">1</span>] * b[<span class="s">0</span>],
        ];
        <span class="k">let</span> length = (normal[<span class="s">0</span>] * normal[<span class="s">0</span>] + normal[<span class="s">1</span>] * normal[<span class="s">1</span>] + normal[<span class="s">2</span>] * normal[<span class="s">2</span>]).sqrt();

        <span class="k">if</span> !length.is_finite() || length &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">for</span> component <span class="k">in</span> &amp;<span class="k">mut</span> normal {
            *component /= length;
        }

        <span class="k">for</span> edge <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> pair = result
                .entry(facet_edge(p[edge], p[(edge + <span class="s">1</span>) % <span class="s">3</span>]))
                .or_default();

            <span class="k">if</span> pair.count &lt; <span class="s">2</span> {
                pair.normals[pair.count] = Some(normal);
            }

            pair.count += <span class="s">1</span>;
        }
    }

    result
}

<span class="c">/// What every edge pipe of one BRep needs.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgePen&lt;'a&gt; {
    <span class="k">pub</span> fms: &amp;'a [Mesh],                                         <span class="c">// face meshes</span>
    <span class="k">pub</span> signs: &amp;'a [f64],                                        <span class="c">// +1 or -1 per face</span>
    <span class="k">pub</span> pen: Pen,                                                <span class="c">// row, width, colour</span>
    facets: Vec&lt;std::collections::HashMap&lt;FacetEdge, FacetPair&gt;&gt;, <span class="c">// per face: triangles at each edge</span>
}

<span class="k">impl</span>&lt;'a&gt; EdgePen&lt;'a&gt; {
    <span class="c">/// Index the triangles of every face once.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(fms: &amp;'a [Mesh], signs: &amp;'a [f64], pen: Pen) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> facets = Vec::with_capacity(fms.len());

        <span class="k">for</span> mesh <span class="k">in</span> fms {
            facets.push(face_facets(mesh));
        }

        <span class="k">Self</span> {
            fms,
            signs,
            pen,
            facets,
        }
    }

    <span class="c">/// Facing word of one pipe; unknown when ambiguous.</span>
    <span class="k">fn</span> facing(&amp;<span class="k">self</span>, chain: &amp;EdgeChain, a: [f64; <span class="s">3</span>], b: [f64; <span class="s">3</span>]) -&gt; u32 {
        <span class="k">let</span> key = facet_edge(a, b);
        <span class="k">let</span> Some(owner) = <span class="k">self</span>.facets[chain.face].get(&amp;key) <span class="k">else</span> {
            <span class="k">return</span> pack_facing(None, None);
        };

        <span class="k">if</span> owner.count == <span class="s">0</span> || owner.count &gt; <span class="s">2</span> {
            <span class="k">return</span> pack_facing(None, None);
        }

        <span class="k">let</span> first = scaled_normal(owner.normals[<span class="s">0</span>], <span class="k">self</span>.signs[chain.face]);
        <span class="k">let</span> second = <span class="k">if</span> <span class="k">let</span> Some(other) = chain.other {
            <span class="k">let</span> Some(pair) = <span class="k">self</span>.facets[other].get(&amp;key) <span class="k">else</span> {
                <span class="k">return</span> pack_facing(None, None);
            };

            <span class="k">if</span> pair.count != <span class="s">1</span> || owner.count != <span class="s">1</span> {
                <span class="k">return</span> pack_facing(None, None);
            }

            scaled_normal(pair.normals[<span class="s">0</span>], <span class="k">self</span>.signs[other])
        } <span class="k">else</span> <span class="k">if</span> owner.count == <span class="s">2</span> {
            scaled_normal(owner.normals[<span class="s">1</span>], <span class="k">self</span>.signs[chain.face])
        } <span class="k">else</span> {
            first
        };
        pack_facing(first.as_ref(), second.as_ref())
    }</code></pre></div>
<p>Delete the lines from <code>let n0 = scaled_normal(mean_normal(a.normal(), b.normal()…</code> to <code>};</code> from <code>lessons/08/src/app/walk/brep_edges.rs</code>.</p>
<p>Replaces the line <code>facing: pack_facing(n0.as_ref(), n1.as_ref()),</code> in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            facing: ep.facing(chain, p0, p1),</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/09/src/app/walk/brep_edges.rs</code> · edit · copy the file</p>
<p>Added after the last test in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The teapot's front meridian stays visible.</span>
    #[test]
    <span class="k">fn</span> teapot_front_meridian_is_not_self_occluded() {
        <span class="k">use</span> session_rust::{Line, Point, Session};
        <span class="k">let</span> scene = Session::pb_load(concat!(
            env!(&quot;<span class="s">CARGO_MANIFEST_DIR</span>&quot;),
            &quot;<span class="s">/assets/pb/view_mixed_teapot.pb</span>&quot;
        ))
        .unwrap();
        <span class="k">let</span> brep = &amp;scene.objects.breps[<span class="s">0</span>];
        <span class="k">let</span> meshes = brep.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> chains = edge_chains(brep, &amp;meshes);
        <span class="k">let</span> chain = chains[<span class="s">14</span>].as_ref().expect(&quot;<span class="s">authored front meridian</span>&quot;);
        <span class="k">let</span> owner = &amp;meshes[chain.face];
        <span class="k">let</span> <span class="k">mut</span> triangles = Vec::new();

        <span class="k">for</span> mesh <span class="k">in</span> &amp;meshes {
            assert!(!mesh.face.is_empty(), &quot;<span class="s">every authored patch remains meshed</span>&quot;);

            <span class="k">for</span> keys <span class="k">in</span> mesh.face.values() {
                triangles.push(
                    keys.iter()
                        .map(|key| mesh.vertex[key].position())
                        .collect::&lt;Vec&lt;_&gt;&gt;(),
                );
            }
        }

        <span class="k">let</span> eye = Point::new(-<span class="s">222</span>.<span class="s">278644</span>, -<span class="s">422</span>.<span class="s">587076</span>, <span class="s">439</span>.<span class="s">224717</span>);
        <span class="k">let</span> <span class="k">mut</span> checked = <span class="s">0</span>;

        <span class="k">for</span> pair <span class="k">in</span> chain.keys.windows(<span class="s">2</span>) {
            <span class="k">let</span> a = owner.vertex[&amp;pair[<span class="s">0</span>]].position();
            <span class="k">let</span> b = owner.vertex[&amp;pair[<span class="s">1</span>]].position();
            <span class="k">let</span> at = Point::new(
                (a[<span class="s">0</span>] + b[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (a[<span class="s">1</span>] + b[<span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (a[<span class="s">2</span>] + b[<span class="s">2</span>]) * <span class="s">0</span>.<span class="s">5</span>,
            );
            assert!(at[<span class="s">0</span>].abs() &lt; <span class="s">1</span>e-<span class="s">9</span> &amp;&amp; at[<span class="s">1</span>] &lt; -<span class="s">140</span>.<span class="s">0</span> &amp;&amp; at[<span class="s">2</span>] &gt;= <span class="s">90</span>.<span class="s">0</span> &amp;&amp; at[<span class="s">2</span>] &lt;= <span class="s">240</span>.<span class="s">0</span>);
            <span class="k">let</span> ray = Line::new(eye[<span class="s">0</span>], eye[<span class="s">1</span>], eye[<span class="s">2</span>], at[<span class="s">0</span>], at[<span class="s">1</span>], at[<span class="s">2</span>]);
            <span class="k">let</span> distance = eye.distance(&amp;at, None);

            <span class="k">for</span> triangle <span class="k">in</span> &amp;triangles {
                <span class="k">if</span> <span class="k">let</span> Some(hit) = session_rust::intersection::ray_triangle(
                    &amp;ray,
                    &amp;triangle[<span class="s">0</span>],
                    &amp;triangle[<span class="s">1</span>],
                    &amp;triangle[<span class="s">2</span>],
                    <span class="s">1</span>e-<span class="s">12</span>,
                ) {
                    assert!(
                        eye.distance(&amp;hit, None) &gt;= distance - <span class="s">1</span>e-<span class="s">6</span>,
                        &quot;<span class="s">front meridian at </span>{<span class="s">:?</span>}<span class="s"> buried by </span>{<span class="s">:?</span>}&quot;,
                        at,
                        triangle
                    );
                }
            }

            checked += <span class="s">1</span>;
        }

        assert!(checked &gt;= <span class="s">8</span>);
    }</code></pre></div>
<p>Replaces the lines from <code>let ep = EdgePen {</code> to <code>assert_ne!(p.facing, FACING_UNKNOWN);</code> in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> ep = EdgePen::new(
            &amp;fms,
            &amp;signs,
            Pen {
                row: <span class="s">3</span>,
                radius: encode_width(<span class="s">1</span>.<span class="s">0</span>),
                color: pack_rgba([<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
            },
        );
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
        <span class="k">let</span> circle = push_edge_pipes(&amp;<span class="k">mut</span> seg, chains[<span class="s">0</span>].as_ref().unwrap(), &amp;ep, &amp;<span class="k">mut</span> bounds);
        assert_eq!(circle, chains[<span class="s">0</span>].as_ref().unwrap().keys.len() - <span class="s">1</span>);

        <span class="k">for</span> p <span class="k">in</span> &amp;seg.pipes {
            assert_ne!(
                p.facing, FACING_UNKNOWN,
                &quot;<span class="s">missing incident facet for </span>{<span class="s">:?</span>}<span class="s"> → </span>{<span class="s">:?</span>}&quot;,
                p.p0, p.p1
            );</code></pre></div>
<p>Replaces the line <code>assert_eq!(p.facing &amp; 0xffff, p.facing &gt;&gt; 16);</code> in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_ne!(p.facing &amp; <span class="s">0xffff</span>, p.facing &gt;&gt; <span class="s">16</span>);
        assert!(seg.ribbons.is_empty());
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        assert!(seg.pipe_ids[..before].iter().all(|&amp;id| id == <span class="s">0</span>));
        assert_eq!(seg.pipe_ids[before], <span class="s">2</span>);
    }

    <span class="c">/// The cone seam faces by its triangles, not the apex fan.</span>
    #[test]
    <span class="k">fn</span> cone_seam_facing_uses_both_incident_facets() {
        <span class="k">let</span> cone = BRep::create_cone(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> meshes = cone.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> chains = edge_chains(&amp;cone, &amp;meshes);
        <span class="k">let</span> signs = vec![<span class="s">1</span>.<span class="s">0</span>; meshes.len()];
        <span class="k">let</span> pen = EdgePen::new(
            &amp;meshes,
            &amp;signs,
            Pen {
                row: <span class="s">4</span>,
                radius: <span class="s">1</span>.<span class="s">0</span>,
                color: <span class="s">0</span>,
            },
        );
        <span class="k">let</span> <span class="k">mut</span> segments = SegRows::default();
        push_edge_pipes(
            &amp;<span class="k">mut</span> segments,
            chains[<span class="s">1</span>].as_ref().unwrap(),
            &amp;pen,
            &amp;<span class="k">mut</span> AABB::empty(),
        );
        assert_eq!(segments.pipes.len(), <span class="s">1</span>);
        assert_eq!(segments.pipe_ids, vec![<span class="s">1</span>]);
        <span class="k">let</span> facing = segments.pipes[<span class="s">0</span>].facing;
        assert_ne!(facing &amp; <span class="s">0xffff</span>, facing &gt;&gt; <span class="s">16</span>);

        <span class="k">for</span> code <span class="k">in</span> [facing &amp; <span class="s">0xffff</span>, facing &gt;&gt; <span class="s">16</span>] {
            <span class="c">// Both cone facet normals occupy the positive-Z octahedron hemisphere.</span>
            <span class="k">let</span> x = (code <span class="k">as</span> u8 <span class="k">as</span> i8) <span class="k">as</span> f64 / <span class="s">127</span>.<span class="s">0</span>;
            <span class="k">let</span> y = ((code &gt;&gt; <span class="s">8</span>) <span class="k">as</span> u8 <span class="k">as</span> i8) <span class="k">as</span> f64 / <span class="s">127</span>.<span class="s">0</span>;
            <span class="k">let</span> z = <span class="s">1</span>.<span class="s">0</span> - x.abs() - y.abs();
            <span class="k">let</span> length = (x * x + y * y + z * z).sqrt();
            assert!(
                (z / length - <span class="s">150</span>.<span class="s">0f64</span> / (<span class="s">150</span>.<span class="s">0f64</span>.powi(<span class="s">2</span>) + <span class="s">400</span>.<span class="s">0f64</span>.powi(<span class="s">2</span>)).sqrt()).abs() &lt; <span class="s">0</span>.<span class="s">02</span>,
                &quot;<span class="s">physical cone facet normal must retain its slope; apex shading average gives z=0.822</span>&quot;
            );
        }
    }</code></pre></div>
<p>Replaces the lines from <code>let ep = EdgePen {</code> to <code>};</code> in <code>lessons/08/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> ep = EdgePen::new(
            &amp;fms,
            &amp;signs,
            Pen {
                row: <span class="s">0</span>,
                radius: <span class="s">1</span>.<span class="s">0</span>,
                color: <span class="s">0</span>,
            },
        );</code></pre></div>
<h2 id="step-5-srcappwalkbreprs">Step 5 · src/app/walk/brep.rs<a class="anchor" href="#/course/09-normals#step-5-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>The BRep walk uploads source normals.</p>
<p><code>lessons/09/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>let ep = EdgePen {</code> to <code>};</code> in <code>lessons/08/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> ep = EdgePen::new(&amp;fms, &amp;signs, pen);</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/09/src/app/walk/brep.rs</code> · edit · copy the file</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            );
        }
    }</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Each pyramid side keeps its own normal at the apex.</span>
    #[test]
    <span class="k">fn</span> pyramid_planar_faces_keep_constant_normals_through_collapsed_apex() {
        <span class="c">/// Dot product in f32.</span>
        <span class="k">fn</span> dot(a: [f32; <span class="s">3</span>], b: [f32; <span class="s">3</span>]) -&gt; f32 {
            a[<span class="s">0</span>] * b[<span class="s">0</span>] + a[<span class="s">1</span>] * b[<span class="s">1</span>] + a[<span class="s">2</span>] * b[<span class="s">2</span>]
        }
        <span class="k">let</span> pyramid = BRep::create_pyramid(<span class="s">400</span>.<span class="s">0</span>, <span class="s">350</span>.<span class="s">0</span>);
        <span class="k">let</span> (arena, _, _, _) = walked(&amp;pyramid);
        <span class="k">let</span> <span class="k">mut</span> apex_normals = Vec::new();

        <span class="k">for</span> triangle <span class="k">in</span> arena.idx.chunks_exact(<span class="s">3</span>) {
            <span class="k">let</span> a = arena.verts[triangle[<span class="s">0</span>] <span class="k">as</span> usize - <span class="s">100</span>];
            <span class="k">let</span> b = arena.verts[triangle[<span class="s">1</span>] <span class="k">as</span> usize - <span class="s">100</span>];
            <span class="k">let</span> c = arena.verts[triangle[<span class="s">2</span>] <span class="k">as</span> usize - <span class="s">100</span>];
            <span class="k">let</span> <span class="k">mut</span> ab = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];
            <span class="k">let</span> <span class="k">mut</span> ac = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

            <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                ab[axis] = b.position[axis] - a.position[axis];
                ac[axis] = c.position[axis] - a.position[axis];
            }

            <span class="k">let</span> <span class="k">mut</span> face = [
                ab[<span class="s">1</span>] * ac[<span class="s">2</span>] - ab[<span class="s">2</span>] * ac[<span class="s">1</span>],
                ab[<span class="s">2</span>] * ac[<span class="s">0</span>] - ab[<span class="s">0</span>] * ac[<span class="s">2</span>],
                ab[<span class="s">0</span>] * ac[<span class="s">1</span>] - ab[<span class="s">1</span>] * ac[<span class="s">0</span>],
            ];
            <span class="k">let</span> length = dot(face, face).sqrt();

            <span class="k">for</span> component <span class="k">in</span> &amp;<span class="k">mut</span> face {
                *component /= length;
            }

            <span class="k">for</span> vertex <span class="k">in</span> [a, b, c] {
                <span class="k">let</span> normal = vertex.normal;
                assert!(
                    dot(normal, face) &gt; <span class="s">1</span>.<span class="s">0</span> - <span class="s">1</span>e-<span class="s">6</span>,
                    &quot;<span class="s">planar face normal </span>{<span class="s">normal:?</span>}<span class="s"> differs from </span>{<span class="s">face:?</span>}&quot;
                );

                <span class="k">if</span> vertex.position[<span class="s">2</span>] == <span class="s">350</span>.<span class="s">0</span> {
                    apex_normals.push(normal);
                }
            }
        }

        assert_eq!(
            apex_normals.len(),
            <span class="s">4</span>,
            &quot;<span class="s">each separate planar side owns its apex vertex</span>&quot;
        );

        <span class="k">for</span> first <span class="k">in</span> <span class="s">0</span>..apex_normals.len() {
            <span class="k">for</span> second <span class="k">in</span> first + <span class="s">1</span>..apex_normals.len() {
                assert!(
                    dot(apex_normals[first], apex_normals[second]) &lt; <span class="s">0</span>.<span class="s">9</span>,
                    &quot;<span class="s">distinct BRep faces must not share a smoothed apex normal</span>&quot;
                );
            }
        }
    }</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/09/</code>.</p>
<h2 id="step-6-srcenginegpumodrs">Step 6 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/09-normals#step-6-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner uploads normals with the vertices.</p>
<p><code>lessons/09/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Replaces the line <code>self.segments.draw_pipes(&amp;mut pass, &amp;ink);</code> in <code>lessons/08/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">self</span>.view.show_mesh_edges {
                <span class="k">self</span>.segments.draw_pipes(&amp;<span class="k">mut</span> pass, &amp;ink);

                <span class="k">if</span> <span class="k">self</span>.view.markers {
                    <span class="k">self</span>.glyphs.draw_spheres(&amp;<span class="k">mut</span> pass, &amp;ink);
                }
            }</code></pre></div>
<h2 id="step-7-srclibrs">Step 7 · src/lib.rs<a class="anchor" href="#/course/09-normals#step-7-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The entry point reports the normal count.</p>
<p><code>lessons/09/src/lib.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.view.show_grid = <span class="s">false</span>;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.view.show_mesh_edges = app::route::query(&quot;<span class="s">fill</span>&quot;).is_none();
        gpu.view.markers = <span class="s">false</span>;</code></pre></div>
<p>Replaces the line <code>Ok(serde_json::json!({&quot;stage&quot;:8,&quot;objects&quot;:self.gpu.object…</code> in <code>lessons/08/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">9</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<h2 id="step-8-srcfixturers">Step 8 · src/fixture.rs<a class="anchor" href="#/course/09-normals#step-8-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: a folded surface with two normals at the crease.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/09/src/fixture.rs</code> · edit · copy the file</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A curved source patch carrying the producer's constrained hole mesh in its supported cache.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A mirrored, unevenly scaled placement.</span>
<span class="k">fn</span> affine_placement() -&gt; Xform {
    <span class="k">let</span> scale = Xform::from_matrix([
        -<span class="s">1</span>.<span class="s">4</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">65</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">15</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>,
    ]);
    Xform::rotation_z(<span class="s">23</span>.<span class="s">0</span>, <span class="s">true</span>) * Xform::rotation_x(<span class="s">17</span>.<span class="s">0</span>, <span class="s">true</span>) * scale
}

<span class="c">/// A folded surface with two normals at the crease.</span>
<span class="k">fn</span> crease_surface() -&gt; NurbsSurface {
    <span class="k">let</span> points = [
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, -<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">0</span>.<span class="s">0</span>, -<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, -<span class="s">100</span>.<span class="s">0</span>, <span class="s">160</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">160</span>.<span class="s">0</span>),
    ];
    <span class="k">let</span> <span class="k">mut</span> surface = NurbsSurface::create(<span class="s">false</span>, <span class="s">false</span>, <span class="s">1</span>, <span class="s">1</span>, <span class="s">3</span>, <span class="s">2</span>, &amp;points).unwrap();
    surface.facecolors = vec![Color::grey()];
    surface.name = &quot;<span class="s">CAD C0 crease</span>&quot;.into();
    surface
}

<span class="c">/// A block with a hole through it.</span>
<span class="k">fn</span> solid(kind: &amp;str) -&gt; BRep {
    <span class="k">let</span> <span class="k">mut</span> brep = <span class="k">match</span> kind {
        &quot;<span class="s">cylinder</span>&quot; =&gt; BRep::create_cylinder(<span class="s">120</span>.<span class="s">0</span>, <span class="s">240</span>.<span class="s">0</span>),
        &quot;<span class="s">sphere</span>&quot; =&gt; BRep::create_sphere(<span class="s">160</span>.<span class="s">0</span>),
        &quot;<span class="s">torus</span>&quot; =&gt; BRep::create_torus(<span class="s">120</span>.<span class="s">0</span>, <span class="s">35</span>.<span class="s">0</span>),
        &quot;<span class="s">hole</span>&quot; =&gt; BRep::create_block_with_hole(<span class="s">400</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">120</span>.<span class="s">0</span>, <span class="s">70</span>.<span class="s">0</span>),
        _ =&gt; panic!(&quot;<span class="s">unknown CAD fixture</span>&quot;),
    };
    brep.name = format!(&quot;<span class="s">CAD </span>{<span class="s">kind</span>}&quot;);
    brep.surfacecolor = Color::grey();
    brep
}</code></pre></div>
<p>Replaces the lines from <code>scene.add(</code> to <code>);</code> in <code>lessons/08/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The URL picks one local test scene.</span>
<span class="k">fn</span> query(name: &amp;str) -&gt; Option&lt;String&gt; {
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> search = web_sys::window()?.location().search().ok()?;
        <span class="k">let</span> prefix = format!(&quot;{<span class="s">name</span>}<span class="s">=</span>&quot;);

        <span class="k">for</span> part <span class="k">in</span> search.trim_start_matches('<span class="s">?</span>').split('<span class="s">&amp;</span>') {
            <span class="k">if</span> <span class="k">let</span> Some(value) = part.strip_prefix(&amp;prefix) {
                <span class="k">return</span> Some(value.to_string());
            }
        }

        None
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    {
        <span class="k">let</span> _ = name;
        None
    }
}

<span class="c">/// Build the first source-face checkpoint entirely from local geometry.</span>
<span class="k">pub</span> <span class="k">fn</span> build() -&gt; CadFixture {
    <span class="k">let</span> <span class="k">mut</span> scene = CadFixture::new();
    <span class="k">let</span> kind = query(&quot;<span class="s">cad</span>&quot;).unwrap_or(&quot;<span class="s">sphere</span>&quot;.into());
    <span class="k">let</span> geometry = <span class="k">match</span> kind.as_str() {
        &quot;<span class="s">crease</span>&quot; =&gt; Geometry::NurbsSurface(Rc::new(crease_surface())),
        &quot;<span class="s">trimmed</span>&quot; =&gt; Geometry::NurbsSurface(Rc::new(trimmed_surface())),
        &quot;<span class="s">torus</span>&quot; =&gt; Geometry::BRep(Rc::new(solid(&quot;<span class="s">torus</span>&quot;))),
        &quot;<span class="s">cylinder</span>&quot; | &quot;<span class="s">sphere</span>&quot; | &quot;<span class="s">hole</span>&quot; =&gt; Geometry::BRep(Rc::new(solid(&amp;kind))),
        _ =&gt; Geometry::BRep(Rc::new(solid(&quot;<span class="s">sphere</span>&quot;))),
    };
    <span class="k">let</span> place = <span class="k">if</span> query(&quot;<span class="s">affine</span>&quot;).as_deref() == Some(&quot;<span class="s">1</span>&quot;) {
        affine_placement()
    } <span class="k">else</span> {
        Xform::identity()
    };
    scene.add(geometry, place);</code></pre></div>
<h2 id="step-9-indexhtml">Step 9 · index.html<a class="anchor" href="#/course/09-normals#step-9-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status says checkpoint 09.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/09/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 08&lt;/title&gt;</code> in <code>lessons/08/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 09&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 08&lt;/output&gt;</code> in <code>lessons/08/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 09&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/08/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 09 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/09-normals#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/09/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The sphere shades smoothly and sharp rims retain separate normals under transformed placements; status: <strong>1 object</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/09.png" alt="Checkpoint 09 at ?cad=sphere&amp;lit=1: the sphere's interior shades smoothly, with no facet pattern." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A sphere looks faceted: triangle normals replace surface normals.</li>
<li>Shading crosses a sharp rim: coincident positions incorrectly share a normal.</li>
<li>A mirrored copy shades differently: the normal transform ignores the inverse transpose.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/09-normals#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/09/src/
├── app/
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs  ~
│   │   ├── brep_orient.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   └── mod.rs
│   ├── knobs.rs
│   ├── mod.rs
│   └── route.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text_outline.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   └── mod.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl  ~
│   ├── physical.wgsl
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   └── triangle.wgsl  ~
├── camera.rs
├── fixture.rs  ~
└── lib.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/09/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/09-normals#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/10-text-layout">10 · Text shaping</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/09-normals#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 09 at <code>?cad=sphere&amp;lit=1</code>: the sphere&#39;s interior shades smoothly, with no facet pattern.</p>
<p><a href="/session/docs/course/docs/screenshots/09.png"><img src="/session/docs/course/docs/screenshots/09.png" alt="Full viewer result for 09 normals" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-session_rustsrcnurbssurface_trimmedrs",text:"Step 1 · session_rust/src/nurbssurface_trimmed.rs"},{level:2,id:"step-2-srcshadersnormalswgsl",text:"Step 2 · src/shaders/normals.wgsl"},{level:2,id:"step-3-srcshaderstrianglewgsl",text:"Step 3 · src/shaders/triangle.wgsl"},{level:2,id:"step-4-srcappwalkbrep_edgesrs",text:"Step 4 · src/app/walk/brep_edges.rs"},{level:2,id:"step-5-srcappwalkbreprs",text:"Step 5 · src/app/walk/brep.rs"},{level:2,id:"step-6-srcenginegpumodrs",text:"Step 6 · src/engine/gpu/mod.rs"},{level:2,id:"step-7-srclibrs",text:"Step 7 · src/lib.rs"},{level:2,id:"step-8-srcfixturers",text:"Step 8 · src/fixture.rs"},{level:2,id:"step-9-indexhtml",text:"Step 9 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
