const s={title:"06 · CAD face rules",html:`<h1 id="06-cad-face-rules">06 · CAD face rules<a class="anchor" href="#/course/06-cad-contract#06-cad-face-rules" aria-label="Link to this section">#</a></h1>
<p>A shaded planar face shows four black boundaries with retained source identities.</p>
<p><img src="/session/docs/course/docs/illustrations/cad-contract.svg" alt="One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<h2 id="step-1-session_rustsrcremesh_nurbssurface_gridrs">Step 1 · session_rust/src/remesh_nurbssurface_grid.rs<a class="anchor" href="#/course/06-cad-contract#step-1-session_rustsrcremesh_nurbssurface_gridrs" aria-label="Link to this section">#</a></h2>
<p>Read the kernel mesher; the lesson uses it, never changes it.</p>
<details class="note"><summary><code>session_rust/src/remesh_nurbssurface_grid.rs</code> · read only</summary><p><a href="#/course/kernel/remesh_nurbssurface_grid">Open the full listing</a></p>
</details>
<p>Run <code>cargo check</code> in <code>lessons/06/</code>.</p>
<h2 id="step-2-srcappwalkencoders">Step 2 · src/app/walk/encode.rs<a class="anchor" href="#/course/06-cad-contract#step-2-srcappwalkencoders" aria-label="Link to this section">#</a></h2>
<p>New file: pack pen width, colour and face normals into the few bytes the shaders read.</p>
<p><code>lessons/06/src/app/walk/encode.rs</code> · 78 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Radius = half the pen width, in world mm; 0 = the default pen, also for the kernel default width 1.0.</span>
<span class="k">pub</span> <span class="k">fn</span> encode_width(w: f64) -&gt; f32 {
    <span class="k">if</span> w.is_finite() &amp;&amp; w &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; (w - <span class="s">1</span>.<span class="s">0</span>).abs() &gt; <span class="s">1</span>e-<span class="s">9</span> {
        (w <span class="k">as</span> f32) * <span class="s">0</span>.<span class="s">5</span>
    } <span class="k">else</span> {
        <span class="s">0</span>.<span class="s">0</span>
    }
}

<span class="c">/// 0.0..=1.0 to 0..=255; the + 0.5 rounds instead of truncating.</span>
<span class="k">fn</span> quant8(v: f32) -&gt; u32 {
    ((v.clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>) * <span class="s">255</span>.<span class="s">0</span> + <span class="s">0</span>.<span class="s">5</span>) <span class="k">as</span> u32) &amp; <span class="s">0xff</span>
}

<span class="c">/// Red in the lowest byte: the order WGSL \`unpack4x8unorm\` reads back.</span>
<span class="k">pub</span> <span class="k">fn</span> pack_rgba(c: [f32; <span class="s">4</span>]) -&gt; u32 {
    quant8(c[<span class="s">0</span>]) | quant8(c[<span class="s">1</span>]) &lt;&lt; <span class="s">8</span> | quant8(c[<span class="s">2</span>]) &lt;&lt; <span class="s">16</span> | quant8(c[<span class="s">3</span>]) &lt;&lt; <span class="s">24</span>
}

<span class="c">/// -1 or +1, never 0, so a coordinate of exactly 0 still folds to one side.</span>
<span class="k">fn</span> sign_not_zero(v: f64) -&gt; f64 {
    <span class="k">if</span> v &lt; <span class="s">0</span>.<span class="s">0</span> { -<span class="s">1</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">1</span>.<span class="s">0</span> }
}

<span class="c">/// -1.0..=1.0 to -127..=127, kept in the low 8 bits.</span>
<span class="k">fn</span> quant_snorm8(v: f64) -&gt; u32 {
    (((v.clamp(-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>) * <span class="s">127</span>.<span class="s">0</span>).round() <span class="k">as</span> i32) <span class="k">as</span> u32) &amp; <span class="s">0xff</span>
}

<span class="c">/// The 16-bit code that oct16_decode in normals.wgsl unpacks; the round trip is good to about a degree.</span>
<span class="k">pub</span> <span class="k">fn</span> oct16(n: &amp;[f64; <span class="s">3</span>]) -&gt; Option&lt;u32&gt; {
    <span class="k">let</span> l = n[<span class="s">0</span>].abs() + n[<span class="s">1</span>].abs() + n[<span class="s">2</span>].abs();

    <span class="k">if</span> l.partial_cmp(&amp;<span class="s">0</span>.<span class="s">0</span>) != Some(std::cmp::Ordering::Greater) {
        <span class="k">return</span> None; <span class="c">// zero or NaN vector</span>
    }

    <span class="k">let</span> (<span class="k">mut</span> x, <span class="k">mut</span> y) = (n[<span class="s">0</span>] / l, n[<span class="s">1</span>] / l); <span class="c">// now |x| + |y| + |z| = 1: a point on an octahedron</span>

    <span class="k">if</span> n[<span class="s">2</span>] &lt; <span class="s">0</span>.<span class="s">0</span> {
        <span class="c">// z &lt; 0: fold into the square's corners, so x and y alone still tell the halves apart</span>
        <span class="k">let</span> (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((<span class="s">1</span>.<span class="s">0</span> - ay) * sign_not_zero(x), (<span class="s">1</span>.<span class="s">0</span> - ax) * sign_not_zero(y));
    }

    Some(quant_snorm8(x) | quant_snorm8(y) &lt;&lt; <span class="s">8</span>)
}

<span class="c">/// 0xAABBGGRR: alpha 255, colour 0.</span>
<span class="k">pub</span> <span class="k">const</span> BLACK: u32 = <span class="s">0xff00_0000</span>;

<span class="c">/// No normals known: the shader draws the edge from every side.</span>
<span class="k">pub</span> <span class="k">const</span> FACING_UNKNOWN: u32 = u32::MAX;

<span class="c">/// The normals of the two faces beside an edge; the shader hides the edge when both face away.</span>
<span class="k">pub</span> <span class="k">fn</span> pack_facing(n0: Option&lt;&amp;[f64; 3]&gt;, n1: Option&lt;&amp;[f64; 3]&gt;) -&gt; u32 {
    <span class="k">let</span> pair = <span class="k">match</span> (n0, n1) {
        (Some(a), Some(b)) =&gt; (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) =&gt; (oct16(a), oct16(a)), <span class="c">// border edge: its one face twice</span>
        _ =&gt; (None, None),
    };

    <span class="k">match</span> pair {
        (Some(a), Some(b)) =&gt; {
            <span class="k">let</span> v = a | b &lt;&lt; <span class="s">16</span>;

            <span class="k">if</span> v == FACING_UNKNOWN { v ^ <span class="s">1</span> } <span class="k">else</span> { v } <span class="c">// flip one bit so real normals never read as &quot;unknown&quot;</span>
        }
        _ =&gt; FACING_UNKNOWN,
    }
}

<span class="c">/// Row, width and colour shared by every segment of one curve or edge set.</span>
<span class="k">pub</span> <span class="k">struct</span> Pen {
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> radius: f32,
    <span class="k">pub</span> color: u32,
}</code></pre></div>
<h2 id="step-3-srcappwalkmodrs">Step 3 · src/app/walk/mod.rs<a class="anchor" href="#/course/06-cad-contract#step-3-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>New file: the walk that turns each source object into GPU rows.</p>
<p><code>lessons/06/src/app/walk/mod.rs</code> · 38 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::AABB;
<span class="k">pub</span> <span class="k">mod</span> bounds;
<span class="k">pub</span> <span class="k">mod</span> brep;
<span class="k">pub</span> <span class="k">mod</span> brep_edges;
<span class="k">pub</span> <span class="k">mod</span> curves;
<span class="k">pub</span> <span class="k">mod</span> encode;
<span class="k">pub</span> <span class="k">mod</span> mesh;
<span class="k">pub</span> <span class="k">mod</span> mesh_ink;
<span class="k">pub</span> <span class="k">mod</span> mesh_topology;

<span class="c">/// One walk over the document turns each object into GPU rows; this says where that object's rows land.</span>
<span class="k">pub</span> <span class="k">struct</span> WalkCx {
    <span class="k">pub</span> vert_base: u32,   <span class="c">// arena vertices already on the GPU</span>
    <span class="k">pub</span> cloud_px: f32,    <span class="c">// point size override in px, 0 = file's own</span>
    <span class="k">pub</span> row: u32,         <span class="c">// this object's row index</span>
}

<span class="c">/// What a producer reports for one object row.</span>
<span class="k">pub</span> <span class="k">struct</span> Row {
    <span class="k">pub</span> bounds: AABB, <span class="c">// local bounding box</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// point or vertex spacing</span>
    <span class="k">pub</span> flags: u32,   <span class="c">// row flag bits</span>
    <span class="k">pub</span> faces: bool,  <span class="c">// row drew faces</span>
}

<span class="k">impl</span> Row {
    <span class="c">/// A row with only a box: lines, points, frames.</span>
    <span class="k">pub</span> <span class="k">fn</span> thin(bounds: AABB) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            bounds,
            spacing: <span class="s">0</span>.<span class="s">0</span>,
            flags: <span class="s">0</span>,
            faces: <span class="s">false</span>,
        }
    }
}</code></pre></div>
<h2 id="step-4-srcappwalkboundsrs">Step 4 · src/app/walk/bounds.rs<a class="anchor" href="#/course/06-cad-contract#step-4-srcappwalkboundsrs" aria-label="Link to this section">#</a></h2>
<p>New file: bounds collected from the rows one document adds.</p>
<p><code>lessons/06/src/app/walk/bounds.rs</code> · 74 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{Instance, Upload};
<span class="k">use</span> session_rust::{AABB, Xform};

<span class="c">/// Table lengths before a file is walked.</span>
<span class="k">pub</span> <span class="k">struct</span> Baselines {
    <span class="k">pub</span> obj: usize,    <span class="c">// object rows so far</span>
    <span class="k">pub</span> pipe: usize,   <span class="c">// pipe segments so far</span>
    <span class="k">pub</span> ribbon: usize, <span class="c">// ribbon segments so far</span>
}

<span class="k">impl</span> Baselines {
    <span class="c">/// Every table's length now.</span>
    <span class="k">pub</span> <span class="k">fn</span> capture(t: &amp;Upload) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            obj: t.obj.rows.len(),
            pipe: t.seg.pipes.len(),
            ribbon: t.seg.ribbons.len(),
        }
    }
}

<span class="c">/// World box of every object added since \`from\`.</span>
<span class="k">pub</span> <span class="k">fn</span> file_extent(t: &amp;Upload, from: &amp;Baselines) -&gt; AABB {
    <span class="k">let</span> <span class="k">mut</span> out = AABB::empty();

    <span class="k">for</span> r <span class="k">in</span> t.obj.rows.iter().skip(from.obj) {
        out.union_with(&amp;r.bounds.transformed(&amp;r.place));
    }

    out
}

<span class="c">/// True when every new row is flat at z = 0 of one placement.</span>
<span class="k">pub</span> <span class="k">fn</span> is_planar(t: &amp;Upload, from: &amp;Baselines, place: &amp;Xform) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> lo = f64::INFINITY;
    <span class="k">let</span> <span class="k">mut</span> hi = f64::NEG_INFINITY;

    <span class="k">for</span> r <span class="k">in</span> t.obj.rows.iter().skip(from.obj) {
        <span class="k">if</span> r.place != *place {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> !r.bounds.is_valid() {
            <span class="k">continue</span>;
        }

        lo = lo.min(r.bounds.cz - r.bounds.hz); <span class="c">// lowest z</span>
        hi = hi.max(r.bounds.cz + r.bounds.hz); <span class="c">// highest z</span>
    }

    lo.is_finite() &amp;&amp; (hi - lo).abs() &lt; <span class="s">1</span>e-<span class="s">3</span> <span class="c">// thinner than a micron</span>
}

<span class="c">/// Flag every new row as sheet content with a 1 mm pen.</span>
<span class="k">pub</span> <span class="k">fn</span> mark_sheet(t: &amp;<span class="k">mut</span> Upload, from: &amp;Baselines) {
    <span class="k">for</span> o <span class="k">in</span> t.obj.rows.iter_mut().skip(from.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }

    <span class="k">for</span> s <span class="k">in</span> t
        .seg
        .pipes
        .iter_mut()
        .skip(from.pipe)
        .chain(t.seg.ribbons.iter_mut().skip(from.ribbon))
    {
        <span class="k">if</span> s.radius &lt;= <span class="s">0</span>.<span class="s">0</span> {
            s.radius = <span class="s">0</span>.<span class="s">5</span>; <span class="c">// half width in mm</span>
        }
    }
}</code></pre></div>
<h2 id="step-5-srcappwalkmesh_topologyrs">Step 5 · src/app/walk/mesh_topology.rs<a class="anchor" href="#/course/06-cad-contract#step-5-srcappwalkmesh_topologyrs" aria-label="Link to this section">#</a></h2>
<p>New file: find each mesh edge once, with the faces beside it and one normal per face.</p>
<p><code>lessons/06/src/app/walk/mesh_topology.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::encode::{BLACK, pack_rgba};
<span class="k">use</span> session_rust::{Mesh, Tolerance};

<span class="c">/// Kernel vertex keys can have gaps (3, 7, 250); a slot is the vertex's index 0..n in our own arrays.</span>
<span class="k">pub</span> <span class="k">struct</span> SlotMap {
    dense: Vec&lt;u32&gt;,                               <span class="c">// dense[key] = slot, while keys are close together</span>
    sparse: std::collections::HashMap&lt;usize, u32&gt;, <span class="c">// key -&gt; slot, when they are spread out</span>
}

<span class="k">impl</span> SlotMap {
    <span class="c">/// \`keys\` must be sorted: the last one is the largest.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(keys: &amp;[usize]) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> max_key = keys.last().copied().unwrap_or(<span class="s">0</span>);

        <span class="c">// a plain array wins while fewer than 3 in 4 of its entries are unused</span>
        <span class="k">if</span> max_key &lt; <span class="s">4</span> * keys.len().max(<span class="s">1</span>) {
            <span class="k">let</span> <span class="k">mut</span> dense = vec![u32::MAX; max_key + <span class="s">1</span>];

            <span class="k">for</span> (s, &amp;k) <span class="k">in</span> keys.iter().enumerate() {
                dense[k] = s <span class="k">as</span> u32;
            }

            <span class="k">return</span> <span class="k">Self</span> {
                dense,
                sparse: std::collections::HashMap::new(),
            };
        }

        <span class="k">let</span> <span class="k">mut</span> sparse = std::collections::HashMap::with_capacity(keys.len());

        <span class="k">for</span> (s, &amp;k) <span class="k">in</span> keys.iter().enumerate() {
            sparse.insert(k, s <span class="k">as</span> u32);
        }

        <span class="k">Self</span> {
            dense: Vec::new(),
            sparse,
        }
    }

    <span class="c">/// Panics on an unknown key: every face corner must be a vertex of the mesh.</span>
    <span class="k">pub</span> <span class="k">fn</span> slot(&amp;<span class="k">self</span>, k: usize) -&gt; usize {
        <span class="k">if</span> <span class="k">self</span>.dense.is_empty() {
            <span class="k">self</span>.sparse[&amp;k] <span class="k">as</span> usize
        } <span class="k">else</span> {
            <span class="k">self</span>.dense[k] <span class="k">as</span> usize
        }
    }
}

<span class="c">/// Each edge once, the faces beside it and one normal per face: what the ink and the shading need.</span>
<span class="k">pub</span> <span class="k">struct</span> MeshTopo {
    <span class="k">pub</span> edges: Vec&lt;(usize, usize, u32)&gt;, <span class="c">// (low key, high key, pen colour)</span>
    <span class="k">pub</span> edge_faces: Vec&lt;[u32; 2]&gt;,       <span class="c">// two faces per edge, u32::MAX = none</span>
    <span class="k">pub</span> opposed: Vec&lt;bool&gt;,              <span class="c">// the two faces walk the edge in opposite directions, as consistent winding requires</span>
    <span class="k">pub</span> normals: Vec&lt;Option&lt;[f64; 3]&gt;&gt;,  <span class="c">// per face, None for a zero-area face</span>
    <span class="k">pub</span> closed: bool,                    <span class="c">// watertight: every edge has two faces</span></code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_topology.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="c">/// None when the polygon has no area, e.g. three corners on one line.</span>
<span class="k">fn</span> face_normal(vs: &amp;[usize], vpos: &amp;[[f64; <span class="s">3</span>]], slots: &amp;SlotMap) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">if</span> vs.len() &lt; <span class="s">3</span> {
        <span class="k">return</span> None;
    }

    <span class="c">// Newell's method: works for any polygon, even a slightly bent one, not just triangles</span>
    <span class="k">let</span> <span class="k">mut</span> n = [<span class="s">0</span>.<span class="s">0f64</span>; <span class="s">3</span>];

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..vs.len() {
        <span class="k">let</span> a = vpos[slots.slot(vs[i])];
        <span class="k">let</span> b = vpos[slots.slot(vs[(i + <span class="s">1</span>) % vs.len()])];
        n[<span class="s">0</span>] += (a[<span class="s">1</span>] - b[<span class="s">1</span>]) * (a[<span class="s">2</span>] + b[<span class="s">2</span>]);
        n[<span class="s">1</span>] += (a[<span class="s">2</span>] - b[<span class="s">2</span>]) * (a[<span class="s">0</span>] + b[<span class="s">0</span>]);
        n[<span class="s">2</span>] += (a[<span class="s">0</span>] - b[<span class="s">0</span>]) * (a[<span class="s">1</span>] + b[<span class="s">1</span>]);
    }

    <span class="k">let</span> len = (n[<span class="s">0</span>] * n[<span class="s">0</span>] + n[<span class="s">1</span>] * n[<span class="s">1</span>] + n[<span class="s">2</span>] * n[<span class="s">2</span>]).sqrt();

    <span class="k">if</span> len &gt; Tolerance::ZERO_TOLERANCE {
        Some([n[<span class="s">0</span>] / len, n[<span class="s">1</span>] / len, n[<span class="s">2</span>] / len])
    } <span class="k">else</span> {
        None
    }
}</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_topology.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Faces sort by kernel key, so the same mesh always yields the same edge order.</span>
<span class="k">fn</span> face_key(face: &amp;(usize, &amp;Vec&lt;usize&gt;)) -&gt; usize {
    face.<span class="s">0</span>
}

<span class="c">/// Finds each edge once with a linked list per vertex, no HashMap of vertex pairs.</span>
<span class="k">pub</span> <span class="k">fn</span> mesh_topology(m: &amp;Mesh, keys: &amp;[usize], vpos: &amp;[[f64; <span class="s">3</span>]], slots: &amp;SlotMap) -&gt; MeshTopo {
    <span class="k">let</span> <span class="k">mut</span> faces: Vec&lt;(usize, &amp;Vec&lt;usize&gt;)&gt; = Vec::with_capacity(m.face.len());

    <span class="k">for</span> (k, v) <span class="k">in</span> m.face.iter() {
        faces.push((*k, v));
    }

    faces.sort_unstable_by_key(face_key);
    <span class="k">let</span> cols = m.get_linecolors();

    <span class="k">let</span> <span class="k">mut</span> normals: Vec&lt;Option&lt;[f64; 3]&gt;&gt; = Vec::with_capacity(faces.len());
    <span class="k">let</span> <span class="k">mut</span> edges: Vec&lt;(usize, usize, u32)&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> edge_faces: Vec&lt;[u32; 2]&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> dir0: Vec&lt;u8&gt; = Vec::new(); <span class="c">// direction the first face walked each edge</span>
    <span class="k">let</span> <span class="k">mut</span> opposed: Vec&lt;bool&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> head: Vec&lt;u32&gt; = vec![u32::MAX; keys.len()]; <span class="c">// head[slot] = first edge whose lower key is this vertex</span>
    <span class="k">let</span> <span class="k">mut</span> next: Vec&lt;u32&gt; = Vec::new(); <span class="c">// next[edge] = the next one from the same vertex: a linked list in two arrays</span>

    <span class="k">for</span> (fs, (_, vs)) <span class="k">in</span> faces.iter().enumerate() {
        normals.push(face_normal(vs, vpos, slots));
        <span class="k">let</span> n = vs.len();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
            <span class="k">let</span> (u, v) = (vs[i], vs[(i + <span class="s">1</span>) % n]);
            <span class="k">let</span> (lo, hi, dir) = <span class="k">if</span> u &lt; v { (u, v, <span class="s">0</span>) } <span class="k">else</span> { (v, u, <span class="s">1</span>) }; <span class="c">// (lo, hi) names the edge whichever way a face walks it</span>
            <span class="k">let</span> ls = slots.slot(lo);
            <span class="k">let</span> <span class="k">mut</span> ei = head[ls];

            <span class="c">// follow lo's list until an edge ends at hi</span>
            <span class="k">while</span> ei != u32::MAX &amp;&amp; edges[ei <span class="k">as</span> usize].<span class="s">1</span> != hi {
                ei = next[ei <span class="k">as</span> usize];
            }

            <span class="c">// not found: append it and put it at the front of lo's list</span>
            <span class="k">if</span> ei == u32::MAX {
                ei = edges.len() <span class="k">as</span> u32;
                <span class="k">let</span> pen = <span class="k">match</span> cols.get(edges.len()) { <span class="c">// the n-th new edge takes the n-th kernel line colour</span>
                    Some(color) =&gt; pack_rgba(color.to_f32()),
                    None =&gt; BLACK,
                };
                edges.push((lo, hi, pen));
                edge_faces.push([u32::MAX; <span class="s">2</span>]);
                dir0.push(dir);
                opposed.push(<span class="s">true</span>);
                next.push(head[ls]);
                head[ls] = ei;
            }

            <span class="c">// the first two faces only; a third (a non-manifold edge) is ignored</span>
            <span class="k">let</span> ef = &amp;<span class="k">mut</span> edge_faces[ei <span class="k">as</span> usize];

            <span class="k">if</span> ef[<span class="s">0</span>] == u32::MAX {
                ef[<span class="s">0</span>] = fs <span class="k">as</span> u32;
                dir0[ei <span class="k">as</span> usize] = dir;
            } <span class="k">else</span> <span class="k">if</span> ef[<span class="s">1</span>] == u32::MAX &amp;&amp; ef[<span class="s">0</span>] != fs <span class="k">as</span> u32 {
                ef[<span class="s">1</span>] = fs <span class="k">as</span> u32;
                opposed[ei <span class="k">as</span> usize] = dir != dir0[ei <span class="k">as</span> usize];
            }
        }
    }

    <span class="c">// closed = no edge is missing its second face</span>
    <span class="k">let</span> <span class="k">mut</span> closed = !m.vertex.is_empty();

    <span class="k">for</span> f <span class="k">in</span> edge_faces.iter() {
        <span class="k">if</span> f[<span class="s">1</span>] == u32::MAX {
            closed = <span class="s">false</span>;
        }
    }

    <span class="c">// an edge of a hole ring has one face here, so ask the kernel instead</span>
    <span class="k">if</span> !closed &amp;&amp; !m.face_holes.is_empty() {
        closed = m.is_closed();
    }

    MeshTopo {
        edges,
        edge_faces,
        opposed,
        normals,
        closed,
    }
}</code></pre></div>
<h2 id="step-6-srcappwalkmesh_inkrs">Step 6 · src/app/walk/mesh_ink.rs<a class="anchor" href="#/course/06-cad-contract#step-6-srcappwalkmesh_inkrs" aria-label="Link to this section">#</a></h2>
<p>New file: turn mesh edges into pipes and vertices into dots, skipping flat diagonals and smooth seams.</p>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::encode::{BLACK, FACING_UNKNOWN, encode_width, oct16, pack_facing};
<span class="k">use</span> super::mesh::{COPLANAR_DOT, CREASE_COS, Lap, WIREFRAME_BLACK_MIN};
<span class="k">use</span> super::mesh_topology::{MeshTopo, SlotMap};
<span class="k">use</span> <span class="k">crate</span>::app::knobs;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, GlyphPoint};
<span class="k">use</span> session_rust::Mesh;
<span class="k">use</span> session_rust::mesh::ColorMode;

<span class="c">/// The two row tables a mesh's ink is appended to; \`&amp;'a mut\` lets them grow in place.</span>
<span class="k">pub</span> <span class="k">struct</span> Ink&lt;'a&gt; {
    <span class="k">pub</span> seg: &amp;'a <span class="k">mut</span> SegRows,     <span class="c">// one pipe per drawn edge</span>
    <span class="k">pub</span> glyph: &amp;'a <span class="k">mut</span> GlyphRows, <span class="c">// one sphere per vertex dot</span>
}

<span class="c">/// What the ink pass needs from the face pass.</span>
<span class="k">pub</span> <span class="k">struct</span> InkCx&lt;'a&gt; {
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> vpos: &amp;'a [[f32; <span class="s">3</span>]], <span class="c">// indexed by slot</span>
    <span class="k">pub</span> slots: &amp;'a SlotMap,
    <span class="k">pub</span> smooth: bool,         <span class="c">// only borders and creases are ink</span>
    <span class="k">pub</span> lap: &amp;'a <span class="k">mut</span> Lap,     <span class="c">// prints the time of each phase when VIEWER_PROFILE is set</span>
}

<span class="c">/// One width may stand for every edge; a missing entry is the default 1.0.</span>
<span class="k">fn</span> width_at(w: &amp;[f64], i: usize) -&gt; f64 {
    <span class="k">if</span> w.len() == <span class="s">1</span> {
        w[<span class="s">0</span>]
    } <span class="k">else</span> {
        w.get(i).copied().unwrap_or(<span class="s">1</span>.<span class="s">0</span>)
    }
}

<span class="c">/// Width 0 hides the edge.</span>
<span class="k">fn</span> hidden(w: &amp;[f64], i: usize) -&gt; bool {
    width_at(w, i) == <span class="s">0</span>.<span class="s">0</span>
}

<span class="c">/// None for the missing side of a border edge, or for a zero-area face.</span>
<span class="k">fn</span> normal_of(topo: &amp;MeshTopo, faces: [u32; <span class="s">2</span>], side: usize) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">if</span> faces[side] == u32::MAX {
        <span class="k">return</span> None;
    }

    topo.normals[faces[side] <span class="k">as</span> usize]
}

<span class="c">/// Both normals point out of the solid, even where the mesh winds one face backwards.</span>
<span class="k">fn</span> edge_normals(topo: &amp;MeshTopo, ei: usize) -&gt; (Option&lt;[f64; 3]&gt;, Option&lt;[f64; 3]&gt;) {
    <span class="k">let</span> f = topo.edge_faces[ei];
    <span class="k">let</span> n0 = normal_of(topo, f, <span class="s">0</span>);
    <span class="k">let</span> n1 = normal_of(topo, f, <span class="s">1</span>);

    <span class="k">if</span> topo.opposed[ei] {
        <span class="k">return</span> (n0, n1);</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    (n0, n1.map(reversed_normal)) <span class="c">// badly wound: flip the second</span>
}

<span class="k">fn</span> dot3(a: &amp;[f64; <span class="s">3</span>], b: &amp;[f64; <span class="s">3</span>]) -&gt; f64 {
    a[<span class="s">0</span>] * b[<span class="s">0</span>] + a[<span class="s">1</span>] * b[<span class="s">1</span>] + a[<span class="s">2</span>] * b[<span class="s">2</span>]
}

<span class="c">/// On a smooth surface only borders and creases count; the other mesh edges are sampling seams.</span>
<span class="k">fn</span> smooth_feature(topo: &amp;MeshTopo, ei: usize, pair: (Option&lt;[f64; 3]&gt;, Option&lt;[f64; 3]&gt;)) -&gt; bool {
    <span class="k">if</span> topo.edge_faces[ei][<span class="s">1</span>] == u32::MAX {
        <span class="k">return</span> <span class="s">true</span>; <span class="c">// border</span>
    }

    <span class="k">match</span> pair {
        (Some(n0), Some(n1)) =&gt; dot3(&amp;n0, &amp;n1) &lt; CREASE_COS,
        _ =&gt; <span class="s">true</span>,
    }
}

<span class="c">/// Adds the faces beside edge ei, each once.</span>
<span class="k">fn</span> push_faces(edge_faces: &amp;[[u32; <span class="s">2</span>]], ei: usize, fkeys: &amp;<span class="k">mut</span> Vec&lt;usize&gt;) {
    <span class="k">for</span> &amp;f <span class="k">in</span> edge_faces[ei].iter() {
        <span class="k">if</span> f == u32::MAX {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> fk = f <span class="k">as</span> usize;

        <span class="k">if</span> !fkeys.contains(&amp;fk) {
            fkeys.push(fk);
        }
    }
}

<span class="c">/// Word k holds codes 2k and 2k + 1; an odd count repeats the last code.</span>
<span class="k">fn</span> facing_word(codes: &amp;[u32], k: usize) -&gt; u32 {
    <span class="k">match</span> (codes.get(<span class="s">2</span> * k).copied(), codes.get(<span class="s">2</span> * k + <span class="s">1</span>).copied()) {
        (Some(a), b) =&gt; {
            <span class="k">let</span> v = a | b.unwrap_or(a) &lt;&lt; <span class="s">16</span>;

            <span class="k">if</span> v == FACING_UNKNOWN { v ^ <span class="s">1</span> } <span class="k">else</span> { v }
        }
        _ =&gt; FACING_UNKNOWN,
    }
}

<span class="c">/// Pipes for the edges worth drawing: hidden, flat-diagonal and smooth-seam edges are skipped.</span>
<span class="k">fn</span> push_pipes(ink: &amp;<span class="k">mut</span> Ink, m: &amp;Mesh, topo: &amp;MeshTopo, cx: &amp;InkCx) {
    <span class="k">let</span> w = m.widths();</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> black_wire = topo.edges.len() &gt;= WIREFRAME_BLACK_MIN; <span class="c">// a dense mesh reads as a wireframe: all black</span>
    ink.seg.pipes.reserve(topo.edges.len());

    <span class="k">for</span> (i, (a, b, col)) <span class="k">in</span> topo.edges.iter().enumerate() {
        <span class="k">let</span> (na, nb) = edge_normals(topo, i);
        <span class="k">let</span> facing = pack_facing(na.as_ref(), nb.as_ref());

        <span class="k">if</span> hidden(w, i) {
            <span class="k">continue</span>;
        }

        <span class="c">// skip the diagonal between two triangles of one flat quad</span>
        <span class="k">if</span> <span class="k">let</span> (Some(n0), Some(n1)) = (na, nb)
            &amp;&amp; dot3(&amp;n0, &amp;n1) &gt;= COPLANAR_DOT
            &amp;&amp; !knobs::all_edges()
        {
            <span class="k">continue</span>;
        }

        <span class="k">if</span> cx.smooth &amp;&amp; !smooth_feature(topo, i, (na, nb)) {
            <span class="k">continue</span>;
        }

        ink.seg
            .pipe_ids
            .push(<span class="k">if</span> cx.smooth { u32::MAX } <span class="k">else</span> { i <span class="k">as</span> u32 }); <span class="c">// the edge picking reports; none on a sampled surface</span>
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: <span class="k">if</span> black_wire { BLACK } <span class="k">else</span> { *col },
            facing,
        });
    }
}

<span class="c">/// Which edges touch each vertex, in one flat list: vertex i owns vinc[vstart[i]..vstart[i + 1]].</span>
<span class="k">struct</span> Incidence {
    best: Vec&lt;(f64, usize)&gt;, <span class="c">// per vertex: widest edge (width, index)</span>
    vstart: Vec&lt;u32&gt;,
    vinc: Vec&lt;u32&gt;,
}

<span class="c">/// Count first, then place: one allocation holds every vertex's list.</span>
<span class="k">fn</span> incidence(m: &amp;Mesh, topo: &amp;MeshTopo, cx: &amp;InkCx) -&gt; Incidence {
    <span class="k">let</span> w = m.widths();
    <span class="k">let</span> nv = cx.vpos.len();
    <span class="k">let</span> <span class="k">mut</span> best = vec![(f64::NEG_INFINITY, <span class="s">0usize</span>); nv];

    <span class="k">for</span> (i, (a, b, _)) <span class="k">in</span> topo.edges.iter().enumerate() {</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> hidden(w, i) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> wi = width_at(w, i);

        <span class="c">// a vertex dot is as wide as its widest edge</span>
        <span class="k">for</span> vk <span class="k">in</span> [*a, *b] {
            <span class="k">let</span> e = &amp;<span class="k">mut</span> best[cx.slots.slot(vk)];

            <span class="k">if</span> wi &gt; e.<span class="s">0</span> {
                *e = (wi, i);
            }
        }
    }

    <span class="c">// count edges per vertex; a running sum turns counts into starts: [0, 3, 2] -&gt; [0, 3, 5]</span>
    <span class="k">let</span> <span class="k">mut</span> vstart = vec![<span class="s">0u32</span>; nv + <span class="s">1</span>];

    <span class="k">for</span> (a, b, _) <span class="k">in</span> topo.edges.iter() {
        vstart[cx.slots.slot(*a) + <span class="s">1</span>] += <span class="s">1</span>;
        vstart[cx.slots.slot(*b) + <span class="s">1</span>] += <span class="s">1</span>;
    }

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nv {
        vstart[i + <span class="s">1</span>] += vstart[i];
    }

    <span class="c">// fill each vertex's range</span>
    <span class="k">let</span> <span class="k">mut</span> vinc = vec![<span class="s">0u32</span>; <span class="s">2</span> * topo.edges.len()];
    <span class="k">let</span> <span class="k">mut</span> cur = vstart.clone(); <span class="c">// cur[s] = next free place in vertex s's range</span>

    <span class="k">for</span> (i, (a, b, _)) <span class="k">in</span> topo.edges.iter().enumerate() {
        <span class="k">for</span> vk <span class="k">in</span> [*a, *b] {
            <span class="k">let</span> s = cx.slots.slot(vk);
            vinc[cur[s] <span class="k">as</span> usize] = i <span class="k">as</span> u32;
            cur[s] += <span class="s">1</span>;
        }
    }

    Incidence { best, vstart, vinc }
}

<span class="c">/// Two lifetimes: this borrow of InkCx ('a) may end before the data InkCx itself borrows ('b).</span>
<span class="k">struct</span> MarkerCx&lt;'a, 'b&gt; {
    cx: &amp;'a InkCx&lt;'b&gt;,
    inc: &amp;'a Incidence,
}

<span class="c">/// One dot per vertex with a visible edge.</span>
<span class="k">fn</span> push_markers(ink: &amp;<span class="k">mut</span> Ink, m: &amp;Mesh, topo: &amp;MeshTopo, input: &amp;MarkerCx) {
    <span class="k">let</span> (cx, inc) = (input.cx, input.inc);
    <span class="k">let</span> pc = m.get_pointcolors();
    <span class="k">let</span> dots_colored = m.color_mode == ColorMode::POINTCOLORS &amp;&amp; pc.len() == m.number_of_vertices(); <span class="c">// only with one colour per vertex</span>
    <span class="k">let</span> nv = cx.vpos.len();
    <span class="k">let</span> <span class="k">mut</span> fkeys: Vec&lt;usize&gt; = Vec::new(); <span class="c">// faces around one vertex</span>
    <span class="k">let</span> <span class="k">mut</span> codes: Vec&lt;u32&gt; = Vec::new(); <span class="c">// their normals as 16-bit codes, no repeats</span>
    ink.glyph.spheres.reserve(nv);

    <span class="k">for</span> (i, &amp;(vw, ei)) <span class="k">in</span> inc.best.iter().enumerate().take(nv) {
        <span class="k">if</span> vw == f64::NEG_INFINITY {
            <span class="k">continue</span>; <span class="c">// no visible edge</span>
        }

        fkeys.clear();
        push_faces(&amp;topo.edge_faces, ei, &amp;<span class="k">mut</span> fkeys);

        <span class="k">for</span> &amp;j <span class="k">in</span> &amp;inc.vinc[inc.vstart[i] <span class="k">as</span> usize..inc.vstart[i + <span class="s">1</span>] <span class="k">as</span> usize] {
            push_faces(&amp;topo.edge_faces, j <span class="k">as</span> usize, &amp;<span class="k">mut</span> fkeys);
        }

        codes.clear();

        <span class="k">for</span> fk <span class="k">in</span> &amp;fkeys {
            <span class="k">if</span> <span class="k">let</span> Some(n) = topo.normals[*fk]
                &amp;&amp; <span class="k">let</span> Some(code) = oct16(&amp;n)
                &amp;&amp; !codes.contains(&amp;code)
            {
                codes.push(code);</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            }
        }

        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: <span class="k">if</span> dots_colored {
                pc[i].to_f32()
            } <span class="k">else</span> {
                [<span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">1</span>, <span class="s">1</span>.<span class="s">0</span>]
            },
            instance_id: cx.row,
            <span class="c">// 3 words hold 6 normals; with more faces, always draw the dot</span>
            facing: <span class="k">if</span> codes.len() &gt; <span class="s">6</span> {
                FACING_UNKNOWN
            } <span class="k">else</span> {
                facing_word(&amp;codes, <span class="s">0</span>)
            },
            facing_ext: <span class="k">if</span> codes.len() &gt; <span class="s">6</span> {
                [FACING_UNKNOWN; <span class="s">2</span>]
            } <span class="k">else</span> {
                [facing_word(&amp;codes, <span class="s">1</span>), facing_word(&amp;codes, <span class="s">2</span>)]
            },
        });
    }
}

<span class="c">/// The ink of one mesh: a pipe per drawn edge, a dot per vertex, each phase timed.</span>
<span class="k">pub</span> <span class="k">fn</span> edges_and_dots(ink: &amp;<span class="k">mut</span> Ink, m: &amp;Mesh, topo: &amp;MeshTopo, cx: &amp;<span class="k">mut</span> InkCx) {
    <span class="k">let</span> inc = incidence(m, topo, cx);
    cx.lap.mark(&quot;<span class="s">incidence</span>&quot;);
    push_pipes(ink, m, topo, cx);
    cx.lap.mark(&quot;<span class="s">pipe loop</span>&quot;);

    <span class="k">if</span> knobs::no_dots() {
        <span class="k">return</span>;
    }

    push_markers(ink, m, topo, &amp;MarkerCx { cx, inc: &amp;inc });
    cx.lap.mark(&quot;<span class="s">markers</span>&quot;);
}

<span class="k">fn</span> reversed_normal(n: [f64; <span class="s">3</span>]) -&gt; [f64; <span class="s">3</span>] {
    [-n[<span class="s">0</span>], -n[<span class="s">1</span>], -n[<span class="s">2</span>]]
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::WalkCx;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::mesh::{MeshCx, MeshOpts, walk_mesh};
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
    <span class="k">use</span> session_rust::Point;

    <span class="c">/// How many pipes one mesh pushes when walked under \`opts\`.</span>
    <span class="k">fn</span> walk_pipes(mesh: &amp;Mesh, opts: &amp;MeshOpts) -&gt; usize {
        <span class="k">let</span> <span class="k">mut</span> arena = ArenaRows::default();
        <span class="k">let</span> <span class="k">mut</span> segments = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();
        <span class="k">let</span> <span class="k">mut</span> ink = Ink {
            seg: &amp;<span class="k">mut</span> segments,
            glyph: &amp;<span class="k">mut</span> glyphs,
        };
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">0</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">0</span>,
        };
        walk_mesh(&amp;<span class="k">mut</span> arena, &amp;<span class="k">mut</span> ink, mesh, &amp;MeshCx { cx: &amp;cx, opts });
        segments.pipes.len()
    }</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A slightly bulged 3x3 quad grid: 24 edges, 12 on the border.</span>
    <span class="k">fn</span> bulged_grid() -&gt; Mesh {
        <span class="k">let</span> <span class="k">mut</span> points = Vec::with_capacity(<span class="s">16</span>);

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
                <span class="k">let</span> (x, y) = (i <span class="k">as</span> f64 * <span class="s">100</span>.<span class="s">0</span>, j <span class="k">as</span> f64 * <span class="s">100</span>.<span class="s">0</span>);
                points.push(Point::new(x, y, <span class="s">2</span>e-<span class="s">4</span> * (x * x + y * y)));
            }
        }

        <span class="k">let</span> <span class="k">mut</span> faces = Vec::with_capacity(<span class="s">9</span>);

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> k = i * <span class="s">4</span> + j;
                faces.push(vec![k, k + <span class="s">4</span>, k + <span class="s">5</span>, k + <span class="s">1</span>]);
            }
        }

        Mesh::from_vertices_and_faces(points, faces)
    }</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/06/src/app/walk/mesh_ink.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Two quads at a right angle: 7 edges, one a fold.</span>
    <span class="k">fn</span> folded_pair() -&gt; Mesh {
        <span class="k">let</span> points = vec![
            Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>),
            Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>),
        ];
        Mesh::from_vertices_and_faces(points, vec![vec![<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>], vec![<span class="s">1</span>, <span class="s">4</span>, <span class="s">5</span>, <span class="s">2</span>]])
    }

    <span class="c">/// A box has 12 edges and 8 dots.</span>
    #[test]
    <span class="k">fn</span> box_ink_rows() {
        <span class="k">let</span> mesh = Mesh::create_box(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> arena = ArenaRows::default();
        <span class="k">let</span> <span class="k">mut</span> segments = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();
        <span class="k">let</span> <span class="k">mut</span> ink = Ink {
            seg: &amp;<span class="k">mut</span> segments,
            glyph: &amp;<span class="k">mut</span> glyphs,
        };
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">50</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">7</span>,
        };
        walk_mesh(
            &amp;<span class="k">mut</span> arena,
            &amp;<span class="k">mut</span> ink,
            &amp;mesh,
            &amp;MeshCx {
                cx: &amp;cx,
                opts: &amp;MeshOpts::OBJECT,
            },
        );
        assert_eq!(segments.pipes.len(), <span class="s">12</span>);
        assert_eq!(glyphs.spheres.len(), <span class="s">8</span>);

        <span class="k">for</span> segment <span class="k">in</span> &amp;segments.pipes {
            assert_ne!(segment.facing, FACING_UNKNOWN);
            assert_eq!(segment.instance_id, <span class="s">7</span>);
        }
    }

    <span class="c">/// A smooth mesh draws only borders and creases.</span>
    #[test]
    <span class="k">fn</span> smooth_mesh_inks_borders_and_creases_only() {
        <span class="k">let</span> grid = bulged_grid();
        assert_eq!(walk_pipes(&amp;grid, &amp;MeshOpts::OBJECT), <span class="s">24</span>);
        assert_eq!(walk_pipes(&amp;grid, &amp;MeshOpts::SURFACE), <span class="s">12</span>);
        assert_eq!(walk_pipes(&amp;folded_pair(), &amp;MeshOpts::SURFACE), <span class="s">7</span>);
    }
}</code></pre></div>
<h2 id="step-7-srcappwalkmeshrs">Step 7 · src/app/walk/mesh.rs<a class="anchor" href="#/course/06-cad-contract#step-7-srcappwalkmeshrs" aria-label="Link to this section">#</a></h2>
<p>New file: walk one mesh into arena triangles, then hand its edges and dots to the ink.</p>
<p><code>lessons/06/src/app/walk/mesh.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::mesh_ink::{Ink, InkCx, edges_and_dots};
<span class="k">use</span> super::mesh_topology::{SlotMap, mesh_topology};
<span class="k">use</span> super::{Row, WalkCx};
<span class="k">use</span> <span class="k">crate</span>::app::knobs;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::Instance;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Mesh;
<span class="k">use</span> session_rust::RenderVertex;

<span class="c">/// Above this many triangles a mesh gets no edges or dots.</span>
<span class="k">pub</span> <span class="k">const</span> MESH_RAW_MIN: usize = <span class="s">200_000</span>;

<span class="c">/// From this many edges the wireframe is black.</span>
<span class="k">pub</span> <span class="k">const</span> WIREFRAME_BLACK_MIN: usize = <span class="s">10_000</span>;

<span class="c">/// dot(n0, n1) of two unit normals is the cosine of their angle; above this the faces lie in one plane.</span>
<span class="k">pub</span> <span class="k">const</span> COPLANAR_DOT: f64 = <span class="s">1</span>.<span class="s">0</span> - <span class="s">1</span>e-<span class="s">9</span>;

<span class="c">/// Normal dot below which a smooth seam is a crease, cos 25°.</span>
<span class="k">pub</span> <span class="k">const</span> CREASE_COS: f64 = <span class="s">0</span>.<span class="s">906_307_787</span>;

<span class="c">/// Rough vertex spacing: box diagonal over sqrt(count), e.g. 10 m and 100 vertices give 1 m.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> mesh_spacing(bounds: &amp;AABB, verts: usize) -&gt; f32 {
    <span class="k">if</span> verts &lt; <span class="s">2</span> {
        <span class="k">return</span> <span class="s">0</span>.<span class="s">0</span>;
    }

    bounds.diagonal() <span class="k">as</span> f32 / (verts <span class="k">as</span> f32).sqrt()
}

<span class="c">/// A single width of 0 marks a print fill: a filled 2D shape on a sheet, drawn without edges.</span>
<span class="k">pub</span> <span class="k">fn</span> is_print_fill(m: &amp;Mesh) -&gt; bool {
    m.widths().len() == <span class="s">1</span> &amp;&amp; m.widths()[<span class="s">0</span>] == <span class="s">0</span>.<span class="s">0</span>
}</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Which kind of mesh this is; the three kinds are the constants below.</span>
<span class="k">pub</span> <span class="k">struct</span> MeshOpts {
    <span class="k">pub</span> sheet_lanes: bool, <span class="c">// print fills go to the sheet runs</span>
    <span class="k">pub</span> allow_open: bool,  <span class="c">// a mesh with holes is drawn with its back faces</span>
    <span class="k">pub</span> smooth: bool,      <span class="c">// a sampled surface, seams are not edges</span>
}

<span class="k">impl</span> MeshOpts {
    <span class="c">/// A plain mesh object.</span>
    <span class="k">pub</span> <span class="k">const</span> OBJECT: MeshOpts = MeshOpts {
        sheet_lanes: <span class="s">true</span>,
        allow_open: <span class="s">true</span>,
        smooth: <span class="s">false</span>,
    };

    <span class="c">/// A sampled NURBS surface.</span>
    <span class="k">pub</span> <span class="k">const</span> SURFACE: MeshOpts = MeshOpts {
        sheet_lanes: <span class="s">false</span>,
        allow_open: <span class="s">true</span>,
        smooth: <span class="s">true</span>,
    };

    <span class="c">/// An element's mesh, never flagged open.</span>
    <span class="k">pub</span> <span class="k">const</span> ELEMENT: MeshOpts = MeshOpts {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/06/src/app/walk/mesh.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        sheet_lanes: <span class="s">true</span>,
        allow_open: <span class="s">false</span>,
        smooth: <span class="s">false</span>,
    };
}

<span class="c">/// Prints the time between marks when VIEWER_PROFILE is set; one bool test when it is not.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">struct</span> Lap {
    on: bool,
    at: std::time::Instant, <span class="c">// last mark</span>
    prefix: &amp;'static str,   <span class="c">// caller name in each line</span>
}

<span class="c">/// \`Instant::now()\` panics on wasm32, so the browser build gets an empty timer.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> Lap;

#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">impl</span> Lap {
    <span class="k">pub</span> <span class="k">fn</span> start(prefix: &amp;'static str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            on: knobs::profile(),
            at: std::time::Instant::now(),
            prefix,
        }
    }

    <span class="c">/// Print the time since the last mark.</span>
    <span class="k">pub</span> <span class="k">fn</span> mark(&amp;<span class="k">mut</span> <span class="k">self</span>, name: &amp;str) {
        <span class="k">if</span> <span class="k">self</span>.on {
            eprintln!(&quot;<span class="s">  </span>{}<span class="s"> </span>{<span class="s">name:&lt;20</span>}<span class="s"> </span>{<span class="s">:?</span>}&quot;, <span class="k">self</span>.prefix, <span class="k">self</span>.at.elapsed());
            <span class="k">self</span>.at = std::time::Instant::now();
        }
    }
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> Lap {
    <span class="k">pub</span> <span class="k">fn</span> start(_prefix: &amp;'static str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>
    }

    <span class="k">pub</span> <span class="k">fn</span> mark(&amp;<span class="k">mut</span> <span class="k">self</span>, _name: &amp;str) {}
}

<span class="c">/// Three index lists draw in order: faces, sheet fills, then sheet lettering on top.</span>
<span class="k">fn</span> index_run&lt;'a&gt;(arena: &amp;'a <span class="k">mut</span> ArenaRows, m: &amp;Mesh, sheet: bool) -&gt; &amp;'a <span class="k">mut</span> Vec&lt;u32&gt; {
    <span class="k">if</span> !sheet {
        <span class="k">return</span> &amp;<span class="k">mut</span> arena.idx;
    }

    <span class="k">if</span> m.name == &quot;<span class="s">text</span>&quot; {
        &amp;<span class="k">mut</span> arena.idx_text
    } <span class="k">else</span> {
        &amp;<span class="k">mut</span> arena.idx_print
    }
}

<span class="c">/// The shared walk context plus this mesh's kind.</span>
<span class="k">pub</span> <span class="k">struct</span> MeshCx&lt;'a&gt; {</code></pre></div>
<p><code>lessons/06/src/app/walk/mesh.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> cx: &amp;'a WalkCx,
    <span class="k">pub</span> opts: &amp;'a MeshOpts,
}

<span class="c">/// Triangles always; edges and dots only up to MESH_RAW_MIN triangles, and never for a print fill.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_mesh(arena: &amp;<span class="k">mut</span> ArenaRows, ink: &amp;<span class="k">mut</span> Ink, m: &amp;Mesh, mc: &amp;MeshCx) -&gt; Row {
    <span class="k">let</span> (cx, o) = (mc.cx, mc.opts);
    <span class="k">let</span> base = cx.vert_base + arena.verts.len() <span class="k">as</span> u32; <span class="c">// this mesh's indices count from after every vertex already stored</span>
    <span class="k">let</span> <span class="k">mut</span> lap = Lap::start(&quot;<span class="s">walk_mesh</span>&quot;);
    <span class="k">let</span> rm = m.to_render(); <span class="c">// the kernel splits every polygon into triangles</span>
    lap.mark(&quot;<span class="s">to_render</span>&quot;);

    <span class="k">let</span> print = is_print_fill(m);
    <span class="k">let</span> decorated = rm.indices.len() / <span class="s">3</span> &lt;= MESH_RAW_MIN &amp;&amp; !print;
    <span class="k">let</span> keys = <span class="k">if</span> decorated { m.vertices() } <span class="k">else</span> { Vec::new() }; <span class="c">// sorted, as SlotMap needs</span>
    <span class="k">let</span> slots = SlotMap::new(&amp;keys);
    <span class="k">let</span> <span class="k">mut</span> vpos64 = Vec::with_capacity(keys.len()); <span class="c">// f64 for exact normals, f32 for the GPU</span>
    <span class="k">let</span> <span class="k">mut</span> vpos = Vec::with_capacity(keys.len());

    <span class="k">for</span> &amp;key <span class="k">in</span> &amp;keys {
        <span class="k">let</span> point = &amp;m.vertex[&amp;key];
        vpos64.push([point.x, point.y, point.z]);
        vpos.push([point.x <span class="k">as</span> f32, point.y <span class="k">as</span> f32, point.z <span class="k">as</span> f32]);
    }

    <span class="k">let</span> topo = <span class="k">if</span> decorated {
        Some(mesh_topology(m, &amp;keys, &amp;vpos64, &amp;slots))
    } <span class="k">else</span> {
        None
    };
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());

    <span class="c">// triangles into the arena</span>
    <span class="k">for</span> v <span class="k">in</span> &amp;rm.vertices {
        bounds.union_with_point(
            v.position[<span class="s">0</span>] <span class="k">as</span> f64,
            v.position[<span class="s">1</span>] <span class="k">as</span> f64,
            v.position[<span class="s">2</span>] <span class="k">as</span> f64,
        );
        arena.verts.push(*v);
        arena.vids.push(cx.row); <span class="c">// so the shader finds this object's matrix</span>
    }

    <span class="k">let</span> idx = index_run(arena, m, o.sheet_lanes &amp;&amp; print);
    idx.reserve(rm.indices.len());

    <span class="k">for</span> &amp;i <span class="k">in</span> &amp;rm.indices {
        idx.push(base + i);
    }

    lap.mark(&quot;<span class="s">vert+idx push</span>&quot;);
    <span class="k">let</span> <span class="k">mut</span> flags = <span class="k">if</span> o.sheet_lanes &amp;&amp; print {
        Instance::FLAG_PRINT
    } <span class="k">else</span> {
        <span class="s">0</span>
    };
    <span class="k">let</span> smooth = o.smooth &amp;&amp; !knobs::seams(); <span class="c">// VIEWER_SEAMS: debug view of every sampling seam</span>

    <span class="k">if</span> smooth {
        flags |= Instance::FLAG_SMOOTH;
    }

    <span class="k">if</span> m.number_of_faces() == <span class="s">1</span> {
        flags |= Instance::FLAG_SINGLE;
    }

    <span class="k">let</span> row = Row {
        bounds,
        spacing: mesh_spacing(&amp;bounds, m.number_of_vertices()),
        flags,
        faces: <span class="s">true</span>,
    };

    <span class="k">if</span> !decorated || knobs::no_edges() {
        <span class="k">return</span> row; <span class="c">// triangles only</span>
    }

    <span class="k">let</span> topo = topo.expect(&quot;<span class="s">decorated mesh has topology</span>&quot;);
    lap.mark(&quot;<span class="s">topology</span>&quot;);
    <span class="k">let</span> <span class="k">mut</span> icx = InkCx {
        row: cx.row,
        vpos: &amp;vpos,
        slots: &amp;slots,
        smooth,
        lap: &amp;<span class="k">mut</span> lap,
    };
    edges_and_dots(ink, m, &amp;topo, &amp;<span class="k">mut</span> icx);

    <span class="c">// an open mesh keeps its back faces</span>
    <span class="k">let</span> open = o.allow_open &amp;&amp; !topo.closed;
    Row {
        flags: <span class="k">if</span> open {
            row.flags | Instance::FLAG_OPEN
        } <span class="k">else</span> {
            row.flags
        },
        ..row
    }
}

<span class="c">/// Just the positions of the render vertices, e.g. for a thickness measure.</span>
<span class="k">fn</span> positions(verts: &amp;[RenderVertex]) -&gt; Vec&lt;[f32; 3]&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(verts.len());

    <span class="k">for</span> v <span class="k">in</span> verts {
        out.push(v.position);
    }

    out
}</code></pre></div>
<h2 id="step-8-srcappwalkcurvesrs">Step 8 · src/app/walk/curves.rs<a class="anchor" href="#/course/06-cad-contract#step-8-srcappwalkcurvesrs" aria-label="Link to this section">#</a></h2>
<p>New file: curves sampled into connected strokes.</p>
<p><code>lessons/06/src/app/walk/curves.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::Row;
<span class="k">use</span> super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::CylinderSegment;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::{Line, NurbsCurve, Polyline};

<span class="c">/// One segment per pair of neighbours; grows \`bounds\`.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> push_polyline(seg: &amp;<span class="k">mut</span> SegRows, pts: &amp;[[f32; <span class="s">3</span>]], pen: &amp;Pen, bounds: &amp;<span class="k">mut</span> AABB) {
    seg.ribbons.reserve(pts.len().saturating_sub(<span class="s">1</span>));

    <span class="k">for</span> w <span class="k">in</span> pts.windows(<span class="s">2</span>) {
        bounds.union_with_point(w[<span class="s">0</span>][<span class="s">0</span>] <span class="k">as</span> f64, w[<span class="s">0</span>][<span class="s">1</span>] <span class="k">as</span> f64, w[<span class="s">0</span>][<span class="s">2</span>] <span class="k">as</span> f64);
        seg.ribbons.push(CylinderSegment {
            p0: w[<span class="s">0</span>],
            radius: pen.radius,
            p1: w[<span class="s">1</span>],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN,
        });
    }

    <span class="k">if</span> <span class="k">let</span> Some(last) = pts.last() {
        bounds.union_with_point(last[<span class="s">0</span>] <span class="k">as</span> f64, last[<span class="s">1</span>] <span class="k">as</span> f64, last[<span class="s">2</span>] <span class="k">as</span> f64);
    }
}

<span class="c">/// A line as one segment.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_line(seg: &amp;<span class="k">mut</span> SegRows, l: &amp;Line, row: u32) -&gt; Row {
    <span class="k">let</span> p0 = [l[<span class="s">0</span>] <span class="k">as</span> f32, l[<span class="s">1</span>] <span class="k">as</span> f32, l[<span class="s">2</span>] <span class="k">as</span> f32];
    <span class="k">let</span> p1 = [l[<span class="s">3</span>] <span class="k">as</span> f32, l[<span class="s">4</span>] <span class="k">as</span> f32, l[<span class="s">5</span>] <span class="k">as</span> f32];
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    bounds.union_with_point(p0[<span class="s">0</span>] <span class="k">as</span> f64, p0[<span class="s">1</span>] <span class="k">as</span> f64, p0[<span class="s">2</span>] <span class="k">as</span> f64);
    bounds.union_with_point(p1[<span class="s">0</span>] <span class="k">as</span> f64, p1[<span class="s">1</span>] <span class="k">as</span> f64, p1[<span class="s">2</span>] <span class="k">as</span> f64);
    seg.ribbons.push(CylinderSegment {
        p0,
        radius: encode_width(l.width),
        p1,
        instance_id: row,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN,
    });
    Row::thin(bounds)
}

<span class="c">/// A polyline as one segment per span.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_polyline(seg: &amp;<span class="k">mut</span> SegRows, pl: &amp;Polyline, row: u32) -&gt; Row {
    <span class="k">let</span> <span class="k">mut</span> pts: Vec&lt;[f32; 3]&gt; = Vec::with_capacity(pl.coords.len() / <span class="s">3</span>);

    <span class="k">for</span> c <span class="k">in</span> pl.coords.chunks_exact(<span class="s">3</span>) {
        pts.push([c[<span class="s">0</span>] <span class="k">as</span> f32, c[<span class="s">1</span>] <span class="k">as</span> f32, c[<span class="s">2</span>] <span class="k">as</span> f32]);
    }

    <span class="k">let</span> pen = Pen {
        row,
        radius: encode_width(pl.width),
        color: pack_rgba(pl.linecolor.to_f32()),
    };
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    push_polyline(seg, &amp;pts, &amp;pen, &amp;<span class="k">mut</span> bounds);
    Row::thin(bounds)
}</code></pre></div>
<p><code>lessons/06/src/app/walk/curves.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Degrees of turning one chord may span.</span>
<span class="k">const</span> CHORD_DEGREES: f64 = <span class="s">5</span>.<span class="s">0</span>;

<span class="c">/// Control point \`i\` with its weight divided out.</span>
<span class="k">fn</span> control_position(c: &amp;NurbsCurve, i: usize) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">let</span> p = c.cv(i)?;
    <span class="k">let</span> w = <span class="k">if</span> c.m_is_rat &amp;&amp; p.len() &gt; <span class="s">3</span> &amp;&amp; p[<span class="s">3</span>] != <span class="s">0</span>.<span class="s">0</span> {
        p[<span class="s">3</span>]
    } <span class="k">else</span> {
        <span class="s">1</span>.<span class="s">0</span>
    };
    Some([p[<span class="s">0</span>] / w, p[<span class="s">1</span>] / w, p[<span class="s">2</span>] / w])
}

<span class="c">/// f64 point to the f32 the GPU takes.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> render_position(point: [f64; <span class="s">3</span>]) -&gt; [f32; <span class="s">3</span>] {
    [point[<span class="s">0</span>] <span class="k">as</span> f32, point[<span class="s">1</span>] <span class="k">as</span> f32, point[<span class="s">2</span>] <span class="k">as</span> f32]
}

<span class="c">/// Total turning of the control polygon in degrees.</span>
<span class="k">fn</span> turning_degrees(c: &amp;NurbsCurve) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> prev: Option&lt;[f64; 3]&gt; = None;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..c.m_cv_count {
        <span class="k">let</span> (Some(a), Some(b)) = (control_position(c, i - <span class="s">1</span>), control_position(c, i)) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> d = [b[<span class="s">0</span>] - a[<span class="s">0</span>], b[<span class="s">1</span>] - a[<span class="s">1</span>], b[<span class="s">2</span>] - a[<span class="s">2</span>]];
        <span class="k">let</span> len = (d[<span class="s">0</span>] * d[<span class="s">0</span>] + d[<span class="s">1</span>] * d[<span class="s">1</span>] + d[<span class="s">2</span>] * d[<span class="s">2</span>]).sqrt();

        <span class="k">if</span> len &lt; <span class="s">1</span>e-<span class="s">12</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> u = [d[<span class="s">0</span>] / len, d[<span class="s">1</span>] / len, d[<span class="s">2</span>] / len]; <span class="c">// unit direction</span>

        <span class="k">if</span> <span class="k">let</span> Some(q) = prev {
            <span class="k">let</span> dot = (q[<span class="s">0</span>] * u[<span class="s">0</span>] + q[<span class="s">1</span>] * u[<span class="s">1</span>] + q[<span class="s">2</span>] * u[<span class="s">2</span>]).clamp(-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
            total += dot.acos().to_degrees(); <span class="c">// angle between neighbours</span>
        }

        prev = Some(u);
    }

    total
}

<span class="c">/// Sample the curve, one chord per \`CHORD_DEGREES\`.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> sample_nurbscurve(c: &amp;NurbsCurve) -&gt; Vec&lt;[f64; 3]&gt; {
    <span class="k">if</span> c.m_cv_count &lt; <span class="s">2</span> {
        <span class="k">return</span> Vec::new();
    }

    <span class="k">let</span> spans = c.span_count().max(<span class="s">1</span>);
    <span class="k">let</span> n = ((turning_degrees(c) / CHORD_DEGREES).ceil() <span class="k">as</span> usize).clamp(spans, <span class="s">512</span>); <span class="c">// chord count</span>

    <span class="k">let</span> (t0, t1) = c.domain();
    <span class="k">let</span> <span class="k">mut</span> pts: Vec&lt;[f64; 3]&gt; = Vec::with_capacity(n + <span class="s">1</span>);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..=n {
        <span class="k">let</span> point = c.point_at(t0 + (t1 - t0) * i <span class="k">as</span> f64 / n <span class="k">as</span> f64);
        pts.push([point[<span class="s">0</span>], point[<span class="s">1</span>], point[<span class="s">2</span>]]);
    }

    pts
}

<span class="c">/// A curve as a sampled polyline.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_nurbscurve(seg: &amp;<span class="k">mut</span> SegRows, c: &amp;NurbsCurve, row: u32) -&gt; Row {
    <span class="k">let</span> pts: Vec&lt;_&gt; = sample_nurbscurve(c)
        .into_iter()
        .map(render_position)
        .collect();
    <span class="k">let</span> color = c
        .linecolors
        .first()
        .map(session_rust::Color::to_f32)
        .unwrap_or([<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]); <span class="c">// black when unset</span>
    <span class="k">let</span> pen = Pen {
        row,
        radius: encode_width(c.width),
        color: pack_rgba(color),
    };
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    push_polyline(seg, &amp;pts, &amp;pen, &amp;<span class="k">mut</span> bounds);</code></pre></div>
<p><code>lessons/06/src/app/walk/curves.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Row::thin(bounds)
}</code></pre></div>
<h2 id="step-9-srcappwalkbrep_edgesrs">Step 9 · src/app/walk/brep_edges.rs<a class="anchor" href="#/course/06-cad-contract#step-9-srcappwalkbrep_edgesrs" aria-label="Link to this section">#</a></h2>
<p>New file: boundary chains shared between faces.</p>
<p><code>lessons/06/src/app/walk/brep_edges.rs</code> · 18 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::brep::BRepOrientation;

<span class="c">/// A BRep edge is shared: each of the two faces meeting there uses the same edge, once from each side.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgeUse {
    <span class="k">pub</span> edge: usize,                  <span class="c">// edge index</span>
    <span class="k">pub</span> face: usize,                  <span class="c">// face index</span>
    <span class="k">pub</span> orientation: BRepOrientation, <span class="c">// which way the face runs it</span>
}

<span class="c">/// The mesh vertices one BRep edge runs along.</span>
<span class="k">pub</span> <span class="k">struct</span> EdgeChain {
    <span class="k">pub</span> edge: usize,          <span class="c">// BRep edge index</span>
    <span class="k">pub</span> face: usize,          <span class="c">// face mesh the keys belong to</span>
    <span class="k">pub</span> keys: Vec&lt;usize&gt;,     <span class="c">// vertex keys along the edge</span>
    <span class="k">pub</span> other: Option&lt;usize&gt;, <span class="c">// the face on the other side</span>
}</code></pre></div>
<h2 id="step-10-srcappwalkbreprs">Step 10 · src/app/walk/brep.rs<a class="anchor" href="#/course/06-cad-contract#step-10-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>New file: the BRep walk, shaded faces and their edges.</p>
<p><code>lessons/06/src/app/walk/brep.rs</code> · 99 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_mesh};
<span class="k">use</span> super::mesh_ink::Ink;
<span class="k">use</span> super::{Row, WalkCx};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::Instance;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
<span class="k">use</span> session_rust::{BRep, NurbsSurface, RenderMesh};

<span class="c">/// Mesh quality: 5° between samples, chord sag 0.001 of the size.</span>
<span class="k">pub</span> <span class="k">const</span> QUALITY: (f64, f64) = (<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">001</span>);

<span class="c">/// Every face uploaded so far.</span>
<span class="k">struct</span> Solid {
    pos: Vec&lt;[f32; 3]&gt;, <span class="c">// vertex positions</span>
    tris: Vec&lt;u32&gt;,     <span class="c">// triangle indices into \`pos\`</span>
    bounds: AABB,       <span class="c">// box of all vertices</span>
}

<span class="c">/// Append one face mesh to the arena.</span>
<span class="k">fn</span> push_face(arena: &amp;<span class="k">mut</span> ArenaRows, rm: &amp;RenderMesh, cx: &amp;WalkCx, solid: &amp;<span class="k">mut</span> Solid) {
    <span class="k">let</span> base = cx.vert_base + arena.verts.len() <span class="k">as</span> u32; <span class="c">// first GPU vertex index</span>
    <span class="k">let</span> local = solid.pos.len() <span class="k">as</span> u32; <span class="c">// first index in \`solid\`</span>
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());

    <span class="k">for</span> v <span class="k">in</span> &amp;rm.vertices {
        solid.bounds.union_with_point(
            v.position[<span class="s">0</span>] <span class="k">as</span> f64,
            v.position[<span class="s">1</span>] <span class="k">as</span> f64,
            v.position[<span class="s">2</span>] <span class="k">as</span> f64,
        );
        solid.pos.push(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }

    arena.idx.reserve(rm.indices.len());

    <span class="k">for</span> &amp;i <span class="k">in</span> &amp;rm.indices {
        arena.idx.push(base + i);
        solid.tris.push(local + i);
    }
}

<span class="c">/// Upload each face mesh on its own.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_brep(arena: &amp;<span class="k">mut</span> ArenaRows, _ink: &amp;<span class="k">mut</span> Ink, brep: &amp;BRep, cx: &amp;WalkCx) -&gt; Row {
    <span class="k">let</span> <span class="k">mut</span> solid = Solid {
        pos: Vec::new(),
        tris: Vec::new(),
        bounds: AABB::empty(),
    };
    <span class="k">let</span> <span class="k">mut</span> vertices = <span class="s">0</span>;

    <span class="k">for</span> <span class="k">mut</span> face <span class="k">in</span> brep.face_meshes_q(Some(QUALITY)) {
        face.set_objectcolor(brep.surfacecolor.clone());
        vertices += face.vertex.len();
        push_face(arena, &amp;face.to_render(), cx, &amp;<span class="k">mut</span> solid);
    }

    Row {
        bounds: solid.bounds,
        spacing: mesh_spacing(&amp;solid.bounds, vertices),
        flags: Instance::FLAG_SMOOTH
            | <span class="k">if</span> brep.is_solid() {
                <span class="s">0</span>
            } <span class="k">else</span> {
                Instance::FLAG_OPEN
            },
        faces: <span class="s">true</span>,
    }
}

<span class="c">/// The whole UV domain; trims come later.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_surface(
    arena: &amp;<span class="k">mut</span> ArenaRows,
    ink: &amp;<span class="k">mut</span> Ink,
    surface: &amp;NurbsSurface,
    cx: &amp;WalkCx,
) -&gt; Row {
    <span class="k">let</span> <span class="k">mut</span> mesh = RemeshNurbsSurfaceGrid::from_u_v_q(surface, <span class="s">0</span>, <span class="s">0</span>, QUALITY.<span class="s">0</span>, QUALITY.<span class="s">1</span>);

    <span class="k">if</span> <span class="k">let</span> Some(color) = surface.facecolors.first() {
        mesh.set_objectcolor(color.clone());
    }

    <span class="k">let</span> options = MeshOpts {
        sheet_lanes: <span class="s">false</span>,
        allow_open: <span class="s">true</span>,
        smooth: <span class="s">true</span>,
    };
    walk_mesh(arena, ink, &amp;mesh, &amp;MeshCx { cx, opts: &amp;options })
}</code></pre></div>
<h2 id="step-11-srcappknobsrs">Step 11 · src/app/knobs.rs<a class="anchor" href="#/course/06-cad-contract#step-11-srcappknobsrs" aria-label="Link to this section">#</a></h2>
<p>Copy the file: debug switches read once from environment variables such as VIEWER_PROFILE.</p>
<p><code>lessons/06/src/app/knobs.rs</code> · 54 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> std::sync::OnceLock;

<span class="c">/// The first call reads the variable into \`slot\`; every later call only reads \`slot\`.</span>
<span class="k">fn</span> env_flag(name: &amp;str, slot: &amp;'static OnceLock&lt;bool&gt;) -&gt; bool {
    *slot.get_or_init(|| read_environment_flag(name))
}

<span class="c">/// Set at all counts, even VIEWER_PROFILE=0; the browser has no environment, so there it is always false.</span>
<span class="k">fn</span> read_environment_flag(name: &amp;str) -&gt; bool {
    std::env::var(name).is_ok()
}

<span class="c">// OnceLock = a value filled on first use, then read-only; that makes it safe in a \`static\`.</span>
<span class="k">static</span> PROFILE: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="k">static</span> DROP_SESSIONS: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="k">static</span> NO_EDGES: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="k">static</span> NO_DOTS: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="k">static</span> ALL_EDGES: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="k">static</span> SEAMS: OnceLock&lt;bool&gt; = OnceLock::new();

<span class="c">/// VIEWER_PROFILE: print timings.</span>
<span class="k">pub</span> <span class="k">fn</span> profile() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_PROFILE</span>&quot;, &amp;PROFILE)
}

<span class="c">/// VIEWER_DROP_SESSIONS: no longer changes anything.</span>
<span class="k">pub</span> <span class="k">fn</span> drop_sessions() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_DROP_SESSIONS</span>&quot;, &amp;DROP_SESSIONS)
}

<span class="c">/// VIEWER_NO_EDGES: faces only, no edges or dots.</span>
<span class="k">pub</span> <span class="k">fn</span> no_edges() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_NO_EDGES</span>&quot;, &amp;NO_EDGES)
}

<span class="c">/// VIEWER_NO_DOTS: edges but no vertex dots.</span>
<span class="k">pub</span> <span class="k">fn</span> no_dots() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_NO_DOTS</span>&quot;, &amp;NO_DOTS)
}

<span class="c">/// VIEWER_ALL_EDGES: also draw edges inside flat regions.</span>
<span class="k">pub</span> <span class="k">fn</span> all_edges() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_ALL_EDGES</span>&quot;, &amp;ALL_EDGES)
}

<span class="c">/// VIEWER_SEAMS: draw every seam of a smooth surface.</span>
<span class="k">pub</span> <span class="k">fn</span> seams() -&gt; bool {
    env_flag(&quot;<span class="s">VIEWER_SEAMS</span>&quot;, &amp;SEAMS)
}</code></pre></div>
<h2 id="step-12-srcappmodrs">Step 12 · src/app/mod.rs<a class="anchor" href="#/course/06-cad-contract#step-12-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The app module declares the walk.</p>
<p><code>lessons/06/src/app/mod.rs</code> · edit · type this</p>
<p>Replaces the line <code>pub mod route;</code> in <code>lessons/05/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> knobs;
<span class="k">pub</span> <span class="k">mod</span> route;
<span class="k">pub</span> <span class="k">mod</span> walk;</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/06/</code>.</p>
<h2 id="step-13-srcfixturers">Step 13 · src/fixture.rs<a class="anchor" href="#/course/06-cad-contract#step-13-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: CAD surfaces from the kernel.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/06/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the <code>fn scene</code> lines in <code>lessons/05/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::walk::brep::{walk_brep, walk_surface};
<span class="k">use</span> <span class="k">crate</span>::app::walk::mesh_ink::Ink;
<span class="k">use</span> <span class="k">crate</span>::app::walk::{Row, WalkCx};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::Upload;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::objects::ObjectRow;
<span class="k">use</span> session_rust::{Color, Geometry, NurbsSurface, Point, Xform};
<span class="k">use</span> std::rc::Rc;

<span class="c">/// The f64 source objects, kept beside their rows.</span>
<span class="k">pub</span> <span class="k">struct</span> CadFixture {
    <span class="k">pub</span> upload: Upload, <span class="c">// the lane tables for the GPU</span>
    <span class="k">pub</span> sources: Vec&lt;Geometry&gt;, <span class="c">// the f64 source objects</span>
    <span class="k">pub</span> identities: Vec&lt;SourceIdentity&gt;, <span class="c">// row and GUID per object</span>
    <span class="k">pub</span> pipe_source_edges: Vec&lt;u32&gt;, <span class="c">// edges drawn as pipes</span>
}

<span class="c">/// Row = GPU address; GUID = the source object.</span>
#[derive(serde::Serialize)]
<span class="k">pub</span> <span class="k">struct</span> SourceIdentity {
    <span class="k">pub</span> object_row: u32, <span class="c">// the GPU row</span>
    <span class="k">pub</span> guid: String, <span class="c">// the source id</span>
    <span class="k">pub</span> kind: &amp;'static str, <span class="c">// mesh, brep or surface</span>
}

<span class="k">impl</span> CadFixture {
    <span class="c">/// Empty until the chosen test scene adds its objects.</span>
    <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            upload: Upload::default(),
            sources: Vec::new(),
            identities: Vec::new(),
            pipe_source_edges: Vec::new(),
        }
    }

    <span class="c">/// Prepare one source object and keep it.</span>
    <span class="k">fn</span> add(&amp;<span class="k">mut</span> <span class="k">self</span>, geometry: Geometry, place: Xform) {
        <span class="k">let</span> row = <span class="k">self</span>.upload.obj.rows.len() <span class="k">as</span> u32;
        <span class="k">let</span> first_pipe = <span class="k">self</span>.upload.seg.pipes.len();
        <span class="k">let</span> context = WalkCx {
            vert_base: <span class="s">0</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row,
        };
        <span class="k">let</span> <span class="k">mut</span> ink = Ink {
            seg: &amp;<span class="k">mut</span> <span class="k">self</span>.upload.seg,
            glyph: &amp;<span class="k">mut</span> <span class="k">self</span>.upload.glyph,
        };
        <span class="k">let</span> (prepared, guid, kind) = <span class="k">match</span> &amp;geometry {
            Geometry::BRep(source) =&gt; (
                walk_brep(&amp;<span class="k">mut</span> <span class="k">self</span>.upload.arena, &amp;<span class="k">mut</span> ink, source, &amp;context),
                source.guid().to_string(),
                &quot;<span class="s">BRep</span>&quot;,
            ),
            Geometry::NurbsSurface(source) =&gt; (
                walk_surface(&amp;<span class="k">mut</span> <span class="k">self</span>.upload.arena, &amp;<span class="k">mut</span> ink, source, &amp;context),
                source.guid().to_string(),
                &quot;<span class="s">NurbsSurface</span>&quot;,
            ),
            _ =&gt; unreachable!(&quot;<span class="s">the CAD fixture only constructs BRep and surface source objects</span>&quot;),
        };
        <span class="k">self</span>.pipe_source_edges
            .extend_from_slice(&amp;<span class="k">self</span>.upload.seg.pipe_ids[first_pipe..]);
        <span class="k">self</span>.push_row(prepared, place);
        <span class="k">self</span>.identities.push(SourceIdentity {
            object_row: row,
            guid,
            kind,
        });
        <span class="k">self</span>.sources.push(geometry);
    }

    <span class="c">/// Local bounds and placement stay separate.</span>
    <span class="k">fn</span> push_row(&amp;<span class="k">mut</span> <span class="k">self</span>, prepared: Row, place: Xform) {
        <span class="k">let</span> <span class="k">mut</span> object = ObjectRow::new(place.clone(), prepared.flags);
        object.bounds = prepared.bounds;
        object.spacing = prepared.spacing;
        object.faces = prepared.faces;
        <span class="k">self</span>.upload
            .bounds
            .union_with(&amp;prepared.bounds.transformed(&amp;place));
        <span class="k">self</span>.upload.obj.rows.push(object);
    }
}

<span class="c">/// A flat surface with its four UV borders.</span>
<span class="k">fn</span> planar_surface() -&gt; NurbsSurface {
    <span class="k">let</span> points = [
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, -<span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, <span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, -<span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, <span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
    ];
    <span class="k">let</span> <span class="k">mut</span> surface = NurbsSurface::create(<span class="s">false</span>, <span class="s">false</span>, <span class="s">1</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">2</span>, &amp;points).unwrap();
    surface.facecolors = vec![Color::grey()];
    surface.name = &quot;<span class="s">local source face</span>&quot;.into();
    surface
}

<span class="c">/// Build the first source-face checkpoint entirely from local geometry.</span>
<span class="k">pub</span> <span class="k">fn</span> build() -&gt; CadFixture {
    <span class="k">let</span> <span class="k">mut</span> scene = CadFixture::new();
    scene.add(
        Geometry::NurbsSurface(Rc::new(planar_surface())),
        Xform::identity(),
    );
    scene</code></pre></div>
<h2 id="step-14-srclibrs">Step 14 · src/lib.rs<a class="anchor" href="#/course/06-cad-contract#step-14-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The entry point runs the walk instead of the fixture.</p>
<p><code>lessons/06/src/lib.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>let mut upload = fixture::scene();</code> to <code>camera.unit = camera::Unit::Meters;</code> in <code>lessons/05/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    fixture: fixture::CadFixture,
}

#[wasm_bindgen] <span class="c">// Everything in this block is exported to JavaScript</span>
<span class="k">impl</span> Tutorial {
    <span class="c">/// Negotiate a presentation compatible browser adapter and build the first pipeline.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> create(canvas: web_sys::HtmlCanvasElement) -&gt; Result&lt;Tutorial, JsValue&gt; {
        console_error_panic_hook::set_once(); <span class="c">// Better error messages in the console</span>
        <span class="k">let</span> <span class="k">mut</span> gpu = Gpu::new(canvas.clone()).<span class="k">await</span>.map_err(js_error)?;
        <span class="k">let</span> <span class="k">mut</span> fixture = fixture::build();
        gpu.set_scene(&amp;fixture.upload);
        fixture.upload.drop_uploaded();
        <span class="c">// fit() picks the distance where this box fills the view, so the triangle is framed at startup.</span>
        <span class="k">let</span> <span class="k">mut</span> camera = camera::Camera::new();
        camera.unit = camera::Unit::Millimeters;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            scale: <span class="s">1</span>.<span class="s">0</span>,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            fixture,</code></pre></div>
<p>Replaces the line <code>Ok(serde_json::json!({&quot;stage&quot;:5,&quot;objects&quot;:self.gpu.object…</code> in <code>lessons/05/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">6</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<h2 id="step-15-srcshaderstrianglewgsl">Step 15 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/06-cad-contract#step-15-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>The mesh shader reads the packed facing.</p>
<p><code>lessons/06/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>o.normal = face_normal(inst.model, in.normal);</code> in <code>lessons/05/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // Flat shading for now; source normals stay in the arena.</span>
    o.normal = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);</code></pre></div>
<h2 id="step-16-indexhtml">Step 16 · index.html<a class="anchor" href="#/course/06-cad-contract#step-16-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status says checkpoint 06.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/06/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 05&lt;/title&gt;</code> in <code>lessons/05/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 06&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 05&lt;/output&gt;</code> in <code>lessons/05/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 06&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/05/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 06 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/06-cad-contract#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/06/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A shaded planar face shows four black boundaries with retained source identities; status: <strong>1 object</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/06.png" alt="Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The face is missing: the producer, upload or arena draw range is empty.</li>
<li>Boundaries float off the face: the two f64 to f32 conversions differ.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/06-cad-contract#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/06/src/app/
├── walk/
│   ├── bounds.rs  +
│   ├── brep.rs  +
│   ├── brep_edges.rs  +
│   ├── curves.rs  +
│   ├── encode.rs  +
│   ├── mesh.rs  +
│   ├── mesh_ink.rs  +
│   ├── mesh_topology.rs  +
│   └── mod.rs  +
├── knobs.rs  +
├── mod.rs  ~
└── route.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/06/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/06-cad-contract#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/07-boundaries">07 · Shared boundaries</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/06-cad-contract#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.</p>
<p><a href="/session/docs/course/docs/screenshots/06.png"><img src="/session/docs/course/docs/screenshots/06.png" alt="Full viewer result for 06 cad contract" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-session_rustsrcremesh_nurbssurface_gridrs",text:"Step 1 · session_rust/src/remesh_nurbssurface_grid.rs"},{level:2,id:"step-2-srcappwalkencoders",text:"Step 2 · src/app/walk/encode.rs"},{level:2,id:"step-3-srcappwalkmodrs",text:"Step 3 · src/app/walk/mod.rs"},{level:2,id:"step-4-srcappwalkboundsrs",text:"Step 4 · src/app/walk/bounds.rs"},{level:2,id:"step-5-srcappwalkmesh_topologyrs",text:"Step 5 · src/app/walk/mesh_topology.rs"},{level:2,id:"step-6-srcappwalkmesh_inkrs",text:"Step 6 · src/app/walk/mesh_ink.rs"},{level:2,id:"step-7-srcappwalkmeshrs",text:"Step 7 · src/app/walk/mesh.rs"},{level:2,id:"step-8-srcappwalkcurvesrs",text:"Step 8 · src/app/walk/curves.rs"},{level:2,id:"step-9-srcappwalkbrep_edgesrs",text:"Step 9 · src/app/walk/brep_edges.rs"},{level:2,id:"step-10-srcappwalkbreprs",text:"Step 10 · src/app/walk/brep.rs"},{level:2,id:"step-11-srcappknobsrs",text:"Step 11 · src/app/knobs.rs"},{level:2,id:"step-12-srcappmodrs",text:"Step 12 · src/app/mod.rs"},{level:2,id:"step-13-srcfixturers",text:"Step 13 · src/fixture.rs"},{level:2,id:"step-14-srclibrs",text:"Step 14 · src/lib.rs"},{level:2,id:"step-15-srcshaderstrianglewgsl",text:"Step 15 · src/shaders/triangle.wgsl"},{level:2,id:"step-16-indexhtml",text:"Step 16 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
