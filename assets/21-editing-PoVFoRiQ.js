const s={title:"21 · Editing: the gumball, the command line and the layers panel",html:`<h1 id="21-editing-the-gumball-the-command-line-and-the-layers-panel">21 · Editing: the gumball, the command line and the layers panel<a class="anchor" href="#/course/21-editing#21-editing-the-gumball-the-command-line-and-the-layers-panel" aria-label="Link to this section">#</a></h1>
<p>Selecting an object shows a gumball, and a drag or typed command records an undoable edit.</p>
<p><img src="/session/docs/course/docs/illustrations/one-gesture.svg" alt="A drag is three moments: grabbing remembers the object's own transform, every move frame writes a preview into the row's GPU placement and touches no document, and letting go writes the document once." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcappmodrs">Step 1 · src/app/mod.rs<a class="anchor" href="#/course/21-editing#step-1-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The application module connects source loading and interaction helpers.</p>
<p><code>lessons/21/src/app/mod.rs</code> · edit · type this</p>
<p>Replaces <code>mod feedback</code> in <code>lessons/20/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> command;
<span class="k">pub</span> <span class="k">mod</span> coords;
<span class="k">pub</span> <span class="k">mod</span> cplane;
<span class="k">pub</span> <span class="k">mod</span> edit;
<span class="k">pub</span> <span class="k">mod</span> feedback;
<span class="k">pub</span> <span class="k">mod</span> gizmo;
<span class="k">pub</span> <span class="k">mod</span> input;
<span class="k">pub</span> <span class="k">mod</span> knobs;
<span class="k">pub</span> <span class="k">mod</span> layers;
<span class="k">pub</span> <span class="k">mod</span> manifest;
<span class="k">pub</span> <span class="k">mod</span> scene;
<span class="k">pub</span> <span class="k">mod</span> selection;
<span class="k">pub</span> <span class="k">mod</span> sheet_query;
<span class="k">pub</span> <span class="k">mod</span> snap;</code></pre></div>
<h2 id="step-2-srcstaters">Step 2 · src/state.rs<a class="anchor" href="#/course/21-editing#step-2-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/21/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>mod cloud_query;</code> line of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> edit;</code></pre></div>
<h2 id="step-3-srcappcplaners">Step 3 · src/app/cplane.rs<a class="anchor" href="#/course/21-editing#step-3-srcappcplaners" aria-label="Link to this section">#</a></h2>
<p>The construction plane maps cursor rays into modeling coordinates.</p>
<p><code>lessons/21/src/app/cplane.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The construction plane: the flat surface new geometry is drawn on, because a click only gives two screen numbers.</span>
<span class="k">use</span> session_rust::{Point, Vector};

<span class="c">/// A construction plane through two world axes.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> CPlane {
    Xy, <span class="c">// ground, normal z</span>
    Yz, <span class="c">// normal x</span>
    Xz, <span class="c">// normal y</span>
}</code></pre></div>
<p><code>lessons/21/src/app/cplane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> CPlane {
    <span class="c">/// The plane the view faces most directly.</span>
    <span class="k">pub</span> <span class="k">fn</span> facing(forward: &amp;Vector) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> (x, y, z) = (forward[<span class="s">0</span>].abs(), forward[<span class="s">1</span>].abs(), forward[<span class="s">2</span>].abs());

        <span class="k">if</span> z &gt;= x &amp;&amp; z &gt;= y {
            CPlane::Xy
        } <span class="k">else</span> <span class="k">if</span> y &gt;= x {
            CPlane::Xz
        } <span class="k">else</span> {
            CPlane::Yz
        }
    }

    <span class="c">/// The plane's unit normal.</span>
    <span class="k">pub</span> <span class="k">fn</span> normal(<span class="k">self</span>) -&gt; Vector {
        <span class="k">match</span> <span class="k">self</span> {
            CPlane::Xy =&gt; Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
            CPlane::Yz =&gt; Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            CPlane::Xz =&gt; Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        }
    }

    <span class="c">/// Where a ray hits this plane through \`origin\`, in front of the eye.</span>
    <span class="k">pub</span> <span class="k">fn</span> hit(<span class="k">self</span>, origin: &amp;Point, from: &amp;Point, direction: &amp;Vector) -&gt; Option&lt;Point&gt; {
        <span class="k">let</span> n = <span class="k">self</span>.normal();
        <span class="k">let</span> denom = direction[<span class="s">0</span>] * n[<span class="s">0</span>] + direction[<span class="s">1</span>] * n[<span class="s">1</span>] + direction[<span class="s">2</span>] * n[<span class="s">2</span>];

        <span class="k">if</span> denom.abs() &lt; <span class="s">1</span>e-<span class="s">12</span> {
            <span class="k">return</span> None; <span class="c">// ray parallel to the plane</span>
        }

        <span class="k">let</span> num = (origin[<span class="s">0</span>] - from[<span class="s">0</span>]) * n[<span class="s">0</span>]
            + (origin[<span class="s">1</span>] - from[<span class="s">1</span>]) * n[<span class="s">1</span>]
            + (origin[<span class="s">2</span>] - from[<span class="s">2</span>]) * n[<span class="s">2</span>];
        <span class="k">let</span> t = num / denom;

        <span class="k">if</span> !t.is_finite() || t &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> None; <span class="c">// behind the eye</span>
        }

        Some(Point::new(
            from[<span class="s">0</span>] + direction[<span class="s">0</span>] * t,
            from[<span class="s">1</span>] + direction[<span class="s">1</span>] * t,
            from[<span class="s">2</span>] + direction[<span class="s">2</span>] * t,
        ))
    }
}</code></pre></div>
<p><code>lessons/21/src/app/cplane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Looking down picks the ground plane.</span>
    #[test]
    <span class="k">fn</span> the_plane_is_the_one_the_camera_faces() {
        assert_eq!(CPlane::facing(&amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>)), CPlane::Xy);
        assert_eq!(CPlane::facing(&amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)), CPlane::Xz);
        assert_eq!(CPlane::facing(&amp;Vector::new(-<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)), CPlane::Yz);
        <span class="c">// a slightly tilted view still picks the ground</span>
        assert_eq!(CPlane::facing(&amp;Vector::new(<span class="s">0</span>.<span class="s">6</span>, <span class="s">0</span>.<span class="s">1</span>, -<span class="s">0</span>.<span class="s">79</span>)), CPlane::Xy);
    }

    <span class="c">/// A ray straight down lands at the plane height.</span>
    #[test]
    <span class="k">fn</span> a_ray_lands_where_it_crosses() {
        <span class="k">let</span> hit = CPlane::Xy
            .hit(
                &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>),
                &amp;Point::new(<span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>),
                &amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>),
            )
            .expect(&quot;<span class="s">a hit</span>&quot;);
        assert!((hit[<span class="s">0</span>] - <span class="s">2</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
        assert!((hit[<span class="s">1</span>] - <span class="s">3</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
        assert!((hit[<span class="s">2</span>] - <span class="s">5</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
    }

    <span class="c">/// Parallel and backward rays give no hit.</span>
    #[test]
    <span class="k">fn</span> parallel_and_backward_rays_do_not_resolve() {
        <span class="k">let</span> origin = Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert!(
            CPlane::Xy
                .hit(
                    &amp;origin,
                    &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>),
                    &amp;Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)
                )
                .is_none(),
            &quot;<span class="s">parallel</span>&quot;
        );
        assert!(
            CPlane::Xy
                .hit(
                    &amp;origin,
                    &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>),
                    &amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)
                )
                .is_none(),
            &quot;<span class="s">pointing away</span>&quot;
        );
    }

    <span class="c">/// f64 keeps a millimetre a kilometre away.</span>
    #[test]
    <span class="k">fn</span> a_distant_plane_keeps_a_small_offset() {
        <span class="k">let</span> expected = <span class="s">1</span>.<span class="s">0</span>e<span class="s">6</span> + <span class="s">1</span>.<span class="s">0</span>e-<span class="s">3</span>;
        <span class="k">let</span> hit = CPlane::Xy
            .hit(
                &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, expected),
                &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>e<span class="s">7</span>),
                &amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>),
            )
            .expect(&quot;<span class="s">a hit</span>&quot;);
        assert!((hit[<span class="s">2</span>] - expected).abs() &lt; <span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>, &quot;<span class="s">kept, within rounding</span>&quot;);
        assert_eq!(expected <span class="k">as</span> f32, <span class="s">1</span>.<span class="s">0</span>e<span class="s">6_f32</span>, &quot;<span class="s">and f32 would have lost it</span>&quot;);
    }
}</code></pre></div>
<h2 id="step-4-srcappcoordsrs">Step 4 · src/app/coords.rs<a class="anchor" href="#/course/21-editing#step-4-srcappcoordsrs" aria-label="Link to this section">#</a></h2>
<p>Coordinate input accepts absolute, relative and polar values.</p>
<p><code>lessons/21/src/app/coords.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Turns a typed coordinate like 10,0,5 into a point, so the command line can place geometry exactly.</span>
<span class="k">use</span> session_rust::{Point, Vector};

<span class="c">/// A typed coordinate, not yet placed in the world.</span>
#[derive(Clone, Copy, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> Typed {
    Absolute { x: f64, y: f64, z: Option&lt;f64&gt; }, <span class="c">// \`1,2,3\` or \`1,2\` on the plane</span>
    Relative { x: f64, y: f64, z: Option&lt;f64&gt; }, <span class="c">// \`@1,2\` from the previous point</span>
    Polar { distance: f64, degrees: f64 }, <span class="c">// \`5&lt;45\` in the plane</span>
    Distance(f64), <span class="c">// \`5\` along the current direction</span>
}</code></pre></div>
<p><code>lessons/21/src/app/coords.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Parse coordinate text; None when it is not a coordinate.</span>
<span class="k">pub</span> <span class="k">fn</span> parse(text: &amp;str) -&gt; Option&lt;Typed&gt; {
    <span class="k">let</span> text = text.trim();

    <span class="k">if</span> text.is_empty() {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> (body, relative) = <span class="k">match</span> text.strip_prefix('<span class="s">@</span>') {
        Some(rest) =&gt; (rest.trim(), <span class="s">true</span>),
        None =&gt; (text, <span class="s">false</span>),
    };

    <span class="k">if</span> <span class="k">let</span> Some((d, a)) = body.split_once('<span class="s">&lt;</span>') {
        <span class="k">let</span> distance = number(d)?;
        <span class="k">let</span> degrees = number(a)?;
        <span class="k">return</span> Some(Typed::Polar { distance, degrees });
    }

    <span class="k">let</span> parts: Vec&lt;&amp;str&gt; = body.split('<span class="s">,</span>').map(str::trim).collect();

    <span class="k">match</span> parts.len() {
        <span class="s">1</span> <span class="k">if</span> relative =&gt; None, <span class="c">// \`@5\` has no direction</span>
        <span class="s">1</span> =&gt; Some(Typed::Distance(number(parts[<span class="s">0</span>])?)),
        <span class="s">2</span> | <span class="s">3</span> =&gt; {
            <span class="k">let</span> x = number(parts[<span class="s">0</span>])?;
            <span class="k">let</span> y = number(parts[<span class="s">1</span>])?;
            <span class="k">let</span> z = <span class="k">match</span> parts.get(<span class="s">2</span>) {
                Some(v) =&gt; Some(number(v)?),
                None =&gt; None,
            };
            Some(<span class="k">if</span> relative {
                Typed::Relative { x, y, z }
            } <span class="k">else</span> {
                Typed::Absolute { x, y, z }
            })
        }
        _ =&gt; None,
    }
}</code></pre></div>
<p><code>lessons/21/src/app/coords.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A typed coordinate as a world point on the plane.</span>
<span class="k">pub</span> <span class="k">fn</span> resolve(
    typed: Typed,
    origin: &amp;Point,
    x_axis: &amp;Vector,
    y_axis: &amp;Vector,
    previous: Option&lt;&amp;Point&gt;,
    along: Option&lt;&amp;Vector&gt;,
) -&gt; Option&lt;Point&gt; {
    <span class="c">// \`base\` plus u along x and v along y</span>
    <span class="k">let</span> on_plane = |u: f64, v: f64, base: &amp;Point| {
        Point::new(
            base[<span class="s">0</span>] + x_axis[<span class="s">0</span>] * u + y_axis[<span class="s">0</span>] * v,
            base[<span class="s">1</span>] + x_axis[<span class="s">1</span>] * u + y_axis[<span class="s">1</span>] * v,
            base[<span class="s">2</span>] + x_axis[<span class="s">2</span>] * u + y_axis[<span class="s">2</span>] * v,
        )
    };

    <span class="k">match</span> typed {
        Typed::Absolute { x, y, z: Some(z) } =&gt; Some(Point::new(x, y, z)),
        Typed::Absolute { x, y, z: None } =&gt; Some(on_plane(x, y, origin)),
        Typed::Relative { x, y, z: Some(z) } =&gt; {
            <span class="k">let</span> p = previous?;
            Some(Point::new(p[<span class="s">0</span>] + x, p[<span class="s">1</span>] + y, p[<span class="s">2</span>] + z))
        }
        Typed::Relative { x, y, z: None } =&gt; Some(on_plane(x, y, previous?)),
        Typed::Polar { distance, degrees } =&gt; {
            <span class="k">let</span> r = degrees.to_radians();
            <span class="k">let</span> base = previous.unwrap_or(origin);
            Some(on_plane(distance * r.cos(), distance * r.sin(), base))
        }
        Typed::Distance(d) =&gt; {
            <span class="k">let</span> p = previous?;
            <span class="k">let</span> v = along?;
            Some(Point::new(
                p[<span class="s">0</span>] + v[<span class="s">0</span>] * d,
                p[<span class="s">1</span>] + v[<span class="s">1</span>] * d,
                p[<span class="s">2</span>] + v[<span class="s">2</span>] * d,
            ))
        }
    }
}

<span class="c">/// A finite number, or None.</span>
<span class="k">fn</span> number(text: &amp;str) -&gt; Option&lt;f64&gt; {
    <span class="k">let</span> value: f64 = text.trim().parse().ok()?;
    value.is_finite().then_some(value)
}</code></pre></div>
<p><code>lessons/21/src/app/coords.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// The world XY plane.</span>
    <span class="k">fn</span> plane() -&gt; (Point, Vector, Vector) {
        (
            Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        )
    }

    <span class="c">/// Absolute, relative, polar and distance all parse.</span>
    #[test]
    <span class="k">fn</span> the_four_forms_parse() {
        assert_eq!(
            parse(&quot;<span class="s">12,4,2</span>&quot;),
            Some(Typed::Absolute {
                x: <span class="s">12</span>.<span class="s">0</span>,
                y: <span class="s">4</span>.<span class="s">0</span>,
                z: Some(<span class="s">2</span>.<span class="s">0</span>)
            })
        );
        assert_eq!(
            parse(&quot;<span class="s"> 12 , 4 </span>&quot;),
            Some(Typed::Absolute {
                x: <span class="s">12</span>.<span class="s">0</span>,
                y: <span class="s">4</span>.<span class="s">0</span>,
                z: None
            })
        );
        assert_eq!(
            parse(&quot;<span class="s">@3,0</span>&quot;),
            Some(Typed::Relative {
                x: <span class="s">3</span>.<span class="s">0</span>,
                y: <span class="s">0</span>.<span class="s">0</span>,
                z: None
            })
        );
        assert_eq!(
            parse(&quot;<span class="s">@5&lt;90</span>&quot;),
            Some(Typed::Polar {
                distance: <span class="s">5</span>.<span class="s">0</span>,
                degrees: <span class="s">90</span>.<span class="s">0</span>
            })
        );
        assert_eq!(parse(&quot;<span class="s">7.5</span>&quot;), Some(Typed::Distance(<span class="s">7</span>.<span class="s">5</span>)));
    }

    <span class="c">/// Words, extra parts and NaN are not coordinates.</span>
    #[test]
    <span class="k">fn</span> what_is_not_a_coordinate() {
        assert_eq!(parse(&quot;<span class="s">line</span>&quot;), None);
        assert_eq!(parse(&quot;&quot;), None);
        assert_eq!(parse(&quot;<span class="s">1,2,3,4</span>&quot;), None);
        assert_eq!(parse(&quot;<span class="s">@5</span>&quot;), None, &quot;<span class="s">no direction</span>&quot;);
        assert_eq!(parse(&quot;<span class="s">nan</span>&quot;), None);
        assert_eq!(parse(&quot;<span class="s">inf,0</span>&quot;), None);
    }

    <span class="c">/// Two numbers land on the plane, three in the world.</span>
    #[test]
    <span class="k">fn</span> two_numbers_are_on_the_plane_and_three_are_not() {
        <span class="k">let</span> lifted = Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">9</span>.<span class="s">0</span>);
        <span class="k">let</span> (_, x, y) = plane();
        <span class="k">let</span> on = resolve(parse(&quot;<span class="s">2,3</span>&quot;).unwrap(), &amp;lifted, &amp;x, &amp;y, None, None).unwrap();
        assert_eq!([on[<span class="s">0</span>], on[<span class="s">1</span>], on[<span class="s">2</span>]], [<span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">9</span>.<span class="s">0</span>]);
        <span class="k">let</span> world = resolve(parse(&quot;<span class="s">2,3,0</span>&quot;).unwrap(), &amp;lifted, &amp;x, &amp;y, None, None).unwrap();
        assert_eq!([world[<span class="s">0</span>], world[<span class="s">1</span>], world[<span class="s">2</span>]], [<span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
    }

    <span class="c">/// Relative and polar measure from the previous point.</span>
    #[test]
    <span class="k">fn</span> relative_forms_measure_from_the_previous_point() {
        <span class="k">let</span> (o, x, y) = plane();
        <span class="k">let</span> prev = Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> r = resolve(parse(&quot;<span class="s">@3,4</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, Some(&amp;prev), None).unwrap();
        assert_eq!([r[<span class="s">0</span>], r[<span class="s">1</span>]], [<span class="s">13</span>.<span class="s">0</span>, <span class="s">14</span>.<span class="s">0</span>]);
        <span class="k">let</span> p = resolve(parse(&quot;<span class="s">@5&lt;90</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, Some(&amp;prev), None).unwrap();
        assert!((p[<span class="s">0</span>] - <span class="s">10</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span> &amp;&amp; (p[<span class="s">1</span>] - <span class="s">15</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
        <span class="k">let</span> first = resolve(parse(&quot;<span class="s">@5&lt;0</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, None, None).unwrap();
        assert_eq!([first[<span class="s">0</span>], first[<span class="s">1</span>]], [<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
    }

    <span class="c">/// A relative form without a previous point gives nothing.</span>
    #[test]
    <span class="k">fn</span> a_missing_reference_does_not_resolve() {
        <span class="k">let</span> (o, x, y) = plane();
        assert!(resolve(parse(&quot;<span class="s">@3,4</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, None, None).is_none());
        assert!(resolve(parse(&quot;<span class="s">5</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, None, None).is_none());
        <span class="k">let</span> prev = Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert!(resolve(parse(&quot;<span class="s">5</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, Some(&amp;prev), None).is_none());
    }

    <span class="c">/// A millimetre typed a kilometre out is kept exactly.</span>
    #[test]
    <span class="k">fn</span> a_typed_coordinate_is_exact() {
        <span class="k">let</span> (o, x, y) = plane();
        <span class="k">let</span> p = resolve(parse(&quot;<span class="s">1000000.001,0,0</span>&quot;).unwrap(), &amp;o, &amp;x, &amp;y, None, None).unwrap();
        assert_eq!(p[<span class="s">0</span>], <span class="s">1_000_000</span>.<span class="s">001</span>);
    }
}</code></pre></div>
<h2 id="step-5-srccamerars">Step 5 · src/camera.rs<a class="anchor" href="#/course/21-editing#step-5-srccamerars" aria-label="Link to this section">#</a></h2>
<p>The camera owns orbit, pan, zoom and projection in the same file used by the finished viewer.</p>
<p><code>lessons/21/src/camera.rs</code> · edit · type this</p>
<p>Added after the <code>self.update_position();</code> line in <code>fn zoom</code> of <code>lessons/20/src/camera.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The world ray under a cursor pixel: origin and unit direction, scene units.</span>
    <span class="k">pub</span> <span class="k">fn</span> ray(&amp;<span class="k">self</span>, cursor: (f64, f64), viewport: (f64, f64)) -&gt; Option&lt;(Point, Vector)&gt; {
        <span class="c">// no ray for an empty viewport or a lost cursor</span>
        <span class="k">if</span> viewport.<span class="s">0</span> &lt;= <span class="s">0</span>.<span class="s">0</span> || viewport.<span class="s">1</span> &lt;= <span class="s">0</span>.<span class="s">0</span> || !cursor.<span class="s">0</span>.is_finite() || !cursor.<span class="s">1</span>.is_finite()
        {
            <span class="k">return</span> None;
        }

        <span class="c">// pixel to -1..1 on both axes, y up</span>
        <span class="k">let</span> ndc_x = <span class="s">2</span>.<span class="s">0</span> * cursor.<span class="s">0</span> / viewport.<span class="s">0</span> - <span class="s">1</span>.<span class="s">0</span>;
        <span class="k">let</span> ndc_y = <span class="s">1</span>.<span class="s">0</span> - <span class="s">2</span>.<span class="s">0</span> * cursor.<span class="s">1</span> / viewport.<span class="s">1</span>;
        <span class="c">// work in scene units, not meters</span>
        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();
        <span class="k">let</span> target = <span class="k">self</span>.origin();
        <span class="k">let</span> distance = <span class="k">self</span>.distance_world();
        <span class="c">// half the view size at the target plane</span>
        <span class="k">let</span> half_h = distance * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan();
        <span class="k">let</span> half_w = half_h * (viewport.<span class="s">0</span> / viewport.<span class="s">1</span>);
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());
        <span class="k">let</span> forward = <span class="k">self</span>.orientation.rotate_vector(Vector::y_axis());
        <span class="c">// the cursor's point on the target plane</span>
        <span class="k">let</span> <span class="k">mut</span> on_plane = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            on_plane[i] = target[i] + right[i] * ndc_x * half_w + <span class="k">self</span>.up[i] * ndc_y * half_h;
        }

        <span class="k">if</span> !<span class="k">self</span>.perspective {
            <span class="c">// orthographic: parallel rays, start behind the whole scene</span>
            <span class="k">let</span> back = distance + <span class="s">2</span>.<span class="s">0</span> * (<span class="k">self</span>.scene_extent / s).max(distance);
            <span class="k">let</span> origin = Point::new(
                on_plane[<span class="s">0</span>] - forward[<span class="s">0</span>] * back,
                on_plane[<span class="s">1</span>] - forward[<span class="s">1</span>] * back,
                on_plane[<span class="s">2</span>] - forward[<span class="s">2</span>] * back,
            );
            <span class="k">return</span> Some((origin, forward));
        }

        <span class="c">// perspective: every ray starts at the eye</span>
        <span class="k">let</span> eye = [
            <span class="k">self</span>.position[<span class="s">0</span>] / s,
            <span class="k">self</span>.position[<span class="s">1</span>] / s,
            <span class="k">self</span>.position[<span class="s">2</span>] / s,
        ];
        <span class="k">let</span> <span class="k">mut</span> dir = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            dir[i] = on_plane[i] - eye[i];
        }

        <span class="k">let</span> length = (dir[<span class="s">0</span>] * dir[<span class="s">0</span>] + dir[<span class="s">1</span>] * dir[<span class="s">1</span>] + dir[<span class="s">2</span>] * dir[<span class="s">2</span>]).sqrt();

        <span class="k">if</span> !length.is_finite() || length &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> None;
        }

        Some((
            Point::new(eye[<span class="s">0</span>], eye[<span class="s">1</span>], eye[<span class="s">2</span>]),
            Vector::new(dir[<span class="s">0</span>] / length, dir[<span class="s">1</span>] / length, dir[<span class="s">2</span>] / length),
        ))
    }

    <span class="c">/// Zoom so the point under the cursor stays under the cursor.</span></code></pre></div>
<p><code>lessons/21/src/camera.rs</code> · edit · type this</p>
<p>Added after the <code>mod wheel_tests {</code> line of <code>lessons/20/src/camera.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(test)]
    <span class="k">mod</span> ray_tests {
        <span class="k">use</span> super::*;

        <span class="k">fn</span> viewport() -&gt; (f64, f64) {
            (<span class="s">800</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>)
        }

        <span class="c">/// The center pixel looks along the view axis.</span>
        #[test]
        <span class="k">fn</span> the_centre_ray_is_the_view_axis() {
            <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
            cam.update_position();

            <span class="k">for</span> perspective <span class="k">in</span> [<span class="s">true</span>, <span class="s">false</span>] {
                cam.perspective = perspective;
                <span class="k">let</span> (_, dir) = cam.ray((<span class="s">400</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);
                <span class="k">let</span> forward = cam.orientation.rotate_vector(Vector::y_axis());

                <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                    assert!((dir[i] - forward[i]).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>, &quot;{<span class="s">perspective</span>}&quot;);
                }
            }
        }

        <span class="c">/// Perspective rays share the eye; orthographic rays share the direction.</span>
        #[test]
        <span class="k">fn</span> perspective_rays_share_an_origin_and_ortho_rays_share_a_direction() {
            <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
            cam.update_position();

            cam.perspective = <span class="s">true</span>;
            <span class="k">let</span> (a, da) = cam.ray((<span class="s">100</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);
            <span class="k">let</span> (b, db) = cam.ray((<span class="s">700</span>.<span class="s">0</span>, <span class="s">320</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                assert!((a[i] - b[i]).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">one eye</span>&quot;);
            }

            assert!(
                (<span class="s">0</span>..<span class="s">3</span>).any(|i| (da[i] - db[i]).abs() &gt; <span class="s">1</span>e-<span class="s">6</span>),
                &quot;<span class="s">different directions</span>&quot;
            );

            cam.perspective = <span class="s">false</span>;
            <span class="k">let</span> (a, da) = cam.ray((<span class="s">100</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);
            <span class="k">let</span> (b, db) = cam.ray((<span class="s">700</span>.<span class="s">0</span>, <span class="s">320</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                assert!((da[i] - db[i]).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>, &quot;<span class="s">one direction</span>&quot;);
            }

            assert!(
                (<span class="s">0</span>..<span class="s">3</span>).any(|i| (a[i] - b[i]).abs() &gt; <span class="s">1</span>e-<span class="s">6</span>),
                &quot;<span class="s">different origins</span>&quot;
            );
        }

        <span class="c">/// A ray through a pixel meets the target plane at that pixel's point.</span>
        #[test]
        <span class="k">fn</span> a_ray_hits_the_target_plane_where_the_cursor_is() {
            <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
            cam.update_position();
            <span class="c">// scene units on both sides</span>
            <span class="k">let</span> target = cam.origin();
            <span class="k">let</span> half_h = cam.distance_world() * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan();
            <span class="k">let</span> half_w = half_h * (viewport().<span class="s">0</span> / viewport().<span class="s">1</span>);
            <span class="k">let</span> right = cam.orientation.rotate_vector(Vector::x_axis());
            <span class="k">let</span> forward = cam.orientation.rotate_vector(Vector::y_axis());
            <span class="c">// a quarter right and a quarter up from the target</span>
            <span class="k">let</span> expected: Vec&lt;f64&gt; = (<span class="s">0</span>..<span class="s">3</span>)
                .map(|i| target[i] + right[i] * <span class="s">0</span>.<span class="s">5</span> * half_w + cam.up[i] * <span class="s">0</span>.<span class="s">5</span> * half_h)
                .collect();

            <span class="k">for</span> perspective <span class="k">in</span> [<span class="s">true</span>, <span class="s">false</span>] {
                cam.perspective = perspective;
                <span class="k">let</span> (origin, dir) = cam.ray((<span class="s">600</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>), viewport()).expect(&quot;<span class="s">a ray</span>&quot;);
                <span class="c">// walk the ray to the target plane</span>
                <span class="k">let</span> denom: f64 = (<span class="s">0</span>..<span class="s">3</span>).map(|i| dir[i] * forward[i]).sum();
                <span class="k">let</span> num: f64 = (<span class="s">0</span>..<span class="s">3</span>).map(|i| (target[i] - origin[i]) * forward[i]).sum();
                <span class="k">let</span> t = num / denom;

                <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                    <span class="k">let</span> hit = origin[i] + dir[i] * t;
                    assert!((hit - expected[i]).abs() &lt; <span class="s">1</span>e-<span class="s">6</span>, &quot;{<span class="s">perspective</span>}<span class="s"> axis </span>{<span class="s">i</span>}&quot;);
                }
            }
        }

        <span class="c">/// The ray is in scene units, not the camera's meters.</span>
        #[test]
        <span class="k">fn</span> the_ray_is_in_world_units_not_the_camera_s_metres() {
            <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
            cam.update_position();
            <span class="c">// the center pixel looks straight at the target</span>
            <span class="k">let</span> (origin, dir) = cam
                .ray((viewport().<span class="s">0</span> * <span class="s">0</span>.<span class="s">5</span>, viewport().<span class="s">1</span> * <span class="s">0</span>.<span class="s">5</span>), viewport())
                .expect(&quot;<span class="s">a ray</span>&quot;);
            <span class="k">let</span> target = cam.origin();
            <span class="k">let</span> reach: f64 = (<span class="s">0</span>..<span class="s">3</span>).map(|i| (target[i] - origin[i]) * dir[i]).sum();
            assert!(
                (reach - cam.distance_world()).abs() &lt; <span class="s">1</span>e-<span class="s">6</span>,
                &quot;<span class="s">the eye is distance_world from the target, in world units</span>&quot;
            );
            assert!(
                cam.distance_world() &gt; cam.distance * <span class="s">100</span>.<span class="s">0</span>,
                &quot;<span class="s">the fixture is a millimetre scene, so the two really do differ</span>&quot;
            );
        }

        <span class="c">/// An empty viewport gives no ray.</span>
        #[test]
        <span class="k">fn</span> a_degenerate_viewport_has_no_ray() {
            <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
            cam.update_position();
            assert!(cam.ray((<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>), (<span class="s">0</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>)).is_none());
            assert!(cam.ray((f64::NAN, <span class="s">1</span>.<span class="s">0</span>), viewport()).is_none());
        }
    }</code></pre></div>
<h2 id="step-6-srcappgizmors">Step 6 · src/app/gizmo.rs<a class="anchor" href="#/course/21-editing#step-6-srcappgizmors" aria-label="Link to this section">#</a></h2>
<p>The gumball computes translation, rotation and scale about the selected object.</p>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The handles drawn on the selection; dragging one moves, rotates or scales along a single axis.</span>
<span class="k">use</span> session_rust::intersection::{line_line_parameters, line_plane};
<span class="k">use</span> session_rust::{Line, Plane, Point, Vector, Xform};

<span class="c">/// The one length everything else is a fraction of, in CSS pixels.</span>
<span class="k">pub</span> <span class="k">const</span> ARM: f64 = <span class="s">72</span>.<span class="s">0</span>;

<span class="c">/// Distance of the scale balls along each arm.</span>
<span class="k">pub</span> <span class="k">const</span> BALL_AT: f64 = ARM * <span class="s">0</span>.<span class="s">5</span>;

<span class="c">/// Grab radius in pixels.</span>
<span class="k">const</span> GRAB: f64 = <span class="s">8</span>.<span class="s">0</span>;

<span class="c">/// Radius of the centre ball.</span>
<span class="k">pub</span> <span class="k">const</span> HUB: f64 = <span class="s">6</span>.<span class="s">0</span>;

<span class="c">/// Exponent that slows a scale drag near the centre.</span>
<span class="k">const</span> SCALE_SOFTENING: f64 = <span class="s">0</span>.<span class="s">5</span>;

<span class="c">/// Smallest scale factor allowed.</span>
<span class="k">const</span> MIN_SCALE: f64 = <span class="s">0</span>.<span class="s">01</span>;

<span class="c">/// One grabbable part of the gizmo.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> Handle {
    Translate(Axis), <span class="c">// an arm</span>
    Rotate(Axis), <span class="c">// an arc</span>
    Scale(Axis), <span class="c">// a ball on an arm</span>
    ScaleUniform, <span class="c">// the centre ball</span>
}

<span class="c">/// A world axis.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> Axis {
    X,
    Y,
    Z,
}</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Axis {
    <span class="c">/// The unit vector along this axis.</span>
    <span class="k">pub</span> <span class="k">fn</span> unit(<span class="k">self</span>) -&gt; Vector {
        <span class="k">match</span> <span class="k">self</span> {
            Axis::X =&gt; Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Axis::Y =&gt; Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            Axis::Z =&gt; Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
        }
    }

    <span class="c">/// The other two axes.</span>
    <span class="k">fn</span> others(<span class="k">self</span>) -&gt; (Vector, Vector) {
        <span class="k">match</span> <span class="k">self</span> {
            Axis::X =&gt; (Axis::Y.unit(), Axis::Z.unit()),
            Axis::Y =&gt; (Axis::Z.unit(), Axis::X.unit()),
            Axis::Z =&gt; (Axis::X.unit(), Axis::Y.unit()),
        }
    }
}

<span class="k">impl</span> Handle {
    <span class="c">/// Verb, axis name and unit for the number box.</span>
    <span class="k">pub</span> <span class="k">fn</span> labels(<span class="k">self</span>) -&gt; (&amp;'static str, &amp;'static str, &amp;'static str) {
        <span class="k">let</span> name = |a: Axis| <span class="k">match</span> a {
            Axis::X =&gt; &quot;<span class="s">X</span>&quot;,
            Axis::Y =&gt; &quot;<span class="s">Y</span>&quot;,
            Axis::Z =&gt; &quot;<span class="s">Z</span>&quot;,
        };

        <span class="k">match</span> <span class="k">self</span> {
            Handle::Translate(a) =&gt; (&quot;<span class="s">Move</span>&quot;, name(a), &quot;<span class="s">mm</span>&quot;),
            Handle::Rotate(a) =&gt; (&quot;<span class="s">Rotate</span>&quot;, name(a), &quot;<span class="s">deg</span>&quot;),
            Handle::Scale(a) =&gt; (&quot;<span class="s">Scale</span>&quot;, name(a), &quot;<span class="s">factor</span>&quot;),
            Handle::ScaleUniform =&gt; (&quot;<span class="s">Scale</span>&quot;, &quot;&quot;, &quot;<span class="s">factor</span>&quot;),
        }
    }
}

<span class="c">/// What a drag remembers from its grab.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Drag {
    <span class="k">pub</span> handle: Handle, <span class="c">// the handle being dragged</span>
    grabbed: Point,
    angle: f64, <span class="c">// grab angle about the axis, for rotate</span>
    reach: f64, <span class="c">// grab distance from the centre, for scale</span>
    plane: Vector, <span class="c">// plane normal a uniform scale is measured in</span>
}

<span class="c">/// The gizmo's position and state.</span>
<span class="k">pub</span> <span class="k">struct</span> Gizmo {
    <span class="k">pub</span> origin: Point, <span class="c">// centre in world</span>
    <span class="k">pub</span> hovered: Option&lt;Handle&gt;, <span class="c">// handle under the pointer</span>
    <span class="k">pub</span> drag: Option&lt;Drag&gt;,
}</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Gizmo {
    <span class="c">/// A gizmo at \`origin\`, idle.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(origin: Point) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            origin,
            hovered: None,
            drag: None,
        }
    }

    <span class="c">/// Move the gizmo and end any drag.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_origin(&amp;<span class="k">mut</span> <span class="k">self</span>, origin: Point) {
        <span class="k">self</span>.origin = origin;
        <span class="k">self</span>.hovered = None;
        <span class="k">self</span>.drag = None;
    }</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The handle under a ray, tested from the centre outward.</span>
    <span class="k">pub</span> <span class="k">fn</span> hit(&amp;<span class="k">self</span>, from: &amp;Point, dir: &amp;Vector, world_per_px: f64) -&gt; Option&lt;Handle&gt; {
        <span class="k">let</span> s = world_per_px; <span class="c">// pixel sizes to world</span>

        <span class="k">if</span> within(from, dir, &amp;<span class="k">self</span>.origin, HUB * s) {
            <span class="k">return</span> Some(Handle::ScaleUniform);
        }

        <span class="k">for</span> axis <span class="k">in</span> [Axis::X, Axis::Y, Axis::Z] {
            <span class="k">if</span> end_on(dir, axis) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> at = &amp;<span class="k">self</span>.origin + &amp;(&amp;axis.unit() * (BALL_AT * s));

            <span class="k">if</span> within(from, dir, &amp;at, GRAB * s) {
                <span class="k">return</span> Some(Handle::Scale(axis));
            }
        }

        <span class="k">for</span> axis <span class="k">in</span> [Axis::X, Axis::Y, Axis::Z] {
            <span class="k">if</span> end_on(dir, axis) {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> <span class="k">let</span> Some(p) = closest_on_axis(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit()) {
                <span class="k">let</span> t = (&amp;p - &amp;<span class="k">self</span>.origin).dot(&amp;axis.unit());

                <span class="c">// The arm's grabbable run starts where the hub ends: inside it all three arms</span>
                <span class="k">if</span> (HUB * s..=ARM * s).contains(&amp;t) &amp;&amp; within(from, dir, &amp;p, GRAB * s) {
                    <span class="k">return</span> Some(Handle::Translate(axis));
                }
            }
        }

        <span class="c">// the rotate arcs, in the quadrant without arms</span>
        <span class="k">for</span> axis <span class="k">in</span> [Axis::X, Axis::Y, Axis::Z] {
            <span class="k">if</span> <span class="k">let</span> Some(p) = plane_hit(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit()) {
                <span class="k">let</span> d = &amp;p - &amp;<span class="k">self</span>.origin;
                <span class="k">let</span> (u, v) = axis.others();

                <span class="c">// A quarter arc, in the quadrant the arms and balls do not occupy. Sharing a</span>
                <span class="k">if</span> d.dot(&amp;u) &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; d.dot(&amp;v) &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; (d.magnitude() - ARM * s).abs() &lt; GRAB * s
                {
                    <span class="k">return</span> Some(Handle::Rotate(axis));
                }
            }
        }

        None
    }</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Start a drag on \`handle\`; None when the ray runs along the axis.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin(&amp;<span class="k">mut</span> <span class="k">self</span>, handle: Handle, from: &amp;Point, dir: &amp;Vector) -&gt; Option&lt;Drag&gt; {
        <span class="k">let</span> drag = <span class="k">match</span> handle {
            Handle::Translate(axis) =&gt; Drag {
                handle,
                grabbed: closest_on_axis(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?,
                angle: <span class="s">0</span>.<span class="s">0</span>,
                reach: <span class="s">1</span>.<span class="s">0</span>,
                plane: axis.unit(),
            },
            Handle::Rotate(axis) =&gt; Drag {
                handle,
                grabbed: <span class="k">self</span>.origin.clone(),
                angle: angle_in_plane(
                    &amp;plane_hit(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?,
                    &amp;<span class="k">self</span>.origin,
                    axis,
                ),
                reach: <span class="s">1</span>.<span class="s">0</span>,
                plane: axis.unit(),
            },
            Handle::Scale(axis) =&gt; {
                <span class="k">let</span> p = closest_on_axis(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?;
                <span class="k">let</span> reach = (&amp;p - &amp;<span class="k">self</span>.origin).dot(&amp;axis.unit());
                Drag {
                    handle,
                    grabbed: p,
                    angle: <span class="s">0</span>.<span class="s">0</span>,
                    reach: nonzero(reach),
                    plane: axis.unit(),
                }
            }
            Handle::ScaleUniform =&gt; {
                <span class="k">let</span> normal = facing(dir); <span class="c">// measure in the plane facing the view</span>
                <span class="k">let</span> p = plane_hit(from, dir, &amp;<span class="k">self</span>.origin, &amp;normal)?;
                <span class="k">let</span> reach = (&amp;p - &amp;<span class="k">self</span>.origin).magnitude();
                Drag {
                    handle,
                    grabbed: p,
                    angle: <span class="s">0</span>.<span class="s">0</span>,
                    reach: nonzero(reach),
                    plane: normal,
                }
            }
        };
        <span class="k">self</span>.drag = Some(drag.clone());
        Some(drag)
    }</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The transform from the grab to where the ray is now.</span>
    <span class="k">pub</span> <span class="k">fn</span> update(&amp;<span class="k">self</span>, drag: &amp;Drag, from: &amp;Point, dir: &amp;Vector) -&gt; Option&lt;Xform&gt; {
        <span class="k">match</span> drag.handle {
            Handle::Translate(axis) =&gt; {
                <span class="k">let</span> now = closest_on_axis(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?;
                <span class="k">let</span> d = &amp;now - &amp;drag.grabbed;
                Some(Xform::translation(d[<span class="s">0</span>], d[<span class="s">1</span>], d[<span class="s">2</span>]))
            }
            Handle::Rotate(axis) =&gt; {
                <span class="k">let</span> now = plane_hit(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?;
                <span class="k">let</span> turned = angle_in_plane(&amp;now, &amp;<span class="k">self</span>.origin, axis) - drag.angle;
                Some(about(&amp;<span class="k">self</span>.origin, rotation(axis, turned)))
            }
            Handle::Scale(axis) =&gt; {
                <span class="k">let</span> now = closest_on_axis(from, dir, &amp;<span class="k">self</span>.origin, &amp;axis.unit())?;
                <span class="k">let</span> reach = (&amp;now - &amp;<span class="k">self</span>.origin).dot(&amp;axis.unit());
                <span class="k">let</span> k = softened(reach / drag.reach);
                <span class="k">let</span> (x, y, z) = <span class="k">match</span> axis {
                    Axis::X =&gt; (k, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
                    Axis::Y =&gt; (<span class="s">1</span>.<span class="s">0</span>, k, <span class="s">1</span>.<span class="s">0</span>),
                    Axis::Z =&gt; (<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, k),
                };
                Some(about(&amp;<span class="k">self</span>.origin, Xform::scale_xyz(x, y, z)))
            }
            Handle::ScaleUniform =&gt; {
                <span class="k">let</span> now = plane_hit(from, dir, &amp;<span class="k">self</span>.origin, &amp;drag.plane)?;
                <span class="k">let</span> k = softened((&amp;now - &amp;<span class="k">self</span>.origin).magnitude() / drag.reach);
                Some(about(&amp;<span class="k">self</span>.origin, Xform::scale_xyz(k, k, k)))
            }
        }
    }

    <span class="c">/// The transform for a typed number: mm, degrees or a factor.</span>
    <span class="k">pub</span> <span class="k">fn</span> typed(&amp;<span class="k">self</span>, handle: Handle, value: f64) -&gt; Xform {
        <span class="k">match</span> handle {
            Handle::Translate(axis) =&gt; {
                <span class="k">let</span> u = axis.unit();
                Xform::translation(u[<span class="s">0</span>] * value, u[<span class="s">1</span>] * value, u[<span class="s">2</span>] * value)
            }
            Handle::Rotate(axis) =&gt; about(&amp;<span class="k">self</span>.origin, rotation(axis, value.to_radians())),
            Handle::Scale(axis) =&gt; {
                <span class="k">let</span> k = value.max(MIN_SCALE);
                <span class="k">let</span> (x, y, z) = <span class="k">match</span> axis {
                    Axis::X =&gt; (k, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
                    Axis::Y =&gt; (<span class="s">1</span>.<span class="s">0</span>, k, <span class="s">1</span>.<span class="s">0</span>),
                    Axis::Z =&gt; (<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, k),
                };
                about(&amp;<span class="k">self</span>.origin, Xform::scale_xyz(x, y, z))
            }
            Handle::ScaleUniform =&gt; {
                <span class="k">let</span> k = value.max(MIN_SCALE);
                about(&amp;<span class="k">self</span>.origin, Xform::scale_xyz(k, k, k))
            }
        }
    }
}</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Ray geometry</span>

<span class="c">/// A scale factor slowed near the centre, never below the minimum.</span>
<span class="k">fn</span> softened(ratio: f64) -&gt; f64 {
    <span class="k">if</span> !ratio.is_finite() || ratio &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> MIN_SCALE;
    }

    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

<span class="c">/// \`v\`, or a tiny value with its sign when zero.</span>
<span class="k">fn</span> nonzero(v: f64) -&gt; f64 {
    <span class="k">if</span> v.abs() &lt; <span class="s">1</span>e-<span class="s">9</span> {
        <span class="s">1</span>e-<span class="s">9_f64</span>.copysign(<span class="k">if</span> v &lt; <span class="s">0</span>.<span class="s">0</span> { -<span class="s">1</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">1</span>.<span class="s">0</span> })
    } <span class="k">else</span> {
        v
    }
}

<span class="c">/// The world axis a ray runs most along.</span>
<span class="k">fn</span> facing(dir: &amp;Vector) -&gt; Vector {
    <span class="k">let</span> (x, y, z) = (dir[<span class="s">0</span>].abs(), dir[<span class="s">1</span>].abs(), dir[<span class="s">2</span>].abs());

    <span class="k">if</span> x &gt;= y &amp;&amp; x &gt;= z {
        Axis::X.unit()
    } <span class="k">else</span> <span class="k">if</span> y &gt;= z {
        Axis::Y.unit()
    } <span class="k">else</span> {
        Axis::Z.unit()
    }
}

<span class="c">/// True when the ray looks almost straight along the axis.</span>
<span class="k">fn</span> end_on(dir: &amp;Vector, axis: Axis) -&gt; bool {
    dir.dot(&amp;axis.unit()).abs() &gt; <span class="s">0</span>.<span class="s">97</span>
}

<span class="c">/// The point on the axis closest to the ray, \`None\` when the two run parallel.</span>
<span class="k">fn</span> closest_on_axis(from: &amp;Point, dir: &amp;Vector, origin: &amp;Point, axis: &amp;Vector) -&gt; Option&lt;Point&gt; {
    <span class="k">let</span> ray = Line::from_point_direction_length(from, dir, <span class="s">1</span>.<span class="s">0</span>);
    <span class="k">let</span> line = Line::from_point_direction_length(origin, axis, <span class="s">1</span>.<span class="s">0</span>);
    <span class="k">let</span> (_, t) = line_line_parameters(&amp;ray, &amp;line, <span class="s">1</span>e-<span class="s">9</span>, <span class="s">false</span>, <span class="s">false</span>)?;

    Some(origin + &amp;(axis * t))
}

<span class="c">/// Where a ray hits the plane through \`origin\`, in front of the eye.</span>
<span class="k">fn</span> plane_hit(from: &amp;Point, dir: &amp;Vector, origin: &amp;Point, normal: &amp;Vector) -&gt; Option&lt;Point&gt; {
    <span class="k">if</span> dir.dot(normal).abs() &lt; <span class="s">1</span>e-<span class="s">9</span> {
        <span class="k">return</span> None; <span class="c">// ray parallel to the plane</span>
    }

    <span class="k">let</span> ray = Line::from_point_direction_length(from, dir, <span class="s">1</span>.<span class="s">0</span>);
    <span class="k">let</span> plane = Plane::from_point_normal(origin.clone(), normal.clone(), Some(<span class="s">false</span>));
    <span class="k">let</span> hit = line_plane(&amp;ray, &amp;plane, <span class="s">false</span>)?;
    <span class="k">let</span> t = (&amp;hit - from).dot(dir);

    (t &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; t.is_finite()).then_some(hit)
}

<span class="c">/// True when the ray passes within \`radius\` of \`at\`.</span>
<span class="k">fn</span> within(from: &amp;Point, dir: &amp;Vector, at: &amp;Point, radius: f64) -&gt; bool {
    <span class="k">let</span> t = (at - from).dot(dir);

    <span class="k">if</span> t &lt; <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> closest = from + &amp;(dir * t);
    (&amp;closest - at).magnitude() &lt;= radius
}

<span class="c">/// Angle of \`p\` around \`axis\`, seen from \`origin\`.</span>
<span class="k">fn</span> angle_in_plane(p: &amp;Point, origin: &amp;Point, axis: Axis) -&gt; f64 {
    <span class="k">let</span> (u, v) = axis.others();
    <span class="k">let</span> d = p - origin;
    d.dot(&amp;v).atan2(d.dot(&amp;u))
}

<span class="c">/// A rotation about one world axis, in radians.</span>
<span class="k">fn</span> rotation(axis: Axis, radians: f64) -&gt; Xform {
    <span class="k">match</span> axis {
        Axis::X =&gt; Xform::rotation_x(radians, <span class="s">false</span>),
        Axis::Y =&gt; Xform::rotation_y(radians, <span class="s">false</span>),
        Axis::Z =&gt; Xform::rotation_z(radians, <span class="s">false</span>),
    }
}

<span class="c">/// \`m\` applied about \`pivot\` instead of the origin.</span>
<span class="k">fn</span> about(pivot: &amp;Point, m: Xform) -&gt; Xform {
    <span class="k">let</span> to = Xform::translation(pivot[<span class="s">0</span>], pivot[<span class="s">1</span>], pivot[<span class="s">2</span>]);
    <span class="k">let</span> back = Xform::translation(-pivot[<span class="s">0</span>], -pivot[<span class="s">1</span>], -pivot[<span class="s">2</span>]);

    &amp;(&amp;to * &amp;m) * &amp;back
}</code></pre></div>
<p><code>lessons/21/src/app/gizmo.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="k">const</span> SCALE: f64 = <span class="s">1</span>.<span class="s">0</span>; <span class="c">// one world unit per pixel</span>

    <span class="c">/// A gizmo at the origin.</span>
    <span class="k">fn</span> at_origin() -&gt; Gizmo {
        Gizmo::new(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>))
    }

    <span class="c">/// A ray straight down through (x, y).</span>
    <span class="k">fn</span> down(x: f64, y: f64) -&gt; (Point, Vector) {
        (Point::new(x, y, <span class="s">500</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>))
    }

    <span class="c">/// Hub, ball, arm and arc each answer at their own place.</span>
    #[test]
    <span class="k">fn</span> the_handles_do_not_shadow_each_other() {
        <span class="k">let</span> g = at_origin();
        <span class="k">let</span> (f, d) = down(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), Some(Handle::ScaleUniform));
        <span class="k">let</span> (f, d) = down(BALL_AT, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), Some(Handle::Scale(Axis::X)));
        <span class="k">let</span> (f, d) = down(ARM * <span class="s">0</span>.<span class="s">8</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), Some(Handle::Translate(Axis::X)));
        <span class="c">// the arcs live in the quadrant the arms do not</span>
        <span class="k">let</span> r = ARM / <span class="s">2</span>.<span class="s">0_f64</span>.sqrt();
        <span class="k">let</span> (f, d) = down(-r, -r);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), Some(Handle::Rotate(Axis::Z)));
        <span class="k">let</span> (f, d) = down(ARM, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(
            g.hit(&amp;f, &amp;d, SCALE),
            Some(Handle::Translate(Axis::X)),
            &quot;<span class="s">the arm tip is not an arc</span>&quot;
        );
        <span class="k">let</span> (f, d) = down(ARM * <span class="s">3</span>.<span class="s">0</span>, ARM * <span class="s">3</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), None);
    }

    <span class="c">/// An axis pointing at the eye cannot be grabbed.</span>
    #[test]
    <span class="k">fn</span> an_axis_seen_end_on_is_not_grabbable() {
        <span class="k">let</span> g = at_origin();
        <span class="c">// just past the hub, where only the Z ball could answer</span>
        <span class="k">let</span> (f, d) = down(-<span class="s">7</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), None);
        <span class="k">let</span> (f, d) = down(BALL_AT, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, SCALE), Some(Handle::Scale(Axis::X)));
    }

    <span class="c">/// Handles keep their pixel size at any zoom.</span>
    #[test]
    <span class="k">fn</span> the_widget_is_screen_constant() {
        <span class="k">let</span> g = at_origin();
        <span class="k">let</span> (f, d) = down(BALL_AT * <span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(g.hit(&amp;f, &amp;d, <span class="s">2</span>.<span class="s">0</span>), Some(Handle::Scale(Axis::X)));
        assert_ne!(g.hit(&amp;f, &amp;d, <span class="s">1</span>.<span class="s">0</span>), Some(Handle::Scale(Axis::X)));
    }

    <span class="c">/// A translate drag moves only along its axis.</span>
    #[test]
    <span class="k">fn</span> a_translate_drag_moves_along_its_axis_only() {
        <span class="k">let</span> <span class="k">mut</span> g = at_origin();
        <span class="k">let</span> (f0, d0) = down(<span class="s">30</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> drag = g
            .begin(Handle::Translate(Axis::X), &amp;f0, &amp;d0)
            .expect(&quot;<span class="s">grabbed</span>&quot;);
        <span class="k">let</span> (f1, d1) = down(<span class="s">42</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> m = g.update(&amp;drag, &amp;f1, &amp;d1).expect(&quot;<span class="s">a transform</span>&quot;);
        assert!((m.m[<span class="s">12</span>] - <span class="s">12</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
        assert!(m.m[<span class="s">13</span>].abs() &lt; <span class="s">1</span>e-<span class="s">9</span> &amp;&amp; m.m[<span class="s">14</span>].abs() &lt; <span class="s">1</span>e-<span class="s">9</span>);
    }

    <span class="c">/// A drag along the viewing direction is refused.</span>
    #[test]
    <span class="k">fn</span> a_drag_down_its_own_axis_refuses() {
        <span class="k">let</span> <span class="k">mut</span> g = at_origin();
        <span class="k">let</span> from = Point::new(-<span class="s">500</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> dir = Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert!(g.begin(Handle::Translate(Axis::X), &amp;from, &amp;dir).is_none());
    }

    <span class="c">/// A quarter turn about Z takes the x axis onto the y axis.</span>
    #[test]
    <span class="k">fn</span> a_rotate_drag_turns_by_the_angle_swept() {
        <span class="k">let</span> <span class="k">mut</span> g = at_origin();
        <span class="k">let</span> (f0, d0) = down(ARM, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> drag = g.begin(Handle::Rotate(Axis::Z), &amp;f0, &amp;d0).expect(&quot;<span class="s">grabbed</span>&quot;);
        <span class="k">let</span> (f1, d1) = down(<span class="s">0</span>.<span class="s">0</span>, ARM);
        <span class="k">let</span> m = g.update(&amp;drag, &amp;f1, &amp;d1).expect(&quot;<span class="s">a transform</span>&quot;);
        <span class="c">// column 0 is the image of the x axis</span>
        assert!((m.m[<span class="s">0</span>]).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">x.x</span>&quot;);
        assert!((m.m[<span class="s">1</span>] - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">x.y</span>&quot;);
    }

    <span class="c">/// A scale starts at 1 and never flips.</span>
    #[test]
    <span class="k">fn</span> a_scale_is_relative_to_the_grab_and_never_collapses() {
        <span class="k">let</span> <span class="k">mut</span> g = at_origin();
        <span class="k">let</span> (f0, d0) = down(BALL_AT, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> drag = g.begin(Handle::Scale(Axis::X), &amp;f0, &amp;d0).expect(&quot;<span class="s">grabbed</span>&quot;);
        <span class="k">let</span> m = g.update(&amp;drag, &amp;f0, &amp;d0).expect(&quot;<span class="s">a transform</span>&quot;);
        assert!((m.m[<span class="s">0</span>] - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">no movement is no change</span>&quot;);

        <span class="k">let</span> (f1, d1) = down(BALL_AT * <span class="s">4</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> grown = g.update(&amp;drag, &amp;f1, &amp;d1).expect(&quot;<span class="s">a transform</span>&quot;);
        assert!(
            grown.m[<span class="s">0</span>] &gt; <span class="s">1</span>.<span class="s">0</span> &amp;&amp; grown.m[<span class="s">5</span>] == <span class="s">1</span>.<span class="s">0</span> &amp;&amp; grown.m[<span class="s">10</span>] == <span class="s">1</span>.<span class="s">0</span>,
            &quot;<span class="s">one axis only</span>&quot;
        );

        <span class="k">let</span> (f2, d2) = down(-BALL_AT * <span class="s">4</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> flipped = g.update(&amp;drag, &amp;f2, &amp;d2).expect(&quot;<span class="s">a transform</span>&quot;);
        assert!(flipped.m[<span class="s">0</span>] &gt;= MIN_SCALE, &quot;<span class="s">never mirrors</span>&quot;);
    }

    <span class="c">/// A point at the gizmo centre does not move.</span>
    #[test]
    <span class="k">fn</span> transforms_act_about_the_gumball() {
        <span class="k">let</span> g = Gizmo::new(Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>));

        <span class="k">for</span> m <span class="k">in</span> [
            g.typed(Handle::Rotate(Axis::Z), <span class="s">90</span>.<span class="s">0</span>),
            g.typed(Handle::ScaleUniform, <span class="s">3</span>.<span class="s">0</span>),
        ] {
            <span class="k">let</span> p = Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>);
            <span class="k">let</span> moved = m.transform_point(&amp;p);

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                assert!((moved[i] - p[i]).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">axis </span>{<span class="s">i</span>}&quot;);
            }
        }
    }

    <span class="c">/// Typed values give the same transforms as drags.</span>
    #[test]
    <span class="k">fn</span> typed_values_match_the_drag_and_are_clamped() {
        <span class="k">let</span> g = at_origin();
        <span class="k">let</span> m = g.typed(Handle::Translate(Axis::Y), <span class="s">12</span>.<span class="s">5</span>);
        assert_eq!([m.m[<span class="s">12</span>], m.m[<span class="s">13</span>], m.m[<span class="s">14</span>]], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">12</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">0</span>]);
        <span class="k">let</span> z = g.typed(Handle::ScaleUniform, <span class="s">0</span>.<span class="s">0</span>);
        assert!(z.m[<span class="s">0</span>] &gt;= MIN_SCALE &amp;&amp; z.m[<span class="s">5</span>] &gt;= MIN_SCALE &amp;&amp; z.m[<span class="s">10</span>] &gt;= MIN_SCALE);
    }
}</code></pre></div>
<h2 id="step-7-srcappsnaprs">Step 7 · src/app/snap.rs<a class="anchor" href="#/course/21-editing#step-7-srcappsnaprs" aria-label="Link to this section">#</a></h2>
<p>Snapping chooses nearby source positions in screen space.</p>
<p><code>lessons/21/src/app/snap.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Pulls the cursor onto a meaningful point - an end, a vertex, a midpoint - so drawings connect exactly instead of nearly.</span>
<span class="k">use</span> session_rust::Point;

<span class="c">/// Kinds of snap point, best first.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
<span class="k">pub</span> <span class="k">enum</span> SnapKind {
    End, <span class="c">// end of an open line</span>
    Vertex, <span class="c">// interior or loop vertex</span>
    Mid, <span class="c">// middle of a segment</span>
    Center,
    Near, <span class="c">// nearest point on a segment</span>
}

<span class="c">/// One snap candidate.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Snap {
    <span class="k">pub</span> point: Point, <span class="c">// where it is</span>
    <span class="k">pub</span> kind: SnapKind, <span class="c">// what it is</span>
    <span class="k">pub</span> owner: u32, <span class="c">// object row it belongs to</span>
}</code></pre></div>
<p><code>lessons/21/src/app/snap.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Add the ends, vertices and midpoints of a polyline.</span>
<span class="k">pub</span> <span class="k">fn</span> from_polyline(points: &amp;[Point], closed: bool, owner: u32, out: &amp;<span class="k">mut</span> Vec&lt;Snap&gt;) {
    <span class="k">if</span> points.is_empty() {
        <span class="k">return</span>;
    }

    <span class="k">let</span> last = points.len() - <span class="s">1</span>;

    <span class="k">for</span> (i, p) <span class="k">in</span> points.iter().enumerate() {
        <span class="k">let</span> interior = i != <span class="s">0</span> &amp;&amp; i != last;
        <span class="k">let</span> kind = <span class="k">if</span> closed || interior {
            SnapKind::Vertex
        } <span class="k">else</span> {
            SnapKind::End
        };
        out.push(Snap {
            point: p.clone(),
            kind,
            owner,
        });
    }

    <span class="k">let</span> spans = <span class="k">if</span> closed { points.len() } <span class="k">else</span> { last }; <span class="c">// segment count</span>

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..spans {
        <span class="k">let</span> a = &amp;points[i];
        <span class="k">let</span> b = &amp;points[(i + <span class="s">1</span>) % points.len()];
        out.push(Snap {
            point: Point::new(
                (a[<span class="s">0</span>] + b[<span class="s">0</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (a[<span class="s">1</span>] + b[<span class="s">1</span>]) * <span class="s">0</span>.<span class="s">5</span>,
                (a[<span class="s">2</span>] + b[<span class="s">2</span>]) * <span class="s">0</span>.<span class="s">5</span>,
            ),
            kind: SnapKind::Mid,
            owner,
        });
    }
}</code></pre></div>
<p><code>lessons/21/src/app/snap.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The point of segment \`a\`-\`b\` nearest to \`to\`.</span>
<span class="k">pub</span> <span class="k">fn</span> nearest_on_segment(a: &amp;Point, b: &amp;Point, to: &amp;Point, owner: u32) -&gt; Snap {
    <span class="k">let</span> ab = [b[<span class="s">0</span>] - a[<span class="s">0</span>], b[<span class="s">1</span>] - a[<span class="s">1</span>], b[<span class="s">2</span>] - a[<span class="s">2</span>]];
    <span class="k">let</span> ap = [to[<span class="s">0</span>] - a[<span class="s">0</span>], to[<span class="s">1</span>] - a[<span class="s">1</span>], to[<span class="s">2</span>] - a[<span class="s">2</span>]];
    <span class="k">let</span> len2 = ab[<span class="s">0</span>] * ab[<span class="s">0</span>] + ab[<span class="s">1</span>] * ab[<span class="s">1</span>] + ab[<span class="s">2</span>] * ab[<span class="s">2</span>];
    <span class="k">let</span> t = <span class="k">if</span> len2 &gt; <span class="s">0</span>.<span class="s">0</span> {
        ((ap[<span class="s">0</span>] * ab[<span class="s">0</span>] + ap[<span class="s">1</span>] * ab[<span class="s">1</span>] + ap[<span class="s">2</span>] * ab[<span class="s">2</span>]) / len2).clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)
    } <span class="k">else</span> {
        <span class="s">0</span>.<span class="s">0</span>
    };
    Snap {
        point: Point::new(a[<span class="s">0</span>] + ab[<span class="s">0</span>] * t, a[<span class="s">1</span>] + ab[<span class="s">1</span>] * t, a[<span class="s">2</span>] + ab[<span class="s">2</span>] * t),
        kind: SnapKind::Near,
        owner,
    }
}

<span class="c">/// The best candidate within \`aperture\` pixels of the cursor.</span>
<span class="k">pub</span> <span class="k">fn</span> best&lt;F&gt;(candidates: &amp;[Snap], cursor: (f64, f64), aperture: f64, project: F) -&gt; Option&lt;Snap&gt;
<span class="k">where</span>
    F: Fn(&amp;Point) -&gt; Option&lt;(f64, f64)&gt;,
{
    <span class="k">let</span> <span class="k">mut</span> winner: Option&lt;(SnapKind, f64, &amp;Snap)&gt; = None;

    <span class="k">for</span> c <span class="k">in</span> candidates {
        <span class="k">let</span> Some((x, y)) = project(&amp;c.point) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> (dx, dy) = (x - cursor.<span class="s">0</span>, y - cursor.<span class="s">1</span>);
        <span class="k">let</span> d = (dx * dx + dy * dy).sqrt();

        <span class="k">if</span> d &gt; aperture {
            <span class="k">continue</span>;
        }

        <span class="c">// better kind first, then nearer</span>
        <span class="k">let</span> better = <span class="k">match</span> winner {
            None =&gt; <span class="s">true</span>,
            Some((kind, best_d, _)) =&gt; c.kind &lt; kind || (c.kind == kind &amp;&amp; d &lt; best_d),
        };

        <span class="k">if</span> better {
            winner = Some((c.kind, d, c));
        }
    }

    winner.map(|(_, _, c)| c.clone())
}</code></pre></div>
<p><code>lessons/21/src/app/snap.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A point on z = 0.</span>
    <span class="k">fn</span> p(x: f64, y: f64) -&gt; Point {
        Point::new(x, y, <span class="s">0</span>.<span class="s">0</span>)
    }

    <span class="c">/// x and y are screen pixels already.</span>
    <span class="k">fn</span> flat(point: &amp;Point) -&gt; Option&lt;(f64, f64)&gt; {
        Some((point[<span class="s">0</span>], point[<span class="s">1</span>]))
    }

    <span class="c">/// An open polyline: two ends, inner vertices, one midpoint per span.</span>
    #[test]
    <span class="k">fn</span> an_open_polyline_offers_ends_vertices_and_midpoints() {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
        from_polyline(
            &amp;[p(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), p(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), p(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>)],
            <span class="s">false</span>,
            <span class="s">7</span>,
            &amp;<span class="k">mut</span> out,
        );
        <span class="k">let</span> count = |k: SnapKind| out.iter().filter(|s| s.kind == k).count();
        assert_eq!(count(SnapKind::End), <span class="s">2</span>);
        assert_eq!(count(SnapKind::Vertex), <span class="s">1</span>);
        assert_eq!(count(SnapKind::Mid), <span class="s">2</span>);
        assert!(out.iter().all(|s| s.owner == <span class="s">7</span>));
    }

    <span class="c">/// A closed loop has no ends.</span>
    #[test]
    <span class="k">fn</span> a_closed_loop_has_no_ends() {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
        from_polyline(
            &amp;[p(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), p(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), p(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>)],
            <span class="s">true</span>,
            <span class="s">0</span>,
            &amp;<span class="k">mut</span> out,
        );
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::End).count(), <span class="s">0</span>);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Vertex).count(), <span class="s">3</span>);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Mid).count(), <span class="s">3</span>);
    }

    <span class="c">/// A farther end beats a nearer near-point.</span>
    #[test]
    <span class="k">fn</span> kind_wins_before_distance() {
        <span class="k">let</span> candidates = vec![
            Snap {
                point: p(<span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                kind: SnapKind::Near,
                owner: <span class="s">0</span>,
            },
            Snap {
                point: p(<span class="s">6</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                kind: SnapKind::End,
                owner: <span class="s">0</span>,
            },
        ];
        <span class="k">let</span> best = best(&amp;candidates, (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">12</span>.<span class="s">0</span>, flat).expect(&quot;<span class="s">a snap</span>&quot;);
        assert_eq!(best.kind, SnapKind::End);
    }

    <span class="c">/// Within one kind, the nearest wins.</span>
    #[test]
    <span class="k">fn</span> distance_decides_within_a_kind() {
        <span class="k">let</span> candidates = vec![
            Snap {
                point: p(<span class="s">9</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                kind: SnapKind::End,
                owner: <span class="s">1</span>,
            },
            Snap {
                point: p(<span class="s">3</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                kind: SnapKind::End,
                owner: <span class="s">2</span>,
            },
        ];
        assert_eq!(best(&amp;candidates, (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">12</span>.<span class="s">0</span>, flat).unwrap().owner, <span class="s">2</span>);
    }

    <span class="c">/// Too far or not visible: no snap.</span>
    #[test]
    <span class="k">fn</span> out_of_reach_and_out_of_sight_do_not_snap() {
        <span class="k">let</span> candidates = vec![Snap {
            point: p(<span class="s">40</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            kind: SnapKind::End,
            owner: <span class="s">0</span>,
        }];
        assert!(
            best(&amp;candidates, (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">12</span>.<span class="s">0</span>, flat).is_none(),
            &quot;<span class="s">too far</span>&quot;
        );
        <span class="k">let</span> near = vec![Snap {
            point: p(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            kind: SnapKind::End,
            owner: <span class="s">0</span>,
        }];
        assert!(
            best(&amp;near, (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">12</span>.<span class="s">0</span>, |_| None).is_none(),
            &quot;<span class="s">not visible</span>&quot;
        );
    }

    <span class="c">/// The nearest point stays on the segment.</span>
    #[test]
    <span class="k">fn</span> nearest_on_a_segment_stays_on_the_segment() {
        <span class="k">let</span> (a, b) = (p(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), p(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> middle = nearest_on_segment(&amp;a, &amp;b, &amp;p(<span class="s">4</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>), <span class="s">0</span>);
        assert!((middle.point[<span class="s">0</span>] - <span class="s">4</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span> &amp;&amp; middle.point[<span class="s">1</span>].abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
        <span class="k">let</span> past = nearest_on_segment(&amp;a, &amp;b, &amp;p(<span class="s">50</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>), <span class="s">0</span>);
        assert!((past.point[<span class="s">0</span>] - <span class="s">10</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
        <span class="k">let</span> before = nearest_on_segment(&amp;a, &amp;b, &amp;p(-<span class="s">50</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>), <span class="s">0</span>);
        assert!(before.point[<span class="s">0</span>].abs() &lt; <span class="s">1</span>e-<span class="s">12</span>);
    }
}</code></pre></div>
<h2 id="step-8-srcappeditrs">Step 8 · src/app/edit.rs<a class="anchor" href="#/course/21-editing#step-8-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Source edits record document transactions and preserve object identity.</p>
<p><code>lessons/21/src/app/edit.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Applies a change to the selection and remembers the state before it, which is what makes undo possible.</span>
<span class="k">use</span> <span class="k">crate</span>::app::scene::Scene;
<span class="k">use</span> session_rust::{Geometry, Point, Xform};
<span class="k">use</span> std::rc::Rc;</code></pre></div>
<p><code>lessons/21/src/app/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Scene {
    <span class="c">/// The row's document and guid, with its session made private.</span>
    <span class="k">fn</span> writable(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) -&gt; Option&lt;(usize, Rc&lt;str&gt;)&gt; {
        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(row)?;
        <span class="k">let</span> file = <span class="k">self</span>.docs.get_mut(doc)?;

        <span class="k">if</span> file.display_only {
            <span class="k">return</span> None;
        }

        <span class="c">// copy the session if another placement shares it</span>
        Rc::make_mut(&amp;<span class="k">mut</span> file.session);
        Some((doc, guid))
    }

    <span class="c">/// One row's local transform.</span>
    <span class="k">pub</span> <span class="k">fn</span> local_xform_of(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;Xform&gt; {
        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(row)?;
        Some(<span class="k">self</span>.docs.get(doc)?.session.xform(&amp;guid))
    }

    <span class="c">/// Set one row's local transform in one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_row_xform(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, local: Xform, label: &amp;str) -&gt; Option&lt;Xform&gt; {
        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.writable(row)?;
        <span class="k">let</span> file = <span class="k">self</span>.docs.get_mut(doc)?;
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> file.session);
        session.begin(label);
        session.set_xform(&amp;guid, local);
        session.commit();
        <span class="k">self</span>.last_edited = Some(doc);
        <span class="k">self</span>.placement_of(row)
    }

    <span class="c">/// The local transform that applies a world \`delta\` on top of \`base\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> local_for_world_delta(&amp;<span class="k">self</span>, row: u32, delta: &amp;Xform, base: &amp;Xform) -&gt; Option&lt;Xform&gt; {
        <span class="k">let</span> placed = <span class="k">self</span>.placement_of(row)?;
        <span class="k">let</span> parent = &amp;placed * &amp;base.inverse()?; <span class="c">// everything above the object</span>
        <span class="k">let</span> back = parent.inverse()?;
        Some(&amp;(&amp;back * &amp;(delta * &amp;parent)) * base)
    }

    <span class="c">/// Apply a world \`delta\` to one row; returns its new placement.</span>
    <span class="k">pub</span> <span class="k">fn</span> transform_row(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, delta: &amp;Xform, label: &amp;str) -&gt; Option&lt;Xform&gt; {
        <span class="k">let</span> base = <span class="k">self</span>.local_xform_of(row)?;
        <span class="k">let</span> local = <span class="k">self</span>.local_for_world_delta(row, delta, &amp;base)?;
        <span class="k">self</span>.set_row_xform(row, local, label)
    }

    <span class="c">/// One row's world placement: file placement times object transform.</span>
    <span class="k">pub</span> <span class="k">fn</span> placement_of(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;Xform&gt; {
        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(row)?;
        <span class="k">let</span> file = <span class="k">self</span>.docs.get(doc)?;
        <span class="k">let</span> world = file.session.world_xform(&amp;guid);
        Some(&amp;file.place * &amp;world)
    }

    <span class="c">/// Delete one row's object; the caller rebuilds the rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> delete_row(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) -&gt; bool {
        <span class="k">let</span> Some((doc, guid)) = <span class="k">self</span>.writable(row) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(file) = <span class="k">self</span>.docs.get_mut(doc) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> file.session);
        session.begin(&quot;<span class="s">delete</span>&quot;);
        <span class="k">let</span> removed = session.remove_object(&amp;guid);
        session.commit();

        <span class="k">if</span> removed {
            <span class="k">self</span>.last_edited = Some(doc);
            <span class="k">self</span>.selected = None;
        }

        removed
    }

    <span class="c">/// Undo the last edit in the last edited document.</span>
    <span class="k">pub</span> <span class="k">fn</span> undo(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.step_history(<span class="s">true</span>)
    }

    <span class="c">/// Redo the last undone edit in the last edited document.</span>
    <span class="k">pub</span> <span class="k">fn</span> redo(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.step_history(<span class="s">false</span>)
    }

    <span class="c">/// Undo or redo in the last edited document.</span>
    <span class="k">fn</span> step_history(&amp;<span class="k">mut</span> <span class="k">self</span>, back: bool) -&gt; bool {
        <span class="k">let</span> Some(doc) = <span class="k">self</span>.last_edited <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(file) = <span class="k">self</span>.docs.get_mut(doc) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> file.session);

        <span class="k">if</span> back { session.undo() } <span class="k">else</span> { session.redo() }
    }
}</code></pre></div>
<p><code>lessons/21/src/app/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Scene {
    <span class="c">/// Move one control point of a polyline or curve in one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_control_point(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, index: usize, to: &amp;Point) -&gt; bool {
        <span class="k">let</span> Some((doc, guid)) = <span class="k">self</span>.writable(row) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(file) = <span class="k">self</span>.docs.get_mut(doc) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> file.session);
        <span class="k">let</span> Some(geometry) = session.lookup.get(guid.as_ref()).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> edited = <span class="k">match</span> &amp;geometry {
            Geometry::Polyline(source) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();

                <span class="k">if</span> index &gt;= next.point_count() {
                    <span class="k">return</span> <span class="s">false</span>;
                }

                next.set_point(index, to);
                Geometry::Polyline(Rc::new(next))
            }
            Geometry::NurbsCurve(source) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();

                <span class="k">if</span> !next.set_cv_point(index, to) {
                    <span class="k">return</span> <span class="s">false</span>;
                }

                Geometry::NurbsCurve(Rc::new(next))
            }
            _ =&gt; <span class="k">return</span> <span class="s">false</span>,
        };
        session.begin(&quot;<span class="s">edit point</span>&quot;);
        <span class="k">let</span> replaced = session.replace(&amp;guid, edited);
        session.commit();

        <span class="k">if</span> replaced {
            <span class="k">self</span>.last_edited = Some(doc);
        }

        replaced
    }
}</code></pre></div>
<p><code>lessons/21/src/app/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::scene::FileDoc;
    <span class="k">use</span> session_rust::{Point, Session};

    <span class="c">/// A document at the origin.</span>
    <span class="k">fn</span> file(name: &amp;str, session: Rc&lt;Session&gt;) -&gt; FileDoc {
        FileDoc {
            name: name.into(),
            session,
            place: Xform::identity(),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">false</span>,
        }
    }

    <span class="c">/// One session placed twice.</span>
    <span class="k">fn</span> one_point_twice() -&gt; Scene {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">twice</span>&quot;);
        source.add_point(Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> shared = Rc::new(source);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">first</span>&quot;, Rc::clone(&amp;shared)));
        scene.add_file(file(&quot;<span class="s">second</span>&quot;, Rc::clone(&amp;shared)));
        scene
    }

    <span class="c">/// Moving one placement of a shared file leaves the other.</span>
    #[test]
    <span class="k">fn</span> moving_one_placement_leaves_the_other_where_it_was() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        assert!(Rc::ptr_eq(&amp;scene.docs[<span class="s">0</span>].session, &amp;scene.docs[<span class="s">1</span>].session));

        <span class="k">let</span> moved = scene
            .transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;)
            .expect(&quot;<span class="s">row 0 is editable</span>&quot;);

        assert!(!Rc::ptr_eq(&amp;scene.docs[<span class="s">0</span>].session, &amp;scene.docs[<span class="s">1</span>].session));
        assert_eq!([moved.m[<span class="s">12</span>], moved.m[<span class="s">13</span>], moved.m[<span class="s">14</span>]], [<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        <span class="k">let</span> still = scene.placement_of(<span class="s">1</span>).expect(&quot;<span class="s">row 1 still exists</span>&quot;);
        assert_eq!([still.m[<span class="s">12</span>], still.m[<span class="s">13</span>], still.m[<span class="s">14</span>]], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
    }

    <span class="c">/// A world move lands in world units under a scaled placement.</span>
    #[test]
    <span class="k">fn</span> a_world_delta_moves_the_object_in_the_world() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">placed</span>&quot;);
        source.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(FileDoc {
            name: &quot;<span class="s">placed</span>&quot;.into(),
            session: Rc::new(source),
            <span class="c">// scaled ten times and shifted</span>
            place: Xform::from_matrix([
                <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
                <span class="s">0</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
                <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
                <span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>,
            ]),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">false</span>,
        });
        <span class="k">let</span> before = scene.placement_of(<span class="s">0</span>).expect(&quot;<span class="s">a placement</span>&quot;);
        assert_eq!(
            [before.m[<span class="s">12</span>], before.m[<span class="s">13</span>], before.m[<span class="s">14</span>]],
            [<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]
        );

        <span class="k">let</span> moved = scene
            .transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;)
            .expect(&quot;<span class="s">row 0 is editable</span>&quot;);
        assert_eq!(
            [moved.m[<span class="s">12</span>], moved.m[<span class="s">13</span>], moved.m[<span class="s">14</span>]],
            [<span class="s">105</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
            &quot;<span class="s">five world units, not fifty</span>&quot;
        );
    }

    <span class="c">/// Two moves add up.</span>
    #[test]
    <span class="k">fn</span> a_second_move_starts_from_the_first() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        scene.transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;);
        <span class="k">let</span> moved = scene
            .transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;)
            .expect(&quot;<span class="s">row 0 is editable</span>&quot;);
        assert_eq!([moved.m[<span class="s">12</span>], moved.m[<span class="s">13</span>], moved.m[<span class="s">14</span>]], [<span class="s">5</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
    }

    <span class="c">/// Undo reverses a move; redo repeats it.</span>
    #[test]
    <span class="k">fn</span> undo_puts_the_object_back() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        assert!(!scene.undo(), &quot;<span class="s">nothing has been edited yet</span>&quot;);

        scene.transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;);
        assert!(scene.undo());
        <span class="k">let</span> back = scene.placement_of(<span class="s">0</span>).expect(&quot;<span class="s">row 0 still exists</span>&quot;);
        assert_eq!([back.m[<span class="s">12</span>], back.m[<span class="s">13</span>], back.m[<span class="s">14</span>]], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);

        assert!(scene.redo());
        <span class="k">let</span> again = scene.placement_of(<span class="s">0</span>).expect(&quot;<span class="s">row 0 still exists</span>&quot;);
        assert_eq!([again.m[<span class="s">12</span>], again.m[<span class="s">13</span>], again.m[<span class="s">14</span>]], [<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
    }

    <span class="c">/// A control point edit undoes.</span>
    #[test]
    <span class="k">fn</span> a_control_point_moves_and_undoes() {
        <span class="k">use</span> session_rust::Polyline;
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">line</span>&quot;);
        source.add_polyline(
            Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]),
            None,
        );
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">line</span>&quot;, Rc::new(source)));

        assert!(scene.set_control_point(<span class="s">0</span>, <span class="s">1</span>, &amp;Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)));
        <span class="k">let</span> moved = scene.docs[<span class="s">0</span>].session.lookup.values().next().cloned();
        <span class="k">let</span> Some(session_rust::Geometry::Polyline(line)) = moved <span class="k">else</span> {
            panic!(&quot;<span class="s">still a polyline</span>&quot;);
        };
        assert_eq!(line.get_point(<span class="s">1</span>).expect(&quot;<span class="s">two points</span>&quot;)[<span class="s">1</span>], <span class="s">5</span>.<span class="s">0</span>);

        assert!(scene.undo());
        <span class="k">let</span> back = scene.docs[<span class="s">0</span>].session.lookup.values().next().cloned();
        <span class="k">let</span> Some(session_rust::Geometry::Polyline(line)) = back <span class="k">else</span> {
            panic!(&quot;<span class="s">still a polyline</span>&quot;);
        };
        assert_eq!(line.get_point(<span class="s">1</span>).expect(&quot;<span class="s">two points</span>&quot;)[<span class="s">1</span>], <span class="s">0</span>.<span class="s">0</span>);
    }

    <span class="c">/// A point has no control points to edit.</span>
    #[test]
    <span class="k">fn</span> a_kind_with_no_control_points_is_refused() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        assert!(!scene.set_control_point(<span class="s">0</span>, <span class="s">0</span>, &amp;Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)));
    }

    <span class="c">/// A display-only document refuses edits.</span>
    #[test]
    <span class="k">fn</span> a_display_only_document_refuses_the_edit() {
        <span class="k">let</span> <span class="k">mut</span> scene = one_point_twice();
        scene.docs[<span class="s">0</span>].display_only = <span class="s">true</span>;
        assert!(
            scene
                .transform_row(<span class="s">0</span>, &amp;Xform::translation(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move</span>&quot;)
                .is_none()
        );
    }
}</code></pre></div>
<h2 id="step-9-srcappsceners">Step 9 · src/app/scene.rs<a class="anchor" href="#/course/21-editing#step-9-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene owns source documents and maps their identities to GPU rows.</p>
<p><code>lessons/21/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the <code>bases: Bases,</code> line in <code>struct Scene</code> of <code>lessons/20/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> last_edited: Option&lt;usize&gt;, <span class="c">// document undo applies to</span></code></pre></div>
<p>Added after the <code>bases: Bases::default(),</code> line in <code>fn new</code> of <code>lessons/20/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            last_edited: None,</code></pre></div>
<h2 id="step-10-srcenginegpuobjectsrs">Step 10 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/21-editing#step-10-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>The object table stores GPU rows separately from source identity.</p>
<p><code>lessons/21/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Split a placement into matrix, translation and world box.</span>
<span class="k">fn</span> placed_row(local: &amp;AABB, place: &amp;Xform) -&gt; ([f32; <span class="s">16</span>], [f64; <span class="s">3</span>], AABB) {
    <span class="k">let</span> world = local.transformed(place);
    <span class="k">let</span> <span class="k">mut</span> model = place.to_f32();
    <span class="c">// translation goes in its own table</span>
    model[<span class="s">12</span>] = <span class="s">0</span>.<span class="s">0</span>;
    model[<span class="s">13</span>] = <span class="s">0</span>.<span class="s">0</span>;
    model[<span class="s">14</span>] = <span class="s">0</span>.<span class="s">0</span>;
    (
        model,
        [place.m[<span class="s">12</span>], place.m[<span class="s">13</span>], place.m[<span class="s">14</span>]],
        <span class="k">if</span> world.is_valid() {
            world
        } <span class="k">else</span> {
            AABB::empty()
        },
    )
}

<span class="c">/// Translation relative to the origin, as the GPU reads it.</span></code></pre></div>
<p>Added after the <code>translation: Vec&lt;[f64; 3]&gt;,</code> line in <code>struct InstanceTable</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    local_bounds: Vec&lt;AABB&gt;, <span class="c">// box per row, in the object's own space</span>
    widget: Option&lt;u32&gt;, <span class="c">// identity row the gumball draws with</span></code></pre></div>
<p>Added after the <code>translation: Vec::new(),</code> line in <code>fn new</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            local_bounds: Vec::new(),
            widget: None,</code></pre></div>
<p>Replaces the 4 lines from <code>if self.translation.is_empty() {</code> in <code>fn append</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// drop the widget row first; it must stay last</span>
        <span class="k">if</span> <span class="k">let</span> Some(widget) = <span class="k">self</span>.widget.take() {
            <span class="k">let</span> keep = widget <span class="k">as</span> usize;
            <span class="k">self</span>.rows.truncate(keep);
            <span class="k">self</span>.translation.truncate(keep);
            <span class="k">self</span>.local_bounds.truncate(keep);
            <span class="k">self</span>.world_bounds.truncate(keep);
            <span class="c">// rewind both buffers by re-appending the kept rows</span>
            <span class="k">self</span>.buffer.reset();
            <span class="k">self</span>.translations.reset();

            <span class="k">if</span> keep &gt; <span class="s">0</span> {
                <span class="k">let</span> rows: Vec&lt;Instance&gt; = <span class="k">self</span>.rows.clone();
                <span class="k">let</span> anchored_rows: Vec&lt;[f32; 4]&gt; = <span class="k">match</span> &amp;<span class="k">self</span>.last_origin {
                    Some(origin) =&gt; <span class="k">self</span>
                        .translation
                        .iter()
                        .map(|t| anchored(*t, origin))
                        .collect(),
                    None =&gt; vec![[<span class="s">0</span>.<span class="s">0f32</span>; <span class="s">4</span>]; keep],
                };
                <span class="k">let</span> grew = <span class="k">self</span>.buffer.append(ctx, &amp;rows);

                <span class="k">if</span> <span class="k">self</span>.translations.append(ctx, &amp;anchored_rows) || grew {
                    <span class="k">self</span>.group = instance_group(ctx, l, &amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf);
                }
            }
        }

        <span class="c">// first upload replaces the placeholder</span>
        <span class="k">if</span> <span class="k">self</span>.translation.is_empty() {
            <span class="k">self</span>.rows.clear();
            <span class="k">self</span>.world_bounds.clear();
            <span class="k">self</span>.local_bounds.clear();</code></pre></div>
<p>Replaces the 20 lines from \`\` of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.local_bounds.reserve(up.rows.len());

        <span class="k">for</span> (i, r) <span class="k">in</span> up.rows.iter().enumerate() {
            <span class="k">let</span> world = world_box(r);

            <span class="c">// rows with faces join the inside test</span>
            <span class="k">if</span> r.faces &amp;&amp; world.is_valid() {
                <span class="k">let</span> lo = world.min_point();
                <span class="k">let</span> hi = world.max_point();
                <span class="k">self</span>.bounded.push(BoundedRow {
                    row: base + i <span class="k">as</span> u32,
                    lo: [lo[<span class="s">0</span>], lo[<span class="s">1</span>], lo[<span class="s">2</span>]],
                    hi: [hi[<span class="s">0</span>], hi[<span class="s">1</span>], hi[<span class="s">2</span>]],</code></pre></div>
<p>Added after the <code>});</code> line in <code>fn append</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.local_bounds.push(r.bounds);</code></pre></div>
<p><code>lessons/21/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl InstanceTable</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Row the gumball draws with: identity, appended once, last.</span>
    <span class="k">pub</span> <span class="k">fn</span> widget_row(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts) -&gt; (u32, bool) {
        <span class="k">if</span> <span class="k">let</span> Some(row) = <span class="k">self</span>.widget {
            <span class="k">return</span> (row, <span class="s">false</span>);
        }

        <span class="k">let</span> row = <span class="k">self</span>.rows.len() <span class="k">as</span> u32;
        <span class="c">// identity matrix, white tint</span>
        <span class="k">self</span>.rows.push(Instance {
            color: [<span class="s">1</span>.<span class="s">0</span>; <span class="s">4</span>],
            ..Instance::placeholder()
        });
        <span class="k">self</span>.translation.push([<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>]);
        <span class="k">self</span>.local_bounds.push(AABB::empty());
        <span class="k">self</span>.world_bounds.push(AABB::empty());
        <span class="c">// world position zero, relative to the origin</span>
        <span class="k">let</span> translation = <span class="k">match</span> &amp;<span class="k">self</span>.last_origin {
            Some(origin) =&gt; anchored([<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>], origin),
            None =&gt; [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>],
        };
        <span class="c">// append grows the buffers if needed</span>
        <span class="k">let</span> grew = <span class="k">self</span>
            .buffer
            .append(ctx, std::slice::from_ref(&amp;<span class="k">self</span>.rows[row <span class="k">as</span> usize]));
        <span class="k">let</span> grew_t = <span class="k">self</span>
            .translations
            .append(ctx, std::slice::from_ref(&amp;translation));

        <span class="k">if</span> grew || grew_t {
            <span class="k">self</span>.group = instance_group(ctx, l, &amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf);
        }

        <span class="k">self</span>.widget = Some(row);
        (row, grew || grew_t)
    }

    <span class="c">/// Move one object; writes only its row.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_placement(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, place: &amp;Xform) -&gt; bool {
        <span class="k">let</span> i = row <span class="k">as</span> usize;
        <span class="k">let</span> (Some(instance), Some(local)) = (<span class="k">self</span>.rows.get_mut(i), <span class="k">self</span>.local_bounds.get(i)) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> (model, translation, world) = placed_row(local, place);
        instance.model = model;
        <span class="k">self</span>.translation[i] = translation;
        <span class="k">self</span>.world_bounds[i] = world;

        <span class="c">// keep the inside-test box in step</span>
        <span class="k">for</span> b <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.bounded {
            <span class="k">if</span> b.row == row {
                <span class="k">let</span> lo = world.min_point();
                <span class="k">let</span> hi = world.max_point();
                b.lo = [lo[<span class="s">0</span>], lo[<span class="s">1</span>], lo[<span class="s">2</span>]];
                b.hi = [hi[<span class="s">0</span>], hi[<span class="s">1</span>], hi[<span class="s">2</span>]];
            }
        }

        <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);
        <span class="k">let</span> instance = *instance;
        <span class="k">self</span>.buffer
            .write_at(ctx, row, std::slice::from_ref(&amp;instance));

        <span class="k">if</span> <span class="k">let</span> Some(origin) = &amp;<span class="k">self</span>.last_origin {
            <span class="k">let</span> t = anchored(<span class="k">self</span>.translation[i], origin);
            <span class="k">self</span>.translations
                .write_at(ctx, row, std::slice::from_ref(&amp;t));
        }

        <span class="s">true</span>
    }

    <span class="c">/// Set or clear one flag bit on one row.</span></code></pre></div>
<p>Replaces the 2 lines from <code>self.rows.clear();</code> in <code>fn reset</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.widget = None;
        <span class="k">self</span>.rows.clear();
        <span class="k">self</span>.translation.clear();
        <span class="k">self</span>.local_bounds.clear();</code></pre></div>
<p>Added after the <code>self.translation.shrink_to_fit();</code> line in <code>fn release</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.local_bounds.shrink_to_fit();</code></pre></div>
<p>Added after the <code>}</code> line in <code>mod tests</code> of <code>lessons/20/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A move changes the translation and box, never the matrix.</span>
    #[test]
    <span class="k">fn</span> a_move_goes_into_the_translation_not_the_matrix() {
        <span class="k">let</span> local = AABB::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> place = Xform::translation(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>);
        <span class="k">let</span> (model, translation, world) = placed_row(&amp;local, &amp;place);

        assert_eq!([model[<span class="s">12</span>], model[<span class="s">13</span>], model[<span class="s">14</span>]], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        assert_eq!(translation, [<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>]);
        assert_eq!(world.min_point(), Point::new(<span class="s">9</span>.<span class="s">0</span>, <span class="s">19</span>.<span class="s">0</span>, <span class="s">29</span>.<span class="s">0</span>));
        assert_eq!(world.max_point(), Point::new(<span class="s">11</span>.<span class="s">0</span>, <span class="s">21</span>.<span class="s">0</span>, <span class="s">31</span>.<span class="s">0</span>));
    }

    <span class="c">/// One row moves, its neighbour stays, the widget row stays last.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="k">fn</span> one_row_moves_and_its_neighbour_does_not() {
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{Gpu, Upload};
        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">64</span>, <span class="s">64</span>)).unwrap();
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();

        <span class="k">for</span> x <span class="k">in</span> [<span class="s">0</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>] {
            <span class="k">let</span> <span class="k">mut</span> row = ObjectRow::new(Xform::translation(x, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>);
            row.bounds = AABB::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
            row.faces = <span class="s">true</span>;
            upload.obj.rows.push(row);
        }

        gpu.set_scene(&amp;upload);
        assert_eq!(gpu.objects.len(), <span class="s">2</span>);

        <span class="k">let</span> moved = Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">50</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert!(gpu.objects.set_placement(&amp;gpu.ctx, <span class="s">0</span>, &amp;moved));

        <span class="k">let</span> first = gpu.objects.row_bounds(<span class="s">0</span>).expect(&quot;<span class="s">row 0 has a box</span>&quot;);
        <span class="k">let</span> second = gpu.objects.row_bounds(<span class="s">1</span>).expect(&quot;<span class="s">row 1 has a box</span>&quot;);
        assert_eq!(first.min_point()[<span class="s">1</span>], <span class="s">49</span>.<span class="s">0</span>);
        assert_eq!(second.min_point()[<span class="s">0</span>], <span class="s">99</span>.<span class="s">0</span>);
        assert_eq!(second.min_point()[<span class="s">1</span>], -<span class="s">1</span>.<span class="s">0</span>, &quot;<span class="s">the neighbour did not move</span>&quot;);

        <span class="c">// widget row comes after every object row</span>
        <span class="k">let</span> (widget, _) = gpu.objects.widget_row(&amp;gpu.ctx, &amp;gpu.layouts);
        assert_eq!(widget, <span class="s">2</span>);
        assert_eq!(gpu.objects.widget_row(&amp;gpu.ctx, &amp;gpu.layouts).<span class="s">0</span>, widget);

        <span class="c">// a later upload drops and remints the widget row</span>
        <span class="k">let</span> <span class="k">mut</span> more = Upload::default();
        more.obj
            .rows
            .push(ObjectRow::new(Xform::translation(<span class="s">200</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>));
        gpu.set_scene(&amp;more);
        assert_eq!(gpu.objects.len(), <span class="s">3</span>, &quot;<span class="s">two rows, one file's row, no widget</span>&quot;);
        assert_eq!(gpu.objects.widget_row(&amp;gpu.ctx, &amp;gpu.layouts).<span class="s">0</span>, <span class="s">3</span>);
    }

    <span class="c">/// A row with no box stays empty, never infinite.</span>
    #[test]
    <span class="k">fn</span> a_row_with_no_box_stays_empty() {
        <span class="k">let</span> (_, _, world) = placed_row(&amp;AABB::empty(), &amp;Xform::translation(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        assert!(!world.is_valid());
    }

    <span class="c">/// A move written to the f32 value is lost at the next rebase.</span></code></pre></div>
<h2 id="step-11-srcenginegpumodrs">Step 11 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/21-editing#step-11-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/21/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub control_net: SegmentLane,</code> line in <code>struct Gpu</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> gizmo_arms: SegmentLane, <span class="c">// The move/rotate/scale widget, drawn as strokes and markers.</span>
    <span class="k">pub</span> gizmo_dots: GlyphLane, <span class="c">// the gizmo's balls</span></code></pre></div>
<p>Added after the <code>+ self.control_net.allocated_bytes()</code> line in <code>fn allocated_bytes</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + <span class="k">self</span>.gizmo_arms.allocated_bytes()
            + <span class="k">self</span>.gizmo_dots.allocated_bytes()</code></pre></div>
<p>Added after the <code>let control_net = SegmentLane::new(&amp;ctx, &amp;lay…</code> line in <code>fn build</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> gizmo_arms = SegmentLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> gizmo_dots = GlyphLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<p>Added after the <code>control_net,</code> line in <code>fn build</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            gizmo_arms,
            gizmo_dots,</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl Gpu</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Fill the widget's two lanes, replacing whatever they held.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_widget_rows(&amp;<span class="k">mut</span> <span class="k">self</span>, segments: &amp;segments::SegRows, glyphs: &amp;glyphs::GlyphRows) {
        <span class="k">self</span>.gizmo_arms.reset();
        <span class="k">self</span>.gizmo_dots.reset();
        <span class="k">self</span>.gizmo_arms.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, segments);
        <span class="k">self</span>.gizmo_dots.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, glyphs);
    }

    <span class="c">/// Grow the scene box to include object \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> grew_bounds(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) {
        <span class="k">if</span> <span class="k">let</span> Some(box_) = <span class="k">self</span>.objects.row_bounds(row) {
            <span class="k">self</span>.bounds.union_with(&amp;box_);
        }
    }

    <span class="c">/// The identity row the widgets draw against, minting it the first time.</span>
    <span class="k">pub</span> <span class="k">fn</span> widget_row(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; u32 {
        <span class="k">let</span> (row, grew) = <span class="k">self</span>.objects.widget_row(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);

        <span class="k">if</span> grew {
            <span class="k">self</span>.rebind_ink();
        }

        row
    }

    <span class="c">/// Rebuild the ink bind group after targets or tiles moved.</span></code></pre></div>
<p>Added after the <code>if flip || resized {</code> line in <code>fn retarget</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.targets.destroy();</code></pre></div>
<p>Added after the <code>self.control_net.retarget(&amp;self.ctx, &amp;self.la…</code> line in <code>fn retarget</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.gizmo_arms.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.gizmo_dots.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);</code></pre></div>
<p>Added after the <code>self.control_net.reset();</code> line in <code>fn reset</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gizmo_arms.reset();
        <span class="k">self</span>.gizmo_dots.reset();</code></pre></div>
<h2 id="step-12-srcenginegpurenderrs">Step 12 · src/engine/gpu/render.rs<a class="anchor" href="#/course/21-editing#step-12-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/21/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the <code>draws += self.controls.draw_dots(pass, &amp;b);</code> line in <code>fn scene_list</code> of <code>lessons/20/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// Last of the ink list, so the widget is drawn over the object it moves.</span>
        draws += <span class="k">self</span>.gizmo_arms.draw_ribbons(pass, &amp;b);
        draws += <span class="k">self</span>.gizmo_dots.draw_dots(pass, &amp;b);</code></pre></div>
<h2 id="step-13-srcstateeditrs">Step 13 · src/state/edit.rs<a class="anchor" href="#/course/21-editing#step-13-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Editing connects commands and gumball previews to document history.</p>
<p><code>lessons/21/src/state/edit.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::command::Command;
<span class="k">use</span> <span class="k">crate</span>::app::cplane::CPlane;
<span class="k">use</span> <span class="k">crate</span>::app::gizmo::{ARM, Axis, BALL_AT, Drag, Gizmo, HUB, Handle};
<span class="k">use</span> <span class="k">crate</span>::app::layers::{<span class="k">self</span>, Layer};
<span class="k">use</span> <span class="k">crate</span>::app::selection::ControlId;
<span class="k">use</span> <span class="k">crate</span>::app::snap::{<span class="k">self</span>, Snap, SnapKind};
<span class="k">use</span> <span class="k">crate</span>::app::walk::encode::FACING_UNKNOWN;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::{GlyphPoint, GlyphRows};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::{CylinderSegment, SegRows};
<span class="k">use</span> <span class="k">crate</span>::state::render_position;
<span class="k">use</span> <span class="k">crate</span>::state::{SelectionMode, State};
<span class="k">use</span> session_rust::{Point, Vector, Xform};

<span class="c">/// A gizmo drag in progress.</span>
<span class="k">pub</span> <span class="k">struct</span> GizmoDrag {
    row: u32, <span class="c">// the main selected row</span>
    base_local: Xform, <span class="c">// local transform at the grab</span>
    base_place: Xform, <span class="c">// the main row's placement at the grab</span>
    drag: Drag, <span class="c">// the handle and where it was grabbed</span>
}

<span class="k">impl</span> State {</code></pre></div>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Put the gizmo at the center of the selection, or remove it.</span>
    <span class="k">pub</span> <span class="k">fn</span> place_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>, row: Option&lt;u32&gt;) {
        <span class="c">// no box, no gizmo</span>
        <span class="k">let</span> Some(box_) = row.and_then(|r| <span class="k">self</span>.gpu.objects.row_bounds(r)) <span class="k">else</span> {
            <span class="k">self</span>.gizmo = None;
            <span class="k">self</span>.upload_gizmo();
            <span class="k">return</span>;
        };
        <span class="k">let</span> origin = Point::new(box_.cx, box_.cy, box_.cz);

        <span class="k">match</span> <span class="k">self</span>.gizmo.as_mut() {
            Some(gizmo) =&gt; gizmo.set_origin(origin),
            None =&gt; <span class="k">self</span>.gizmo = Some(Gizmo::new(origin)),
        }

        <span class="k">self</span>.upload_gizmo();
    }

    <span class="c">/// Grab a gizmo handle under the mouse; false when the click missed it.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some((from, dir)) = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> per_px = <span class="k">self</span>.world_per_px();
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(handle) = gizmo.hit(&amp;from, &amp;dir, per_px) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(drag) = gizmo.begin(handle, &amp;from, &amp;dir) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(base_local) = <span class="k">self</span>.scene.local_xform_of(row) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(base_place) = <span class="k">self</span>.scene.placement_of(row) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>.dragging = Some(GizmoDrag {
            row,
            base_local,
            base_place,
            drag,
        });
        <span class="s">true</span>
    }

    <span class="c">/// Move the selection with the pointer; a preview, the document is untouched.</span>
    <span class="k">pub</span> <span class="k">fn</span> drag_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(active) = <span class="k">self</span>.dragging.as_ref() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some((from, dir)) = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_ref() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(delta) = gizmo.update(&amp;active.drag, &amp;from, &amp;dir) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> place = &amp;delta * &amp;active.base_place;
        <span class="k">self</span>.gpu
            .objects
            .set_placement(&amp;<span class="k">self</span>.gpu.ctx, active.row, &amp;place);
        <span class="k">self</span>.gpu.grew_bounds(active.row);
        <span class="k">self</span>.place_gizmo(Some(active.row));
        <span class="k">self</span>.touch();
        <span class="s">true</span>
    }</code></pre></div>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Release: the document records the whole gesture as one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> end_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(active) = <span class="k">self</span>.dragging.take() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some((from, dir)) = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        gizmo.drag = None;
        <span class="k">let</span> Some(delta) = gizmo.update(&amp;active.drag, &amp;from, &amp;dir) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="c">// world delta to local transform</span>
        <span class="k">let</span> Some(local) = <span class="k">self</span>
            .scene
            .local_for_world_delta(active.row, &amp;delta, &amp;active.base_local)
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="c">// the undo label</span>
        <span class="k">let</span> label = <span class="k">match</span> active.drag.handle {
            Handle::Translate(_) =&gt; &quot;<span class="s">move</span>&quot;,
            Handle::Rotate(_) =&gt; &quot;<span class="s">rotate</span>&quot;,
            Handle::Scale(_) | Handle::ScaleUniform =&gt; &quot;<span class="s">scale</span>&quot;,
        };

        <span class="k">if</span> <span class="k">let</span> Some(place) = <span class="k">self</span>.scene.set_row_xform(active.row, local, label) {
            <span class="k">self</span>.gpu
                .objects
                .set_placement(&amp;<span class="k">self</span>.gpu.ctx, active.row, &amp;place);
            <span class="k">self</span>.gpu.grew_bounds(active.row);
        }

        <span class="k">self</span>.touch();
        <span class="s">true</span>
    }

    <span class="c">/// Drop a drag that will never be released; everything goes back.</span>
    <span class="k">pub</span> <span class="k">fn</span> cancel_gesture(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">let</span> Some(active) = <span class="k">self</span>.dragging.take() {
            <span class="k">self</span>.gpu
                .objects
                .set_placement(&amp;<span class="k">self</span>.gpu.ctx, active.row, &amp;active.base_place);

            <span class="k">if</span> <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() {
                gizmo.drag = None;
            }

            <span class="k">self</span>.place_gizmo(Some(active.row));
            <span class="k">self</span>.touch();
        }

        <span class="k">if</span> <span class="k">self</span>.control_drag.take().is_some() {
            <span class="c">// The control preview is a dot in a temporary lane; re-uploading from the source</span>
            <span class="k">self</span>.upload_controls();
            <span class="k">self</span>.touch();
        }
    }

    <span class="c">/// Delete the selection.</span>
    <span class="k">pub</span> <span class="k">fn</span> delete_selected(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> !<span class="k">self</span>.scene.delete_row(row) {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.select(None);
        <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.place_gizmo(None);
        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }</code></pre></div>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Ctrl+Z: undo the last edit.</span>
    <span class="k">pub</span> <span class="k">fn</span> undo(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.scene.undo() {
            <span class="k">self</span>.after_history();
        }
    }

    <span class="c">/// Ctrl+Y: redo it.</span>
    <span class="k">pub</span> <span class="k">fn</span> redo(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.scene.redo() {
            <span class="k">self</span>.after_history();
        }
    }

    <span class="c">/// Undo may add or remove objects: rebuild rows.</span>
    <span class="k">fn</span> after_history(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.select(None);
        <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.place_gizmo(None);
        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Scene length of one CSS pixel at the gizmo.</span>
    <span class="k">fn</span> world_per_px(&amp;<span class="k">self</span>) -&gt; f64 {
        world_per_css_px(
            <span class="k">self</span>.camera.distance_world(),
            <span class="k">self</span>.viewport().<span class="s">1</span>,
            <span class="k">self</span>.pixel_scale(),
        )
    }

    <span class="c">/// Physical pixels per CSS pixel.</span>
    <span class="k">fn</span> pixel_scale(&amp;<span class="k">self</span>) -&gt; f64 {
        <span class="k">let</span> logical = <span class="k">self</span>.logical_size()[<span class="s">0</span>];

        <span class="k">if</span> logical &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> <span class="s">1</span>.<span class="s">0</span>;
        }

        f64::from(<span class="k">self</span>.gpu.config.width) / logical
    }
}

<span class="c">/// Scene length of one CSS pixel at \`world_distance\` from the eye.</span>
<span class="k">fn</span> world_per_css_px(world_distance: f64, physical_height: f64, physical_per_css: f64) -&gt; f64 {
    <span class="k">if</span> physical_height &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> <span class="s">1</span>.<span class="s">0</span>;
    }

    <span class="c">// view height at that distance, over the pixels it covers</span>
    <span class="k">let</span> per_physical =
        <span class="s">2</span>.<span class="s">0</span> * world_distance * (<span class="k">crate</span>::camera::FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan() / physical_height;
    per_physical * physical_per_css
}

<span class="c">/// segments per quarter-circle arc</span>
<span class="k">const</span> ARC_STEPS: u32 = <span class="s">12</span>;

<span class="c">/// The two axes an arc about \`axis\` is drawn in.</span>
<span class="k">fn</span> arc_axes(axis: Axis) -&gt; ([f64; <span class="s">3</span>], [f64; <span class="s">3</span>]) {
    <span class="k">match</span> axis {
        Axis::X =&gt; ([<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
        Axis::Y =&gt; ([<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>], [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]),
        Axis::Z =&gt; ([<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]),
    }
}

<span class="c">/// axis ball radius in CSS pixels</span>
<span class="k">const</span> BALL_PX: f64 = <span class="s">5</span>.<span class="s">0</span>;

<span class="c">/// the three axis colours, packed</span>
<span class="k">const</span> AXIS_COLORS: [u32; <span class="s">3</span>] = [<span class="s">0xff2222dd</span>, <span class="s">0xff22bb22</span>, <span class="s">0xffdd4422</span>];

<span class="k">impl</span> State {
    <span class="c">/// Tell the GPU where the gizmo is and which handle lights up.</span>
    <span class="k">pub</span> <span class="k">fn</span> upload_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_ref() <span class="k">else</span> {
            <span class="k">self</span>.gpu
                .set_widget_rows(&amp;SegRows::default(), &amp;GlyphRows::default());
            <span class="k">return</span>;
        };
        <span class="k">let</span> origin = gizmo.origin.clone();
        <span class="k">let</span> per_px = <span class="k">self</span>.world_per_px();
        <span class="c">// negative radius = physical pixels</span>
        <span class="k">let</span> scale = <span class="k">self</span>.pixel_scale();
        <span class="c">// World coordinates, drawn against the identity row.</span>
        <span class="k">let</span> widget = <span class="k">self</span>.gpu.widget_row();
        <span class="k">let</span> (segments, glyphs) = widget_rows(&amp;origin, per_px, scale, widget);
        <span class="k">self</span>.gpu.set_widget_rows(&amp;segments, &amp;glyphs);
    }
}

<span class="c">/// The widget's rows: arms, arcs, balls, hub.</span>
<span class="k">fn</span> widget_rows(origin: &amp;Point, per_px: f64, pixel_scale: f64, widget: u32) -&gt; (SegRows, GlyphRows) {
    <span class="k">let</span> arm = ARM * per_px;
    <span class="k">let</span> ball = BALL_AT * per_px;
    <span class="k">let</span> <span class="k">mut</span> segments = SegRows::default();
    <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();
    <span class="k">let</span> stroke = |p0: [f64; <span class="s">3</span>], p1: [f64; <span class="s">3</span>], color: u32| CylinderSegment {
        p0: render_position(p0),
        p1: render_position(p1),
        radius: <span class="s">0</span>.<span class="s">0</span>,
        color,
        instance_id: widget, <span class="c">// the gizmo's row</span>
        facing: FACING_UNKNOWN,
    };

    <span class="k">for</span> (i, axis) <span class="k">in</span> [Axis::X, Axis::Y, Axis::Z].into_iter().enumerate() {
        <span class="k">let</span> u = axis.unit();
        <span class="k">let</span> at = |d: f64| {
            [
                origin[<span class="s">0</span>] + u[<span class="s">0</span>] * d,
                origin[<span class="s">1</span>] + u[<span class="s">1</span>] * d,
                origin[<span class="s">2</span>] + u[<span class="s">2</span>] * d,
            ]
        };
        segments.ribbons.push(stroke(
            [origin[<span class="s">0</span>], origin[<span class="s">1</span>], origin[<span class="s">2</span>]],
            at(arm),
            AXIS_COLORS[i],
        ));
        glyphs.dots.push(GlyphPoint {
            center: render_position(at(ball)),
            radius: -(BALL_PX * pixel_scale) <span class="k">as</span> f32, <span class="c">// negative = physical pixels</span>
            color: unpack_color(AXIS_COLORS[i]),
            instance_id: widget, <span class="c">// the gizmo's row</span>
            facing: FACING_UNKNOWN,
            facing_ext: [FACING_UNKNOWN; <span class="s">2</span>],
        });
    }

    <span class="c">// The three rotation arcs where \`hit\` looks for them.</span>
    <span class="k">for</span> (i, axis) <span class="k">in</span> [Axis::X, Axis::Y, Axis::Z].into_iter().enumerate() {
        <span class="k">let</span> (u, v) = arc_axes(axis);
        <span class="k">let</span> <span class="k">mut</span> previous: Option&lt;[f64; 3]&gt; = None;

        <span class="k">for</span> step <span class="k">in</span> <span class="s">0</span>..=ARC_STEPS {
            <span class="k">let</span> t = std::f64::consts::FRAC_PI_2 * f64::from(step) / f64::from(ARC_STEPS);
            <span class="k">let</span> (c, d) = (-t.cos() * arm, -t.sin() * arm);
            <span class="k">let</span> at = [
                origin[<span class="s">0</span>] + u[<span class="s">0</span>] * c + v[<span class="s">0</span>] * d,
                origin[<span class="s">1</span>] + u[<span class="s">1</span>] * c + v[<span class="s">1</span>] * d,
                origin[<span class="s">2</span>] + u[<span class="s">2</span>] * c + v[<span class="s">2</span>] * d,
            ];

            <span class="k">if</span> <span class="k">let</span> Some(from) = previous {
                segments.ribbons.push(stroke(from, at, AXIS_COLORS[i]));
            }

            previous = Some(at);
        }
    }

    glyphs.dots.push(GlyphPoint {
        center: render_position([origin[<span class="s">0</span>], origin[<span class="s">1</span>], origin[<span class="s">2</span>]]), <span class="c">// the hub</span>
        radius: -(HUB * pixel_scale) <span class="k">as</span> f32, <span class="c">// negative = physical pixels</span>
        color: [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>], <span class="c">// white</span>
        instance_id: widget, <span class="c">// the gizmo's row</span>
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; <span class="s">2</span>],
    });
    (segments, glyphs)
}

<span class="c">/// A packed colour as four floats, red in the low byte.</span>
<span class="k">fn</span> unpack_color(packed: u32) -&gt; [f32; <span class="s">4</span>] {
    [
        (packed &amp; <span class="s">0xff</span>) <span class="k">as</span> f32 / <span class="s">255</span>.<span class="s">0</span>,
        ((packed &gt;&gt; <span class="s">8</span>) &amp; <span class="s">0xff</span>) <span class="k">as</span> f32 / <span class="s">255</span>.<span class="s">0</span>,
        ((packed &gt;&gt; <span class="s">16</span>) &amp; <span class="s">0xff</span>) <span class="k">as</span> f32 / <span class="s">255</span>.<span class="s">0</span>,
        ((packed &gt;&gt; <span class="s">24</span>) &amp; <span class="s">0xff</span>) <span class="k">as</span> f32 / <span class="s">255</span>.<span class="s">0</span>,
    ]</code></pre></div>
<h2 id="step-14-srcappcommandrs">Step 14 · src/app/command.rs<a class="anchor" href="#/course/21-editing#step-14-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>The command parser turns typed input into editing actions.</p>
<p><code>lessons/21/src/app/command.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Every verb the viewer can do, named once, so the command line and the buttons trigger exactly the same code.</span>
<span class="k">use</span> <span class="k">crate</span>::app::coords;
<span class="k">use</span> <span class="k">crate</span>::app::gizmo::Axis;

<span class="c">/// What a line asked for.</span>
#[derive(Clone, Copy, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> Command {
    Move([f64; <span class="s">3</span>]), <span class="c">// move the selection by mm</span>
    Rotate { axis: Axis, degrees: f64 }, <span class="c">// Turn the selection about one axis through its own centre, in degrees.</span>
    Scale(f64), <span class="c">// scale about the selection centre</span>
    Delete,
    Undo,
    Redo,
    Hide,
    ShowAll, <span class="c">// show everything hidden</span>
    Fit, <span class="c">// zoom to selection or scene</span>
    Escape, <span class="c">// clear the selection</span>
}</code></pre></div>
<p><code>lessons/21/src/app/command.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Parse one line; the error is the message to show.</span>
<span class="k">pub</span> <span class="k">fn</span> parse(line: &amp;str) -&gt; Result&lt;Command, String&gt; {
    <span class="k">let</span> line = line.trim();

    <span class="k">if</span> line.is_empty() {
        <span class="k">return</span> Err(&quot;<span class="s">nothing typed</span>&quot;.into());
    }

    <span class="k">let</span> <span class="k">mut</span> words = line.split_whitespace();
    <span class="k">let</span> verb = words.next().unwrap_or_default().to_ascii_lowercase();
    <span class="k">let</span> rest: Vec&lt;&amp;str&gt; = words.collect(); <span class="c">// the arguments</span>

    <span class="k">match</span> verb.as_str() {
        &quot;<span class="s">move</span>&quot; | &quot;<span class="s">m</span>&quot; =&gt; offset(&amp;rest).map(Command::Move),
        &quot;<span class="s">rotate</span>&quot; | &quot;<span class="s">rot</span>&quot; =&gt; {
            <span class="k">let</span> (axis, degrees) = axis_and_number(&amp;rest, &quot;<span class="s">rotate x 90</span>&quot;)?;
            Ok(Command::Rotate { axis, degrees })
        }
        &quot;<span class="s">scale</span>&quot; | &quot;<span class="s">s</span>&quot; =&gt; {
            <span class="k">let</span> k = number(rest.first().copied(), &quot;<span class="s">scale 2</span>&quot;)?;

            <span class="k">if</span> k &lt;= <span class="s">0</span>.<span class="s">0</span> {
                <span class="k">return</span> Err(&quot;<span class="s">scale wants a factor above zero</span>&quot;.into());
            }

            Ok(Command::Scale(k))
        }
        &quot;<span class="s">delete</span>&quot; | &quot;<span class="s">del</span>&quot; =&gt; Ok(Command::Delete),
        &quot;<span class="s">undo</span>&quot; =&gt; Ok(Command::Undo),
        &quot;<span class="s">redo</span>&quot; =&gt; Ok(Command::Redo),
        &quot;<span class="s">hide</span>&quot; =&gt; Ok(Command::Hide),
        &quot;<span class="s">show</span>&quot; =&gt; Ok(Command::ShowAll),
        &quot;<span class="s">fit</span>&quot; =&gt; Ok(Command::Fit),
        &quot;<span class="s">escape</span>&quot; | &quot;<span class="s">esc</span>&quot; =&gt; Ok(Command::Escape),
        other =&gt; Err(format!(&quot;<span class="s">no command \`</span>{<span class="s">other</span>}<span class="s">\`</span>&quot;)),
    }
}

<span class="c">/// A move offset from \`10 0 0\`, \`@10,0\` or \`10&lt;45\`.</span>
<span class="k">fn</span> offset(words: &amp;[&amp;str]) -&gt; Result&lt;[f64; 3], String&gt; {
    <span class="k">let</span> joined = words.join(&quot;<span class="s"> </span>&quot;);
    <span class="c">// spaces between numbers become commas</span>
    <span class="k">let</span> text = <span class="k">if</span> joined.contains('<span class="s">,</span>') || joined.contains('<span class="s">&lt;</span>') {
        joined.replace('<span class="s"> </span>', &quot;&quot;)
    } <span class="k">else</span> {
        words.join(&quot;<span class="s">,</span>&quot;).replacen(&quot;<span class="s">@,</span>&quot;, &quot;<span class="s">@</span>&quot;, <span class="s">1</span>)
    };
    <span class="k">let</span> Some(typed) = coords::parse(&amp;text) <span class="k">else</span> {
        <span class="k">return</span> Err(format!(&quot;<span class="s">\`</span>{<span class="s">joined</span>}<span class="s">\` is not an offset; try \`move 10 0 0\`</span>&quot;));
    };

    <span class="k">match</span> typed {
        coords::Typed::Absolute { x, y, z } | coords::Typed::Relative { x, y, z } =&gt; {
            Ok([x, y, z.unwrap_or(<span class="s">0</span>.<span class="s">0</span>)])
        }
        coords::Typed::Polar { distance, degrees } =&gt; {
            <span class="c">// polar in the world XY plane</span>
            <span class="k">let</span> r = degrees.to_radians();
            Ok([distance * r.cos(), distance * r.sin(), <span class="s">0</span>.<span class="s">0</span>])
        }
        coords::Typed::Distance(d) =&gt; Ok([d, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]), <span class="c">// a bare number moves along x</span>
    }
}

<span class="c">/// An axis letter and a number.</span>
<span class="k">fn</span> axis_and_number(words: &amp;[&amp;str], example: &amp;str) -&gt; Result&lt;(Axis, f64), String&gt; {
    <span class="k">let</span> axis = <span class="k">match</span> words.first().map(|w| w.to_ascii_lowercase()) {
        Some(a) <span class="k">if</span> a == &quot;<span class="s">x</span>&quot; =&gt; Axis::X,
        Some(a) <span class="k">if</span> a == &quot;<span class="s">y</span>&quot; =&gt; Axis::Y,
        Some(a) <span class="k">if</span> a == &quot;<span class="s">z</span>&quot; =&gt; Axis::Z,
        _ =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">which axis? try \`</span>{<span class="s">example</span>}<span class="s">\`</span>&quot;)),
    };
    Ok((axis, number(words.get(<span class="s">1</span>).copied(), example)?))
}

<span class="c">/// A finite number, or a message with the example.</span>
<span class="k">fn</span> number(word: Option&lt;&amp;str&gt;, example: &amp;str) -&gt; Result&lt;f64, String&gt; {
    <span class="k">let</span> Some(word) = word <span class="k">else</span> {
        <span class="k">return</span> Err(format!(&quot;<span class="s">missing a number; try \`</span>{<span class="s">example</span>}<span class="s">\`</span>&quot;));
    };

    <span class="k">match</span> word.parse::&lt;f64&gt;() {
        Ok(v) <span class="k">if</span> v.is_finite() =&gt; Ok(v),
        _ =&gt; Err(format!(&quot;<span class="s">\`</span>{<span class="s">word</span>}<span class="s">\` is not a number</span>&quot;)),
    }
}</code></pre></div>
<p><code>lessons/21/src/app/command.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Tab completes the verb, then its option.</span>
    #[test]
    <span class="k">fn</span> the_verbs_and_their_short_forms() {
        assert_eq!(parse(&quot;<span class="s">delete</span>&quot;), Ok(Command::Delete));
        assert_eq!(parse(&quot;<span class="s">del</span>&quot;), Ok(Command::Delete));
        assert_eq!(parse(&quot;<span class="s">  UNDO </span>&quot;), Ok(Command::Undo));
        assert_eq!(parse(&quot;<span class="s">fit</span>&quot;), Ok(Command::Fit));
    }

    <span class="c">/// Move accepts every coordinate form.</span>
    #[test]
    <span class="k">fn</span> move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
        assert_eq!(parse(&quot;<span class="s">move 10 0 0</span>&quot;), Ok(Command::Move([<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>])));
        assert_eq!(parse(&quot;<span class="s">m @0 5</span>&quot;), Ok(Command::Move([<span class="s">0</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>])));
        <span class="k">let</span> Ok(Command::Move(polar)) = parse(&quot;<span class="s">move 10&lt;90</span>&quot;) <span class="k">else</span> {
            panic!(&quot;<span class="s">a polar offset is an offset</span>&quot;);
        };
        assert!((polar[<span class="s">1</span>] - <span class="s">10</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>, &quot;<span class="s">90 degrees is +y</span>&quot;);
    }

    <span class="c">/// A bad line gets a message naming the problem.</span>
    #[test]
    <span class="k">fn</span> a_line_it_cannot_do_says_so() {
        assert_eq!(parse(&quot;<span class="s">fly 3</span>&quot;), Err(&quot;<span class="s">no command \`fly\`</span>&quot;.into()));
        assert_eq!(parse(&quot;&quot;), Err(&quot;<span class="s">nothing typed</span>&quot;.into()));
        assert!(
            parse(&quot;<span class="s">scale 0</span>&quot;).is_err(),
            &quot;<span class="s">a zero scale collapses the object</span>&quot;
        );
        assert!(parse(&quot;<span class="s">rotate 90</span>&quot;).is_err(), &quot;<span class="s">no axis</span>&quot;);
        assert!(parse(&quot;<span class="s">rotate x</span>&quot;).is_err(), &quot;<span class="s">no angle</span>&quot;);
        assert!(parse(&quot;<span class="s">move sideways</span>&quot;).is_err());
    }

    <span class="c">/// Rotate carries its axis.</span>
    #[test]
    <span class="k">fn</span> rotation_names_its_axis() {
        assert_eq!(
            parse(&quot;<span class="s">rotate z 45</span>&quot;),
            Ok(Command::Rotate {
                axis: Axis::Z,
                degrees: <span class="s">45</span>.<span class="s">0</span>
            })
        );
    }
}</code></pre></div>
<h2 id="step-15-srcstateeditrs">Step 15 · src/state/edit.rs<a class="anchor" href="#/course/21-editing#step-15-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Editing connects commands and gumball previews to document history.</p>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="k">impl</span> State {
    <span class="c">/// Run one command line; the answer is what to show the person.</span>
    <span class="k">pub</span> <span class="k">fn</span> run_command(&amp;<span class="k">mut</span> <span class="k">self</span>, line: &amp;str) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> command = <span class="k">crate</span>::app::command::parse(line)?;
        <span class="c">// commands that act on the selection</span>
        <span class="k">let</span> needs_selection = matches!(
            command,
            Command::Move(_) | Command::Rotate { .. } | Command::Scale(_) | Command::Delete
        );

        <span class="k">if</span> needs_selection &amp;&amp; <span class="k">self</span>.scene.selected.is_none() {
            <span class="k">return</span> Err(&quot;<span class="s">nothing is selected</span>&quot;.into());
        }

        <span class="k">match</span> command {
            Command::Move(d) =&gt; <span class="k">self</span>.apply(Xform::translation(d[<span class="s">0</span>], d[<span class="s">1</span>], d[<span class="s">2</span>]), &quot;<span class="s">move</span>&quot;),
            Command::Rotate { axis, degrees } =&gt; {
                <span class="k">let</span> about = <span class="k">self</span>.gizmo.as_ref().map(|g| g.origin.clone());
                <span class="k">let</span> turn = rotation_about(axis, degrees, about.as_ref());
                <span class="k">self</span>.apply(turn, &quot;<span class="s">rotate</span>&quot;)
            }
            Command::Scale(k) =&gt; {
                <span class="k">let</span> about = <span class="k">self</span>.gizmo.as_ref().map(|g| g.origin.clone());
                <span class="k">self</span>.apply(scaling_about(k, about.as_ref()), &quot;<span class="s">scale</span>&quot;)
            }
            Command::Delete =&gt; {
                <span class="k">self</span>.delete_selected();
                Ok(&quot;<span class="s">deleted</span>&quot;.into())
            }
            Command::Undo =&gt; {
                <span class="k">self</span>.undo();
                Ok(&quot;<span class="s">undone</span>&quot;.into())
            }
            Command::Redo =&gt; {
                <span class="k">self</span>.redo();
                Ok(&quot;<span class="s">redone</span>&quot;.into())
            }
            Command::Hide =&gt; {
                <span class="k">self</span>.hide_selected();
                Ok(&quot;<span class="s">hidden</span>&quot;.into())
            }
            Command::ShowAll =&gt; {
                <span class="k">self</span>.show_all();
                Ok(&quot;<span class="s">everything shown</span>&quot;.into())
            }
            Command::Fit =&gt; {
                <span class="k">self</span>.fit_selected_or_all();
                Ok(&quot;<span class="s">fitted</span>&quot;.into())
            }
            Command::Escape =&gt; {
                <span class="k">self</span>.escape_selection();
                Ok(&quot;<span class="s">selection cleared</span>&quot;.into())
            }
        }
    }

    <span class="c">/// Apply one transform to the selection and record it.</span>
    <span class="k">fn</span> apply(&amp;<span class="k">mut</span> <span class="k">self</span>, delta: Xform, label: &amp;str) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected <span class="k">else</span> {
            <span class="k">return</span> Err(&quot;<span class="s">nothing is selected</span>&quot;.into());
        };
        <span class="k">let</span> Some(place) = <span class="k">self</span>.scene.transform_row(row, &amp;delta, label) <span class="k">else</span> {
            <span class="k">return</span> Err(&quot;<span class="s">this row cannot be edited</span>&quot;.into());
        };
        <span class="k">self</span>.gpu.objects.set_placement(&amp;<span class="k">self</span>.gpu.ctx, row, &amp;place);
        <span class="k">self</span>.gpu.grew_bounds(row);
        <span class="k">self</span>.place_gizmo(Some(row));
        <span class="k">self</span>.touch();
        Ok(label.into())
    }
}

<span class="c">/// A rotation about a point.</span>
<span class="k">fn</span> rotation_about(axis: Axis, degrees: f64, about: Option&lt;&amp;Point&gt;) -&gt; Xform {
    <span class="k">let</span> turn = <span class="k">match</span> axis {
        Axis::X =&gt; Xform::rotation_x(degrees, <span class="s">true</span>),
        Axis::Y =&gt; Xform::rotation_y(degrees, <span class="s">true</span>),
        Axis::Z =&gt; Xform::rotation_z(degrees, <span class="s">true</span>),
    };
    centred(turn, about)
}

<span class="c">/// A uniform scale about a point.</span>
<span class="k">fn</span> scaling_about(factor: f64, about: Option&lt;&amp;Point&gt;) -&gt; Xform {
    <span class="k">match</span> about {
        Some(p) =&gt; Xform::scale_uniform(p, factor),
        None =&gt; Xform::scale_xyz(factor, factor, factor),
    }
}

<span class="c">/// Move \`about\` to the origin, apply \`inner\`, move back.</span>
<span class="k">fn</span> centred(inner: Xform, about: Option&lt;&amp;Point&gt;) -&gt; Xform {
    <span class="k">let</span> Some(p) = about <span class="k">else</span> {
        <span class="k">return</span> inner;
    };
    <span class="k">let</span> to = Xform::translation(p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]);
    <span class="k">let</span> back = Xform::translation(-p[<span class="s">0</span>], -p[<span class="s">1</span>], -p[<span class="s">2</span>]);
    &amp;(&amp;to * &amp;inner) * &amp;back</code></pre></div>
<h2 id="step-16-srcapplayersrs">Step 16 · src/app/layers.rs<a class="anchor" href="#/course/21-editing#step-16-srcapplayersrs" aria-label="Link to this section">#</a></h2>
<p>Layer rows collect objects by document or geometry kind.</p>
<p><code>lessons/21/src/app/layers.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Groups objects so a whole set can be hidden or shown at once.</span>
<span class="k">use</span> <span class="k">crate</span>::app::scene::Scene;

<span class="c">/// What one panel row controls.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> Layer {
    Document(usize), <span class="c">// one loaded file, by index</span>
    Kind(Kind),
}

<span class="c">/// The kinds the panel groups objects by.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
<span class="k">pub</span> <span class="k">enum</span> Kind {
    Solids, <span class="c">// BReps, boxes, elements</span>
    Surfaces, <span class="c">// NURBS surfaces, planes</span>
    Meshes,
    Curves, <span class="c">// lines, polylines, NURBS curves</span>
    Points,
    Clouds,
}

<span class="k">impl</span> Kind {
    <span class="c">/// The row label.</span>
    <span class="k">pub</span> <span class="k">fn</span> label(<span class="k">self</span>) -&gt; &amp;'static str {
        <span class="k">match</span> <span class="k">self</span> {
            Kind::Solids =&gt; &quot;<span class="s">solids</span>&quot;,
            Kind::Surfaces =&gt; &quot;<span class="s">surfaces</span>&quot;,
            Kind::Meshes =&gt; &quot;<span class="s">meshes</span>&quot;,
            Kind::Curves =&gt; &quot;<span class="s">curves</span>&quot;,
            Kind::Points =&gt; &quot;<span class="s">points</span>&quot;,
            Kind::Clouds =&gt; &quot;<span class="s">clouds</span>&quot;,
        }
    }

    <span class="c">/// The kind of one geometry.</span>
    <span class="k">fn</span> of(geometry: &amp;session_rust::Geometry) -&gt; <span class="k">Self</span> {
        <span class="k">use</span> session_rust::Geometry <span class="k">as</span> G;

        <span class="k">match</span> geometry {
            G::BRep(_) | G::OBB(_) | G::Element(_) =&gt; Kind::Solids,
            G::NurbsSurface(_) | G::Plane(_) =&gt; Kind::Surfaces,
            G::Mesh(_) =&gt; Kind::Meshes,
            G::Line(_) | G::Polyline(_) | G::NurbsCurve(_) =&gt; Kind::Curves,
            G::Point(_) =&gt; Kind::Points,
            G::PointCloud(_) =&gt; Kind::Clouds,
        }
    }
}</code></pre></div>
<p><code>lessons/21/src/app/layers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Layer {
    <span class="c">/// The row's text key, e.g. \`doc:0\` or \`kind:curves\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> key(<span class="k">self</span>) -&gt; String {
        <span class="k">match</span> <span class="k">self</span> {
            Layer::Document(index) =&gt; format!(&quot;<span class="s">doc:</span>{<span class="s">index</span>}&quot;),
            Layer::Kind(kind) =&gt; format!(&quot;<span class="s">kind:</span>{}&quot;, kind.label()),
        }
    }

    <span class="c">/// The layer from a row key.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_key(key: &amp;str) -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(index) = key.strip_prefix(&quot;<span class="s">doc:</span>&quot;) {
            <span class="k">return</span> index.parse().ok().map(Layer::Document);
        }

        <span class="k">let</span> label = key.strip_prefix(&quot;<span class="s">kind:</span>&quot;)?;

        <span class="k">for</span> kind <span class="k">in</span> [
            Kind::Solids,
            Kind::Surfaces,
            Kind::Meshes,
            Kind::Curves,
            Kind::Points,
            Kind::Clouds,
        ] {
            <span class="k">if</span> kind.label() == label {
                <span class="k">return</span> Some(Layer::Kind(kind));
            }
        }

        None
    }
}

<span class="c">/// One line of the panel.</span>
<span class="k">pub</span> <span class="k">struct</span> Row {
    <span class="k">pub</span> layer: Layer, <span class="c">// what it controls</span>
    <span class="k">pub</span> label: String, <span class="c">// text shown</span>
    <span class="k">pub</span> count: usize, <span class="c">// objects it controls</span>
    <span class="k">pub</span> hidden: bool,
}</code></pre></div>
<p><code>lessons/21/src/app/layers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The panel rows: documents, then the kinds present.</span>
<span class="k">pub</span> <span class="k">fn</span> rows(scene: &amp;Scene) -&gt; Vec&lt;Row&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

    <span class="k">for</span> (index, doc) <span class="k">in</span> scene.docs.iter().enumerate() {
        <span class="k">let</span> rows = of_layer(scene, Layer::Document(index));

        <span class="k">if</span> rows.is_empty() {
            <span class="k">continue</span>;
        }

        out.push(Row {
            layer: Layer::Document(index),
            label: doc.name.clone(), <span class="c">// the document's name</span>
            count: rows.len(), <span class="c">// objects in this layer</span>
            hidden: all_hidden(scene, &amp;rows), <span class="c">// the layer's checkbox state</span>
        });
    }

    <span class="k">let</span> <span class="k">mut</span> kinds: Vec&lt;Kind&gt; = Vec::new();

    <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
        <span class="k">if</span> <span class="k">let</span> Some(geometry) = scene.geometry(row) {
            <span class="k">let</span> kind = Kind::of(geometry);

            <span class="k">if</span> !kinds.contains(&amp;kind) {
                kinds.push(kind);
            }
        }
    }

    kinds.sort();

    <span class="k">for</span> kind <span class="k">in</span> kinds {
        <span class="k">let</span> rows = of_layer(scene, Layer::Kind(kind));

        <span class="k">if</span> rows.is_empty() {
            <span class="k">continue</span>;
        }

        out.push(Row {
            layer: Layer::Kind(kind),
            label: kind.label().to_string(), <span class="c">// the kind's name</span>
            count: rows.len(), <span class="c">// objects in this layer</span>
            hidden: all_hidden(scene, &amp;rows), <span class="c">// the layer's checkbox state</span>
        });
    }

    out
}

<span class="c">/// The object rows of one layer.</span>
<span class="k">pub</span> <span class="k">fn</span> of_layer(scene: &amp;Scene, layer: Layer) -&gt; Vec&lt;u32&gt; {
    <span class="k">let</span> <span class="k">mut</span> rows = Vec::new();

    <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..scene.object_count() <span class="k">as</span> u32 {
        <span class="k">let</span> matches = <span class="k">match</span> layer {
            Layer::Document(index) =&gt; scene.identity_of(row).map(|(doc, _)| doc) == Some(index),
            Layer::Kind(kind) =&gt; scene.geometry(row).map(Kind::of) == Some(kind),
        };

        <span class="k">if</span> matches {
            rows.push(row);
        }
    }

    rows
}

<span class="c">/// True when every row of a layer is hidden.</span>
<span class="k">fn</span> all_hidden(scene: &amp;Scene, rows: &amp;[u32]) -&gt; bool {
    !rows.is_empty()
        &amp;&amp; rows.iter().all(|&amp;row| {
            scene
                .identity_of(row)
                .is_some_and(|identity| scene.hidden.contains(&amp;identity))
        })
}</code></pre></div>
<p><code>lessons/21/src/app/layers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::scene::FileDoc;
    <span class="k">use</span> session_rust::{Point, Polyline, Session, Xform};
    <span class="k">use</span> std::rc::Rc;

    <span class="c">/// Two files: a point and a polyline, then a point.</span>
    <span class="k">fn</span> scene_with_two_files() -&gt; Scene {
        <span class="k">let</span> <span class="k">mut</span> left = Session::new(&quot;<span class="s">left</span>&quot;);
        left.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        left.add_polyline(
            Polyline::new(vec![
                Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            ]),
            None,
        );
        <span class="k">let</span> <span class="k">mut</span> right = Session::new(&quot;<span class="s">right</span>&quot;);
        right.add_point(Point::new(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();

        <span class="k">for</span> (name, session) <span class="k">in</span> [(&quot;<span class="s">left</span>&quot;, left), (&quot;<span class="s">right</span>&quot;, right)] {
            scene.add_file(FileDoc {
                name: name.into(),
                session: Rc::new(session),
                place: Xform::identity(),
                point_px: <span class="s">0</span>.<span class="s">0</span>,
                display_only: <span class="s">false</span>,
            });
        }

        scene
    }

    <span class="c">/// Documents first, then only the kinds present.</span>
    #[test]
    <span class="k">fn</span> the_panel_lists_documents_then_the_kinds_present() {
        <span class="k">let</span> scene = scene_with_two_files();
        <span class="k">let</span> rows = rows(&amp;scene);
        <span class="k">let</span> labels: Vec&lt;&amp;str&gt; = rows.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec![&quot;<span class="s">left</span>&quot;, &quot;<span class="s">right</span>&quot;, &quot;<span class="s">curves</span>&quot;, &quot;<span class="s">points</span>&quot;]);
        assert_eq!(rows[<span class="s">0</span>].count, <span class="s">2</span>, &quot;<span class="s">left holds a point and a polyline</span>&quot;);
        assert_eq!(rows[<span class="s">3</span>].count, <span class="s">2</span>, &quot;<span class="s">one point in each file</span>&quot;);
    }

    <span class="c">/// A kind spans documents; a document is only its own.</span>
    #[test]
    <span class="k">fn</span> a_layer_names_the_rows_it_controls() {
        <span class="k">let</span> scene = scene_with_two_files();
        assert_eq!(of_layer(&amp;scene, Layer::Document(<span class="s">1</span>)), vec![<span class="s">2</span>]);
        assert_eq!(of_layer(&amp;scene, Layer::Kind(Kind::Points)), vec![<span class="s">0</span>, <span class="s">2</span>]);
        assert!(of_layer(&amp;scene, Layer::Kind(Kind::Clouds)).is_empty());
    }

    <span class="c">/// A key parses back to its layer.</span>
    #[test]
    <span class="k">fn</span> a_row_key_survives_the_round_trip() {
        <span class="k">for</span> layer <span class="k">in</span> [
            Layer::Document(<span class="s">0</span>),
            Layer::Document(<span class="s">17</span>),
            Layer::Kind(Kind::Clouds),
        ] {
            assert_eq!(Layer::from_key(&amp;layer.key()), Some(layer));
        }

        assert_eq!(Layer::from_key(&quot;<span class="s">kind:sandwiches</span>&quot;), None);
        assert_eq!(Layer::from_key(&quot;<span class="s">nonsense</span>&quot;), None);
    }

    <span class="c">/// A layer is hidden only when all its objects are.</span>
    #[test]
    <span class="k">fn</span> a_layer_is_hidden_when_all_of_it_is() {
        <span class="k">let</span> <span class="k">mut</span> scene = scene_with_two_files();
        assert!(!rows(&amp;scene)[<span class="s">1</span>].hidden);
        <span class="k">let</span> identity = scene.identity_of(<span class="s">2</span>).expect(&quot;<span class="s">row 2 exists</span>&quot;);
        scene.hidden.insert(identity);
        <span class="k">let</span> rows = rows(&amp;scene);
        assert!(rows[<span class="s">1</span>].hidden, &quot;<span class="s">the whole of \`right\` is hidden</span>&quot;);
        assert!(!rows[<span class="s">3</span>].hidden, &quot;<span class="s">only one of the two points is</span>&quot;);
    }
}</code></pre></div>
<h2 id="step-17-srcstateeditrs">Step 17 · src/state/edit.rs<a class="anchor" href="#/course/21-editing#step-17-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Editing connects commands and gumball previews to document history.</p>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="k">impl</span> State {
    <span class="c">/// Hide a layer, or show it when it is fully hidden.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_layer(&amp;<span class="k">mut</span> <span class="k">self</span>, layer: Layer) {
        <span class="k">let</span> rows = layers::of_layer(&amp;<span class="k">self</span>.scene, layer);

        <span class="k">if</span> rows.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> hidden: Vec&lt;bool&gt; = rows
            .iter()
            .map(|&amp;row| {
                <span class="k">self</span>.scene
                    .identity_of(row)
                    .is_some_and(|id| <span class="k">self</span>.scene.hidden.contains(&amp;id))
            })
            .collect();
        <span class="k">let</span> hide = !hidden.iter().all(|&amp;h| h);

        <span class="k">for</span> (&amp;row, was) <span class="k">in</span> rows.iter().zip(&amp;hidden) {
            <span class="k">if</span> *was == hide {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> Some(identity) = <span class="k">self</span>.scene.identity_of(row) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> hide {
                <span class="k">self</span>.scene.hidden.insert(identity);
            } <span class="k">else</span> {
                <span class="k">self</span>.scene.hidden.remove(&amp;identity);
            }

            <span class="k">self</span>.gpu.set_hidden(row, hide);
        }

        <span class="k">if</span> <span class="k">self</span>.scene.selected.is_some_and(|row| rows.contains(&amp;row)) &amp;&amp; hide {
            <span class="k">self</span>.select(None);
        }

        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Refill the layers panel, when it is open.</span>
    <span class="k">pub</span> <span class="k">fn</span> refresh_layers(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> !<span class="k">crate</span>::app::feedback::layers_open() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> rows: Vec&lt;<span class="k">crate</span>::app::feedback::LayerRow&gt; = layers::rows(&amp;<span class="k">self</span>.scene)
            .into_iter()
            .map(|row| <span class="k">crate</span>::app::feedback::LayerRow {
                key: row.layer.key(),
                label: row.label, <span class="c">// the layer's name</span>
                count: row.count, <span class="c">// objects in the layer</span>
                hidden: row.hidden, <span class="c">// the checkbox state</span>
            })
            .collect();
        <span class="k">crate</span>::app::feedback::layers_panel(&amp;rows);
    }
}

<span class="k">impl</span> State {
    <span class="c">/// L: open or close the layers panel.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_layers_panel(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> open = !<span class="k">crate</span>::app::feedback::layers_open();
        <span class="k">crate</span>::app::feedback::layers_visible(open);

        <span class="k">if</span> open {
            <span class="k">self</span>.refresh_layers();
        }

        <span class="k">self</span>.touch();
    }
}</code></pre></div>
<p><code>lessons/21/src/state/edit.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A control point drag in progress.</span>
<span class="k">pub</span> <span class="k">struct</span> ControlDrag {
    parent: u32, <span class="c">// the object's row</span>
    index: usize, <span class="c">// which dot in \`controls.points\`</span>
    id: ControlId,
    plane: CPlane, <span class="c">// the plane the point moves in</span>
}

<span class="k">impl</span> State {
    <span class="c">/// Grab the selected control point, if the press is on it.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_control_drag(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> SelectionMode::Controls {
            parent,
            selected: Some(id),
            ..
        } = <span class="k">self</span>.selection
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(index) = <span class="k">self</span>.controls.points.iter().position(|c| c.id == id) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> at = <span class="k">self</span>.controls.points[index].position;
        <span class="k">let</span> Some((sx, sy)) = <span class="k">self</span>.project(at) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> grab = GRAB_CSS * <span class="k">self</span>.pixel_scale(); <span class="c">// grab radius in device pixels</span>

        <span class="k">if</span> (sx - x).abs() &gt; grab || (sy - y).abs() &gt; grab {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> forward = <span class="k">self</span>.camera.orientation.rotate_vector(Vector::y_axis());
        <span class="k">self</span>.control_drag = Some(ControlDrag {
            parent,
            index,
            id,
            plane: CPlane::facing(&amp;forward),
        });
        <span class="s">true</span>
    }

    <span class="c">/// Move the control point with the pointer; a preview.</span>
    <span class="k">pub</span> <span class="k">fn</span> drag_control(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(active) = <span class="k">self</span>.control_drag.as_ref() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(point) = <span class="k">self</span>.control_target(active, x, y) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> index = active.index;
        <span class="k">self</span>.controls.points[index].position = [point[<span class="s">0</span>], point[<span class="s">1</span>], point[<span class="s">2</span>]];
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.touch();
        <span class="s">true</span>
    }

    <span class="c">/// Release: the geometry takes the moved control point.</span>
    <span class="k">pub</span> <span class="k">fn</span> end_control_drag(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(active) = <span class="k">self</span>.control_drag.take() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(point) = <span class="k">self</span>.control_target(&amp;active, x, y) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> index = <span class="k">match</span> active.id {
            ControlId::Curve { point, .. } =&gt; point,
            ControlId::Vertex(index) =&gt; index,
            _ =&gt; <span class="k">return</span> <span class="s">false</span>,
        };

        <span class="k">if</span> !<span class="k">self</span>.scene.set_control_point(active.parent, index, &amp;point) {
            <span class="k">self</span>.status(&quot;<span class="s">This geometry's control points cannot be edited</span>&quot;);
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.select(Some(active.parent));
        <span class="k">self</span>.enable_controls();
        <span class="k">self</span>.touch();
        <span class="s">true</span>
    }

    <span class="c">/// Where the dragged control point lands: a snap, or the plane.</span>
    <span class="k">fn</span> control_target(&amp;<span class="k">self</span>, active: &amp;ControlDrag, x: f64, y: f64) -&gt; Option&lt;Point&gt; {
        <span class="k">let</span> (from, dir) = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport())?;
        <span class="k">let</span> origin = {
            <span class="k">let</span> at = <span class="k">self</span>.controls.points[active.index].position;
            Point::new(at[<span class="s">0</span>], at[<span class="s">1</span>], at[<span class="s">2</span>])
        };
        <span class="k">let</span> free = active.plane.hit(&amp;origin, &amp;from, &amp;dir)?;
        <span class="k">let</span> <span class="k">mut</span> candidates = Vec::new();

        <span class="c">// the other control points are snap targets</span>
        <span class="k">for</span> (i, control) <span class="k">in</span> <span class="k">self</span>.controls.points.iter().enumerate() {
            <span class="k">if</span> i == active.index {
                <span class="k">continue</span>;
            }

            candidates.push(Snap {
                point: Point::new(
                    control.position[<span class="s">0</span>],
                    control.position[<span class="s">1</span>],
                    control.position[<span class="s">2</span>],
                ),
                kind: SnapKind::Vertex,
                owner: active.parent,
            });
        }

        <span class="c">// nearest on screen, within the aperture</span>
        <span class="k">let</span> project = |p: &amp;Point| <span class="k">self</span>.project([p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]);

        <span class="k">match</span> snap::best(
            &amp;candidates,
            (x, y),
            SNAP_APERTURE_PX * <span class="k">self</span>.pixel_scale(),
            project,
        ) {
            Some(hit) =&gt; Some(hit.point),
            None =&gt; Some(free),
        }
    }

    <span class="c">/// A world point in framebuffer pixels, or \`None\` when it is behind the eye.</span>
    <span class="k">fn</span> project(&amp;<span class="k">self</span>, at: [f64; <span class="s">3</span>]) -&gt; Option&lt;(f64, f64)&gt; {
        <span class="k">let</span> (w, h) = <span class="k">self</span>.viewport();
        <span class="k">let</span> anchor = Point::new(at[<span class="s">0</span>], at[<span class="s">1</span>], at[<span class="s">2</span>]);
        <span class="c">// anchored at the point, its clip position is the translation</span>
        <span class="k">let</span> mvp = <span class="k">self</span>.camera.view_proj_anchored(<span class="k">self</span>.aspect(), &amp;anchor);
        <span class="k">let</span> clip = [mvp.m[<span class="s">12</span>], mvp.m[<span class="s">13</span>], mvp.m[<span class="s">14</span>], mvp.m[<span class="s">15</span>]];

        <span class="k">if</span> clip[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> None;
        }

        Some((
            (clip[<span class="s">0</span>] / clip[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span> + <span class="s">0</span>.<span class="s">5</span>) * w,
            (<span class="s">0</span>.<span class="s">5</span> - clip[<span class="s">1</span>] / clip[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span>) * h,
        ))
    }
}

<span class="c">/// Grab radius of a control dot, CSS pixels.</span>
<span class="k">const</span> GRAB_CSS: f64 = <span class="s">10</span>.<span class="s">0</span>;

<span class="c">/// Snap reach, CSS pixels.</span>
<span class="k">const</span> SNAP_APERTURE_PX: f64 = <span class="s">12</span>.<span class="s">0</span>;

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// What the widget would draw, without a device.</span>
    #[test]
    <span class="c">/// The widget's row counts.</span>
    <span class="k">fn</span> the_widget_draws_three_arms_three_arcs_and_four_balls() {
        <span class="k">let</span> origin = Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>);
        <span class="k">let</span> (segments, glyphs) = widget_rows(&amp;origin, <span class="s">2</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">7</span>);

        assert_eq!(segments.ribbons.len(), <span class="s">3</span> + <span class="s">3</span> * ARC_STEPS <span class="k">as</span> usize);
        assert_eq!(glyphs.dots.len(), <span class="s">4</span>);
        assert!(
            segments.ribbons.iter().all(|r| r.instance_id == <span class="s">7</span>)
                &amp;&amp; glyphs.dots.iter().all(|d| d.instance_id == <span class="s">7</span>),
            &quot;<span class="s">every row draws against the identity instance, not an object's</span>&quot;
        );

        <span class="c">// The X arm runs ARM * per_px along +x from the origin.</span>
        <span class="k">let</span> x_arm = &amp;segments.ribbons[<span class="s">0</span>];
        assert_eq!(x_arm.p0, [<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>]);
        assert_eq!(x_arm.p1, [<span class="s">10</span>.<span class="s">0</span> + (ARM * <span class="s">2</span>.<span class="s">0</span>) <span class="k">as</span> f32, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>]);

        <span class="c">// arm and ball share one colour</span>
        <span class="k">let</span> unpacked = unpack_color(x_arm.color);
        assert_eq!(glyphs.dots[<span class="s">0</span>].color, unpacked);
        assert!(
            unpacked[<span class="s">0</span>] &gt; <span class="s">0</span>.<span class="s">8</span> &amp;&amp; unpacked[<span class="s">1</span>] &lt; <span class="s">0</span>.<span class="s">2</span> &amp;&amp; unpacked[<span class="s">2</span>] &lt; <span class="s">0</span>.<span class="s">2</span>,
            &quot;<span class="s">X is red on both sides, </span>{<span class="s">unpacked:?</span>}&quot;
        );
        assert_eq!(
            glyphs.dots[<span class="s">3</span>].color,
            [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
            &quot;<span class="s">the hub is white</span>&quot;
        );

        <span class="c">// The balls are screen-sized, which the lane reads as a NEGATIVE radius.</span>
        assert!(glyphs.dots.iter().all(|d| d.radius &lt; <span class="s">0</span>.<span class="s">0</span>));
    }

    <span class="c">/// The widget draws three colored arms.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="k">fn</span> the_widget_reaches_the_pixels() {
        <span class="k">use</span> <span class="k">crate</span>::camera::Camera;
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        <span class="k">use</span> session_rust::Xform;

        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">256</span>, <span class="s">256</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), <span class="s">0</span>));
        gpu.set_scene(&amp;upload);

        <span class="k">let</span> <span class="k">mut</span> camera = Camera::new();
        camera.target = [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>];
        camera.distance = <span class="s">0</span>.<span class="s">2</span>; <span class="c">// meters: 200 scene units</span>
        camera.update_position();
        <span class="k">let</span> rebase = gpu.rebase_anchor(&amp;camera.origin(), camera.distance_world(), <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> input = FrameInput {
            view_proj: camera.view_proj_anchored(<span class="s">1</span>.<span class="s">0</span>, &amp;rebase.anchor),
            clear: wgpu::Color::BLACK,
            now_ms: <span class="s">0</span>.<span class="s">0</span>,
        };
        <span class="k">let</span> before = gpu.render_offscreen(&amp;input);

        <span class="k">let</span> widget = gpu.widget_row();
        <span class="c">// arm length in world units at this zoom</span>
        <span class="k">let</span> (segments, glyphs) = widget_rows(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">0</span>, widget);
        gpu.set_widget_rows(&amp;segments, &amp;glyphs);
        <span class="k">let</span> after = gpu.render_offscreen(&amp;input);

        <span class="k">let</span> (<span class="k">mut</span> red, <span class="k">mut</span> green, <span class="k">mut</span> blue, <span class="k">mut</span> changed) = (<span class="s">0</span>, <span class="s">0</span>, <span class="s">0</span>, <span class="s">0</span>);

        <span class="k">for</span> (a, b) <span class="k">in</span> before.chunks_exact(<span class="s">4</span>).zip(after.chunks_exact(<span class="s">4</span>)) {
            <span class="k">if</span> a[..<span class="s">3</span>] == b[..<span class="s">3</span>] {
                <span class="k">continue</span>;
            }

            changed += <span class="s">1</span>;
            <span class="k">let</span> (r, g, bl) = (i32::from(b[<span class="s">0</span>]), i32::from(b[<span class="s">1</span>]), i32::from(b[<span class="s">2</span>]));

            <span class="k">if</span> r &gt; g + <span class="s">40</span> &amp;&amp; r &gt; bl + <span class="s">40</span> {
                red += <span class="s">1</span>;
            } <span class="k">else</span> <span class="k">if</span> g &gt; r + <span class="s">40</span> &amp;&amp; g &gt; bl + <span class="s">40</span> {
                green += <span class="s">1</span>;
            } <span class="k">else</span> <span class="k">if</span> bl &gt; r + <span class="s">40</span> &amp;&amp; bl &gt; g + <span class="s">40</span> {
                blue += <span class="s">1</span>;
            }
        }

        assert!(
            changed &gt; <span class="s">100</span>,
            &quot;<span class="s">the widget changed the picture: </span>{<span class="s">changed</span>}<span class="s"> pixels</span>&quot;
        );
        assert!(
            red &gt; <span class="s">10</span> &amp;&amp; green &gt; <span class="s">10</span> &amp;&amp; blue &gt; <span class="s">10</span>,
            &quot;<span class="s">three coloured arms: red </span>{<span class="s">red</span>}<span class="s">, green </span>{<span class="s">green</span>}<span class="s">, blue </span>{<span class="s">blue</span>}<span class="s">, of </span>{<span class="s">changed</span>}<span class="s"> changed</span>&quot;
        );
    }

    <span class="c">/// Every arc point lies on the hit-test circle.</span>
    #[test]
    <span class="c">/// Every arc point passes \`hit\`.</span>
    <span class="k">fn</span> the_arcs_are_where_the_hit_test_expects_them() {
        <span class="k">let</span> origin = Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> per_px = <span class="s">1</span>.<span class="s">0</span>;
        <span class="k">let</span> (segments, _) = widget_rows(&amp;origin, per_px, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>);
        <span class="c">// the Z arc, in the x/y plane</span>
        <span class="k">let</span> z_arc = &amp;segments.ribbons[<span class="s">3</span> + <span class="s">2</span> * ARC_STEPS <span class="k">as</span> usize..];
        assert_eq!(z_arc.len(), ARC_STEPS <span class="k">as</span> usize);

        <span class="k">for</span> segment <span class="k">in</span> z_arc {
            <span class="k">for</span> p <span class="k">in</span> [segment.p0, segment.p1] {
                <span class="k">let</span> r = (f64::from(p[<span class="s">0</span>]).powi(<span class="s">2</span>) + f64::from(p[<span class="s">1</span>]).powi(<span class="s">2</span>)).sqrt();
                assert!((r - ARM * per_px).abs() &lt; <span class="s">0</span>.<span class="s">5</span>, &quot;<span class="s">on the arm's circle: </span>{<span class="s">r</span>}&quot;);
                assert!(
                    p[<span class="s">0</span>] &lt;= <span class="s">1</span>e-<span class="s">3</span> &amp;&amp; p[<span class="s">1</span>] &lt;= <span class="s">1</span>e-<span class="s">3</span>,
                    &quot;<span class="s">in the quadrant hit() tests: </span>{<span class="s">p:?</span>}&quot;
                );
                assert!(p[<span class="s">2</span>].abs() &lt; <span class="s">1</span>e-<span class="s">6</span>, &quot;<span class="s">in the plane normal to Z</span>&quot;);
            }
        }
    }

    <span class="c">/// A CSS pixel is the same scene length on a 1x and a 2x display.</span>
    #[test]
    <span class="k">fn</span> a_css_pixel_is_worth_more_world_on_a_denser_display() {
        <span class="k">let</span> one_to_one = world_per_css_px(<span class="s">1000</span>.<span class="s">0</span>, <span class="s">800</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> retina = world_per_css_px(<span class="s">1000</span>.<span class="s">0</span>, <span class="s">1600</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>);
        assert!(
            (one_to_one - retina).abs() &lt; <span class="s">1</span>e-<span class="s">9</span>,
            &quot;<span class="s">the same CSS pixel, either way</span>&quot;
        );

        <span class="k">let</span> closer = world_per_css_px(<span class="s">500</span>.<span class="s">0</span>, <span class="s">800</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
        assert!(closer &lt; one_to_one, &quot;<span class="s">nearer camera, less world in a pixel</span>&quot;);
        <span class="c">// the answer scales with the distance</span>
        assert!(
            world_per_css_px(<span class="s">1</span>.<span class="s">0</span>, <span class="s">800</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>) * <span class="s">1000</span>.<span class="s">0</span> - world_per_css_px(<span class="s">1000</span>.<span class="s">0</span>, <span class="s">800</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)
                &lt; <span class="s">1</span>e-<span class="s">9</span>,
            &quot;<span class="s">the answer scales with the distance, so the distance must be in world units</span>&quot;
        );
        assert_eq!(
            world_per_css_px(<span class="s">1000</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">1</span>.<span class="s">0</span>,
            &quot;<span class="s">no surface, no answer</span>&quot;
        );
    }
}</code></pre></div>
<h2 id="step-18-srcappfeedbackrs">Step 18 · src/app/feedback.rs<a class="anchor" href="#/course/21-editing#step-18-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Feedback publishes status and panel information from the same application state.</p>
<p><code>lessons/21/src/app/feedback.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line of <code>lessons/20/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Open or close the command line.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="c">/// Show or hide the command box; returns it.</span>
<span class="k">pub</span> <span class="k">fn</span> command_line(open: bool) -&gt; Option&lt;web_sys::HtmlInputElement&gt; {
    <span class="k">use</span> wasm_bindgen::JsCast;
    <span class="k">let</span> document = web_sys::window()?.document()?;
    <span class="k">let</span> input: web_sys::HtmlInputElement = document
        .get_element_by_id(&quot;<span class="s">viewer-command</span>&quot;)?
        .dyn_into()
        .ok()?;

    <span class="k">if</span> open {
        input.set_hidden(<span class="s">false</span>);
        input.set_value(&quot;&quot;);
        <span class="k">let</span> _ = input.focus();
    } <span class="k">else</span> {
        input.set_hidden(<span class="s">true</span>);

        <span class="k">if</span> <span class="k">let</span> Some(canvas) = document.get_element_by_id(&quot;<span class="s">canvas</span>&quot;)
            &amp;&amp; <span class="k">let</span> Ok(canvas) = canvas.dyn_into::&lt;web_sys::HtmlElement&gt;()
        {
            <span class="k">let</span> _ = canvas.focus();
        }
    }

    Some(input)
}

<span class="c">/// Give the canvas keyboard focus.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> focus_canvas() {
    <span class="k">use</span> wasm_bindgen::JsCast;

    <span class="k">if</span> <span class="k">let</span> Some(document) = web_sys::window().and_then(|w| w.document())
        &amp;&amp; <span class="k">let</span> Some(canvas) = document.get_element_by_id(&quot;<span class="s">canvas</span>&quot;)
        &amp;&amp; <span class="k">let</span> Ok(canvas) = canvas.dyn_into::&lt;web_sys::HtmlElement&gt;()
    {
        <span class="k">let</span> _ = canvas.focus();
    }
}

<span class="c">/// No canvas on native.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> focus_canvas() {}

<span class="c">/// The command line, when it is open.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="c">/// The text typed in the command box.</span>
<span class="k">pub</span> <span class="k">fn</span> command_text() -&gt; Option&lt;String&gt; {
    <span class="k">use</span> wasm_bindgen::JsCast;
    <span class="k">let</span> input: web_sys::HtmlInputElement = web_sys::window()?
        .document()?
        .get_element_by_id(&quot;<span class="s">viewer-command</span>&quot;)?
        .dyn_into()
        .ok()?;
    (!input.hidden()).then(|| input.value())
}

<span class="c">/// No command line on native.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> command_line(_open: bool) {}

<span class="k">pub</span> <span class="k">struct</span> LayerRow {
    <span class="k">pub</span> key: String, <span class="c">// unique id of the row</span>
    <span class="k">pub</span> label: String, <span class="c">// text shown</span>
    <span class="k">pub</span> count: usize, <span class="c">// objects under it</span>
    <span class="k">pub</span> hidden: bool, <span class="c">// eye toggled off</span>
}

<span class="c">/// Replace the rows of the layers panel.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> layers_panel(rows: &amp;[LayerRow]) {
    <span class="k">let</span> Some(document) = web_sys::window().and_then(|w| w.document()) <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> Some(panel) = document.get_element_by_id(&quot;<span class="s">viewer-layers</span>&quot;) <span class="k">else</span> {
        <span class="k">return</span>;
    };
    panel.set_text_content(None);

    <span class="k">for</span> row <span class="k">in</span> rows {
        <span class="c">// one button per layer row</span>
        <span class="k">let</span> Ok(line) = document.create_element(&quot;<span class="s">button</span>&quot;) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> _ = line.set_attribute(&quot;<span class="s">type</span>&quot;, &quot;<span class="s">button</span>&quot;);
        <span class="k">let</span> _ = line.set_attribute(&quot;<span class="s">aria-pressed</span>&quot;, <span class="k">if</span> row.hidden { &quot;<span class="s">true</span>&quot; } <span class="k">else</span> { &quot;<span class="s">false</span>&quot; });
        <span class="k">let</span> _ = line.set_attribute(&quot;<span class="s">data-layer</span>&quot;, &amp;row.key);
        <span class="k">let</span> _ = line.set_attribute(
            &quot;<span class="s">style</span>&quot;,
            &quot;<span class="s">display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:1</span>&quot;,
        );

        <span class="k">if</span> row.hidden {
            <span class="k">let</span> _ = line.set_attribute(
                &quot;<span class="s">style</span>&quot;,
                &quot;<span class="s">display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:0.45</span>&quot;,
            );
        }

        <span class="k">let</span> mark = <span class="k">if</span> row.hidden { &quot;<span class="s">·</span>&quot; } <span class="k">else</span> { &quot;<span class="s">•</span>&quot; };
        line.set_text_content(Some(&amp;format!(&quot;{<span class="s">mark</span>}<span class="s"> </span>{}<span class="s"> (</span>{}<span class="s">)</span>&quot;, row.label, row.count)));
        <span class="k">let</span> _ = panel.append_child(&amp;line);
    }
}

<span class="c">/// Show or hide the panel; returns it so a caller can attach its one listener.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="c">/// Show or hide the layers panel; returns it.</span>
<span class="k">pub</span> <span class="k">fn</span> layers_visible(open: bool) -&gt; Option&lt;web_sys::Element&gt; {
    <span class="k">let</span> panel = web_sys::window()?
        .document()?
        .get_element_by_id(&quot;<span class="s">viewer-layers</span>&quot;)?;
    <span class="k">let</span> _ = <span class="k">if</span> open {
        panel.remove_attribute(&quot;<span class="s">hidden</span>&quot;)
    } <span class="k">else</span> {
        panel.set_attribute(&quot;<span class="s">hidden</span>&quot;, &quot;&quot;)
    };
    Some(panel)
}

<span class="c">/// Whether the layers panel is open.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> layers_open() -&gt; bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(&quot;<span class="s">viewer-layers</span>&quot;))
        .is_some_and(|panel| !panel.has_attribute(&quot;<span class="s">hidden</span>&quot;))
}

<span class="c">/// No panel on native.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> layers_panel(_rows: &amp;[LayerRow]) {}

<span class="c">/// No panel on native.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> layers_visible(_open: bool) {}

<span class="c">/// No panel on native.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> layers_open() -&gt; bool {
    <span class="s">false</span>
}</code></pre></div>
<h2 id="step-19-indexhtml">Step 19 · index.html<a class="anchor" href="#/course/21-editing#step-19-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Labels go in with <code>textContent</code>, so a document named after a tag cannot become markup.</p>
<p><code>lessons/21/index.html</code> · edit · type this</p>
<p>Replaces the 3 lines from <code>&lt;canvas id=&quot;canvas&quot; tabindex=&quot;0&quot;&gt;&lt;/canvas&gt;</code> of <code>lessons/20/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot; role=&quot;<span class="s">application</span>&quot; aria-label=&quot;<span class="s">3D viewer. Left click selects, right drag orbits, L opens the layers panel, colon opens the command line.</span>&quot;&gt;&lt;/canvas&gt;
    &lt;a id=&quot;<span class="s">viewer-docs</span>&quot; href=&quot;<span class="s">docs/</span>&quot; target=&quot;<span class="s">_blank</span>&quot; rel=&quot;<span class="s">noopener</span>&quot; title=&quot;<span class="s">Open the documentation</span>&quot; aria-label=&quot;<span class="s">Open the documentation</span>&quot;&gt;&lt;/a&gt;
    &lt;div id=&quot;<span class="s">viewer-status</span>&quot; role=&quot;<span class="s">status</span>&quot; aria-live=&quot;<span class="s">polite</span>&quot; style=&quot;<span class="s">position: fixed; bottom: 12px; left: 12px; color: #fff; background: #222b; font: 14px system-ui; padding: 4px 8px; pointer-events: none;</span>&quot;&gt;&lt;/div&gt;
    <span class="c">&lt;!-- The layers panel: one element, filled from Rust with textContent, never innerHTML.</span>
<span class="c">    \`L\` opens and closes it; a click on a row hides or shows that layer. --&gt;</span>
    &lt;div id=&quot;<span class="s">viewer-layers</span>&quot; hidden role=&quot;<span class="s">group</span>&quot; aria-label=&quot;<span class="s">Layers</span>&quot;
       style=&quot;<span class="s">position: fixed; top: 12px; left: 12px; min-width: 180px; max-height: 70vh; overflow: auto; color: #fff; background: #222d; font: 13px/1.7 system-ui; padding: 6px 0; border-radius: 4px; outline: 1px solid #555;</span>&quot;&gt;&lt;/div&gt;
    <span class="c">&lt;!-- The command line. Hidden until the colon key opens it, and the canvas takes the keyboard</span>
<span class="c">    back the moment it closes, so typing \`z\` in here is a letter and not an undo. --&gt;</span>
    &lt;input id=&quot;<span class="s">viewer-command</span>&quot; type=&quot;<span class="s">text</span>&quot; spellcheck=&quot;<span class="s">false</span>&quot; autocomplete=&quot;<span class="s">off</span>&quot; hidden
         aria-label=&quot;<span class="s">Command line</span>&quot;
         style=&quot;<span class="s">position: fixed; bottom: 44px; left: 12px; width: min(420px,60vw); color: #fff; background: #222d; border: 0; border-radius: 4px; font: 14px/1.6 ui-monospace,monospace; padding: 4px 8px; outline: 1px solid #555;</span>&quot;&gt;</code></pre></div>
<h2 id="step-20-cargotoml">Step 20 · Cargo.toml<a class="anchor" href="#/course/21-editing#step-20-cargotoml" aria-label="Link to this section">#</a></h2>
<p>The manifest adds the dependencies and browser features used by these edits.</p>
<p><code>lessons/21/Cargo.toml</code> · edit · type this</p>
<p>Replaces the 3 lines from <code>&quot;EventTarget&quot;,</code> of <code>lessons/20/Cargo.toml</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &quot;HtmlElement&quot;,
    &quot;HtmlInputElement&quot;,
    &quot;KeyboardEvent&quot;,
    &quot;EventTarget&quot;,
    &quot;Event&quot;,
    &quot;Location&quot;,
    &quot;History&quot;,</code></pre></div>
<h2 id="step-21-srcappinputrs">Step 21 · src/app/input.rs<a class="anchor" href="#/course/21-editing#step-21-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/21/src/app/input.rs</code> · edit · type this</p>
<p>Added after the <code>shift: bool,</code> line in <code>struct Input</code> of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    gizmo_drag: bool,
    control_drag: bool,</code></pre></div>
<p>Added after the <code>shift: false,</code> line in <code>fn new</code> of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            gizmo_drag: <span class="s">false</span>,
            control_drag: <span class="s">false</span>,</code></pre></div>
<p>Added after the <code>Key::Named(NamedKey::F10) =&gt; state.enable_con…</code> line in <code>fn key</code> of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Named(NamedKey::Delete) =&gt; state.delete_selected(),
            <span class="c">// colon opens the command line</span>
            Key::Character(&quot;<span class="s">:</span>&quot;) =&gt; {
                <span class="k">crate</span>::app::feedback::command_line(<span class="s">true</span>);
            }
            Key::Character(&quot;<span class="s">l</span>&quot; | &quot;<span class="s">L</span>&quot;) =&gt; state.toggle_layers_panel(),
            <span class="c">// Ctrl+Z undo, Ctrl+Shift+Z redo</span>
            Key::Character(&quot;<span class="s">z</span>&quot; | &quot;<span class="s">Z</span>&quot;) <span class="k">if</span> <span class="k">self</span>.ctrl =&gt; {
                <span class="k">if</span> <span class="k">self</span>.shift {
                    state.redo()
                } <span class="k">else</span> {
                    state.undo()
                }
            }
            Key::Character(&quot;<span class="s">y</span>&quot; | &quot;<span class="s">Y</span>&quot;) <span class="k">if</span> <span class="k">self</span>.ctrl =&gt; state.redo(),</code></pre></div>
<p>Added after the <code>winit::dpi::PhysicalPosition::new(position.x…</code> line in <code>fn mouse</code> of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> <span class="k">self</span>.control_drag {
                    <span class="k">self</span>.last_cursor = (position.x, position.y);
                    <span class="k">return</span> state.drag_control(position.x, position.y);
                }

                <span class="k">if</span> <span class="k">self</span>.gizmo_drag {
                    <span class="k">self</span>.last_cursor = (position.x, position.y);
                    <span class="k">return</span> state.drag_gizmo(position.x, position.y);
                }</code></pre></div>
<p>Replaces the 12 lines from <code>self.left_down = None;</code> in <code>fn cancel</code> of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gizmo_drag = <span class="s">false</span>;
        <span class="k">self</span>.control_drag = <span class="s">false</span>;
        <span class="k">self</span>.left_down = None;
        <span class="k">self</span>.touch = Touches::new();
    }

    <span class="c">/// Left button: control drag, then gizmo drag, then a click.</span>
    <span class="k">fn</span> left(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State, btn: ElementState) -&gt; bool {
        <span class="k">match</span> btn {
            ElementState::Pressed =&gt; {
                <span class="k">if</span> state.begin_control_drag(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>) {
                    <span class="k">self</span>.control_drag = <span class="s">true</span>;
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">if</span> state.begin_gizmo(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>) {
                    <span class="k">self</span>.gizmo_drag = <span class="s">true</span>;
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">self</span>.left_down = Some(<span class="k">self</span>.last_cursor);
                <span class="s">false</span>
            }
            ElementState::Released =&gt; {
                <span class="k">if</span> <span class="k">self</span>.control_drag {
                    <span class="k">self</span>.control_drag = <span class="s">false</span>;
                    state.end_control_drag(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>);
                    <span class="k">return</span> <span class="s">true</span>;
                }

                <span class="k">if</span> <span class="k">self</span>.gizmo_drag {
                    <span class="k">self</span>.gizmo_drag = <span class="s">false</span>;
                    state.end_gizmo(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>);
                    <span class="k">return</span> <span class="s">true</span>;
                }</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/20/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// the command box's own key listener</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="c">/// Key listener installed on the command box.</span>
<span class="k">pub</span> <span class="k">struct</span> CommandKeys {
    input: web_sys::HtmlInputElement,
    callback: wasm_bindgen::closure::Closure&lt;<span class="k">dyn</span> FnMut(web_sys::KeyboardEvent)&gt;, <span class="c">// runs on every key in the box</span>
    blur: wasm_bindgen::closure::Closure&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;, <span class="c">// runs when the box loses focus</span>
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> CommandKeys {
    <span class="c">/// Enter sends the line, Escape drops it.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        input: web_sys::HtmlInputElement,
        proxy: winit::event_loop::EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;,
    ) -&gt; Result&lt;<span class="k">Self</span>, wasm_bindgen::JsValue&gt; {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> box_ = input.clone();
        <span class="k">let</span> callback = wasm_bindgen::closure::Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::KeyboardEvent)&gt;::new(
            <span class="k">move</span> |event: web_sys::KeyboardEvent| <span class="k">match</span> event.key().as_str() {
                &quot;<span class="s">Enter</span>&quot; =&gt; {
                    <span class="k">let</span> line = box_.value();
                    <span class="k">crate</span>::app::feedback::command_line(<span class="s">false</span>);

                    <span class="k">if</span> !line.trim().is_empty() {
                        <span class="k">let</span> _ = proxy.send_event(<span class="k">crate</span>::Msg::Command(line));
                    }
                }
                &quot;<span class="s">Escape</span>&quot; =&gt; {
                    <span class="k">crate</span>::app::feedback::command_line(<span class="s">false</span>);
                }
                _ =&gt; {}
            },
        );
        input.add_event_listener_with_callback(&quot;<span class="s">keydown</span>&quot;, callback.as_ref().unchecked_ref())?;
        <span class="c">// Clicking away closes it. Otherwise the box keeps the keyboard with no key that</span>
        <span class="k">let</span> shut = input.clone();
        <span class="k">let</span> blur = wasm_bindgen::closure::Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;::new(
            <span class="k">move</span> |_: web_sys::Event| {
                <span class="k">if</span> !shut.hidden() {
                    <span class="k">crate</span>::app::feedback::command_line(<span class="s">false</span>);
                }
            },
        );
        input.add_event_listener_with_callback(&quot;<span class="s">blur</span>&quot;, blur.as_ref().unchecked_ref())?;
        Ok(<span class="k">Self</span> {
            input,
            callback,
            blur,
        })
    }
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> Drop <span class="k">for</span> CommandKeys {
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> _ = <span class="k">self</span>
            .input
            .remove_event_listener_with_callback(&quot;<span class="s">keydown</span>&quot;, <span class="k">self</span>.callback.as_ref().unchecked_ref());
        <span class="k">let</span> _ = <span class="k">self</span>
            .input
            .remove_event_listener_with_callback(&quot;<span class="s">blur</span>&quot;, <span class="k">self</span>.blur.as_ref().unchecked_ref());
    }
}

<span class="c">/// one click listener for the whole panel</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="c">/// Click listener installed on the layers panel.</span>
<span class="k">pub</span> <span class="k">struct</span> LayerClicks {
    panel: web_sys::Element,
    callback: wasm_bindgen::closure::Closure&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;,
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> LayerClicks {
    <span class="k">pub</span> <span class="k">fn</span> new(
        panel: web_sys::Element,
        proxy: winit::event_loop::EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;,
    ) -&gt; Result&lt;<span class="k">Self</span>, wasm_bindgen::JsValue&gt; {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> callback = wasm_bindgen::closure::Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;::new(
            <span class="k">move</span> |event: web_sys::Event| {
                <span class="k">let</span> Some(target) = event.target() <span class="k">else</span> { <span class="k">return</span> };
                <span class="k">let</span> Ok(element) = target.dyn_into::&lt;web_sys::Element&gt;() <span class="k">else</span> {
                    <span class="k">return</span>;
                };
                <span class="k">let</span> Some(key) = element.get_attribute(&quot;<span class="s">data-layer</span>&quot;) <span class="k">else</span> {
                    <span class="k">return</span>;
                };
                <span class="c">// A click in the panel takes the focus off the canvas, and every key binding</span>
                <span class="k">crate</span>::app::feedback::focus_canvas();
                <span class="k">let</span> _ = proxy.send_event(<span class="k">crate</span>::Msg::ToggleLayer(key));
            },
        );
        panel.add_event_listener_with_callback(&quot;<span class="s">click</span>&quot;, callback.as_ref().unchecked_ref())?;
        Ok(<span class="k">Self</span> { panel, callback })
    }
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> Drop <span class="k">for</span> LayerClicks {
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> _ = <span class="k">self</span>
            .panel
            .remove_event_listener_with_callback(&quot;<span class="s">click</span>&quot;, <span class="k">self</span>.callback.as_ref().unchecked_ref());
    }
}

<span class="c">/// Send the cancel message to the event loop.</span></code></pre></div>
<h2 id="step-22-srclibrs">Step 22 · src/lib.rs<a class="anchor" href="#/course/21-editing#step-22-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/21/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>CancelPointer,</code> line in <code>enum Msg</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Command(String), <span class="c">// A line typed into the command box, sent when Enter was pressed in it.</span>
    ToggleLayer(String), <span class="c">// A layers-panel row was clicked, carrying its key.</span></code></pre></div>
<p>Added after the <code>pointer_cancellation: Option&lt;app::input::Poin…</code> line in <code>struct App</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    command_keys: Option&lt;app::input::CommandKeys&gt;,
    layer_clicks: Option&lt;app::input::LayerClicks&gt;, <span class="c">// the panel's click listener</span></code></pre></div>
<p>Added after the <code>pointer_cancellation: None,</code> line in <code>fn run</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            command_keys: None,
            layer_clicks: None, <span class="c">// installed with the panel</span></code></pre></div>
<p>Added after the <code>if let Some((w, h)) = desired_canvas_size() {</code> line in <code>fn adopt</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> _ = state.resize(w, h);</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn resumed</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(input) = app::feedback::command_line(<span class="s">false</span>) {
                <span class="k">match</span> app::input::CommandKeys::new(input, proxy.clone()) {
                    Ok(listener) =&gt; <span class="k">self</span>.command_keys = Some(listener),
                    Err(error) =&gt; log::warn!(&quot;<span class="s">Cannot register the command line: </span>{<span class="s">error:?</span>}&quot;),
                }
            }

            <span class="k">if</span> <span class="k">let</span> Some(panel) = app::feedback::layers_visible(<span class="s">false</span>) {
                <span class="k">match</span> app::input::LayerClicks::new(panel, proxy.clone()) {
                    Ok(listener) =&gt; <span class="k">self</span>.layer_clicks = Some(listener),
                    Err(error) =&gt; log::warn!(&quot;<span class="s">Cannot register the layers panel: </span>{<span class="s">error:?</span>}&quot;),
                }
            }

            <span class="c">// async: GPU setup, then Msg::Ready</span></code></pre></div>
<p>Added after the <code>}</code> line in <code>fn user_event</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::Command(line) =&gt; {
                <span class="k">let</span> said = <span class="k">match</span> state.run_command(&amp;line) {
                    Ok(done) =&gt; done,
                    Err(why) =&gt; why,
                };
                app::feedback::status(&amp;said);
            }
            Msg::ToggleLayer(key) =&gt; {
                <span class="k">if</span> <span class="k">let</span> Some(layer) = app::layers::Layer::from_key(&amp;key) {
                    state.toggle_layer(layer);
                }
            }</code></pre></div>
<h2 id="step-23-srcstaters">Step 23 · src/state.rs<a class="anchor" href="#/course/21-editing#step-23-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/21/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>last_frame_ms: f64,</code> line in <code>struct State</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    last_resize_ms: f64,</code></pre></div>
<p>Added after the <code>sheet_generation: u64,</code> line in <code>struct State</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> gizmo: Option&lt;<span class="k">crate</span>::app::gizmo::Gizmo&gt;, <span class="c">// the move/rotate/scale widget</span>
    dragging: Option&lt;edit::GizmoDrag&gt;,
    control_drag: Option&lt;edit::ControlDrag&gt;,</code></pre></div>
<p>Added after the <code>last_frame_ms: 0.0,</code> line in <code>fn new</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            last_resize_ms: f64::NEG_INFINITY,</code></pre></div>
<p>Added after the <code>sheet_generation: 0,</code> line in <code>fn new</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            gizmo: None,
            dragging: None,
            control_drag: None,</code></pre></div>
<p>Added after the <code>self.annotate_document(first_row);</code> line in <code>fn append</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// An open panel is a view of the scene, and the scene just changed under it.</span>
        <span class="k">self</span>.refresh_layers();</code></pre></div>
<p>Added after the <code>self.scene.clear(&amp;mut self.gpu);</code> line in <code>fn clear</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.place_gizmo(None);
        <span class="k">self</span>.refresh_layers();</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl State</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Minimum time between two resizes.</span>
    <span class="k">const</span> RESIZE_HOLD_MS: f64 = <span class="s">100</span>.<span class="s">0</span>;

    <span class="c">/// Resize the GPU targets; false when asked too soon after the last one.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32) -&gt; bool {
        <span class="k">let</span> now = now_ms();

        <span class="c">// a window drag resizes every frame; wait between remakes</span>
        <span class="k">if</span> now - <span class="k">self</span>.last_resize_ms &lt; <span class="k">Self</span>::RESIZE_HOLD_MS {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">self</span>.last_resize_ms = now;
        <span class="k">self</span>.gpu.resize(width, height);
        <span class="k">self</span>.gpu.logical_size = <span class="k">self</span>.logical_size();
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.touch();
        <span class="s">true</span></code></pre></div>
<p>Added after the <code>self.scene.selected = row;</code> line in <code>fn select</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.place_gizmo(row);</code></pre></div>
<p>Added after the <code>self.gpu.set_hidden(row, true);</code> line in <code>fn hide_selected</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.refresh_layers();</code></pre></div>
<p>Added after the <code>self.scene.hidden.clear();</code> line in <code>fn show_all</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.refresh_layers();</code></pre></div>
<p>Replaces the 3 lines from <code>&amp;&amp; crate::engine::gpu::view::device_pixel_rat…</code> in <code>fn render</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// slow frames: drop to device scale 1 and no antialiasing</span>
            <span class="k">if</span> <span class="k">self</span>.gpu.performance.take_slow_interaction()
                &amp;&amp; (<span class="k">crate</span>::engine::gpu::view::device_pixel_ratio() &gt; <span class="s">1</span>.<span class="s">0</span>
                    || <span class="k">self</span>.gpu.targets.samples &gt; <span class="s">1</span>)
            {
                <span class="k">crate</span>::engine::gpu::view::reduce();
                <span class="k">self</span>.gpu
                    .resize(<span class="k">self</span>.gpu.config.width, <span class="k">self</span>.gpu.config.height);</code></pre></div>
<h2 id="step-24-srcenginegputargetsrs">Step 24 · src/engine/gpu/targets.rs<a class="anchor" href="#/course/21-editing#step-24-srcenginegputargetsrs" aria-label="Link to this section">#</a></h2>
<p>Targets own the depth and color attachments for a frame.</p>
<p><code>lessons/21/src/engine/gpu/targets.rs</code> · edit · type this</p>
<p>Replaces the 8 lines from <code>pub depth: wgpu::TextureView,</code> in <code>struct Targets</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The frame's depth and color textures at one sample count.</span>
<span class="k">pub</span> <span class="k">struct</span> Targets {
    <span class="k">pub</span> depth: Attachment,
    <span class="k">pub</span> msaa: Option&lt;Attachment&gt;, <span class="c">// multisampled color, only at 4x</span>
    <span class="k">pub</span> depth_single: wgpu::TextureView, <span class="c">// depth at 1x, or a 1x1 placeholder</span>
    <span class="k">pub</span> depth_msaa: wgpu::TextureView, <span class="c">// depth at 4x, or a 1x1 placeholder</span>
    <span class="k">pub</span> samples: u32, <span class="c">// MSAA samples, 1 or 4</span>
    <span class="k">pub</span> gradient: Attachment, <span class="c">// depth slope per pixel</span>
    <span class="k">pub</span> gradient_single: wgpu::TextureView, <span class="c">// gradient at 1x, or a placeholder</span>
    <span class="k">pub</span> gradient_msaa: wgpu::TextureView, <span class="c">// gradient at 4x, or a placeholder</span>
    _placeholders: [Attachment; <span class="s">2</span>], <span class="c">// the 1x1 textures, freed with the rest</span></code></pre></div>
<p>Replaces the <code>texture_view(</code> line in <code>fn new</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Attachment::new(</code></pre></div>
<p>Replaces the 3 lines from <code>(depth.clone(), empty_depth)</code> in <code>fn new</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            (depth.view.clone(), empty_depth.view.clone())
        } <span class="k">else</span> {
            (empty_depth.view.clone(), depth.view.clone())</code></pre></div>
<p>Replaces the 3 lines from <code>(gradient.clone(), empty_gradient)</code> in <code>fn new</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            (gradient.view.clone(), empty_gradient.view.clone())
        } <span class="k">else</span> {
            (empty_gradient.view.clone(), gradient.view.clone())</code></pre></div>
<p>Added after the <code>samples,</code> line in <code>fn new</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            _placeholders: [empty_depth, empty_gradient],
        }
    }

    <span class="c">/// Free every texture now.</span>
    <span class="k">pub</span> <span class="k">fn</span> destroy(&amp;<span class="k">self</span>) {
        <span class="k">self</span>.depth.destroy();
        <span class="k">self</span>.gradient.destroy();

        <span class="k">if</span> <span class="k">let</span> Some(msaa) = &amp;<span class="k">self</span>.msaa {
            msaa.destroy();
        }

        <span class="k">for</span> placeholder <span class="k">in</span> &amp;<span class="k">self</span>._placeholders {
            placeholder.destroy();</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Sample count: 4x only with solids, within budget, below device scale 2.</span></code></pre></div>
<p>Replaces the 5 lines from <code>if let Some(s) = forced {</code> in <code>fn samples_for</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a page that lost its device stays at 1x</span>
        <span class="k">if</span> super::view::reduced() {
            <span class="k">return</span> <span class="s">1</span>;
        }

        <span class="k">if</span> <span class="k">let</span> Some(s) = forced {
            <span class="k">return</span> <span class="k">if</span> s == <span class="s">4</span> { <span class="s">4</span> } <span class="k">else</span> { <span class="s">1</span> };
        }

        <span class="k">if</span> pixel_scale &gt;= MSAA_MAX_PIXEL_SCALE {</code></pre></div>
<p>Replaces the <code>let target = self.msaa.as_ref().unwrap_or(view);</code> line in <code>fn begin_faces</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> target = <span class="k">self</span>.msaa.as_deref().unwrap_or(view);</code></pre></div>
<p>Replaces the <code>let (target, resolve) = match &amp;self.msaa {</code> line in <code>fn begin_ink</code> of <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// at 4x the pass resolves into the canvas here</span>
        <span class="k">let</span> (target, resolve) = <span class="k">match</span> <span class="k">self</span>.msaa.as_deref() {</code></pre></div>
<p>Replaces <code>fn texture_view</code> in <code>lessons/20/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A texture and its view; dropping it frees the memory at once.</span>
<span class="k">pub</span> <span class="k">struct</span> Attachment {
    texture: wgpu::Texture,
    <span class="k">pub</span> view: wgpu::TextureView,
}

<span class="k">impl</span> Attachment {
    <span class="c">/// Create a texture from \`spec\` and its default view.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, label: &amp;str, spec: &amp;TextureSpec) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> texture = texture(ctx, label, spec);
        <span class="k">let</span> view = texture.create_view(&amp;wgpu::TextureViewDescriptor::default());
        <span class="k">Self</span> { texture, view }
    }

    <span class="c">/// The texture itself, for copies.</span>
    <span class="k">pub</span> <span class="k">fn</span> texture(&amp;<span class="k">self</span>) -&gt; &amp;wgpu::Texture {
        &amp;<span class="k">self</span>.texture
    }

    <span class="c">/// Free the memory now; destroying twice is fine.</span>
    <span class="k">pub</span> <span class="k">fn</span> destroy(&amp;<span class="k">self</span>) {
        <span class="k">self</span>.texture.destroy();
    }
}

<span class="c">/// An Attachment can be used wherever a view is expected.</span>
<span class="k">impl</span> std::ops::Deref <span class="k">for</span> Attachment {
    <span class="k">type</span> Target = wgpu::TextureView;

    <span class="k">fn</span> deref(&amp;<span class="k">self</span>) -&gt; &amp;wgpu::TextureView {
        &amp;<span class="k">self</span>.view
    }
}

<span class="c">/// Free the texture on drop.</span>
<span class="k">impl</span> Drop <span class="k">for</span> Attachment {
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.texture.destroy();
    }</code></pre></div>
<h2 id="step-25-srcenginegpumodrs">Step 25 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/21-editing#step-25-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/21/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the <code>self.control_net.release(&amp;self.ctx, &amp;self.lay…</code> line in <code>fn release</code> of <code>lessons/20/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gizmo_arms.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.gizmo_dots.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);</code></pre></div>
<h2 id="step-26-srcenginegpupickrs">Step 26 · src/engine/gpu/pick.rs<a class="anchor" href="#/course/21-editing#step-26-srcenginegpupickrs" aria-label="Link to this section">#</a></h2>
<p>Picking reads an object and subobject ID asynchronously.</p>
<p><code>lessons/21/src/engine/gpu/pick.rs</code> · edit · type this</p>
<p>Replaces the <code>use super::targets::{TextureSpec, texture, te…</code> line of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::targets::{Attachment, TextureSpec};</code></pre></div>
<p>Replaces the 4 lines from <code>id: wgpu::Texture,</code> in <code>struct IdTargets</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    id: Attachment, <span class="c">// object and sub id per pixel</span>
    depth: Attachment, <span class="c">// depth per pixel</span>
    gradient: Attachment, <span class="c">// depth slope per pixel</span></code></pre></div>
<p>Replaces the <code>view: &amp;target.id_view,</code> line in <code>fn begin_source</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                view: &amp;target.id,</code></pre></div>
<p>Replaces the <code>let id = texture(</code> line in <code>fn begin_pass</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> id = Attachment::new(</code></pre></div>
<p>Replaces the 2 lines from <code>let id_view = id.create_view(&amp;wgpu::TextureVi…</code> in <code>fn begin_pass</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> depth = Attachment::new(</code></pre></div>
<p>Replaces the <code>let gradient = texture_view(</code> line in <code>fn begin_pass</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> gradient = Attachment::new(</code></pre></div>
<p>Delete the <code>view: &amp;t.id_view,</code> line in <code>fn begin_pass</code> of <code>lessons/20/src/engine/gpu/pick.rs</code>.</p>
<p>Replaces the <code>view: &amp;t.id_view,</code> line in <code>fn begin_pass</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    view: &amp;t.id,</code></pre></div>
<p>Replaces the <code>view: &amp;targets.id_view,</code> line in <code>fn begin_ink</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                view: &amp;targets.id,</code></pre></div>
<p>Replaces the <code>texture: &amp;t.id,</code> line in <code>fn copy_window</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                texture: t.id.texture(),</code></pre></div>
<p>Replaces the <code>texture: &amp;target.id,</code> line in <code>fn copy_window</code> of <code>lessons/20/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                texture: target.id.texture(),</code></pre></div>
<h2 id="step-27-srcenginegpusplatrs">Step 27 · src/engine/gpu/splat.rs<a class="anchor" href="#/course/21-editing#step-27-srcenginegpusplatrs" aria-label="Link to this section">#</a></h2>
<p>The splat pass chooses visible points before compositing their color and depth.</p>
<p><code>lessons/21/src/engine/gpu/splat.rs</code> · edit · type this</p>
<p>Replaces the <code>use super::targets::{TextureSpec, texture_view};</code> line of <code>lessons/20/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::targets::{Attachment, TextureSpec};</code></pre></div>
<p>Replaces the 2 lines from <code>depth: wgpu::TextureView,</code> in <code>struct SplatTargets</code> of <code>lessons/20/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    depth: Attachment, <span class="c">// nearest point per pixel</span>
    color: Attachment,</code></pre></div>
<p>Replaces the <code>let depth = texture_view(</code> line in <code>fn new</code> of <code>lessons/20/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> depth = Attachment::new(</code></pre></div>
<p>Replaces the <code>let color = texture_view(</code> line in <code>fn new</code> of <code>lessons/20/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> color = Attachment::new(</code></pre></div>
<h2 id="step-28-srcenginegpusurface_outliners">Step 28 · src/engine/gpu/surface_outline.rs<a class="anchor" href="#/course/21-editing#step-28-srcenginegpusurface_outliners" aria-label="Link to this section">#</a></h2>
<p>Surface masks add outlines around visible coverage.</p>
<p><code>lessons/21/src/engine/gpu/surface_outline.rs</code> · edit · type this</p>
<p>Replaces the 9 lines from <code>use super::targets::{Targets, TextureSpec, te…</code> of <code>lessons/20/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::targets::{Attachment, Targets, TextureSpec};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

<span class="c">/// One coverage mask: where the surfaces are on screen.</span>
<span class="k">struct</span> Mask {
    resolved: Attachment, <span class="c">// coverage, one sample per pixel</span>
    multisampled: Option&lt;Attachment&gt;, <span class="c">// coverage at the scene's sample count</span>
    group: wgpu::BindGroup, <span class="c">// mask, radius and coarse mask, for the compositor</span>
    coarse: Attachment, <span class="c">// max of each 16x16 block; empty blocks are skipped</span></code></pre></div>
<p>Replaces the 3 lines from <code>let resolved = texture_view(ctx, &quot;selection c…</code> in <code>fn prepare</code> of <code>lessons/20/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> resolved = Attachment::new(ctx, &quot;<span class="s">selection coverage</span>&quot;, &amp;spec);
            <span class="k">let</span> multisampled = <span class="k">if</span> samples &gt; <span class="s">1</span> {
                Some(Attachment::new(</code></pre></div>
<p>Replaces the <code>let coarse = texture_view(</code> line in <code>fn prepare</code> of <code>lessons/20/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> coarse = Attachment::new(</code></pre></div>
<h2 id="step-29-srcenginegputriangle_tilesrs">Step 29 · src/engine/gpu/triangle_tiles.rs<a class="anchor" href="#/course/21-editing#step-29-srcenginegputriangle_tilesrs" aria-label="Link to this section">#</a></h2>
<p>Triangle tiles limit visibility queries to finite projected geometry.</p>
<p><code>lessons/21/src/engine/gpu/triangle_tiles.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>use super::buffers::{GpuCtx, ROWS, bind_group…</code> of <code>lessons/20/src/engine/gpu/triangle_tiles.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Splits the screen into tiles and lists which triangles touch each tile, so a pixel tests a few triangles instead of all.</span>
<span class="k">use</span> super::buffers::{GpuCtx, ROWS, bind_group, replace_buffer, uniform_buffer, zeroed_buffer};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> super::targets::{Attachment, TextureSpec};</code></pre></div>
<p>Replaces the <code>target: Option&lt;wgpu::TextureView&gt;,</code> line in <code>struct TriangleTiles</code> of <code>lessons/20/src/engine/gpu/triangle_tiles.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    target: Option&lt;Attachment&gt;, <span class="c">// one pixel per tile, drawn into but never read</span></code></pre></div>
<p>Replaces the 5 lines from <code>self.projected = zeroed_buffer(</code> in <code>fn prepare</code> of <code>lessons/20/src/engine/gpu/triangle_tiles.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            replace_buffer(
                &amp;<span class="k">mut</span> <span class="k">self</span>.projected,
                zeroed_buffer(
                    &amp;ctx.device,
                    &quot;<span class="s">triangle.projected</span>&quot;,
                    triangles <span class="k">as</span> u64 * PROJECTED_BYTES,
                    ROWS,
                ),</code></pre></div>
<p>Replaces the 24 lines from <code>self.buffer = zeroed_buffer(</code> in <code>fn prepare</code> of <code>lessons/20/src/engine/gpu/triangle_tiles.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            replace_buffer(
                &amp;<span class="k">mut</span> <span class="k">self</span>.buffer,
                zeroed_buffer(
                    &amp;ctx.device,
                    &quot;<span class="s">triangle.tiles</span>&quot;,
                    layout.buffer_bytes(pool_words).min(limit),
                    ROWS,
                ),
            );
            <span class="k">self</span>.target = Some(Attachment::new(
                ctx,
                &quot;<span class="s">triangle.tiles.target</span>&quot;,
                &amp;TextureSpec {
                    size: (layout.width, layout.height),
                    format: wgpu::TextureFormat::R8Unorm,
                    samples: <span class="s">1</span>,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                },
            ));</code></pre></div>
<p>Replaces the 2 lines from <code>self.buffer = zeroed_buffer(&amp;ctx.device, &quot;tri…</code> in <code>fn release_data</code> of <code>lessons/20/src/engine/gpu/triangle_tiles.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        replace_buffer(
            &amp;<span class="k">mut</span> <span class="k">self</span>.buffer,
            zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.tiles</span>&quot;, <span class="s">16</span>, ROWS),
        );
        replace_buffer(
            &amp;<span class="k">mut</span> <span class="k">self</span>.projected,
            zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.projected</span>&quot;, PROJECTED_BYTES, ROWS),
        );</code></pre></div>
<h2 id="step-30-srcenginegputext_planers">Step 30 · src/engine/gpu/text_plane.rs<a class="anchor" href="#/course/21-editing#step-30-srcenginegputext_planers" aria-label="Link to this section">#</a></h2>
<p>Plane text projects labels through their scene placement.</p>
<p><code>lessons/21/src/engine/gpu/text_plane.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>_texture: wgpu::Texture,</code> in <code>struct CachedPlane</code> of <code>lessons/20/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    texture: wgpu::Texture, <span class="c">// glyph coverage, one byte per pixel</span>
    bind: wgpu::BindGroup, <span class="c">// texture and sampler</span>
}

<span class="k">impl</span> Drop <span class="k">for</span> CachedPlane {
    <span class="c">/// Free the texture at once.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.texture.destroy();
    }
}

<span class="c">/// Every plane label: cached textures, and this frame's quads.</span></code></pre></div>
<p>Replaces the <code>_texture: texture,</code> line in <code>impl Planes</code> of <code>lessons/20/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    texture,</code></pre></div>
<h2 id="step-31-srcenginegpubuffersrs">Step 31 · src/engine/gpu/buffers.rs<a class="anchor" href="#/course/21-editing#step-31-srcenginegpubuffersrs" aria-label="Link to this section">#</a></h2>
<p>Growable buffers keep existing rows while new geometry arrives.</p>
<p><code>lessons/21/src/engine/gpu/buffers.rs</code> · edit · type this</p>
<p>Replaces the <code>self.buf = nb;</code> line in <code>fn grow</code> of <code>lessons/20/src/engine/gpu/buffers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        replace_buffer(&amp;<span class="k">mut</span> <span class="k">self</span>.buf, nb);</code></pre></div>
<p>Replaces the <code>self.buf = zeroed_buffer(&amp;ctx.device, self.la…</code> line in <code>fn release</code> of <code>lessons/20/src/engine/gpu/buffers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        replace_buffer(
            &amp;<span class="k">mut</span> <span class="k">self</span>.buf,
            zeroed_buffer(&amp;ctx.device, <span class="k">self</span>.label, <span class="k">self</span>.stride, <span class="k">self</span>.usage),
        );</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/20/src/engine/gpu/buffers.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Swap in \`fresh\` and free the old buffer now.</span>
<span class="k">pub</span> <span class="k">fn</span> replace_buffer(slot: &amp;<span class="k">mut</span> wgpu::Buffer, fresh: wgpu::Buffer) {
    std::mem::replace(slot, fresh).destroy();
}

<span class="c">/// A small buffer holding one \`T\` for shaders to read.</span></code></pre></div>
<h2 id="step-32-srcenginegpupresentrs">Step 32 · src/engine/gpu/present.rs<a class="anchor" href="#/course/21-editing#step-32-srcenginegpupresentrs" aria-label="Link to this section">#</a></h2>
<p>Presentation acquires the frame and submits rendering work.</p>
<p><code>lessons/21/src/engine/gpu/present.rs</code> · edit · type this</p>
<p>Added after the <code>pub fn render_ids_offscreen(&amp;mut self, input:…</code> line in <code>impl Gpu</code> of <code>lessons/20/src/engine/gpu/present.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a pending pick would fight over the id textures</span>
        <span class="k">self</span>.pick.cancel();</code></pre></div>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/21/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/21/src/text_quality.rs</code></li>
</ul>
<h2 id="step-33-srcstaters">Step 33 · src/state.rs<a class="anchor" href="#/course/21-editing#step-33-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/21/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl State</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Canvas size in CSS pixels; device pixels natively.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> logical_size(&amp;<span class="k">self</span>) -&gt; [f64; <span class="s">2</span>] {</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn enable_controls</code> of <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the gizmo would take the same clicks</span>
        <span class="k">self</span>.place_gizmo(None);</code></pre></div>
<p>Replaces <code>fn upload_controls</code> in <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Send the control dots and their links to the GPU.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> upload_controls(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="c">// start empty, so nothing doubles</span></code></pre></div>
<h2 id="step-34-srclibrs">Step 34 · src/lib.rs<a class="anchor" href="#/course/21-editing#step-34-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/21/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>Msg::CancelPointer =&gt; {</code> line in <code>fn user_event</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                state.cancel_gesture();</code></pre></div>
<p>Replaces the 4 lines from <code>if let Some((w, h)) = desired_canvas_size()</code> in <code>fn window_event</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="c">// resize first; a resize not ready yet holds the frame</span>
                <span class="k">let</span> held = <span class="k">match</span> desired_canvas_size() {
                    Some((w, h)) <span class="k">if</span> (w, h) != (state.gpu.config.width, state.gpu.config.height) =&gt; {
                        !state.resize(w, h)
                    }
                    _ =&gt; <span class="s">false</span>,
                };

                <span class="k">if</span> held {
                    state.needs_frame = <span class="s">true</span>;
                } <span class="k">else</span> {
                    state.render();</code></pre></div>
<h2 id="step-35-srcenginegpuviewrs">Step 35 · src/engine/gpu/view.rs<a class="anchor" href="#/course/21-editing#step-35-srcenginegpuviewrs" aria-label="Link to this section">#</a></h2>
<p>View settings control display features without changing source geometry.</p>
<p><code>lessons/21/src/engine/gpu/view.rs</code> · edit · type this</p>
<p>Replaces <code>fn reduce_for_slow_frames</code> in <code>lessons/20/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Drop to device scale 1 without MSAA until reload.</span>
<span class="k">pub</span> <span class="k">fn</span> reduce() {
    REDUCED.store(<span class="s">true</span>, std::sync::atomic::Ordering::Relaxed);
}

<span class="c">/// True once \`reduce\` was called.</span></code></pre></div>
<h2 id="step-36-srcstaters">Step 36 · src/state.rs<a class="anchor" href="#/course/21-editing#step-36-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/21/src/state.rs</code> · edit · type this</p>
<p>Replaces <code>fn render_position</code> in <code>lessons/20/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A position as the f32 the GPU takes.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> render_position(position: [f64; <span class="s">3</span>]) -&gt; [f32; <span class="s">3</span>] {</code></pre></div>
<h2 id="step-37-srcapprouters">Step 37 · src/app/route.rs<a class="anchor" href="#/course/21-editing#step-37-srcapprouters" aria-label="Link to this section">#</a></h2>
<p>Route helpers read viewer options from the page URL.</p>
<p><code>lessons/21/src/app/route.rs</code> · edit · type this</p>
<p>Replaces the <code>if !message.contains(&quot;device lost&quot;) || query(…</code> line in <code>fn recover_from_device_loss</code> of <code>lessons/20/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Reload once at reduced quality after a lost GPU device.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> recover_from_device_loss(message: &amp;str) -&gt; bool {
    <span class="k">if</span> !message.contains(&quot;<span class="s">device lost</span>&quot;) || <span class="k">crate</span>::engine::gpu::view::reduced() {</code></pre></div>
<p>Replaces the 18 lines from <code>let kept: Vec&lt;&amp;str&gt; = search</code> in <code>fn recover_from_device_loss</code> of <code>lessons/20/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> query = query_without(&amp;search, &quot;<span class="s">recovered</span>&quot;);

    <span class="k">if</span> !query.is_empty() {
        query.push('<span class="s">&amp;</span>');
    }

    <span class="k">let</span> reason: String = message.chars().take(<span class="s">200</span>).collect();
    query.push_str(&quot;<span class="s">recovered=</span>&quot;);
    query.push_str(&amp;String::from(js_sys::encode_uri_component(&amp;reason)));</code></pre></div>
<p>Replaces the 6 lines from <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> of <code>lessons/20/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The query string without \`?\` and without \`name=\`.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> query_without(search: &amp;str, name: &amp;str) -&gt; String {
    <span class="k">let</span> prefix = format!(&quot;{<span class="s">name</span>}<span class="s">=</span>&quot;);
    <span class="k">let</span> kept: Vec&lt;&amp;str&gt; = search
        .strip_prefix('<span class="s">?</span>')
        .unwrap_or(&quot;&quot;)
        .split('<span class="s">&amp;</span>')
        .filter(|pair| !pair.is_empty() &amp;&amp; !pair.starts_with(&amp;prefix) &amp;&amp; *pair != name)
        .collect();
    kept.join(&quot;<span class="s">&amp;</span>&quot;)
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">static</span> RECOVERED: std::sync::OnceLock&lt;String&gt; = std::sync::OnceLock::new();

<span class="c">/// After a recovery reload: draw reduced, clean the URL, return the notice.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> adopt_recovery() -&gt; Option&lt;&amp;'static str&gt; {
    <span class="k">let</span> reason = query(&quot;<span class="s">recovered</span>&quot;)?;
    <span class="k">crate</span>::engine::gpu::view::reduce();

    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window() {
        <span class="k">let</span> location = window.location();

        <span class="k">if</span> <span class="k">let</span> (Ok(path), Ok(search), Ok(hash), Ok(history)) = (
            location.pathname(),
            location.search(),
            location.hash(),
            window.history(),
        ) {
            <span class="k">let</span> query = query_without(&amp;search, &quot;<span class="s">recovered</span>&quot;);
            <span class="k">let</span> url = <span class="k">if</span> query.is_empty() {
                format!(&quot;{<span class="s">path</span>}{<span class="s">hash</span>}&quot;)
            } <span class="k">else</span> {
                format!(&quot;{<span class="s">path</span>}<span class="s">?</span>{<span class="s">query</span>}{<span class="s">hash</span>}&quot;)
            };
            <span class="k">let</span> _ = history.replace_state_with_url(&amp;wasm_bindgen::JsValue::NULL, &quot;&quot;, Some(&amp;url));
        }
    }

    <span class="k">let</span> reason = <span class="k">if</span> reason.is_empty() {
        &quot;<span class="s">WebGPU device lost</span>&quot;.to_string()
    } <span class="k">else</span> {
        reason
    };
    <span class="k">let</span> notice =
        format!(&quot;{<span class="s">reason</span>}<span class="s">; drawing at device scale 1 without antialiasing until the next reload</span>&quot;);
    Some(RECOVERED.get_or_init(|| notice).as_str())
}

<span class="c">/// The recovery notice, if this page reloaded after a device loss.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> recovered_notice() -&gt; Option&lt;&amp;'static str&gt; {
    RECOVERED.get().map(String::as_str)</code></pre></div>
<h2 id="step-38-srclibrs">Step 38 · src/lib.rs<a class="anchor" href="#/course/21-editing#step-38-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/21/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>fn run_web</code> of <code>lessons/20/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// after a GPU-loss reload, show the notice</span>
    <span class="k">if</span> <span class="k">let</span> Some(notice) = app::route::adopt_recovery() {
        app::feedback::status(notice);
    }</code></pre></div>
<h2 id="step-39-srcenginegpudevicers">Step 39 · src/engine/gpu/device.rs<a class="anchor" href="#/course/21-editing#step-39-srcenginegpudevicers" aria-label="Link to this section">#</a></h2>
<p>Device setup chooses supported limits and reports GPU failures.</p>
<p><code>lessons/21/src/engine/gpu/device.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>device.set_device_lost_callback(move |reason,…</code> in <code>fn open</code> of <code>lessons/20/src/engine/gpu/device.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// ask for a frame so the loss is seen</span>
        <span class="k">let</span> redraw = window.clone();
        device.set_device_lost_callback(<span class="k">move</span> |reason, message| {
            remember_device_loss(&amp;lost, reason, &amp;message);

            <span class="k">if</span> <span class="k">let</span> Some(window) = &amp;redraw {
                window.request_redraw();
            }</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/21/</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/21-editing#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/21/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: Selecting an object shows a gumball, and a drag or typed command records an undoable edit; status: <strong>the status names the selected object</strong>.</p>
<p><a href="/session/docs/course/docs/screenshots/21-editing-overview.png"><img src="/session/docs/course/docs/screenshots/21-editing-overview.png" alt="Full viewer result for 21 editing" loading="lazy" decoding="async"></a></p>
<p>If it fails:</p>
<ul>
<li>A drag jumps on release: the world delta is applied as a local transform.</li>
<li>A cancelled drag leaves an object moved: the GPU preview is not restored.</li>
<li>A control marker moves before the shape changes: the source is committed on release.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/21-editing#what-changed" aria-label="Link to this section">#</a></h2>
<p>Every file at this point: <code>lessons/21/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/21-editing#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/22-runtime-helpers">22 · Refresh diagnostics and resource checks</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/21-editing#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Select the placed polyline, press <strong>7</strong>, <strong>L</strong> and <strong>:</strong>; the gumball, layers and command field appear.</p>
<p><a href="/session/docs/course/docs/screenshots/21-editing-overview.png"><img src="/session/docs/course/docs/screenshots/21-editing-overview.png" alt="Full viewer result for 21 editing" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappmodrs",text:"Step 1 · src/app/mod.rs"},{level:2,id:"step-2-srcstaters",text:"Step 2 · src/state.rs"},{level:2,id:"step-3-srcappcplaners",text:"Step 3 · src/app/cplane.rs"},{level:2,id:"step-4-srcappcoordsrs",text:"Step 4 · src/app/coords.rs"},{level:2,id:"step-5-srccamerars",text:"Step 5 · src/camera.rs"},{level:2,id:"step-6-srcappgizmors",text:"Step 6 · src/app/gizmo.rs"},{level:2,id:"step-7-srcappsnaprs",text:"Step 7 · src/app/snap.rs"},{level:2,id:"step-8-srcappeditrs",text:"Step 8 · src/app/edit.rs"},{level:2,id:"step-9-srcappsceners",text:"Step 9 · src/app/scene.rs"},{level:2,id:"step-10-srcenginegpuobjectsrs",text:"Step 10 · src/engine/gpu/objects.rs"},{level:2,id:"step-11-srcenginegpumodrs",text:"Step 11 · src/engine/gpu/mod.rs"},{level:2,id:"step-12-srcenginegpurenderrs",text:"Step 12 · src/engine/gpu/render.rs"},{level:2,id:"step-13-srcstateeditrs",text:"Step 13 · src/state/edit.rs"},{level:2,id:"step-14-srcappcommandrs",text:"Step 14 · src/app/command.rs"},{level:2,id:"step-15-srcstateeditrs",text:"Step 15 · src/state/edit.rs"},{level:2,id:"step-16-srcapplayersrs",text:"Step 16 · src/app/layers.rs"},{level:2,id:"step-17-srcstateeditrs",text:"Step 17 · src/state/edit.rs"},{level:2,id:"step-18-srcappfeedbackrs",text:"Step 18 · src/app/feedback.rs"},{level:2,id:"step-19-indexhtml",text:"Step 19 · index.html"},{level:2,id:"step-20-cargotoml",text:"Step 20 · Cargo.toml"},{level:2,id:"step-21-srcappinputrs",text:"Step 21 · src/app/input.rs"},{level:2,id:"step-22-srclibrs",text:"Step 22 · src/lib.rs"},{level:2,id:"step-23-srcstaters",text:"Step 23 · src/state.rs"},{level:2,id:"step-24-srcenginegputargetsrs",text:"Step 24 · src/engine/gpu/targets.rs"},{level:2,id:"step-25-srcenginegpumodrs",text:"Step 25 · src/engine/gpu/mod.rs"},{level:2,id:"step-26-srcenginegpupickrs",text:"Step 26 · src/engine/gpu/pick.rs"},{level:2,id:"step-27-srcenginegpusplatrs",text:"Step 27 · src/engine/gpu/splat.rs"},{level:2,id:"step-28-srcenginegpusurface_outliners",text:"Step 28 · src/engine/gpu/surface_outline.rs"},{level:2,id:"step-29-srcenginegputriangle_tilesrs",text:"Step 29 · src/engine/gpu/triangle_tiles.rs"},{level:2,id:"step-30-srcenginegputext_planers",text:"Step 30 · src/engine/gpu/text_plane.rs"},{level:2,id:"step-31-srcenginegpubuffersrs",text:"Step 31 · src/engine/gpu/buffers.rs"},{level:2,id:"step-32-srcenginegpupresentrs",text:"Step 32 · src/engine/gpu/present.rs"},{level:2,id:"step-33-srcstaters",text:"Step 33 · src/state.rs"},{level:2,id:"step-34-srclibrs",text:"Step 34 · src/lib.rs"},{level:2,id:"step-35-srcenginegpuviewrs",text:"Step 35 · src/engine/gpu/view.rs"},{level:2,id:"step-36-srcstaters",text:"Step 36 · src/state.rs"},{level:2,id:"step-37-srcapprouters",text:"Step 37 · src/app/route.rs"},{level:2,id:"step-38-srclibrs",text:"Step 38 · src/lib.rs"},{level:2,id:"step-39-srcenginegpudevicers",text:"Step 39 · src/engine/gpu/device.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
