const s={title:"session_rust/src/remesh_nurbssurface_grid.rs",html:`<h1 id="session_rustsrcremesh_nurbssurface_gridrs">session_rust/src/remesh_nurbssurface_grid.rs<a class="anchor" href="#/course/kernel/remesh_nurbssurface_grid#session_rustsrcremesh_nurbssurface_gridrs" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> std::collections::HashMap;
<span class="k">use</span> std::collections::HashSet;

<span class="k">use</span> <span class="k">crate</span>::mesh::Mesh;
<span class="k">use</span> <span class="k">crate</span>::nurbssurface::NurbsSurface;
<span class="k">use</span> <span class="k">crate</span>::point::Point;
<span class="k">use</span> <span class="k">crate</span>::tolerance::Tolerance;
<span class="k">use</span> <span class="k">crate</span>::vector::Vector;

<span class="c">/// Grid mesh of a NURBS surface: spans split by normal turn and chord height, poles fanned, seams closed.</span>
<span class="k">pub</span> <span class="k">struct</span> RemeshNurbsSurfaceGrid;

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Helpers</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="k">const</span> MAX_SUBS: usize = <span class="s">24</span>;

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Sampling</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Euclidean length without the zero gate of magnitude().</span>
<span class="k">fn</span> norm(v: &amp;Vector) -&gt; f64 {
    v.magnitude_squared().sqrt()
}

<span class="c">/// Surface point at t along dir with the other parameter fixed.</span>
<span class="k">fn</span> point_along(s: &amp;NurbsSurface, dir: usize, t: f64, fixed: f64) -&gt; Point {
    <span class="k">if</span> dir == <span class="s">0</span> {
        s.point_at(t, fixed).unwrap_or_default()
    } <span class="k">else</span> {
        s.point_at(fixed, t).unwrap_or_default()
    }
}

<span class="c">/// Surface normal at t along dir with the other parameter fixed.</span>
<span class="k">fn</span> normal_along(s: &amp;NurbsSurface, dir: usize, t: f64, fixed: f64) -&gt; Vector {
    <span class="k">if</span> dir == <span class="s">0</span> {
        s.normal_at(t, fixed)
    } <span class="k">else</span> {
        s.normal_at(fixed, t)
    }
}

<span class="c">/// Sv x Su unnormalized, zero when the surface cannot be evaluated; normal_at would give a +Z sentinel at a pole.</span>
<span class="k">fn</span> raw_normal(s: &amp;NurbsSurface, u: f64, v: f64) -&gt; Vector {
    <span class="k">let</span> derivatives = s.evaluate(u, v, <span class="s">1</span>);

    <span class="k">if</span> derivatives.len() &lt; <span class="s">3</span> {
        <span class="k">return</span> Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
    }

    derivatives[<span class="s">2</span>].cross(&amp;derivatives[<span class="s">1</span>])
}

<span class="c">/// Diagonal of the control point bounding box.</span>
<span class="k">fn</span> bbox_diagonal(s: &amp;NurbsSurface) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> lo = Point::new(<span class="s">1</span>e<span class="s">30</span>, <span class="s">1</span>e<span class="s">30</span>, <span class="s">1</span>e<span class="s">30</span>);
    <span class="k">let</span> <span class="k">mut</span> hi = Point::new(-<span class="s">1</span>e<span class="s">30</span>, -<span class="s">1</span>e<span class="s">30</span>, -<span class="s">1</span>e<span class="s">30</span>);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..s.cv_count(<span class="s">0</span>) {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..s.cv_count(<span class="s">1</span>) {
            <span class="k">let</span> p = s.get_cv(i, j).unwrap_or_default();

            <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
    }

    norm(&amp;(hi - lo))
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Subdivisions</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Largest turn of the unit normal in degrees over [t0, t1], sampled at the span midpoints of the other direction.</span>
<span class="k">fn</span> span_angle(s: &amp;NurbsSurface, dir: usize, t0: f64, t1: f64, osp: &amp;[f64]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> max_angle = <span class="s">0</span>.<span class="s">0_f64</span>;

    <span class="k">for</span> si <span class="k">in</span> <span class="s">0</span>..osp.len() - <span class="s">1</span> {
        <span class="k">let</span> fixed = (osp[si] + osp[si + <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>;

        <span class="k">let</span> <span class="k">mut</span> first = Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> last = Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> has_first = <span class="s">false</span>;

        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..=<span class="s">4</span> {
            <span class="k">let</span> n = normal_along(s, dir, t0 + k <span class="k">as</span> f64 * (t1 - t0) / <span class="s">4</span>.<span class="s">0</span>, fixed);
            <span class="k">let</span> length = norm(&amp;n);

            <span class="k">if</span> length &lt; <span class="s">1</span>e-<span class="s">10</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> unit = n / length;

            <span class="k">if</span> !has_first {
                first = unit.clone();
            }

            has_first = <span class="s">true</span>;
            last = unit;
        }

        <span class="k">if</span> !has_first {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> dot = first.dot(&amp;last).clamp(-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);

        max_angle = max_angle.max(dot.acos() * <span class="s">180</span>.<span class="s">0</span> / Tolerance::PI);
    }

    max_angle
}

<span class="c">/// Largest height of [t0, t1] over its chord, at up to four positions across the other direction.</span>
<span class="k">fn</span> span_deviation(s: &amp;NurbsSurface, dir: usize, t0: f64, t1: f64, osp: &amp;[f64]) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> max_dev = <span class="s">0</span>.<span class="s">0_f64</span>;
    <span class="k">let</span> nc = (osp.len() - <span class="s">1</span>).min(<span class="s">3</span>);

    <span class="k">for</span> ci <span class="k">in</span> <span class="s">0</span>..=nc {
        <span class="k">let</span> fixed = osp[<span class="s">0</span>] + ci <span class="k">as</span> f64 * (osp[osp.len() - <span class="s">1</span>] - osp[<span class="s">0</span>]) / nc.max(<span class="s">1</span>) <span class="k">as</span> f64;
        <span class="k">let</span> p0 = point_along(s, dir, t0, fixed);
        <span class="k">let</span> p1 = point_along(s, dir, t1, fixed);

        <span class="k">for</span> k <span class="k">in</span> <span class="s">1</span>..=<span class="s">3</span> {
            <span class="k">let</span> frac = k <span class="k">as</span> f64 / <span class="s">4</span>.<span class="s">0</span>;
            <span class="k">let</span> pm = point_along(s, dir, t0 + frac * (t1 - t0), fixed);

            max_dev = max_dev.max(norm(&amp;(pm - (&amp;p0 + (&amp;p1 - &amp;p0) * frac))));
        }
    }

    max_dev
}

<span class="c">/// Subdivisions per span along dir: the normal turn against max_angle_deg, the chord height against chord_tol, at least two on a curved span.</span>
<span class="k">fn</span> span_subs(
    s: &amp;NurbsSurface,
    dir: usize,
    sp: &amp;[f64],
    osp: &amp;[f64],
    max_angle_deg: f64,
    chord_tol: f64,
) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> degree = s.degree(dir);
    <span class="k">let</span> <span class="k">mut</span> subs = vec![<span class="s">1</span>; sp.len() - <span class="s">1</span>];

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..sp.len() - <span class="s">1</span> {
        <span class="k">if</span> degree &gt; <span class="s">1</span> {
            <span class="k">let</span> angle = span_angle(s, dir, sp[i], sp[i + <span class="s">1</span>], osp);

            subs[i] = ((angle / max_angle_deg).ceil() <span class="k">as</span> usize).clamp(<span class="s">1</span>, MAX_SUBS);
        }

        <span class="k">let</span> dev = span_deviation(s, dir, sp[i], sp[i + <span class="s">1</span>], osp);

        <span class="k">if</span> dev &gt; chord_tol {
            subs[i] = subs[i].max(((dev / chord_tol).sqrt().ceil() <span class="k">as</span> usize).clamp(<span class="s">2</span>, MAX_SUBS));
        }

        <span class="k">if</span> degree &gt; <span class="s">1</span> {
            subs[i] = subs[i].max(<span class="s">2</span>);
        }
    }

    subs
}

<span class="c">/// Length of the iso-curve at fixed along dir as a polyline of n steps.</span>
<span class="k">fn</span> isocurve_length(s: &amp;NurbsSurface, dir: usize, sp: &amp;[f64], fixed: f64, n: usize) -&gt; f64 {
    <span class="k">let</span> <span class="k">mut</span> length = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> prev = point_along(s, dir, sp[<span class="s">0</span>], fixed);

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..=n {
        <span class="k">let</span> next = point_along(
            s,
            dir,
            sp[<span class="s">0</span>] + i <span class="k">as</span> f64 * (sp[sp.len() - <span class="s">1</span>] - sp[<span class="s">0</span>]) / n <span class="k">as</span> f64,
            fixed,
        );

        length += norm(&amp;(&amp;next - &amp;prev));
        prev = next;
    }

    length
}

<span class="c">/// Scale up the curved direction whose spacing is more than twice the other's.</span>
<span class="k">fn</span> balance_subs(
    s: &amp;NurbsSurface,
    usp: &amp;[f64],
    vsp: &amp;[f64],
    u_subs: &amp;<span class="k">mut</span> [usize],
    v_subs: &amp;<span class="k">mut</span> [usize],
) {
    <span class="k">let</span> <span class="k">mut</span> total_u = <span class="s">1</span>;
    <span class="k">let</span> <span class="k">mut</span> total_v = <span class="s">1</span>;

    <span class="k">for</span> sub <span class="k">in</span> u_subs.iter() {
        total_u += sub;
    }

    <span class="k">for</span> sub <span class="k">in</span> v_subs.iter() {
        total_v += sub;
    }

    <span class="k">let</span> u_len = isocurve_length(
        s,
        <span class="s">0</span>,
        usp,
        (vsp[<span class="s">0</span>] + vsp[vsp.len() - <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
        total_u.max(<span class="s">10</span>),
    );
    <span class="k">let</span> v_len = isocurve_length(
        s,
        <span class="s">1</span>,
        vsp,
        (usp[<span class="s">0</span>] + usp[usp.len() - <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
        total_v.max(<span class="s">10</span>),
    );

    <span class="k">if</span> u_len &lt;= <span class="s">1</span>e-<span class="s">14</span> || v_len &lt;= <span class="s">1</span>e-<span class="s">14</span> {
        <span class="k">return</span>;
    }

    <span class="k">let</span> ratio = (u_len / total_u <span class="k">as</span> f64) / (v_len / total_v <span class="k">as</span> f64);

    <span class="k">if</span> ratio &gt; <span class="s">2</span>.<span class="s">0</span> &amp;&amp; s.degree(<span class="s">0</span>) &gt; <span class="s">1</span> {
        <span class="k">let</span> scale = ratio.sqrt();

        <span class="k">for</span> sub <span class="k">in</span> u_subs.iter_mut() {
            *sub = MAX_SUBS.min((*sub <span class="k">as</span> f64 * scale).ceil() <span class="k">as</span> usize);
        }
    } <span class="k">else</span> <span class="k">if</span> ratio &lt; <span class="s">0</span>.<span class="s">5</span> &amp;&amp; s.degree(<span class="s">1</span>) &gt; <span class="s">1</span> {
        <span class="k">let</span> scale = (<span class="s">1</span>.<span class="s">0</span> / ratio).sqrt();

        <span class="k">for</span> sub <span class="k">in</span> v_subs.iter_mut() {
            *sub = MAX_SUBS.min((*sub <span class="k">as</span> f64 * scale).ceil() <span class="k">as</span> usize);
        }
    }
}

<span class="c">/// Subdivisions both directions of a bilinear surface need for its twist, 1 when every span centre lies within twist_tol of its diagonal midpoint.</span>
<span class="k">fn</span> twist_subs(s: &amp;NurbsSurface, usp: &amp;[f64], vsp: &amp;[f64], twist_tol: f64) -&gt; usize {
    <span class="k">let</span> <span class="k">mut</span> max_twist = <span class="s">0</span>.<span class="s">0_f64</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..usp.len() - <span class="s">1</span> {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..vsp.len() - <span class="s">1</span> {
            <span class="k">let</span> pm = s
                .point_at((usp[i] + usp[i + <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>, (vsp[j] + vsp[j + <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>)
                .unwrap_or_default();
            <span class="k">let</span> p00 = s.point_at(usp[i], vsp[j]).unwrap_or_default();
            <span class="k">let</span> p11 = s.point_at(usp[i + <span class="s">1</span>], vsp[j + <span class="s">1</span>]).unwrap_or_default();

            max_twist = max_twist.max(norm(&amp;(pm - Point::sum(&amp;p00, &amp;p11) * <span class="s">0</span>.<span class="s">5</span>)));
        }
    }

    <span class="k">if</span> max_twist &lt;= twist_tol {
        <span class="k">return</span> <span class="s">1</span>;
    }

    ((<span class="s">2</span>.<span class="s">0</span> * (max_twist / twist_tol).sqrt()).ceil() <span class="k">as</span> usize).clamp(<span class="s">4</span>, MAX_SUBS)
}

<span class="c">/// One more subdivision on the largest span when the total is even, so a closed direction triangulates seamlessly.</span>
<span class="k">fn</span> set_odd_total(subs: &amp;<span class="k">mut</span> [usize]) {
    <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>;

    <span class="k">for</span> sub <span class="k">in</span> subs.iter() {
        total += sub;
    }

    <span class="k">if</span> total % <span class="s">2</span> != <span class="s">0</span> {
        <span class="k">return</span>;
    }

    <span class="k">let</span> <span class="k">mut</span> largest = <span class="s">0</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..subs.len() {
        <span class="k">if</span> subs[i] &gt; subs[largest] {
            largest = i;
        }
    }

    subs[largest] += <span class="s">1</span>;
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Parameters</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// n parameters spaced evenly by arc length along the iso-curve at fixed.</span>
<span class="k">fn</span> arclen_params(s: &amp;NurbsSurface, dir: usize, n: usize, sp: &amp;[f64], fixed: f64) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> nsample = (n * <span class="s">20</span>).max(<span class="s">200</span>);

    <span class="k">let</span> <span class="k">mut</span> st = vec![<span class="s">0</span>.<span class="s">0</span>; nsample + <span class="s">1</span>];
    <span class="k">let</span> <span class="k">mut</span> sl = vec![<span class="s">0</span>.<span class="s">0</span>; nsample + <span class="s">1</span>];
    <span class="k">let</span> <span class="k">mut</span> prev = point_along(s, dir, sp[<span class="s">0</span>], fixed);

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..=nsample {
        st[k] = sp[<span class="s">0</span>] + k <span class="k">as</span> f64 * (sp[sp.len() - <span class="s">1</span>] - sp[<span class="s">0</span>]) / nsample <span class="k">as</span> f64;

        <span class="k">if</span> k == <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> next = point_along(s, dir, st[k], fixed);

        sl[k] = sl[k - <span class="s">1</span>] + norm(&amp;(&amp;next - &amp;prev));
        prev = next;
    }

    <span class="k">let</span> <span class="k">mut</span> params = vec![sp[<span class="s">0</span>]];
    <span class="k">let</span> <span class="k">mut</span> j = <span class="s">0</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..n - <span class="s">1</span> {
        <span class="k">let</span> target = sl[nsample] * i <span class="k">as</span> f64 / (n - <span class="s">1</span>) <span class="k">as</span> f64;

        <span class="k">while</span> j &lt; nsample &amp;&amp; sl[j] &lt; target {
            j += <span class="s">1</span>;
        }

        <span class="k">let</span> a = <span class="k">if</span> j &gt; <span class="s">0</span> { j - <span class="s">1</span> } <span class="k">else</span> { <span class="s">0</span> };
        <span class="k">let</span> frac = <span class="k">if</span> sl[j] &gt; sl[a] {
            (target - sl[a]) / (sl[j] - sl[a])
        } <span class="k">else</span> {
            <span class="s">0</span>.<span class="s">0</span>
        };

        params.push(st[a] + frac * (st[j] - st[a]));
    }

    params.push(sp[sp.len() - <span class="s">1</span>]);

    params
}

<span class="c">/// Every span split into its subdivisions, ending on the last span boundary.</span>
<span class="k">fn</span> span_params(sp: &amp;[f64], subs: &amp;[usize]) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> <span class="k">mut</span> params = Vec::new();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..sp.len() - <span class="s">1</span> {
        <span class="k">for</span> sub <span class="k">in</span> <span class="s">0</span>..subs[i] {
            params.push(sp[i] + sub <span class="k">as</span> f64 * (sp[i + <span class="s">1</span>] - sp[i]) / subs[i] <span class="k">as</span> f64);
        }
    }

    params.push(sp[sp.len() - <span class="s">1</span>]);

    params
}

<span class="c">/// Closed direction: drop the duplicate end and fill a wrap gap wider than 1.5 times the largest step.</span>
<span class="k">fn</span> fix_closed_gap(params: &amp;<span class="k">mut</span> Vec&lt;f64&gt;, domain_end: f64) {
    <span class="k">if</span> params.len() &lt; <span class="s">3</span> {
        <span class="k">return</span>;
    }

    params.pop();

    <span class="k">let</span> wrap_gap = domain_end - params[params.len() - <span class="s">1</span>];
    <span class="k">let</span> <span class="k">mut</span> max_gap = <span class="s">0</span>.<span class="s">0_f64</span>;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">1</span>..params.len() {
        max_gap = max_gap.max(params[i] - params[i - <span class="s">1</span>]);
    }

    <span class="k">if</span> max_gap &lt;= <span class="s">0</span>.<span class="s">0</span> || wrap_gap &lt;= max_gap * <span class="s">1</span>.<span class="s">5</span> {
        <span class="k">return</span>;
    }

    <span class="k">let</span> extra = (wrap_gap / max_gap).ceil() <span class="k">as</span> usize - <span class="s">1</span>;
    <span class="k">let</span> step = wrap_gap / (extra + <span class="s">1</span>) <span class="k">as</span> f64;

    <span class="k">for</span> _ <span class="k">in</span> <span class="s">1</span>..=extra {
        params.push(params[params.len() - <span class="s">1</span>] + step);
    }
}

<span class="c">/// Parameters along dir: arc-length spaced when count is positive, else the span subdivisions; a closed direction made odd and its wrap gap filled.</span>
<span class="k">fn</span> grid_params(
    s: &amp;NurbsSurface,
    dir: usize,
    count: usize,
    sp: &amp;[f64],
    fixed: f64,
    <span class="k">mut</span> subs: Vec&lt;usize&gt;,
) -&gt; Vec&lt;f64&gt; {
    <span class="k">let</span> closed = s.is_closed(dir);

    <span class="k">if</span> closed &amp;&amp; count == <span class="s">0</span> {
        set_odd_total(&amp;<span class="k">mut</span> subs);
    }

    <span class="k">let</span> <span class="k">mut</span> params = <span class="k">if</span> count &gt; <span class="s">0</span> {
        arclen_params(s, dir, count.max(<span class="s">2</span>), sp, fixed)
    } <span class="k">else</span> {
        span_params(sp, &amp;subs)
    };

    <span class="k">if</span> closed {
        fix_closed_gap(&amp;<span class="k">mut</span> params, sp[sp.len() - <span class="s">1</span>]);
    }

    params
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Vertices and faces</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Vertex at S(u, v) tagged with its parameters.</span>
<span class="k">fn</span> add_vertex_uv(s: &amp;NurbsSurface, mesh: &amp;<span class="k">mut</span> Mesh, u: f64, v: f64) -&gt; usize {
    <span class="k">let</span> key = mesh.add_vertex(s.point_at(u, v).unwrap_or_default(), None);
    <span class="k">let</span> vd = mesh.vertex.get_mut(&amp;key).unwrap();

    vd.attributes.insert(&quot;<span class="s">u</span>&quot;.to_string(), u);
    vd.attributes.insert(&quot;<span class="s">v</span>&quot;.to_string(), v);

    key
}

<span class="c">/// Grid vertices row by row over us and the rows j_start..j_end of vs.</span>
<span class="k">fn</span> add_grid(
    s: &amp;NurbsSurface,
    mesh: &amp;<span class="k">mut</span> Mesh,
    us: &amp;[f64],
    vs: &amp;[f64],
    j_start: usize,
    j_end: usize,
) -&gt; Vec&lt;usize&gt; {
    <span class="k">let</span> <span class="k">mut</span> grid = Vec::new();

    <span class="k">for</span> &amp;u <span class="k">in</span> us {
        <span class="k">for</span> &amp;v <span class="k">in</span> &amp;vs[j_start..j_end] {
            grid.push(add_vertex_uv(s, mesh, u, v));
        }
    }

    grid
}

<span class="c">/// Fans from the south pole, checkerboard-split quads, fans to the north pole.</span>
<span class="k">fn</span> add_faces(
    mesh: &amp;<span class="k">mut</span> Mesh,
    grid: &amp;[usize],
    nu: usize,
    closed_u: bool,
    wrap_v: bool,
    south: Option&lt;usize&gt;,
    north: Option&lt;usize&gt;,
) {
    <span class="k">let</span> nv = grid.len() / nu;
    <span class="k">let</span> nu_faces = <span class="k">if</span> closed_u { nu } <span class="k">else</span> { nu - <span class="s">1</span> };
    <span class="k">let</span> nv_faces = <span class="k">if</span> wrap_v { nv } <span class="k">else</span> { nv - <span class="s">1</span> };

    <span class="k">if</span> <span class="k">let</span> Some(south) = south {
        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu_faces {
            mesh.add_face(vec![south, grid[((i + <span class="s">1</span>) % nu) * nv], grid[i * nv]], None);
        }
    }

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu_faces {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..nv_faces {
            <span class="k">let</span> i1 = (i + <span class="s">1</span>) % nu;
            <span class="k">let</span> j1 = (j + <span class="s">1</span>) % nv;
            <span class="k">let</span> v00 = grid[i * nv + j];
            <span class="k">let</span> v10 = grid[i1 * nv + j];
            <span class="k">let</span> v01 = grid[i * nv + j1];
            <span class="k">let</span> v11 = grid[i1 * nv + j1];

            <span class="k">if</span> (i + j) % <span class="s">2</span> == <span class="s">0</span> {
                mesh.add_face(vec![v00, v10, v11], None);
                mesh.add_face(vec![v00, v11, v01], None);
            } <span class="k">else</span> {
                mesh.add_face(vec![v00, v10, v01], None);
                mesh.add_face(vec![v10, v11, v01], None);
            }
        }
    }

    <span class="k">if</span> <span class="k">let</span> Some(north) = north {
        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu_faces {
            mesh.add_face(
                vec![
                    grid[i * nv + nv - <span class="s">1</span>],
                    grid[((i + <span class="s">1</span>) % nu) * nv + nv - <span class="s">1</span>],
                    north,
                ],
                None,
            );
        }
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Normals</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// Sum of the unnormalized face normals around each vertex key, faces taken in key order.</span>
<span class="k">fn</span> fan_normals(mesh: &amp;Mesh) -&gt; Vec&lt;Vector&gt; {
    <span class="k">let</span> <span class="k">mut</span> sums = vec![Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>); mesh.vertex.len()];
    <span class="k">let</span> <span class="k">mut</span> face_keys: Vec&lt;usize&gt; = mesh.face.keys().copied().collect();
    face_keys.sort_unstable();

    <span class="k">for</span> key <span class="k">in</span> face_keys {
        <span class="k">let</span> vertices = &amp;mesh.face[&amp;key];

        <span class="k">if</span> vertices.len() &lt; <span class="s">3</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> p0 = mesh.vertex[&amp;vertices[<span class="s">0</span>]].position();
        <span class="k">let</span> p1 = mesh.vertex[&amp;vertices[<span class="s">1</span>]].position();
        <span class="k">let</span> p2 = mesh.vertex[&amp;vertices[<span class="s">2</span>]].position();
        <span class="k">let</span> n = (&amp;p1 - &amp;p0).cross(&amp;(&amp;p2 - &amp;p0));

        <span class="k">for</span> &amp;vertex <span class="k">in</span> vertices {
            sums[vertex] += &amp;n;
        }
    }

    sums
}

<span class="c">/// Unit surface normal on the side of the fan normal; the fan normal at the poles and where the surface normal vanishes, +Z when the fan vanishes too.</span>
<span class="k">fn</span> set_normals(s: &amp;NurbsSurface, mesh: &amp;<span class="k">mut</span> Mesh, south: Option&lt;usize&gt;, north: Option&lt;usize&gt;) {
    <span class="k">let</span> sums = fan_normals(mesh);

    <span class="k">for</span> (&amp;key, vd) <span class="k">in</span> mesh.vertex.iter_mut() {
        <span class="k">let</span> fan_length = norm(&amp;sums[key]);

        <span class="k">let</span> <span class="k">mut</span> n = Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);

        <span class="k">if</span> fan_length.is_finite() &amp;&amp; fan_length &gt; <span class="s">0</span>.<span class="s">0</span> {
            n = &amp;sums[key] / fan_length;
        }

        <span class="k">if</span> Some(key) != south &amp;&amp; Some(key) != north {
            <span class="k">let</span> raw = raw_normal(
                s,
                *vd.attributes.get(&quot;<span class="s">u</span>&quot;).unwrap(),
                *vd.attributes.get(&quot;<span class="s">v</span>&quot;).unwrap(),
            );
            <span class="k">let</span> length = norm(&amp;raw);

            <span class="k">if</span> length.is_finite() &amp;&amp; length &gt; <span class="s">0</span>.<span class="s">0</span> {
                n = <span class="k">if</span> raw.dot(&amp;n) &lt; <span class="s">0</span>.<span class="s">0</span> {
                    -raw / length
                } <span class="k">else</span> {
                    raw / length
                };
            }
        }

        vd.set_normal(n[<span class="s">0</span>], n[<span class="s">1</span>], n[<span class="s">2</span>]);
    }
}

<span class="c">/// Bit per direction where (u, v) sits on an internal knot of full multiplicity whose one-sided normals disagree.</span>
<span class="k">fn</span> crease_flags(s: &amp;NurbsSurface, u: f64, v: f64) -&gt; u32 {
    <span class="k">let</span> uv = [u, v];
    <span class="k">let</span> <span class="k">mut</span> flags = <span class="s">0</span>;

    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">let</span> domain = s.domain(dir).unwrap_or_default();
        <span class="k">let</span> value = uv[dir];

        <span class="k">if</span> value &lt;= domain.<span class="s">0</span> || value &gt;= domain.<span class="s">1</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> multiplicity = <span class="s">0</span>;

        <span class="k">for</span> &amp;knot <span class="k">in</span> &amp;s.m_nurbsknot[dir] {
            <span class="k">if</span> knot == value {
                multiplicity += <span class="s">1</span>;
            }
        }

        <span class="k">if</span> multiplicity &lt; s.degree(dir) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> lo = [u, v];
        <span class="k">let</span> <span class="k">mut</span> hi = [u, v];

        lo[dir] = value.next_down();
        hi[dir] = value.next_up();

        <span class="k">let</span> a = s.normal_at(lo[<span class="s">0</span>], lo[<span class="s">1</span>]);
        <span class="k">let</span> b = s.normal_at(hi[<span class="s">0</span>], hi[<span class="s">1</span>]);
        <span class="k">let</span> length = (a.magnitude_squared() * b.magnitude_squared()).sqrt();

        <span class="k">if</span> length == <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> dot = a.dot(&amp;b) / length;

        <span class="k">if</span> dot.is_finite() &amp;&amp; dot &lt; <span class="s">1</span>.<span class="s">0</span> - <span class="s">64</span>.<span class="s">0</span> * f64::EPSILON {
            flags |= <span class="s">1</span> &lt;&lt; dir;
        }
    }

    flags
}

<span class="c">/// Nudge uv one ulp toward center in each flagged direction; bit per direction nudged upward.</span>
<span class="k">fn</span> crease_side(center: &amp;[f64; <span class="s">2</span>], uv: &amp;<span class="k">mut</span> [f64; <span class="s">2</span>], flags: u32) -&gt; u32 {
    <span class="k">let</span> <span class="k">mut</span> side = <span class="s">0</span>;

    <span class="k">for</span> dir <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
        <span class="k">if</span> flags &amp; (<span class="s">1</span> &lt;&lt; dir) == <span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> high = center[dir] &gt; uv[dir];

        <span class="k">if</span> high {
            side |= <span class="s">1</span> &lt;&lt; dir;
        }

        uv[dir] = <span class="k">if</span> high {
            uv[dir].next_up()
        } <span class="k">else</span> {
            uv[dir].next_down()
        };
    }

    side
}

<span class="c">/// Vertex carrying a corner: the original the first time its key is met, then one copy per (key, side).</span>
<span class="k">fn</span> crease_target(
    mesh: &amp;<span class="k">mut</span> Mesh,
    copies: &amp;<span class="k">mut</span> HashMap&lt;(usize, u32), usize&gt;,
    used: &amp;<span class="k">mut</span> HashSet&lt;usize&gt;,
    key: usize,
    side: u32,
) -&gt; usize {
    <span class="k">let</span> identity = (key, side);

    <span class="k">if</span> <span class="k">let</span> Some(&amp;target) = copies.get(&amp;identity) {
        <span class="k">return</span> target;
    }

    <span class="k">if</span> used.insert(key) {
        copies.insert(identity, key);

        <span class="k">return</span> key;
    }

    <span class="k">let</span> target = mesh.add_vertex(mesh.vertex[&amp;key].position(), None);

    mesh.vertex.insert(target, mesh.vertex[&amp;key].clone());
    copies.insert(identity, target);

    target
}

<span class="k">impl</span> RemeshNurbsSurfaceGrid {
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Static constructors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Grid at 20 degrees and 0.5 percent of the bbox diagonal; max_u and max_v fix the parameter counts when positive.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_u_v(s: &amp;NurbsSurface, max_u: usize, max_v: usize) -&gt; Mesh {
        <span class="k">Self</span>::from_u_v_q(s, max_u, max_v, <span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">005</span>)
    }

    <span class="c">/// Grid with the normal turn per subdivision capped at max_angle_deg and the chord height at chord_factor of the bbox diagonal; vertex normals are unit surface normals on the fan side, fan normals at poles.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_u_v_q(
        s: &amp;NurbsSurface,
        max_u: usize,
        max_v: usize,
        max_angle_deg: f64,
        chord_factor: f64,
    ) -&gt; Mesh {
        <span class="k">let</span> usp = s.get_span_vector(<span class="s">0</span>);
        <span class="k">let</span> vsp = s.get_span_vector(<span class="s">1</span>);
        <span class="k">let</span> bbox_diag = bbox_diagonal(s);
        <span class="k">let</span> chord_tol = bbox_diag * chord_factor;

        <span class="k">let</span> <span class="k">mut</span> u_subs = span_subs(s, <span class="s">0</span>, &amp;usp, &amp;vsp, max_angle_deg, chord_tol);
        <span class="k">let</span> <span class="k">mut</span> v_subs = span_subs(s, <span class="s">1</span>, &amp;vsp, &amp;usp, max_angle_deg, chord_tol);

        balance_subs(s, &amp;usp, &amp;vsp, &amp;<span class="k">mut</span> u_subs, &amp;<span class="k">mut</span> v_subs);

        <span class="k">let</span> sing_v0 = s.is_singular(<span class="s">0</span>);
        <span class="k">let</span> sing_v1 = s.is_singular(<span class="s">2</span>);

        <span class="k">if</span> s.degree(<span class="s">0</span>) == <span class="s">1</span> &amp;&amp; s.degree(<span class="s">1</span>) == <span class="s">1</span> &amp;&amp; !sing_v0 &amp;&amp; !sing_v1 {
            <span class="k">let</span> twist = twist_subs(
                s,
                &amp;usp,
                &amp;vsp,
                <span class="k">if</span> bbox_diag &gt; <span class="s">0</span>.<span class="s">0</span> { chord_tol } <span class="k">else</span> { <span class="s">1</span>e-<span class="s">6</span> },
            );

            <span class="k">for</span> sub <span class="k">in</span> u_subs.iter_mut() {
                *sub = (*sub).max(twist);
            }

            <span class="k">for</span> sub <span class="k">in</span> v_subs.iter_mut() {
                *sub = (*sub).max(twist);
            }
        }

        <span class="k">let</span> u_mid = (usp[<span class="s">0</span>] + usp[usp.len() - <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> v_mid = (vsp[<span class="s">0</span>] + vsp[vsp.len() - <span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>;

        <span class="k">let</span> us = grid_params(s, <span class="s">0</span>, max_u, &amp;usp, v_mid, u_subs);
        <span class="k">let</span> vs = grid_params(s, <span class="s">1</span>, max_v, &amp;vsp, u_mid, v_subs);
        <span class="k">let</span> nv = vs.len();

        <span class="k">if</span> sing_v0 &amp;&amp; sing_v1 &amp;&amp; nv &lt; <span class="s">3</span> {
            <span class="k">return</span> Mesh::new();
        }

        <span class="k">let</span> <span class="k">mut</span> mesh = Mesh::new();
        <span class="k">let</span> <span class="k">mut</span> south = None;
        <span class="k">let</span> <span class="k">mut</span> north = None;

        <span class="k">if</span> sing_v0 {
            south = Some(add_vertex_uv(s, &amp;<span class="k">mut</span> mesh, us[<span class="s">0</span>], vs[<span class="s">0</span>]));
        }

        <span class="k">if</span> sing_v1 {
            north = Some(add_vertex_uv(s, &amp;<span class="k">mut</span> mesh, us[<span class="s">0</span>], vs[nv - <span class="s">1</span>]));
        }

        <span class="k">let</span> grid = add_grid(
            s,
            &amp;<span class="k">mut</span> mesh,
            &amp;us,
            &amp;vs,
            <span class="k">if</span> sing_v0 { <span class="s">1</span> } <span class="k">else</span> { <span class="s">0</span> },
            <span class="k">if</span> sing_v1 { nv - <span class="s">1</span> } <span class="k">else</span> { nv },
        );

        add_faces(
            &amp;<span class="k">mut</span> mesh,
            &amp;grid,
            us.len(),
            s.is_closed(<span class="s">0</span>),
            s.is_closed(<span class="s">1</span>) &amp;&amp; !sing_v0 &amp;&amp; !sing_v1,
            south,
            north,
        );
        set_normals(s, &amp;<span class="k">mut</span> mesh, south, north);
        <span class="k">Self</span>::split_crease_normals(s, &amp;<span class="k">mut</span> mesh);

        mesh
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Normals</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Split shading vertices at internal C0 knots whose one-sided normals disagree.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> split_crease_normals(s: &amp;NurbsSurface, mesh: &amp;<span class="k">mut</span> Mesh) {
        <span class="k">let</span> <span class="k">mut</span> candidates = HashMap::&lt;usize, u32&gt;::new();

        <span class="k">for</span> (&amp;key, vd) <span class="k">in</span> &amp;mesh.vertex {
            <span class="k">let</span> (Some(&amp;u), Some(&amp;v)) = (vd.attributes.get(&quot;<span class="s">u</span>&quot;), vd.attributes.get(&quot;<span class="s">v</span>&quot;)) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">let</span> flags = crease_flags(s, u, v);

            <span class="k">if</span> flags != <span class="s">0</span> {
                candidates.insert(key, flags);
            }
        }

        <span class="k">if</span> candidates.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> copies = HashMap::&lt;(usize, u32), usize&gt;::new();
        <span class="k">let</span> <span class="k">mut</span> used = HashSet::&lt;usize&gt;::new();

        <span class="k">let</span> <span class="k">mut</span> face_keys: Vec&lt;usize&gt; = mesh.face.keys().copied().collect();
        face_keys.sort_unstable();

        <span class="k">for</span> face_key <span class="k">in</span> face_keys {
            <span class="k">let</span> <span class="k">mut</span> vertices = mesh.face[&amp;face_key].clone();
            <span class="k">let</span> <span class="k">mut</span> center = [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>];

            <span class="k">for</span> &amp;key <span class="k">in</span> &amp;vertices {
                center[<span class="s">0</span>] += mesh.vertex[&amp;key].attributes.get(&quot;<span class="s">u</span>&quot;).unwrap();
                center[<span class="s">1</span>] += mesh.vertex[&amp;key].attributes.get(&quot;<span class="s">v</span>&quot;).unwrap();
            }

            center[<span class="s">0</span>] /= vertices.len() <span class="k">as</span> f64;
            center[<span class="s">1</span>] /= vertices.len() <span class="k">as</span> f64;

            <span class="k">let</span> face_normal = mesh.face_normal(face_key);

            <span class="k">for</span> slot <span class="k">in</span> vertices.iter_mut() {
                <span class="k">let</span> key = *slot;
                <span class="k">let</span> Some(&amp;flags) = candidates.get(&amp;key) <span class="k">else</span> {
                    <span class="k">continue</span>;
                };

                <span class="k">let</span> <span class="k">mut</span> uv = [
                    *mesh.vertex[&amp;key].attributes.get(&quot;<span class="s">u</span>&quot;).unwrap(),
                    *mesh.vertex[&amp;key].attributes.get(&quot;<span class="s">v</span>&quot;).unwrap(),
                ];

                <span class="k">let</span> side = crease_side(&amp;center, &amp;<span class="k">mut</span> uv, flags);
                <span class="k">let</span> target = crease_target(mesh, &amp;<span class="k">mut</span> copies, &amp;<span class="k">mut</span> used, key, side);
                <span class="k">let</span> n = s.normal_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>]);
                <span class="k">let</span> length = norm(&amp;n);

                <span class="k">if</span> length.is_finite() &amp;&amp; length &gt; <span class="s">0</span>.<span class="s">0</span> {
                    <span class="k">let</span> sign = <span class="k">match</span> &amp;face_normal {
                        Some(normal) <span class="k">if</span> n.dot(normal) &lt; <span class="s">0</span>.<span class="s">0</span> =&gt; -<span class="s">1</span>.<span class="s">0</span>,
                        _ =&gt; <span class="s">1</span>.<span class="s">0</span>,
                    };

                    mesh.vertex.get_mut(&amp;target).unwrap().set_normal(
                        sign * n[<span class="s">0</span>] / length,
                        sign * n[<span class="s">1</span>] / length,
                        sign * n[<span class="s">2</span>] / length,
                    );
                }

                *slot = target;
            }

            mesh.face.insert(face_key, vertices);
        }

        mesh.rebuild_halfedges();
    }
}</code></pre></div>
`,toc:[]};export{s as default};
