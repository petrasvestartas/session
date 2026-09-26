const s={title:"session_rust/src/nurbssurface_trimmed.rs",html:`<h1 id="session_rustsrcnurbssurface_trimmedrs">session_rust/src/nurbssurface_trimmed.rs<a class="anchor" href="#/course/kernel/nurbssurface_trimmed#session_rustsrcnurbssurface_trimmedrs" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#![allow(
    clippy::manual_range_contains,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
<span class="k">use</span> <span class="k">crate</span>::closest::Closest;
<span class="k">use</span> <span class="k">crate</span>::color::Color;
<span class="k">use</span> <span class="k">crate</span>::mesh::Mesh;
<span class="k">use</span> <span class="k">crate</span>::nurbscurve::NurbsCurve;
<span class="k">use</span> <span class="k">crate</span>::nurbssurface::NurbsSurface;
<span class="k">use</span> <span class="k">crate</span>::point::Point;
<span class="k">use</span> <span class="k">crate</span>::primitives::Primitives;
<span class="k">use</span> <span class="k">crate</span>::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
<span class="k">use</span> <span class="k">crate</span>::tolerance::PI;
<span class="k">use</span> <span class="k">crate</span>::vector::Vector;
<span class="k">use</span> <span class="k">crate</span>::xform::Xform;
<span class="k">use</span> serde::Deserialize;
<span class="k">use</span> serde::Serialize;
<span class="k">use</span> std::collections::BTreeMap;
<span class="k">use</span> std::collections::HashMap;
<span class="k">use</span> std::collections::HashSet;

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Normal on the side of a C0 knot line that belongs to the triangle around center.</span>
<span class="k">fn</span> crease_side_normal(
    surface: &amp;NurbsSurface,
    knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    center: [f64; <span class="s">2</span>],
    <span class="k">mut</span> uv: [f64; <span class="s">2</span>],
) -&gt; Vector {
    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">if</span> knots[dir].contains(&amp;uv[dir]) {
            <span class="k">if</span> center[dir] &lt; uv[dir] {
                uv[dir] = uv[dir].next_down();
            }

            <span class="k">if</span> center[dir] &gt; uv[dir] {
                uv[dir] = uv[dir].next_up();
            }
        }
    }

    surface.normal_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>])
}

<span class="c">/// Winding-number test of (u, v) against a closed UV polygon.</span>
<span class="k">fn</span> point_in_polygon_2d(u: f64, v: f64, poly: &amp;[Point]) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> winding = <span class="s">0</span>;
    <span class="k">let</span> n = poly.len();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> j = (i + <span class="s">1</span>) % n;
        <span class="k">let</span> x0 = poly[i][<span class="s">0</span>];
        <span class="k">let</span> y0 = poly[i][<span class="s">1</span>];
        <span class="k">let</span> x1 = poly[j][<span class="s">0</span>];
        <span class="k">let</span> y1 = poly[j][<span class="s">1</span>];
        <span class="k">let</span> cross = (x1 - x0) * (v - y0) - (y1 - y0) * (u - x0);

        <span class="k">if</span> y0 &lt;= v &amp;&amp; y1 &gt; v &amp;&amp; cross &gt; <span class="s">0</span>.<span class="s">0</span> {
            winding += <span class="s">1</span>;
        }

        <span class="k">if</span> y0 &gt; v &amp;&amp; y1 &lt;= v &amp;&amp; cross &lt; <span class="s">0</span>.<span class="s">0</span> {
            winding -= <span class="s">1</span>;
        }
    }

    winding != <span class="s">0</span>
}

<span class="c">/// True when (u, v) lies inside the outer loop and outside every hole; bounds is the outer loop's UV box.</span>
<span class="k">fn</span> inside_loops(u: f64, v: f64, loops_uv: &amp;[Vec&lt;Point&gt;], bounds: &amp;[f64; <span class="s">4</span>]) -&gt; bool {
    <span class="k">if</span> u &lt; bounds[<span class="s">0</span>] || v &lt; bounds[<span class="s">1</span>] || u &gt; bounds[<span class="s">2</span>] || v &gt; bounds[<span class="s">3</span>] {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">if</span> !point_in_polygon_2d(u, v, &amp;loops_uv[<span class="s">0</span>]) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">for</span> li <span class="k">in</span> <span class="s">1</span>..loops_uv.len() {
        <span class="k">if</span> point_in_polygon_2d(u, v, &amp;loops_uv[li]) {
            <span class="k">return</span> <span class="s">false</span>;
        }
    }

    <span class="s">true</span>
}

<span class="c">/// Surface point at (u, v) as a plain array.</span>
<span class="k">fn</span> eval3(srf: &amp;NurbsSurface, u: f64, v: f64) -&gt; [f64; <span class="s">3</span>] {
    <span class="k">let</span> p = srf.point_at(u, v).unwrap_or_default();

    [p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]
}

<span class="c">/// Signed distance of the surface point at (u, v) to the plane (q, n).</span>
<span class="k">fn</span> plane_field(srf: &amp;NurbsSurface, q: &amp;[f64; <span class="s">3</span>], n: &amp;[f64; <span class="s">3</span>], u: f64, v: f64) -&gt; f64 {
    <span class="k">let</span> p = eval3(srf, u, v);

    (p[<span class="s">0</span>] - q[<span class="s">0</span>]) * n[<span class="s">0</span>] + (p[<span class="s">1</span>] - q[<span class="s">1</span>]) * n[<span class="s">1</span>] + (p[<span class="s">2</span>] - q[<span class="s">2</span>]) * n[<span class="s">2</span>]
}

<span class="c">/// Newton steps of (u, v) onto the plane (q, n) along the field gradient.</span>
<span class="k">fn</span> refine_crossing(
    srf: &amp;NurbsSurface,
    q: &amp;[f64; <span class="s">3</span>],
    n: &amp;[f64; <span class="s">3</span>],
    <span class="k">mut</span> u: f64,
    <span class="k">mut</span> v: f64,
) -&gt; (f64, f64) {
    <span class="k">for</span> _it <span class="k">in</span> <span class="s">0</span>..<span class="s">12</span> {
        <span class="k">let</span> fv = plane_field(srf, q, n, u, v);

        <span class="k">if</span> fv.abs() &lt; <span class="s">1</span>e-<span class="s">9</span> {
            <span class="k">break</span>;
        }

        <span class="k">let</span> h = <span class="s">1</span>e-<span class="s">4</span>;
        <span class="k">let</span> a = eval3(srf, u + h, v);
        <span class="k">let</span> b = eval3(srf, u - h, v);
        <span class="k">let</span> c = eval3(srf, u, v + h);
        <span class="k">let</span> d = eval3(srf, u, v - h);
        <span class="k">let</span> gu = ((a[<span class="s">0</span>] - b[<span class="s">0</span>]) * n[<span class="s">0</span>] + (a[<span class="s">1</span>] - b[<span class="s">1</span>]) * n[<span class="s">1</span>] + (a[<span class="s">2</span>] - b[<span class="s">2</span>]) * n[<span class="s">2</span>]) / (<span class="s">2</span>.<span class="s">0</span> * h);
        <span class="k">let</span> gv = ((c[<span class="s">0</span>] - d[<span class="s">0</span>]) * n[<span class="s">0</span>] + (c[<span class="s">1</span>] - d[<span class="s">1</span>]) * n[<span class="s">1</span>] + (c[<span class="s">2</span>] - d[<span class="s">2</span>]) * n[<span class="s">2</span>]) / (<span class="s">2</span>.<span class="s">0</span> * h);
        <span class="k">let</span> g2 = gu * gu + gv * gv;

        <span class="k">if</span> g2 &lt; <span class="s">1</span>e-<span class="s">20</span> {
            <span class="k">break</span>;
        }

        u -= fv * gu / g2;
        v -= fv * gv / g2;
    }

    (u, v)
}

<span class="c">/// Normal turn in degrees along dir over [t0, t1] on the line smid of the other direction, summed over four steps.</span>
<span class="k">fn</span> span_turn(srf: &amp;NurbsSurface, dir: usize, t0: f64, t1: f64, smid: f64) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> ma = <span class="s">0</span>.<span class="s">0_f64</span>;
    <span class="k">let</span> <span class="k">mut</span> pn = Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..=<span class="s">4</span> {
        <span class="k">let</span> t = t0 + k <span class="k">as</span> f64 * (t1 - t0) / <span class="s">4</span>.<span class="s">0</span>;
        <span class="k">let</span> nm = <span class="k">if</span> dir == <span class="s">0</span> {
            srf.normal_at(t, smid)
        } <span class="k">else</span> {
            srf.normal_at(smid, t)
        };

        <span class="k">if</span> k &gt; <span class="s">0</span> {
            <span class="k">let</span> d = pn.dot(&amp;nm).clamp(-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
            ma += d.acos() * <span class="s">180</span>.<span class="s">0</span> / PI;
        }

        pn = nm;
    }

    ma
}

<span class="c">/// Largest distance of the quarter points along dir over [t0, t1] on the line smid from their chord.</span>
<span class="k">fn</span> span_deviation(srf: &amp;NurbsSurface, dir: usize, t0: f64, t1: f64, smid: f64) -&gt; f64 {
    <span class="k">let</span> p0 = <span class="k">if</span> dir == <span class="s">0</span> {
        eval3(srf, t0, smid)
    } <span class="k">else</span> {
        eval3(srf, smid, t0)
    };
    <span class="k">let</span> p1 = <span class="k">if</span> dir == <span class="s">0</span> {
        eval3(srf, t1, smid)
    } <span class="k">else</span> {
        eval3(srf, smid, t1)
    };
    <span class="k">let</span> <span class="k">mut</span> dev = <span class="s">0</span>.<span class="s">0_f64</span>;

    <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..=<span class="s">3</span> {
        <span class="k">let</span> fr = k <span class="k">as</span> f64 / <span class="s">4</span>.<span class="s">0</span>;
        <span class="k">let</span> tm = t0 + fr * (t1 - t0);
        <span class="k">let</span> pm = <span class="k">if</span> dir == <span class="s">0</span> {
            eval3(srf, tm, smid)
        } <span class="k">else</span> {
            eval3(srf, smid, tm)
        };
        <span class="k">let</span> lx = p0[<span class="s">0</span>] + fr * (p1[<span class="s">0</span>] - p0[<span class="s">0</span>]);
        <span class="k">let</span> ly = p0[<span class="s">1</span>] + fr * (p1[<span class="s">1</span>] - p0[<span class="s">1</span>]);
        <span class="k">let</span> lz = p0[<span class="s">2</span>] + fr * (p1[<span class="s">2</span>] - p0[<span class="s">2</span>]);
        <span class="k">let</span> dd = ((pm[<span class="s">0</span>] - lx) * (pm[<span class="s">0</span>] - lx)
            + (pm[<span class="s">1</span>] - ly) * (pm[<span class="s">1</span>] - ly)
            + (pm[<span class="s">2</span>] - lz) * (pm[<span class="s">2</span>] - lz))
            .sqrt();

        <span class="k">if</span> dd &gt; dev {
            dev = dd;
        }
    }

    dev
}

<span class="c">/// Subdivisions per span along dir from the normal turn (max_angle_deg) and the chord deviation (chord_tol) at the mid line of the other direction.</span>
<span class="k">fn</span> span_subdivisions(
    srf: &amp;NurbsSurface,
    dir: usize,
    sp: &amp;[f64],
    osp: &amp;[f64],
    deg: usize,
    max_angle_deg: f64,
    chord_tol: f64,
) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> n = sp.len() - <span class="s">1</span>;
    <span class="k">let</span> <span class="k">mut</span> subs = vec![<span class="k">if</span> deg &gt; <span class="s">1</span> { <span class="s">2usize</span> } <span class="k">else</span> { <span class="s">1</span> }; n];
    <span class="k">let</span> smid = (osp[<span class="s">0</span>] + osp[osp.len() - <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> t0 = sp[i];
        <span class="k">let</span> t1 = sp[i + <span class="s">1</span>];

        <span class="k">if</span> deg &gt; <span class="s">1</span> {
            <span class="k">let</span> ma = span_turn(srf, dir, t0, t1, smid);
            subs[i] = subs[i].max(<span class="s">1</span>.max(((ma / max_angle_deg).ceil() <span class="k">as</span> usize).min(<span class="s">64</span>)));
        }

        <span class="k">let</span> dev = span_deviation(srf, dir, t0, t1, smid);

        <span class="k">if</span> dev &gt; chord_tol {
            subs[i] = subs[i].max(((dev / chord_tol).sqrt().ceil() <span class="k">as</span> usize).min(<span class="s">64</span>));
        }
    }

    subs
}

<span class="c">/// Grid parameters: each span of sp cut into subs[i] equal steps, ending on the last knot.</span>
<span class="k">fn</span> span_parameters(sp: &amp;[f64], subs: &amp;[usize]) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..sp.len() - <span class="s">1</span> {
        <span class="k">for</span> st <span class="k">in</span> <span class="s">0</span>..subs[i] {
            out.push(sp[i] + st <span class="k">as</span> f64 * (sp[i + <span class="s">1</span>] - sp[i]) / subs[i] <span class="k">as</span> f64);
        }
    }

    out.push(sp[sp.len() - <span class="s">1</span>]);

    out
}

<span class="c">/// Span-adaptive grid parameters in u and v; None when the surface has no span in a direction.</span>
<span class="k">fn</span> span_grid(
    srf: &amp;NurbsSurface,
    max_angle_deg: f64,
    chord_tol: f64,
) -&gt; Option&lt;(Vec&lt;f64&gt;, Vec&lt;f64&gt;)&gt; {
    <span class="k">let</span> usp = srf.get_span_vector(<span class="s">0</span>);
    <span class="k">let</span> vsp = srf.get_span_vector(<span class="s">1</span>);

    <span class="k">if</span> usp.len() &lt; <span class="s">2</span> || vsp.len() &lt; <span class="s">2</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> us = span_parameters(
        &amp;usp,
        &amp;span_subdivisions(srf, <span class="s">0</span>, &amp;usp, &amp;vsp, srf.degree(<span class="s">0</span>), max_angle_deg, chord_tol),
    );
    <span class="k">let</span> vs = span_parameters(
        &amp;vsp,
        &amp;span_subdivisions(srf, <span class="s">1</span>, &amp;vsp, &amp;usp, srf.degree(<span class="s">1</span>), max_angle_deg, chord_tol),
    );

    <span class="k">if</span> us.len() &lt; <span class="s">2</span> || vs.len() &lt; <span class="s">2</span> {
        <span class="k">return</span> None;
    }

    Some((us, vs))
}

<span class="c">/// Unit normal as a plain array, or none when degenerate.</span>
<span class="k">fn</span> unit3(n: &amp;Vector) -&gt; Option&lt;[f64; 3]&gt; {
    <span class="k">let</span> nl = n.magnitude_squared().sqrt();

    <span class="k">if</span> nl &lt; <span class="s">1</span>e-<span class="s">12</span> {
        <span class="k">return</span> None;
    }

    Some([n[<span class="s">0</span>] / nl, n[<span class="s">1</span>] / nl, n[<span class="s">2</span>] / nl])
}

<span class="c">/// Parameters of the 2D segment crossing p1p2 x p3p4, or false when parallel or outside.</span>
<span class="k">fn</span> segment_intersection(
    p1: &amp;[f64; <span class="s">2</span>],
    p2: &amp;[f64; <span class="s">2</span>],
    p3: &amp;[f64; <span class="s">2</span>],
    p4: &amp;[f64; <span class="s">2</span>],
) -&gt; Option&lt;(f64, f64)&gt; {
    <span class="k">let</span> d1u = p2[<span class="s">0</span>] - p1[<span class="s">0</span>];
    <span class="k">let</span> d1v = p2[<span class="s">1</span>] - p1[<span class="s">1</span>];
    <span class="k">let</span> d2u = p4[<span class="s">0</span>] - p3[<span class="s">0</span>];
    <span class="k">let</span> d2v = p4[<span class="s">1</span>] - p3[<span class="s">1</span>];
    <span class="k">let</span> den = d1u * d2v - d1v * d2u;

    <span class="k">if</span> den.abs() &lt; <span class="s">1</span>e-<span class="s">20</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> s = ((p3[<span class="s">0</span>] - p1[<span class="s">0</span>]) * d2v - (p3[<span class="s">1</span>] - p1[<span class="s">1</span>]) * d2u) / den;
    <span class="k">let</span> t = ((p3[<span class="s">0</span>] - p1[<span class="s">0</span>]) * d1v - (p3[<span class="s">1</span>] - p1[<span class="s">1</span>]) * d1u) / den;

    <span class="k">if</span> s &lt; -<span class="s">1</span>e-<span class="s">12</span> || s &gt; <span class="s">1</span>.<span class="s">0</span> + <span class="s">1</span>e-<span class="s">12</span> || t &lt; -<span class="s">1</span>e-<span class="s">12</span> || t &gt; <span class="s">1</span>.<span class="s">0</span> + <span class="s">1</span>e-<span class="s">12</span> {
        <span class="k">return</span> None;
    }

    Some((s, t))
}

<span class="c">/// Newton refinement of a UV curve-curve crossing (ta, tb), clamped to the domains.</span>
<span class="k">fn</span> newton_curve_curve(
    ca: &amp;NurbsCurve,
    <span class="k">mut</span> ta: f64,
    cb: &amp;NurbsCurve,
    <span class="k">mut</span> tb: f64,
    tol: f64,
) -&gt; (f64, f64) {
    <span class="k">for</span> _it <span class="k">in</span> <span class="s">0</span>..<span class="s">8</span> {
        <span class="k">let</span> da = ca.evaluate(ta, <span class="s">1</span>);
        <span class="k">let</span> db = cb.evaluate(tb, <span class="s">1</span>);
        <span class="k">let</span> fu = da[<span class="s">0</span>][<span class="s">0</span>] - db[<span class="s">0</span>][<span class="s">0</span>];
        <span class="k">let</span> fv = da[<span class="s">0</span>][<span class="s">1</span>] - db[<span class="s">0</span>][<span class="s">1</span>];

        <span class="k">if</span> fu.hypot(fv) &lt; tol {
            <span class="k">break</span>;
        }

        <span class="k">let</span> j00 = da[<span class="s">1</span>][<span class="s">0</span>];
        <span class="k">let</span> j01 = -db[<span class="s">1</span>][<span class="s">0</span>];
        <span class="k">let</span> j10 = da[<span class="s">1</span>][<span class="s">1</span>];
        <span class="k">let</span> j11 = -db[<span class="s">1</span>][<span class="s">1</span>];
        <span class="k">let</span> den = j00 * j11 - j01 * j10;

        <span class="k">if</span> den.abs() &lt; <span class="s">1</span>e-<span class="s">20</span> {
            <span class="k">break</span>;
        }

        ta -= (fu * j11 - j01 * fv) / den;
        tb -= (j00 * fv - fu * j10) / den;
        <span class="k">let</span> adom = ca.domain();
        <span class="k">let</span> bdom = cb.domain();
        ta = ta.max(adom.<span class="s">0</span>).min(adom.<span class="s">1</span>);
        tb = tb.max(bdom.<span class="s">0</span>).min(bdom.<span class="s">1</span>);
    }

    (ta, tb)
}

<span class="c">/// Signed area of a closed UV loop sampled at 64 parameters.</span>
<span class="k">fn</span> loop_signed_area(loop_crv: &amp;NurbsCurve) -&gt; f64 {
    <span class="k">let</span> n = <span class="s">64</span>;
    <span class="k">let</span> (l0, l1) = loop_crv.domain();
    <span class="k">let</span> <span class="k">mut</span> s = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> prev = loop_crv.point_at(l0);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..=n {
        <span class="k">let</span> p = loop_crv.point_at(l0 + (l1 - l0) * i <span class="k">as</span> f64 / n <span class="k">as</span> f64);
        s += prev[<span class="s">0</span>] * p[<span class="s">1</span>] - p[<span class="s">0</span>] * prev[<span class="s">1</span>];
        prev = p;
    }

    s * <span class="s">0</span>.<span class="s">5</span>
}

<span class="c">/// Plane coordinates of pt in the affine frame (p00, u_axis, v_axis).</span>
<span class="k">fn</span> project_to_uv(
    pt: &amp;Point,
    p00: &amp;Point,
    u_axis: &amp;Vector,
    v_axis: &amp;Vector,
    u_len2: f64,
    v_len2: f64,
) -&gt; Point {
    <span class="k">let</span> d = pt - p00;

    Point::new(d.dot(u_axis) / u_len2, d.dot(v_axis) / v_len2, <span class="s">0</span>.<span class="s">0</span>)
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// VertexWelder</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Adds 3D points to a mesh, returning the existing vertex when one lies within tol.</span>
<span class="k">struct</span> VertexWelder {
    tol: f64,                                             <span class="c">// Weld tolerance.</span>
    cell: f64,                                            <span class="c">// Hash cell size.</span>
    cells: HashMap&lt;(i64, i64, i64), Vec&lt;(Point, usize)&gt;&gt;, <span class="c">// Vertices per cell.</span>
}

<span class="k">impl</span> VertexWelder {
    <span class="c">/// Construct with a weld tolerance and a hash cell size.</span>
    <span class="k">fn</span> new(tol: f64, cell: f64) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            tol,
            cell,
            cells: HashMap::new(),
        }
    }

    <span class="c">/// Weld a 3D point, returning the existing vertex within tol or a new one.</span>
    <span class="k">fn</span> weld(&amp;<span class="k">mut</span> <span class="k">self</span>, mesh: &amp;<span class="k">mut</span> Mesh, p: Point) -&gt; usize {
        <span class="k">let</span> ci = (p[<span class="s">0</span>] / <span class="k">self</span>.cell).floor() <span class="k">as</span> i64;
        <span class="k">let</span> cj = (p[<span class="s">1</span>] / <span class="k">self</span>.cell).floor() <span class="k">as</span> i64;
        <span class="k">let</span> ck = (p[<span class="s">2</span>] / <span class="k">self</span>.cell).floor() <span class="k">as</span> i64;

        <span class="k">for</span> di <span class="k">in</span> -<span class="s">1</span>..=<span class="s">1</span> {
            <span class="k">for</span> dj <span class="k">in</span> -<span class="s">1</span>..=<span class="s">1</span> {
                <span class="k">for</span> dk <span class="k">in</span> -<span class="s">1</span>..=<span class="s">1</span> {
                    <span class="k">let</span> Some(bucket) = <span class="k">self</span>.cells.get(&amp;(ci + di, cj + dj, ck + dk)) <span class="k">else</span> {
                        <span class="k">continue</span>;
                    };

                    <span class="k">for</span> (q, vk) <span class="k">in</span> bucket {
                        <span class="k">if</span> (q - &amp;p).magnitude_squared() &lt;= <span class="k">self</span>.tol * <span class="k">self</span>.tol {
                            <span class="k">return</span> *vk;
                        }
                    }
                }
            }
        }

        <span class="k">let</span> vk = mesh.add_vertex(p.clone(), None);
        <span class="k">self</span>.cells.entry((ci, cj, ck)).or_default().push((p, vk));

        vk
    }

    <span class="c">/// Weld the surface point at (u, v); a new vertex gets the surface normal.</span>
    <span class="k">fn</span> weld_surface(&amp;<span class="k">mut</span> <span class="k">self</span>, mesh: &amp;<span class="k">mut</span> Mesh, srf: &amp;NurbsSurface, u: f64, v: f64) -&gt; usize {
        <span class="k">let</span> before = mesh.number_of_vertices();
        <span class="k">let</span> vk = <span class="k">self</span>.weld(mesh, srf.point_at(u, v).unwrap_or_default());

        <span class="k">if</span> mesh.number_of_vertices() &gt; before {
            <span class="k">let</span> nm = srf.normal_at(u, v);

            <span class="k">if</span> <span class="k">let</span> Some(vd) = mesh.vertex.get_mut(&amp;vk) {
                vd.set_normal(nm[<span class="s">0</span>], nm[<span class="s">1</span>], nm[<span class="s">2</span>]);
            }
        }

        vk
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Plane clipping</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Welded polygon of the part of a grid cell where the field is &lt;= 0: kept corners and Newton-refined edge crossings in order.</span>
<span class="k">fn</span> clip_cell(
    welder: &amp;<span class="k">mut</span> VertexWelder,
    mesh: &amp;<span class="k">mut</span> Mesh,
    srf: &amp;NurbsSurface,
    q: &amp;[f64; <span class="s">3</span>],
    n: &amp;[f64; <span class="s">3</span>],
    cu: &amp;[f64; <span class="s">4</span>],
    cv: &amp;[f64; <span class="s">4</span>],
    fc: &amp;[f64; <span class="s">4</span>],
) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> inn = [fc[<span class="s">0</span>] &lt;= <span class="s">0</span>.<span class="s">0</span>, fc[<span class="s">1</span>] &lt;= <span class="s">0</span>.<span class="s">0</span>, fc[<span class="s">2</span>] &lt;= <span class="s">0</span>.<span class="s">0</span>, fc[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span>];
    <span class="k">let</span> <span class="k">mut</span> poly: Vec&lt;usize&gt; = Vec::new();

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
        <span class="k">let</span> kn = (k + <span class="s">1</span>) % <span class="s">4</span>;

        <span class="k">if</span> inn[k] {
            poly.push(welder.weld_surface(mesh, srf, cu[k], cv[k]));
        }

        <span class="k">if</span> inn[k] != inn[kn] {
            <span class="k">let</span> t = <span class="k">if</span> (fc[k] - fc[kn]).abs() &gt; <span class="s">1</span>e-<span class="s">30</span> {
                fc[k] / (fc[k] - fc[kn])
            } <span class="k">else</span> {
                <span class="s">0</span>.<span class="s">5</span>
            };
            <span class="k">let</span> u = cu[k] + (cu[kn] - cu[k]) * t;
            <span class="k">let</span> v = cv[k] + (cv[kn] - cv[k]) * t;
            <span class="k">let</span> (u, v) = refine_crossing(srf, q, n, u, v);
            poly.push(welder.weld_surface(mesh, srf, u, v));
        }
    }

    poly
}

<span class="c">/// Fan a welded polygon into the mesh from its first vertex, skipping triangles with a repeated vertex.</span>
<span class="k">fn</span> add_fan(mesh: &amp;<span class="k">mut</span> Mesh, poly: &amp;[usize]) {
    <span class="k">for</span> t <span class="k">in</span> <span class="s">1</span>..poly.len().saturating_sub(<span class="s">1</span>) {
        <span class="k">let</span> a = poly[<span class="s">0</span>];
        <span class="k">let</span> b = poly[t];
        <span class="k">let</span> c = poly[t + <span class="s">1</span>];

        <span class="k">if</span> a == b || b == c || c == a {
            <span class="k">continue</span>;
        }

        mesh.add_face(vec![a, b, c], None);
    }
}

<span class="c">/// Two UV triangles per cell of the grid us x vs.</span>
<span class="k">fn</span> grid_triangles(us: &amp;[f64], vs: &amp;[f64]) -&gt; Vec&lt;[[f64; 2]; 3]&gt; {
    <span class="k">let</span> <span class="k">mut</span> tris: Vec&lt;[[f64; 2]; 3]&gt; = Vec::with_capacity((us.len() - <span class="s">1</span>) * (vs.len() - <span class="s">1</span>) * <span class="s">2</span>);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..us.len() - <span class="s">1</span> {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..vs.len() - <span class="s">1</span> {
            <span class="k">let</span> a = [us[i], vs[j]];
            <span class="k">let</span> b = [us[i + <span class="s">1</span>], vs[j]];
            <span class="k">let</span> c = [us[i + <span class="s">1</span>], vs[j + <span class="s">1</span>]];
            <span class="k">let</span> d = [us[i], vs[j + <span class="s">1</span>]];
            tris.push([a, b, c]);
            tris.push([a, c, d]);
        }
    }

    tris
}

<span class="c">/// UV triangles clipped to the half (S-q).n &lt;= 1e-9, each kept part fanned from its first corner.</span>
<span class="k">fn</span> clip_triangles(
    srf: &amp;NurbsSurface,
    q: &amp;[f64; <span class="s">3</span>],
    n: &amp;[f64; <span class="s">3</span>],
    tris: &amp;[[[f64; <span class="s">2</span>]; <span class="s">3</span>]],
) -&gt; Vec&lt;[[f64; 2]; 3]&gt; {
    <span class="k">let</span> eps = <span class="s">1</span>e-<span class="s">9</span>;
    <span class="k">let</span> <span class="k">mut</span> next: Vec&lt;[[f64; 2]; 3]&gt; = Vec::new();

    <span class="k">for</span> t <span class="k">in</span> tris {
        <span class="k">let</span> <span class="k">mut</span> poly: Vec&lt;[f64; 2]&gt; = Vec::new();

        <span class="k">for</span> e <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> p = t[e];
            <span class="k">let</span> r = t[(e + <span class="s">1</span>) % <span class="s">3</span>];
            <span class="k">let</span> fp = plane_field(srf, q, n, p[<span class="s">0</span>], p[<span class="s">1</span>]);
            <span class="k">let</span> fr = plane_field(srf, q, n, r[<span class="s">0</span>], r[<span class="s">1</span>]);
            <span class="k">let</span> pin = fp &lt;= eps;
            <span class="k">let</span> rin = fr &lt;= eps;

            <span class="k">if</span> pin {
                poly.push(p);
            }

            <span class="k">if</span> pin != rin {
                <span class="k">let</span> tt = <span class="k">if</span> (fp - fr).abs() &gt; <span class="s">1</span>e-<span class="s">30</span> {
                    fp / (fp - fr)
                } <span class="k">else</span> {
                    <span class="s">0</span>.<span class="s">5</span>
                };
                <span class="k">let</span> cu = p[<span class="s">0</span>] + (r[<span class="s">0</span>] - p[<span class="s">0</span>]) * tt;
                <span class="k">let</span> cv = p[<span class="s">1</span>] + (r[<span class="s">1</span>] - p[<span class="s">1</span>]) * tt;
                <span class="k">let</span> (cu, cv) = refine_crossing(srf, q, n, cu, cv);
                poly.push([cu, cv]);
            }
        }

        <span class="k">for</span> w <span class="k">in</span> <span class="s">1</span>..poly.len().saturating_sub(<span class="s">1</span>) {
            next.push([poly[<span class="s">0</span>], poly[w], poly[w + <span class="s">1</span>]]);
        }
    }

    next
}

<span class="c">/// Mesh of UV triangles lifted onto the surface, seams welded within weld_tol, degenerate faces skipped.</span>
<span class="k">fn</span> weld_triangles(srf: &amp;NurbsSurface, tris: &amp;[[[f64; <span class="s">2</span>]; <span class="s">3</span>]], weld_tol: f64) -&gt; Mesh {
    <span class="k">let</span> <span class="k">mut</span> result = Mesh::new();
    <span class="k">let</span> <span class="k">mut</span> welder = VertexWelder::new(weld_tol, weld_tol);

    <span class="k">for</span> t <span class="k">in</span> tris {
        <span class="k">let</span> a = welder.weld_surface(&amp;<span class="k">mut</span> result, srf, t[<span class="s">0</span>][<span class="s">0</span>], t[<span class="s">0</span>][<span class="s">1</span>]);
        <span class="k">let</span> b = welder.weld_surface(&amp;<span class="k">mut</span> result, srf, t[<span class="s">1</span>][<span class="s">0</span>], t[<span class="s">1</span>][<span class="s">1</span>]);
        <span class="k">let</span> c = welder.weld_surface(&amp;<span class="k">mut</span> result, srf, t[<span class="s">2</span>][<span class="s">0</span>], t[<span class="s">2</span>][<span class="s">1</span>]);

        <span class="k">if</span> a == b || b == c || c == a {
            <span class="k">continue</span>;
        }

        result.add_face(vec![a, b, c], None);
    }

    result
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// UVGraph</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Snapped UV vertices of the split graph: points within snap of each other share one id.</span>
<span class="k">struct</span> UVVertexPool {
    snap: f64,                              <span class="c">// Snap distance.</span>
    cells: HashMap&lt;(i64, i64), Vec&lt;usize&gt;&gt;, <span class="c">// Ids per cell.</span>
    verts: Vec&lt;[f64; 2]&gt;,                   <span class="c">// UV position per id.</span>
}

<span class="k">impl</span> UVVertexPool {
    <span class="c">/// Construct with the snap distance.</span>
    <span class="k">fn</span> new(snap: f64) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            snap,
            cells: HashMap::new(),
            verts: Vec::new(),
        }
    }

    <span class="c">/// Id of the vertex within snap of p, a new one when none.</span>
    <span class="k">fn</span> id(&amp;<span class="k">mut</span> <span class="k">self</span>, p: [f64; <span class="s">2</span>]) -&gt; usize {
        <span class="k">let</span> ci = (p[<span class="s">0</span>] / <span class="k">self</span>.snap).floor() <span class="k">as</span> i64;
        <span class="k">let</span> cj = (p[<span class="s">1</span>] / <span class="k">self</span>.snap).floor() <span class="k">as</span> i64;

        <span class="k">for</span> di <span class="k">in</span> -<span class="s">1i64</span>..=<span class="s">1</span> {
            <span class="k">for</span> dj <span class="k">in</span> -<span class="s">1i64</span>..=<span class="s">1</span> {
                <span class="k">let</span> Some(bucket) = <span class="k">self</span>.cells.get(&amp;(ci + di, cj + dj)) <span class="k">else</span> {
                    <span class="k">continue</span>;
                };

                <span class="k">for</span> &amp;vk <span class="k">in</span> bucket {
                    <span class="k">let</span> q = <span class="k">self</span>.verts[vk];

                    <span class="k">if</span> (q[<span class="s">0</span>] - p[<span class="s">0</span>]).hypot(q[<span class="s">1</span>] - p[<span class="s">1</span>]) &lt;= <span class="k">self</span>.snap {
                        <span class="k">return</span> vk;
                    }
                }
            }
        }

        <span class="k">let</span> vk = <span class="k">self</span>.verts.len();
        <span class="k">self</span>.verts.push([p[<span class="s">0</span>], p[<span class="s">1</span>]]);
        <span class="k">self</span>.cells.entry((ci, cj)).or_default().push(vk);

        vk
    }
}

<span class="c">/// Graph edge between two pool vertices on pcurve cidx (negative: a domain border) over [ta, tb].</span>
#[derive(Clone, Copy)]
<span class="k">struct</span> SplitEdge {
    a: usize,  <span class="c">// First pool vertex.</span>
    b: usize,  <span class="c">// Second pool vertex.</span>
    cidx: i32, <span class="c">// Pcurve index, negative for a domain border.</span>
    ta: f64,   <span class="c">// Parameter at a.</span>
    tb: f64,   <span class="c">// Parameter at b.</span>
}

<span class="c">/// Directed copy of a split edge: fwd when it runs a -&gt; b.</span>
<span class="k">struct</span> HalfEdge {
    tail: usize, <span class="c">// Start vertex.</span>
    head: usize, <span class="c">// End vertex.</span>
    eidx: usize, <span class="c">// Split edge index.</span>
    fwd: bool,   <span class="c">// True when it runs a -&gt; b.</span>
}

<span class="c">/// UV domain of the split surface and the distance under which UV points snap together.</span>
#[derive(Clone, Copy)]
<span class="k">struct</span> SplitDomain {
    u0: f64,   <span class="c">// Start of the u domain.</span>
    u1: f64,   <span class="c">// End of the u domain.</span>
    v0: f64,   <span class="c">// Start of the v domain.</span>
    v1: f64,   <span class="c">// End of the v domain.</span>
    snap: f64, <span class="c">// Snap distance in UV.</span>
}

<span class="c">/// Sampled pcurve in UV with the parameter of each sample.</span>
<span class="k">struct</span> UVPoly {
    cidx: i32,          <span class="c">// Pcurve index, negative for a domain border.</span>
    pts: Vec&lt;[f64; 2]&gt;, <span class="c">// UV samples.</span>
    ts: Vec&lt;f64&gt;,       <span class="c">// Parameter per sample.</span>
}

<span class="c">/// Consecutive half-edges of a cycle on one pcurve.</span>
<span class="k">struct</span> Run {
    cidx: i32, <span class="c">// Pcurve index, negative for a domain border.</span>
    va: usize, <span class="c">// First vertex.</span>
    vb: usize, <span class="c">// Last vertex.</span>
    ta: f64,   <span class="c">// Parameter at va.</span>
    tb: f64,   <span class="c">// Parameter at vb.</span>
}

<span class="c">/// Domain of the surface with the snap distance: tolerance carried from 3D into UV, else 1e-7 of the shorter side.</span>
<span class="k">fn</span> split_domain(srf: &amp;NurbsSurface, tolerance: f64) -&gt; SplitDomain {
    <span class="k">let</span> (u0, u1) = srf.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> (v0, v1) = srf.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
    <span class="k">let</span> range_u = u1 - u0;
    <span class="k">let</span> range_v = v1 - v0;

    <span class="k">let</span> spans_u = srf.get_span_vector(<span class="s">0</span>);
    <span class="k">let</span> spans_v = srf.get_span_vector(<span class="s">1</span>);
    <span class="k">let</span> nu = spans_u.len().saturating_sub(<span class="s">1</span>).max(<span class="s">1</span>) * <span class="s">4</span>;
    <span class="k">let</span> nv = spans_v.len().saturating_sub(<span class="s">1</span>).max(<span class="s">1</span>) * <span class="s">4</span>;
    <span class="k">let</span> du = range_u / nu <span class="k">as</span> f64;
    <span class="k">let</span> dv = range_v / nv <span class="k">as</span> f64;
    <span class="k">let</span> mu = (u0 + u1) * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> mv = (v0 + v1) * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> pmid = srf.point_at(mu, mv).unwrap_or_default();
    <span class="k">let</span> uv_to_3d_u = pmid.distance(
        &amp;srf.point_at((mu + du).min(u1), mv).unwrap_or_default(),
        None,
    ) / du;
    <span class="k">let</span> uv_to_3d_v = pmid.distance(
        &amp;srf.point_at(mu, (mv + dv).min(v1)).unwrap_or_default(),
        None,
    ) / dv;
    <span class="k">let</span> <span class="k">mut</span> uv_to_3d = uv_to_3d_u.max(uv_to_3d_v);

    <span class="k">if</span> uv_to_3d &lt; <span class="s">1</span>e-<span class="s">10</span> {
        uv_to_3d = <span class="s">1</span>.<span class="s">0</span>;
    }

    <span class="k">let</span> snap = <span class="k">if</span> tolerance &gt; <span class="s">0</span>.<span class="s">0</span> {
        (tolerance / uv_to_3d).max(<span class="s">1</span>e-<span class="s">9</span>)
    } <span class="k">else</span> {
        range_u.min(range_v) * <span class="s">1</span>e-<span class="s">7</span>
    };

    SplitDomain {
        u0,
        u1,
        v0,
        v1,
        snap,
    }
}

<span class="c">/// Snap a UV point onto the domain border when within the snap distance of it.</span>
<span class="k">fn</span> snap_to_border(p: &amp;<span class="k">mut</span> [f64; <span class="s">2</span>], dom: &amp;SplitDomain) {
    <span class="k">if</span> (p[<span class="s">0</span>] - dom.u0).abs() &lt; dom.snap {
        p[<span class="s">0</span>] = dom.u0;
    }

    <span class="k">if</span> (p[<span class="s">0</span>] - dom.u1).abs() &lt; dom.snap {
        p[<span class="s">0</span>] = dom.u1;
    }

    <span class="k">if</span> (p[<span class="s">1</span>] - dom.v0).abs() &lt; dom.snap {
        p[<span class="s">1</span>] = dom.v0;
    }

    <span class="k">if</span> (p[<span class="s">1</span>] - dom.v1).abs() &lt; dom.snap {
        p[<span class="s">1</span>] = dom.v1;
    }
}

<span class="c">/// One pass inserting the parameter midpoint of every chord farther than samp_tol from the curve; the count inserted.</span>
<span class="k">fn</span> refine_samples(crv: &amp;NurbsCurve, entries: &amp;<span class="k">mut</span> Vec&lt;[f64; 3]&gt;, samp_tol: f64) -&gt; usize {
    <span class="k">let</span> <span class="k">mut</span> inserted = <span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> i = <span class="s">0</span>;

    <span class="k">while</span> i + <span class="s">1</span> &lt; entries.len() {
        <span class="k">let</span> a = entries[i];
        <span class="k">let</span> b = entries[i + <span class="s">1</span>];
        <span class="k">let</span> tm = (a[<span class="s">0</span>] + b[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> pm = crv.point_at(tm);
        <span class="k">let</span> exu = b[<span class="s">1</span>] - a[<span class="s">1</span>];
        <span class="k">let</span> exv = b[<span class="s">2</span>] - a[<span class="s">2</span>];
        <span class="k">let</span> l2 = exu * exu + exv * exv;
        <span class="k">let</span> <span class="k">mut</span> dev = <span class="s">0</span>.<span class="s">0</span>;

        <span class="k">if</span> l2 &gt; <span class="s">1</span>e-<span class="s">30</span> {
            <span class="k">let</span> s = ((pm[<span class="s">0</span>] - a[<span class="s">1</span>]) * exu + (pm[<span class="s">1</span>] - a[<span class="s">2</span>]) * exv) / l2;
            <span class="k">let</span> cx = a[<span class="s">1</span>] + s * exu;
            <span class="k">let</span> cy = a[<span class="s">2</span>] + s * exv;
            dev = (pm[<span class="s">0</span>] - cx).hypot(pm[<span class="s">1</span>] - cy);
        }

        <span class="k">if</span> dev &gt; samp_tol &amp;&amp; entries.len() &lt; <span class="s">4096</span> {
            entries.insert(i + <span class="s">1</span>, [tm, pm[<span class="s">0</span>], pm[<span class="s">1</span>]]);
            inserted += <span class="s">1</span>;
            i += <span class="s">2</span>;
        } <span class="k">else</span> {
            i += <span class="s">1</span>;
        }
    }

    inserted
}

<span class="c">/// Samples (t, u, v) of a pcurve: uniform in t, then up to six passes of chord refinement.</span>
<span class="k">fn</span> sample_pcurve(crv: &amp;NurbsCurve, samp_tol: f64) -&gt; Vec&lt;[f64; 3]&gt; {
    <span class="k">let</span> (ct0, ct1) = crv.domain();
    <span class="k">let</span> n = (crv.cv_count() * <span class="s">4</span>).clamp(<span class="s">16</span>, <span class="s">2048</span>);
    <span class="k">let</span> <span class="k">mut</span> entries: Vec&lt;[f64; 3]&gt; = Vec::new();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..=n {
        <span class="k">let</span> t = ct0 + (ct1 - ct0) * i <span class="k">as</span> f64 / n <span class="k">as</span> f64;
        <span class="k">let</span> p = crv.point_at(t);
        entries.push([t, p[<span class="s">0</span>], p[<span class="s">1</span>]]);
    }

    <span class="k">for</span> _depth <span class="k">in</span> <span class="s">0</span>..<span class="s">6</span> {
        <span class="k">if</span> refine_samples(crv, &amp;<span class="k">mut</span> entries, samp_tol) == <span class="s">0</span> {
            <span class="k">break</span>;
        }
    }

    entries
}

<span class="c">/// Polyline of pcurve cidx from its samples: clamped into the domain, snapped to the border, repeats dropped.</span>
<span class="k">fn</span> clamp_samples(entries: &amp;[[f64; <span class="s">3</span>]], cidx: i32, dom: &amp;SplitDomain) -&gt; UVPoly {
    <span class="k">let</span> <span class="k">mut</span> poly = UVPoly {
        cidx,
        pts: Vec::new(),
        ts: Vec::new(),
    };

    <span class="k">for</span> e <span class="k">in</span> entries {
        <span class="k">let</span> <span class="k">mut</span> p = [e[<span class="s">1</span>].max(dom.u0).min(dom.u1), e[<span class="s">2</span>].max(dom.v0).min(dom.v1)];
        snap_to_border(&amp;<span class="k">mut</span> p, dom);

        <span class="k">if</span> <span class="k">let</span> Some(last) = poly.pts.last() {
            <span class="k">if</span> (p[<span class="s">0</span>] - last[<span class="s">0</span>]).abs() &lt; <span class="s">1</span>e-<span class="s">15</span> &amp;&amp; (p[<span class="s">1</span>] - last[<span class="s">1</span>]).abs() &lt; <span class="s">1</span>e-<span class="s">15</span> {
                <span class="k">continue</span>;
            }
        }

        poly.pts.push(p);
        poly.ts.push(e[<span class="s">0</span>]);
    }

    poly
}

<span class="c">/// True when every point lies within the snap distance of one domain side.</span>
<span class="k">fn</span> on_border(pts: &amp;[[f64; <span class="s">2</span>]], dom: &amp;SplitDomain) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> on_u0 = <span class="s">true</span>;
    <span class="k">let</span> <span class="k">mut</span> on_u1 = <span class="s">true</span>;
    <span class="k">let</span> <span class="k">mut</span> on_v0 = <span class="s">true</span>;
    <span class="k">let</span> <span class="k">mut</span> on_v1 = <span class="s">true</span>;

    <span class="k">for</span> p <span class="k">in</span> pts {
        <span class="k">if</span> (p[<span class="s">0</span>] - dom.u0).abs() &gt;= dom.snap {
            on_u0 = <span class="s">false</span>;
        }

        <span class="k">if</span> (p[<span class="s">0</span>] - dom.u1).abs() &gt;= dom.snap {
            on_u1 = <span class="s">false</span>;
        }

        <span class="k">if</span> (p[<span class="s">1</span>] - dom.v0).abs() &gt;= dom.snap {
            on_v0 = <span class="s">false</span>;
        }

        <span class="k">if</span> (p[<span class="s">1</span>] - dom.v1).abs() &gt;= dom.snap {
            on_v1 = <span class="s">false</span>;
        }
    }

    on_u0 || on_u1 || on_v0 || on_v1
}

<span class="c">/// Length of a UV polyline.</span>
<span class="k">fn</span> polyline_length(pts: &amp;[[f64; <span class="s">2</span>]]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> ext = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..pts.len() {
        ext += (pts[k][<span class="s">0</span>] - pts[k - <span class="s">1</span>][<span class="s">0</span>]).hypot(pts[k][<span class="s">1</span>] - pts[k - <span class="s">1</span>][<span class="s">1</span>]);
    }

    ext
}

<span class="c">/// Polylines of the valid pcurves that neither hug the border nor fall short of min_ext, then the four domain sides.</span>
<span class="k">fn</span> uv_polylines(pcurves: &amp;[NurbsCurve], dom: &amp;SplitDomain) -&gt; Vec&lt;UVPoly&gt; {
    <span class="k">let</span> range_u = dom.u1 - dom.u0;
    <span class="k">let</span> range_v = dom.v1 - dom.v0;
    <span class="k">let</span> samp_tol = range_u.max(range_v) * <span class="s">2</span>e-<span class="s">5</span>;
    <span class="k">let</span> min_ext = (dom.snap * <span class="s">8</span>.<span class="s">0</span>).max(range_u.min(range_v) * <span class="s">1</span>e-<span class="s">5</span>);
    <span class="k">let</span> <span class="k">mut</span> polylines: Vec&lt;UVPoly&gt; = Vec::new();

    <span class="k">for</span> (cidx, crv) <span class="k">in</span> pcurves.iter().enumerate() {
        <span class="k">if</span> !crv.is_valid() {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> poly = clamp_samples(&amp;sample_pcurve(crv, samp_tol), cidx <span class="k">as</span> i32, dom);

        <span class="k">if</span> poly.pts.len() &gt;= <span class="s">2</span>
            &amp;&amp; !on_border(&amp;poly.pts, dom)
            &amp;&amp; polyline_length(&amp;poly.pts) &gt;= min_ext
        {
            polylines.push(poly);
        }
    }

    polylines.push(UVPoly {
        cidx: -<span class="s">1</span>,
        pts: vec![[dom.u0, dom.v0], [dom.u1, dom.v0]],
        ts: vec![dom.u0, dom.u1],
    });
    polylines.push(UVPoly {
        cidx: -<span class="s">2</span>,
        pts: vec![[dom.u1, dom.v0], [dom.u1, dom.v1]],
        ts: vec![dom.v0, dom.v1],
    });
    polylines.push(UVPoly {
        cidx: -<span class="s">3</span>,
        pts: vec![[dom.u1, dom.v1], [dom.u0, dom.v1]],
        ts: vec![dom.u1, dom.u0],
    });
    polylines.push(UVPoly {
        cidx: -<span class="s">4</span>,
        pts: vec![[dom.u0, dom.v1], [dom.u0, dom.v0]],
        ts: vec![dom.v1, dom.v0],
    });

    polylines
}

<span class="c">/// UV bounds (umin, umax, vmin, vmax) of a polyline.</span>
<span class="k">fn</span> uv_bounds(pts: &amp;[[f64; <span class="s">2</span>]]) -&gt; [f64; <span class="s">4</span>] {
    <span class="k">let</span> <span class="k">mut</span> bounds = [pts[<span class="s">0</span>][<span class="s">0</span>], pts[<span class="s">0</span>][<span class="s">0</span>], pts[<span class="s">0</span>][<span class="s">1</span>], pts[<span class="s">0</span>][<span class="s">1</span>]];

    <span class="k">for</span> p <span class="k">in</span> pts {
        bounds[<span class="s">0</span>] = bounds[<span class="s">0</span>].min(p[<span class="s">0</span>]);
        bounds[<span class="s">1</span>] = bounds[<span class="s">1</span>].max(p[<span class="s">0</span>]);
        bounds[<span class="s">2</span>] = bounds[<span class="s">2</span>].min(p[<span class="s">1</span>]);
        bounds[<span class="s">3</span>] = bounds[<span class="s">3</span>].max(p[<span class="s">1</span>]);
    }

    bounds
}

<span class="c">/// True when the bounds of B meet the bounds of A grown by snap.</span>
<span class="k">fn</span> boxes_overlap(a_poly: &amp;UVPoly, b_poly: &amp;UVPoly, snap: f64) -&gt; bool {
    <span class="k">let</span> a = uv_bounds(&amp;a_poly.pts);
    <span class="k">let</span> b = uv_bounds(&amp;b_poly.pts);

    !(b[<span class="s">0</span>] &gt; a[<span class="s">1</span>] + snap || b[<span class="s">1</span>] &lt; a[<span class="s">0</span>] - snap || b[<span class="s">2</span>] &gt; a[<span class="s">3</span>] + snap || b[<span class="s">3</span>] &lt; a[<span class="s">2</span>] - snap)
}

<span class="c">/// Parameter of a point along domain side cidx: u on the bottom and top sides, v on the left and right.</span>
<span class="k">fn</span> border_parameter(cidx: i32, hp: &amp;[f64; <span class="s">2</span>]) -&gt; f64 {
    <span class="k">if</span> cidx == -<span class="s">1</span> || cidx == -<span class="s">3</span> {
        hp[<span class="s">0</span>]
    } <span class="k">else</span> {
        hp[<span class="s">1</span>]
    }
}

<span class="c">/// UV point of a crossing moved onto its pcurves, Newton-refined when both are pcurves, snapped to the border; ta and tb follow it.</span>
<span class="k">fn</span> crossing_point(
    acidx: i32,
    <span class="k">mut</span> ta: f64,
    bcidx: i32,
    <span class="k">mut</span> tb: f64,
    hit: [f64; <span class="s">2</span>],
    pcurves: &amp;[NurbsCurve],
    dom: &amp;SplitDomain,
) -&gt; ([f64; <span class="s">2</span>], f64, f64) {
    <span class="k">let</span> <span class="k">mut</span> hp = hit;

    <span class="k">if</span> acidx &gt;= <span class="s">0</span> &amp;&amp; bcidx &gt;= <span class="s">0</span> {
        (ta, tb) = newton_curve_curve(
            &amp;pcurves[acidx <span class="k">as</span> usize],
            ta,
            &amp;pcurves[bcidx <span class="k">as</span> usize],
            tb,
            dom.snap * <span class="s">0</span>.<span class="s">01</span>,
        );
    }

    <span class="k">if</span> acidx &gt;= <span class="s">0</span> {
        <span class="k">let</span> pa = pcurves[acidx <span class="k">as</span> usize].point_at(ta);
        hp = [pa[<span class="s">0</span>], pa[<span class="s">1</span>]];
    } <span class="k">else</span> <span class="k">if</span> bcidx &gt;= <span class="s">0</span> {
        <span class="k">let</span> pb = pcurves[bcidx <span class="k">as</span> usize].point_at(tb);
        hp = [pb[<span class="s">0</span>], pb[<span class="s">1</span>]];
    }

    snap_to_border(&amp;<span class="k">mut</span> hp, dom);

    <span class="k">if</span> bcidx &lt; <span class="s">0</span> {
        tb = border_parameter(bcidx, &amp;hp);
    }

    <span class="k">if</span> acidx &lt; <span class="s">0</span> {
        ta = border_parameter(acidx, &amp;hp);
    }

    (hp, ta, tb)
}

<span class="c">/// Crossings of polylines pi and pj as events (fraction, u, v, parameter) on each crossed segment.</span>
<span class="k">fn</span> add_crossings(
    polylines: &amp;[UVPoly],
    pi: usize,
    pj: usize,
    pcurves: &amp;[NurbsCurve],
    dom: &amp;SplitDomain,
    splits: &amp;<span class="k">mut</span> HashMap&lt;(usize, usize), Vec&lt;(f64, f64, f64, f64)&gt;&gt;,
) {
    <span class="k">let</span> a_poly = &amp;polylines[pi];
    <span class="k">let</span> b_poly = &amp;polylines[pj];

    <span class="k">for</span> ia <span class="k">in</span> <span class="s">0</span>..a_poly.pts.len() - <span class="s">1</span> {
        <span class="k">for</span> ib <span class="k">in</span> <span class="s">0</span>..b_poly.pts.len() - <span class="s">1</span> {
            <span class="k">let</span> Some((s, t)) = segment_intersection(
                &amp;a_poly.pts[ia],
                &amp;a_poly.pts[ia + <span class="s">1</span>],
                &amp;b_poly.pts[ib],
                &amp;b_poly.pts[ib + <span class="s">1</span>],
            ) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> ta = a_poly.ts[ia] + (a_poly.ts[ia + <span class="s">1</span>] - a_poly.ts[ia]) * s;
            <span class="k">let</span> tb = b_poly.ts[ib] + (b_poly.ts[ib + <span class="s">1</span>] - b_poly.ts[ib]) * t;
            <span class="k">let</span> hit = [
                a_poly.pts[ia][<span class="s">0</span>] + (a_poly.pts[ia + <span class="s">1</span>][<span class="s">0</span>] - a_poly.pts[ia][<span class="s">0</span>]) * s,
                a_poly.pts[ia][<span class="s">1</span>] + (a_poly.pts[ia + <span class="s">1</span>][<span class="s">1</span>] - a_poly.pts[ia][<span class="s">1</span>]) * s,
            ];
            <span class="k">let</span> (hp, ta, tb) = crossing_point(a_poly.cidx, ta, b_poly.cidx, tb, hit, pcurves, dom);
            splits
                .entry((pi, ia))
                .or_default()
                .push((s, hp[<span class="s">0</span>], hp[<span class="s">1</span>], ta));
            splits
                .entry((pj, ib))
                .or_default()
                .push((t, hp[<span class="s">0</span>], hp[<span class="s">1</span>], tb));
        }
    }
}

<span class="c">/// Crossing events of every pair of overlapping polylines with at least one pcurve, keyed by (polyline, segment).</span>
<span class="k">fn</span> polyline_crossings(
    polylines: &amp;[UVPoly],
    pcurves: &amp;[NurbsCurve],
    dom: &amp;SplitDomain,
) -&gt; HashMap&lt;(usize, usize), Vec&lt;(f64, f64, f64, f64)&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> splits: HashMap&lt;(usize, usize), Vec&lt;(f64, f64, f64, f64)&gt;&gt; = HashMap::new();

    <span class="k">for</span> pi <span class="k">in</span> <span class="s">0</span>..polylines.len() {
        <span class="k">for</span> pj <span class="k">in</span> (pi + <span class="s">1</span>)..polylines.len() {
            <span class="k">if</span> (polylines[pi].cidx &gt;= <span class="s">0</span> || polylines[pj].cidx &gt;= <span class="s">0</span>)
                &amp;&amp; boxes_overlap(&amp;polylines[pi], &amp;polylines[pj], dom.snap)
            {
                add_crossings(polylines, pi, pj, pcurves, dom, &amp;<span class="k">mut</span> splits);
            }
        }
    }

    splits
}

<span class="c">/// Graph edges along every polyline between consecutive pool vertices, its crossings inserted in order.</span>
<span class="k">fn</span> split_edges(
    polylines: &amp;[UVPoly],
    splits: &amp;HashMap&lt;(usize, usize), Vec&lt;(f64, f64, f64, f64)&gt;&gt;,
    pool: &amp;<span class="k">mut</span> UVVertexPool,
) -&gt; Vec&lt;SplitEdge&gt; {
    <span class="k">let</span> <span class="k">mut</span> edges: Vec&lt;SplitEdge&gt; = Vec::new();

    <span class="k">for</span> (pi, poly) <span class="k">in</span> polylines.iter().enumerate() {
        <span class="k">let</span> <span class="k">mut</span> chain: Vec&lt;(usize, f64)&gt; = Vec::new();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..poly.pts.len() {
            chain.push((pool.id(poly.pts[i]), poly.ts[i]));

            <span class="k">if</span> i + <span class="s">1</span> &lt; poly.pts.len() {
                <span class="k">if</span> <span class="k">let</span> Some(sp) = splits.get(&amp;(pi, i)) {
                    <span class="k">let</span> <span class="k">mut</span> evs = sp.clone();
                    evs.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

                    <span class="k">for</span> ev <span class="k">in</span> evs {
                        chain.push((pool.id([ev.<span class="s">1</span>, ev.<span class="s">2</span>]), ev.<span class="s">3</span>));
                    }
                }
            }
        }

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..chain.len().saturating_sub(<span class="s">1</span>) {
            <span class="k">let</span> (a, ta) = chain[i];
            <span class="k">let</span> (b, tb) = chain[i + <span class="s">1</span>];

            <span class="k">if</span> a == b {
                <span class="k">continue</span>;
            }

            edges.push(SplitEdge {
                a,
                b,
                cidx: poly.cidx,
                ta,
                tb,
            });
        }
    }

    edges
}

<span class="c">/// Edges left after repeatedly dropping every edge with an end of degree one.</span>
<span class="k">fn</span> prune_dangling(edges: &amp;[SplitEdge]) -&gt; Vec&lt;SplitEdge&gt; {
    <span class="k">let</span> <span class="k">mut</span> alive = vec![<span class="s">true</span>; edges.len()];
    <span class="k">let</span> <span class="k">mut</span> changed = <span class="s">true</span>;

    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..=edges.len() {
        <span class="k">if</span> !changed {
            <span class="k">break</span>;
        }

        changed = <span class="s">false</span>;
        <span class="k">let</span> <span class="k">mut</span> degree: HashMap&lt;usize, usize&gt; = HashMap::new();

        <span class="k">for</span> (ei, e) <span class="k">in</span> edges.iter().enumerate() {
            <span class="k">if</span> !alive[ei] {
                <span class="k">continue</span>;
            }

            *degree.entry(e.a).or_insert(<span class="s">0</span>) += <span class="s">1</span>;
            *degree.entry(e.b).or_insert(<span class="s">0</span>) += <span class="s">1</span>;
        }

        <span class="k">for</span> (ei, e) <span class="k">in</span> edges.iter().enumerate() {
            <span class="k">if</span> !alive[ei] {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> degree.get(&amp;e.a).copied().unwrap_or(<span class="s">0</span>) == <span class="s">1</span>
                || degree.get(&amp;e.b).copied().unwrap_or(<span class="s">0</span>) == <span class="s">1</span>
            {
                alive[ei] = <span class="s">false</span>;
                changed = <span class="s">true</span>;
            }
        }
    }

    <span class="k">let</span> <span class="k">mut</span> live_edges: Vec&lt;SplitEdge&gt; = Vec::new();

    <span class="k">for</span> (ei, e) <span class="k">in</span> edges.iter().enumerate() {
        <span class="k">if</span> alive[ei] {
            live_edges.push(*e);
        }
    }

    live_edges
}

<span class="c">/// Two opposite half-edges per edge, the forward one at the even index.</span>
<span class="k">fn</span> half_edges(edges: &amp;[SplitEdge]) -&gt; Vec&lt;HalfEdge&gt; {
    <span class="k">let</span> <span class="k">mut</span> hes: Vec&lt;HalfEdge&gt; = Vec::new();

    <span class="k">for</span> (ei, e) <span class="k">in</span> edges.iter().enumerate() {
        hes.push(HalfEdge {
            tail: e.a,
            head: e.b,
            eidx: ei,
            fwd: <span class="s">true</span>,
        });
        hes.push(HalfEdge {
            tail: e.b,
            head: e.a,
            eidx: ei,
            fwd: <span class="s">false</span>,
        });
    }

    hes
}

<span class="c">/// Successor of every half-edge around its face: the twin of an outgoing half-edge continues with its predecessor in the angle-sorted fan.</span>
<span class="k">fn</span> next_half_edges(hes: &amp;[HalfEdge], verts: &amp;[[f64; <span class="s">2</span>]]) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> <span class="k">mut</span> out_map: Vec&lt;Vec&lt;usize&gt;&gt; = vec![Vec::new(); verts.len()];

    <span class="k">for</span> (hi, he) <span class="k">in</span> hes.iter().enumerate() {
        out_map[he.tail].push(hi);
    }

    <span class="k">for</span> vid <span class="k">in</span> <span class="s">0</span>..out_map.len() {
        <span class="k">let</span> <span class="k">mut</span> fan: Vec&lt;(f64, usize)&gt; = Vec::new();

        <span class="k">for</span> &amp;hi <span class="k">in</span> &amp;out_map[vid] {
            <span class="k">let</span> angle = (verts[hes[hi].head][<span class="s">1</span>] - verts[vid][<span class="s">1</span>])
                .atan2(verts[hes[hi].head][<span class="s">0</span>] - verts[vid][<span class="s">0</span>]);
            fan.push((angle, hi));
        }

        fan.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..fan.len() {
            out_map[vid][k] = fan[k].<span class="s">1</span>;
        }
    }

    <span class="k">let</span> <span class="k">mut</span> next_he = vec![usize::MAX; hes.len()];

    <span class="k">for</span> outs <span class="k">in</span> &amp;out_map {
        <span class="k">for</span> pos <span class="k">in</span> <span class="s">0</span>..outs.len() {
            next_he[outs[pos] ^ <span class="s">1</span>] = outs[(pos + outs.len() - <span class="s">1</span>) % outs.len()];
        }
    }

    next_he
}

<span class="c">/// Cycles of at least two half-edges traced through next_he, each half-edge in one cycle.</span>
<span class="k">fn</span> face_cycles(next_he: &amp;[usize]) -&gt; Vec&lt;Vec&lt;usize&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> visited = vec![<span class="s">false</span>; next_he.len()];
    <span class="k">let</span> <span class="k">mut</span> faces: Vec&lt;Vec&lt;usize&gt;&gt; = Vec::new();

    <span class="k">for</span> hi <span class="k">in</span> <span class="s">0</span>..next_he.len() {
        <span class="k">if</span> visited[hi] {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> cycle = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> cur = hi;

        <span class="k">while</span> cur != usize::MAX &amp;&amp; !visited[cur] {
            visited[cur] = <span class="s">true</span>;
            cycle.push(cur);
            cur = next_he[cur];
        }

        <span class="k">if</span> cycle.len() &gt;= <span class="s">2</span> {
            faces.push(cycle);
        }
    }

    faces
}

<span class="c">/// Signed area of a half-edge cycle.</span>
<span class="k">fn</span> cycle_area(cycle: &amp;[usize], hes: &amp;[HalfEdge], verts: &amp;[[f64; <span class="s">2</span>]]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> s = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> &amp;hi <span class="k">in</span> cycle {
        <span class="k">let</span> a = verts[hes[hi].tail];
        <span class="k">let</span> b = verts[hes[hi].head];
        s += a[<span class="s">0</span>] * b[<span class="s">1</span>] - b[<span class="s">0</span>] * a[<span class="s">1</span>];
    }

    s * <span class="s">0</span>.<span class="s">5</span>
}

<span class="c">/// True when a cycle passes through a vertex of the domain border.</span>
<span class="k">fn</span> touches_border(cycle: &amp;[usize], hes: &amp;[HalfEdge], border_vids: &amp;HashSet&lt;usize&gt;) -&gt; bool {
    <span class="k">for</span> &amp;hi <span class="k">in</span> cycle {
        <span class="k">if</span> border_vids.contains(&amp;hes[hi].tail) {
            <span class="k">return</span> <span class="s">true</span>;
        }
    }

    <span class="s">false</span>
}

<span class="c">/// Face cycles by orientation: counter-clockwise faces with their area, clockwise holes clear of the domain border.</span>
<span class="k">fn</span> classify_faces(
    faces: Vec&lt;Vec&lt;usize&gt;&gt;,
    hes: &amp;[HalfEdge],
    verts: &amp;[[f64; <span class="s">2</span>]],
    edges: &amp;[SplitEdge],
    snap: f64,
) -&gt; (Vec&lt;(Vec&lt;usize&gt;, f64)&gt;, Vec&lt;Vec&lt;usize&gt;&gt;) {
    <span class="k">let</span> <span class="k">mut</span> border_vids: HashSet&lt;usize&gt; = HashSet::new();

    <span class="k">for</span> e <span class="k">in</span> edges {
        <span class="k">if</span> e.cidx &lt; <span class="s">0</span> {
            border_vids.insert(e.a);
            border_vids.insert(e.b);
        }
    }

    <span class="k">let</span> <span class="k">mut</span> pos_faces: Vec&lt;(Vec&lt;usize&gt;, f64)&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> neg_faces: Vec&lt;Vec&lt;usize&gt;&gt; = Vec::new();

    <span class="k">for</span> cycle <span class="k">in</span> faces {
        <span class="k">let</span> area = cycle_area(&amp;cycle, hes, verts);

        <span class="k">if</span> area &gt; snap * snap {
            pos_faces.push((cycle, area));
        } <span class="k">else</span> <span class="k">if</span> area &lt; -snap * snap &amp;&amp; !touches_border(&amp;cycle, hes, &amp;border_vids) {
            neg_faces.push(cycle);
        }
    }

    (pos_faces, neg_faces)
}

<span class="c">/// Even-odd test of p against a half-edge cycle.</span>
<span class="k">fn</span> point_in_cycle(p: [f64; <span class="s">2</span>], cycle: &amp;[usize], hes: &amp;[HalfEdge], verts: &amp;[[f64; <span class="s">2</span>]]) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> inside = <span class="s">false</span>;

    <span class="k">for</span> &amp;hi <span class="k">in</span> cycle {
        <span class="k">let</span> a = verts[hes[hi].tail];
        <span class="k">let</span> b = verts[hes[hi].head];

        <span class="k">if</span> (a[<span class="s">1</span>] &gt; p[<span class="s">1</span>]) != (b[<span class="s">1</span>] &gt; p[<span class="s">1</span>])
            &amp;&amp; p[<span class="s">0</span>] &lt; (b[<span class="s">0</span>] - a[<span class="s">0</span>]) * (p[<span class="s">1</span>] - a[<span class="s">1</span>]) / (b[<span class="s">1</span>] - a[<span class="s">1</span>]) + a[<span class="s">0</span>]
        {
            inside = !inside;
        }
    }

    inside
}

<span class="c">/// True when two cycles pass through the same set of vertices.</span>
<span class="k">fn</span> same_vertices(a: &amp;[usize], b: &amp;[usize], hes: &amp;[HalfEdge]) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> a_vids: HashSet&lt;usize&gt; = HashSet::new();
    <span class="k">let</span> <span class="k">mut</span> b_vids: HashSet&lt;usize&gt; = HashSet::new();

    <span class="k">for</span> &amp;hi <span class="k">in</span> a {
        a_vids.insert(hes[hi].tail);
    }

    <span class="k">for</span> &amp;hi <span class="k">in</span> b {
        b_vids.insert(hes[hi].tail);
    }

    a_vids == b_vids
}

<span class="c">/// Holes per positive face: each hole goes to the smallest face that contains it and is not its own vertex ring.</span>
<span class="k">fn</span> assign_holes(
    neg_faces: &amp;[Vec&lt;usize&gt;],
    pos_faces: &amp;[(Vec&lt;usize&gt;, f64)],
    hes: &amp;[HalfEdge],
    verts: &amp;[[f64; <span class="s">2</span>]],
) -&gt; Vec&lt;Vec&lt;Vec&lt;usize&gt;&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> holes_of: Vec&lt;Vec&lt;Vec&lt;usize&gt;&gt;&gt; = vec![Vec::new(); pos_faces.len()];

    <span class="k">for</span> cycle <span class="k">in</span> neg_faces {
        <span class="k">let</span> sample = verts[hes[cycle[<span class="s">0</span>]].tail];
        <span class="k">let</span> <span class="k">mut</span> best: i32 = -<span class="s">1</span>;
        <span class="k">let</span> <span class="k">mut</span> best_area = f64::INFINITY;

        <span class="k">for</span> (fi, (fc, area)) <span class="k">in</span> pos_faces.iter().enumerate() {
            <span class="k">if</span> *area &lt; best_area
                &amp;&amp; point_in_cycle(sample, fc, hes, verts)
                &amp;&amp; !same_vertices(cycle, fc, hes)
            {
                best = fi <span class="k">as</span> i32;
                best_area = *area;
            }
        }

        <span class="k">if</span> best &gt;= <span class="s">0</span> {
            holes_of[best <span class="k">as</span> usize].push(cycle.clone());
        }
    }

    holes_of
}

<span class="c">/// Runs of a cycle: consecutive half-edges on one pcurve merged.</span>
<span class="k">fn</span> cycle_runs(cycle: &amp;[usize], hes: &amp;[HalfEdge], edges: &amp;[SplitEdge]) -&gt; Vec&lt;Run&gt; {
    <span class="k">let</span> <span class="k">mut</span> runs: Vec&lt;Run&gt; = Vec::new();

    <span class="k">for</span> &amp;hi <span class="k">in</span> cycle {
        <span class="k">let</span> he = &amp;hes[hi];
        <span class="k">let</span> e = edges[he.eidx];
        <span class="k">let</span> ta = <span class="k">if</span> he.fwd { e.ta } <span class="k">else</span> { e.tb };
        <span class="k">let</span> tb = <span class="k">if</span> he.fwd { e.tb } <span class="k">else</span> { e.ta };

        <span class="k">if</span> <span class="k">let</span> Some(last) = runs.last_mut() {
            <span class="k">if</span> last.cidx == e.cidx &amp;&amp; last.vb == he.tail {
                last.vb = he.head;
                last.tb = tb;
                <span class="k">continue</span>;
            }
        }

        runs.push(Run {
            cidx: e.cidx,
            va: he.tail,
            vb: he.head,
            ta,
            tb,
        });
    }

    runs
}

<span class="c">/// Pcurve piece of a run, trimmed to its parameters and oriented along it; None when the run cannot be cut.</span>
<span class="k">fn</span> run_piece(run: &amp;Run, pcurves: &amp;[NurbsCurve]) -&gt; Option&lt;NurbsCurve&gt; {
    <span class="k">if</span> run.cidx &lt; <span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> crv = &amp;pcurves[run.cidx <span class="k">as</span> usize];
    <span class="k">let</span> (c0, c1) = crv.domain();
    <span class="k">let</span> lo = c0.max(run.ta.min(run.tb));
    <span class="k">let</span> hi_ = c1.min(run.ta.max(run.tb));
    <span class="k">let</span> <span class="k">mut</span> piece = crv.duplicate();

    <span class="k">if</span> hi_ - lo &lt; (c1 - c0) - <span class="s">1</span>e-<span class="s">12</span> &amp;&amp; hi_ - lo &gt; <span class="s">1</span>e-<span class="s">14</span> {
        <span class="k">if</span> !piece.trim(lo, hi_) {
            <span class="k">return</span> None;
        }
    } <span class="k">else</span> <span class="k">if</span> hi_ - lo &lt;= <span class="s">1</span>e-<span class="s">14</span> &amp;&amp; !(run.va == run.vb &amp;&amp; piece.is_closed()) {
        <span class="k">return</span> None;
    }

    <span class="k">if</span> !piece.is_valid() {
        <span class="k">return</span> None;
    }

    <span class="k">if</span> run.ta &gt; run.tb &amp;&amp; !piece.reverse() {
        <span class="k">return</span> None;
    }

    Some(piece)
}

<span class="c">/// Pieces of a cycle: trimmed pcurve runs, straight UV segments where a run cannot be cut.</span>
<span class="k">fn</span> cycle_to_segments(
    cycle: &amp;[usize],
    hes: &amp;[HalfEdge],
    edges: &amp;[SplitEdge],
    verts: &amp;[[f64; <span class="s">2</span>]],
    pcurves: &amp;[NurbsCurve],
) -&gt; Vec&lt;NurbsCurve&gt; {
    <span class="k">let</span> <span class="k">mut</span> pieces: Vec&lt;NurbsCurve&gt; = Vec::new();

    <span class="k">for</span> run <span class="k">in</span> &amp;cycle_runs(cycle, hes, edges) {
        <span class="k">if</span> <span class="k">let</span> Some(piece) = run_piece(run, pcurves) {
            pieces.push(piece);
            <span class="k">continue</span>;
        }

        <span class="k">let</span> pa = verts[run.va];
        <span class="k">let</span> pb = verts[run.vb];

        <span class="k">if</span> (pb[<span class="s">0</span>] - pa[<span class="s">0</span>]).hypot(pb[<span class="s">1</span>] - pa[<span class="s">1</span>]) &gt; <span class="s">1</span>e-<span class="s">14</span> {
            <span class="k">let</span> seg_pts = vec![Point::new(pa[<span class="s">0</span>], pa[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>), Point::new(pb[<span class="s">0</span>], pb[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>)];
            pieces.push(NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;seg_pts));
        }
    }

    pieces
}

<span class="c">/// Close a curve whose ends lie within tol by moving its last control point onto the first; true when it ends closed.</span>
<span class="k">fn</span> close_curve(curve: &amp;<span class="k">mut</span> NurbsCurve, tol: f64) -&gt; bool {
    <span class="k">if</span> curve.is_closed() {
        <span class="k">return</span> <span class="s">true</span>;
    }

    <span class="k">if</span> curve.point_at_start().distance(&amp;curve.point_at_end(), None) &gt; tol {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> last = curve.cv_count() - <span class="s">1</span>;
    <span class="k">let</span> (Some((x, y, z, _w)), Some((_xe, _ye, _ze, we))) =
        (curve.get_cv_4d(<span class="s">0</span>), curve.get_cv_4d(last))
    <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };

    curve.set_cv_4d(last, x, y, z, we) &amp;&amp; curve.is_closed()
}

<span class="c">/// Closed loop of a cycle: the joined pieces when they close, else the polygon through its vertices.</span>
<span class="k">fn</span> cycle_to_loop(
    cycle: &amp;[usize],
    hes: &amp;[HalfEdge],
    edges: &amp;[SplitEdge],
    verts: &amp;[[f64; <span class="s">2</span>]],
    pcurves: &amp;[NurbsCurve],
    snap_uv: f64,
) -&gt; NurbsCurve {
    <span class="k">let</span> pieces = cycle_to_segments(cycle, hes, edges, verts, pcurves);

    <span class="k">if</span> pieces.is_empty() {
        <span class="k">return</span> NurbsCurve::default();
    }

    <span class="k">let</span> join_tol = snap_uv * <span class="s">4</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> joined = NurbsCurve::join(&amp;pieces, Some(join_tol));

    <span class="k">if</span> joined.len() == <span class="s">1</span> &amp;&amp; joined[<span class="s">0</span>].is_valid() &amp;&amp; close_curve(&amp;<span class="k">mut</span> joined[<span class="s">0</span>], join_tol) {
        <span class="k">return</span> joined.remove(<span class="s">0</span>);
    }

    <span class="k">let</span> <span class="k">mut</span> loop_pts: Vec&lt;Point&gt; = Vec::new();

    <span class="k">for</span> &amp;hi <span class="k">in</span> cycle {
        <span class="k">let</span> a = verts[hes[hi].tail];
        loop_pts.push(Point::new(a[<span class="s">0</span>], a[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>));
    }

    loop_pts.push(Point::new(loop_pts[<span class="s">0</span>][<span class="s">0</span>], loop_pts[<span class="s">0</span>][<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>));

    NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;loop_pts)
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Delaunay2D</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// UV vertex of the triangulation.</span>
<span class="k">struct</span> Vertex2D {
    x: f64, <span class="c">// U coordinate.</span>
    y: f64, <span class="c">// V coordinate.</span>
}

<span class="c">/// Triangle with per-edge neighbours; edge k is opposite vertex k.</span>
<span class="k">struct</span> Triangle {
    v: [i32; <span class="s">3</span>],            <span class="c">// Vertex indices.</span>
    adj: [i32; <span class="s">3</span>],          <span class="c">// Neighbour across each edge, -1 on the hull.</span>
    constrained: [bool; <span class="s">3</span>], <span class="c">// True where an edge is a constraint.</span>
    alive: bool,            <span class="c">// False once removed.</span>
}

<span class="c">/// Incremental constrained Delaunay triangulation in UV with Bowyer-Watson insertion.</span>
<span class="k">struct</span> Delaunay2D {
    vertices: Vec&lt;Vertex2D&gt;,  <span class="c">// Vertices, the super triangle first.</span>
    triangles: Vec&lt;Triangle&gt;, <span class="c">// Triangle pool, dead ones flagged.</span>
    super_v: [i32; <span class="s">3</span>],        <span class="c">// Super triangle vertices.</span>
    edge_map: HashMap&lt;(i32, i32), (i32, i32)&gt;, <span class="c">// Hull edge -&gt; (triangle, edge index).</span>
    last_found: i32,          <span class="c">// Triangle the last locate ended in.</span>
    visit_epoch: i32,         <span class="c">// Stamp of the current search.</span>
    visit_stamp: Vec&lt;i32&gt;,    <span class="c">// Last search stamp per triangle.</span>
}

<span class="k">impl</span> Delaunay2D {
    <span class="c">/// Order-independent key of an edge.</span>
    <span class="k">fn</span> edge_key(a: i32, b: i32) -&gt; (i32, i32) {
        (a.min(b), a.max(b))
    }

    <span class="c">/// Positive when d lies inside the circumcircle of a, b, c.</span>
    <span class="k">fn</span> in_circumcircle(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        cx: f64,
        cy: f64,
        dx: f64,
        dy: f64,
    ) -&gt; f64 {
        <span class="k">let</span> adx = ax - dx;
        <span class="k">let</span> ady = ay - dy;
        <span class="k">let</span> bdx = bx - dx;
        <span class="k">let</span> bdy = by - dy;
        <span class="k">let</span> cdx = cx - dx;
        <span class="k">let</span> cdy = cy - dy;

        (adx * adx + ady * ady) * (bdx * cdy - cdx * bdy)
            + (bdx * bdx + bdy * bdy) * (cdx * ady - adx * cdy)
            + (cdx * cdx + cdy * cdy) * (adx * bdy - bdx * ady)
    }

    <span class="c">/// Twice the signed area of a, b, c.</span>
    <span class="k">fn</span> orient2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -&gt; f64 {
        (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
    }

    <span class="c">/// Construct with a super triangle around the box.</span>
    <span class="k">fn</span> new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> dx = xmax - xmin;
        <span class="k">let</span> dy = ymax - ymin;
        <span class="k">let</span> d = dx.max(dy);
        <span class="k">let</span> cx = (xmin + xmax) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> cy = (ymin + ymax) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> scale = <span class="s">20</span>.<span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> dt = Delaunay2D {
            vertices: Vec::new(),
            triangles: Vec::new(),
            super_v: [-<span class="s">1</span>; <span class="s">3</span>],
            edge_map: HashMap::new(),
            last_found: <span class="s">0</span>,
            visit_epoch: <span class="s">0</span>,
            visit_stamp: Vec::new(),
        };
        dt.vertices.push(Vertex2D {
            x: cx - scale * d,
            y: cy - scale * d,
        });
        dt.vertices.push(Vertex2D {
            x: cx + scale * d,
            y: cy - scale * d,
        });
        dt.vertices.push(Vertex2D {
            x: cx,
            y: cy + scale * d,
        });
        dt.super_v = [<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>];
        dt.triangles.push(Triangle {
            v: [<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>],
            adj: [-<span class="s">1</span>, -<span class="s">1</span>, -<span class="s">1</span>],
            constrained: [<span class="s">false</span>; <span class="s">3</span>],
            alive: <span class="s">true</span>,
        });
        dt.register_edges(<span class="s">0</span>);

        dt
    }

    <span class="c">/// Record the hull edges of triangle ti in the edge map.</span>
    <span class="k">fn</span> register_edges(&amp;<span class="k">mut</span> <span class="k">self</span>, ti: i32) {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> a = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">1</span>) % <span class="s">3</span>];
            <span class="k">let</span> b = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">2</span>) % <span class="s">3</span>];
            <span class="k">let</span> key = <span class="k">Self</span>::edge_key(a, b);

            <span class="k">if</span> <span class="k">let</span> Some(&amp;(oti, ok)) = <span class="k">self</span>.edge_map.get(&amp;key) {
                <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].adj[k] = oti;
                <span class="k">self</span>.triangles[oti <span class="k">as</span> usize].adj[ok <span class="k">as</span> usize] = ti;
                <span class="k">self</span>.edge_map.remove(&amp;key);
            } <span class="k">else</span> {
                <span class="k">self</span>.edge_map.insert(key, (ti, k <span class="k">as</span> i32));
            }
        }
    }

    <span class="c">/// Drop the hull edges of triangle ti from the edge map and its neighbours.</span>
    <span class="k">fn</span> unregister_edges(&amp;<span class="k">mut</span> <span class="k">self</span>, ti: i32) {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> a = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">1</span>) % <span class="s">3</span>];
            <span class="k">let</span> b = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">2</span>) % <span class="s">3</span>];
            <span class="k">let</span> key = <span class="k">Self</span>::edge_key(a, b);
            <span class="k">let</span> adj_ti = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].adj[k];

            <span class="k">if</span> adj_ti &gt;= <span class="s">0</span>
                &amp;&amp; adj_ti &lt; <span class="k">self</span>.triangles.len() <span class="k">as</span> i32
                &amp;&amp; <span class="k">self</span>.triangles[adj_ti <span class="k">as</span> usize].alive
            {
                <span class="k">let</span> <span class="k">mut</span> adj_kk = -<span class="s">1i32</span>;

                <span class="k">for</span> kk <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                    <span class="k">if</span> <span class="k">self</span>.triangles[adj_ti <span class="k">as</span> usize].adj[kk] == ti {
                        adj_kk = kk <span class="k">as</span> i32;
                        <span class="k">break</span>;
                    }
                }

                <span class="k">if</span> adj_kk &gt;= <span class="s">0</span> {
                    <span class="k">self</span>.triangles[adj_ti <span class="k">as</span> usize].adj[adj_kk <span class="k">as</span> usize] = -<span class="s">1</span>;
                    <span class="k">let</span> adj_a = <span class="k">self</span>.triangles[adj_ti <span class="k">as</span> usize].v[(adj_kk <span class="k">as</span> usize + <span class="s">1</span>) % <span class="s">3</span>];
                    <span class="k">let</span> adj_b = <span class="k">self</span>.triangles[adj_ti <span class="k">as</span> usize].v[(adj_kk <span class="k">as</span> usize + <span class="s">2</span>) % <span class="s">3</span>];
                    <span class="k">self</span>.edge_map
                        .insert(<span class="k">Self</span>::edge_key(adj_a, adj_b), (adj_ti, adj_kk));
                }
            } <span class="k">else</span> <span class="k">if</span> <span class="k">let</span> Some(&amp;(eti, _)) = <span class="k">self</span>.edge_map.get(&amp;key) {
                <span class="k">if</span> eti == ti {
                    <span class="k">self</span>.edge_map.remove(&amp;key);
                }
            }
        }
    }

    <span class="c">/// Triangle containing (x, y) by walking from start_tri, -1 when none.</span>
    <span class="k">fn</span> locate(&amp;<span class="k">self</span>, x: f64, y: f64, <span class="k">mut</span> start_tri: i32) -&gt; i32 {
        <span class="k">if</span> start_tri &lt; <span class="s">0</span>
            || start_tri &gt;= <span class="k">self</span>.triangles.len() <span class="k">as</span> i32
            || !<span class="k">self</span>.triangles[start_tri <span class="k">as</span> usize].alive
        {
            start_tri = <span class="k">self</span>.triangles.len() <span class="k">as</span> i32 - <span class="s">1</span>;

            <span class="k">while</span> start_tri &gt;= <span class="s">0</span> &amp;&amp; !<span class="k">self</span>.triangles[start_tri <span class="k">as</span> usize].alive {
                start_tri -= <span class="s">1</span>;
            }

            <span class="k">if</span> start_tri &lt; <span class="s">0</span> {
                <span class="k">return</span> -<span class="s">1</span>;
            }
        }

        <span class="k">let</span> <span class="k">mut</span> cur = start_tri;
        <span class="k">let</span> max_iter = <span class="k">self</span>.triangles.len() <span class="k">as</span> i32;

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..max_iter {
            <span class="k">let</span> t = &amp;<span class="k">self</span>.triangles[cur <span class="k">as</span> usize];
            <span class="k">let</span> <span class="k">mut</span> moved = <span class="s">false</span>;

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> a = t.v[k];
                <span class="k">let</span> b = t.v[(k + <span class="s">1</span>) % <span class="s">3</span>];
                <span class="k">let</span> ax = <span class="k">self</span>.vertices[a <span class="k">as</span> usize].x;
                <span class="k">let</span> ay = <span class="k">self</span>.vertices[a <span class="k">as</span> usize].y;
                <span class="k">let</span> bx = <span class="k">self</span>.vertices[b <span class="k">as</span> usize].x;
                <span class="k">let</span> by = <span class="k">self</span>.vertices[b <span class="k">as</span> usize].y;

                <span class="k">if</span> <span class="k">Self</span>::orient2d(ax, ay, bx, by, x, y) &lt; <span class="s">0</span>.<span class="s">0</span> {
                    <span class="k">let</span> opp = t.adj[(k + <span class="s">2</span>) % <span class="s">3</span>];

                    <span class="k">if</span> opp &gt;= <span class="s">0</span>
                        &amp;&amp; (opp <span class="k">as</span> usize) &lt; <span class="k">self</span>.triangles.len()
                        &amp;&amp; <span class="k">self</span>.triangles[opp <span class="k">as</span> usize].alive
                    {
                        cur = opp;
                        moved = <span class="s">true</span>;
                        <span class="k">break</span>;
                    }
                }
            }

            <span class="k">if</span> !moved {
                <span class="k">return</span> cur;
            }
        }

        cur
    }

    <span class="c">/// Insert a point and return its vertex index, the existing one when coincident.</span>
    <span class="k">fn</span> insert(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; i32 {
        <span class="k">let</span> start = <span class="k">self</span>.locate(x, y, <span class="k">self</span>.last_found);
        <span class="k">let</span> existing = <span class="k">self</span>.find_coincident(start, x, y);

        <span class="k">if</span> existing &gt;= <span class="s">0</span> {
            <span class="k">return</span> existing;
        }

        <span class="k">let</span> vi = <span class="k">self</span>.vertices.len() <span class="k">as</span> i32;
        <span class="k">self</span>.vertices.push(Vertex2D { x, y });
        <span class="k">let</span> bad = <span class="k">self</span>.collect_cavity(start, x, y);

        <span class="k">if</span> bad.is_empty() {
            <span class="k">self</span>.vertices.pop();

            <span class="k">return</span> -<span class="s">1</span>;
        }

        <span class="k">let</span> polygon = <span class="k">self</span>.cavity_polygon(&amp;bad);
        <span class="k">self</span>.fill_cavity(vi, &amp;bad, &amp;polygon);
        <span class="k">self</span>.last_found = <span class="k">self</span>.triangles.len() <span class="k">as</span> i32 - <span class="s">1</span>;

        vi
    }

    <span class="c">/// Corner of triangle ti holding vertex v, -1 when none does.</span>
    <span class="k">fn</span> vertex_index(&amp;<span class="k">self</span>, ti: i32, v: i32) -&gt; i32 {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">if</span> <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[k] == v {
                <span class="k">return</span> k <span class="k">as</span> i32;
            }
        }

        -<span class="s">1</span>
    }

    <span class="c">/// Vertex of triangle ti across its edge shared with triangle nb, -1 when they are not neighbours.</span>
    <span class="k">fn</span> opposite_vertex(&amp;<span class="k">self</span>, ti: i32, nb: i32) -&gt; i32 {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">if</span> <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].adj[k] == nb {
                <span class="k">return</span> <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[k];
            }
        }

        -<span class="s">1</span>
    }

    <span class="c">/// Vertex of triangle start within 1e-6 of (x, y), -1 when none.</span>
    <span class="k">fn</span> find_coincident(&amp;<span class="k">self</span>, start: i32, x: f64, y: f64) -&gt; i32 {
        <span class="k">if</span> start &lt; <span class="s">0</span> || !<span class="k">self</span>.triangles[start <span class="k">as</span> usize].alive {
            <span class="k">return</span> -<span class="s">1</span>;
        }

        <span class="k">for</span> &amp;vi <span class="k">in</span> &amp;<span class="k">self</span>.triangles[start <span class="k">as</span> usize].v {
            <span class="k">let</span> ddx = <span class="k">self</span>.vertices[vi <span class="k">as</span> usize].x - x;
            <span class="k">let</span> ddy = <span class="k">self</span>.vertices[vi <span class="k">as</span> usize].y - y;

            <span class="k">if</span> ddx * ddx + ddy * ddy &lt; <span class="s">1</span>e-<span class="s">12</span> {
                <span class="k">return</span> vi;
            }
        }

        -<span class="s">1</span>
    }

    <span class="c">/// True when (x, y) lies inside the circumcircle of triangle ti.</span>
    <span class="k">fn</span> circumcircle_contains(&amp;<span class="k">self</span>, ti: i32, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> [v0, v1, v2] = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v;
        <span class="k">let</span> ax = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].x;
        <span class="k">let</span> ay = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].y;
        <span class="k">let</span> bx = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].x;
        <span class="k">let</span> by = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].y;
        <span class="k">let</span> cx = <span class="k">self</span>.vertices[v2 <span class="k">as</span> usize].x;
        <span class="k">let</span> cy = <span class="k">self</span>.vertices[v2 <span class="k">as</span> usize].y;
        <span class="k">let</span> o = <span class="k">Self</span>::orient2d(ax, ay, bx, by, cx, cy);
        <span class="k">let</span> ic = <span class="k">if</span> o &gt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">Self</span>::in_circumcircle(ax, ay, bx, by, cx, cy, x, y)
        } <span class="k">else</span> {
            <span class="k">Self</span>::in_circumcircle(ax, ay, cx, cy, bx, by, x, y)
        };

        ic &gt; <span class="s">0</span>.<span class="s">0</span>
    }

    <span class="c">/// Triangles whose circumcircle holds (x, y), grown from start across unconstrained edges.</span>
    <span class="k">fn</span> collect_cavity(&amp;<span class="k">mut</span> <span class="k">self</span>, start: i32, x: f64, y: f64) -&gt; Vec&lt;i32&gt; {
        <span class="k">self</span>.visit_epoch += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">self</span>.visit_stamp.len() &lt; <span class="k">self</span>.triangles.len() + <span class="s">64</span> {
            <span class="k">self</span>.visit_stamp.resize(<span class="k">self</span>.triangles.len() + <span class="s">64</span>, <span class="s">0</span>);
        }

        <span class="k">let</span> <span class="k">mut</span> bad: Vec&lt;i32&gt; = Vec::new();

        <span class="k">if</span> start &gt;= <span class="s">0</span> {
            bad.push(start);
            <span class="k">self</span>.visit_stamp[start <span class="k">as</span> usize] = <span class="k">self</span>.visit_epoch;
        }

        <span class="k">let</span> <span class="k">mut</span> front = <span class="s">0</span>;

        <span class="k">while</span> front &lt; bad.len() {
            <span class="k">let</span> ti = bad[front];
            front += <span class="s">1</span>;

            <span class="k">if</span> !<span class="k">self</span>.triangles[ti <span class="k">as</span> usize].alive || !<span class="k">self</span>.circumcircle_contains(ti, x, y) {
                bad[front - <span class="s">1</span>] = -<span class="s">1</span>;
                <span class="k">continue</span>;
            }

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> nb = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].adj[k];

                <span class="k">if</span> <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].constrained[k]
                    || nb &lt; <span class="s">0</span>
                    || <span class="k">self</span>.visit_stamp[nb <span class="k">as</span> usize] == <span class="k">self</span>.visit_epoch
                {
                    <span class="k">continue</span>;
                }

                <span class="k">self</span>.visit_stamp[nb <span class="k">as</span> usize] = <span class="k">self</span>.visit_epoch;
                bad.push(nb);
            }
        }

        bad.retain(|&amp;ti| ti &gt;= <span class="s">0</span>);

        bad
    }

    <span class="c">/// Edges of the bad triangles that face a good neighbour or the hull.</span>
    <span class="k">fn</span> cavity_polygon(&amp;<span class="k">self</span>, bad: &amp;[i32]) -&gt; Vec&lt;(i32, i32, bool)&gt; {
        <span class="k">let</span> <span class="k">mut</span> bad_set: HashSet&lt;i32&gt; = HashSet::new();

        <span class="k">for</span> &amp;ti <span class="k">in</span> bad {
            bad_set.insert(ti);
        }

        <span class="k">let</span> <span class="k">mut</span> polygon: Vec&lt;(i32, i32, bool)&gt; = Vec::new();

        <span class="k">for</span> &amp;ti <span class="k">in</span> bad {
            <span class="k">let</span> t = &amp;<span class="k">self</span>.triangles[ti <span class="k">as</span> usize];

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> nb = t.adj[k];

                <span class="k">if</span> nb &lt; <span class="s">0</span> || !bad_set.contains(&amp;nb) {
                    polygon.push((t.v[(k + <span class="s">1</span>) % <span class="s">3</span>], t.v[(k + <span class="s">2</span>) % <span class="s">3</span>], t.constrained[k]));
                }
            }
        }

        polygon
    }

    <span class="c">/// Replace the bad triangles by a fan from vertex vi to the polygon edges.</span>
    <span class="k">fn</span> fill_cavity(&amp;<span class="k">mut</span> <span class="k">self</span>, vi: i32, bad: &amp;[i32], polygon: &amp;[(i32, i32, bool)]) {
        <span class="k">for</span> &amp;ti <span class="k">in</span> bad {
            <span class="k">self</span>.unregister_edges(ti);
            <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].alive = <span class="s">false</span>;
        }

        <span class="k">for</span> &amp;(e0, e1, constr) <span class="k">in</span> polygon {
            <span class="k">let</span> o = <span class="k">Self</span>::orient2d(
                <span class="k">self</span>.vertices[vi <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[vi <span class="k">as</span> usize].y,
                <span class="k">self</span>.vertices[e0 <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[e0 <span class="k">as</span> usize].y,
                <span class="k">self</span>.vertices[e1 <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[e1 <span class="k">as</span> usize].y,
            );

            <span class="k">if</span> o.abs() &lt; <span class="s">1</span>e-<span class="s">20</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> new_ti = <span class="k">self</span>.triangles.len() <span class="k">as</span> i32;
            <span class="k">let</span> (va, vb) = <span class="k">if</span> o &gt; <span class="s">0</span>.<span class="s">0</span> { (e0, e1) } <span class="k">else</span> { (e1, e0) };
            <span class="k">self</span>.triangles.push(Triangle {
                v: [vi, va, vb],
                adj: [-<span class="s">1</span>, -<span class="s">1</span>, -<span class="s">1</span>],
                constrained: [constr, <span class="s">false</span>, <span class="s">false</span>],
                alive: <span class="s">true</span>,
            });
            <span class="k">self</span>.register_edges(new_ti);
        }
    }

    <span class="c">/// Force the edge v0-v1 into the triangulation by flipping the edges it crosses.</span>
    <span class="k">fn</span> insert_constraint(&amp;<span class="k">mut</span> <span class="k">self</span>, v0: i32, v1: i32) {
        <span class="k">if</span> v0 == v1 || <span class="k">self</span>.constrain_existing(v0, v1) {
            <span class="k">return</span>;
        }

        <span class="k">let</span> start_ti = <span class="k">self</span>.first_triangle_at(v0);

        <span class="k">if</span> start_ti &lt; <span class="s">0</span> {
            <span class="k">return</span>;
        }

        <span class="k">let</span> Some((it, ivl, ivr)) = <span class="k">self</span>.first_crossed(start_ti, v0, v1) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> poly_l: Vec&lt;i32&gt; = vec![v0, ivl];
        <span class="k">let</span> <span class="k">mut</span> poly_r: Vec&lt;i32&gt; = vec![v0, ivr];
        <span class="k">let</span> <span class="k">mut</span> intersected: Vec&lt;i32&gt; = vec![it];
        <span class="k">self</span>.walk_crossed(v0, v1, ivl, ivr, &amp;<span class="k">mut</span> poly_l, &amp;<span class="k">mut</span> poly_r, &amp;<span class="k">mut</span> intersected);
        poly_l.push(v1);
        poly_r.push(v1);
        <span class="k">self</span>.retriangulate(v0, v1, &amp;poly_l, &amp;poly_r, &amp;intersected);
    }

    <span class="c">/// Mark v0-v1 constrained when it already is a triangle edge; false when it is not.</span>
    <span class="k">fn</span> constrain_existing(&amp;<span class="k">mut</span> <span class="k">self</span>, v0: i32, v1: i32) -&gt; bool {
        <span class="k">for</span> ti <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() {
            <span class="k">if</span> !<span class="k">self</span>.triangles[ti].alive {
                <span class="k">continue</span>;
            }

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> e0 = <span class="k">self</span>.triangles[ti].v[(k + <span class="s">1</span>) % <span class="s">3</span>];
                <span class="k">let</span> e1 = <span class="k">self</span>.triangles[ti].v[(k + <span class="s">2</span>) % <span class="s">3</span>];

                <span class="k">if</span> !((e0 == v0 &amp;&amp; e1 == v1) || (e0 == v1 &amp;&amp; e1 == v0)) {
                    <span class="k">continue</span>;
                }

                <span class="k">self</span>.triangles[ti].constrained[k] = <span class="s">true</span>;
                <span class="k">let</span> nb = <span class="k">self</span>.triangles[ti].adj[k];

                <span class="k">if</span> nb &gt;= <span class="s">0</span>
                    &amp;&amp; (nb <span class="k">as</span> usize) &lt; <span class="k">self</span>.triangles.len()
                    &amp;&amp; <span class="k">self</span>.triangles[nb <span class="k">as</span> usize].alive
                {
                    <span class="k">for</span> kk <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                        <span class="k">if</span> <span class="k">self</span>.triangles[nb <span class="k">as</span> usize].adj[kk] == ti <span class="k">as</span> i32 {
                            <span class="k">self</span>.triangles[nb <span class="k">as</span> usize].constrained[kk] = <span class="s">true</span>;
                            <span class="k">break</span>;
                        }
                    }
                }

                <span class="k">return</span> <span class="s">true</span>;
            }
        }

        <span class="s">false</span>
    }

    <span class="c">/// Lowest live triangle with vertex v, -1 when none.</span>
    <span class="k">fn</span> first_triangle_at(&amp;<span class="k">self</span>, v: i32) -&gt; i32 {
        <span class="k">for</span> ti <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() {
            <span class="k">if</span> <span class="k">self</span>.triangles[ti].alive &amp;&amp; <span class="k">self</span>.has_vertex(ti <span class="k">as</span> i32, v) {
                <span class="k">return</span> ti <span class="k">as</span> i32;
            }
        }

        -<span class="s">1</span>
    }

    <span class="c">/// Triangle around v0 whose opposite edge the segment v0-v1 crosses, with that edge's left and right ends; None when none.</span>
    <span class="k">fn</span> first_crossed(&amp;<span class="k">self</span>, start_ti: i32, v0: i32, v1: i32) -&gt; Option&lt;(i32, i32, i32)&gt; {
        <span class="k">let</span> ax = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].x;
        <span class="k">let</span> ay = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].y;
        <span class="k">let</span> bx = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].x;
        <span class="k">let</span> by = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].y;
        <span class="k">let</span> <span class="k">mut</span> ti = start_ti;

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() + <span class="s">4</span> {
            <span class="k">if</span> !<span class="k">self</span>.triangles[ti <span class="k">as</span> usize].alive {
                <span class="k">return</span> None;
            }

            <span class="k">let</span> k_v0 = <span class="k">self</span>.vertex_index(ti, v0);

            <span class="k">if</span> k_v0 &lt; <span class="s">0</span> {
                <span class="k">return</span> None;
            }

            <span class="k">let</span> k = k_v0 <span class="k">as</span> usize;
            <span class="k">let</span> ip2 = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">1</span>) % <span class="s">3</span>];
            <span class="k">let</span> ip1 = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[(k + <span class="s">2</span>) % <span class="s">3</span>];
            <span class="k">let</span> p2 = &amp;<span class="k">self</span>.vertices[ip2 <span class="k">as</span> usize];
            <span class="k">let</span> p1 = &amp;<span class="k">self</span>.vertices[ip1 <span class="k">as</span> usize];
            <span class="k">let</span> op2 = <span class="k">Self</span>::orient2d(ax, ay, bx, by, p2.x, p2.y);
            <span class="k">let</span> op1 = <span class="k">Self</span>::orient2d(ax, ay, bx, by, p1.x, p1.y);

            <span class="k">if</span> op2 &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; op1 &gt;= <span class="s">0</span>.<span class="s">0</span> {
                <span class="k">return</span> Some((ti, ip1, ip2));
            }

            <span class="k">let</span> next = <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].adj[(k + <span class="s">1</span>) % <span class="s">3</span>];

            <span class="k">if</span> next &lt; <span class="s">0</span> || !<span class="k">self</span>.triangles[next <span class="k">as</span> usize].alive || next == start_ti {
                <span class="k">return</span> None;
            }

            ti = next;
        }

        None
    }

    <span class="c">/// Walk the triangles crossed by v0-v1 from intersected[0], collecting the vertices left and right of it.</span>
    <span class="k">fn</span> walk_crossed(
        &amp;<span class="k">self</span>,
        v0: i32,
        v1: i32,
        <span class="k">mut</span> ivl: i32,
        <span class="k">mut</span> ivr: i32,
        poly_l: &amp;<span class="k">mut</span> Vec&lt;i32&gt;,
        poly_r: &amp;<span class="k">mut</span> Vec&lt;i32&gt;,
        intersected: &amp;<span class="k">mut</span> Vec&lt;i32&gt;,
    ) {
        <span class="k">let</span> ax = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].x;
        <span class="k">let</span> ay = <span class="k">self</span>.vertices[v0 <span class="k">as</span> usize].y;
        <span class="k">let</span> bx = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].x;
        <span class="k">let</span> by = <span class="k">self</span>.vertices[v1 <span class="k">as</span> usize].y;
        <span class="k">let</span> <span class="k">mut</span> iv = v0;
        <span class="k">let</span> <span class="k">mut</span> cur_it = intersected[<span class="s">0</span>];

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() * <span class="s">2</span> + <span class="s">8</span> {
            <span class="k">if</span> <span class="k">self</span>.has_vertex(cur_it, v1) {
                <span class="k">break</span>;
            }

            <span class="k">let</span> k_iv = <span class="k">self</span>.vertex_index(cur_it, iv);

            <span class="k">if</span> k_iv &lt; <span class="s">0</span> {
                <span class="k">break</span>;
            }

            <span class="k">let</span> i_topo = <span class="k">self</span>.triangles[cur_it <span class="k">as</span> usize].adj[k_iv <span class="k">as</span> usize];

            <span class="k">if</span> i_topo &lt; <span class="s">0</span> || !<span class="k">self</span>.triangles[i_topo <span class="k">as</span> usize].alive {
                <span class="k">break</span>;
            }

            <span class="k">let</span> i_vopo = <span class="k">self</span>.opposite_vertex(i_topo, cur_it);

            <span class="k">if</span> i_vopo &lt; <span class="s">0</span> {
                <span class="k">break</span>;
            }

            <span class="k">let</span> p = &amp;<span class="k">self</span>.vertices[i_vopo <span class="k">as</span> usize];
            <span class="k">let</span> o = <span class="k">Self</span>::orient2d(ax, ay, bx, by, p.x, p.y);

            <span class="k">if</span> o &lt; <span class="s">0</span>.<span class="s">0</span> {
                <span class="k">if</span> i_vopo != v1 {
                    poly_r.push(i_vopo);
                }

                iv = ivr;
                ivr = i_vopo;
            } <span class="k">else</span> {
                <span class="k">if</span> i_vopo != v1 {
                    poly_l.push(i_vopo);
                }

                iv = ivl;
                ivl = i_vopo;
            }

            intersected.push(i_topo);
            cur_it = i_topo;
        }
    }

    <span class="c">/// Replace the crossed triangles by the two fans on either side of v0-v1 and constrain it.</span>
    <span class="k">fn</span> retriangulate(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        v0: i32,
        v1: i32,
        poly_l: &amp;[i32],
        poly_r: &amp;[i32],
        intersected: &amp;[i32],
    ) {
        <span class="k">for</span> &amp;ti <span class="k">in</span> intersected {
            <span class="k">self</span>.unregister_edges(ti);
            <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].alive = <span class="s">false</span>;
        }

        <span class="k">let</span> first_new = <span class="k">self</span>.triangles.len();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..poly_l.len().saturating_sub(<span class="s">2</span>) {
            <span class="k">self</span>.add_triangle(v1, poly_l[i + <span class="s">1</span>], poly_l[i]);
        }

        <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..poly_r.len().saturating_sub(<span class="s">1</span>) {
            <span class="k">self</span>.add_triangle(v0, poly_r[i], poly_r[i + <span class="s">1</span>]);
        }

        <span class="k">self</span>.inherit_constraints(first_new);
        <span class="k">self</span>.mark_edge(v0, v1);
    }

    <span class="c">/// Constrain every edge of a triangle from first_new on that its older neighbour holds constrained.</span>
    <span class="k">fn</span> inherit_constraints(&amp;<span class="k">mut</span> <span class="k">self</span>, first_new: usize) {
        <span class="k">for</span> new_ti <span class="k">in</span> first_new..<span class="k">self</span>.triangles.len() {
            <span class="k">if</span> !<span class="k">self</span>.triangles[new_ti].alive {
                <span class="k">continue</span>;
            }

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> nb = <span class="k">self</span>.triangles[new_ti].adj[k];

                <span class="k">if</span> nb &lt; <span class="s">0</span> || nb <span class="k">as</span> usize &gt;= first_new || !<span class="k">self</span>.triangles[nb <span class="k">as</span> usize].alive {
                    <span class="k">continue</span>;
                }

                <span class="k">for</span> kk <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                    <span class="k">if</span> <span class="k">self</span>.triangles[nb <span class="k">as</span> usize].adj[kk] == new_ti <span class="k">as</span> i32
                        &amp;&amp; <span class="k">self</span>.triangles[nb <span class="k">as</span> usize].constrained[kk]
                    {
                        <span class="k">self</span>.triangles[new_ti].constrained[k] = <span class="s">true</span>;
                        <span class="k">break</span>;
                    }
                }
            }
        }
    }

    <span class="c">/// Mark the edge v0-v1 constrained in every live triangle that has it.</span>
    <span class="k">fn</span> mark_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, v0: i32, v1: i32) {
        <span class="k">for</span> tri <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.triangles {
            <span class="k">if</span> !tri.alive {
                <span class="k">continue</span>;
            }

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> e0 = tri.v[(k + <span class="s">1</span>) % <span class="s">3</span>];
                <span class="k">let</span> e1 = tri.v[(k + <span class="s">2</span>) % <span class="s">3</span>];

                <span class="k">if</span> (e0 == v0 &amp;&amp; e1 == v1) || (e0 == v1 &amp;&amp; e1 == v0) {
                    tri.constrained[k] = <span class="s">true</span>;
                }
            }
        }
    }

    <span class="c">/// True when triangle ti has vertex v.</span>
    <span class="k">fn</span> has_vertex(&amp;<span class="k">self</span>, ti: i32, v: i32) -&gt; bool {
        <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[<span class="s">0</span>] == v
            || <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[<span class="s">1</span>] == v
            || <span class="k">self</span>.triangles[ti <span class="k">as</span> usize].v[<span class="s">2</span>] == v
    }

    <span class="c">/// New counter-clockwise triangle over three vertices; skipped when degenerate.</span>
    <span class="k">fn</span> add_triangle(&amp;<span class="k">mut</span> <span class="k">self</span>, pa: i32, pb: i32, pc: i32) {
        <span class="k">let</span> o = <span class="k">Self</span>::orient2d(
            <span class="k">self</span>.vertices[pa <span class="k">as</span> usize].x,
            <span class="k">self</span>.vertices[pa <span class="k">as</span> usize].y,
            <span class="k">self</span>.vertices[pb <span class="k">as</span> usize].x,
            <span class="k">self</span>.vertices[pb <span class="k">as</span> usize].y,
            <span class="k">self</span>.vertices[pc <span class="k">as</span> usize].x,
            <span class="k">self</span>.vertices[pc <span class="k">as</span> usize].y,
        );

        <span class="k">if</span> o.abs() &lt; <span class="s">1</span>e-<span class="s">20</span> {
            <span class="k">return</span>;
        }

        <span class="k">let</span> new_ti = <span class="k">self</span>.triangles.len() <span class="k">as</span> i32;
        <span class="k">let</span> (vb, vc) = <span class="k">if</span> o &gt; <span class="s">0</span>.<span class="s">0</span> { (pb, pc) } <span class="k">else</span> { (pc, pb) };
        <span class="k">self</span>.triangles.push(Triangle {
            v: [pa, vb, vc],
            adj: [-<span class="s">1</span>, -<span class="s">1</span>, -<span class="s">1</span>],
            constrained: [<span class="s">false</span>; <span class="s">3</span>],
            alive: <span class="s">true</span>,
        });
        <span class="k">self</span>.register_edges(new_ti);
    }

    <span class="c">/// Drop the triangles touching the super triangle.</span>
    <span class="k">fn</span> cleanup(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> sv = <span class="k">self</span>.super_v;

        <span class="k">for</span> ti <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() {
            <span class="k">if</span> !<span class="k">self</span>.triangles[ti].alive {
                <span class="k">continue</span>;
            }

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">if</span> <span class="k">self</span>.triangles[ti].v[k] == sv[<span class="s">0</span>]
                    || <span class="k">self</span>.triangles[ti].v[k] == sv[<span class="s">1</span>]
                    || <span class="k">self</span>.triangles[ti].v[k] == sv[<span class="s">2</span>]
                {
                    <span class="k">self</span>.unregister_edges(ti <span class="k">as</span> i32);
                    <span class="k">self</span>.triangles[ti].alive = <span class="s">false</span>;
                    <span class="k">break</span>;
                }
            }
        }

        <span class="k">self</span>.last_found = <span class="s">0</span>;

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.triangles.len() {
            <span class="k">if</span> <span class="k">self</span>.triangles[i].alive {
                <span class="k">self</span>.last_found = i <span class="k">as</span> i32;
                <span class="k">break</span>;
            }
        }
    }

    <span class="c">/// Vertex index triples of the live triangles.</span>
    <span class="k">fn</span> get_triangles(&amp;<span class="k">self</span>) -&gt; Vec&lt;[i32; 3]&gt; {
        <span class="k">let</span> <span class="k">mut</span> result = Vec::new();

        <span class="k">for</span> t <span class="k">in</span> &amp;<span class="k">self</span>.triangles {
            <span class="k">if</span> !t.alive {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> [a, b, c] = [t.v[<span class="s">0</span>], t.v[<span class="s">1</span>], t.v[<span class="s">2</span>]];
            <span class="k">let</span> o = <span class="k">Self</span>::orient2d(
                <span class="k">self</span>.vertices[a <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[a <span class="k">as</span> usize].y,
                <span class="k">self</span>.vertices[b <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[b <span class="k">as</span> usize].y,
                <span class="k">self</span>.vertices[c <span class="k">as</span> usize].x,
                <span class="k">self</span>.vertices[c <span class="k">as</span> usize].y,
            );
            result.push(<span class="k">if</span> o &gt; <span class="s">0</span>.<span class="s">0</span> { [a, b, c] } <span class="k">else</span> { [a, c, b] });
        }

        result
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Triangulation</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Loop polygon in UV before refinement: the control points of a polyline, else samples, the closing repeat dropped.</span>
<span class="k">fn</span> loop_points(crv: &amp;NurbsCurve) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> raw: Vec&lt;Point&gt; = Vec::new();

    <span class="k">if</span> crv.degree() &lt;= <span class="s">1</span> &amp;&amp; !crv.is_rational() {
        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..crv.cv_count() {
            raw.push(crv.get_cv(i).unwrap_or_default());
        }
    } <span class="k">else</span> {
        <span class="k">let</span> n = (crv.cv_count() * <span class="s">4</span>).clamp(<span class="s">16</span>, <span class="s">2048</span>);
        raw = crv.divide_by_count(n, <span class="s">true</span>).<span class="s">0</span>;
    }

    <span class="k">while</span> raw.len() &gt; <span class="s">1</span> {
        <span class="k">let</span> dx = raw[<span class="s">0</span>][<span class="s">0</span>] - raw[raw.len() - <span class="s">1</span>][<span class="s">0</span>];
        <span class="k">let</span> dy = raw[<span class="s">0</span>][<span class="s">1</span>] - raw[raw.len() - <span class="s">1</span>][<span class="s">1</span>];

        <span class="k">if</span> dx * dx + dy * dy &lt; <span class="s">1</span>e-<span class="s">20</span> {
            raw.pop();
        } <span class="k">else</span> {
            <span class="k">break</span>;
        }
    }

    raw
}

<span class="c">/// Append the UV points of the edge start-end without end, halved up to six times until each 3D chord is within deflection.</span>
<span class="k">fn</span> subdivide_edge(
    srf: &amp;NurbsSurface,
    start: &amp;Point,
    end: &amp;Point,
    deflection: f64,
    out: &amp;<span class="k">mut</span> Vec&lt;Point&gt;,
) {
    <span class="k">let</span> <span class="k">mut</span> stack: Vec&lt;(Point, Point, i32)&gt; = vec![(start.clone(), end.clone(), <span class="s">0</span>)];

    <span class="k">while</span> <span class="k">let</span> Some((a, b, depth)) = stack.pop() {
        <span class="k">let</span> mu = (a[<span class="s">0</span>] + b[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> mv = (a[<span class="s">1</span>] + b[<span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> pa = srf.point_at(a[<span class="s">0</span>], a[<span class="s">1</span>]).unwrap_or_default();
        <span class="k">let</span> pm = srf.point_at(mu, mv).unwrap_or_default();
        <span class="k">let</span> edge = &amp;srf.point_at(b[<span class="s">0</span>], b[<span class="s">1</span>]).unwrap_or_default() - &amp;pa;
        <span class="k">let</span> l2 = edge.magnitude_squared();
        <span class="k">let</span> dev = <span class="k">if</span> l2 &gt; <span class="s">1</span>e-<span class="s">30</span> {
            <span class="k">let</span> t = (&amp;pm - &amp;pa).dot(&amp;edge) / l2;
            (&amp;pm - &amp;(&amp;pa + &amp;(&amp;edge * t))).magnitude_squared().sqrt()
        } <span class="k">else</span> {
            (&amp;pm - &amp;pa).magnitude_squared().sqrt()
        };

        <span class="k">if</span> dev &gt; deflection &amp;&amp; depth &lt; <span class="s">6</span> {
            stack.push((Point::new(mu, mv, <span class="s">0</span>.<span class="s">0</span>), b, depth + <span class="s">1</span>));
            stack.push((a, Point::new(mu, mv, <span class="s">0</span>.<span class="s">0</span>), depth + <span class="s">1</span>));
        } <span class="k">else</span> {
            out.push(a);
        }
    }
}

<span class="c">/// Interior knots per direction whose multiplicity reaches the degree: the C0 lines of the surface.</span>
<span class="k">fn</span> find_crease_knots(surface: &amp;NurbsSurface) -&gt; [Vec&lt;f64&gt;; <span class="s">2</span>] {
    <span class="k">let</span> <span class="k">mut</span> crease_knots = [Vec::new(), Vec::new()];

    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">let</span> Some((start, end)) = surface.domain(dir) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> knots = &amp;surface.m_nurbsknot[dir];

        <span class="k">for</span> &amp;knot <span class="k">in</span> knots {
            <span class="k">if</span> knot &lt;= start || knot &gt;= end || crease_knots[dir].contains(&amp;knot) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> <span class="k">mut</span> multiplicity = <span class="s">0</span>;

            <span class="k">for</span> &amp;value <span class="k">in</span> knots {
                <span class="k">if</span> value == knot {
                    multiplicity += <span class="s">1</span>;
                }
            }

            <span class="k">if</span> multiplicity &gt;= surface.degree(dir) {
                crease_knots[dir].push(knot);
            }
        }
    }

    crease_knots
}

<span class="c">/// UV bounds (umin, vmin, umax, vmax) of a loop polygon.</span>
<span class="k">fn</span> loop_bounds(pts: &amp;[Point]) -&gt; [f64; <span class="s">4</span>] {
    <span class="k">let</span> <span class="k">mut</span> bounds = [<span class="s">1</span>e<span class="s">30_f64</span>, <span class="s">1</span>e<span class="s">30_f64</span>, -<span class="s">1</span>e<span class="s">30_f64</span>, -<span class="s">1</span>e<span class="s">30_f64</span>];

    <span class="k">for</span> p <span class="k">in</span> pts {
        <span class="k">if</span> p[<span class="s">0</span>] &lt; bounds[<span class="s">0</span>] {
            bounds[<span class="s">0</span>] = p[<span class="s">0</span>];
        }

        <span class="k">if</span> p[<span class="s">1</span>] &lt; bounds[<span class="s">1</span>] {
            bounds[<span class="s">1</span>] = p[<span class="s">1</span>];
        }

        <span class="k">if</span> p[<span class="s">0</span>] &gt; bounds[<span class="s">2</span>] {
            bounds[<span class="s">2</span>] = p[<span class="s">0</span>];
        }

        <span class="k">if</span> p[<span class="s">1</span>] &gt; bounds[<span class="s">3</span>] {
            bounds[<span class="s">3</span>] = p[<span class="s">1</span>];
        }
    }

    bounds
}

<span class="c">/// Constrain loop edge i in pieces cut where it crosses a crease knot line, each crossing inserted and recorded.</span>
<span class="k">fn</span> insert_loop_edge(
    dt: &amp;<span class="k">mut</span> Delaunay2D,
    pts: &amp;[Point],
    vis: &amp;[i32],
    li: usize,
    i: usize,
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    boundary_intervals: &amp;<span class="k">mut</span> BTreeMap&lt;usize, (usize, usize, f64)&gt;,
) {
    <span class="k">let</span> j = (i + <span class="s">1</span>) % vis.len();
    <span class="k">let</span> <span class="k">mut</span> events = vec![(<span class="s">0</span>.<span class="s">0</span>, vis[i]), (<span class="s">1</span>.<span class="s">0</span>, vis[j])];

    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">let</span> delta = pts[j][dir] - pts[i][dir];

        <span class="k">if</span> delta == <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">for</span> &amp;knot <span class="k">in</span> &amp;crease_knots[dir] {
            <span class="k">let</span> t = (knot - pts[i][dir]) / delta;

            <span class="k">if</span> t &lt;= <span class="s">0</span>.<span class="s">0</span> || t &gt;= <span class="s">1</span>.<span class="s">0</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> <span class="k">mut</span> uv = [
                pts[i][<span class="s">0</span>] + t * (pts[j][<span class="s">0</span>] - pts[i][<span class="s">0</span>]),
                pts[i][<span class="s">1</span>] + t * (pts[j][<span class="s">1</span>] - pts[i][<span class="s">1</span>]),
            ];
            uv[dir] = knot;
            <span class="k">let</span> vi = dt.insert(uv[<span class="s">0</span>], uv[<span class="s">1</span>]);

            <span class="k">if</span> vi &gt;= <span class="s">0</span> {
                boundary_intervals.insert(vi <span class="k">as</span> usize, (li, i, t));
            }

            events.push((t, vi));
        }
    }

    events.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

    <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..events.len() {
        <span class="k">if</span> events[k - <span class="s">1</span>].<span class="s">1</span> &gt;= <span class="s">0</span> &amp;&amp; events[k].<span class="s">1</span> &gt;= <span class="s">0</span> &amp;&amp; events[k - <span class="s">1</span>].<span class="s">1</span> != events[k].<span class="s">1</span> {
            dt.insert_constraint(events[k - <span class="s">1</span>].<span class="s">1</span>, events[k].<span class="s">1</span>);
        }
    }
}

<span class="c">/// Insert each loop's vertices, then constrain its edges; the vertex index of every loop sample.</span>
<span class="k">fn</span> insert_loops(
    dt: &amp;<span class="k">mut</span> Delaunay2D,
    loops_uv: &amp;[Vec&lt;Point&gt;],
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    boundary_intervals: &amp;<span class="k">mut</span> BTreeMap&lt;usize, (usize, usize, f64)&gt;,
) -&gt; Vec&lt;Vec&lt;i32&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> loop_vids: Vec&lt;Vec&lt;i32&gt;&gt; = Vec::new();

    <span class="k">for</span> (li, pts) <span class="k">in</span> loops_uv.iter().enumerate() {
        <span class="k">let</span> <span class="k">mut</span> vis: Vec&lt;i32&gt; = Vec::new();

        <span class="k">for</span> p <span class="k">in</span> pts {
            vis.push(dt.insert(p[<span class="s">0</span>], p[<span class="s">1</span>]));
        }

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..vis.len() {
            insert_loop_edge(dt, pts, &amp;vis, li, i, crease_knots, boundary_intervals);
        }

        loop_vids.push(vis);
    }

    loop_vids
}

<span class="c">/// Insert the crease knot crossings inside the loops and constrain each knot line between consecutive vertices on it.</span>
<span class="k">fn</span> insert_crease_lines(
    dt: &amp;<span class="k">mut</span> Delaunay2D,
    loops_uv: &amp;[Vec&lt;Point&gt;],
    bounds: &amp;[f64; <span class="s">4</span>],
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
) {
    <span class="k">for</span> &amp;u <span class="k">in</span> &amp;crease_knots[<span class="s">0</span>] {
        <span class="k">for</span> &amp;v <span class="k">in</span> &amp;crease_knots[<span class="s">1</span>] {
            <span class="k">if</span> inside_loops(u, v, loops_uv, bounds) {
                dt.insert(u, v);
            }
        }
    }

    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">for</span> &amp;knot <span class="k">in</span> &amp;crease_knots[dir] {
            <span class="k">let</span> <span class="k">mut</span> nodes = Vec::new();

            <span class="k">for</span> (vi, vertex) <span class="k">in</span> dt.vertices.iter().enumerate() {
                <span class="k">let</span> uv = [vertex.x, vertex.y];

                <span class="k">if</span> uv[dir] == knot {
                    nodes.push((uv[<span class="s">1</span> - dir], vi <span class="k">as</span> i32));
                }
            }

            nodes.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

            <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..nodes.len() {
                <span class="k">let</span> <span class="k">mut</span> uv = [knot, knot];
                uv[<span class="s">1</span> - dir] = (nodes[k - <span class="s">1</span>].<span class="s">0</span> + nodes[k].<span class="s">0</span>) * <span class="s">0</span>.<span class="s">5</span>;

                <span class="k">if</span> inside_loops(uv[<span class="s">0</span>], uv[<span class="s">1</span>], loops_uv, bounds) {
                    dt.insert_constraint(nodes[k - <span class="s">1</span>].<span class="s">1</span>, nodes[k].<span class="s">1</span>);
                }
            }
        }
    }
}

<span class="c">/// Smallest dot product between the crease-side normals at the corners of triangle abc around its centroid.</span>
<span class="k">fn</span> min_normal_dot(
    surface: &amp;NurbsSurface,
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    center: [f64; <span class="s">2</span>],
    a: &amp;Vertex2D,
    b: &amp;Vertex2D,
    c: &amp;Vertex2D,
) -&gt; f64 {
    <span class="k">let</span> na = crease_side_normal(surface, crease_knots, center, [a.x, a.y]);
    <span class="k">let</span> nb = crease_side_normal(surface, crease_knots, center, [b.x, b.y]);
    <span class="k">let</span> nc2 = crease_side_normal(surface, crease_knots, center, [c.x, c.y]);
    <span class="k">let</span> d1 = na.dot(&amp;nb);
    <span class="k">let</span> d2 = nb.dot(&amp;nc2);
    <span class="k">let</span> d3 = na.dot(&amp;nc2);

    d1.min(d2.min(d3))
}

<span class="c">/// Centroids of the live triangles inside the loops whose chord leaves deflection or whose corner normals turn past the angle bound.</span>
<span class="k">fn</span> refinement_points(
    dt: &amp;Delaunay2D,
    surface: &amp;NurbsSurface,
    loops_uv: &amp;[Vec&lt;Point&gt;],
    bounds: &amp;[f64; <span class="s">4</span>],
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    deflection: f64,
    cos_max_angle: f64,
) -&gt; Vec&lt;[f64; 2]&gt; {
    <span class="k">let</span> <span class="k">mut</span> to_insert: Vec&lt;[f64; 2]&gt; = Vec::new();

    <span class="k">for</span> tri <span class="k">in</span> &amp;dt.triangles {
        <span class="k">if</span> !tri.alive {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> a = &amp;dt.vertices[tri.v[<span class="s">0</span>] <span class="k">as</span> usize];
        <span class="k">let</span> b = &amp;dt.vertices[tri.v[<span class="s">1</span>] <span class="k">as</span> usize];
        <span class="k">let</span> c = &amp;dt.vertices[tri.v[<span class="s">2</span>] <span class="k">as</span> usize];
        <span class="k">let</span> cu = (a.x + b.x + c.x) / <span class="s">3</span>.<span class="s">0</span>;
        <span class="k">let</span> cv = (a.y + b.y + c.y) / <span class="s">3</span>.<span class="s">0</span>;

        <span class="k">if</span> !inside_loops(cu, cv, loops_uv, bounds) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> pa = surface.point_at(a.x, a.y).unwrap_or_default();
        <span class="k">let</span> pb = surface.point_at(b.x, b.y).unwrap_or_default();
        <span class="k">let</span> pc = surface.point_at(c.x, c.y).unwrap_or_default();
        <span class="k">let</span> pm = surface.point_at(cu, cv).unwrap_or_default();
        <span class="k">let</span> n = (&amp;pb - &amp;pa).cross(&amp;(&amp;pc - &amp;pa));
        <span class="k">let</span> nl = n.magnitude_squared().sqrt();

        <span class="k">if</span> nl &lt; <span class="s">1</span>e-<span class="s">30</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> dev = ((&amp;pm - &amp;pa).dot(&amp;n) / nl).abs();

        <span class="k">if</span> dev &gt; deflection
            || min_normal_dot(surface, crease_knots, [cu, cv], a, b, c) &lt; cos_max_angle
        {
            to_insert.push([cu, cv]);
        }
    }

    to_insert
}

<span class="c">/// Insert refinement centroids for up to eight rounds, until none is needed or the vertex cap is hit.</span>
<span class="k">fn</span> refine(
    dt: &amp;<span class="k">mut</span> Delaunay2D,
    surface: &amp;NurbsSurface,
    loops_uv: &amp;[Vec&lt;Point&gt;],
    bounds: &amp;[f64; <span class="s">4</span>],
    crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>],
    deflection: f64,
    cos_max_angle: f64,
) {
    <span class="k">const</span> MAX_ITERS: i32 = <span class="s">8</span>;
    <span class="k">const</span> MAX_VERTS: usize = <span class="s">200000</span>;

    <span class="k">for</span> _iter <span class="k">in</span> <span class="s">0</span>..MAX_ITERS {
        <span class="k">let</span> to_insert = refinement_points(
            dt,
            surface,
            loops_uv,
            bounds,
            crease_knots,
            deflection,
            cos_max_angle,
        );

        <span class="k">if</span> to_insert.is_empty() {
            <span class="k">break</span>;
        }

        <span class="k">for</span> uv <span class="k">in</span> &amp;to_insert {
            <span class="k">if</span> dt.vertices.len() &gt;= MAX_VERTS {
                <span class="k">break</span>;
            }

            dt.insert(uv[<span class="s">0</span>], uv[<span class="s">1</span>]);
        }

        <span class="k">if</span> dt.vertices.len() &gt;= MAX_VERTS {
            <span class="k">break</span>;
        }
    }
}

<span class="c">/// Drop the super triangle and every triangle whose centroid lies outside the loops.</span>
<span class="k">fn</span> trim_outside(dt: &amp;<span class="k">mut</span> Delaunay2D, loops_uv: &amp;[Vec&lt;Point&gt;], bounds: &amp;[f64; <span class="s">4</span>]) {
    dt.cleanup();

    <span class="k">for</span> ti <span class="k">in</span> <span class="s">0</span>..dt.triangles.len() {
        <span class="k">if</span> !dt.triangles[ti].alive {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> [v0, v1, v2] = dt.triangles[ti].v;
        <span class="k">let</span> cu =
            (dt.vertices[v0 <span class="k">as</span> usize].x + dt.vertices[v1 <span class="k">as</span> usize].x + dt.vertices[v2 <span class="k">as</span> usize].x)
                / <span class="s">3</span>.<span class="s">0</span>;
        <span class="k">let</span> cv =
            (dt.vertices[v0 <span class="k">as</span> usize].y + dt.vertices[v1 <span class="k">as</span> usize].y + dt.vertices[v2 <span class="k">as</span> usize].y)
                / <span class="s">3</span>.<span class="s">0</span>;

        <span class="k">if</span> !inside_loops(cu, cv, loops_uv, bounds) {
            dt.triangles[ti].alive = <span class="s">false</span>;
        }
    }
}

<span class="c">/// True when a triangle spans a crease knot line in either direction.</span>
<span class="k">fn</span> crosses_crease(tris: &amp;[[i32; <span class="s">3</span>]], dt: &amp;Delaunay2D, crease_knots: &amp;[Vec&lt;f64&gt;; <span class="s">2</span>]) -&gt; bool {
    <span class="k">for</span> tri <span class="k">in</span> tris {
        <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
            <span class="k">let</span> <span class="k">mut</span> low = f64::INFINITY;
            <span class="k">let</span> <span class="k">mut</span> high = f64::NEG_INFINITY;

            <span class="k">for</span> &amp;vi <span class="k">in</span> tri {
                <span class="k">let</span> value = [dt.vertices[vi <span class="k">as</span> usize].x, dt.vertices[vi <span class="k">as</span> usize].y][dir];
                low = low.min(value);
                high = high.max(value);
            }

            <span class="k">for</span> &amp;knot <span class="k">in</span> &amp;crease_knots[dir] {
                <span class="k">if</span> low &lt; knot &amp;&amp; knot &lt; high {
                    <span class="k">return</span> <span class="s">true</span>;
                }
            }
        }
    }

    <span class="s">false</span>
}

<span class="c">/// Loop and sample of the 3D point given for each triangulation vertex, None where none is.</span>
<span class="k">fn</span> given_points(
    count: usize,
    loops: &amp;TrimLoops,
    loop_vids: &amp;[Vec&lt;i32&gt;],
) -&gt; Vec&lt;Option&lt;(usize, usize)&gt;&gt; {
    <span class="k">let</span> <span class="k">mut</span> given: Vec&lt;Option&lt;(usize, usize)&gt;&gt; = vec![None; count];

    <span class="k">for</span> (li, vids) <span class="k">in</span> loop_vids.iter().enumerate() {
        <span class="k">if</span> li &gt;= loops.xyz.len() {
            <span class="k">break</span>;
        }

        <span class="k">for</span> (k, &amp;vi) <span class="k">in</span> vids.iter().enumerate() {
            <span class="k">if</span> vi &gt;= <span class="s">0</span> &amp;&amp; k &lt; loops.xyz[li].len() {
                given[vi <span class="k">as</span> usize] = Some((li, k));
            }
        }
    }

    given
}

<span class="c">/// 3D point of triangulation vertex vi: its given loop point, the loop chord at a knot crossing, else the surface point.</span>
<span class="k">fn</span> vertex_point(
    surface: &amp;NurbsSurface,
    dt: &amp;Delaunay2D,
    vi: usize,
    loops: &amp;TrimLoops,
    given: &amp;[Option&lt;(usize, usize)&gt;],
    boundary_intervals: &amp;BTreeMap&lt;usize, (usize, usize, f64)&gt;,
) -&gt; Point {
    <span class="k">if</span> <span class="k">let</span> Some((li, k)) = given[vi] {
        <span class="k">return</span> loops.xyz[li][k].clone();
    }

    <span class="k">if</span> <span class="k">let</span> Some(&amp;(li, segment, t)) = boundary_intervals.get(&amp;vi) {
        <span class="k">if</span> !loops.xyz.is_empty() {
            <span class="k">let</span> a = &amp;loops.xyz[li][segment];
            <span class="k">let</span> b = &amp;loops.xyz[li][(segment + <span class="s">1</span>) % loops.xyz[li].len()];

            <span class="k">return</span> a + &amp;(&amp;(b - a) * t);
        }
    }

    surface
        .point_at(dt.vertices[vi].x, dt.vertices[vi].y)
        .unwrap_or_default()
}

<span class="c">/// Welded mesh vertex of every triangulation vertex a triangle uses, None for the others.</span>
<span class="k">fn</span> weld_vertices(
    welder: &amp;<span class="k">mut</span> VertexWelder,
    mesh: &amp;<span class="k">mut</span> Mesh,
    surface: &amp;NurbsSurface,
    dt: &amp;Delaunay2D,
    tris: &amp;[[i32; <span class="s">3</span>]],
    loops: &amp;TrimLoops,
    loop_vids: &amp;[Vec&lt;i32&gt;],
    boundary_intervals: &amp;BTreeMap&lt;usize, (usize, usize, f64)&gt;,
) -&gt; Vec&lt;Option&lt;usize&gt;&gt; {
    <span class="k">let</span> given = given_points(dt.vertices.len(), loops, loop_vids);
    <span class="k">let</span> <span class="k">mut</span> vert_map: Vec&lt;Option&lt;usize&gt;&gt; = vec![None; dt.vertices.len()];

    <span class="k">for</span> tri <span class="k">in</span> tris {
        <span class="k">for</span> &amp;vi <span class="k">in</span> tri {
            <span class="k">if</span> vert_map[vi <span class="k">as</span> usize].is_none() {
                <span class="k">let</span> p = vertex_point(surface, dt, vi <span class="k">as</span> usize, loops, &amp;given, boundary_intervals);
                vert_map[vi <span class="k">as</span> usize] = Some(welder.weld(mesh, p));
            }
        }
    }

    vert_map
}

<span class="c">/// One face per triangle over its welded vertices, collapsed ones skipped.</span>
<span class="k">fn</span> add_faces(mesh: &amp;<span class="k">mut</span> Mesh, tris: &amp;[[i32; <span class="s">3</span>]], vert_map: &amp;[Option&lt;usize&gt;]) {
    <span class="k">for</span> &amp;[a, b, c] <span class="k">in</span> tris {
        <span class="k">let</span> (Some(v0), Some(v1), Some(v2)) = (
            vert_map[a <span class="k">as</span> usize],
            vert_map[b <span class="k">as</span> usize],
            vert_map[c <span class="k">as</span> usize],
        ) <span class="k">else</span> {
            <span class="k">continue</span>;
        };

        <span class="k">if</span> v0 == v1 || v1 == v2 || v2 == v0 {
            <span class="k">continue</span>;
        }

        mesh.add_face(vec![v0, v1, v2], None);
    }
}

<span class="c">/// Area-weighted sum of the face normals around each mesh vertex.</span>
<span class="k">fn</span> fan_normals(mesh: &amp;Mesh) -&gt; HashMap&lt;usize, Vector&gt; {
    <span class="k">let</span> <span class="k">mut</span> fan: HashMap&lt;usize, Vector&gt; = HashMap::new();
    <span class="k">let</span> <span class="k">mut</span> fkeys: Vec&lt;usize&gt; = Vec::new();

    <span class="k">for</span> &amp;fk <span class="k">in</span> mesh.face.keys() {
        fkeys.push(fk);
    }

    fkeys.sort_unstable();

    <span class="k">for</span> fk <span class="k">in</span> fkeys {
        <span class="k">let</span> verts = &amp;mesh.face[&amp;fk];
        <span class="k">let</span> a = mesh.vertex[&amp;verts[<span class="s">0</span>]].position();
        <span class="k">let</span> b = mesh.vertex[&amp;verts[<span class="s">1</span>]].position();
        <span class="k">let</span> c = mesh.vertex[&amp;verts[<span class="s">2</span>]].position();
        <span class="k">let</span> n = (&amp;b - &amp;a).cross(&amp;(&amp;c - &amp;a));

        <span class="k">for</span> &amp;vk <span class="k">in</span> verts {
            *fan.entry(vk).or_insert(Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)) += &amp;n;
        }
    }

    fan
}

<span class="c">/// Normal of every used vertex from the surface derivatives, the fan normal where they degenerate, and its u and v.</span>
<span class="k">fn</span> set_vertex_normals(
    mesh: &amp;<span class="k">mut</span> Mesh,
    surface: &amp;NurbsSurface,
    dt: &amp;Delaunay2D,
    vert_map: &amp;[Option&lt;usize&gt;],
) {
    <span class="k">let</span> fan = fan_normals(mesh);

    <span class="k">for</span> vi <span class="k">in</span> <span class="s">0</span>..vert_map.len() {
        <span class="k">let</span> Some(vk) = vert_map[vi] <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> u = dt.vertices[vi].x;
        <span class="k">let</span> v = dt.vertices[vi].y;
        <span class="k">let</span> derivatives = surface.evaluate(u, v, <span class="s">1</span>);
        <span class="k">let</span> <span class="k">mut</span> nrm = Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);

        <span class="k">if</span> derivatives.len() &gt;= <span class="s">3</span> {
            nrm = derivatives[<span class="s">2</span>].cross(&amp;derivatives[<span class="s">1</span>]);
        }

        <span class="k">let</span> nl = nrm.magnitude_squared().sqrt();

        <span class="k">if</span> nl.is_finite() &amp;&amp; nl &gt; <span class="s">0</span>.<span class="s">0</span> {
            nrm = &amp;nrm / nl;
        } <span class="k">else</span> {
            <span class="k">let</span> f = fan.get(&amp;vk).cloned().unwrap_or(Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
            <span class="k">let</span> fl = f.magnitude_squared().sqrt();
            nrm = <span class="k">if</span> fl.is_finite() &amp;&amp; fl &gt; <span class="s">0</span>.<span class="s">0</span> {
                &amp;f / fl
            } <span class="k">else</span> {
                Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)
            };
        }

        <span class="k">if</span> <span class="k">let</span> Some(vd) = mesh.vertex.get_mut(&amp;vk) {
            vd.set_normal(nrm[<span class="s">0</span>], nrm[<span class="s">1</span>], nrm[<span class="s">2</span>]);
            vd.attributes.insert(&quot;<span class="s">u</span>&quot;.to_string(), u);
            vd.attributes.insert(&quot;<span class="s">v</span>&quot;.to_string(), v);
        }
    }
}

<span class="c">/// Tag loop vertices boundary/{loop}/{sample} and knot crossings boundary_interval/{loop}/{segment} with their chord parameter.</span>
<span class="k">fn</span> tag_boundary(
    mesh: &amp;<span class="k">mut</span> Mesh,
    loop_vids: &amp;[Vec&lt;i32&gt;],
    boundary_intervals: &amp;BTreeMap&lt;usize, (usize, usize, f64)&gt;,
    vert_map: &amp;[Option&lt;usize&gt;],
) {
    <span class="k">for</span> (li, vids) <span class="k">in</span> loop_vids.iter().enumerate() {
        <span class="k">for</span> (k, &amp;vi) <span class="k">in</span> vids.iter().enumerate() {
            <span class="k">if</span> vi &lt; <span class="s">0</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> key = format!(&quot;<span class="s">boundary/</span>{<span class="s">li</span>}<span class="s">/</span>{<span class="s">k</span>}&quot;);

            <span class="k">if</span> <span class="k">let</span> Some(vk) = vert_map[vi <span class="k">as</span> usize] {
                <span class="k">if</span> <span class="k">let</span> Some(vd) = mesh.vertex.get_mut(&amp;vk) {
                    vd.attributes.insert(key, <span class="s">1</span>.<span class="s">0</span>);
                }
            }
        }
    }

    <span class="k">for</span> (&amp;vi, &amp;(li, segment, t)) <span class="k">in</span> boundary_intervals {
        <span class="k">let</span> key = format!(&quot;<span class="s">boundary_interval/</span>{<span class="s">li</span>}<span class="s">/</span>{<span class="s">segment</span>}&quot;);

        <span class="k">if</span> <span class="k">let</span> Some(vk) = vert_map[vi] {
            <span class="k">if</span> <span class="k">let</span> Some(vd) = mesh.vertex.get_mut(&amp;vk) {
                vd.attributes.insert(key, t);
            }
        }
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// TrimLoops</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Trim wires of one face as UV polygons, optional 3D points per loop vertex shared bit for bit with the neighbouring face, and interior UV seeds.</span>
#[derive(Debug, Clone, Default)]
<span class="k">pub</span> <span class="k">struct</span> TrimLoops {
    <span class="k">pub</span> uv: Vec&lt;Vec&lt;Point&gt;&gt;,     <span class="c">// UV polygon per loop.</span>
    <span class="k">pub</span> xyz: Vec&lt;Vec&lt;Point&gt;&gt;,    <span class="c">// 3D point per loop vertex, empty when not shared.</span>
    <span class="k">pub</span> interior_uv: Vec&lt;Point&gt;, <span class="c">// UV seeds inside the face.</span>
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// NurbsSurfaceTrimmed</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// A NURBS surface bounded by a closed outer loop and optional inner loops in its UV space.</span>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = &quot;<span class="s">type</span>&quot;, rename = &quot;<span class="s">NurbsSurfaceTrimmed</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> NurbsSurfaceTrimmed {
    #[serde(
        serialize_with = &quot;<span class="s">crate::guid_serde::serialize</span>&quot;,
        deserialize_with = &quot;<span class="s">crate::guid_serde::deserialize</span>&quot;
    )]
    guid: std::sync::OnceLock&lt;String&gt;, <span class="c">// Lazily minted GUID.</span>
    <span class="k">pub</span> name: String,        <span class="c">// Face name.</span>
    <span class="k">pub</span> width: f64,          <span class="c">// Display width.</span>
    <span class="k">pub</span> surfacecolor: Color, <span class="c">// Display color of the surface.</span>
    #[serde(rename = &quot;<span class="s">surface</span>&quot;)]
    <span class="k">pub</span> m_surface: NurbsSurface, <span class="c">// Underlying surface.</span>
    #[serde(rename = &quot;<span class="s">outer_loop</span>&quot;)]
    #[serde(skip_serializing_if = &quot;<span class="s">Option::is_none</span>&quot;)]
    #[serde(default)]
    <span class="k">pub</span> m_outer_loop: Option&lt;NurbsCurve&gt;, <span class="c">// Closed outer loop in UV space.</span>
    #[serde(rename = &quot;<span class="s">inner_loops</span>&quot;)]
    #[serde(default)]
    <span class="k">pub</span> m_inner_loops: Vec&lt;NurbsCurve&gt;, <span class="c">// Closed hole loops in UV space.</span>
    <span class="c">// SESSION_VIEWER</span>
    #[serde(default, skip_serializing_if = &quot;<span class="s">Option::is_none</span>&quot;)]
    <span class="k">pub</span> cut_q0: Option&lt;Point&gt;,
    #[serde(default, skip_serializing_if = &quot;<span class="s">Option::is_none</span>&quot;)]
    <span class="k">pub</span> cut_n: Option&lt;Vector&gt;,
    #[serde(default, skip_serializing_if = &quot;<span class="s">Vec::is_empty</span>&quot;)]
    <span class="k">pub</span> cut_planes: Vec&lt;(Point, Vector)&gt;,
}

<span class="k">impl</span> Default <span class="k">for</span> NurbsSurfaceTrimmed {
    <span class="c">/// Construct an empty untrimmed face.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}

<span class="k">impl</span> NurbsSurfaceTrimmed {
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Constructors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Construct an empty untrimmed face.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        NurbsSurfaceTrimmed {
            guid: std::sync::OnceLock::new(),
            name: &quot;<span class="s">my_nurbssurface_trimmed</span>&quot;.to_string(),
            width: <span class="s">1</span>.<span class="s">0</span>,
            surfacecolor: Color::black(),
            m_surface: NurbsSurface::default(),
            m_outer_loop: None,
            m_inner_loops: Vec::new(),
            cut_q0: None,
            cut_n: None,
            cut_planes: Vec::new(),
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
    <span class="c">/// Surface with a closed outer loop given in its UV parameter space.</span>
    <span class="k">pub</span> <span class="k">fn</span> create(surface: &amp;NurbsSurface, outer_loop: &amp;NurbsCurve) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> ts = <span class="k">Self</span>::new();
        ts.m_surface = surface.duplicate();
        ts.m_outer_loop = Some(outer_loop.duplicate());

        ts
    }

    <span class="c">/// Planar surface fitted to a closed 3D boundary, the boundary projected as the outer loop.</span>
    <span class="k">pub</span> <span class="k">fn</span> create_planar(boundary: &amp;NurbsCurve) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> srf = Primitives::create_planar(boundary);

        <span class="k">if</span> !srf.is_valid() {
            <span class="k">return</span> <span class="k">Self</span>::new();
        }

        <span class="k">let</span> p00 = srf.get_cv(<span class="s">0</span>, <span class="s">0</span>).unwrap_or_default();
        <span class="k">let</span> u_axis = &amp;srf.get_cv(<span class="s">1</span>, <span class="s">0</span>).unwrap_or_default() - &amp;p00;
        <span class="k">let</span> v_axis = &amp;srf.get_cv(<span class="s">0</span>, <span class="s">1</span>).unwrap_or_default() - &amp;p00;
        <span class="k">let</span> u_len2 = u_axis.magnitude_squared();
        <span class="k">let</span> v_len2 = v_axis.magnitude_squared();

        <span class="k">if</span> u_len2 &lt; <span class="s">1</span>e-<span class="s">28</span> || v_len2 &lt; <span class="s">1</span>e-<span class="s">28</span> {
            <span class="k">return</span> <span class="k">Self</span>::new();
        }

        <span class="k">let</span> <span class="k">mut</span> uv_pts: Vec&lt;Point&gt; = Vec::new();

        <span class="k">if</span> boundary.degree() &lt;= <span class="s">1</span> {
            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..boundary.cv_count() {
                uv_pts.push(project_to_uv(
                    &amp;boundary.get_cv(i).unwrap_or_default(),
                    &amp;p00,
                    &amp;u_axis,
                    &amp;v_axis,
                    u_len2,
                    v_len2,
                ));
            }
        } <span class="k">else</span> {
            <span class="k">let</span> spans = boundary.get_span_vector();
            <span class="k">let</span> n_sub = <span class="s">10</span>;

            <span class="k">for</span> si <span class="k">in</span> <span class="s">0</span>..spans.len().saturating_sub(<span class="s">1</span>) {
                <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..=n_sub {
                    <span class="k">let</span> t = spans[si] + (spans[si + <span class="s">1</span>] - spans[si]) * k <span class="k">as</span> f64 / n_sub <span class="k">as</span> f64;
                    <span class="k">let</span> uv = project_to_uv(
                        &amp;boundary.point_at(t),
                        &amp;p00,
                        &amp;u_axis,
                        &amp;v_axis,
                        u_len2,
                        v_len2,
                    );

                    <span class="k">if</span> uv_pts.is_empty()
                        || (&amp;uv - &amp;uv_pts[uv_pts.len() - <span class="s">1</span>]).magnitude_squared() &gt; <span class="s">1</span>e-<span class="s">24</span>
                    {
                        uv_pts.push(uv);
                    }
                }
            }
        }

        <span class="k">let</span> <span class="k">mut</span> ts = <span class="k">Self</span>::new();
        ts.m_surface = srf;

        <span class="k">if</span> uv_pts.len() &gt;= <span class="s">3</span> {
            ts.m_outer_loop = Some(NurbsCurve::create(<span class="s">false</span>, <span class="s">1</span>, &amp;uv_pts));
        }

        ts
    }

    <span class="c">/// One trimmed face per region of the UV domain carved by the pcurves (x=u, y=v, z=0); dangling cutters are discarded.</span>
    <span class="k">pub</span> <span class="k">fn</span> split_by_uv_curves(
        srf: &amp;NurbsSurface,
        pcurves: &amp;[NurbsCurve],
        tolerance: f64,
    ) -&gt; Vec&lt;NurbsSurfaceTrimmed&gt; {
        <span class="k">if</span> !srf.is_valid() {
            <span class="k">return</span> Vec::new();
        }

        <span class="k">let</span> dom = split_domain(srf, tolerance);
        <span class="k">let</span> polylines = uv_polylines(pcurves, &amp;dom);
        <span class="k">let</span> <span class="k">mut</span> pool = UVVertexPool::new(dom.snap);
        <span class="k">let</span> splits = polyline_crossings(&amp;polylines, pcurves, &amp;dom);
        <span class="k">let</span> live_edges = prune_dangling(&amp;split_edges(&amp;polylines, &amp;splits, &amp;<span class="k">mut</span> pool));

        <span class="k">if</span> live_edges.is_empty() {
            <span class="k">return</span> Vec::new();
        }

        <span class="k">let</span> verts = &amp;pool.verts;
        <span class="k">let</span> hes = half_edges(&amp;live_edges);
        <span class="k">let</span> faces = face_cycles(&amp;next_half_edges(&amp;hes, verts));
        <span class="k">let</span> (pos_faces, neg_faces) = classify_faces(faces, &amp;hes, verts, &amp;live_edges, dom.snap);
        <span class="k">let</span> holes_of = assign_holes(&amp;neg_faces, &amp;pos_faces, &amp;hes, verts);
        <span class="k">let</span> <span class="k">mut</span> result: Vec&lt;NurbsSurfaceTrimmed&gt; = Vec::new();

        <span class="k">for</span> (fi, (cycle, _area)) <span class="k">in</span> pos_faces.iter().enumerate() {
            <span class="k">let</span> <span class="k">mut</span> outer = cycle_to_loop(cycle, &amp;hes, &amp;live_edges, verts, pcurves, dom.snap);

            <span class="k">if</span> !outer.is_valid() || (loop_signed_area(&amp;outer) &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; !outer.reverse()) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> <span class="k">mut</span> ts = NurbsSurfaceTrimmed::create(srf, &amp;outer);

            <span class="k">for</span> hole_cycle <span class="k">in</span> &amp;holes_of[fi] {
                <span class="k">let</span> <span class="k">mut</span> hole =
                    cycle_to_loop(hole_cycle, &amp;hes, &amp;live_edges, verts, pcurves, dom.snap);

                <span class="k">if</span> !hole.is_valid() || (loop_signed_area(&amp;hole) &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; !hole.reverse()) {
                    <span class="k">continue</span>;
                }

                ts.add_inner_loop(hole);
            }

            result.push(ts);
        }

        result
    }

    <span class="c">/// One trimmed face per non-empty region carved by the planes (all 2^K sign combinations).</span>
    <span class="k">pub</span> <span class="k">fn</span> split_by_planes(
        srf: &amp;NurbsSurface,
        planes: &amp;[(Point, Vector)],
    ) -&gt; Vec&lt;NurbsSurfaceTrimmed&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
        <span class="k">let</span> k = planes.len();

        <span class="k">if</span> k == <span class="s">0</span> || k &gt; <span class="s">16</span> {
            <span class="k">return</span> out;
        }

        <span class="k">for</span> mask <span class="k">in</span> <span class="s">0u32</span>..(<span class="s">1u32</span> &lt;&lt; k) {
            <span class="k">let</span> <span class="k">mut</span> cp: Vec&lt;(Point, Vector)&gt; = Vec::new();

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..k {
                <span class="k">let</span> q = &amp;planes[i].<span class="s">0</span>;
                <span class="k">let</span> n = &amp;planes[i].<span class="s">1</span>;
                <span class="k">let</span> flip = ((mask &gt;&gt; i) &amp; <span class="s">1</span>) == <span class="s">1</span>;
                <span class="k">let</span> nn = <span class="k">if</span> flip {
                    Vector::new(-n[<span class="s">0</span>], -n[<span class="s">1</span>], -n[<span class="s">2</span>])
                } <span class="k">else</span> {
                    Vector::new(n[<span class="s">0</span>], n[<span class="s">1</span>], n[<span class="s">2</span>])
                };
                cp.push((q.clone(), nn));
            }

            <span class="k">let</span> <span class="k">mut</span> ts = NurbsSurfaceTrimmed::new();
            ts.m_surface = srf.duplicate();
            <span class="k">let</span> m = ts.mesh_by_planes(&amp;cp, <span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">01</span>);

            <span class="k">if</span> m.number_of_faces() &gt; <span class="s">0</span> {
                <span class="c">// SESSION_VIEWER</span>
                ts.cut_planes = cp;
                out.push(ts);
            }
        }

        out
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Transformation</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Transform the surface in place; the loops live in UV and stay.</span>
    <span class="k">pub</span> <span class="k">fn</span> transform(&amp;<span class="k">mut</span> <span class="k">self</span>, xform: &amp;Xform) {
        <span class="k">self</span>.m_surface.transform(xform);

        <span class="c">// SESSION_VIEWER</span>
        <span class="k">if</span> <span class="k">let</span> Some(q0) = <span class="k">self</span>.cut_q0.as_mut() {
            q0.transform(xform);
        }

        <span class="k">if</span> <span class="k">let</span> Some(n) = <span class="k">self</span>.cut_n.as_mut() {
            n.transform(xform);
        }

        <span class="k">for</span> (q, n) <span class="k">in</span> <span class="k">self</span>.cut_planes.iter_mut() {
            q.transform(xform);
            n.transform(xform);
        }
    }

    <span class="c">/// Return a transformed copy.</span>
    <span class="k">pub</span> <span class="k">fn</span> transformed(&amp;<span class="k">self</span>, xform: &amp;Xform) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> ts = <span class="k">self</span>.duplicate();
        ts.transform(xform);

        ts
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

    <span class="c">/// Return the underlying surface.</span>
    <span class="k">pub</span> <span class="k">fn</span> surface(&amp;<span class="k">self</span>) -&gt; &amp;NurbsSurface {
        &amp;<span class="k">self</span>.m_surface
    }

    <span class="c">/// Return the outer loop, None when untrimmed.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_outer_loop(&amp;<span class="k">self</span>) -&gt; Option&lt;&amp;NurbsCurve&gt; {
        <span class="k">self</span>.m_outer_loop.as_ref()
    }

    <span class="c">/// Replace the outer loop.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_outer_loop(&amp;<span class="k">mut</span> <span class="k">self</span>, loop_crv: NurbsCurve) {
        <span class="k">self</span>.m_outer_loop = Some(loop_crv);
    }

    <span class="c">/// Return whether the outer loop is a valid curve.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_trimmed(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.m_outer_loop.as_ref().is_some_and(|c| c.is_valid())
    }

    <span class="c">/// Return whether the underlying surface is valid.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_valid(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.m_surface.is_valid()
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Inner loops</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Hole given directly as a closed 2D curve in UV space.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_inner_loop(&amp;<span class="k">mut</span> <span class="k">self</span>, loop_2d: NurbsCurve) {
        <span class="k">self</span>.m_inner_loops.push(loop_2d);
    }

    <span class="c">/// Hole from a 3D curve pulled onto the surface and normalized into [0,1]^2.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_hole(&amp;<span class="k">mut</span> <span class="k">self</span>, curve_3d: &amp;NurbsCurve) {
        <span class="k">let</span> dom = curve_3d.domain();
        <span class="k">let</span> sdom_u = <span class="k">self</span>.m_surface.domain(<span class="s">0</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> sdom_v = <span class="k">self</span>.m_surface.domain(<span class="s">1</span>).unwrap_or((<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>));
        <span class="k">let</span> range_u = sdom_u.<span class="s">1</span> - sdom_u.<span class="s">0</span>;
        <span class="k">let</span> range_v = sdom_v.<span class="s">1</span> - sdom_v.<span class="s">0</span>;
        <span class="k">let</span> n_samples = (curve_3d.cv_count() * <span class="s">4</span>).clamp(<span class="s">32</span>, <span class="s">2048</span>);
        <span class="k">let</span> <span class="k">mut</span> uv_pts = Vec::new();

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n_samples {
            <span class="k">let</span> t = dom.<span class="s">0</span> + (dom.<span class="s">1</span> - dom.<span class="s">0</span>) * i <span class="k">as</span> f64 / n_samples <span class="k">as</span> f64;
            <span class="k">let</span> pt3d = curve_3d.point_at(t);
            <span class="k">let</span> (u, v, _dist) = Closest::surface_point(&amp;<span class="k">self</span>.m_surface, &amp;pt3d, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
            <span class="k">let</span> nu = (u - sdom_u.<span class="s">0</span>) / range_u;
            <span class="k">let</span> nv = (v - sdom_v.<span class="s">0</span>) / range_v;
            uv_pts.push(Point::new(nu, nv, <span class="s">0</span>.<span class="s">0</span>));
        }

        <span class="k">if</span> uv_pts.len() &gt;= <span class="s">3</span> {
            <span class="k">self</span>.m_inner_loops
                .push(NurbsCurve::create(<span class="s">true</span>, <span class="s">1</span>, &amp;uv_pts));
        }
    }

    <span class="c">/// Add one hole per 3D curve pulled onto the surface.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_holes(&amp;<span class="k">mut</span> <span class="k">self</span>, curves_3d: &amp;[NurbsCurve]) {
        <span class="k">for</span> crv <span class="k">in</span> curves_3d {
            <span class="k">self</span>.add_hole(crv);
        }
    }

    <span class="c">/// Return the inner loop at index.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_inner_loop(&amp;<span class="k">self</span>, index: usize) -&gt; &amp;NurbsCurve {
        &amp;<span class="k">self</span>.m_inner_loops[index]
    }

    <span class="c">/// Return the number of inner loops.</span>
    <span class="k">pub</span> <span class="k">fn</span> inner_loop_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.m_inner_loops.len()
    }

    <span class="c">/// Remove every inner loop.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear_inner_loops(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.m_inner_loops.clear();
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Evaluation</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return the surface point at (u, v), None when the surface is invalid.</span>
    <span class="k">pub</span> <span class="k">fn</span> point_at(&amp;<span class="k">self</span>, u: f64, v: f64) -&gt; Option&lt;Point&gt; {
        <span class="k">self</span>.m_surface.point_at(u, v)
    }

    <span class="c">/// Return the unit surface normal at (u, v).</span>
    <span class="k">pub</span> <span class="k">fn</span> normal_at(&amp;<span class="k">self</span>, u: f64, v: f64) -&gt; Vector {
        <span class="k">self</span>.m_surface.normal_at(u, v)
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Meshing</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return mesh_q at 20 degrees and a chord factor of 0.005.</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh(&amp;<span class="k">self</span>) -&gt; Mesh {
        <span class="k">self</span>.mesh_q(<span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">005</span>)
    }

    <span class="c">/// Deflection-refined constrained Delaunay of the trim loops: angular bound in degrees, chord factor as a fraction of the bbox diagonal.</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh_q(&amp;<span class="k">self</span>, max_angle_deg: f64, chord_factor: f64) -&gt; Mesh {
        <span class="k">if</span> !<span class="k">self</span>.is_trimmed() {
            <span class="k">return</span> <span class="k">self</span>.m_surface.mesh();
        }

        <span class="k">let</span> deflection = <span class="k">self</span>.bbox_diagonal() * chord_factor;

        <span class="k">let</span> <span class="k">mut</span> loops = TrimLoops::default();
        <span class="k">let</span> Some(outer) = <span class="k">self</span>.m_outer_loop.as_ref() <span class="k">else</span> {
            <span class="k">return</span> <span class="k">self</span>.m_surface.mesh();
        };
        loops.uv.push(<span class="k">self</span>.discretize_loop(outer, deflection));

        <span class="k">for</span> inner <span class="k">in</span> &amp;<span class="k">self</span>.m_inner_loops {
            loops.uv.push(<span class="k">self</span>.discretize_loop(inner, deflection));
        }

        <span class="k">self</span>.triangulate(&amp;loops, max_angle_deg, chord_factor)
    }

    <span class="c">/// Mesh sampled loops (outer first, then holes) keeping every loop vertex, tagged boundary/{loop}/{sample}; knot crossings add boundary_interval/{loop}/{segment}; empty mesh on invalid input.</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh_loops(&amp;<span class="k">self</span>, loops: &amp;TrimLoops, max_angle_deg: f64, chord_factor: f64) -&gt; Mesh {
        <span class="k">if</span> loops.uv.is_empty()
            || !max_angle_deg.is_finite()
            || max_angle_deg &lt;= <span class="s">0</span>.<span class="s">0</span>
            || !chord_factor.is_finite()
            || chord_factor &lt;= <span class="s">0</span>.<span class="s">0</span>
            || (!loops.xyz.is_empty() &amp;&amp; loops.xyz.len() != loops.uv.len())
        {
            <span class="k">return</span> Mesh::new();
        }

        <span class="k">let</span> <span class="k">mut</span> expected = <span class="s">0</span>;

        <span class="k">for</span> (li, points) <span class="k">in</span> loops.uv.iter().enumerate() {
            <span class="k">if</span> points.len() &lt; <span class="s">3</span> || (!loops.xyz.is_empty() &amp;&amp; loops.xyz[li].len() != points.len()) {
                <span class="k">return</span> Mesh::new();
            }

            <span class="k">for</span> point <span class="k">in</span> points {
                <span class="k">if</span> !point[<span class="s">0</span>].is_finite() || !point[<span class="s">1</span>].is_finite() {
                    <span class="k">return</span> Mesh::new();
                }
            }

            <span class="k">if</span> !loops.xyz.is_empty() {
                <span class="k">for</span> point <span class="k">in</span> &amp;loops.xyz[li] {
                    <span class="k">if</span> !point[<span class="s">0</span>].is_finite() || !point[<span class="s">1</span>].is_finite() || !point[<span class="s">2</span>].is_finite() {
                        <span class="k">return</span> Mesh::new();
                    }
                }
            }

            expected += points.len();
        }

        <span class="k">let</span> result = <span class="k">self</span>.triangulate(loops, max_angle_deg, chord_factor);
        <span class="k">let</span> <span class="k">mut</span> actual: HashSet&lt;String&gt; = HashSet::new();

        <span class="k">for</span> vd <span class="k">in</span> result.vertex.values() {
            <span class="k">for</span> name <span class="k">in</span> vd.attributes.keys() {
                <span class="k">if</span> name.starts_with(&quot;<span class="s">boundary/</span>&quot;) {
                    actual.insert(name.clone());
                }
            }
        }

        <span class="k">if</span> actual.len() == expected {
            result
        } <span class="k">else</span> {
            Mesh::new()
        }
    }

    <span class="c">/// Mesh of the half (S-q0).n &lt;= 0: span-adaptive grid, marching-squares clip with Newton-refined crossings, seams welded.</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh_by_plane(
        &amp;<span class="k">self</span>,
        q0: &amp;Point,
        normal: &amp;Vector,
        max_angle_deg: f64,
        chord_factor: f64,
    ) -&gt; Mesh {
        <span class="k">let</span> srf = &amp;<span class="k">self</span>.m_surface;
        <span class="k">let</span> Some(n) = unit3(normal) <span class="k">else</span> {
            <span class="k">return</span> srf.mesh();
        };
        <span class="k">let</span> q = [q0[<span class="s">0</span>], q0[<span class="s">1</span>], q0[<span class="s">2</span>]];
        <span class="k">let</span> bbox_diag = <span class="k">self</span>.bbox_diagonal();
        <span class="k">let</span> Some((us, vs)) = span_grid(srf, max_angle_deg, bbox_diag * chord_factor) <span class="k">else</span> {
            <span class="k">return</span> srf.mesh();
        };
        <span class="k">let</span> nu = us.len();
        <span class="k">let</span> nv = vs.len();
        <span class="k">let</span> <span class="k">mut</span> field = vec![vec![<span class="s">0</span>.<span class="s">0f64</span>; nv]; nu];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..nv {
                field[i][j] = plane_field(srf, &amp;q, &amp;n, us[i], vs[j]);
            }
        }

        <span class="k">let</span> <span class="k">mut</span> result = Mesh::new();
        <span class="k">let</span> weld_tol = bbox_diag * <span class="s">1</span>e-<span class="s">5</span>;
        <span class="k">let</span> <span class="k">mut</span> welder = VertexWelder::new(weld_tol, weld_tol);

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu - <span class="s">1</span> {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..nv - <span class="s">1</span> {
                <span class="k">let</span> cu = [us[i], us[i + <span class="s">1</span>], us[i + <span class="s">1</span>], us[i]];
                <span class="k">let</span> cv = [vs[j], vs[j], vs[j + <span class="s">1</span>], vs[j + <span class="s">1</span>]];
                <span class="k">let</span> fc = [
                    field[i][j],
                    field[i + <span class="s">1</span>][j],
                    field[i + <span class="s">1</span>][j + <span class="s">1</span>],
                    field[i][j + <span class="s">1</span>],
                ];
                <span class="k">let</span> poly = clip_cell(&amp;<span class="k">mut</span> welder, &amp;<span class="k">mut</span> result, srf, &amp;q, &amp;n, &amp;cu, &amp;cv, &amp;fc);
                add_fan(&amp;<span class="k">mut</span> result, &amp;poly);
            }
        }

        <span class="k">if</span> result.face.is_empty() {
            <span class="k">return</span> srf.mesh();
        }

        result
    }

    <span class="c">/// Mesh of the region inside every half-space (S-q).n &lt;= 0: triangle soup clipped plane by plane, seams welded.</span>
    <span class="k">pub</span> <span class="k">fn</span> mesh_by_planes(
        &amp;<span class="k">self</span>,
        planes: &amp;[(Point, Vector)],
        max_angle_deg: f64,
        chord_factor: f64,
    ) -&gt; Mesh {
        <span class="k">let</span> srf = &amp;<span class="k">self</span>.m_surface;
        <span class="k">let</span> <span class="k">mut</span> pl: Vec&lt;([f64; 3], [f64; 3])&gt; = Vec::new();

        <span class="k">for</span> (qn_q, qn_n) <span class="k">in</span> planes {
            <span class="k">let</span> Some(n) = unit3(qn_n) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            pl.push(([qn_q[<span class="s">0</span>], qn_q[<span class="s">1</span>], qn_q[<span class="s">2</span>]], n));
        }

        <span class="k">if</span> pl.is_empty() {
            <span class="k">return</span> srf.mesh();
        }

        <span class="k">let</span> bbox_diag = <span class="k">self</span>.bbox_diagonal();
        <span class="k">let</span> Some((us, vs)) = span_grid(srf, max_angle_deg, bbox_diag * chord_factor) <span class="k">else</span> {
            <span class="k">return</span> srf.mesh();
        };
        <span class="k">let</span> <span class="k">mut</span> tris = grid_triangles(&amp;us, &amp;vs);

        <span class="k">for</span> (q, n) <span class="k">in</span> &amp;pl {
            tris = clip_triangles(srf, q, n, &amp;tris);

            <span class="k">if</span> tris.is_empty() {
                <span class="k">break</span>;
            }
        }

        <span class="k">if</span> tris.is_empty() {
            <span class="k">return</span> Mesh::new();
        }

        <span class="k">let</span> result = weld_triangles(srf, &amp;tris, bbox_diag * <span class="s">1</span>e-<span class="s">5</span>);

        <span class="k">if</span> result.face.is_empty() {
            <span class="k">return</span> Mesh::new();
        }

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
        <span class="k">self</span>.jsondump()
            .expect(&quot;<span class="s">Failed to serialize NurbsSurfaceTrimmed JSON</span>&quot;)
    }

    <span class="c">/// Deserialize from a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_loads(json_string: &amp;str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::jsonload(json_string).expect(&quot;<span class="s">Failed to parse NurbsSurfaceTrimmed JSON</span>&quot;)
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
    <span class="k">pub</span> <span class="k">fn</span> to_proto(&amp;<span class="k">self</span>) -&gt; <span class="k">crate</span>::proto::NurbsSurfaceTrimmed {
        <span class="k">let</span> <span class="k">mut</span> outer_loop = None;

        <span class="k">if</span> <span class="k">self</span>.is_trimmed() {
            <span class="k">if</span> <span class="k">let</span> Some(outer) = <span class="k">self</span>.m_outer_loop.as_ref() {
                outer_loop = Some(outer.to_proto());
            }
        }

        <span class="k">let</span> <span class="k">mut</span> inner_loops = Vec::new();

        <span class="k">for</span> inner <span class="k">in</span> &amp;<span class="k">self</span>.m_inner_loops {
            inner_loops.push(inner.to_proto());
        }

        <span class="k">crate</span>::proto::NurbsSurfaceTrimmed {
            guid: <span class="k">self</span>.guid.get().cloned().unwrap_or_default(),
            name: <span class="k">self</span>.name.clone(),
            surface: Some(<span class="k">self</span>.m_surface.to_proto()),
            outer_loop,
            inner_loops,
            width: <span class="k">self</span>.width,
            surfacecolor: Some(<span class="k">self</span>.surfacecolor.to_proto()),
        }
    }

    <span class="c">/// Construct from the protobuf message.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_proto(
        proto: <span class="k">crate</span>::proto::NurbsSurfaceTrimmed,
    ) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">let</span> <span class="k">mut</span> ts = <span class="k">Self</span>::new();

        <span class="k">if</span> !proto.guid.is_empty() {
            ts.set_guid(proto.guid.clone());
        }

        ts.name = proto.name;
        ts.width = proto.width;

        <span class="k">if</span> <span class="k">let</span> Some(surface) = proto.surface {
            ts.m_surface = NurbsSurface::from_proto(surface)?;
        }

        <span class="k">if</span> <span class="k">let</span> Some(outer) = proto.outer_loop {
            ts.m_outer_loop = Some(NurbsCurve::from_proto(outer));
        }

        <span class="k">for</span> inner <span class="k">in</span> proto.inner_loops {
            ts.m_inner_loops.push(NurbsCurve::from_proto(inner));
        }

        ts.surfacecolor = Color::from_proto(proto.surfacecolor.unwrap_or_default());

        Ok(ts)
    }

    <span class="c">/// Serialize to protobuf bytes.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_dumps(&amp;<span class="k">self</span>) -&gt; Vec&lt;u8&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">self</span>.to_proto().encode_to_vec()
    }

    <span class="c">/// Deserialize from protobuf bytes.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_loads(data: &amp;[u8]) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">Self</span>::from_proto(<span class="k">crate</span>::proto::NurbsSurfaceTrimmed::decode(data)?)
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
    <span class="c">/// Return &quot;NurbsSurfaceTrimmed(name=..., trimmed=..., holes=...)&quot;.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">NurbsSurfaceTrimmed(name=</span>{}<span class="s">, trimmed=</span>{}<span class="s">, holes=</span>{}<span class="s">)</span>&quot;,
            <span class="k">self</span>.name,
            <span class="k">self</span>.is_trimmed(),
            <span class="k">self</span>.inner_loop_count()
        )
    }

    <span class="c">/// Return the multi-line form with the surface.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">NurbsSurfaceTrimmed(\\n  name=</span>{}<span class="s">,\\n  trimmed=</span>{}<span class="s">,\\n  holes=</span>{}<span class="s">,\\n  surface=</span>{}<span class="s">\\n)</span>&quot;,
            <span class="k">self</span>.name,
            <span class="k">self</span>.is_trimmed(),
            <span class="k">self</span>.inner_loop_count(),
            <span class="k">self</span>.m_surface.str()
        )
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Private helpers</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return the diagonal of the control-point box, the scale every deflection tolerance is a fraction of.</span>
    <span class="k">fn</span> bbox_diagonal(&amp;<span class="k">self</span>) -&gt; f64 {
        <span class="k">let</span> <span class="k">mut</span> bmin = [<span class="s">1</span>e<span class="s">30f64</span>; <span class="s">3</span>];
        <span class="k">let</span> <span class="k">mut</span> bmax = [-<span class="s">1</span>e<span class="s">30f64</span>; <span class="s">3</span>];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.m_surface.cv_count(<span class="s">0</span>) {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.m_surface.cv_count(<span class="s">1</span>) {
                <span class="k">let</span> p = <span class="k">self</span>.m_surface.get_cv(i, j).unwrap_or_default();

                <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                    <span class="k">if</span> p[k] &lt; bmin[k] {
                        bmin[k] = p[k];
                    }

                    <span class="k">if</span> p[k] &gt; bmax[k] {
                        bmax[k] = p[k];
                    }
                }
            }
        }

        <span class="k">let</span> bbox_diag = ((bmax[<span class="s">0</span>] - bmin[<span class="s">0</span>]) * (bmax[<span class="s">0</span>] - bmin[<span class="s">0</span>])
            + (bmax[<span class="s">1</span>] - bmin[<span class="s">1</span>]) * (bmax[<span class="s">1</span>] - bmin[<span class="s">1</span>])
            + (bmax[<span class="s">2</span>] - bmin[<span class="s">2</span>]) * (bmax[<span class="s">2</span>] - bmin[<span class="s">2</span>]))
            .sqrt();

        <span class="k">if</span> bbox_diag &lt; <span class="s">1</span>e-<span class="s">12</span> {
            <span class="s">1</span>.<span class="s">0</span>
        } <span class="k">else</span> {
            bbox_diag
        }
    }

    <span class="c">/// UV polygon of a trim loop: control points or samples, each edge split until its 3D chord is within deflection.</span>
    <span class="k">fn</span> discretize_loop(&amp;<span class="k">self</span>, crv: &amp;NurbsCurve, deflection: f64) -&gt; Vec&lt;Point&gt; {
        <span class="k">let</span> raw = loop_points(crv);

        <span class="k">if</span> raw.len() &lt; <span class="s">2</span> {
            <span class="k">return</span> raw;
        }

        <span class="k">let</span> <span class="k">mut</span> out: Vec&lt;Point&gt; = Vec::with_capacity(raw.len() * <span class="s">2</span>);

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..raw.len() {
            subdivide_edge(
                &amp;<span class="k">self</span>.m_surface,
                &amp;raw[i],
                &amp;raw[(i + <span class="s">1</span>) % raw.len()],
                deflection,
                &amp;<span class="k">mut</span> out,
            );
        }

        out
    }

    <span class="c">/// Constrained Delaunay of the loops in UV, refined, trimmed, lifted and welded: the one body mesh_q and mesh_loops share.</span>
    <span class="k">fn</span> triangulate(&amp;<span class="k">self</span>, loops: &amp;TrimLoops, max_angle_deg: f64, chord_factor: f64) -&gt; Mesh {
        <span class="k">if</span> loops.uv.is_empty() || loops.uv[<span class="s">0</span>].len() &lt; <span class="s">3</span> {
            <span class="k">return</span> <span class="k">self</span>.m_surface.mesh();
        }

        <span class="k">let</span> bbox_diag = <span class="k">self</span>.bbox_diagonal();
        <span class="k">let</span> deflection = bbox_diag * chord_factor;
        <span class="k">let</span> cos_max_angle = (max_angle_deg.clamp(<span class="s">0</span>.<span class="s">1</span>, <span class="s">179</span>.<span class="s">0</span>) * PI / <span class="s">180</span>.<span class="s">0</span>).cos();
        <span class="k">let</span> crease_knots = find_crease_knots(&amp;<span class="k">self</span>.m_surface);
        <span class="k">let</span> bounds = loop_bounds(&amp;loops.uv[<span class="s">0</span>]);

        <span class="k">let</span> <span class="k">mut</span> dt = Delaunay2D::new(bounds[<span class="s">0</span>], bounds[<span class="s">1</span>], bounds[<span class="s">2</span>], bounds[<span class="s">3</span>]);
        <span class="k">let</span> <span class="k">mut</span> boundary_intervals: BTreeMap&lt;usize, (usize, usize, f64)&gt; = BTreeMap::new();
        <span class="k">let</span> loop_vids = insert_loops(&amp;<span class="k">mut</span> dt, &amp;loops.uv, &amp;crease_knots, &amp;<span class="k">mut</span> boundary_intervals);
        insert_crease_lines(&amp;<span class="k">mut</span> dt, &amp;loops.uv, &amp;bounds, &amp;crease_knots);

        <span class="k">for</span> p <span class="k">in</span> &amp;loops.interior_uv {
            <span class="k">if</span> inside_loops(p[<span class="s">0</span>], p[<span class="s">1</span>], &amp;loops.uv, &amp;bounds) {
                dt.insert(p[<span class="s">0</span>], p[<span class="s">1</span>]);
            }
        }

        refine(
            &amp;<span class="k">mut</span> dt,
            &amp;<span class="k">self</span>.m_surface,
            &amp;loops.uv,
            &amp;bounds,
            &amp;crease_knots,
            deflection,
            cos_max_angle,
        );
        trim_outside(&amp;<span class="k">mut</span> dt, &amp;loops.uv, &amp;bounds);
        <span class="k">let</span> tris = dt.get_triangles();

        <span class="k">if</span> tris.is_empty() || crosses_crease(&amp;tris, &amp;dt, &amp;crease_knots) {
            <span class="k">return</span> Mesh::new();
        }

        <span class="k">let</span> <span class="k">mut</span> result = Mesh::new();
        <span class="k">let</span> weld_tol = <span class="k">if</span> loops.xyz.is_empty() {
            bbox_diag * <span class="s">1</span>e-<span class="s">5</span>
        } <span class="k">else</span> {
            <span class="s">0</span>.<span class="s">0</span>
        };
        <span class="k">let</span> <span class="k">mut</span> welder = VertexWelder::new(weld_tol, bbox_diag * <span class="s">1</span>e-<span class="s">5</span>);
        <span class="k">let</span> vert_map = weld_vertices(
            &amp;<span class="k">mut</span> welder,
            &amp;<span class="k">mut</span> result,
            &amp;<span class="k">self</span>.m_surface,
            &amp;dt,
            &amp;tris,
            loops,
            &amp;loop_vids,
            &amp;boundary_intervals,
        );
        add_faces(&amp;<span class="k">mut</span> result, &amp;tris, &amp;vert_map);
        set_vertex_normals(&amp;<span class="k">mut</span> result, &amp;<span class="k">self</span>.m_surface, &amp;dt, &amp;vert_map);
        tag_boundary(&amp;<span class="k">mut</span> result, &amp;loop_vids, &amp;boundary_intervals, &amp;vert_map);
        RemeshNurbsSurfaceGrid::split_crease_normals(&amp;<span class="k">self</span>.m_surface, &amp;<span class="k">mut</span> result);

        result
    }
}

<span class="k">impl</span> std::fmt::Display <span class="k">for</span> NurbsSurfaceTrimmed {
    <span class="c">/// Stream the str() form.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> std::fmt::Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Operators</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="k">impl</span> PartialEq <span class="k">for</span> NurbsSurfaceTrimmed {
    <span class="c">/// Compare name, width, color, surface and trim loops; guid ignored.</span>
    <span class="k">fn</span> eq(&amp;<span class="k">self</span>, other: &amp;<span class="k">Self</span>) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.name != other.name {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.width != other.width {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.surfacecolor != other.surfacecolor {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.m_surface != other.m_surface {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.m_outer_loop != other.m_outer_loop {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">self</span>.m_inner_loops == other.m_inner_loops
    }
}</code></pre></div>
`,toc:[]};export{s as default};
