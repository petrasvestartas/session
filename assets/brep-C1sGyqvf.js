const s={title:"session_rust/src/brep.rs",html:`<h1 id="session_rustsrcbreprs">session_rust/src/brep.rs<a class="anchor" href="#/course/kernel/brep#session_rustsrcbreprs" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::color::Color;
<span class="k">use</span> <span class="k">crate</span>::mesh::Mesh;
<span class="k">use</span> <span class="k">crate</span>::nurbscurve::NurbsCurve;
<span class="k">use</span> <span class="k">crate</span>::nurbssurface::NurbsSurface;
<span class="k">use</span> <span class="k">crate</span>::nurbssurface_trimmed::NurbsSurfaceTrimmed;
<span class="k">use</span> <span class="k">crate</span>::nurbssurface_trimmed::TrimLoops;
<span class="k">use</span> <span class="k">crate</span>::plane::Plane;
<span class="k">use</span> <span class="k">crate</span>::point::Point;
<span class="k">use</span> <span class="k">crate</span>::polyline::Polyline;
<span class="k">use</span> <span class="k">crate</span>::primitives::Primitives;
<span class="k">use</span> <span class="k">crate</span>::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
<span class="k">use</span> <span class="k">crate</span>::tolerance::Tolerance;
<span class="k">use</span> <span class="k">crate</span>::tolerance::PI;
<span class="k">use</span> <span class="k">crate</span>::vector::Vector;
<span class="k">use</span> <span class="k">crate</span>::xform::Xform;
<span class="k">use</span> serde::ser::SerializeMap;
<span class="k">use</span> serde::Deserialize;
<span class="k">use</span> serde::Deserializer;
<span class="k">use</span> serde::Serialize;
<span class="k">use</span> serde::Serializer;
<span class="k">use</span> std::cmp::Ordering;
<span class="k">use</span> std::collections::BTreeMap;
<span class="k">use</span> std::collections::HashMap;

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Orientation</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// TopAbs_Orientation: carried by the parent -&gt; child reference, never by the shape</span>
#[derive(Debug, Clone, Copy, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> BRepOrientation {
    Forward = <span class="s">0</span>,  <span class="c">// Same direction as the shape.</span>
    Reversed = <span class="s">1</span>, <span class="c">// Opposite direction.</span>
    Internal = <span class="s">2</span>, <span class="c">// Inside the parent, both sides.</span>
    External = <span class="s">3</span>, <span class="c">// Outside the parent, no side.</span>
}

<span class="c">/// TopAbs::Reverse</span>
<span class="k">pub</span> <span class="k">fn</span> brep_reverse(o: BRepOrientation) -&gt; BRepOrientation {
    <span class="k">if</span> o == BRepOrientation::Forward {
        <span class="k">return</span> BRepOrientation::Reversed;
    }

    <span class="k">if</span> o == BRepOrientation::Reversed {
        <span class="k">return</span> BRepOrientation::Forward;
    }

    o
}

<span class="c">/// TopAbs::Compose: the orientation of a sub-shape reached through a parent with orientation \`a\`</span>
<span class="k">pub</span> <span class="k">fn</span> brep_compose(a: BRepOrientation, b: BRepOrientation) -&gt; BRepOrientation {
    <span class="k">if</span> a == BRepOrientation::Internal || a == BRepOrientation::External {
        <span class="k">return</span> a;
    }

    <span class="k">if</span> a == BRepOrientation::Forward {
        <span class="k">return</span> b;
    }

    brep_reverse(b)
}

<span class="k">const</span> F: BRepOrientation = BRepOrientation::Forward;
<span class="k">const</span> R: BRepOrientation = BRepOrientation::Reversed;

<span class="c">/// JSON name of an orientation</span>
<span class="k">fn</span> orientation_to_str(o: BRepOrientation) -&gt; &amp;'static str {
    <span class="k">if</span> o == BRepOrientation::Reversed {
        <span class="k">return</span> &quot;<span class="s">reversed</span>&quot;;
    }

    <span class="k">if</span> o == BRepOrientation::Internal {
        <span class="k">return</span> &quot;<span class="s">internal</span>&quot;;
    }

    <span class="k">if</span> o == BRepOrientation::External {
        <span class="k">return</span> &quot;<span class="s">external</span>&quot;;
    }

    &quot;<span class="s">forward</span>&quot;
}

<span class="c">/// Orientation of a JSON name, Forward when unknown</span>
<span class="k">fn</span> orientation_from_str(s: &amp;str) -&gt; BRepOrientation {
    <span class="k">if</span> s == &quot;<span class="s">reversed</span>&quot; {
        <span class="k">return</span> BRepOrientation::Reversed;
    }

    <span class="k">if</span> s == &quot;<span class="s">internal</span>&quot; {
        <span class="k">return</span> BRepOrientation::Internal;
    }

    <span class="k">if</span> s == &quot;<span class="s">external</span>&quot; {
        <span class="k">return</span> BRepOrientation::External;
    }

    BRepOrientation::Forward
}

<span class="c">/// Orientation of a proto enum value, Forward when unknown</span>
<span class="k">fn</span> orientation_from_i32(v: i32) -&gt; BRepOrientation {
    <span class="k">if</span> v == <span class="s">1</span> {
        <span class="k">return</span> BRepOrientation::Reversed;
    }

    <span class="k">if</span> v == <span class="s">2</span> {
        <span class="k">return</span> BRepOrientation::Internal;
    }

    <span class="k">if</span> v == <span class="s">3</span> {
        <span class="k">return</span> BRepOrientation::External;
    }

    BRepOrientation::Forward
}

<span class="c">/// True when \`index\` addresses one of \`count\` table entries</span>
<span class="k">fn</span> in_range(index: i32, count: usize) -&gt; bool {
    index &gt;= <span class="s">0</span> &amp;&amp; (index <span class="k">as</span> usize) &lt; count
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Shapes</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// TopoDS_Shape: an oriented reference to a sub-shape (index into the owning table)</span>
#[derive(Debug, Clone, Copy, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepRef {
    <span class="k">pub</span> index: i32,                   <span class="c">// Index into the owning table.</span>
    <span class="k">pub</span> orientation: BRepOrientation, <span class="c">// Orientation of this use.</span>
}

<span class="k">impl</span> BRepRef {
    <span class="c">/// Reference to entry \`index\` used with \`orientation\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(index: i32, orientation: BRepOrientation) -&gt; <span class="k">Self</span> {
        BRepRef { index, orientation }
    }
}

<span class="c">/// BRep_TVertex</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepVertex {
    <span class="k">pub</span> point: Point,   <span class="c">// Position.</span>
    <span class="k">pub</span> tolerance: f64, <span class="c">// Vertex tolerance.</span>
}

<span class="c">/// BRep_CurveOnSurface: curve_2d_index_2 is the pcurve of the REVERSED use on a closed surface (seam), -1 otherwise; pcurves run in the edge's own direction</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepCurveOnSurface {
    <span class="k">pub</span> surface_index: i32,    <span class="c">// Surface the pcurve lies on.</span>
    <span class="k">pub</span> curve_2d_index: i32,   <span class="c">// Pcurve of the forward use.</span>
    <span class="k">pub</span> curve_2d_index_2: i32, <span class="c">// Reversed use on a seam, else -1.</span>
}

<span class="c">/// BRep_TEdge: curve_3d_index is -1 for a degenerated edge (sphere pole, cone apex)</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepEdge {
    <span class="k">pub</span> curve_3d_index: i32,              <span class="c">// 3D curve, -1 when degenerated.</span>
    <span class="k">pub</span> start_vertex: i32,                <span class="c">// Start vertex.</span>
    <span class="k">pub</span> end_vertex: i32,                  <span class="c">// End vertex.</span>
    <span class="k">pub</span> tolerance: f64,                   <span class="c">// Edge tolerance.</span>
    <span class="k">pub</span> degenerated: bool,                <span class="c">// True for a pole or apex edge.</span>
    <span class="k">pub</span> pcurves: Vec&lt;BRepCurveOnSurface&gt;, <span class="c">// One per surface.</span>
}

<span class="c">/// TopoDS_TWire</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepWire {
    <span class="k">pub</span> edges: Vec&lt;BRepRef&gt;, <span class="c">// Traversal order.</span>
}

<span class="c">/// BRep_TFace: the first wire is the outer boundary; facecolor None means unset</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepFace {
    <span class="k">pub</span> surface_index: i32,       <span class="c">// Underlying surface.</span>
    <span class="k">pub</span> wires: Vec&lt;BRepRef&gt;,      <span class="c">// Outer wire first, then holes.</span>
    <span class="k">pub</span> tolerance: f64,           <span class="c">// Face tolerance.</span>
    <span class="k">pub</span> facecolor: Option&lt;Color&gt;, <span class="c">// Display color, None when unset.</span>
}

<span class="c">/// TopoDS_TShell</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepShell {
    <span class="k">pub</span> faces: Vec&lt;BRepRef&gt;, <span class="c">// Oriented faces.</span>
}

<span class="c">/// TopoDS_TSolid</span>
#[derive(Debug, Clone, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> BRepSolid {
    <span class="k">pub</span> shells: Vec&lt;BRepRef&gt;, <span class="c">// Outer first.</span>
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Geometry helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Bilinear planar patch: u runs p00 -&gt; p10, v runs p00 -&gt; p01, natural normal = u x v</span>
<span class="k">fn</span> bilinear_patch(p00: &amp;Point, p10: &amp;Point, p01: &amp;Point, p11: &amp;Point) -&gt; NurbsSurface {
    <span class="k">let</span> <span class="k">mut</span> srf = NurbsSurface::new(<span class="s">3</span>, <span class="s">false</span>, <span class="s">2</span>, <span class="s">2</span>, <span class="s">2</span>, <span class="s">2</span>);

    <span class="k">if</span> !srf.set_cv(<span class="s">0</span>, <span class="s">0</span>, p00)
        || !srf.set_cv(<span class="s">1</span>, <span class="s">0</span>, p10)
        || !srf.set_cv(<span class="s">0</span>, <span class="s">1</span>, p01)
        || !srf.set_cv(<span class="s">1</span>, <span class="s">1</span>, p11)
    {
        <span class="k">return</span> NurbsSurface::default();
    }

    srf
}

<span class="c">/// Straight pcurve from (u0, v0) to (u1, v1)</span>
<span class="k">fn</span> uv_line(u0: f64, v0: f64, u1: f64, v1: f64) -&gt; NurbsCurve {
    NurbsCurve::create(
        <span class="s">false</span>,
        <span class="s">1</span>,
        &amp;[Point::new(u0, v0, <span class="s">0</span>.<span class="s">0</span>), Point::new(u1, v1, <span class="s">0</span>.<span class="s">0</span>)],
    )
}

<span class="c">/// Exact pcurve of a 3D curve lying on a bilinear planar patch: the affine image of its CVs</span>
<span class="k">fn</span> project_to_patch(crv: &amp;NurbsCurve, srf: &amp;NurbsSurface) -&gt; NurbsCurve {
    <span class="k">let</span> p00 = srf.get_cv(<span class="s">0</span>, <span class="s">0</span>).unwrap_or_default();
    <span class="k">let</span> eu = &amp;srf.get_cv(<span class="s">1</span>, <span class="s">0</span>).unwrap_or_default() - &amp;p00;
    <span class="k">let</span> ev = &amp;srf.get_cv(<span class="s">0</span>, <span class="s">1</span>).unwrap_or_default() - &amp;p00;
    <span class="k">let</span> eu2 = eu.dot(&amp;eu);
    <span class="k">let</span> ev2 = ev.dot(&amp;ev);
    <span class="k">let</span> <span class="k">mut</span> c2 = NurbsCurve::new(<span class="s">3</span>, crv.is_rational(), crv.order(), crv.cv_count());

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..crv.nurbsknot_count() {
        <span class="k">if</span> !c2.set_nurbsknot(i, crv.nurbsknot(i).unwrap_or(<span class="s">0</span>.<span class="s">0</span>)) {
            <span class="k">return</span> NurbsCurve::default();
        }
    }

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..crv.cv_count() {
        <span class="k">let</span> (wx, wy, wz, w) = crv.get_cv_4d(i).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> d = &amp;Point::new(wx / w, wy / w, wz / w) - &amp;p00;
        <span class="k">let</span> u = d.dot(&amp;eu) / eu2;
        <span class="k">let</span> v = d.dot(&amp;ev) / ev2;

        <span class="k">let</span> written = <span class="k">if</span> crv.is_rational() {
            c2.set_cv_4d(i, u * w, v * w, <span class="s">0</span>.<span class="s">0</span>, w)
        } <span class="k">else</span> {
            c2.set_cv(i, &amp;Point::new(u, v, <span class="s">0</span>.<span class="s">0</span>))
        };

        <span class="k">if</span> !written {
            <span class="k">return</span> NurbsCurve::default();
        }
    }

    c2
}

<span class="c">/// Signed area of a closed pcurve's sampled polygon (positive = counter-clockwise)</span>
<span class="k">fn</span> uv_signed_area(c2d: &amp;NurbsCurve) -&gt; f64 {
    <span class="k">let</span> pts = c2d.divide_by_count((c2d.cv_count() * <span class="s">4</span>).max(<span class="s">16</span>), <span class="s">true</span>).<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> area = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..pts.len().saturating_sub(<span class="s">1</span>) {
        area += pts[i][<span class="s">0</span>] * pts[i + <span class="s">1</span>][<span class="s">1</span>] - pts[i + <span class="s">1</span>][<span class="s">0</span>] * pts[i][<span class="s">1</span>];
    }

    <span class="s">0</span>.<span class="s">5</span> * area
}

<span class="c">/// Signed area of a closed UV polygon (positive = counter-clockwise)</span>
<span class="k">fn</span> polygon_signed_area(pts: &amp;[Point]) -&gt; f64 {
    <span class="k">let</span> n = pts.len();
    <span class="k">let</span> <span class="k">mut</span> area = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> p = &amp;pts[i];
        <span class="k">let</span> q = &amp;pts[(i + <span class="s">1</span>) % n];
        area += p[<span class="s">0</span>] * q[<span class="s">1</span>] - q[<span class="s">0</span>] * p[<span class="s">1</span>];
    }

    <span class="s">0</span>.<span class="s">5</span> * area
}

<span class="c">/// Diagonal of the control point bounding box</span>
<span class="k">fn</span> bbox_diagonal(srf: &amp;NurbsSurface) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> lo = Point::new(<span class="s">1</span>e<span class="s">30</span>, <span class="s">1</span>e<span class="s">30</span>, <span class="s">1</span>e<span class="s">30</span>);
    <span class="k">let</span> <span class="k">mut</span> hi = Point::new(-<span class="s">1</span>e<span class="s">30</span>, -<span class="s">1</span>e<span class="s">30</span>, -<span class="s">1</span>e<span class="s">30</span>);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..srf.cv_count(<span class="s">0</span>) {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..srf.cv_count(<span class="s">1</span>) {
            <span class="k">let</span> p = srf.get_cv(i, j).unwrap_or_default();

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
    }

    hi.distance(&amp;lo, None)
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Factory helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Planar polygon faces from a vertex table: edges run lo -&gt; hi vertex and are shared, a face lists its vertices counter-clockwise seen from outside so the patch normal points outward</span>
<span class="k">struct</span> PolyFaceBuilder {
    edge_map: HashMap&lt;(usize, usize), usize&gt;, <span class="c">// Edge per (lo, hi) pair.</span>
}

<span class="k">impl</span> PolyFaceBuilder {
    <span class="c">/// Builder with no edges yet</span>
    <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        PolyFaceBuilder {
            edge_map: HashMap::new(),
        }
    }

    <span class="c">/// Straight edge between two vertices, shared by every face that uses it</span>
    <span class="k">fn</span> edge(&amp;<span class="k">mut</span> <span class="k">self</span>, b: &amp;<span class="k">mut</span> BRep, v0: usize, v1: usize) -&gt; usize {
        <span class="k">let</span> lo = v0.min(v1);
        <span class="k">let</span> hi = v0.max(v1);

        <span class="k">if</span> <span class="k">let</span> Some(&amp;ei) = <span class="k">self</span>.edge_map.get(&amp;(lo, hi)) {
            <span class="k">return</span> ei;
        }

        <span class="k">let</span> line = NurbsCurve::create(
            <span class="s">false</span>,
            <span class="s">1</span>,
            &amp;[
                b.m_vertices[lo].point.clone(),
                b.m_vertices[hi].point.clone(),
            ],
        );
        <span class="k">let</span> ci = b.add_curve_3d(&amp;line);
        <span class="k">let</span> ei = b.add_edge(ci <span class="k">as</span> i32, lo <span class="k">as</span> i32, hi <span class="k">as</span> i32);
        <span class="k">self</span>.edge_map.insert((lo, hi), ei);

        ei
    }

    <span class="c">/// Oriented edge references of the vertex cycle \`vi\`, each with its pcurve on surface \`si\`</span>
    <span class="k">fn</span> wire_refs(&amp;<span class="k">mut</span> <span class="k">self</span>, b: &amp;<span class="k">mut</span> BRep, si: usize, vi: &amp;[usize]) -&gt; Vec&lt;BRepRef&gt; {
        <span class="k">let</span> n = vi.len();
        <span class="k">let</span> <span class="k">mut</span> refs = Vec::new();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
            <span class="k">let</span> va = vi[i];
            <span class="k">let</span> vb = vi[(i + <span class="s">1</span>) % n];
            <span class="k">let</span> ei = <span class="k">self</span>.edge(b, va, vb);
            <span class="k">let</span> c2d = project_to_patch(
                &amp;b.m_curves_3d[b.m_edges[ei].curve_3d_index <span class="k">as</span> usize],
                &amp;b.m_surfaces[si],
            );
            <span class="k">let</span> ci = b.add_curve_2d(&amp;c2d);
            b.add_pcurve(ei, si, ci <span class="k">as</span> i32, -<span class="s">1</span>);
            <span class="k">let</span> o = <span class="k">if</span> b.m_edges[ei].start_vertex == va <span class="k">as</span> i32 {
                F
            } <span class="k">else</span> {
                R
            };
            refs.push(BRepRef::new(ei <span class="k">as</span> i32, o));
        }

        refs
    }

    <span class="c">/// Face on \`srf\` bounded by the vertex cycle \`vi\`, with one inner wire per hole cycle; returns the face index</span>
    <span class="k">fn</span> face(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        b: &amp;<span class="k">mut</span> BRep,
        srf: &amp;NurbsSurface,
        vi: &amp;[usize],
        holes: &amp;[Vec&lt;usize&gt;],
    ) -&gt; usize {
        <span class="k">let</span> si = b.add_surface(srf);
        <span class="k">let</span> refs = <span class="k">self</span>.wire_refs(b, si, vi);
        <span class="k">let</span> <span class="k">mut</span> wires = vec![BRepRef::new(b.add_wire(&amp;refs) <span class="k">as</span> i32, F)];

        <span class="k">for</span> hole <span class="k">in</span> holes {
            <span class="k">let</span> hole_refs = <span class="k">self</span>.wire_refs(b, si, hole);
            wires.push(BRepRef::new(b.add_wire(&amp;hole_refs) <span class="k">as</span> i32, F));
        }

        b.add_face(si <span class="k">as</span> i32, &amp;wires, <span class="s">0</span>.<span class="s">0</span>)
    }
}

<span class="k">const</span> BOX_FACES: [[usize; <span class="s">4</span>]; <span class="s">6</span>] = [
    [<span class="s">0</span>, <span class="s">3</span>, <span class="s">2</span>, <span class="s">1</span>],
    [<span class="s">4</span>, <span class="s">5</span>, <span class="s">6</span>, <span class="s">7</span>],
    [<span class="s">0</span>, <span class="s">1</span>, <span class="s">5</span>, <span class="s">4</span>],
    [<span class="s">1</span>, <span class="s">2</span>, <span class="s">6</span>, <span class="s">5</span>],
    [<span class="s">2</span>, <span class="s">3</span>, <span class="s">7</span>, <span class="s">6</span>],
    [<span class="s">3</span>, <span class="s">0</span>, <span class="s">4</span>, <span class="s">7</span>],
];

<span class="c">/// Bilinear patch spanned by four vertex indices in face order (p00, p10, p11, p01)</span>
<span class="k">fn</span> quad_patch(b: &amp;BRep, fv: &amp;[usize; <span class="s">4</span>]) -&gt; NurbsSurface {
    bilinear_patch(
        &amp;b.m_vertices[fv[<span class="s">0</span>]].point,
        &amp;b.m_vertices[fv[<span class="s">1</span>]].point,
        &amp;b.m_vertices[fv[<span class="s">3</span>]].point,
        &amp;b.m_vertices[fv[<span class="s">2</span>]].point,
    )
}

<span class="c">/// The eight corners of an origin-centered box, bottom ring then top ring</span>
<span class="k">fn</span> box_corners(b: &amp;<span class="k">mut</span> BRep, sx: f64, sy: f64, sz: f64) {
    <span class="k">let</span> hx = sx * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> hy = sy * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> hz = sz * <span class="s">0</span>.<span class="s">5</span>;

    b.add_vertex(&amp;Point::new(-hx, -hy, -hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(hx, -hy, -hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(hx, hy, -hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(-hx, hy, -hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(-hx, -hy, hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(hx, -hy, hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(hx, hy, hz), <span class="s">0</span>.<span class="s">0</span>);
    b.add_vertex(&amp;Point::new(-hx, hy, hz), <span class="s">0</span>.<span class="s">0</span>);
}

<span class="c">/// Planar cap at height z with natural normal +Z (up) or -Z (down), spanning [-r, r]^2</span>
<span class="k">fn</span> cap_patch(r: f64, z: f64, up: bool) -&gt; NurbsSurface {
    <span class="k">if</span> up {
        <span class="k">return</span> bilinear_patch(
            &amp;Point::new(-r, -r, z),
            &amp;Point::new(r, -r, z),
            &amp;Point::new(-r, r, z),
            &amp;Point::new(r, r, z),
        );
    }

    bilinear_patch(
        &amp;Point::new(-r, -r, z),
        &amp;Point::new(-r, r, z),
        &amp;Point::new(r, -r, z),
        &amp;Point::new(r, r, z),
    )
}

<span class="c">/// Cap face bounded by one closed edge: outer wire counter-clockwise in the patch's UV</span>
<span class="k">fn</span> cap_face(b: &amp;<span class="k">mut</span> BRep, cap: &amp;NurbsSurface, edge: usize) -&gt; usize {
    <span class="k">let</span> si = b.add_surface(cap);
    <span class="k">let</span> c2d = project_to_patch(&amp;b.m_curves_3d[b.m_edges[edge].curve_3d_index <span class="k">as</span> usize], cap);
    <span class="k">let</span> o = <span class="k">if</span> uv_signed_area(&amp;c2d) &gt; <span class="s">0</span>.<span class="s">0</span> { F } <span class="k">else</span> { R };
    <span class="k">let</span> ci = b.add_curve_2d(&amp;c2d);
    b.add_pcurve(edge, si, ci <span class="k">as</span> i32, -<span class="s">1</span>);
    <span class="k">let</span> wi = b.add_wire(&amp;[BRepRef::new(edge <span class="k">as</span> i32, o)]);

    b.add_face(si <span class="k">as</span> i32, &amp;[BRepRef::new(wi <span class="k">as</span> i32, F)], <span class="s">0</span>.<span class="s">0</span>)
}

<span class="c">/// Periodic body face (cylinder / cone / bore): seam from v0 to v1 at u0 == u1, bottom ring forward at v0, top ring (or degenerated apex) reversed at v1</span>
<span class="k">fn</span> body_face(b: &amp;<span class="k">mut</span> BRep, si: usize, e_bot: usize, e_seam: usize, e_top: usize) -&gt; usize {
    <span class="k">let</span> (u0, u1) = b.m_surfaces[si].domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = b.m_surfaces[si].domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));

    <span class="k">let</span> c_bot = b.add_curve_2d(&amp;uv_line(u0, v0, u1, v0));
    b.add_pcurve(e_bot, si, c_bot <span class="k">as</span> i32, -<span class="s">1</span>);
    <span class="k">let</span> c_top = b.add_curve_2d(&amp;uv_line(u0, v1, u1, v1));
    b.add_pcurve(e_top, si, c_top <span class="k">as</span> i32, -<span class="s">1</span>);
    <span class="k">let</span> c_right = b.add_curve_2d(&amp;uv_line(u1, v0, u1, v1));
    <span class="k">let</span> c_left = b.add_curve_2d(&amp;uv_line(u0, v0, u0, v1));
    b.add_pcurve(e_seam, si, c_right <span class="k">as</span> i32, c_left <span class="k">as</span> i32);
    <span class="k">let</span> wi = b.add_wire(&amp;[
        BRepRef::new(e_bot <span class="k">as</span> i32, F),
        BRepRef::new(e_seam <span class="k">as</span> i32, F),
        BRepRef::new(e_top <span class="k">as</span> i32, R),
        BRepRef::new(e_seam <span class="k">as</span> i32, R),
    ]);

    b.add_face(si <span class="k">as</span> i32, &amp;[BRepRef::new(wi <span class="k">as</span> i32, F)], <span class="s">0</span>.<span class="s">0</span>)
}

<span class="c">/// Point of the plane (org, xa, ya) at (u, v)</span>
<span class="k">fn</span> plane_point(org: &amp;Point, xa: &amp;Vector, ya: &amp;Vector, u: f64, v: f64) -&gt; Point {
    org + xa * u + ya * v
}

<span class="c">/// Padded bilinear patch through \`pts\` in the plane (org, xa, ya)</span>
<span class="k">fn</span> planar_patch_through(pts: &amp;[Point], org: &amp;Point, xa: &amp;Vector, ya: &amp;Vector) -&gt; NurbsSurface {
    <span class="k">let</span> <span class="k">mut</span> umin: f64 = <span class="s">1</span>e<span class="s">30</span>;
    <span class="k">let</span> <span class="k">mut</span> umax: f64 = -<span class="s">1</span>e<span class="s">30</span>;
    <span class="k">let</span> <span class="k">mut</span> vmin: f64 = <span class="s">1</span>e<span class="s">30</span>;
    <span class="k">let</span> <span class="k">mut</span> vmax: f64 = -<span class="s">1</span>e<span class="s">30</span>;

    <span class="k">for</span> p <span class="k">in</span> pts {
        <span class="k">let</span> d = p - org;
        <span class="k">let</span> u = d.dot(xa);
        <span class="k">let</span> v = d.dot(ya);
        umin = umin.min(u);
        umax = umax.max(u);
        vmin = vmin.min(v);
        vmax = vmax.max(v);
    }

    <span class="k">let</span> pad = (umax - umin).max(vmax - vmin) * <span class="s">0</span>.<span class="s">01</span>;
    umin -= pad;
    umax += pad;
    vmin -= pad;
    vmax += pad;

    bilinear_patch(
        &amp;plane_point(org, xa, ya, umin, vmin),
        &amp;plane_point(org, xa, ya, umax, vmin),
        &amp;plane_point(org, xa, ya, umin, vmax),
        &amp;plane_point(org, xa, ya, umax, vmax),
    )
}

<span class="c">/// Signed area of a closed cycle of points seen in the plane (org, xa, ya): positive when it runs counter-clockwise</span>
<span class="k">fn</span> signed_area_in_plane(pts: &amp;[Point], org: &amp;Point, xa: &amp;Vector, ya: &amp;Vector) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> area = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> n = pts.len();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> a = &amp;pts[i] - org;
        <span class="k">let</span> b = &amp;pts[(i + <span class="s">1</span>) % n] - org;
        area += a.dot(xa) * b.dot(ya) - b.dot(xa) * a.dot(ya);
    }

    area * <span class="s">0</span>.<span class="s">5</span>
}

<span class="c">/// The vertices of a polyline without the closing duplicate</span>
<span class="k">fn</span> open_points(pl: &amp;Polyline) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> pts = pl.get_points();
    <span class="k">let</span> n = <span class="k">if</span> pl.is_closed() {
        pts.len().saturating_sub(<span class="s">1</span>)
    } <span class="k">else</span> {
        pts.len()
    };

    pts[..n].to_vec()
}

<span class="c">/// Index of the first vertex within \`tol\` of \`p\`, a new vertex when none is</span>
<span class="k">fn</span> find_or_add_vertex(b: &amp;<span class="k">mut</span> BRep, p: &amp;Point, tol: f64) -&gt; usize {
    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..b.m_vertices.len() {
        <span class="k">if</span> b.m_vertices[i].point.distance(p, None) &lt; tol {
            <span class="k">return</span> i;
        }
    }

    b.add_vertex(p, <span class="s">0</span>.<span class="s">0</span>)
}

<span class="c">/// Euclidean control points of a curve, zero weights skipped</span>
<span class="k">fn</span> cv_points(c: &amp;NurbsCurve) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> pts = Vec::new();

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..c.cv_count() {
        <span class="k">let</span> (wx, wy, wz, w) = c.get_cv_4d(k).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));

        <span class="k">if</span> w != <span class="s">0</span>.<span class="s">0</span> {
            pts.push(Point::new(wx / w, wy / w, wz / w));
        }
    }

    pts
}

<span class="c">/// One-edge wire of a closed or open curve on planar surface \`si\`, sharing vertices within \`tol\`</span>
<span class="k">fn</span> curve_wire(b: &amp;<span class="k">mut</span> BRep, crv: &amp;NurbsCurve, si: usize, tol: f64) -&gt; usize {
    <span class="k">let</span> sp = crv.point_at(crv.domain().<span class="s">0</span>);
    <span class="k">let</span> ep = crv.point_at(crv.domain().<span class="s">1</span>);
    <span class="k">let</span> vs = find_or_add_vertex(b, &amp;sp, tol);
    <span class="k">let</span> ve = <span class="k">if</span> crv.is_closed() {
        vs
    } <span class="k">else</span> {
        find_or_add_vertex(b, &amp;ep, tol)
    };
    <span class="k">let</span> ci = b.add_curve_3d(crv);
    <span class="k">let</span> ei = b.add_edge(ci <span class="k">as</span> i32, vs <span class="k">as</span> i32, ve <span class="k">as</span> i32);
    <span class="k">let</span> c2 = b.add_curve_2d(&amp;project_to_patch(crv, &amp;b.m_surfaces[si]));
    b.add_pcurve(ei, si, c2 <span class="k">as</span> i32, -<span class="s">1</span>);

    b.add_wire(&amp;[BRepRef::new(ei <span class="k">as</span> i32, F)])
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Sewing helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Face keys of a mesh in ascending order</span>
<span class="k">fn</span> sorted_face_keys(mesh: &amp;Mesh) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> <span class="k">mut</span> keys: Vec&lt;usize&gt; = mesh.face.keys().copied().collect();
    keys.sort();

    keys
}

<span class="c">/// Signed volume of face meshes (positive when the windings point outward)</span>
<span class="k">fn</span> signed_volume(meshes: &amp;[Mesh]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> fm <span class="k">in</span> meshes {
        <span class="k">for</span> fk <span class="k">in</span> sorted_face_keys(fm) {
            <span class="k">let</span> fverts = &amp;fm.face[&amp;fk];

            <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..fverts.len().saturating_sub(<span class="s">1</span>) {
                <span class="k">let</span> a = fm.vertex[&amp;fverts[<span class="s">0</span>]].position();
                <span class="k">let</span> b = fm.vertex[&amp;fverts[k]].position();
                <span class="k">let</span> c = fm.vertex[&amp;fverts[k + <span class="s">1</span>]].position();
                total += a[<span class="s">0</span>] * (b[<span class="s">1</span>] * c[<span class="s">2</span>] - b[<span class="s">2</span>] * c[<span class="s">1</span>]) - a[<span class="s">1</span>] * (b[<span class="s">0</span>] * c[<span class="s">2</span>] - b[<span class="s">2</span>] * c[<span class="s">0</span>])
                    + a[<span class="s">2</span>] * (b[<span class="s">0</span>] * c[<span class="s">1</span>] - b[<span class="s">1</span>] * c[<span class="s">0</span>]);
            }
        }
    }

    total / <span class="s">6</span>.<span class="s">0</span>
}

<span class="c">/// Face uses of every edge as (face, composed orientation); empty when some edge is not used exactly twice</span>
<span class="k">fn</span> edge_uses(b: &amp;BRep) -&gt; Vec&lt;Vec&lt;(usize, BRepOrientation)&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> uses: Vec&lt;Vec&lt;(usize, BRepOrientation)&gt;&gt; = vec![Vec::new(); b.m_edges.len()];

    <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..b.face_count() {
        <span class="k">for</span> wr <span class="k">in</span> &amp;b.m_faces[fi].wires {
            <span class="k">for</span> er <span class="k">in</span> b.wire_edges(wr) {
                uses[er.index <span class="k">as</span> usize].push((fi, er.orientation));
            }
        }
    }

    <span class="k">for</span> use_ <span class="k">in</span> &amp;uses {
        <span class="k">if</span> use_.len() != <span class="s">2</span> {
            <span class="k">return</span> Vec::new();
        }
    }

    uses
}

<span class="c">/// Connected components of faces, each face oriented consistently with the neighbour it was reached from</span>
<span class="k">fn</span> face_components(
    b: &amp;BRep,
    uses: &amp;[Vec&lt;(usize, BRepOrientation)&gt;],
    fo: &amp;<span class="k">mut</span> [BRepOrientation],
) -&gt; Vec&lt;Vec&lt;usize&gt;&gt; {
    <span class="k">let</span> nf = b.face_count();
    <span class="k">let</span> <span class="k">mut</span> seen = vec![<span class="s">false</span>; nf];
    <span class="k">let</span> <span class="k">mut</span> components: Vec&lt;Vec&lt;usize&gt;&gt; = Vec::new();

    <span class="k">for</span> seed <span class="k">in</span> <span class="s">0</span>..nf {
        <span class="k">if</span> seen[seed] {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> comp = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> stack = vec![seed];
        seen[seed] = <span class="s">true</span>;

        <span class="k">while</span> <span class="k">let</span> Some(fi) = stack.pop() {
            comp.push(fi);

            <span class="k">for</span> wr <span class="k">in</span> &amp;b.m_faces[fi].wires {
                <span class="k">for</span> er <span class="k">in</span> b.wire_edges(wr) {
                    <span class="k">for</span> &amp;(g, og) <span class="k">in</span> &amp;uses[er.index <span class="k">as</span> usize] {
                        <span class="k">if</span> g == fi || seen[g] {
                            <span class="k">continue</span>;
                        }

                        fo[g] = <span class="k">if</span> og == er.orientation {
                            brep_reverse(fo[fi])
                        } <span class="k">else</span> {
                            fo[fi]
                        };
                        seen[g] = <span class="s">true</span>;
                        stack.push(g);
                    }
                }
            }
        }

        components.push(comp);
    }

    components
}

<span class="c">/// BRepBuilderAPI_Sewing + MakeSolid for free faces: when every edge is shared by exactly two face uses, one shell per connected component wound outward and one solid per shell</span>
<span class="k">fn</span> close_free_faces(b: &amp;<span class="k">mut</span> BRep) {
    <span class="k">let</span> nf = b.face_count();

    <span class="k">if</span> nf == <span class="s">0</span> {
        <span class="k">return</span>;
    }

    <span class="k">let</span> uses = edge_uses(b);

    <span class="k">if</span> uses.is_empty() {
        <span class="k">return</span>;
    }

    <span class="k">let</span> <span class="k">mut</span> fo = vec![F; nf];
    <span class="k">let</span> <span class="k">mut</span> shells = Vec::new();

    <span class="k">for</span> comp <span class="k">in</span> face_components(b, &amp;uses, &amp;<span class="k">mut</span> fo) {
        <span class="k">let</span> <span class="k">mut</span> refs = Vec::new();

        <span class="k">for</span> fi <span class="k">in</span> comp {
            refs.push(BRepRef::new(fi <span class="k">as</span> i32, fo[fi]));
        }

        shells.push(BRepRef::new(b.add_shell(&amp;refs) <span class="k">as</span> i32, F));
    }

    <span class="k">let</span> fm = b.face_meshes();

    <span class="k">for</span> sr <span class="k">in</span> shells {
        <span class="k">let</span> <span class="k">mut</span> part = Vec::new();

        <span class="k">for</span> fr <span class="k">in</span> &amp;b.m_shells[sr.index <span class="k">as</span> usize].faces {
            part.push(fm[fr.index <span class="k">as</span> usize].clone());
        }

        <span class="k">if</span> signed_volume(&amp;part) &lt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">for</span> fr <span class="k">in</span> &amp;<span class="k">mut</span> b.m_shells[sr.index <span class="k">as</span> usize].faces {
                fr.orientation = brep_reverse(fr.orientation);
            }
        }

        b.add_solid(&amp;[sr]);
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Planar face helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="k">const</span> CURVED_EDGE_SAMPLES: usize = <span class="s">16</span>; <span class="c">// Samples per curved edge of a planar face.</span>

<span class="c">/// Open outline of a face's outer wire in wire order: vertices of straight edges, samples of curved ones</span>
<span class="k">fn</span> face_outline(b: &amp;BRep, fi: usize) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> points: Vec&lt;Point&gt; = Vec::new();

    <span class="k">for</span> er <span class="k">in</span> b.wire_edges(&amp;b.m_faces[fi].wires[<span class="s">0</span>]) {
        <span class="k">if</span> er.index &lt; <span class="s">0</span> || er.index <span class="k">as</span> usize &gt;= b.m_edges.len() {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> edge = &amp;b.m_edges[er.index <span class="k">as</span> usize];

        <span class="k">if</span> edge.degenerated {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> reversed = er.orientation == BRepOrientation::Reversed;
        <span class="k">let</span> curved =
            edge.curve_3d_index &gt;= <span class="s">0</span> &amp;&amp; b.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize].degree() &gt; <span class="s">1</span>;

        <span class="k">if</span> curved {
            <span class="k">let</span> c = &amp;b.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize];
            <span class="k">let</span> (d0, d1) = c.domain();

            <span class="k">for</span> s <span class="k">in</span> <span class="s">0</span>..CURVED_EDGE_SAMPLES {
                <span class="k">let</span> u = s <span class="k">as</span> f64 / CURVED_EDGE_SAMPLES <span class="k">as</span> f64;
                <span class="k">let</span> t = <span class="k">if</span> reversed {
                    d1 + (d0 - d1) * u
                } <span class="k">else</span> {
                    d0 + (d1 - d0) * u
                };
                points.push(c.point_at(t));
            }
        } <span class="k">else</span> {
            <span class="k">let</span> start = <span class="k">if</span> reversed {
                edge.end_vertex
            } <span class="k">else</span> {
                edge.start_vertex
            };

            <span class="k">if</span> start &gt;= <span class="s">0</span> &amp;&amp; (start <span class="k">as</span> usize) &lt; b.m_vertices.len() {
                points.push(b.m_vertices[start <span class="k">as</span> usize].point.clone());
            }
        }
    }

    points
}

<span class="c">/// Signed volume enclosed by closed outlines (tetrahedra fans from the origin), positive when wound outward</span>
<span class="k">fn</span> outline_volume(polylines: &amp;[Polyline]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> pl <span class="k">in</span> polylines {
        <span class="k">let</span> pts = pl.get_points();

        <span class="k">if</span> pts.len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> p0 = &amp;pts[<span class="s">0</span>];

        <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..pts.len() - <span class="s">1</span> {
            <span class="k">let</span> p1 = &amp;pts[k];
            <span class="k">let</span> p2 = &amp;pts[k + <span class="s">1</span>];
            total += p0[<span class="s">0</span>] * (p1[<span class="s">1</span>] * p2[<span class="s">2</span>] - p1[<span class="s">2</span>] * p2[<span class="s">1</span>])
                + p0[<span class="s">1</span>] * (p1[<span class="s">2</span>] * p2[<span class="s">0</span>] - p1[<span class="s">0</span>] * p2[<span class="s">2</span>])
                + p0[<span class="s">2</span>] * (p1[<span class="s">0</span>] * p2[<span class="s">1</span>] - p1[<span class="s">1</span>] * p2[<span class="s">0</span>]);
        }
    }

    total / <span class="s">6</span>.<span class="s">0</span>
}

<span class="c">/// Outer polyline and outward plane of every planar face in one walk, so the two stay index-aligned</span>
<span class="k">fn</span> planar_faces(b: &amp;BRep) -&gt; (Vec&lt;Polyline&gt;, Vec&lt;Plane&gt;) {
    <span class="k">let</span> <span class="k">mut</span> polylines = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> planes = Vec::new();

    <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..b.face_count() {
        <span class="k">let</span> face = &amp;b.m_faces[fi];

        <span class="k">if</span> face.surface_index &lt; <span class="s">0</span> || face.wires.is_empty() {
            <span class="k">continue</span>;
        }

        <span class="k">if</span> !b.m_surfaces[face.surface_index <span class="k">as</span> usize].is_planar(None, Tolerance::ZERO_TOLERANCE) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> points = face_outline(b, fi);

        <span class="k">if</span> points.len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> origin = Point::centroid(&amp;points);
        <span class="k">let</span> <span class="k">mut</span> normal = Vector::average_normal(&amp;points);

        <span class="k">if</span> b.face_orientation(fi) == BRepOrientation::Reversed {
            normal.reverse();
        }

        points.push(points[<span class="s">0</span>].clone());
        polylines.push(Polyline::new(points));
        planes.push(Plane::from_point_normal(origin, normal, None));
    }

    <span class="k">if</span> b.is_solid() &amp;&amp; outline_volume(&amp;polylines) &lt; <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">for</span> pl <span class="k">in</span> planes.iter_mut() {
            <span class="k">let</span> <span class="k">mut</span> n = pl.z_axis();
            n.reverse();
            *pl = Plane::from_point_normal(pl.origin(), n, None);
        }
    }

    (polylines, planes)
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Meshing helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Canonical boundary of every shared edge: model points, the (face, pcurve, parameters) that produced them, and refined (t, uv) samples</span>
#[derive(Default)]
<span class="k">struct</span> EdgeBoundary {
    points: HashMap&lt;usize, Vec&lt;Point&gt;&gt;, <span class="c">// Canonical model points per edge.</span>
    basis: BTreeMap&lt;usize, (usize, usize, Vec&lt;f64&gt;)&gt;, <span class="c">// (face, pcurve, parameters) per edge.</span>
    samples: HashMap&lt;usize, Vec&lt;(f64, Point)&gt;&gt;, <span class="c">// Refined (t, uv) samples per edge.</span>
}

<span class="c">/// Order (parameter, point) pairs by parameter</span>
<span class="k">fn</span> parameter_order(a: &amp;(f64, Point), b: &amp;(f64, Point)) -&gt; Ordering {
    a.<span class="s">0</span>.total_cmp(&amp;b.<span class="s">0</span>)
}

<span class="c">/// Same parameter of two (parameter, point) pairs</span>
<span class="k">fn</span> parameter_equal(a: &amp;<span class="k">mut</span> (f64, Point), b: &amp;<span class="k">mut</span> (f64, Point)) -&gt; bool {
    a.<span class="s">0</span> == b.<span class="s">0</span>
}

<span class="c">/// Order boundary samples (t, uv, point) by t</span>
<span class="k">fn</span> sample_order(a: &amp;(f64, Point, Point), b: &amp;(f64, Point, Point)) -&gt; Ordering {
    a.<span class="s">0</span>.total_cmp(&amp;b.<span class="s">0</span>)
}

<span class="c">/// Same t of two boundary samples</span>
<span class="k">fn</span> sample_equal(a: &amp;<span class="k">mut</span> (f64, Point, Point), b: &amp;<span class="k">mut</span> (f64, Point, Point)) -&gt; bool {
    a.<span class="s">0</span> == b.<span class="s">0</span>
}

<span class="c">/// Order UV points by u, then v</span>
<span class="k">fn</span> uv_order(a: &amp;Point, b: &amp;Point) -&gt; Ordering {
    a[<span class="s">0</span>].total_cmp(&amp;b[<span class="s">0</span>]).then(a[<span class="s">1</span>].total_cmp(&amp;b[<span class="s">1</span>]))
}

<span class="c">/// UV polygon of one wire of a face (pcurves sampled in traversal order)</span>
<span class="k">fn</span> wire_uv_points(b: &amp;BRep, face_index: usize, wire: &amp;BRepRef) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> pts = Vec::new();

    <span class="k">for</span> er <span class="k">in</span> b.wire_edges(wire) {
        <span class="k">let</span> ci = b.pcurve_index(er.index <span class="k">as</span> usize, face_index, er.orientation);

        <span class="k">if</span> ci &lt; <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> crv = &amp;b.m_curves_2d[ci <span class="k">as</span> usize];
        <span class="k">let</span> <span class="k">mut</span> seg: Vec&lt;Point&gt; = Vec::new();

        <span class="k">if</span> crv.degree() &lt;= <span class="s">1</span> &amp;&amp; !crv.is_rational() {
            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..crv.cv_count() {
                seg.push(crv.get_cv(k).unwrap_or_default());
            }
        } <span class="k">else</span> {
            seg = crv.divide_by_count((crv.cv_count() * <span class="s">4</span>).max(<span class="s">16</span>), <span class="s">true</span>).<span class="s">0</span>;
        }

        <span class="k">if</span> er.orientation == BRepOrientation::Reversed {
            seg.reverse();
        }

        <span class="k">for</span> uv <span class="k">in</span> seg.iter().take(seg.len().saturating_sub(<span class="s">1</span>)) {
            pts.push(uv.clone());
        }
    }

    pts
}

<span class="c">/// Distance from \`point\` to the surface point the pcurve reaches at t</span>
<span class="k">fn</span> lifted_distance(surface: &amp;NurbsSurface, curve: &amp;NurbsCurve, point: &amp;Point, t: f64) -&gt; f64 {
    <span class="k">let</span> uv = curve.point_at(t);

    <span class="k">match</span> surface.point_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>]) {
        Some(lifted) =&gt; lifted.distance(point, None),
        None =&gt; f64::INFINITY,
    }
}

<span class="c">/// Parameter of the lifted pcurve closest to \`point\`: a coarse scan then 64 golden-section steps in the best cell</span>
<span class="k">fn</span> boundary_parameter(surface: &amp;NurbsSurface, curve: &amp;NurbsCurve, point: &amp;Point) -&gt; f64 {
    <span class="k">let</span> (start, end) = curve.domain();
    <span class="k">let</span> count = (curve.cv_count() * <span class="s">4</span>).clamp(<span class="s">32</span>, <span class="s">4096</span>);
    <span class="k">let</span> step = (end - start) / count <span class="k">as</span> f64;
    <span class="k">let</span> <span class="k">mut</span> best = start;
    <span class="k">let</span> <span class="k">mut</span> error = lifted_distance(surface, curve, point, start);

    <span class="k">for</span> index <span class="k">in</span> <span class="s">1</span>..=count {
        <span class="k">let</span> t = <span class="k">if</span> index == count {
            end
        } <span class="k">else</span> {
            start + index <span class="k">as</span> f64 * step
        };
        <span class="k">let</span> candidate = lifted_distance(surface, curve, point, t);

        <span class="k">if</span> candidate &lt; error {
            best = t;
            error = candidate;
        }
    }

    <span class="k">let</span> <span class="k">mut</span> left = (best - step).max(start);
    <span class="k">let</span> <span class="k">mut</span> right = (best + step).min(end);
    <span class="k">let</span> ratio = (<span class="s">5</span>.<span class="s">0f64</span>.sqrt() - <span class="s">1</span>.<span class="s">0</span>) * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> <span class="k">mut</span> a = right - ratio * (right - left);
    <span class="k">let</span> <span class="k">mut</span> b = left + ratio * (right - left);
    <span class="k">let</span> <span class="k">mut</span> da = lifted_distance(surface, curve, point, a);
    <span class="k">let</span> <span class="k">mut</span> db = lifted_distance(surface, curve, point, b);

    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">64</span> {
        <span class="k">if</span> da &lt; db {
            right = b;
            b = a;
            db = da;
            a = right - ratio * (right - left);
            da = lifted_distance(surface, curve, point, a);
        } <span class="k">else</span> {
            left = a;
            a = b;
            da = db;
            b = left + ratio * (right - left);
            db = lifted_distance(surface, curve, point, b);
        }
    }

    <span class="k">if</span> da &lt; error {
        best = a;
        error = da;
    }

    <span class="k">if</span> db &lt; error {
        best = b;
    }

    best
}

<span class="c">/// Unit normal on a boundary, taking the one-sided limit toward \`toward\` at a singular endpoint</span>
<span class="k">fn</span> boundary_normal(
    surface: &amp;NurbsSurface,
    curve: &amp;NurbsCurve,
    t: f64,
    toward: f64,
) -&gt; Option&lt;Vector&gt; {
    <span class="k">for</span> at <span class="k">in</span> [t, t + (toward - t) * <span class="s">1</span>e-<span class="s">6</span>] {
        <span class="k">let</span> uv = curve.point_at(at);
        <span class="k">let</span> derivatives = surface.evaluate(uv[<span class="s">0</span>], uv[<span class="s">1</span>], <span class="s">1</span>);

        <span class="k">if</span> derivatives.len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> n = derivatives[<span class="s">1</span>].cross(&amp;derivatives[<span class="s">2</span>]);
        <span class="k">let</span> scale = n[<span class="s">0</span>].abs().max(n[<span class="s">1</span>].abs()).max(n[<span class="s">2</span>].abs());

        <span class="k">if</span> !scale.is_finite() || scale == <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        n /= scale;
        <span class="k">let</span> length = n.magnitude();

        <span class="k">if</span> length.is_finite() &amp;&amp; length &gt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> Some(n / length);
        }
    }

    None
}

<span class="c">/// True when the normals at ta, t and tb turn more than the angle whose cosine is given</span>
<span class="k">fn</span> boundary_turns(
    surface: &amp;NurbsSurface,
    curve: &amp;NurbsCurve,
    ta: f64,
    t: f64,
    tb: f64,
    cosine: f64,
) -&gt; bool {
    <span class="k">let</span> normals = [
        boundary_normal(surface, curve, ta, tb),
        boundary_normal(surface, curve, t, ta),
        boundary_normal(surface, curve, tb, ta),
    ];

    <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
        <span class="k">for</span> k <span class="k">in</span> j + <span class="s">1</span>..<span class="s">3</span> {
            <span class="k">let</span> (Some(n), Some(m)) = (&amp;normals[j], &amp;normals[k]) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> n.dot(m) &lt; cosine {
                <span class="k">return</span> <span class="s">true</span>;
            }
        }
    }

    <span class="s">false</span>
}

<span class="c">/// Refine samples of a lifted pcurve until chord and angle hold; existing samples stay exact, eight split levels and 4096 added points per edge bound the work</span>
<span class="k">fn</span> refine_surface_boundary(
    surface: &amp;NurbsSurface,
    curve: &amp;NurbsCurve,
    samples: &amp;[(f64, Point, Point)],
    angle: f64,
    chord: f64,
) -&gt; Vec&lt;(f64, Point, Point)&gt; {
    <span class="k">if</span> samples.len() &lt; <span class="s">2</span> {
        <span class="k">return</span> samples.to_vec();
    }

    <span class="k">let</span> tolerance = bbox_diagonal(surface) * chord;
    <span class="k">let</span> cosine = (angle.clamp(<span class="s">0</span>.<span class="s">1</span>, <span class="s">179</span>.<span class="s">0</span>) * PI / <span class="s">180</span>.<span class="s">0</span>).cos();
    <span class="k">let</span> <span class="k">mut</span> result = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> added = <span class="s">0</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..samples.len() {
        <span class="k">let</span> <span class="k">mut</span> stack = vec![(samples[i - <span class="s">1</span>].clone(), samples[i].clone(), <span class="s">0</span>)];

        <span class="k">while</span> <span class="k">let</span> Some((a, b, depth)) = stack.pop() {
            <span class="k">let</span> t = (a.<span class="s">0</span> + b.<span class="s">0</span>) * <span class="s">0</span>.<span class="s">5</span>;
            <span class="k">let</span> uv = curve.point_at(t);
            <span class="k">let</span> Some(point) = surface.point_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>]) <span class="k">else</span> {
                result.push(a);
                <span class="k">continue</span>;
            };
            <span class="k">let</span> pa = &amp;a.<span class="s">2</span>;
            <span class="k">let</span> pb = &amp;b.<span class="s">2</span>;
            <span class="k">let</span> center = Point::new(
                (pa[<span class="s">0</span>] + pb[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (pa[<span class="s">1</span>] + pb[<span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (pa[<span class="s">2</span>] + pb[<span class="s">2</span>]) * <span class="s">0</span>.<span class="s">5</span>,
            );
            <span class="k">let</span> split = (point.distance(&amp;center, None) &gt; tolerance
                || boundary_turns(surface, curve, a.<span class="s">0</span>, t, b.<span class="s">0</span>, cosine))
                &amp;&amp; depth &lt; <span class="s">8</span>
                &amp;&amp; added &lt; <span class="s">4096</span>;

            <span class="k">if</span> !split {
                result.push(a);
                <span class="k">continue</span>;
            }

            added += <span class="s">1</span>;
            <span class="k">let</span> middle = (t, uv, point);
            stack.push((middle.clone(), b, depth + <span class="s">1</span>));
            stack.push((a, middle, depth + <span class="s">1</span>));
        }
    }

    result.push(samples[samples.len() - <span class="s">1</span>].clone());

    result
}

<span class="c">/// Compare canonical boundary positions exactly, without tolerance</span>
<span class="k">fn</span> same_boundary_point(a: &amp;Point, b: &amp;Point) -&gt; bool {
    a[<span class="s">0</span>] == b[<span class="s">0</span>] &amp;&amp; a[<span class="s">1</span>] == b[<span class="s">1</span>] &amp;&amp; a[<span class="s">2</span>] == b[<span class="s">2</span>]
}

<span class="c">/// Phase 1: the outer wire is the full UV rectangle (straight pcurves enclosing the whole domain, no holes), so the face meshes directly on the surface grid</span>
<span class="k">fn</span> direct_face(b: &amp;BRep, fi: usize) -&gt; bool {
    <span class="k">let</span> face = &amp;b.m_faces[fi];

    <span class="k">if</span> face.wires.len() != <span class="s">1</span> {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">for</span> er <span class="k">in</span> b.wire_edges(&amp;face.wires[<span class="s">0</span>]) {
        <span class="k">let</span> ci = b.pcurve_index(er.index <span class="k">as</span> usize, fi, er.orientation);

        <span class="k">if</span> ci &lt; <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">if</span> b.m_curves_2d[ci <span class="k">as</span> usize].degree() &gt; <span class="s">1</span> || b.m_curves_2d[ci <span class="k">as</span> usize].is_rational() {
            <span class="k">return</span> <span class="s">false</span>;
        }
    }

    <span class="k">let</span> outer = wire_uv_points(b, fi, &amp;face.wires[<span class="s">0</span>]);

    <span class="k">if</span> outer.len() &lt; <span class="s">3</span> {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> srf = &amp;b.m_surfaces[face.surface_index <span class="k">as</span> usize];
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));

    <span class="k">for</span> er <span class="k">in</span> b.wire_edges(&amp;face.wires[<span class="s">0</span>]) {
        <span class="k">let</span> ci = b.pcurve_index(er.index <span class="k">as</span> usize, fi, er.orientation);

        <span class="k">if</span> ci &lt; <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> curve = &amp;b.m_curves_2d[ci <span class="k">as</span> usize];

        <span class="k">for</span> k <span class="k">in</span> [<span class="s">0</span>, curve.cv_count().saturating_sub(<span class="s">1</span>)] {
            <span class="k">let</span> p = curve.get_cv(k).unwrap_or_default();
            <span class="k">let</span> corner_u = (p[<span class="s">0</span>] - u0).abs().min((p[<span class="s">0</span>] - u1).abs()) &lt;= (u1 - u0) * <span class="s">1</span>e-<span class="s">9</span>;
            <span class="k">let</span> corner_v = (p[<span class="s">1</span>] - v0).abs().min((p[<span class="s">1</span>] - v1).abs()) &lt;= (v1 - v0) * <span class="s">1</span>e-<span class="s">9</span>;

            <span class="k">if</span> !corner_u || !corner_v {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }
    }

    <span class="k">let</span> domain_area = (u1 - u0) * (v1 - v0);

    (polygon_signed_area(&amp;outer).abs() - domain_area).abs() &lt; <span class="s">1</span>e-<span class="s">3</span> * domain_area
}

<span class="c">/// Grid vertices on the domain sides flagged by (at_v0, at_v1, at_u0, at_u1), as (parameter along the side, model point) sorted and unique</span>
<span class="k">fn</span> grid_side_points(
    grid: &amp;Mesh,
    srf: &amp;NurbsSurface,
    at_v0: bool,
    at_v1: bool,
    at_u0: bool,
    at_u1: bool,
) -&gt; Vec&lt;(f64, Point)&gt; {
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> utol = (u1 - u0) * <span class="s">0</span>.<span class="s">001</span>;
    <span class="k">let</span> vtol = (v1 - v0) * <span class="s">0</span>.<span class="s">001</span>;
    <span class="k">let</span> <span class="k">mut</span> pts: Vec&lt;(f64, Point)&gt; = Vec::new();

    <span class="k">for</span> vd <span class="k">in</span> grid.vertex.values() {
        <span class="k">let</span> (Some(&amp;iu), Some(&amp;iv)) = (vd.attributes.get(&quot;<span class="s">u</span>&quot;), vd.attributes.get(&quot;<span class="s">v</span>&quot;)) <span class="k">else</span> {
            <span class="k">continue</span>;
        };

        <span class="k">if</span> (at_v0 &amp;&amp; (iv - v0).abs() &lt; vtol * <span class="s">0</span>.<span class="s">1</span>) || (at_v1 &amp;&amp; (iv - v1).abs() &lt; vtol * <span class="s">0</span>.<span class="s">1</span>) {
            pts.push((iu, vd.position()));
        } <span class="k">else</span> <span class="k">if</span> (at_u0 &amp;&amp; (iu - u0).abs() &lt; utol * <span class="s">0</span>.<span class="s">1</span>) || (at_u1 &amp;&amp; (iu - u1).abs() &lt; utol * <span class="s">0</span>.<span class="s">1</span>)
        {
            pts.push((iv, vd.position()));
        }
    }

    pts.sort_by(parameter_order);
    pts.dedup_by(parameter_equal);

    pts
}

<span class="c">/// Phase 2: grid vertices of a direct face along a shared edge that runs on a domain side, as (pcurve parameter, model point) sorted along the edge; empty elsewhere</span>
<span class="k">fn</span> grid_edge_samples(b: &amp;BRep, fi: usize, grid: &amp;Mesh, er: &amp;BRepRef) -&gt; Vec&lt;(f64, Point)&gt; {
    <span class="k">let</span> <span class="k">mut</span> samples: Vec&lt;(f64, Point)&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> shared = <span class="s">false</span>;

    <span class="k">for</span> fr <span class="k">in</span> b.edge_faces(er.index <span class="k">as</span> usize) {
        <span class="k">if</span> fr.index <span class="k">as</span> usize != fi {
            shared = <span class="s">true</span>;
        }
    }

    <span class="k">if</span> !shared {
        <span class="k">return</span> samples;
    }

    <span class="k">let</span> ci = b.pcurve_index(er.index <span class="k">as</span> usize, fi, er.orientation);

    <span class="k">if</span> ci &lt; <span class="s">0</span> {
        <span class="k">return</span> samples;
    }

    <span class="k">let</span> srf = &amp;b.m_surfaces[b.m_faces[fi].surface_index <span class="k">as</span> usize];
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> utol = (u1 - u0) * <span class="s">0</span>.<span class="s">001</span>;
    <span class="k">let</span> vtol = (v1 - v0) * <span class="s">0</span>.<span class="s">001</span>;
    <span class="k">let</span> c2d = &amp;b.m_curves_2d[ci <span class="k">as</span> usize];
    <span class="k">let</span> (Some(sp), Some(ep)) = (c2d.get_cv(<span class="s">0</span>), c2d.get_cv(c2d.cv_count().saturating_sub(<span class="s">1</span>))) <span class="k">else</span> {
        <span class="k">return</span> samples;
    };
    <span class="k">let</span> at_v0 = (sp[<span class="s">1</span>] - v0).abs() &lt; vtol &amp;&amp; (ep[<span class="s">1</span>] - v0).abs() &lt; vtol;
    <span class="k">let</span> at_v1 = (sp[<span class="s">1</span>] - v1).abs() &lt; vtol &amp;&amp; (ep[<span class="s">1</span>] - v1).abs() &lt; vtol;
    <span class="k">let</span> at_u0 = (sp[<span class="s">0</span>] - u0).abs() &lt; utol &amp;&amp; (ep[<span class="s">0</span>] - u0).abs() &lt; utol;
    <span class="k">let</span> at_u1 = (sp[<span class="s">0</span>] - u1).abs() &lt; utol &amp;&amp; (ep[<span class="s">0</span>] - u1).abs() &lt; utol;

    <span class="k">if</span> !at_v0 &amp;&amp; !at_v1 &amp;&amp; !at_u0 &amp;&amp; !at_u1 {
        <span class="k">return</span> samples;
    }

    <span class="k">let</span> pts = grid_side_points(grid, srf, at_v0, at_v1, at_u0, at_u1);

    <span class="k">if</span> pts.len() &lt; <span class="s">2</span> {
        <span class="k">return</span> samples;
    }

    <span class="k">let</span> varying = <span class="k">if</span> at_v0 || at_v1 { <span class="s">0</span> } <span class="k">else</span> { <span class="s">1</span> };
    <span class="k">let</span> (t0, t1) = c2d.domain();

    <span class="k">for</span> (p, pt) <span class="k">in</span> pts {
        samples.push((
            t0 + (p - sp[varying]) / (ep[varying] - sp[varying]) * (t1 - t0),
            pt,
        ));
    }

    samples
}

<span class="c">/// Phase 2: the first incident grid supplies the canonical polygon of every shared edge; true when this face's grid disagrees with an earlier one and must be rebuilt</span>
<span class="k">fn</span> grid_boundaries(b: &amp;BRep, fi: usize, grid: &amp;Mesh, boundary: &amp;<span class="k">mut</span> EdgeBoundary) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> rebuild = <span class="s">false</span>;

    <span class="k">for</span> er <span class="k">in</span> b.wire_edges(&amp;b.m_faces[fi].wires[<span class="s">0</span>]) {
        <span class="k">let</span> eidx = er.index <span class="k">as</span> usize;
        <span class="k">let</span> samples = grid_edge_samples(b, fi, grid, &amp;er);

        <span class="k">if</span> samples.is_empty() {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> parameters = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> bnd = Vec::new();

        <span class="k">for</span> (t, pt) <span class="k">in</span> samples {
            parameters.push(t);
            bnd.push(pt);
        }

        <span class="k">let</span> Some(canonical) = boundary.points.get(&amp;eidx) <span class="k">else</span> {
            boundary.points.insert(eidx, bnd);
            <span class="k">let</span> ci = b.pcurve_index(eidx, fi, er.orientation) <span class="k">as</span> usize;
            boundary.basis.insert(eidx, (fi, ci, parameters));
            <span class="k">continue</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> forward = <span class="s">true</span>;
        <span class="k">let</span> <span class="k">mut</span> backward = <span class="s">true</span>;

        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..canonical.len().min(bnd.len()) {
            forward = forward &amp;&amp; same_boundary_point(&amp;canonical[k], &amp;bnd[k]);
            backward = backward &amp;&amp; same_boundary_point(&amp;canonical[k], &amp;bnd[bnd.len() - <span class="s">1</span> - k]);
        }

        <span class="k">let</span> matches = canonical.len() == bnd.len() &amp;&amp; (forward || backward);
        rebuild = rebuild || !matches;
    }

    rebuild
}

<span class="c">/// Refine the canonical polygon of every edge shared with a curved CDT face, then mark every incident face for rebuild with the same refined polygon</span>
<span class="k">fn</span> refine_shared_boundaries(
    b: &amp;BRep,
    face_direct: &amp;[bool],
    rebuild_grid: &amp;<span class="k">mut</span> [bool],
    boundary: &amp;<span class="k">mut</span> EdgeBoundary,
    angle: f64,
    chord: f64,
) {
    <span class="k">for</span> (&amp;edge, (face, pcurve, parameters)) <span class="k">in</span> &amp;boundary.basis {
        <span class="k">let</span> <span class="k">mut</span> curved_cdt = <span class="s">false</span>;

        <span class="k">for</span> incident <span class="k">in</span> b.edge_faces(edge) {
            <span class="k">let</span> fi = incident.index <span class="k">as</span> usize;
            <span class="k">let</span> cdt = !face_direct[fi] || rebuild_grid[fi];
            curved_cdt = curved_cdt
                || (cdt
                    &amp;&amp; !b.m_surfaces[b.m_faces[fi].surface_index <span class="k">as</span> usize].is_planar(None, <span class="s">0</span>.<span class="s">0</span>));
        }

        <span class="k">if</span> !curved_cdt {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> surface = &amp;b.m_surfaces[b.m_faces[*face].surface_index <span class="k">as</span> usize];
        <span class="k">let</span> curve = &amp;b.m_curves_2d[*pcurve];
        <span class="k">let</span> points = &amp;boundary.points[&amp;edge];
        <span class="k">let</span> <span class="k">mut</span> samples: Vec&lt;(f64, Point, Point)&gt; = Vec::new();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..parameters.len() {
            samples.push((
                parameters[i],
                curve.point_at(parameters[i]),
                points[i].clone(),
            ));
        }

        samples.sort_by(sample_order);
        <span class="k">let</span> end = curve.domain().<span class="s">1</span>;

        <span class="k">if</span> b.m_edges[edge].start_vertex == b.m_edges[edge].end_vertex
            &amp;&amp; !samples.is_empty()
            &amp;&amp; samples[samples.len() - <span class="s">1</span>].<span class="s">0</span> &lt; end
        {
            samples.push((end, curve.point_at(end), samples[<span class="s">0</span>].<span class="s">2</span>.clone()));
        }

        <span class="k">let</span> refined = refine_surface_boundary(surface, curve, &amp;samples, angle, chord);

        <span class="k">if</span> refined.len() &lt;= samples.len() {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> refined_points = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> refined_samples = Vec::new();

        <span class="k">for</span> (t, uv, p) <span class="k">in</span> refined {
            refined_points.push(p);
            refined_samples.push((t, uv));
        }

        boundary.points.insert(edge, refined_points);
        boundary.samples.insert(edge, refined_samples);

        <span class="k">for</span> incident <span class="k">in</span> b.edge_faces(edge) {
            rebuild_grid[incident.index <span class="k">as</span> usize] = <span class="s">true</span>;
        }
    }
}

<span class="c">/// Interior UV seeds of a rebuilt face: its grid vertices strictly inside the domain</span>
<span class="k">fn</span> grid_interior_uv(srf: &amp;NurbsSurface, grid: &amp;Mesh) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> <span class="k">mut</span> seeds = Vec::new();

    <span class="k">for</span> vertex <span class="k">in</span> grid.vertex.values() {
        <span class="k">let</span> (Some(&amp;u), Some(&amp;v)) = (vertex.attributes.get(&quot;<span class="s">u</span>&quot;), vertex.attributes.get(&quot;<span class="s">v</span>&quot;)) <span class="k">else</span> {
            <span class="k">continue</span>;
        };

        <span class="k">if</span> u &gt; u0 &amp;&amp; u &lt; u1 &amp;&amp; v &gt; v0 &amp;&amp; v &lt; v1 {
            seeds.push(Point::new(u, v, <span class="s">0</span>.<span class="s">0</span>));
        }
    }

    seeds.sort_by(uv_order);

    seeds
}

<span class="c">/// Planarity tolerance for a surface of any size: 1e-9 of its control-point bounding box diagonal, never below the zero tolerance</span>
<span class="k">fn</span> planar_patch_tolerance(srf: &amp;NurbsSurface) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> lo: [f64; <span class="s">3</span>] = [<span class="s">1</span>e<span class="s">300</span>; <span class="s">3</span>];
    <span class="k">let</span> <span class="k">mut</span> hi: [f64; <span class="s">3</span>] = [-<span class="s">1</span>e<span class="s">300</span>; <span class="s">3</span>];

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..srf.cv_count(<span class="s">0</span>) {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..srf.cv_count(<span class="s">1</span>) {
            <span class="k">let</span> p = srf.get_cv(i, j).unwrap_or_default();

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
    }

    <span class="k">let</span> diagonal = ((hi[<span class="s">0</span>] - lo[<span class="s">0</span>]) * (hi[<span class="s">0</span>] - lo[<span class="s">0</span>])
        + (hi[<span class="s">1</span>] - lo[<span class="s">1</span>]) * (hi[<span class="s">1</span>] - lo[<span class="s">1</span>])
        + (hi[<span class="s">2</span>] - lo[<span class="s">2</span>]) * (hi[<span class="s">2</span>] - lo[<span class="s">2</span>]))
        .sqrt();

    (<span class="s">1</span>e-<span class="s">9</span> * diagonal).max(Tolerance::ZERO_TOLERANCE)
}

<span class="c">/// True for a surface flat within planar_patch_tolerance, whatever its coordinates</span>
<span class="k">fn</span> is_planar_patch(srf: &amp;NurbsSurface) -&gt; bool {
    srf.is_planar(None, planar_patch_tolerance(srf))
}

<span class="c">/// Surface parameters of a point on a degree-1 parallelogram patch by two dot products; None when the patch is not that shape</span>
<span class="k">fn</span> planar_patch_uv(srf: &amp;NurbsSurface, p: &amp;Point) -&gt; Option&lt;(f64, f64)&gt; {
    <span class="k">if</span> srf.degree(<span class="s">0</span>) != <span class="s">1</span> || srf.degree(<span class="s">1</span>) != <span class="s">1</span> || srf.cv_count(<span class="s">0</span>) != <span class="s">2</span> || srf.cv_count(<span class="s">1</span>) != <span class="s">2</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> p00 = srf.get_cv(<span class="s">0</span>, <span class="s">0</span>)?;
    <span class="k">let</span> p10 = srf.get_cv(<span class="s">1</span>, <span class="s">0</span>)?;
    <span class="k">let</span> p01 = srf.get_cv(<span class="s">0</span>, <span class="s">1</span>)?;
    <span class="k">let</span> p11 = srf.get_cv(<span class="s">1</span>, <span class="s">1</span>)?;
    <span class="k">let</span> eu = &amp;p10 - &amp;p00;
    <span class="k">let</span> ev = &amp;p01 - &amp;p00;
    <span class="k">let</span> skew = (p11 - p10) - ev.clone();

    <span class="k">if</span> skew.magnitude() &gt; planar_patch_tolerance(srf) {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> eu2 = eu.dot(&amp;eu);
    <span class="k">let</span> ev2 = ev.dot(&amp;ev);

    <span class="k">if</span> eu2 &lt;= <span class="s">0</span>.<span class="s">0</span> || ev2 &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> d = p - &amp;p00;
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>)?;
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>)?;

    Some((
        u0 + d.dot(&amp;eu) / eu2 * (u1 - u0),
        v0 + d.dot(&amp;ev) / ev2 * (v1 - v0),
    ))
}

<span class="c">/// Parameter of the closest point on a two-point degree-1 pcurve by one projection; None for any other curve</span>
<span class="k">fn</span> linear_pcurve_parameter(crv: &amp;NurbsCurve, uv: (f64, f64)) -&gt; Option&lt;f64&gt; {
    <span class="k">if</span> crv.degree() != <span class="s">1</span> || crv.is_rational() || crv.cv_count() != <span class="s">2</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> c0 = crv.get_cv(<span class="s">0</span>)?;
    <span class="k">let</span> c1 = crv.get_cv(<span class="s">1</span>)?;
    <span class="k">let</span> dx = c1[<span class="s">0</span>] - c0[<span class="s">0</span>];
    <span class="k">let</span> dy = c1[<span class="s">1</span>] - c0[<span class="s">1</span>];
    <span class="k">let</span> length_squared = dx * dx + dy * dy;

    <span class="k">if</span> length_squared &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> fraction = (((uv.<span class="s">0</span> - c0[<span class="s">0</span>]) * dx + (uv.<span class="s">1</span> - c0[<span class="s">1</span>]) * dy) / length_squared).clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
    <span class="k">let</span> (t0, t1) = crv.domain();

    Some(t0 + fraction * (t1 - t0))
}

<span class="c">/// True when the pcurve point \`q\` lifts onto the surface farther than \`tolerance\` from \`p\`, or cannot be lifted</span>
<span class="k">fn</span> lift_misses(srf: &amp;NurbsSurface, q: &amp;Point, p: &amp;Point, tolerance: f64) -&gt; bool {
    <span class="k">match</span> srf.point_at(q[<span class="s">0</span>], q[<span class="s">1</span>]) {
        Some(lifted) =&gt; lifted.distance(p, None) &gt; tolerance,
        None =&gt; <span class="s">true</span>,
    }
}

<span class="c">/// Phase 3: map the canonical points of edge \`ei\` onto pcurve \`ci\` of face \`fi\`, checked in model space; false when a point cannot be lifted</span>
<span class="k">fn</span> lift_canonical(
    b: &amp;BRep,
    fi: usize,
    ei: usize,
    ci: usize,
    boundary: &amp;EdgeBoundary,
    samples: &amp;<span class="k">mut</span> Vec&lt;(f64, Point, Point)&gt;,
) -&gt; bool {
    <span class="k">let</span> face = &amp;b.m_faces[fi];
    <span class="k">let</span> edge = &amp;b.m_edges[ei];
    <span class="k">let</span> srf = &amp;b.m_surfaces[face.surface_index <span class="k">as</span> usize];
    <span class="k">let</span> crv = &amp;b.m_curves_2d[ci];
    <span class="k">let</span> cached = <span class="k">match</span> boundary.basis.get(&amp;ei) {
        Some(basis) =&gt; basis.<span class="s">0</span> == fi &amp;&amp; basis.<span class="s">1</span> == ci &amp;&amp; boundary.samples.contains_key(&amp;ei),
        None =&gt; <span class="s">false</span>,
    };
    <span class="k">let</span> points = &amp;boundary.points[&amp;ei];
    <span class="k">let</span> planar = is_planar_patch(srf);

    <span class="k">for</span> (index, p) <span class="k">in</span> points.iter().enumerate() {
        <span class="k">let</span> (<span class="k">mut</span> t, <span class="k">mut</span> q) = <span class="k">if</span> cached {
            boundary.samples[&amp;ei][index].clone()
        } <span class="k">else</span> {
            <span class="k">let</span> patch_uv = <span class="k">if</span> planar {
                planar_patch_uv(srf, p)
            } <span class="k">else</span> {
                None
            };
            <span class="k">let</span> (u, v) = <span class="k">match</span> patch_uv {
                Some(uv) =&gt; uv,
                None =&gt; srf.closest_parameters(p),
            };
            <span class="k">let</span> linear = <span class="k">match</span> patch_uv {
                Some(uv) =&gt; linear_pcurve_parameter(crv, uv),
                None =&gt; None,
            };
            <span class="k">let</span> t = <span class="k">match</span> linear {
                Some(t) =&gt; t,
                None =&gt; crv.closest_parameter(&amp;Point::new(u, v, <span class="s">0</span>.<span class="s">0</span>)),
            };
            (t, crv.point_at(t))
        };
        <span class="k">let</span> scale = p[<span class="s">0</span>].abs().max(p[<span class="s">1</span>].abs()).max(p[<span class="s">2</span>].abs()).max(<span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> tolerance = edge
            .tolerance
            .max(face.tolerance)
            .max(f64::EPSILON.sqrt() * scale);

        <span class="k">if</span> lift_misses(srf, &amp;q, p, tolerance) {
            t = boundary_parameter(srf, crv, p);
            q = crv.point_at(t);

            <span class="k">if</span> lift_misses(srf, &amp;q, p, tolerance) {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }

        samples.push((t, q, p.clone()));
    }

    samples.sort_by(sample_order);
    samples.dedup_by(sample_equal);

    <span class="s">true</span>
}

<span class="c">/// Phase 3: fresh samples of a pcurve nobody has sampled yet, refined to the face's angle and chord; empty when the surface cannot be evaluated</span>
<span class="k">fn</span> fresh_samples(
    srf: &amp;NurbsSurface,
    crv: &amp;NurbsCurve,
    angle: f64,
    chord: f64,
) -&gt; Vec&lt;(f64, Point, Point)&gt; {
    <span class="k">let</span> <span class="k">mut</span> points = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> parameters = Vec::new();

    <span class="k">if</span> crv.degree() &lt;= <span class="s">1</span> &amp;&amp; !crv.is_rational() &amp;&amp; is_planar_patch(srf) {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..crv.cv_count() {
            points.push(crv.get_cv(k).unwrap_or_default());
            parameters.push(crv.greville_abcissa(k));
        }
    } <span class="k">else</span> {
        <span class="k">let</span> count = (crv.cv_count() * <span class="s">4</span>)
            .max((<span class="s">360</span>.<span class="s">0</span> / angle.max(<span class="s">0</span>.<span class="s">1</span>)).ceil() <span class="k">as</span> usize)
            .min(<span class="s">4096</span>);
        (points, parameters) = crv.divide_by_count(count, <span class="s">true</span>);
    }

    <span class="k">let</span> <span class="k">mut</span> samples: Vec&lt;(f64, Point, Point)&gt; = Vec::new();

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..points.len() {
        <span class="k">let</span> q = points[k].clone();
        <span class="k">let</span> Some(p) = srf.point_at(q[<span class="s">0</span>], q[<span class="s">1</span>]) <span class="k">else</span> {
            <span class="k">return</span> Vec::new();
        };
        samples.push((parameters[k], q, p));
    }

    refine_surface_boundary(srf, crv, &amp;samples, angle, chord)
}

<span class="c">/// Phase 3: samples of one edge use of a CDT face in traversal direction, a closed edge repeating its first point at the end; false when the edge has no pcurve or cannot be lifted</span>
<span class="k">fn</span> edge_use_samples(
    b: &amp;BRep,
    fi: usize,
    er: &amp;BRepRef,
    boundary: &amp;<span class="k">mut</span> EdgeBoundary,
    angle: f64,
    chord: f64,
    samples: &amp;<span class="k">mut</span> Vec&lt;(f64, Point, Point)&gt;,
) -&gt; bool {
    <span class="k">let</span> ei = er.index <span class="k">as</span> usize;
    <span class="k">let</span> edge = &amp;b.m_edges[ei];
    <span class="k">let</span> ci = b.pcurve_index(ei, fi, er.orientation);

    <span class="k">if</span> ci &lt; <span class="s">0</span> {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> crv = &amp;b.m_curves_2d[ci <span class="k">as</span> usize];
    <span class="k">let</span> canonical = boundary.points.contains_key(&amp;ei);

    <span class="k">if</span> canonical {
        <span class="k">if</span> !lift_canonical(b, fi, ei, ci <span class="k">as</span> usize, boundary, samples) {
            <span class="k">return</span> <span class="s">false</span>;
        }
    } <span class="k">else</span> {
        *samples = fresh_samples(
            &amp;b.m_surfaces[b.m_faces[fi].surface_index <span class="k">as</span> usize],
            crv,
            angle,
            chord,
        );
        <span class="k">let</span> <span class="k">mut</span> positions = Vec::new();

        <span class="k">for</span> sample <span class="k">in</span> samples.iter() {
            positions.push(sample.<span class="s">2</span>.clone());
        }

        boundary.points.insert(ei, positions);
    }

    <span class="k">if</span> edge.start_vertex == edge.end_vertex &amp;&amp; samples.len() &gt; <span class="s">1</span> {
        <span class="k">let</span> first = samples[<span class="s">0</span>].clone();

        <span class="k">if</span> !same_boundary_point(&amp;first.<span class="s">2</span>, &amp;samples[samples.len() - <span class="s">1</span>].<span class="s">2</span>) {
            samples.push((crv.domain().<span class="s">1</span>, first.<span class="s">1</span>, first.<span class="s">2</span>));
        }
    }

    <span class="k">if</span> er.orientation == BRepOrientation::Reversed {
        samples.reverse();
    }

    samples.len() &gt;= <span class="s">2</span>
}

<span class="c">/// Phase 3: trim loops of a CDT face, every edge use keeping its boundary-node identities; false when some edge cannot be sampled</span>
<span class="k">fn</span> trim_loops(
    b: &amp;BRep,
    fi: usize,
    boundary: &amp;<span class="k">mut</span> EdgeBoundary,
    angle: f64,
    chord: f64,
    loops: &amp;<span class="k">mut</span> TrimLoops,
    uses: &amp;<span class="k">mut</span> Vec&lt;(usize, usize, usize, usize)&gt;,
) -&gt; bool {
    <span class="k">let</span> face = &amp;b.m_faces[fi];

    <span class="k">for</span> wi <span class="k">in</span> <span class="s">0</span>..face.wires.len() {
        <span class="k">let</span> <span class="k">mut</span> uv = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> xyz = Vec::new();

        <span class="k">for</span> er <span class="k">in</span> b.wire_edges(&amp;face.wires[wi]) {
            <span class="k">let</span> <span class="k">mut</span> samples: Vec&lt;(f64, Point, Point)&gt; = Vec::new();

            <span class="k">if</span> !edge_use_samples(b, fi, &amp;er, boundary, angle, chord, &amp;<span class="k">mut</span> samples) {
                <span class="k">return</span> <span class="s">false</span>;
            }

            uses.push((er.index <span class="k">as</span> usize, wi, uv.len(), samples.len()));

            <span class="k">for</span> sample <span class="k">in</span> samples.iter().take(samples.len().saturating_sub(<span class="s">1</span>)) {
                uv.push(sample.<span class="s">1</span>.clone());
                xyz.push(sample.<span class="s">2</span>.clone());
            }
        }

        loops.uv.push(uv);
        loops.xyz.push(xyz);
    }

    <span class="s">true</span>
}

<span class="c">/// Set every vertex normal and tag every loop vertex boundary/{loop}/{sample} as mesh_loops does</span>
<span class="k">fn</span> tag_loop_vertices(mesh: &amp;<span class="k">mut</span> Mesh, loops: &amp;TrimLoops, normal: &amp;Vector) {
    <span class="k">let</span> <span class="k">mut</span> lookup: HashMap&lt;(u64, u64, u64), (usize, usize)&gt; = HashMap::new();

    <span class="k">for</span> li <span class="k">in</span> <span class="s">0</span>..loops.xyz.len() {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..loops.xyz[li].len() {
            <span class="k">let</span> p = &amp;loops.xyz[li][k];
            lookup
                .entry((p[<span class="s">0</span>].to_bits(), p[<span class="s">1</span>].to_bits(), p[<span class="s">2</span>].to_bits()))
                .or_insert((li, k));
        }
    }

    <span class="k">for</span> vd <span class="k">in</span> mesh.vertex.values_mut() {
        vd.set_normal(normal[<span class="s">0</span>], normal[<span class="s">1</span>], normal[<span class="s">2</span>]);
        <span class="k">let</span> position = vd.position();

        <span class="k">if</span> <span class="k">let</span> Some(&amp;(li, k)) = lookup.get(&amp;(
            position[<span class="s">0</span>].to_bits(),
            position[<span class="s">1</span>].to_bits(),
            position[<span class="s">2</span>].to_bits(),
        )) {
            vd.attributes.insert(format!(&quot;<span class="s">boundary/</span>{<span class="s">li</span>}<span class="s">/</span>{<span class="s">k</span>}&quot;), <span class="s">1</span>.<span class="s">0</span>);
        }
    }
}

<span class="c">/// Add the border and hole vertices and the CDT triangles of their 2D rings, degenerate triangles skipped</span>
<span class="k">fn</span> add_ring_faces(
    mesh: &amp;<span class="k">mut</span> Mesh,
    border: &amp;[Point],
    holes: &amp;[Vec&lt;Point&gt;],
    border_2d: &amp;[Point],
    holes_2d: &amp;[Vec&lt;Point&gt;],
) {
    <span class="k">use</span> <span class="k">crate</span>::remesh_cdt::cdt_triangulate;

    <span class="k">let</span> <span class="k">mut</span> vkeys = Vec::new();

    <span class="k">for</span> p <span class="k">in</span> border {
        vkeys.push(mesh.add_vertex(p.clone(), None));
    }

    <span class="k">for</span> hole <span class="k">in</span> holes {
        <span class="k">for</span> p <span class="k">in</span> hole {
            vkeys.push(mesh.add_vertex(p.clone(), None));
        }
    }

    <span class="k">for</span> (a, b, c) <span class="k">in</span> cdt_triangulate(border_2d, holes_2d) {
        <span class="k">if</span> a != b &amp;&amp; b != c &amp;&amp; c != a {
            mesh.add_face(vec![vkeys[a], vkeys[b], vkeys[c]], None);
        }
    }
}

<span class="c">/// Flip the mesh when its first face winds against the normal</span>
<span class="k">fn</span> wind_to_normal(mesh: &amp;<span class="k">mut</span> Mesh, normal: &amp;Vector) {
    <span class="k">if</span> mesh.face.is_empty() {
        <span class="k">return</span>;
    }

    <span class="k">let</span> fverts = &amp;mesh.face[&amp;sorted_face_keys(mesh)[<span class="s">0</span>]];
    <span class="k">let</span> a = mesh.vertex[&amp;fverts[<span class="s">0</span>]].position();
    <span class="k">let</span> b = mesh.vertex[&amp;fverts[<span class="s">1</span>]].position();
    <span class="k">let</span> c = mesh.vertex[&amp;fverts[<span class="s">2</span>]].position();

    <span class="k">if</span> (&amp;b - &amp;a).cross(&amp;(&amp;c - &amp;a)).dot(normal) &lt; <span class="s">0</span>.<span class="s">0</span> {
        mesh.flip();
    }
}

<span class="c">/// Phase 3 for a planar face: the sampled loops triangulated as one polygon with holes, wound to the surface normal, every loop vertex tagged boundary/{loop}/{sample} as mesh_loops does; no grid, no surface evaluation</span>
<span class="k">fn</span> planar_loops_mesh(srf: &amp;NurbsSurface, loops: &amp;TrimLoops) -&gt; Mesh {
    <span class="k">use</span> <span class="k">crate</span>::remesh_cdt::project_2d;
    <span class="k">use</span> <span class="k">crate</span>::remesh_cdt::signed_area;

    <span class="k">let</span> <span class="k">mut</span> mesh = Mesh::new();

    <span class="k">if</span> loops.xyz.is_empty() || loops.xyz[<span class="s">0</span>].len() &lt; <span class="s">3</span> {
        <span class="k">return</span> mesh;
    }

    <span class="k">let</span> <span class="k">mut</span> all_pts: Vec&lt;Point&gt; = Vec::new();

    <span class="k">for</span> loop_pts <span class="k">in</span> &amp;loops.xyz {
        all_pts.extend(loop_pts.iter().cloned());
    }

    <span class="k">let</span> (origin, xaxis, yaxis, _zaxis) = Polyline::new(all_pts).get_average_plane();
    <span class="k">let</span> <span class="k">mut</span> border = loops.xyz[<span class="s">0</span>].clone();
    <span class="k">let</span> <span class="k">mut</span> border_2d = project_2d(&amp;border, &amp;origin, &amp;xaxis, &amp;yaxis);

    <span class="k">if</span> signed_area(&amp;border_2d) &lt; <span class="s">0</span>.<span class="s">0</span> {
        border.reverse();
        border_2d.reverse();
    }

    <span class="k">let</span> <span class="k">mut</span> holes: Vec&lt;Vec&lt;Point&gt;&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> holes_2d: Vec&lt;Vec&lt;Point&gt;&gt; = Vec::new();

    <span class="k">for</span> li <span class="k">in</span> <span class="s">1</span>..loops.xyz.len() {
        <span class="k">if</span> loops.xyz[li].len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> hole = loops.xyz[li].clone();
        <span class="k">let</span> <span class="k">mut</span> hole_2d = project_2d(&amp;hole, &amp;origin, &amp;xaxis, &amp;yaxis);

        <span class="k">if</span> signed_area(&amp;hole_2d) &gt; <span class="s">0</span>.<span class="s">0</span> {
            hole.reverse();
            hole_2d.reverse();
        }

        holes.push(hole);
        holes_2d.push(hole_2d);
    }

    add_ring_faces(&amp;<span class="k">mut</span> mesh, &amp;border, &amp;holes, &amp;border_2d, &amp;holes_2d);

    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> normal = srf.normal_at(<span class="s">0</span>.<span class="s">5</span> * (u0 + u1), <span class="s">0</span>.<span class="s">5</span> * (v0 + v1));
    wind_to_normal(&amp;<span class="k">mut</span> mesh, &amp;normal);
    tag_loop_vertices(&amp;<span class="k">mut</span> mesh, loops, &amp;normal);

    mesh
}

<span class="c">/// Tag every boundary vertex of a CDT mesh with the edge use it samples; each use keeps both ends, including the next edge's start</span>
<span class="k">fn</span> tag_edge_uses(mesh: &amp;<span class="k">mut</span> Mesh, loops: &amp;TrimLoops, uses: &amp;[(usize, usize, usize, usize)]) {
    <span class="k">for</span> (use_id, &amp;(edge, li, start, count)) <span class="k">in</span> uses.iter().enumerate() {
        <span class="k">let</span> length = loops.uv[li].len();

        <span class="k">if</span> length == <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">for</span> sample <span class="k">in</span> <span class="s">0</span>..count {
            <span class="k">let</span> key = format!(&quot;<span class="s">boundary/</span>{<span class="s">li</span>}<span class="s">/</span>{}&quot;, (start + sample) % length);
            <span class="k">let</span> tag = format!(&quot;<span class="s">brep_edge/</span>{<span class="s">edge</span>}<span class="s">/</span>{<span class="s">use_id</span>}<span class="s">/</span>{<span class="s">sample</span>}&quot;);

            <span class="k">for</span> vd <span class="k">in</span> mesh.vertex.values_mut() {
                <span class="k">if</span> vd.attributes.contains_key(&amp;key) {
                    vd.attributes.insert(tag.clone(), <span class="s">1</span>.<span class="s">0</span>);
                }
            }

            <span class="k">if</span> sample + <span class="s">1</span> &gt;= count {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> interval = format!(&quot;<span class="s">boundary_interval/</span>{<span class="s">li</span>}<span class="s">/</span>{}&quot;, (start + sample) % length);
            <span class="k">let</span> interval_tag = format!(&quot;<span class="s">brep_edge_interval/</span>{<span class="s">edge</span>}<span class="s">/</span>{<span class="s">use_id</span>}<span class="s">/</span>{<span class="s">sample</span>}&quot;);

            <span class="k">for</span> vd <span class="k">in</span> mesh.vertex.values_mut() {
                <span class="k">if</span> <span class="k">let</span> Some(&amp;t) = vd.attributes.get(&amp;interval) {
                    vd.attributes.insert(interval_tag.clone(), t);
                }
            }
        }
    }
}

<span class="c">/// Flip every face mesh of a face Reversed in its shell, vertex normals included</span>
<span class="k">fn</span> flip_reversed_faces(b: &amp;BRep, fmesh: &amp;<span class="k">mut</span> [Mesh]) {
    <span class="k">for</span> (fi, fm) <span class="k">in</span> fmesh.iter_mut().enumerate() {
        <span class="k">if</span> b.face_orientation(fi) != BRepOrientation::Reversed {
            <span class="k">continue</span>;
        }

        fm.flip();

        <span class="k">for</span> vd <span class="k">in</span> fm.vertex.values_mut() {
            <span class="k">if</span> <span class="k">let</span> Some(n) = vd.normal() {
                vd.set_normal(-n[<span class="s">0</span>], -n[<span class="s">1</span>], -n[<span class="s">2</span>]);
            }
        }
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Cutting helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Vertex rings of every face in one mesh keyed by BRep vertex index, outer rings wound to the face normal, holes as face holes; false when some face or edge is curved</span>
<span class="k">fn</span> face_rings(b: &amp;BRep, rings: &amp;<span class="k">mut</span> Mesh) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> planar = <span class="s">true</span>;

    <span class="k">for</span> vi <span class="k">in</span> <span class="s">0</span>..b.vertex_count() {
        rings.add_vertex(b.m_vertices[vi].point.clone(), Some(vi));
    }

    <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..b.face_count() {
        <span class="k">let</span> surface = &amp;b.m_surfaces[b.m_faces[fi].surface_index <span class="k">as</span> usize];
        <span class="k">let</span> u = surface.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> v = surface.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> frame = Plane::from_point_normal(
            Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            b.normal_at(fi, (u.<span class="s">0</span> + u.<span class="s">1</span>) * <span class="s">0</span>.<span class="s">5</span>, (v.<span class="s">0</span> + v.<span class="s">1</span>) * <span class="s">0</span>.<span class="s">5</span>),
            None,
        );
        <span class="k">let</span> <span class="k">mut</span> loops: Vec&lt;Vec&lt;usize&gt;&gt; = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> outer: Vec&lt;Point&gt; = Vec::new();
        planar = planar &amp;&amp; is_planar_patch(surface);

        <span class="k">for</span> wire <span class="k">in</span> &amp;b.m_faces[fi].wires {
            <span class="k">let</span> <span class="k">mut</span> ring: Vec&lt;usize&gt; = Vec::new();

            <span class="k">for</span> er <span class="k">in</span> b.wire_edges(wire) {
                <span class="k">let</span> edge = &amp;b.m_edges[er.index <span class="k">as</span> usize];

                <span class="k">if</span> edge.degenerated {
                    <span class="k">continue</span>;
                }

                planar = planar &amp;&amp; b.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize].degree() == <span class="s">1</span>;
                <span class="k">let</span> start = <span class="k">if</span> er.orientation == BRepOrientation::Reversed {
                    edge.end_vertex
                } <span class="k">else</span> {
                    edge.start_vertex
                };
                ring.push(start <span class="k">as</span> usize);
            }

            loops.push(ring);
        }

        <span class="k">for</span> vi <span class="k">in</span> &amp;loops[<span class="s">0</span>] {
            outer.push(b.m_vertices[*vi].point.clone());
        }

        <span class="k">if</span> signed_area_in_plane(&amp;outer, &amp;frame.origin(), &amp;frame.x_axis(), &amp;frame.y_axis()) &lt; <span class="s">0</span>.<span class="s">0</span> {
            loops[<span class="s">0</span>].reverse();
        }

        <span class="k">let</span> fk = rings.add_face(loops[<span class="s">0</span>].clone(), None);

        <span class="k">if</span> <span class="k">let</span> Some(fk) = fk {
            <span class="k">if</span> loops.len() &gt; <span class="s">1</span> {
                rings.set_face_holes(fk, loops[<span class="s">1</span>..].to_vec());
            }
        }
    }

    planar
}

<span class="c">/// Closed polyline through the positions of a vertex ring of \`mesh\`</span>
<span class="k">fn</span> ring_polyline(mesh: &amp;Mesh, ring: &amp;[usize]) -&gt; Polyline {
    <span class="k">let</span> <span class="k">mut</span> points: Vec&lt;Point&gt; = Vec::with_capacity(ring.len() + <span class="s">1</span>);

    <span class="k">for</span> vk <span class="k">in</span> ring {
        points.push(mesh.vertex[vk].position());
    }

    points.push(points[<span class="s">0</span>].clone());

    Polyline::new(points)
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Serialization helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
#[derive(Serialize, Deserialize)]
<span class="k">struct</span> RefJson {
    index: i32,          <span class="c">// Index into the owning table.</span>
    orientation: String, <span class="c">// Orientation name.</span>
}

<span class="c">/// JSON array of oriented references</span>
<span class="k">fn</span> refs_to_json(refs: &amp;[BRepRef]) -&gt; Vec&lt;RefJson&gt; {
    <span class="k">let</span> <span class="k">mut</span> arr = Vec::new();

    <span class="k">for</span> r <span class="k">in</span> refs {
        arr.push(RefJson {
            index: r.index,
            orientation: orientation_to_str(r.orientation).to_string(),
        });
    }

    arr
}

<span class="c">/// Oriented references of a JSON array</span>
<span class="k">fn</span> refs_from_json(arr: &amp;[RefJson]) -&gt; Vec&lt;BRepRef&gt; {
    <span class="k">let</span> <span class="k">mut</span> refs = Vec::new();

    <span class="k">for</span> r <span class="k">in</span> arr {
        refs.push(BRepRef::new(r.index, orientation_from_str(&amp;r.orientation)));
    }

    refs
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> PCurveJson {
    curve_2d_index: i32,   <span class="c">// Pcurve of the forward use.</span>
    curve_2d_index_2: i32, <span class="c">// Pcurve of the reversed use.</span>
    surface_index: i32,    <span class="c">// Surface the pcurve lies on.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> EdgeJson {
    curve_3d_index: i32,      <span class="c">// 3D curve.</span>
    degenerated: bool,        <span class="c">// Pole or apex edge.</span>
    end_vertex: i32,          <span class="c">// End vertex.</span>
    pcurves: Vec&lt;PCurveJson&gt;, <span class="c">// Pcurve records.</span>
    start_vertex: i32,        <span class="c">// Start vertex.</span>
    tolerance: f64,           <span class="c">// Edge tolerance.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> FaceJson {
    #[serde(skip_serializing_if = &quot;<span class="s">Option::is_none</span>&quot;)]
    #[serde(default)]
    facecolor: Option&lt;Color&gt;, <span class="c">// Display color when set.</span>
    surface_index: i32,  <span class="c">// Underlying surface.</span>
    tolerance: f64,      <span class="c">// Face tolerance.</span>
    wires: Vec&lt;RefJson&gt;, <span class="c">// Outer wire first, then holes.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> ShellJson {
    faces: Vec&lt;RefJson&gt;, <span class="c">// Oriented faces.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> SolidJson {
    shells: Vec&lt;RefJson&gt;, <span class="c">// Oriented shells.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> VertexJson {
    point: [f64; <span class="s">3</span>], <span class="c">// Position.</span>
    tolerance: f64,  <span class="c">// Vertex tolerance.</span>
}

#[derive(Serialize, Deserialize)]
<span class="k">struct</span> WireJson {
    edges: Vec&lt;RefJson&gt;, <span class="c">// Oriented edges.</span>
}

<span class="c">/// Oriented references as a repeated proto field</span>
<span class="k">fn</span> refs_to_proto(refs: &amp;[BRepRef]) -&gt; Vec&lt;<span class="k">crate</span>::proto::BRepRef&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

    <span class="k">for</span> r <span class="k">in</span> refs {
        out.push(<span class="k">crate</span>::proto::BRepRef {
            index: r.index,
            orientation: r.orientation <span class="k">as</span> i32,
        });
    }

    out
}

<span class="c">/// Oriented references of a repeated proto field</span>
<span class="k">fn</span> refs_from_proto(refs: &amp;[<span class="k">crate</span>::proto::BRepRef]) -&gt; Vec&lt;BRepRef&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

    <span class="k">for</span> r <span class="k">in</span> refs {
        out.push(BRepRef::new(r.index, orientation_from_i32(r.orientation)));
    }

    out
}

<span class="c">/// Proto message of an edge</span>
<span class="k">fn</span> edge_to_proto(e: &amp;BRepEdge) -&gt; <span class="k">crate</span>::proto::BRepEdge {
    <span class="k">let</span> <span class="k">mut</span> p = <span class="k">crate</span>::proto::BRepEdge {
        curve_3d_index: e.curve_3d_index,
        start_vertex: e.start_vertex,
        end_vertex: e.end_vertex,
        tolerance: e.tolerance,
        degenerated: e.degenerated,
        pcurves: Vec::new(),
    };

    <span class="k">for</span> pc <span class="k">in</span> &amp;e.pcurves {
        p.pcurves.push(<span class="k">crate</span>::proto::BRepCurveOnSurface {
            surface_index: pc.surface_index,
            curve_2d_index: pc.curve_2d_index,
            curve_2d_index_2: pc.curve_2d_index_2,
        });
    }

    p
}

<span class="c">/// Edge of a proto message</span>
<span class="k">fn</span> edge_from_proto(e: &amp;<span class="k">crate</span>::proto::BRepEdge) -&gt; BRepEdge {
    <span class="k">let</span> <span class="k">mut</span> be = BRepEdge {
        curve_3d_index: e.curve_3d_index,
        start_vertex: e.start_vertex,
        end_vertex: e.end_vertex,
        tolerance: e.tolerance,
        degenerated: e.degenerated,
        pcurves: Vec::new(),
    };

    <span class="k">for</span> pc <span class="k">in</span> &amp;e.pcurves {
        be.pcurves.push(BRepCurveOnSurface {
            surface_index: pc.surface_index,
            curve_2d_index: pc.curve_2d_index,
            curve_2d_index_2: pc.curve_2d_index_2,
        });
    }

    be
}

<span class="c">/// Proto message of a face, facecolor only when set</span>
<span class="k">fn</span> face_to_proto(f: &amp;BRepFace) -&gt; <span class="k">crate</span>::proto::BRepFace {
    <span class="k">let</span> <span class="k">mut</span> p = <span class="k">crate</span>::proto::BRepFace {
        surface_index: f.surface_index,
        wires: refs_to_proto(&amp;f.wires),
        tolerance: f.tolerance,
        facecolor: None,
    };

    <span class="k">if</span> <span class="k">let</span> Some(fc) = &amp;f.facecolor {
        p.facecolor = Some(fc.to_proto());
    }

    p
}

<span class="c">/// Face of a proto message</span>
<span class="k">fn</span> face_from_proto(f: &amp;<span class="k">crate</span>::proto::BRepFace) -&gt; BRepFace {
    <span class="k">let</span> <span class="k">mut</span> bf = BRepFace {
        surface_index: f.surface_index,
        wires: refs_from_proto(&amp;f.wires),
        tolerance: f.tolerance,
        facecolor: None,
    };

    <span class="k">if</span> <span class="k">let</span> Some(fc) = &amp;f.facecolor {
        bf.facecolor = Some(Color::from_proto(fc.clone()));
    }

    bf
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// BRep</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Boundary representation after OCCT's TopoDS/BRep model: geometry pools, indexed shape tables, every parent -&gt; child link a BRepRef carrying the orientation</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> BRep {
    guid: std::sync::OnceLock&lt;String&gt;, <span class="c">// Lazily minted GUID.</span>
    <span class="k">pub</span> name: String,                  <span class="c">// BRep name.</span>
    <span class="k">pub</span> width: f64,                    <span class="c">// Display width.</span>
    <span class="k">pub</span> surfacecolor: Color,           <span class="c">// Display color of the faces.</span>
    <span class="k">pub</span> m_surfaces: Vec&lt;NurbsSurface&gt;, <span class="c">// Surface pool.</span>
    <span class="k">pub</span> m_curves_3d: Vec&lt;NurbsCurve&gt;,  <span class="c">// 3D edge curve pool.</span>
    <span class="k">pub</span> m_curves_2d: Vec&lt;NurbsCurve&gt;,  <span class="c">// Pcurve pool.</span>
    <span class="k">pub</span> m_vertices: Vec&lt;BRepVertex&gt;,   <span class="c">// Vertex table.</span>
    <span class="k">pub</span> m_edges: Vec&lt;BRepEdge&gt;,        <span class="c">// Edge table.</span>
    <span class="k">pub</span> m_wires: Vec&lt;BRepWire&gt;,        <span class="c">// Wire table.</span>
    <span class="k">pub</span> m_faces: Vec&lt;BRepFace&gt;,        <span class="c">// Face table.</span>
    <span class="k">pub</span> m_shells: Vec&lt;BRepShell&gt;,      <span class="c">// Shell table.</span>
    <span class="k">pub</span> m_solids: Vec&lt;BRepSolid&gt;,      <span class="c">// Solid table.</span>
}

<span class="k">impl</span> Default <span class="k">for</span> BRep {
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}

<span class="k">impl</span> BRep {
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Constructors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Construct an empty BRep.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        BRep {
            guid: std::sync::OnceLock::new(),
            name: &quot;<span class="s">my_brep</span>&quot;.to_string(),
            width: <span class="s">1</span>.<span class="s">0</span>,
            surfacecolor: Color::lightgrey(),
            m_surfaces: Vec::new(),
            m_curves_3d: Vec::new(),
            m_curves_2d: Vec::new(),
            m_vertices: Vec::new(),
            m_edges: Vec::new(),
            m_wires: Vec::new(),
            m_faces: Vec::new(),
            m_shells: Vec::new(),
            m_solids: Vec::new(),
        }
    }

    <span class="c">/// Copy with a new guid and the same data.</span>
    <span class="k">pub</span> <span class="k">fn</span> duplicate(&amp;<span class="k">self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> copy = <span class="k">self</span>.clone();
        copy.guid = std::sync::OnceLock::new();

        copy
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Static constructors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Axis-aligned box centered at the origin: 6 faces, 12 edges, 8 vertices, one solid</span>
    <span class="k">pub</span> <span class="k">fn</span> create_box(sx: f64, sy: f64, sz: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">box</span>&quot;.to_string();
        box_corners(&amp;<span class="k">mut</span> b, sx, sy, sz);
        <span class="k">let</span> <span class="k">mut</span> pb = PolyFaceBuilder::new();
        <span class="k">let</span> <span class="k">mut</span> faces = Vec::new();

        <span class="k">for</span> fv <span class="k">in</span> &amp;BOX_FACES {
            <span class="k">let</span> srf = quad_patch(&amp;b, fv);
            faces.push(BRepRef::new(pb.face(&amp;<span class="k">mut</span> b, &amp;srf, fv, &amp;[]) <span class="k">as</span> i32, F));
        }

        <span class="k">let</span> sh = b.add_shell(&amp;faces);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Cylinder along +Z: one periodic body face (seam edge) and two planar caps</span>
    <span class="k">pub</span> <span class="k">fn</span> create_cylinder(radius: f64, height: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">cylinder</span>&quot;.to_string();
        <span class="k">let</span> body = Primitives::cylinder_surface(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius, height);
        <span class="k">let</span> p_bot = body.point_at_corner(<span class="s">0</span>, <span class="s">0</span>).unwrap_or_default();
        <span class="k">let</span> p_top = body.point_at_corner(<span class="s">0</span>, <span class="s">1</span>).unwrap_or_default();
        <span class="k">let</span> v_bot = b.add_vertex(&amp;p_bot, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> v_top = b.add_vertex(&amp;p_top, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> c_bot = b.add_curve_3d(&amp;Primitives::circle(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius)) <span class="k">as</span> i32;
        <span class="k">let</span> e_bot = b.add_edge(c_bot, v_bot, v_bot);
        <span class="k">let</span> c_top = b.add_curve_3d(&amp;Primitives::circle(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, height, radius)) <span class="k">as</span> i32;
        <span class="k">let</span> e_top = b.add_edge(c_top, v_top, v_top);
        <span class="k">let</span> c_seam = b.add_curve_3d(&amp;NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;[p_bot, p_top])) <span class="k">as</span> i32;
        <span class="k">let</span> e_seam = b.add_edge(c_seam, v_bot, v_top);
        <span class="k">let</span> si = b.add_surface(&amp;body);
        <span class="k">let</span> f_body = body_face(&amp;<span class="k">mut</span> b, si, e_bot, e_seam, e_top);
        <span class="k">let</span> f_bot = cap_face(&amp;<span class="k">mut</span> b, &amp;cap_patch(radius, <span class="s">0</span>.<span class="s">0</span>, <span class="s">false</span>), e_bot);
        <span class="k">let</span> f_top = cap_face(&amp;<span class="k">mut</span> b, &amp;cap_patch(radius, height, <span class="s">true</span>), e_top);
        <span class="k">let</span> sh = b.add_shell(&amp;[
            BRepRef::new(f_body <span class="k">as</span> i32, F),
            BRepRef::new(f_bot <span class="k">as</span> i32, F),
            BRepRef::new(f_top <span class="k">as</span> i32, F),
        ]);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Sphere centered at the origin: one face, a seam meridian and two degenerated pole edges</span>
    <span class="k">pub</span> <span class="k">fn</span> create_sphere(radius: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">sphere</span>&quot;.to_string();
        <span class="k">let</span> srf = Primitives::sphere_surface(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius);
        <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> v_s = b.add_vertex(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -radius), <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> v_n = b.add_vertex(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius), <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> c_seam = b.add_curve_3d(&amp;srf.iso_curve(<span class="s">1</span>, u0).unwrap_or_default()) <span class="k">as</span> i32;
        <span class="k">let</span> e_seam = b.add_edge(c_seam, v_s, v_n);
        <span class="k">let</span> e_south = b.add_edge(-<span class="s">1</span>, v_s, v_s);
        <span class="k">let</span> e_north = b.add_edge(-<span class="s">1</span>, v_n, v_n);
        <span class="k">let</span> si = b.add_surface(&amp;srf);
        <span class="k">let</span> c_south = b.add_curve_2d(&amp;uv_line(u0, v0, u1, v0)) <span class="k">as</span> i32;
        b.add_pcurve(e_south, si, c_south, -<span class="s">1</span>);
        <span class="k">let</span> c_north = b.add_curve_2d(&amp;uv_line(u0, v1, u1, v1)) <span class="k">as</span> i32;
        b.add_pcurve(e_north, si, c_north, -<span class="s">1</span>);
        <span class="k">let</span> c_right = b.add_curve_2d(&amp;uv_line(u1, v0, u1, v1)) <span class="k">as</span> i32;
        <span class="k">let</span> c_left = b.add_curve_2d(&amp;uv_line(u0, v0, u0, v1)) <span class="k">as</span> i32;
        b.add_pcurve(e_seam, si, c_right, c_left);
        <span class="k">let</span> wi = b.add_wire(&amp;[
            BRepRef::new(e_south <span class="k">as</span> i32, F),
            BRepRef::new(e_seam <span class="k">as</span> i32, F),
            BRepRef::new(e_north <span class="k">as</span> i32, R),
            BRepRef::new(e_seam <span class="k">as</span> i32, R),
        ]);
        <span class="k">let</span> fi = b.add_face(si <span class="k">as</span> i32, &amp;[BRepRef::new(wi <span class="k">as</span> i32, F)], <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> sh = b.add_shell(&amp;[BRepRef::new(fi <span class="k">as</span> i32, F)]);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Cone along +Z: base circle at z=0, apex at z=height (degenerated apex edge), planar base</span>
    <span class="k">pub</span> <span class="k">fn</span> create_cone(radius: f64, height: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">cone</span>&quot;.to_string();
        <span class="k">let</span> body = Primitives::cone_surface(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius, height);
        <span class="k">let</span> p_base = body.point_at_corner(<span class="s">0</span>, <span class="s">0</span>).unwrap_or_default();
        <span class="k">let</span> p_apex = Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, height);
        <span class="k">let</span> v_base = b.add_vertex(&amp;p_base, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> v_apex = b.add_vertex(&amp;p_apex, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> c_base = b.add_curve_3d(&amp;Primitives::circle(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, radius)) <span class="k">as</span> i32;
        <span class="k">let</span> e_base = b.add_edge(c_base, v_base, v_base);
        <span class="k">let</span> c_seam = b.add_curve_3d(&amp;NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;[p_base, p_apex])) <span class="k">as</span> i32;
        <span class="k">let</span> e_seam = b.add_edge(c_seam, v_base, v_apex);
        <span class="k">let</span> e_apex = b.add_edge(-<span class="s">1</span>, v_apex, v_apex);
        <span class="k">let</span> si = b.add_surface(&amp;body);
        <span class="k">let</span> f_body = body_face(&amp;<span class="k">mut</span> b, si, e_base, e_seam, e_apex);
        <span class="k">let</span> f_base = cap_face(&amp;<span class="k">mut</span> b, &amp;cap_patch(radius, <span class="s">0</span>.<span class="s">0</span>, <span class="s">false</span>), e_base);
        <span class="k">let</span> sh = b.add_shell(&amp;[
            BRepRef::new(f_body <span class="k">as</span> i32, F),
            BRepRef::new(f_base <span class="k">as</span> i32, F),
        ]);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Square pyramid: base edge \`base\` centered at the origin in z=0, apex at (0,0,height)</span>
    <span class="k">pub</span> <span class="k">fn</span> create_pyramid(base: f64, height: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">pyramid</span>&quot;.to_string();
        <span class="k">let</span> h = base * <span class="s">0</span>.<span class="s">5</span>;
        b.add_vertex(&amp;Point::new(-h, -h, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">0</span>);
        b.add_vertex(&amp;Point::new(h, -h, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">0</span>);
        b.add_vertex(&amp;Point::new(h, h, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">0</span>);
        b.add_vertex(&amp;Point::new(-h, h, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> v_apex = b.add_vertex(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, height), <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> pb = PolyFaceBuilder::new();
        <span class="k">let</span> fv = [<span class="s">0usize</span>, <span class="s">3</span>, <span class="s">2</span>, <span class="s">1</span>];
        <span class="k">let</span> base_srf = quad_patch(&amp;b, &amp;fv);
        <span class="k">let</span> <span class="k">mut</span> faces = vec![BRepRef::new(pb.face(&amp;<span class="k">mut</span> b, &amp;base_srf, &amp;fv, &amp;[]) <span class="k">as</span> i32, F)];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">4usize</span> {
            <span class="k">let</span> a = i;
            <span class="k">let</span> c = (i + <span class="s">1</span>) % <span class="s">4</span>;
            <span class="k">let</span> srf = bilinear_patch(
                &amp;b.m_vertices[a].point,
                &amp;b.m_vertices[c].point,
                &amp;b.m_vertices[v_apex].point,
                &amp;b.m_vertices[v_apex].point,
            );
            <span class="k">let</span> si = b.add_surface(&amp;srf);
            <span class="k">let</span> e_ac = pb.edge(&amp;<span class="k">mut</span> b, a, c);
            <span class="k">let</span> e_c = pb.edge(&amp;<span class="k">mut</span> b, c, v_apex);
            <span class="k">let</span> e_a = pb.edge(&amp;<span class="k">mut</span> b, a, v_apex);
            <span class="k">let</span> e_deg = b.add_edge(-<span class="s">1</span>, v_apex <span class="k">as</span> i32, v_apex <span class="k">as</span> i32);
            <span class="k">let</span> ac_fwd = b.m_edges[e_ac].start_vertex == a <span class="k">as</span> i32;
            <span class="k">let</span> c_ac = b.add_curve_2d(&amp;<span class="k">if</span> ac_fwd {
                uv_line(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)
            } <span class="k">else</span> {
                uv_line(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)
            }) <span class="k">as</span> i32;
            b.add_pcurve(e_ac, si, c_ac, -<span class="s">1</span>);
            <span class="k">let</span> c_c = b.add_curve_2d(&amp;uv_line(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)) <span class="k">as</span> i32;
            b.add_pcurve(e_c, si, c_c, -<span class="s">1</span>);
            <span class="k">let</span> c_deg = b.add_curve_2d(&amp;uv_line(<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)) <span class="k">as</span> i32;
            b.add_pcurve(e_deg, si, c_deg, -<span class="s">1</span>);
            <span class="k">let</span> c_a = b.add_curve_2d(&amp;uv_line(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)) <span class="k">as</span> i32;
            b.add_pcurve(e_a, si, c_a, -<span class="s">1</span>);
            <span class="k">let</span> wire = b.add_wire(&amp;[
                BRepRef::new(e_ac <span class="k">as</span> i32, <span class="k">if</span> ac_fwd { F } <span class="k">else</span> { R }),
                BRepRef::new(e_c <span class="k">as</span> i32, F),
                BRepRef::new(e_deg <span class="k">as</span> i32, F),
                BRepRef::new(e_a <span class="k">as</span> i32, R),
            ]);
            faces.push(BRepRef::new(
                b.add_face(si <span class="k">as</span> i32, &amp;[BRepRef::new(wire <span class="k">as</span> i32, F)], <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32,
                F,
            ));
        }

        <span class="k">let</span> sh = b.add_shell(&amp;faces);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Torus in the XY plane: one face closed in both directions, two seam edges, one vertex</span>
    <span class="k">pub</span> <span class="k">fn</span> create_torus(major_radius: f64, minor_radius: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">torus</span>&quot;.to_string();
        <span class="k">let</span> srf = Primitives::torus_surface(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, major_radius, minor_radius);
        <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> v = b.add_vertex(&amp;srf.point_at_corner(<span class="s">0</span>, <span class="s">0</span>).unwrap_or_default(), <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> c_u = b.add_curve_3d(&amp;srf.iso_curve(<span class="s">1</span>, u0).unwrap_or_default()) <span class="k">as</span> i32;
        <span class="k">let</span> e_u = b.add_edge(c_u, v, v);
        <span class="k">let</span> c_v = b.add_curve_3d(&amp;srf.iso_curve(<span class="s">0</span>, v0).unwrap_or_default()) <span class="k">as</span> i32;
        <span class="k">let</span> e_v = b.add_edge(c_v, v, v);
        <span class="k">let</span> si = b.add_surface(&amp;srf);
        <span class="k">let</span> c_bottom = b.add_curve_2d(&amp;uv_line(u0, v0, u1, v0)) <span class="k">as</span> i32;
        <span class="k">let</span> c_top = b.add_curve_2d(&amp;uv_line(u0, v1, u1, v1)) <span class="k">as</span> i32;
        b.add_pcurve(e_v, si, c_bottom, c_top);
        <span class="k">let</span> c_right = b.add_curve_2d(&amp;uv_line(u1, v0, u1, v1)) <span class="k">as</span> i32;
        <span class="k">let</span> c_left = b.add_curve_2d(&amp;uv_line(u0, v0, u0, v1)) <span class="k">as</span> i32;
        b.add_pcurve(e_u, si, c_right, c_left);
        <span class="k">let</span> wi = b.add_wire(&amp;[
            BRepRef::new(e_v <span class="k">as</span> i32, F),
            BRepRef::new(e_u <span class="k">as</span> i32, F),
            BRepRef::new(e_v <span class="k">as</span> i32, R),
            BRepRef::new(e_u <span class="k">as</span> i32, R),
        ]);
        <span class="k">let</span> fi = b.add_face(si <span class="k">as</span> i32, &amp;[BRepRef::new(wi <span class="k">as</span> i32, F)], <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> sh = b.add_shell(&amp;[BRepRef::new(fi <span class="k">as</span> i32, F)]);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// Axis-aligned box with a cylindrical through-hole along Z</span>
    <span class="k">pub</span> <span class="k">fn</span> create_block_with_hole(sx: f64, sy: f64, sz: f64, hole_radius: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">block_with_hole</span>&quot;.to_string();
        <span class="k">let</span> hz = sz * <span class="s">0</span>.<span class="s">5</span>;
        box_corners(&amp;<span class="k">mut</span> b, sx, sy, sz);
        <span class="k">let</span> <span class="k">mut</span> pb = PolyFaceBuilder::new();
        <span class="k">let</span> <span class="k">mut</span> faces = Vec::new();

        <span class="k">for</span> fv <span class="k">in</span> &amp;BOX_FACES[<span class="s">2</span>..] {
            <span class="k">let</span> srf = quad_patch(&amp;b, fv);
            faces.push(BRepRef::new(pb.face(&amp;<span class="k">mut</span> b, &amp;srf, fv, &amp;[]) <span class="k">as</span> i32, F));
        }

        <span class="k">let</span> p_bot = Point::new(hole_radius, <span class="s">0</span>.<span class="s">0</span>, -hz);
        <span class="k">let</span> p_top = Point::new(hole_radius, <span class="s">0</span>.<span class="s">0</span>, hz);
        <span class="k">let</span> v_bot = b.add_vertex(&amp;p_bot, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> v_top = b.add_vertex(&amp;p_top, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32;
        <span class="k">let</span> c_bot = b.add_curve_3d(&amp;Primitives::circle(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -hz, hole_radius)) <span class="k">as</span> i32;
        <span class="k">let</span> e_bot = b.add_edge(c_bot, v_bot, v_bot);
        <span class="k">let</span> c_top = b.add_curve_3d(&amp;Primitives::circle(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, hz, hole_radius)) <span class="k">as</span> i32;
        <span class="k">let</span> e_top = b.add_edge(c_top, v_top, v_top);
        <span class="k">let</span> c_seam = b.add_curve_3d(&amp;NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;[p_bot, p_top])) <span class="k">as</span> i32;
        <span class="k">let</span> e_seam = b.add_edge(c_seam, v_bot, v_top);
        <span class="k">let</span> bore = Primitives::cylinder_surface(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -hz, hole_radius, sz);
        <span class="k">let</span> si_bore = b.add_surface(&amp;bore);
        faces.push(BRepRef::new(
            body_face(&amp;<span class="k">mut</span> b, si_bore, e_bot, e_seam, e_top) <span class="k">as</span> i32,
            R,
        ));

        <span class="k">for</span> (fi, fv) <span class="k">in</span> BOX_FACES.iter().enumerate().take(<span class="s">2</span>) {
            <span class="k">let</span> cap = quad_patch(&amp;b, fv);
            <span class="k">let</span> si = b.add_surface(&amp;cap);
            <span class="k">let</span> outer = pb.wire_refs(&amp;<span class="k">mut</span> b, si, fv);
            <span class="k">let</span> e_hole = <span class="k">if</span> fi == <span class="s">0</span> { e_bot } <span class="k">else</span> { e_top };
            <span class="k">let</span> c2d = project_to_patch(
                &amp;b.m_curves_3d[b.m_edges[e_hole].curve_3d_index <span class="k">as</span> usize],
                &amp;cap,
            );
            <span class="k">let</span> o = <span class="k">if</span> uv_signed_area(&amp;c2d) &lt; <span class="s">0</span>.<span class="s">0</span> { F } <span class="k">else</span> { R };
            <span class="k">let</span> ci = b.add_curve_2d(&amp;c2d) <span class="k">as</span> i32;
            b.add_pcurve(e_hole, si, ci, -<span class="s">1</span>);
            <span class="k">let</span> w_outer = b.add_wire(&amp;outer);
            <span class="k">let</span> w_inner = b.add_wire(&amp;[BRepRef::new(e_hole <span class="k">as</span> i32, o)]);
            <span class="k">let</span> wires = [
                BRepRef::new(w_outer <span class="k">as</span> i32, F),
                BRepRef::new(w_inner <span class="k">as</span> i32, F),
            ];
            faces.push(BRepRef::new(b.add_face(si <span class="k">as</span> i32, &amp;wires, <span class="s">0</span>.<span class="s">0</span>) <span class="k">as</span> i32, F));
        }

        <span class="k">let</span> sh = b.add_shell(&amp;faces);
        b.add_solid(&amp;[BRepRef::new(sh <span class="k">as</span> i32, F)]);

        b
    }

    <span class="c">/// One planar face per closed polyline, holes[i] the closed polylines bounding the holes of face i; coincident vertices and edges are shared, closed sheets become solids</span>
    <span class="k">pub</span> <span class="k">fn</span> from_polylines(polylines: &amp;[Polyline], holes: &amp;[Vec&lt;Polyline&gt;]) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">polysurface</span>&quot;.to_string();
        <span class="k">let</span> tol = <span class="s">1</span>e-<span class="s">6</span>;
        <span class="k">let</span> <span class="k">mut</span> pb = PolyFaceBuilder::new();

        <span class="k">for</span> pi <span class="k">in</span> <span class="s">0</span>..polylines.len() {
            <span class="k">let</span> pts = open_points(&amp;polylines[pi]);

            <span class="k">if</span> pts.len() &lt; <span class="s">3</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> (org, plane) = polylines[pi].get_fast_plane();

            <span class="k">if</span> !plane.is_valid() {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> xa = plane.x_axis();
            <span class="k">let</span> ya = plane.y_axis();
            <span class="k">let</span> outer_area = signed_area_in_plane(&amp;pts, &amp;org, &amp;xa, &amp;ya);
            <span class="k">let</span> <span class="k">mut</span> vi = Vec::new();

            <span class="k">for</span> pt <span class="k">in</span> &amp;pts {
                vi.push(find_or_add_vertex(&amp;<span class="k">mut</span> b, pt, tol));
            }

            <span class="k">let</span> <span class="k">mut</span> all_pts = pts.clone();
            <span class="k">let</span> <span class="k">mut</span> hole_cycles: Vec&lt;Vec&lt;usize&gt;&gt; = Vec::new();

            <span class="k">if</span> pi &lt; holes.len() {
                <span class="k">for</span> h <span class="k">in</span> &amp;holes[pi] {
                    <span class="k">let</span> <span class="k">mut</span> hp = open_points(h);

                    <span class="k">if</span> hp.len() &lt; <span class="s">3</span> {
                        <span class="k">continue</span>;
                    }

                    <span class="k">if</span> signed_area_in_plane(&amp;hp, &amp;org, &amp;xa, &amp;ya) * outer_area &gt; <span class="s">0</span>.<span class="s">0</span> {
                        hp.reverse();
                    }

                    <span class="k">let</span> <span class="k">mut</span> cycle = Vec::new();

                    <span class="k">for</span> pt <span class="k">in</span> &amp;hp {
                        cycle.push(find_or_add_vertex(&amp;<span class="k">mut</span> b, pt, tol));
                    }

                    hole_cycles.push(cycle);
                    all_pts.extend(hp);
                }
            }

            <span class="k">let</span> srf = planar_patch_through(&amp;all_pts, &amp;org, &amp;xa, &amp;ya);
            pb.face(&amp;<span class="k">mut</span> b, &amp;srf, &amp;vi, &amp;hole_cycles);
        }

        close_free_faces(&amp;<span class="k">mut</span> b);

        b
    }

    <span class="c">/// One planar face per closed curve with optional hole curves (inner wires); closed sheets become solids</span>
    <span class="k">pub</span> <span class="k">fn</span> from_nurbscurves(curves: &amp;[NurbsCurve], holes: &amp;[Vec&lt;NurbsCurve&gt;]) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();
        b.name = &quot;<span class="s">polysurface</span>&quot;.to_string();
        <span class="k">let</span> tol = <span class="s">1</span>e-<span class="s">6</span>;

        <span class="k">for</span> ci <span class="k">in</span> <span class="s">0</span>..curves.len() {
            <span class="k">let</span> crv = &amp;curves[ci];
            <span class="k">let</span> <span class="k">mut</span> pts = cv_points(crv);

            <span class="k">if</span> pts.len() &gt;= <span class="s">2</span> &amp;&amp; pts[<span class="s">0</span>].distance(&amp;pts[pts.len() - <span class="s">1</span>], None) &lt; tol {
                pts.pop();
            }

            <span class="k">if</span> pts.len() &lt; <span class="s">3</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> (org, plane) = Polyline::new(pts.clone()).get_fast_plane();

            <span class="k">if</span> !plane.is_valid() {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> ci &lt; holes.len() {
                <span class="k">for</span> h <span class="k">in</span> &amp;holes[ci] {
                    pts.extend(cv_points(h));
                }
            }

            <span class="k">let</span> si = b.add_surface(&amp;planar_patch_through(
                &amp;pts,
                &amp;org,
                &amp;plane.x_axis(),
                &amp;plane.y_axis(),
            ));
            <span class="k">let</span> <span class="k">mut</span> wires = vec![BRepRef::new(curve_wire(&amp;<span class="k">mut</span> b, crv, si, tol) <span class="k">as</span> i32, F)];

            <span class="k">if</span> ci &lt; holes.len() {
                <span class="k">for</span> h <span class="k">in</span> &amp;holes[ci] {
                    wires.push(BRepRef::new(curve_wire(&amp;<span class="k">mut</span> b, h, si, tol) <span class="k">as</span> i32, F));
                }
            }

            b.add_face(si <span class="k">as</span> i32, &amp;wires, <span class="s">0</span>.<span class="s">0</span>);
        }

        close_free_faces(&amp;<span class="k">mut</span> b);

        b
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Accessors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return whether the lazy guid has been created.</span>
    <span class="k">pub</span> <span class="k">fn</span> has_guid(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.guid.get().is_some()
    }

    <span class="c">/// Return the guid, creating it on first access.</span>
    <span class="k">pub</span> <span class="k">fn</span> guid(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">self</span>.guid.get_or_init(|| uuid::Uuid::new_v4().to_string())
    }

    <span class="c">/// Set the guid.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_guid(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: String) {
        <span class="k">self</span>.guid = std::sync::OnceLock::from(guid);
    }

    <span class="c">/// Clear the guid so a fresh one mints lazily on next read</span>
    <span class="k">pub</span> <span class="k">fn</span> refresh_guid(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.guid = std::sync::OnceLock::new();
    }

    <span class="c">/// Return the number of vertices.</span>
    <span class="k">pub</span> <span class="k">fn</span> vertex_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_vertices.len()
    }

    <span class="c">/// Return the number of edges.</span>
    <span class="k">pub</span> <span class="k">fn</span> edge_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_edges.len()
    }

    <span class="c">/// Return the number of wires.</span>
    <span class="k">pub</span> <span class="k">fn</span> wire_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_wires.len()
    }

    <span class="c">/// Return the number of faces.</span>
    <span class="k">pub</span> <span class="k">fn</span> face_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_faces.len()
    }

    <span class="c">/// Return the number of shells.</span>
    <span class="k">pub</span> <span class="k">fn</span> shell_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_shells.len()
    }

    <span class="c">/// Return the number of solids.</span>
    <span class="k">pub</span> <span class="k">fn</span> solid_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_solids.len()
    }

    <span class="c">/// Every reference resolves into its table, every face has a surface and an outer wire, every edge two vertices and (unless degenerated) a 3D curve</span>
    <span class="k">pub</span> <span class="k">fn</span> is_valid(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.m_faces.is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">for</span> e <span class="k">in</span> &amp;<span class="k">self</span>.m_edges {
            <span class="k">if</span> !in_range(e.start_vertex, <span class="k">self</span>.m_vertices.len())
                || !in_range(e.end_vertex, <span class="k">self</span>.m_vertices.len())
            {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">if</span> !e.degenerated &amp;&amp; !in_range(e.curve_3d_index, <span class="k">self</span>.m_curves_3d.len()) {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">for</span> pc <span class="k">in</span> &amp;e.pcurves {
                <span class="k">if</span> !in_range(pc.surface_index, <span class="k">self</span>.m_surfaces.len())
                    || !in_range(pc.curve_2d_index, <span class="k">self</span>.m_curves_2d.len())
                {
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">if</span> pc.curve_2d_index_2 &gt;= <span class="s">0</span>
                    &amp;&amp; !in_range(pc.curve_2d_index_2, <span class="k">self</span>.m_curves_2d.len())
                {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="k">for</span> w <span class="k">in</span> &amp;<span class="k">self</span>.m_wires {
            <span class="k">if</span> w.edges.is_empty() {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">for</span> r <span class="k">in</span> &amp;w.edges {
                <span class="k">if</span> !in_range(r.index, <span class="k">self</span>.m_edges.len()) {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="k">for</span> f <span class="k">in</span> &amp;<span class="k">self</span>.m_faces {
            <span class="k">if</span> !in_range(f.surface_index, <span class="k">self</span>.m_surfaces.len()) || f.wires.is_empty() {
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">for</span> r <span class="k">in</span> &amp;f.wires {
                <span class="k">if</span> !in_range(r.index, <span class="k">self</span>.m_wires.len()) {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_shells {
            <span class="k">for</span> r <span class="k">in</span> &amp;s.faces {
                <span class="k">if</span> !in_range(r.index, <span class="k">self</span>.m_faces.len()) {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_solids {
            <span class="k">for</span> r <span class="k">in</span> &amp;s.shells {
                <span class="k">if</span> !in_range(r.index, <span class="k">self</span>.m_shells.len()) {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="s">true</span>
    }

    <span class="c">/// BRep_Tool::IsClosed(shell): every non-degenerated edge is used exactly twice by the shell's faces (a seam counts twice through its two pcurves)</span>
    <span class="k">pub</span> <span class="k">fn</span> is_closed(&amp;<span class="k">self</span>, shell_index: usize) -&gt; bool {
        <span class="k">if</span> shell_index &gt;= <span class="k">self</span>.m_shells.len() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> uses = vec![<span class="s">0usize</span>; <span class="k">self</span>.m_edges.len()];

        <span class="k">for</span> fr <span class="k">in</span> &amp;<span class="k">self</span>.m_shells[shell_index].faces {
            <span class="k">for</span> wr <span class="k">in</span> &amp;<span class="k">self</span>.m_faces[fr.index <span class="k">as</span> usize].wires {
                <span class="k">for</span> er <span class="k">in</span> <span class="k">self</span>.wire_edges(wr) {
                    uses[er.index <span class="k">as</span> usize] += <span class="s">1</span>;
                }
            }
        }

        <span class="k">for</span> (i, e) <span class="k">in</span> <span class="k">self</span>.m_edges.iter().enumerate() {
            <span class="k">if</span> !e.degenerated &amp;&amp; uses[i] != <span class="s">0</span> &amp;&amp; uses[i] != <span class="s">2</span> {
                <span class="k">return</span> <span class="s">false</span>;
            }
        }

        !<span class="k">self</span>.m_shells[shell_index].faces.is_empty()
    }

    <span class="c">/// At least one solid, and every shell of every solid is closed</span>
    <span class="k">pub</span> <span class="k">fn</span> is_solid(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.m_solids.is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_solids {
            <span class="k">for</span> r <span class="k">in</span> &amp;s.shells {
                <span class="k">if</span> !<span class="k">self</span>.is_closed(r.index <span class="k">as</span> usize) {
                    <span class="k">return</span> <span class="s">false</span>;
                }
            }
        }

        <span class="s">true</span>
    }

    <span class="c">/// Orientation of a face inside its first parent shell; Forward for a free face</span>
    <span class="k">pub</span> <span class="k">fn</span> face_orientation(&amp;<span class="k">self</span>, face_index: usize) -&gt; BRepOrientation {
        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_shells {
            <span class="k">for</span> r <span class="k">in</span> &amp;s.faces {
                <span class="k">if</span> r.index <span class="k">as</span> usize == face_index {
                    <span class="k">return</span> r.orientation;
                }
            }
        }

        BRepOrientation::Forward
    }

    <span class="c">/// BRep_Tool::CurveOnSurface(E, F): the pcurve index of an edge on a face's surface for the given use orientation (the REVERSED pcurve on a seam); -1 if none</span>
    <span class="k">pub</span> <span class="k">fn</span> pcurve_index(
        &amp;<span class="k">self</span>,
        edge_index: usize,
        face_index: usize,
        orientation: BRepOrientation,
    ) -&gt; i32 {
        <span class="k">if</span> edge_index &gt;= <span class="k">self</span>.m_edges.len() {
            <span class="k">return</span> -<span class="s">1</span>;
        }

        <span class="k">if</span> face_index &gt;= <span class="k">self</span>.m_faces.len() {
            <span class="k">return</span> -<span class="s">1</span>;
        }

        <span class="k">let</span> si = <span class="k">self</span>.m_faces[face_index].surface_index;

        <span class="k">for</span> pc <span class="k">in</span> &amp;<span class="k">self</span>.m_edges[edge_index].pcurves {
            <span class="k">if</span> pc.surface_index == si {
                <span class="k">if</span> orientation == BRepOrientation::Reversed &amp;&amp; pc.curve_2d_index_2 &gt;= <span class="s">0</span> {
                    <span class="k">return</span> pc.curve_2d_index_2;
                }

                <span class="k">return</span> pc.curve_2d_index;
            }
        }

        -<span class="s">1</span>
    }

    <span class="c">/// The edges of a wire composed with the wire's own orientation (a Reversed wire is traversed backwards with every edge reversed)</span>
    <span class="k">pub</span> <span class="k">fn</span> wire_edges(&amp;<span class="k">self</span>, wire: &amp;BRepRef) -&gt; Vec&lt;BRepRef&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">if</span> wire.index &lt; <span class="s">0</span> || wire.index <span class="k">as</span> usize &gt;= <span class="k">self</span>.m_wires.len() {
            <span class="k">return</span> out;
        }

        <span class="k">for</span> r <span class="k">in</span> &amp;<span class="k">self</span>.m_wires[wire.index <span class="k">as</span> usize].edges {
            out.push(BRepRef::new(
                r.index,
                brep_compose(wire.orientation, r.orientation),
            ));
        }

        <span class="k">if</span> wire.orientation == BRepOrientation::Reversed {
            out.reverse();
        }

        out
    }

    <span class="c">/// Faces sharing an edge, each with the orientation of that edge use</span>
    <span class="k">pub</span> <span class="k">fn</span> edge_faces(&amp;<span class="k">self</span>, edge_index: usize) -&gt; Vec&lt;BRepRef&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.m_faces.len() {
            <span class="k">let</span> fo = <span class="k">self</span>.face_orientation(fi);

            <span class="k">for</span> wr <span class="k">in</span> &amp;<span class="k">self</span>.m_faces[fi].wires {
                <span class="k">for</span> er <span class="k">in</span> <span class="k">self</span>.wire_edges(wr) {
                    <span class="k">if</span> er.index <span class="k">as</span> usize == edge_index {
                        out.push(BRepRef::new(fi <span class="k">as</span> i32, brep_compose(fo, er.orientation)));
                    }
                }
            }
        }

        out
    }

    <span class="c">/// Vertex positions, in vertex order</span>
    <span class="k">pub</span> <span class="k">fn</span> vertex_points(&amp;<span class="k">self</span>) -&gt; Vec&lt;Point&gt; {
        <span class="k">let</span> <span class="k">mut</span> pts = Vec::new();

        <span class="k">for</span> v <span class="k">in</span> &amp;<span class="k">self</span>.m_vertices {
            pts.push(v.point.clone());
        }

        pts
    }

    <span class="c">/// One closed polyline per PLANAR face: the outer wire walked in wire order with its winding untouched (lofts pair loops by it), inner wires ignored; index-aligned with face_planes</span>
    <span class="k">pub</span> <span class="k">fn</span> face_polylines(&amp;<span class="k">self</span>) -&gt; Vec&lt;Polyline&gt; {
        planar_faces(<span class="k">self</span>).<span class="s">0</span>
    }

    <span class="c">/// The plane of every face face_polylines emits: centroid origin, Newell normal flipped for a Reversed face, and the whole set flipped when a closed solid encloses negative volume so normals point outward; free faces keep the wire's sign</span>
    <span class="k">pub</span> <span class="k">fn</span> face_planes(&amp;<span class="k">self</span>) -&gt; Vec&lt;Plane&gt; {
        planar_faces(<span class="k">self</span>).<span class="s">1</span>
    }

    <span class="c">/// BRepLib::UpdateTolerances: raise every edge tolerance to the worst gap between its curve ends (3D and lifted pcurves) and its vertices, every vertex to its worst edge; returns the largest</span>
    <span class="k">pub</span> <span class="k">fn</span> update_tolerances(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; f64 {
        <span class="k">let</span> <span class="k">mut</span> worst: f64 = <span class="s">0</span>.<span class="s">0</span>;

        <span class="k">for</span> ei <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.m_edges.len() {
            <span class="k">let</span> vs_i = <span class="k">self</span>.m_edges[ei].start_vertex <span class="k">as</span> usize;
            <span class="k">let</span> ve_i = <span class="k">self</span>.m_edges[ei].end_vertex <span class="k">as</span> usize;
            <span class="k">let</span> vs = <span class="k">self</span>.m_vertices[vs_i].point.clone();
            <span class="k">let</span> ve = <span class="k">self</span>.m_vertices[ve_i].point.clone();
            <span class="k">let</span> <span class="k">mut</span> tol: f64 = <span class="k">self</span>.m_edges[ei].tolerance;

            <span class="k">if</span> <span class="k">self</span>.m_edges[ei].curve_3d_index &gt;= <span class="s">0</span> {
                <span class="k">let</span> c = &amp;<span class="k">self</span>.m_curves_3d[<span class="k">self</span>.m_edges[ei].curve_3d_index <span class="k">as</span> usize];
                tol = tol.max(c.point_at(c.domain().<span class="s">0</span>).distance(&amp;vs, None));
                tol = tol.max(c.point_at(c.domain().<span class="s">1</span>).distance(&amp;ve, None));
            }

            <span class="k">for</span> pc <span class="k">in</span> &amp;<span class="k">self</span>.m_edges[ei].pcurves {
                <span class="k">let</span> srf = &amp;<span class="k">self</span>.m_surfaces[pc.surface_index <span class="k">as</span> usize];

                <span class="k">for</span> ci <span class="k">in</span> [pc.curve_2d_index, pc.curve_2d_index_2] {
                    <span class="k">if</span> ci &lt; <span class="s">0</span> {
                        <span class="k">continue</span>;
                    }

                    <span class="k">let</span> c2 = &amp;<span class="k">self</span>.m_curves_2d[ci <span class="k">as</span> usize];
                    <span class="k">let</span> a = c2.point_at(c2.domain().<span class="s">0</span>);
                    <span class="k">let</span> z = c2.point_at(c2.domain().<span class="s">1</span>);

                    <span class="k">if</span> <span class="k">let</span> Some(p) = srf.point_at(a[<span class="s">0</span>], a[<span class="s">1</span>]) {
                        tol = tol.max(p.distance(&amp;vs, None));
                    }

                    <span class="k">if</span> <span class="k">let</span> Some(p) = srf.point_at(z[<span class="s">0</span>], z[<span class="s">1</span>]) {
                        tol = tol.max(p.distance(&amp;ve, None));
                    }
                }
            }

            <span class="k">self</span>.m_edges[ei].tolerance = tol;
            <span class="k">self</span>.m_vertices[vs_i].tolerance = <span class="k">self</span>.m_vertices[vs_i].tolerance.max(tol);
            <span class="k">self</span>.m_vertices[ve_i].tolerance = <span class="k">self</span>.m_vertices[ve_i].tolerance.max(tol);
            worst = worst.max(tol);
        }

        worst
    }

    <span class="c">/// Volume of the tessellated boundary (divergence theorem); meaningful for solids only</span>
    <span class="k">pub</span> <span class="k">fn</span> volume(&amp;<span class="k">self</span>) -&gt; f64 {
        <span class="k">self</span>.mesh().volume()
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Building</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Append a surface to the pool; returns its index.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_surface(&amp;<span class="k">mut</span> <span class="k">self</span>, srf: &amp;NurbsSurface) -&gt; usize {
        <span class="k">self</span>.m_surfaces.push(srf.clone());

        <span class="k">self</span>.m_surfaces.len() - <span class="s">1</span>
    }

    <span class="c">/// Append a 3D curve to the pool; returns its index.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_curve_3d(&amp;<span class="k">mut</span> <span class="k">self</span>, crv: &amp;NurbsCurve) -&gt; usize {
        <span class="k">self</span>.m_curves_3d.push(crv.clone());

        <span class="k">self</span>.m_curves_3d.len() - <span class="s">1</span>
    }

    <span class="c">/// Append a pcurve to the pool; returns its index.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_curve_2d(&amp;<span class="k">mut</span> <span class="k">self</span>, crv: &amp;NurbsCurve) -&gt; usize {
        <span class="k">self</span>.m_curves_2d.push(crv.clone());

        <span class="k">self</span>.m_curves_2d.len() - <span class="s">1</span>
    }

    <span class="c">/// MakeVertex</span>
    <span class="k">pub</span> <span class="k">fn</span> add_vertex(&amp;<span class="k">mut</span> <span class="k">self</span>, pt: &amp;Point, tolerance: f64) -&gt; usize {
        <span class="k">self</span>.m_vertices.push(BRepVertex {
            point: pt.clone(),
            tolerance,
        });

        <span class="k">self</span>.m_vertices.len() - <span class="s">1</span>
    }

    <span class="c">/// MakeEdge: curve_3d_index -1 makes a degenerated edge (start == end vertex); tolerance starts at 0</span>
    <span class="k">pub</span> <span class="k">fn</span> add_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, curve_3d_index: i32, start_vertex: i32, end_vertex: i32) -&gt; usize {
        <span class="k">self</span>.m_edges.push(BRepEdge {
            curve_3d_index,
            start_vertex,
            end_vertex,
            tolerance: <span class="s">0</span>.<span class="s">0</span>,
            degenerated: curve_3d_index &lt; <span class="s">0</span>,
            pcurves: Vec::new(),
        });

        <span class="k">self</span>.m_edges.len() - <span class="s">1</span>
    }

    <span class="c">/// UpdateEdge(E, pcurve, S): attach a pcurve on a surface, curve_2d_index_2 for the reversed use on a closed surface; replaces an existing record for the same surface</span>
    <span class="k">pub</span> <span class="k">fn</span> add_pcurve(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        edge_index: usize,
        surface_index: usize,
        curve_2d_index: i32,
        curve_2d_index_2: i32,
    ) {
        <span class="k">for</span> pc <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.m_edges[edge_index].pcurves {
            <span class="k">if</span> pc.surface_index == surface_index <span class="k">as</span> i32 {
                pc.curve_2d_index = curve_2d_index;
                pc.curve_2d_index_2 = curve_2d_index_2;
                <span class="k">return</span>;
            }
        }

        <span class="k">self</span>.m_edges[edge_index].pcurves.push(BRepCurveOnSurface {
            surface_index: surface_index <span class="k">as</span> i32,
            curve_2d_index,
            curve_2d_index_2,
        });
    }

    <span class="c">/// MakeWire + Add(edges)</span>
    <span class="k">pub</span> <span class="k">fn</span> add_wire(&amp;<span class="k">mut</span> <span class="k">self</span>, edges: &amp;[BRepRef]) -&gt; usize {
        <span class="k">self</span>.m_wires.push(BRepWire {
            edges: edges.to_vec(),
        });

        <span class="k">self</span>.m_wires.len() - <span class="s">1</span>
    }

    <span class="c">/// MakeFace(S) + Add(wires); the first wire is the outer boundary</span>
    <span class="k">pub</span> <span class="k">fn</span> add_face(&amp;<span class="k">mut</span> <span class="k">self</span>, surface_index: i32, wires: &amp;[BRepRef], tolerance: f64) -&gt; usize {
        <span class="k">self</span>.m_faces.push(BRepFace {
            surface_index,
            wires: wires.to_vec(),
            tolerance,
            facecolor: None,
        });

        <span class="k">self</span>.m_faces.len() - <span class="s">1</span>
    }

    <span class="c">/// MakeShell + Add(faces)</span>
    <span class="k">pub</span> <span class="k">fn</span> add_shell(&amp;<span class="k">mut</span> <span class="k">self</span>, faces: &amp;[BRepRef]) -&gt; usize {
        <span class="k">self</span>.m_shells.push(BRepShell {
            faces: faces.to_vec(),
        });

        <span class="k">self</span>.m_shells.len() - <span class="s">1</span>
    }

    <span class="c">/// MakeSolid + Add(shells)</span>
    <span class="k">pub</span> <span class="k">fn</span> add_solid(&amp;<span class="k">mut</span> <span class="k">self</span>, shells: &amp;[BRepRef]) -&gt; usize {
        <span class="k">self</span>.m_solids.push(BRepSolid {
            shells: shells.to_vec(),
        });

        <span class="k">self</span>.m_solids.len() - <span class="s">1</span>
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Meshing</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// One welded triangle mesh of every face, wound to the face's outward orientation</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh(&amp;<span class="k">self</span>) -&gt; Mesh {
        <span class="k">let</span> <span class="k">mut</span> polygons: Vec&lt;Vec&lt;Point&gt;&gt; = Vec::new();

        <span class="k">for</span> fm <span class="k">in</span> <span class="k">self</span>.face_meshes() {
            <span class="k">for</span> fk <span class="k">in</span> sorted_face_keys(&amp;fm) {
                <span class="k">let</span> <span class="k">mut</span> poly = Vec::new();

                <span class="k">for</span> vi <span class="k">in</span> &amp;fm.face[&amp;fk] {
                    poly.push(fm.vertex[vi].position());
                }

                polygons.push(poly);
            }
        }

        Mesh::from_polylines(polygons, Some(<span class="s">1</span>e-<span class="s">6</span>))
    }

    <span class="c">/// One mesh per face, in face order (vertices not shared across faces)</span>
    <span class="k">pub</span> <span class="k">fn</span> face_meshes(&amp;<span class="k">self</span>) -&gt; Vec&lt;Mesh&gt; {
        <span class="k">self</span>.face_meshes_q(None)
    }

    <span class="c">/// As face_meshes with a tessellation-quality override (max_angle_deg, chord_factor) when given</span>
    <span class="k">pub</span> <span class="k">fn</span> face_meshes_q(&amp;<span class="k">self</span>, quality: Option&lt;(f64, f64)&gt;) -&gt; Vec&lt;Mesh&gt; {
        <span class="k">let</span> nf = <span class="k">self</span>.m_faces.len();
        <span class="k">let</span> (angle, chord) = quality.unwrap_or((<span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">005</span>));
        <span class="k">let</span> <span class="k">mut</span> face_direct = vec![<span class="s">false</span>; nf];
        <span class="k">let</span> <span class="k">mut</span> rebuild_grid = vec![<span class="s">false</span>; nf];
        <span class="k">let</span> <span class="k">mut</span> fmesh: Vec&lt;Mesh&gt; = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> boundary = EdgeBoundary::default();

        <span class="k">for</span> (fi, direct) <span class="k">in</span> face_direct.iter_mut().enumerate() {
            *direct = direct_face(<span class="k">self</span>, fi);
            fmesh.push(Mesh::new());
        }

        <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..nf {
            <span class="k">if</span> !face_direct[fi] {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> srf = &amp;<span class="k">self</span>.m_surfaces[<span class="k">self</span>.m_faces[fi].surface_index <span class="k">as</span> usize];
            fmesh[fi] = <span class="k">match</span> quality {
                Some((a, c)) =&gt; RemeshNurbsSurfaceGrid::from_u_v_q(srf, <span class="s">0</span>, <span class="s">0</span>, a, c),
                None =&gt; srf.mesh(),
            };
            rebuild_grid[fi] = grid_boundaries(<span class="k">self</span>, fi, &amp;fmesh[fi], &amp;<span class="k">mut</span> boundary);
        }

        refine_shared_boundaries(
            <span class="k">self</span>,
            &amp;face_direct,
            &amp;<span class="k">mut</span> rebuild_grid,
            &amp;<span class="k">mut</span> boundary,
            angle,
            chord,
        );

        <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..nf {
            <span class="k">if</span> rebuild_grid[fi] {
                face_direct[fi] = <span class="s">false</span>;
            }
        }

        <span class="k">for</span> fi <span class="k">in</span> <span class="s">0</span>..nf {
            <span class="k">if</span> face_direct[fi] {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> srf = &amp;<span class="k">self</span>.m_surfaces[<span class="k">self</span>.m_faces[fi].surface_index <span class="k">as</span> usize];
            <span class="k">let</span> <span class="k">mut</span> loops = TrimLoops::default();

            <span class="k">if</span> rebuild_grid[fi] {
                loops.interior_uv = grid_interior_uv(srf, &amp;fmesh[fi]);
            }

            <span class="k">let</span> <span class="k">mut</span> uses: Vec&lt;(usize, usize, usize, usize)&gt; = Vec::new();

            <span class="k">if</span> !trim_loops(<span class="k">self</span>, fi, &amp;<span class="k">mut</span> boundary, angle, chord, &amp;<span class="k">mut</span> loops, &amp;<span class="k">mut</span> uses) {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> loops.interior_uv.is_empty() &amp;&amp; is_planar_patch(srf) {
                fmesh[fi] = planar_loops_mesh(srf, &amp;loops);
            } <span class="k">else</span> {
                <span class="k">let</span> <span class="k">mut</span> ts = NurbsSurfaceTrimmed::new();
                ts.m_surface = srf.clone();
                fmesh[fi] = ts.mesh_loops(&amp;loops, angle, chord);
            }

            tag_edge_uses(&amp;<span class="k">mut</span> fmesh[fi], &amp;loops, &amp;uses);
        }

        flip_reversed_faces(<span class="k">self</span>, &amp;<span class="k">mut</span> fmesh);

        fmesh
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Evaluation</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Surface point of a face at (u, v)</span>
    <span class="k">pub</span> <span class="k">fn</span> point_at(&amp;<span class="k">self</span>, face_index: usize, u: f64, v: f64) -&gt; Point {
        <span class="k">if</span> face_index &gt;= <span class="k">self</span>.m_faces.len() {
            <span class="k">return</span> Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        }

        <span class="k">self</span>.m_surfaces[<span class="k">self</span>.m_faces[face_index].surface_index <span class="k">as</span> usize]
            .point_at(u, v)
            .unwrap_or_default()
    }

    <span class="c">/// Surface normal of a face at (u, v), flipped when the face is Reversed in its shell</span>
    <span class="k">pub</span> <span class="k">fn</span> normal_at(&amp;<span class="k">self</span>, face_index: usize, u: f64, v: f64) -&gt; Vector {
        <span class="k">if</span> face_index &gt;= <span class="k">self</span>.m_faces.len() {
            <span class="k">return</span> Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        }

        <span class="k">let</span> n = <span class="k">self</span>.m_surfaces[<span class="k">self</span>.m_faces[face_index].surface_index <span class="k">as</span> usize].normal_at(u, v);

        <span class="k">if</span> <span class="k">self</span>.face_orientation(face_index) == BRepOrientation::Reversed {
            <span class="k">return</span> -n;
        }

        n
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Transformation</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Transform surfaces, 3D curves and vertices in place (pcurves are parametric, untouched)</span>
    <span class="k">pub</span> <span class="k">fn</span> transform(&amp;<span class="k">mut</span> <span class="k">self</span>, xform: &amp;Xform) {
        <span class="k">for</span> srf <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.m_surfaces {
            srf.transform(xform);
        }

        <span class="k">for</span> crv <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.m_curves_3d {
            crv.transform(xform);
        }

        <span class="k">for</span> v <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.m_vertices {
            v.point = xform.transform_point(&amp;v.point);
        }
    }

    <span class="c">/// Return a transformed copy</span>
    <span class="k">pub</span> <span class="k">fn</span> transformed(&amp;<span class="k">self</span>, xform: &amp;Xform) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> b = <span class="k">self</span>.duplicate();
        b.transform(xform);

        b
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Cutting</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return the part on the side the plane normal points to, every section loop capped by one planar face; a copy when everything lies on that side, empty when the plane cuts a BRep with a curved face or edge</span>
    <span class="k">pub</span> <span class="k">fn</span> cut_by_plane(&amp;<span class="k">self</span>, plane: &amp;Plane) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> rings = Mesh::new();

        <span class="k">if</span> !face_rings(<span class="k">self</span>, &amp;<span class="k">mut</span> rings) {
            <span class="k">let</span> tessellation = <span class="k">self</span>.mesh();

            <span class="k">return</span> <span class="k">if</span> tessellation.cut_by_plane(plane) == tessellation {
                <span class="k">self</span>.duplicate()
            } <span class="k">else</span> {
                BRep::new()
            };
        }

        <span class="k">let</span> cut = rings.cut_by_plane(plane);

        <span class="k">if</span> cut == rings {
            <span class="k">return</span> <span class="k">self</span>.duplicate();
        }

        <span class="k">if</span> cut.is_empty() {
            <span class="k">return</span> BRep::new();
        }

        <span class="k">let</span> <span class="k">mut</span> polylines: Vec&lt;Polyline&gt; = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> holes: Vec&lt;Vec&lt;Polyline&gt;&gt; = Vec::new();

        <span class="k">for</span> fk <span class="k">in</span> sorted_face_keys(&amp;cut) {
            polylines.push(ring_polyline(&amp;cut, &amp;cut.face[&amp;fk]));
            holes.push(Vec::new());

            <span class="k">if</span> <span class="k">let</span> Some(hole_rings) = cut.face_holes.get(&amp;fk) {
                <span class="k">for</span> hole <span class="k">in</span> hole_rings {
                    holes.last_mut().unwrap().push(ring_polyline(&amp;cut, hole));
                }
            }
        }

        <span class="k">let</span> <span class="k">mut</span> result = BRep::from_polylines(&amp;polylines, &amp;holes);
        result.name = <span class="k">self</span>.name.clone();
        result.width = <span class="k">self</span>.width;
        result.surfacecolor = <span class="k">self</span>.surfacecolor.clone();

        result
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// JSON</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Serialize to a sorted JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> jsondump(&amp;<span class="k">self</span>) -&gt; Result&lt;String, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">crate</span>::file_encoders::file_json_dumps(<span class="k">self</span>, <span class="s">false</span>)
    }

    <span class="c">/// Deserialize from a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> jsonload(json_data: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        Ok(serde_json::from_str(json_data)?)
    }

    <span class="c">/// Serialize to a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_dumps(&amp;<span class="k">self</span>) -&gt; String {
        <span class="k">self</span>.jsondump().expect(&quot;<span class="s">Failed to serialize BRep JSON</span>&quot;)
    }

    <span class="c">/// Deserialize from a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_loads(json_string: &amp;str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::jsonload(json_string).expect(&quot;<span class="s">Failed to parse BRep JSON</span>&quot;)
    }

    <span class="c">/// Write JSON to a file.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_dump(&amp;<span class="k">self</span>, filename: &amp;str) -&gt; Result&lt;(), Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">crate</span>::file_encoders::file_json_dump(<span class="k">self</span>, filename, <span class="s">true</span>)
    }

    <span class="c">/// Read JSON from a file.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_load(filename: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">Self</span>::jsonload(&amp;std::fs::read_to_string(filename)?)
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Protobuf</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Convert to the protobuf message.</span>
    <span class="k">pub</span> <span class="k">fn</span> to_proto(&amp;<span class="k">self</span>) -&gt; <span class="k">crate</span>::proto::BRep {
        <span class="k">let</span> <span class="k">mut</span> proto = <span class="k">crate</span>::proto::BRep {
            guid: <span class="k">self</span>.guid.get().cloned().unwrap_or_default(),
            name: <span class="k">self</span>.name.clone(),
            width: <span class="k">self</span>.width,
            ..Default::default()
        };

        <span class="k">for</span> c <span class="k">in</span> &amp;<span class="k">self</span>.m_curves_2d {
            proto.curves_2d.push(c.to_proto());
        }

        <span class="k">for</span> c <span class="k">in</span> &amp;<span class="k">self</span>.m_curves_3d {
            proto.curves_3d.push(c.to_proto());
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_surfaces {
            proto.surfaces.push(s.to_proto());
        }

        <span class="k">for</span> v <span class="k">in</span> &amp;<span class="k">self</span>.m_vertices {
            proto.vertices.push(<span class="k">crate</span>::proto::BRepVertex {
                point: Some(<span class="k">crate</span>::proto::Point {
                    x: v.point[<span class="s">0</span>],
                    y: v.point[<span class="s">1</span>],
                    z: v.point[<span class="s">2</span>],
                    ..Default::default()
                }),
                tolerance: v.tolerance,
            });
        }

        <span class="k">for</span> e <span class="k">in</span> &amp;<span class="k">self</span>.m_edges {
            proto.edges.push(edge_to_proto(e));
        }

        <span class="k">for</span> w <span class="k">in</span> &amp;<span class="k">self</span>.m_wires {
            proto.wires.push(<span class="k">crate</span>::proto::BRepWire {
                edges: refs_to_proto(&amp;w.edges),
            });
        }

        <span class="k">for</span> f <span class="k">in</span> &amp;<span class="k">self</span>.m_faces {
            proto.faces.push(face_to_proto(f));
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_shells {
            proto.shells.push(<span class="k">crate</span>::proto::BRepShell {
                faces: refs_to_proto(&amp;s.faces),
            });
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_solids {
            proto.solids.push(<span class="k">crate</span>::proto::BRepSolid {
                shells: refs_to_proto(&amp;s.shells),
            });
        }

        proto.surfacecolor = Some(<span class="k">self</span>.surfacecolor.to_proto());

        proto
    }

    <span class="c">/// Construct from the protobuf message.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_proto(proto: <span class="k">crate</span>::proto::BRep) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();

        <span class="k">if</span> !proto.guid.is_empty() {
            b.set_guid(proto.guid.clone());
        }

        b.name = proto.name;
        b.width = proto.width;

        <span class="k">for</span> c <span class="k">in</span> proto.curves_2d {
            b.m_curves_2d.push(NurbsCurve::from_proto(c));
        }

        <span class="k">for</span> c <span class="k">in</span> proto.curves_3d {
            b.m_curves_3d.push(NurbsCurve::from_proto(c));
        }

        <span class="k">for</span> s <span class="k">in</span> proto.surfaces {
            b.m_surfaces.push(NurbsSurface::from_proto(s)?);
        }

        <span class="k">for</span> v <span class="k">in</span> &amp;proto.vertices {
            <span class="k">let</span> point = <span class="k">match</span> &amp;v.point {
                Some(p) =&gt; Point::new(p.x, p.y, p.z),
                None =&gt; Point::default(),
            };
            b.m_vertices.push(BRepVertex {
                point,
                tolerance: v.tolerance,
            });
        }

        <span class="k">for</span> e <span class="k">in</span> &amp;proto.edges {
            b.m_edges.push(edge_from_proto(e));
        }

        <span class="k">for</span> w <span class="k">in</span> &amp;proto.wires {
            b.m_wires.push(BRepWire {
                edges: refs_from_proto(&amp;w.edges),
            });
        }

        <span class="k">for</span> f <span class="k">in</span> &amp;proto.faces {
            b.m_faces.push(face_from_proto(f));
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;proto.shells {
            b.m_shells.push(BRepShell {
                faces: refs_from_proto(&amp;s.faces),
            });
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;proto.solids {
            b.m_solids.push(BRepSolid {
                shells: refs_from_proto(&amp;s.shells),
            });
        }

        <span class="k">if</span> <span class="k">let</span> Some(c) = proto.surfacecolor {
            b.surfacecolor = Color::from_proto(c);
        }

        Ok(b)
    }

    <span class="c">/// Serialize to protobuf bytes.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_dumps(&amp;<span class="k">self</span>) -&gt; Vec&lt;u8&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">self</span>.to_proto().encode_to_vec()
    }

    <span class="c">/// Deserialize from protobuf bytes.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_loads(data: &amp;[u8]) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">Self</span>::from_proto(<span class="k">crate</span>::proto::BRep::decode(data)?)
    }

    <span class="c">/// Write to a protobuf file.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_dump(&amp;<span class="k">self</span>, filename: &amp;str) -&gt; Result&lt;(), Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        std::fs::write(filename, <span class="k">self</span>.pb_dumps())?;

        Ok(())
    }

    <span class="c">/// Read from a protobuf file.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_load(filename: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">Self</span>::pb_loads(&amp;std::fs::read(filename)?)
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// String</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return &quot;BRep(name=..., faces=..., edges=..., vertices=...)&quot;.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">BRep(name=</span>{}<span class="s">, faces=</span>{}<span class="s">, edges=</span>{}<span class="s">, vertices=</span>{}<span class="s">)</span>&quot;,
            <span class="k">self</span>.name,
            <span class="k">self</span>.face_count(),
            <span class="k">self</span>.edge_count(),
            <span class="k">self</span>.vertex_count()
        )
    }

    <span class="c">/// Return the multi-line form with the solid flag.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">BRep(\\n  name=</span>{}<span class="s">,\\n  faces=</span>{}<span class="s">,\\n  edges=</span>{}<span class="s">,\\n  vertices=</span>{}<span class="s">,\\n  solid=</span>{}<span class="s">\\n)</span>&quot;,
            <span class="k">self</span>.name,
            <span class="k">self</span>.face_count(),
            <span class="k">self</span>.edge_count(),
            <span class="k">self</span>.vertex_count(),
            <span class="k">if</span> <span class="k">self</span>.is_solid() { &quot;<span class="s">true</span>&quot; } <span class="k">else</span> { &quot;<span class="s">false</span>&quot; }
        )
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Operators</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="k">impl</span> PartialEq <span class="k">for</span> BRep {
    <span class="c">/// Compare name, width, color and table sizes; guid ignored.</span>
    <span class="k">fn</span> eq(&amp;<span class="k">self</span>, other: &amp;<span class="k">Self</span>) -&gt; bool {
        <span class="k">self</span>.name == other.name
            &amp;&amp; <span class="k">self</span>.width == other.width
            &amp;&amp; <span class="k">self</span>.surfacecolor == other.surfacecolor
            &amp;&amp; <span class="k">self</span>.m_surfaces.len() == other.m_surfaces.len()
            &amp;&amp; <span class="k">self</span>.m_vertices.len() == other.m_vertices.len()
            &amp;&amp; <span class="k">self</span>.m_edges.len() == other.m_edges.len()
            &amp;&amp; <span class="k">self</span>.m_wires.len() == other.m_wires.len()
            &amp;&amp; <span class="k">self</span>.m_faces.len() == other.m_faces.len()
            &amp;&amp; <span class="k">self</span>.m_shells.len() == other.m_shells.len()
            &amp;&amp; <span class="k">self</span>.m_solids.len() == other.m_solids.len()
    }
}

<span class="k">impl</span> std::fmt::Display <span class="k">for</span> BRep {
    <span class="c">/// Write the str() form.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> std::fmt::Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Serde</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="k">impl</span> Serialize <span class="k">for</span> BRep {
    <span class="k">fn</span> serialize&lt;S&gt;(&amp;<span class="k">self</span>, serializer: S) -&gt; Result&lt;S::Ok, S::Error&gt;
    <span class="k">where</span>
        S: Serializer,
    {
        <span class="k">let</span> <span class="k">mut</span> map = serializer.serialize_map(None)?;
        map.serialize_entry(&quot;<span class="s">curves_2d</span>&quot;, &amp;<span class="k">self</span>.m_curves_2d)?;
        map.serialize_entry(&quot;<span class="s">curves_3d</span>&quot;, &amp;<span class="k">self</span>.m_curves_3d)?;
        <span class="k">let</span> <span class="k">mut</span> edges: Vec&lt;EdgeJson&gt; = Vec::new();

        <span class="k">for</span> e <span class="k">in</span> &amp;<span class="k">self</span>.m_edges {
            <span class="k">let</span> <span class="k">mut</span> pcurves = Vec::new();

            <span class="k">for</span> pc <span class="k">in</span> &amp;e.pcurves {
                pcurves.push(PCurveJson {
                    curve_2d_index: pc.curve_2d_index,
                    curve_2d_index_2: pc.curve_2d_index_2,
                    surface_index: pc.surface_index,
                });
            }

            edges.push(EdgeJson {
                curve_3d_index: e.curve_3d_index,
                degenerated: e.degenerated,
                end_vertex: e.end_vertex,
                pcurves,
                start_vertex: e.start_vertex,
                tolerance: e.tolerance,
            });
        }

        map.serialize_entry(&quot;<span class="s">edges</span>&quot;, &amp;edges)?;
        <span class="k">let</span> <span class="k">mut</span> faces: Vec&lt;FaceJson&gt; = Vec::new();

        <span class="k">for</span> f <span class="k">in</span> &amp;<span class="k">self</span>.m_faces {
            faces.push(FaceJson {
                facecolor: f.facecolor.clone(),
                surface_index: f.surface_index,
                tolerance: f.tolerance,
                wires: refs_to_json(&amp;f.wires),
            });
        }

        map.serialize_entry(&quot;<span class="s">faces</span>&quot;, &amp;faces)?;
        map.serialize_entry(&quot;<span class="s">guid</span>&quot;, &amp;<span class="k">self</span>.guid())?;
        map.serialize_entry(&quot;<span class="s">name</span>&quot;, &amp;<span class="k">self</span>.name)?;
        <span class="k">let</span> <span class="k">mut</span> shells: Vec&lt;ShellJson&gt; = Vec::new();

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_shells {
            shells.push(ShellJson {
                faces: refs_to_json(&amp;s.faces),
            });
        }

        map.serialize_entry(&quot;<span class="s">shells</span>&quot;, &amp;shells)?;
        <span class="k">let</span> <span class="k">mut</span> solids: Vec&lt;SolidJson&gt; = Vec::new();

        <span class="k">for</span> s <span class="k">in</span> &amp;<span class="k">self</span>.m_solids {
            solids.push(SolidJson {
                shells: refs_to_json(&amp;s.shells),
            });
        }

        map.serialize_entry(&quot;<span class="s">solids</span>&quot;, &amp;solids)?;
        map.serialize_entry(&quot;<span class="s">surfacecolor</span>&quot;, &amp;<span class="k">self</span>.surfacecolor)?;
        map.serialize_entry(&quot;<span class="s">surfaces</span>&quot;, &amp;<span class="k">self</span>.m_surfaces)?;
        map.serialize_entry(&quot;<span class="s">type</span>&quot;, &quot;<span class="s">BRep</span>&quot;)?;
        <span class="k">let</span> <span class="k">mut</span> vertices: Vec&lt;VertexJson&gt; = Vec::new();

        <span class="k">for</span> v <span class="k">in</span> &amp;<span class="k">self</span>.m_vertices {
            vertices.push(VertexJson {
                point: [v.point[<span class="s">0</span>], v.point[<span class="s">1</span>], v.point[<span class="s">2</span>]],
                tolerance: v.tolerance,
            });
        }

        map.serialize_entry(&quot;<span class="s">vertices</span>&quot;, &amp;vertices)?;
        map.serialize_entry(&quot;<span class="s">width</span>&quot;, &amp;<span class="k">self</span>.width)?;
        <span class="k">let</span> <span class="k">mut</span> wires: Vec&lt;WireJson&gt; = Vec::new();

        <span class="k">for</span> w <span class="k">in</span> &amp;<span class="k">self</span>.m_wires {
            wires.push(WireJson {
                edges: refs_to_json(&amp;w.edges),
            });
        }

        map.serialize_entry(&quot;<span class="s">wires</span>&quot;, &amp;wires)?;

        map.end()
    }
}

<span class="k">impl</span>&lt;'de&gt; Deserialize&lt;'de&gt; <span class="k">for</span> BRep {
    <span class="k">fn</span> deserialize&lt;D&gt;(deserializer: D) -&gt; Result&lt;<span class="k">Self</span>, D::Error&gt;
    <span class="k">where</span>
        D: Deserializer&lt;'de&gt;,
    {
        #[derive(Deserialize)]
        <span class="k">struct</span> BRepData {
            #[serde(default)]
            guid: Option&lt;String&gt;,
            #[serde(default)]
            name: Option&lt;String&gt;,
            #[serde(default)]
            width: Option&lt;f64&gt;,
            #[serde(default)]
            surfacecolor: Option&lt;Color&gt;,
            #[serde(default)]
            curves_2d: Vec&lt;NurbsCurve&gt;,
            #[serde(default)]
            curves_3d: Vec&lt;NurbsCurve&gt;,
            #[serde(default)]
            surfaces: Vec&lt;NurbsSurface&gt;,
            #[serde(default)]
            vertices: Vec&lt;VertexJson&gt;,
            #[serde(default)]
            edges: Vec&lt;EdgeJson&gt;,
            #[serde(default)]
            wires: Vec&lt;WireJson&gt;,
            #[serde(default)]
            faces: Vec&lt;FaceJson&gt;,
            #[serde(default)]
            shells: Vec&lt;ShellJson&gt;,
            #[serde(default)]
            solids: Vec&lt;SolidJson&gt;,
        }

        <span class="k">let</span> data = BRepData::deserialize(deserializer)?;
        <span class="k">let</span> <span class="k">mut</span> b = BRep::new();

        <span class="k">if</span> <span class="k">let</span> Some(g) = data.guid {
            b.set_guid(g);
        }

        <span class="k">if</span> <span class="k">let</span> Some(n) = data.name {
            b.name = n;
        }

        <span class="k">if</span> <span class="k">let</span> Some(w) = data.width {
            b.width = w;
        }

        <span class="k">if</span> <span class="k">let</span> Some(c) = data.surfacecolor {
            b.surfacecolor = c;
        }

        b.m_curves_2d = data.curves_2d;
        b.m_curves_3d = data.curves_3d;
        b.m_surfaces = data.surfaces;

        <span class="k">for</span> v <span class="k">in</span> &amp;data.vertices {
            b.m_vertices.push(BRepVertex {
                point: Point::new(v.point[<span class="s">0</span>], v.point[<span class="s">1</span>], v.point[<span class="s">2</span>]),
                tolerance: v.tolerance,
            });
        }

        <span class="k">for</span> e <span class="k">in</span> &amp;data.edges {
            <span class="k">let</span> <span class="k">mut</span> be = BRepEdge {
                curve_3d_index: e.curve_3d_index,
                start_vertex: e.start_vertex,
                end_vertex: e.end_vertex,
                tolerance: e.tolerance,
                degenerated: e.degenerated,
                pcurves: Vec::new(),
            };

            <span class="k">for</span> pc <span class="k">in</span> &amp;e.pcurves {
                be.pcurves.push(BRepCurveOnSurface {
                    surface_index: pc.surface_index,
                    curve_2d_index: pc.curve_2d_index,
                    curve_2d_index_2: pc.curve_2d_index_2,
                });
            }

            b.m_edges.push(be);
        }

        <span class="k">for</span> w <span class="k">in</span> &amp;data.wires {
            b.m_wires.push(BRepWire {
                edges: refs_from_json(&amp;w.edges),
            });
        }

        <span class="k">for</span> f <span class="k">in</span> data.faces {
            b.m_faces.push(BRepFace {
                surface_index: f.surface_index,
                wires: refs_from_json(&amp;f.wires),
                tolerance: f.tolerance,
                facecolor: f.facecolor,
            });
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;data.shells {
            b.m_shells.push(BRepShell {
                faces: refs_from_json(&amp;s.faces),
            });
        }

        <span class="k">for</span> s <span class="k">in</span> &amp;data.solids {
            b.m_solids.push(BRepSolid {
                shells: refs_from_json(&amp;s.shells),
            });
        }

        Ok(b)
    }
}</code></pre></div>
`,toc:[]};export{s as default};
