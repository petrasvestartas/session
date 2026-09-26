const s={title:"14 · Loading scenes",html:`<h1 id="14-loading-scenes">14 · Loading scenes<a class="anchor" href="#/course/14-loading#14-loading-scenes" aria-label="Link to this section">#</a></h1>
<p>The local fixture loads from a manifest and still supports selection and control points.</p>
<p><img src="/session/docs/course/docs/illustrations/loading.svg" alt="Two request generations in flight: the older one is dropped, the newer one is staged in manifest order and swapped in whole while the previous scene stays on screen." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcappmanifestrs">Step 1 · src/app/manifest.rs<a class="anchor" href="#/course/14-loading#step-1-srcappmanifestrs" aria-label="Link to this section">#</a></h2>
<p>The manifest describes scene files and their placements.</p>
<p><code>lessons/14/src/app/manifest.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The small index listing every file of a scene, read first so the viewer knows what to fetch.</span>
<span class="k">use</span> serde::Deserialize;
<span class="k">use</span> session_rust::Xform;

<span class="c">/// One manifest entry: a file and its placement.</span>
#[derive(Clone, Deserialize)]
<span class="k">pub</span> <span class="k">struct</span> Item {
    <span class="k">pub</span> file: String, <span class="c">// path like \`pb/box.pb\`, relative to the scene</span>
    #[serde(default)]
    <span class="k">pub</span> name: String, <span class="c">// display name, empty = the file's own</span>
    #[serde(default)]
    <span class="k">pub</span> at: Option&lt;[f64; 3]&gt;, <span class="c">// translation in world units</span>
    #[serde(default)]
    <span class="k">pub</span> xform: Option&lt;[f64; 16]&gt;, <span class="c">// full matrix, wins over \`at\`</span>
    #[serde(default)]
    <span class="k">pub</span> point_size: f64, <span class="c">// cloud point size in px, 0 = the file's own</span>
    #[serde(default)]
    <span class="k">pub</span> display_only: bool, <span class="c">// old flag, no longer changes anything</span>
}

<span class="c">/// One text placed in the world.</span>
#[derive(Clone, Debug, Deserialize)]
<span class="k">pub</span> <span class="k">struct</span> TextItem {
    <span class="k">pub</span> text: String,
    #[serde(default)]
    <span class="k">pub</span> at: [f64; <span class="s">3</span>], <span class="c">// world origin of the text</span>
    #[serde(default = &quot;<span class="s">text_right</span>&quot;)]
    <span class="k">pub</span> right: [f64; <span class="s">3</span>], <span class="c">// unit direction of the text line</span>
    #[serde(default = &quot;<span class="s">text_up</span>&quot;)]
    <span class="k">pub</span> up: [f64; <span class="s">3</span>], <span class="c">// unit direction up the text</span>
    <span class="k">pub</span> height: f64, <span class="c">// letter height in world units</span>
}

<span class="c">/// The parsed scene file.</span>
#[derive(Clone, Deserialize)]
<span class="k">pub</span> <span class="k">struct</span> Manifest {
    #[serde(default)]
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> items: Vec&lt;Item&gt;, <span class="c">// geometry files</span>
    #[serde(default)]
    <span class="k">pub</span> texts: Vec&lt;TextItem&gt;,
}</code></pre></div>
<p><code>lessons/14/src/app/manifest.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Item {
    <span class="c">/// The item's placement, None for the auto grid.</span>
    <span class="k">pub</span> <span class="k">fn</span> placement(&amp;<span class="k">self</span>) -&gt; Option&lt;Xform&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(m) = <span class="k">self</span>.xform {
            <span class="k">let</span> <span class="k">mut</span> x = Xform::identity();
            x.m = m;
            <span class="k">return</span> Some(x);
        }

        <span class="k">self</span>.at.map(translation)
    }
}

<span class="k">impl</span> Manifest {
    <span class="c">/// Parse YAML, JSON or TOML and check every value.</span>
    <span class="k">pub</span> <span class="k">fn</span> parse(bytes: &amp;[u8]) -&gt; Result&lt;<span class="k">Self</span>, String&gt; {
        <span class="k">if</span> bytes.len() &gt; <span class="s">4</span> * <span class="s">1024</span> * <span class="s">1024</span> {
            <span class="k">return</span> Err(&quot;<span class="s">manifest exceeds 4 MiB</span>&quot;.to_string());
        }

        <span class="k">let</span> text = <span class="k">match</span> std::str::from_utf8(bytes) {
            Ok(text) =&gt; text,
            Err(error) =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">manifest is not UTF-8: </span>{<span class="s">error</span>}&quot;)),
        };
        <span class="k">let</span> manifest: <span class="k">Self</span> = <span class="k">match</span> serde_yaml_ng::from_str(text) {
            Ok(manifest) =&gt; manifest,
            Err(yaml) =&gt; <span class="k">match</span> toml::from_str(text) {
                Ok(manifest) =&gt; manifest,
                Err(toml) =&gt; {
                    <span class="k">return</span> Err(format!(
                        &quot;<span class="s">Invalid manifest. YAML: </span>{}{}<span class="s">. TOML: </span>{<span class="s">toml</span>}&quot;,
                        yaml,
                        yaml_at(&amp;yaml)
                    ));
                }
            },
        };

        <span class="k">if</span> manifest.items.len() &gt; <span class="s">100_000</span> {
            <span class="k">return</span> Err(&quot;<span class="s">manifest exceeds 100,000 items</span>&quot;.to_string());
        }

        <span class="k">for</span> (index, item) <span class="k">in</span> manifest.items.iter().enumerate() {
            <span class="k">if</span> item.file.trim().is_empty() {
                <span class="k">return</span> Err(format!(&quot;<span class="s">item </span>{<span class="s">index</span>}<span class="s">: missing geometry file</span>&quot;));
            }

            <span class="k">if</span> !item.point_size.is_finite() || item.point_size &lt; <span class="s">0</span>.<span class="s">0</span> {
                <span class="k">return</span> Err(format!(&quot;<span class="s">item </span>{<span class="s">index</span>}<span class="s">: invalid point size</span>&quot;));
            }

            <span class="k">if</span> item.at.is_some_and(nonfinite_transform)
                || item.xform.is_some_and(nonfinite_transform)
            {
                <span class="k">return</span> Err(format!(&quot;<span class="s">item </span>{<span class="s">index</span>}<span class="s">: non-finite transform</span>&quot;));
            }

            <span class="k">if</span> <span class="k">let</span> Some(matrix) = item.xform
                &amp;&amp; (matrix[<span class="s">3</span>] != <span class="s">0</span>.<span class="s">0</span> || matrix[<span class="s">7</span>] != <span class="s">0</span>.<span class="s">0</span> || matrix[<span class="s">11</span>] != <span class="s">0</span>.<span class="s">0</span> || matrix[<span class="s">15</span>] != <span class="s">1</span>.<span class="s">0</span>)
            {
                <span class="k">return</span> Err(format!(&quot;<span class="s">item </span>{<span class="s">index</span>}<span class="s">: placement must be affine</span>&quot;));
            }
        }

        <span class="k">if</span> manifest.texts.len() &gt; <span class="s">1024</span> {
            <span class="k">return</span> Err(&quot;<span class="s">manifest exceeds 1,024 text records</span>&quot;.to_string());
        }

        <span class="k">let</span> <span class="k">mut</span> text_bytes = <span class="s">0usize</span>;

        <span class="k">for</span> (index, item) <span class="k">in</span> manifest.texts.iter().enumerate() {
            text_bytes = text_bytes.saturating_add(item.text.len());

            <span class="k">if</span> text_bytes &gt; <span class="s">256</span> * <span class="s">1024</span> {
                <span class="k">return</span> Err(&quot;<span class="s">manifest text exceeds 256 KiB of UTF-8 content</span>&quot;.to_string());
            }

            item.validate(index)?;
        }

        Ok(manifest)
    }</code></pre></div>
<p><code>lessons/14/src/app/manifest.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Item \`i\`'s placement, or its auto grid slot.</span>
    <span class="k">pub</span> <span class="k">fn</span> place(&amp;<span class="k">self</span>, i: usize, cell: [f64; <span class="s">2</span>]) -&gt; Xform {
        <span class="k">match</span> <span class="k">self</span>.items[i].placement() {
            Some(placement) =&gt; placement,
            None =&gt; auto_grid(i, <span class="k">self</span>.items.len(), cell),
        }
    }

    <span class="c">/// Item \`i\`'s name, or \`fallback\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> name_of(&amp;<span class="k">self</span>, i: usize, fallback: &amp;str) -&gt; String {
        <span class="k">let</span> n = &amp;<span class="k">self</span>.items[i].name;

        <span class="k">if</span> n.is_empty() {
            fallback.to_string()
        } <span class="k">else</span> {
            n.clone()
        }
    }
}</code></pre></div>
<p><code>lessons/14/src/app/manifest.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> TextItem {
    <span class="c">/// Check the text, height and plane axes.</span>
    <span class="k">fn</span> validate(&amp;<span class="k">self</span>, index: usize) -&gt; Result&lt;(), String&gt; {
        <span class="k">if</span> <span class="k">self</span>.text.trim().is_empty() {
            <span class="k">return</span> Err(format!(&quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: missing text content</span>&quot;));
        }

        <span class="k">if</span> !<span class="k">self</span>.height.is_finite() || <span class="k">self</span>.height &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> Err(format!(&quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: height must be positive and finite</span>&quot;));
        }

        <span class="k">if</span> nonfinite_transform(<span class="k">self</span>.at) {
            <span class="k">return</span> Err(format!(&quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: non-finite position</span>&quot;));
        }

        <span class="k">for</span> axis <span class="k">in</span> [<span class="k">self</span>.right, <span class="k">self</span>.up] {
            <span class="k">if</span> nonfinite_transform(axis) {
                <span class="k">return</span> Err(format!(&quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: non-finite plane axis</span>&quot;));
            }

            <span class="k">let</span> length = (axis[<span class="s">0</span>] * axis[<span class="s">0</span>] + axis[<span class="s">1</span>] * axis[<span class="s">1</span>] + axis[<span class="s">2</span>] * axis[<span class="s">2</span>]).sqrt();

            <span class="k">if</span> (length - <span class="s">1</span>.<span class="s">0</span>).abs() &gt; <span class="s">1</span>e-<span class="s">6</span> {
                <span class="k">return</span> Err(format!(
                    &quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: plane axes must be unit length within 1e-6</span>&quot;
                ));
            }
        }

        <span class="k">let</span> dot =
            <span class="k">self</span>.right[<span class="s">0</span>] * <span class="k">self</span>.up[<span class="s">0</span>] + <span class="k">self</span>.right[<span class="s">1</span>] * <span class="k">self</span>.up[<span class="s">1</span>] + <span class="k">self</span>.right[<span class="s">2</span>] * <span class="k">self</span>.up[<span class="s">2</span>];

        <span class="k">if</span> dot.abs() &gt; <span class="s">1</span>e-<span class="s">6</span> {
            <span class="k">return</span> Err(format!(
                &quot;<span class="s">text </span>{<span class="s">index</span>}<span class="s">: plane axes must be orthogonal within 1e-6</span>&quot;
            ));
        }

        Ok(())
    }
}

<span class="c">/// World +X.</span>
<span class="k">fn</span> text_right() -&gt; [f64; <span class="s">3</span>] {
    [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]
}

<span class="c">/// World +Y.</span>
<span class="k">fn</span> text_up() -&gt; [f64; <span class="s">3</span>] {
    [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]
}

<span class="c">/// A translation matrix.</span>
<span class="k">fn</span> translation(at: [f64; <span class="s">3</span>]) -&gt; Xform {
    Xform::translation(at[<span class="s">0</span>], at[<span class="s">1</span>], at[<span class="s">2</span>])
}

<span class="c">/// True when any value is not finite.</span>
<span class="k">fn</span> nonfinite_transform&lt;<span class="k">const</span> N: usize&gt;(values: [f64; N]) -&gt; bool {
    !values.into_iter().all(f64::is_finite)
}

<span class="c">/// The line and column of a YAML error, if known.</span>
<span class="k">fn</span> yaml_at(e: &amp;serde_yaml_ng::Error) -&gt; String {
    <span class="k">match</span> e.location() {
        Some(l) =&gt; format!(&quot;<span class="s"> (line </span>{}<span class="s">, column </span>{}<span class="s">)</span>&quot;, l.line(), l.column()),
        None =&gt; String::new(),
    }
}

<span class="c">/// Grid slot \`index\` of \`count\`, \`cell\` apart.</span>
<span class="k">pub</span> <span class="k">fn</span> auto_grid(index: usize, count: usize, cell: [f64; <span class="s">2</span>]) -&gt; Xform {
    <span class="k">let</span> cols = (count <span class="k">as</span> f64).sqrt().ceil().max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> usize;
    Xform::translation(
        (index % cols) <span class="k">as</span> f64 * cell[<span class="s">0</span>],
        (index / cols) <span class="k">as</span> f64 * cell[<span class="s">1</span>],
        <span class="s">0</span>.<span class="s">0</span>,
    )
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/14/src/app/manifest.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// The three formats parse the same scene.</span>
    #[test]
    <span class="k">fn</span> yaml_json_and_toml_have_identical_scene_semantics() {
        <span class="k">let</span> forms = [
            &quot;<span class="s">name: sample\\nitems:\\n  - file: pb/box.pb\\n    at: [1, 2, 3]\\n</span>&quot;,
            &quot;<span class="s">{\\&quot;name\\&quot;:\\&quot;sample\\&quot;,\\&quot;items\\&quot;:[{\\&quot;file\\&quot;:\\&quot;pb/box.pb\\&quot;,\\&quot;at\\&quot;:[1,2,3]}]}</span>&quot;,
            &quot;<span class="s">name = \\&quot;sample\\&quot;\\n[[items]]\\nfile = \\&quot;pb/box.pb\\&quot;\\nat = [1, 2, 3]\\n</span>&quot;,
        ];

        <span class="k">for</span> form <span class="k">in</span> forms {
            <span class="k">let</span> manifest = Manifest::parse(form.as_bytes()).unwrap();
            assert!(manifest.texts.is_empty());
            assert_eq!(manifest.name, &quot;<span class="s">sample</span>&quot;);
            assert_eq!(manifest.items[<span class="s">0</span>].file, &quot;<span class="s">pb/box.pb</span>&quot;);
            assert_eq!(manifest.items[<span class="s">0</span>].at, Some([<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>]));
        }
    }

    <span class="c">/// Bad items are refused.</span>
    #[test]
    <span class="k">fn</span> invalid_manifest_does_not_reach_gpu_preparation() {
        <span class="k">for</span> source <span class="k">in</span> [
            &quot;<span class="s">items: [</span>{<span class="s">file: ''</span>}<span class="s">]</span>&quot;,
            &quot;<span class="s">items: [</span>{<span class="s">file: box.pb, point_size: -.inf</span>}<span class="s">]</span>&quot;,
            &quot;<span class="s">items: [</span>{<span class="s">file: box.pb, at: [.nan, 0, 0]</span>}<span class="s">]</span>&quot;,
            &quot;<span class="s">items: [</span>&quot;,
        ] {
            assert!(Manifest::parse(source.as_bytes()).is_err(), &quot;{<span class="s">source</span>}&quot;);
        }

        assert!(Manifest::parse(&amp;[<span class="s">0xff</span>]).is_err());
    }

    <span class="c">/// The three formats parse the same text.</span>
    #[test]
    <span class="k">fn</span> world_text_yaml_json_and_toml_are_equivalent() {
        <span class="k">let</span> forms = [
            &quot;<span class="s">items: []\\ntexts:\\n  - text: 'Fixed Ω cube'\\n    at: [1, 2, 3]\\n    right: [0, 1, 0]\\n    up: [0, 0, -1]\\n    height: 12.5\\n</span>&quot;,
            <span class="s">r</span>#&quot;<span class="s">{&quot;items&quot;:[],&quot;texts&quot;:[{&quot;text&quot;:&quot;Fixed Ω cube&quot;,&quot;at&quot;:[1,2,3],&quot;right&quot;:[0,1,0],&quot;up&quot;:[0,0,-1],&quot;height&quot;:12.5}]}</span>&quot;#,
            &quot;<span class="s">items = []\\n[[texts]]\\ntext = \\&quot;Fixed Ω cube\\&quot;\\nat = [1, 2, 3]\\nright = [0, 1, 0]\\nup = [0, 0, -1]\\nheight = 12.5\\n</span>&quot;,
        ];

        <span class="k">for</span> form <span class="k">in</span> forms {
            <span class="k">let</span> manifest = Manifest::parse(form.as_bytes()).unwrap();
            assert_eq!(manifest.texts.len(), <span class="s">1</span>);
            <span class="k">let</span> text = &amp;manifest.texts[<span class="s">0</span>];
            assert_eq!(text.text, &quot;<span class="s">Fixed Ω cube</span>&quot;);
            assert_eq!(text.at, [<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>]);
            assert_eq!(text.right, [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
            assert_eq!(text.up, [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>]);
            assert_eq!(text.height, <span class="s">12</span>.<span class="s">5</span>);
        }
    }

    <span class="c">/// Text defaults apply; \`items\` and \`height\` stay required.</span>
    #[test]
    <span class="k">fn</span> world_text_defaults_preserve_manifest_compatibility() {
        <span class="k">let</span> manifest = Manifest::parse(<span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: label, height: 2</span>}<span class="s">]</span>&quot;).unwrap();
        <span class="k">let</span> text = &amp;manifest.texts[<span class="s">0</span>];
        assert_eq!(text.at, [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>]);
        assert_eq!(text.right, [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        assert_eq!(text.up, [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        assert!(Manifest::parse(<span class="s">b</span>&quot;<span class="s">texts: [</span>{<span class="s">text: label, height: 2</span>}<span class="s">]</span>&quot;).is_err());
        assert!(Manifest::parse(<span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: label</span>}<span class="s">]</span>&quot;).is_err());
    }

    <span class="c">/// Bad text records are refused.</span>
    #[test]
    <span class="k">fn</span> invalid_world_text_frames_and_heights_are_recoverable() {
        <span class="k">for</span> record <span class="k">in</span> [
            &quot;{<span class="s">text: '', height: 1</span>}&quot;,
            &quot;{<span class="s">text: '   ', height: 1</span>}&quot;,
            &quot;{<span class="s">text: label, height: 0</span>}&quot;,
            &quot;{<span class="s">text: label, height: -1</span>}&quot;,
            &quot;{<span class="s">text: label, height: .inf</span>}&quot;,
            &quot;{<span class="s">text: label, height: .nan</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, at: [0, .inf, 0]</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, right: [0, 0, 0]</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, right: [2, 0, 0]</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, right: [.nan, 0, 0]</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, up: [1, 0, 0]</span>}&quot;,
            &quot;{<span class="s">text: label, height: 1, up: [0, 1, .inf]</span>}&quot;,
        ] {
            <span class="k">let</span> source = format!(&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">record</span>}<span class="s">]</span>&quot;);
            assert!(Manifest::parse(source.as_bytes()).is_err(), &quot;{<span class="s">record</span>}&quot;);
        }
        assert!(Manifest::parse(<span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: valid, height: 1</span>}<span class="s">]</span>&quot;).is_ok());
    }

    <span class="c">/// Axes may be off unit length by 1e-6.</span>
    #[test]
    <span class="k">fn</span> world_text_axis_tolerance_is_bounded() {
        <span class="k">let</span> source = <span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: label, height: 1, right: [1.0000005, 0, 0], up: [0.0000005, 1, 0]</span>}<span class="s">]</span>&quot;;
        <span class="k">let</span> manifest = Manifest::parse(source).unwrap();
        assert_eq!(manifest.texts[<span class="s">0</span>].right[<span class="s">0</span>], <span class="s">1</span>.<span class="s">0000005</span>);

        <span class="k">for</span> source <span class="k">in</span> [
            <span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: label, height: 1, right: [1.000002, 0, 0]</span>}<span class="s">]</span>&quot;.as_slice(),
            <span class="s">b</span>&quot;<span class="s">items: []\\ntexts: [</span>{<span class="s">text: label, height: 1, up: [0.000002, 1, 0]</span>}<span class="s">]</span>&quot;.as_slice(),
        ] {
            assert!(Manifest::parse(source).is_err());
        }
    }

    <span class="c">/// Text bytes and record count have limits.</span>
    #[test]
    <span class="k">fn</span> world_text_content_and_record_limits_are_enforced() {
        <span class="k">let</span> half = &quot;<span class="s">é</span>&quot;.repeat(<span class="s">65_536</span>);
        <span class="k">let</span> <span class="k">mut</span> records = vec![serde_json::json!({&quot;<span class="s">text</span>&quot;:half,&quot;<span class="s">height</span>&quot;:<span class="s">1</span>}); <span class="s">2</span>];
        <span class="k">let</span> accepted = serde_json::json!({&quot;<span class="s">items</span>&quot;:[],&quot;<span class="s">texts</span>&quot;:records});
        assert!(Manifest::parse(accepted.to_string().as_bytes()).is_ok());
        records.push(serde_json::json!({&quot;<span class="s">text</span>&quot;:&quot;<span class="s">x</span>&quot;,&quot;<span class="s">height</span>&quot;:<span class="s">1</span>}));
        <span class="k">let</span> rejected = serde_json::json!({&quot;<span class="s">items</span>&quot;:[],&quot;<span class="s">texts</span>&quot;:records});
        assert!(
            Manifest::parse(rejected.to_string().as_bytes())
                .err()
                .unwrap()
                .contains(&quot;<span class="s">256 KiB</span>&quot;)
        );
        <span class="k">let</span> <span class="k">mut</span> records = vec![serde_json::json!({&quot;<span class="s">text</span>&quot;:&quot;<span class="s">x</span>&quot;,&quot;<span class="s">height</span>&quot;:<span class="s">1</span>}); <span class="s">1024</span>];
        <span class="k">let</span> accepted = serde_json::json!({&quot;<span class="s">items</span>&quot;:[],&quot;<span class="s">texts</span>&quot;:records});
        assert!(Manifest::parse(accepted.to_string().as_bytes()).is_ok());
        records.push(serde_json::json!({&quot;<span class="s">text</span>&quot;:&quot;<span class="s">x</span>&quot;,&quot;<span class="s">height</span>&quot;:<span class="s">1</span>}));
        <span class="k">let</span> rejected = serde_json::json!({&quot;<span class="s">items</span>&quot;:[],&quot;<span class="s">texts</span>&quot;:records});
        assert!(
            Manifest::parse(rejected.to_string().as_bytes())
                .err()
                .unwrap()
                .contains(&quot;<span class="s">1,024</span>&quot;)
        );
    }
}</code></pre></div>
<h2 id="step-2-srcappvalidaters">Step 2 · src/app/validate.rs<a class="anchor" href="#/course/14-loading#step-2-srcappvalidaters" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/14/src/app/validate.rs</code> · copy the file, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Checks a file before use, so a truncated download fails with a message instead of a wrong picture.</span>
<span class="k">use</span> prost::Message;
<span class="k">use</span> session_rust::proto;

<span class="c">/// Check every NURBS record in session JSON before it is parsed.</span>
<span class="k">pub</span> <span class="k">fn</span> json(text: &amp;str) -&gt; Result&lt;(), String&gt; {
    <span class="k">let</span> source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    <span class="k">let</span> <span class="k">mut</span> pending = vec![(&amp;source, <span class="s">0usize</span>)];

    <span class="k">while</span> <span class="k">let</span> Some((value, depth)) = pending.pop() {
        <span class="k">if</span> depth &gt; <span class="s">64</span> {
            <span class="k">return</span> Err(&quot;<span class="s">session JSON exceeds 64 nested levels</span>&quot;.into());
        }

        <span class="k">match</span> value {
            serde_json::Value::Object(fields) =&gt; {
                <span class="k">if</span> fields.contains_key(&quot;<span class="s">control_points</span>&quot;)
                    &amp;&amp; (fields.contains_key(&quot;<span class="s">cv_count</span>&quot;) || fields.contains_key(&quot;<span class="s">cv_count_u</span>&quot;))
                {
                    json_controls(value)?;
                }

                <span class="k">for</span> child <span class="k">in</span> fields.values() {
                    pending.push((child, depth + <span class="s">1</span>));
                }
            }
            serde_json::Value::Array(values) =&gt; {
                <span class="k">for</span> child <span class="k">in</span> values {
                    pending.push((child, depth + <span class="s">1</span>));
                }
            }
            _ =&gt; {}
        }
    }

    Ok(())
}

<span class="c">/// A JSON parse error as a message.</span>
<span class="k">fn</span> json_error(error: serde_json::Error) -&gt; String {
    format!(&quot;<span class="s">invalid session JSON: </span>{<span class="s">error</span>}&quot;)
}

<span class="c">/// A count field, defaulted and capped.</span>
<span class="k">fn</span> json_count(
    source: &amp;serde_json::Value,
    key: &amp;str,
    default: u64,
    maximum: u64,
) -&gt; Result&lt;usize, String&gt; {
    <span class="k">let</span> count = <span class="k">match</span> source.get(key) {
        Some(value) =&gt; <span class="k">match</span> value.as_u64() {
            Some(value) =&gt; value,
            None =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">invalid JSON </span>{<span class="s">key</span>}&quot;)),
        },
        None =&gt; default,
    };

    <span class="k">if</span> count &gt; maximum {
        <span class="k">return</span> Err(format!(&quot;<span class="s">JSON </span>{<span class="s">key</span>}<span class="s"> exceeds its supported count</span>&quot;));
    }

    Ok(count <span class="k">as</span> usize)
}

<span class="c">/// The highest control index a surface reads, None on overflow.</span>
<span class="k">fn</span> addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -&gt; Option&lt;usize&gt; {
    (rows - <span class="s">1</span>)
        .checked_mul(stride_u)?
        .checked_add((columns - <span class="s">1</span>).checked_mul(stride_v)?)?
        .checked_add(size)
}

<span class="c">/// Check one JSON curve or surface's controls against its counts.</span>
<span class="k">fn</span> json_controls(source: &amp;serde_json::Value) -&gt; Result&lt;(), String&gt; {
    <span class="k">let</span> controls = source[&quot;<span class="s">control_points</span>&quot;]
        .as_array()
        .ok_or(&quot;<span class="s">JSON controls must be an array</span>&quot;)?;
    <span class="k">let</span> dimension = json_count(source, &quot;<span class="s">dimension</span>&quot;, <span class="s">3</span>, <span class="s">3</span>)?;

    <span class="k">if</span> dimension &lt; <span class="s">2</span> {
        <span class="k">return</span> Err(&quot;<span class="s">invalid JSON control dimension</span>&quot;.into());
    }

    <span class="k">let</span> size = dimension + usize::from(source[&quot;<span class="s">is_rational</span>&quot;].as_bool().unwrap_or(<span class="s">false</span>));

    <span class="k">if</span> source.get(&quot;<span class="s">cv_count_u</span>&quot;).is_some() {
        <span class="k">let</span> rows = json_count(source, &quot;<span class="s">cv_count_u</span>&quot;, <span class="s">0</span>, <span class="s">1_000_000</span>)?;
        <span class="k">let</span> columns = json_count(source, &quot;<span class="s">cv_count_v</span>&quot;, <span class="s">0</span>, <span class="s">1_000_000</span>)?;
        <span class="k">let</span> total = rows <span class="k">as</span> u64 * columns <span class="k">as</span> u64;

        <span class="k">if</span> total &gt; <span class="s">4_000_000</span> || controls.len() <span class="k">as</span> u64 != total * size <span class="k">as</span> u64 {
            <span class="k">return</span> Err(&quot;<span class="s">JSON surface controls disagree with their declared counts</span>&quot;.into());
        }

        json_axis(source, &quot;<span class="s">order_u</span>&quot;, rows, &quot;<span class="s">nurbsknots_u</span>&quot;)?;
        json_axis(source, &quot;<span class="s">order_v</span>&quot;, columns, &quot;<span class="s">nurbsknots_v</span>&quot;)?;
    } <span class="k">else</span> {
        <span class="k">let</span> count = json_count(source, &quot;<span class="s">cv_count</span>&quot;, <span class="s">0</span>, <span class="s">1_000_000</span>)?;
        <span class="k">let</span> stride = json_count(source, &quot;<span class="s">cv_stride</span>&quot;, size <span class="k">as</span> u64, <span class="s">4</span>)?;

        <span class="k">if</span> count != controls.len() || stride != size {
            <span class="k">return</span> Err(&quot;<span class="s">JSON curve controls disagree with their declared count or stride</span>&quot;.into());
        }

        <span class="k">for</span> control <span class="k">in</span> controls {
            <span class="k">if</span> control.as_array().map(Vec::len) != Some(size) {
                <span class="k">return</span> Err(&quot;<span class="s">incomplete JSON curve control</span>&quot;.into());
            }
        }

        json_axis(source, &quot;<span class="s">order</span>&quot;, count, &quot;<span class="s">nurbsknots</span>&quot;)?;
    }

    Ok(())
}

<span class="c">/// Check one JSON knot vector.</span>
<span class="k">fn</span> json_axis(
    source: &amp;serde_json::Value,
    order_key: &amp;str,
    count: usize,
    knots_key: &amp;str,
) -&gt; Result&lt;(), String&gt; {
    <span class="k">let</span> order = json_count(source, order_key, <span class="s">4</span>, <span class="s">64</span>)?;
    <span class="k">let</span> values = source[knots_key]
        .as_array()
        .ok_or(&quot;<span class="s">missing JSON NURBS knots</span>&quot;)?;
    <span class="k">let</span> <span class="k">mut</span> knots = Vec::with_capacity(values.len());

    <span class="k">for</span> value <span class="k">in</span> values {
        knots.push(value.as_f64().ok_or(&quot;<span class="s">invalid JSON NURBS knot</span>&quot;)?);
    }

    axis(order <span class="k">as</span> i32, count <span class="k">as</span> i32, &amp;knots)?;
    Ok(())
}

<span class="c">/// Check every geometry of a loaded session.</span>
<span class="k">pub</span> <span class="k">fn</span> retained(source: &amp;session_rust::Session) -&gt; Result&lt;(), String&gt; {
    <span class="k">for</span> item <span class="k">in</span> &amp;source.objects.meshes {
        mesh(&amp;item.to_proto())?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.objects.pointclouds {
        cloud(&amp;item.to_proto())?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.objects.nurbscurves {
        curve(&amp;item.to_proto())?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.objects.nurbssurfaces {
        surface(&amp;item.to_proto())?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.objects.breps {
        brep(&amp;item.to_proto())?;
    }

    Ok(())
}</code></pre></div>
<p><code>lessons/14/src/app/validate.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Check every record of a protobuf session.</span>
<span class="k">pub</span> <span class="k">fn</span> session(source: &amp;proto::Session) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> <span class="k">let</span> Some(objects) = &amp;source.objects {
        <span class="k">let</span> count = objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        <span class="k">if</span> count &gt; <span class="s">2_000_000</span> {
            <span class="k">return</span> Err(&quot;<span class="s">scene exceeds two million source objects</span>&quot;.into());
        }

        <span class="k">for</span> point <span class="k">in</span> &amp;objects.points {
            finite(&amp;[point.x, point.y, point.z, point.width], &quot;<span class="s">point</span>&quot;)?;
        }

        <span class="k">for</span> line <span class="k">in</span> &amp;objects.lines {
            <span class="k">if</span> line.coords.len() != <span class="s">6</span> {
                <span class="k">return</span> Err(&quot;<span class="s">line needs exactly two endpoint triples</span>&quot;.into());
            }

            finite(&amp;line.coords, &quot;<span class="s">line coordinates</span>&quot;)?;
            finite(&amp;line.dash, &quot;<span class="s">line dashes</span>&quot;)?;
        }

        <span class="k">for</span> polyline <span class="k">in</span> &amp;objects.polylines {
            triples(&amp;polyline.coords, &quot;<span class="s">polyline coordinates</span>&quot;)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.meshes {
            mesh(item)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.pointclouds {
            cloud(item)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.nurbscurves {
            curve(item)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.nurbssurfaces {
            surface(item)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.breps {
            brep(item)?;
        }

        <span class="k">for</span> item <span class="k">in</span> &amp;objects.elements {
            <span class="k">match</span> item.geometry_type.as_str() {
                &quot;<span class="s">Mesh</span>&quot; =&gt; mesh(
                    &amp;proto::Mesh::decode(item.geometry_data.as_slice()).map_err(decode_error)?,
                )?,
                &quot;<span class="s">BRep</span>&quot; =&gt; brep(
                    &amp;proto::BRep::decode(item.geometry_data.as_slice()).map_err(decode_error)?,
                )?,
                _ =&gt; {}
            }
        }
    }

    <span class="k">for</span> entry <span class="k">in</span> &amp;source.xforms {
        <span class="k">let</span> Some(transform) = &amp;entry.xform <span class="k">else</span> {
            <span class="k">return</span> Err(&quot;<span class="s">missing object transform</span>&quot;.into());
        };
        finite(&amp;transform.matrix, &quot;<span class="s">object transform</span>&quot;)?;

        <span class="k">if</span> transform.matrix.len() != <span class="s">16</span>
            || transform.matrix[<span class="s">3</span>] != <span class="s">0</span>.<span class="s">0</span>
            || transform.matrix[<span class="s">7</span>] != <span class="s">0</span>.<span class="s">0</span>
            || transform.matrix[<span class="s">11</span>] != <span class="s">0</span>.<span class="s">0</span>
            || transform.matrix[<span class="s">15</span>] != <span class="s">1</span>.<span class="s">0</span>
        {
            <span class="k">return</span> Err(&quot;<span class="s">object placement must be a sixteen-value affine matrix</span>&quot;.into());
        }
    }

    <span class="k">if</span> <span class="k">let</span> Some(tree) = &amp;source.tree
        &amp;&amp; <span class="k">let</span> Some(root) = &amp;tree.root
    {
        <span class="k">let</span> <span class="k">mut</span> pending = vec![(root, <span class="s">0usize</span>)];

        <span class="k">while</span> <span class="k">let</span> Some((node, depth)) = pending.pop() {
            <span class="k">if</span> depth &gt; <span class="s">64</span> {
                <span class="k">return</span> Err(&quot;<span class="s">scene hierarchy exceeds 64 levels</span>&quot;.into());
            }

            <span class="k">for</span> child <span class="k">in</span> &amp;node.children {
                pending.push((child, depth + <span class="s">1</span>));
            }
        }
    }

    Ok(())
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/14/src/app/validate.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A protobuf decode error as a message.</span>
<span class="k">fn</span> decode_error(error: prost::DecodeError) -&gt; String {
    format!(&quot;<span class="s">invalid element geometry: </span>{<span class="s">error</span>}&quot;)
}

<span class="c">/// Every value must be finite.</span>
<span class="k">fn</span> finite(values: &amp;[f64], label: &amp;str) -&gt; Result&lt;(), String&gt; {
    <span class="k">for</span> value <span class="k">in</span> values {
        <span class="k">if</span> !value.is_finite() {
            <span class="k">return</span> Err(format!(&quot;{<span class="s">label</span>}<span class="s">: non-finite value</span>&quot;));
        }
    }

    Ok(())
}

<span class="c">/// A finite array of whole xyz triples.</span>
<span class="k">fn</span> triples(values: &amp;[f64], label: &amp;str) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> !values.len().is_multiple_of(<span class="s">3</span>) {
        <span class="k">return</span> Err(format!(&quot;{<span class="s">label</span>}<span class="s">: incomplete coordinate triple</span>&quot;));
    }

    finite(values, label)
}

<span class="c">/// Check order, count and knots; returns the knot count.</span>
<span class="k">fn</span> axis(order: i32, count: i32, knots: &amp;[f64]) -&gt; Result&lt;usize, String&gt; {
    <span class="k">if</span> !(<span class="s">2</span>..=<span class="s">64</span>).contains(&amp;order) || count &lt; order || count &gt; <span class="s">1_000_000</span> {
        <span class="k">return</span> Err(&quot;<span class="s">invalid or oversized NURBS order/control count</span>&quot;.into());
    }

    <span class="k">if</span> knots.len() != (order + count - <span class="s">2</span>) <span class="k">as</span> usize {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS knot count does not match its order and controls</span>&quot;.into());
    }

    finite(knots, &quot;<span class="s">NURBS knots</span>&quot;)?;

    <span class="k">for</span> pair <span class="k">in</span> knots.windows(<span class="s">2</span>) {
        <span class="k">if</span> pair[<span class="s">0</span>] &gt; pair[<span class="s">1</span>] {
            <span class="k">return</span> Err(&quot;<span class="s">NURBS knots are not monotonic</span>&quot;.into());
        }
    }

    <span class="k">if</span> knots[(order - <span class="s">2</span>) <span class="k">as</span> usize] &gt;= knots[(count - <span class="s">1</span>) <span class="k">as</span> usize] {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS parameter domain is empty</span>&quot;.into());
    }

    Ok(count <span class="k">as</span> usize)
}

<span class="c">/// Check one protobuf curve.</span>
<span class="k">fn</span> curve(source: &amp;proto::NurbsCurve) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> !(<span class="s">2</span>..=<span class="s">3</span>).contains(&amp;source.dimension) {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS curve dimension must be 2 or 3</span>&quot;.into());
    }

    <span class="k">let</span> count = axis(source.order, source.cv_count, &amp;source.nurbsknots)?;
    <span class="k">let</span> size = source.dimension <span class="k">as</span> usize + usize::from(source.is_rational);

    <span class="k">if</span> source.cvs.len() != count * size {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS curve control storage is incomplete</span>&quot;.into());
    }

    finite(&amp;source.cvs, &quot;<span class="s">NURBS curve controls</span>&quot;)
}

<span class="c">/// Check one protobuf surface.</span>
<span class="k">fn</span> surface(source: &amp;proto::NurbsSurface) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> source.dimension != <span class="s">3</span> {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS surface dimension must be 3</span>&quot;.into());
    }

    <span class="k">let</span> rows = axis(source.order_u, source.cv_count_u, &amp;source.nurbsknots_u)?;
    <span class="k">let</span> columns = axis(source.order_v, source.cv_count_v, &amp;source.nurbsknots_v)?;

    <span class="k">if</span> rows.saturating_mul(columns) &gt; <span class="s">4_000_000</span> {
        <span class="k">return</span> Err(&quot;<span class="s">surface exceeds four million controls</span>&quot;.into());
    }

    <span class="k">let</span> size = <span class="s">3</span> + usize::from(source.is_rational);
    <span class="k">let</span> stride_u = <span class="k">if</span> source.cv_stride_u &gt; <span class="s">0</span> {
        source.cv_stride_u <span class="k">as</span> usize
    } <span class="k">else</span> {
        columns * size
    };
    <span class="k">let</span> stride_v = <span class="k">if</span> source.cv_stride_v &gt; <span class="s">0</span> {
        source.cv_stride_v <span class="k">as</span> usize
    } <span class="k">else</span> {
        size
    };
    <span class="k">let</span> addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);

    <span class="k">if</span> !matches!(addressed, Some(length) <span class="k">if</span> length &lt;= source.cvs.len()) {
        <span class="k">return</span> Err(&quot;<span class="s">NURBS surface control storage is incomplete</span>&quot;.into());
    }

    finite(&amp;source.cvs, &quot;<span class="s">NURBS surface controls</span>&quot;)?;

    <span class="k">if</span> <span class="k">let</span> Some(cached) = &amp;source.cached_mesh {
        mesh(cached)?;
    }

    Ok(())
}

<span class="c">/// Check one protobuf mesh.</span>
<span class="k">fn</span> mesh(source: &amp;proto::Mesh) -&gt; Result&lt;(), String&gt; {
    <span class="k">for</span> (key, vertex) <span class="k">in</span> &amp;source.vertices {
        <span class="k">if</span> *key &gt; u32::MAX <span class="k">as</span> u64 {
            <span class="k">return</span> Err(&quot;<span class="s">mesh vertex key exceeds the browser index range</span>&quot;.into());
        }

        finite(&amp;[vertex.x, vertex.y, vertex.z], &quot;<span class="s">mesh vertex</span>&quot;)?;

        <span class="k">for</span> value <span class="k">in</span> vertex.attributes.values() {
            <span class="k">if</span> !value.is_finite() {
                <span class="k">return</span> Err(&quot;<span class="s">mesh vertex has a non-finite attribute</span>&quot;.into());
            }
        }
    }

    <span class="k">for</span> face <span class="k">in</span> source.faces.values() {
        <span class="k">if</span> face.vertices.len() &lt; <span class="s">3</span> {
            <span class="k">return</span> Err(&quot;<span class="s">mesh face has fewer than three vertices</span>&quot;.into());
        }

        mesh_indices(source, &amp;face.vertices)?;

        <span class="k">for</span> hole <span class="k">in</span> &amp;face.holes {
            mesh_indices(source, &amp;hole.vertices)?;
        }
    }

    <span class="k">for</span> (face, triangles) <span class="k">in</span> &amp;source.triangulation {
        <span class="k">if</span> !source.faces.contains_key(face) || !triangles.vertices.len().is_multiple_of(<span class="s">3</span>) {
            <span class="k">return</span> Err(&quot;<span class="s">invalid mesh triangulation record</span>&quot;.into());
        }

        mesh_indices(source, &amp;triangles.vertices)?;
    }

    Ok(())
}

<span class="c">/// Every index must name a vertex.</span>
<span class="k">fn</span> mesh_indices(source: &amp;proto::Mesh, indices: &amp;[u64]) -&gt; Result&lt;(), String&gt; {
    <span class="k">for</span> index <span class="k">in</span> indices {
        <span class="k">if</span> !source.vertices.contains_key(index) {
            <span class="k">return</span> Err(format!(&quot;<span class="s">mesh references missing vertex </span>{<span class="s">index</span>}&quot;));
        }
    }

    Ok(())
}

<span class="c">/// Check one protobuf point cloud.</span>
<span class="k">fn</span> cloud(source: &amp;proto::PointCloud) -&gt; Result&lt;(), String&gt; {
    triples(&amp;source.coords, &quot;<span class="s">point cloud</span>&quot;)?;
    finite(&amp;source.normals, &quot;<span class="s">point normals</span>&quot;)?;
    <span class="k">let</span> points = source.coords.len() / <span class="s">3</span>;

    <span class="k">if</span> (!source.normals.is_empty() &amp;&amp; source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() &amp;&amp; source.colors.len() != points * <span class="s">4</span>)
        || (!source.point_ids.is_empty() &amp;&amp; source.point_ids.len() != points)
    {
        <span class="k">return</span> Err(&quot;<span class="s">point-cloud attribute counts disagree</span>&quot;.into());
    }

    <span class="k">let</span> nodes = source.lod_size.len();

    <span class="k">if</span> nodes &gt; <span class="s">0</span> {
        <span class="k">if</span> source.lod_min.len() != nodes * <span class="s">3</span>
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * <span class="s">8</span>
        {
            <span class="k">return</span> Err(&quot;<span class="s">point-cloud octree arrays disagree</span>&quot;.into());
        }

        finite(&amp;source.lod_min, &quot;<span class="s">octree bounds</span>&quot;)?;
        finite(&amp;source.lod_size, &quot;<span class="s">octree sizes</span>&quot;)?;

        <span class="k">for</span> node <span class="k">in</span> <span class="s">0</span>..nodes {
            <span class="k">if</span> source.lod_first[node] &lt; <span class="s">0</span>
                || source.lod_count[node] &lt; <span class="s">0</span>
                || source.lod_first[node] <span class="k">as</span> u64 + source.lod_count[node] <span class="k">as</span> u64 &gt; points <span class="k">as</span> u64
            {
                <span class="k">return</span> Err(&quot;<span class="s">octree range exceeds its source points</span>&quot;.into());
            }
        }

        <span class="k">for</span> child <span class="k">in</span> &amp;source.lod_children {
            <span class="k">if</span> *child &lt; -<span class="s">1</span> || *child &gt;= nodes <span class="k">as</span> i32 {
                <span class="k">return</span> Err(&quot;<span class="s">octree references an absent child</span>&quot;.into());
            }
        }
    }

    Ok(())
}

<span class="c">/// Check one protobuf BRep.</span>
<span class="k">fn</span> brep(source: &amp;proto::BRep) -&gt; Result&lt;(), String&gt; {
    <span class="k">for</span> item <span class="k">in</span> &amp;source.curves_2d {
        curve(item)?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.curves_3d {
        curve(item)?;
    }

    <span class="k">for</span> item <span class="k">in</span> &amp;source.surfaces {
        surface(item)?;
    }

    <span class="k">for</span> vertex <span class="k">in</span> &amp;source.vertices {
        <span class="k">if</span> <span class="k">let</span> Some(point) = &amp;vertex.point {
            finite(&amp;[point.x, point.y, point.z], &quot;<span class="s">BRep vertex</span>&quot;)?;
        }
    }

    Ok(())
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Huge or wrong counts are refused.</span>
    #[test]
    <span class="k">fn</span> hostile_counts_do_not_reach_kernel_allocations() {
        <span class="k">let</span> source = proto::NurbsCurve {
            dimension: <span class="s">3</span>,
            order: <span class="s">2</span>,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&amp;source).is_err());
        <span class="k">let</span> source = proto::NurbsSurface {
            dimension: <span class="s">3</span>,
            order_u: <span class="s">2</span>,
            order_v: <span class="s">2</span>,
            cv_count_u: -<span class="s">1</span>,
            cv_count_v: <span class="s">2</span>,
            ..Default::default()
        };
        assert!(surface(&amp;source).is_err());
    }

    <span class="c">/// Bad indices and short cloud arrays are refused.</span>
    #[test]
    <span class="k">fn</span> missing_indices_and_partial_cloud_attributes_are_errors() {
        <span class="k">let</span> <span class="k">mut</span> source = proto::Mesh::default();
        source.faces.insert(
            <span class="s">0</span>,
            proto::FaceData {
                vertices: vec![<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>],
                ..Default::default()
            },
        );
        assert!(mesh(&amp;source).is_err());
        <span class="k">let</span> cloud_source = proto::PointCloud {
            coords: vec![<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>.],
            point_ids: vec![<span class="s">7</span>, <span class="s">8</span>],
            ..Default::default()
        };
        assert!(cloud(&amp;cloud_source).is_err());
    }

    <span class="c">/// JSON counts must match the controls given.</span>
    #[test]
    <span class="k">fn</span> json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(<span class="s">r</span>#&quot;<span class="s">{&quot;control_points&quot;:[],&quot;cv_count&quot;:18446744073709551615,&quot;dimension&quot;:3}</span>&quot;#).is_err()
        );
        assert!(
            json(<span class="s">r</span>#&quot;<span class="s">{&quot;control_points&quot;:[],&quot;cv_count_u&quot;:1000000,&quot;cv_count_v&quot;:1000000}</span>&quot;#).is_err()
        );
        assert!(json(<span class="s">r</span>#&quot;<span class="s">{&quot;control_points&quot;:[[0,0,0],[1,0,0]],&quot;cv_count&quot;:2,&quot;dimension&quot;:3,&quot;order&quot;:2,&quot;nurbsknots&quot;:[0,1]}</span>&quot;#).is_ok());
    }
}</code></pre></div>
<h2 id="step-3-srcappdecoders">Step 3 · src/app/decode.rs<a class="anchor" href="#/course/14-loading#step-3-srcappdecoders" aria-label="Link to this section">#</a></h2>
<p>Decode converts serialized bytes into retained source documents.</p>
<p><code>lessons/14/src/app/decode.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Turns the raw bytes of a scene file into kernel objects.</span>
<span class="k">use</span> super::fetch::next_tick;
<span class="k">use</span> prost::Message;
<span class="k">use</span> session_rust::proto;
<span class="k">use</span> session_rust::tree::{Tree, TreeNode};
<span class="k">use</span> session_rust::{
    BRep, Element, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, PointCloud,
    Polyline, Session, Xform,
};
<span class="k">use</span> std::rc::Rc;

<span class="c">/// Objects converted before yielding to the browser.</span>
<span class="k">const</span> CHUNK: usize = <span class="s">25_000</span>;

<span class="c">/// Counts conversions.</span>
<span class="k">struct</span> Pacer {
    n: usize, <span class="c">// objects converted so far</span>
}

<span class="k">impl</span> Pacer {
    <span class="c">/// Count one; true every \`CHUNK\`.</span>
    <span class="k">fn</span> tick(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.n += <span class="s">1</span>;
        <span class="k">self</span>.n.is_multiple_of(CHUNK)
    }
}

<span class="c">/// Convert a list of proto objects, yielding now and then.</span>
macro_rules! convert {
    ($s:expr, $pacer:expr, $vec:expr, $ty:ident, $variant:ident, $slot:ident) =&gt; {
        <span class="k">for</span> x <span class="k">in</span> $vec {
            <span class="k">let</span> g = Rc::new($ty::from_proto(x));
            $s.lookup
                .insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&amp;g)));
            $s.objects.$slot.push(g);

            <span class="k">if</span> $pacer.tick() {
                next_tick().<span class="k">await</span>;
            }
        }
    };
    (fallible $s:expr, $pacer:expr, $vec:expr, $ty:ident, $variant:ident, $slot:ident) =&gt; {
        <span class="k">for</span> x <span class="k">in</span> $vec {
            <span class="k">let</span> v = <span class="k">match</span> $ty::from_proto(x) {
                Ok(value) =&gt; value,
                Err(error) =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">invalid </span>{}<span class="s">: </span>{<span class="s">error</span>}&quot;, stringify!($ty))),
            };
            <span class="k">let</span> g = Rc::new(v);
            $s.lookup
                .insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&amp;g)));
            $s.objects.$slot.push(g);

            <span class="k">if</span> $pacer.tick() {
                next_tick().<span class="k">await</span>;
            }
        }
    };
}</code></pre></div>
<p><code>lessons/14/src/app/decode.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A session from file bytes, yielding while converting.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> session_from_bytes(url: &amp;str, bytes: Vec&lt;u8&gt;) -&gt; Result&lt;Session, String&gt; {
    <span class="k">if</span> bytes.len() &gt; <span class="s">512</span> * <span class="s">1024</span> * <span class="s">1024</span> {
        <span class="k">return</span> Err(
            &quot;<span class="s">geometry payload exceeds the 512 MiB whole-file limit; use cloud streaming</span>&quot;
                .to_string(),
        );
    }

    <span class="k">if</span> url.ends_with(&quot;<span class="s">.json</span>&quot;) {
        <span class="k">let</span> text = <span class="k">match</span> std::str::from_utf8(&amp;bytes) {
            Ok(text) =&gt; text,
            Err(error) =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">invalid JSON encoding: </span>{<span class="s">error</span>}&quot;)),
        };
        super::validate::json(text)?;
        <span class="k">let</span> session = <span class="k">match</span> Session::jsonload(text) {
            Ok(session) =&gt; session,
            Err(error) =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">invalid session JSON: </span>{<span class="s">error</span>}&quot;)),
        };
        super::validate::retained(&amp;session)?;
        <span class="k">return</span> Ok(session);
    }

    <span class="k">let</span> p = <span class="k">match</span> proto::Session::decode(bytes.as_slice()) {
        Ok(session) =&gt; session,
        Err(error) =&gt; <span class="k">return</span> Err(format!(&quot;<span class="s">invalid session protobuf: </span>{<span class="s">error</span>}&quot;)),
    };
    drop(bytes); <span class="c">// free the raw file early</span>
    super::validate::session(&amp;p)?;
    <span class="k">let</span> <span class="k">mut</span> s = Session::new(&amp;p.name);
    s.set_guid(p.guid.clone());
    <span class="k">let</span> <span class="k">mut</span> pacer = Pacer { n: <span class="s">0</span> };

    <span class="k">if</span> <span class="k">let</span> Some(o) = p.objects {
        s.objects.set_guid(o.guid);
        s.objects.name = o.name;
        convert!(s, pacer, o.points, Point, Point, points);
        convert!(s, pacer, o.lines, Line, Line, lines);
        convert!(s, pacer, o.planes, Plane, Plane, planes);
        convert!(fallible s, pacer, o.bboxes, OBB, OBB, bboxes);
        convert!(s, pacer, o.polylines, Polyline, Polyline, polylines);
        convert!(s, pacer, o.pointclouds, PointCloud, PointCloud, pointclouds);
        convert!(s, pacer, o.meshes, Mesh, Mesh, meshes);
        convert!(s, pacer, o.nurbscurves, NurbsCurve, NurbsCurve, nurbscurves);
        convert!(fallible s, pacer, o.nurbssurfaces, NurbsSurface, NurbsSurface, nurbssurfaces);
        convert!(fallible s, pacer, o.breps, BRep, BRep, breps);
        convert!(fallible s, pacer, o.elements, Element, Element, elements);
    }

    <span class="k">for</span> entry <span class="k">in</span> &amp;p.xforms {
        <span class="k">let</span> Some(xf) = &amp;entry.xform <span class="k">else</span> { <span class="k">continue</span> };
        <span class="k">let</span> <span class="k">mut</span> xform = Xform::identity();
        xform.set_guid(xf.guid.clone());
        xform.name = xf.name.clone();

        <span class="k">for</span> (i, val) <span class="k">in</span> xf.matrix.iter().enumerate().take(<span class="s">16</span>) {
            xform.m[i] = *val;
        }

        s.xforms.insert(entry.guid.clone(), xform);
    }

    <span class="k">if</span> <span class="k">let</span> Some(gp) = &amp;p.graph {
        s.graph = session_rust::Graph::new(&amp;gp.name);
        s.graph.set_guid(gp.guid.clone());

        <span class="k">for</span> (name, v) <span class="k">in</span> &amp;gp.vertices {
            s.graph.add_node(name, &amp;v.attribute);
        }

        <span class="k">for</span> e <span class="k">in</span> &amp;gp.edges {
            s.graph.add_edge(&amp;e.v0, &amp;e.v1, &amp;e.attribute);
        }
    }

    <span class="k">if</span> <span class="k">let</span> Some(tp) = &amp;p.tree {
        s.tree = Tree::new(&amp;tp.name);
        s.tree.set_guid(tp.guid.clone());

        <span class="k">if</span> <span class="k">let</span> Some(rp) = &amp;tp.root {
            <span class="k">let</span> root = build_tree(rp);
            s.tree.add(&amp;root, None);
        }
    }

    s.reindex(); <span class="c">// filled directly, not via add_*: rebuild the guid maps that delete and undo search</span>

    Ok(s)
}

<span class="c">/// A tree node and its children.</span>
<span class="k">fn</span> build_tree(proto: &amp;proto::TreeNode) -&gt; Rc&lt;std::cell::RefCell&lt;TreeNode&gt;&gt; {
    <span class="k">let</span> node = TreeNode::new(&amp;proto.name);

    <span class="k">for</span> c <span class="k">in</span> &amp;proto.children {
        <span class="k">let</span> child = build_tree(c);
        node.borrow_mut().add(&amp;child);
    }

    node
}</code></pre></div>
<h2 id="step-4-srcapprouters">Step 4 · src/app/route.rs<a class="anchor" href="#/course/14-loading#step-4-srcapprouters" aria-label="Link to this section">#</a></h2>
<p>Route helpers read viewer options from the page URL.</p>
<p><code>lessons/14/src/app/route.rs</code> · edit · type this</p>
<p>Added at the top of <code>lessons/13/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The public bucket scenes come from.</span>
<span class="k">pub</span> <span class="k">const</span> DATA_BASE: &amp;str = &quot;<span class="s">https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/</span>&quot;;

<span class="c">/// The scene a local \`trunk serve\` shows.</span>
<span class="k">pub</span> <span class="k">const</span> LOCAL_SCENE: &amp;str = &quot;<span class="s">view_local.yaml</span>&quot;;

<span class="c">/// Spacing for items without a placement; zero stacks them.</span>
<span class="k">pub</span> <span class="k">const</span> AUTO_GRID: [f64; <span class="s">2</span>] = [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>];

<span class="c">/// True for a localhost URL.</span>
<span class="k">pub</span> <span class="k">fn</span> is_local_url(url: &amp;str) -&gt; bool {
    url.starts_with(&quot;<span class="s">http://localhost:</span>&quot;)
        || url.starts_with(&quot;<span class="s">http://127.0.0.1:</span>&quot;)
        || url.starts_with(&quot;<span class="s">http://[::1]:</span>&quot;)
}

<span class="c">/// Where a scene and its files are.</span>
<span class="k">pub</span> <span class="k">struct</span> SceneRoute {
    <span class="k">pub</span> manifest: String, <span class="c">// the scene file URL</span>
    <span class="k">pub</span> base: String, <span class="c">// prefix for its \`file\` entries</span>
}

<span class="c">/// The \`?name=\` value of the page URL.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/13/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// An integer knob from the query string.</span>
<span class="k">pub</span> <span class="k">fn</span> knob_u32(name: &amp;str) -&gt; Option&lt;u32&gt; {
    query(name)?.parse().ok()
}

<span class="c">/// True when the page is served from localhost.</span>
<span class="k">pub</span> <span class="k">fn</span> page_is_local() -&gt; bool {
    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    <span class="k">let</span> Ok(hostname) = window.location().hostname() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    matches!(
        hostname.as_str(),
        &quot;<span class="s">localhost</span>&quot; | &quot;<span class="s">127.0.0.1</span>&quot; | &quot;<span class="s">[::1]</span>&quot; | &quot;<span class="s">::1</span>&quot;
    )
}

<span class="c">/// The scene named by the last path segment, e.g. \`/view_lines\`.</span>
<span class="k">pub</span> <span class="k">fn</span> path_scene() -&gt; Option&lt;String&gt; {
    <span class="k">let</span> path = web_sys::window()?.location().pathname().ok()?;
    <span class="k">let</span> last = path.rsplit('<span class="s">/</span>').next()?.to_string();
    <span class="k">let</span> safe = !last.is_empty()
        &amp;&amp; !last.ends_with(&quot;<span class="s">.html</span>&quot;)
        &amp;&amp; !last.contains('<span class="s">:</span>')
        &amp;&amp; !last.starts_with('<span class="s">.</span>');
    safe.then_some(last)
}

<span class="c">/// The \`?scene=\` value, refused when it escapes the tree.</span>
<span class="k">pub</span> <span class="k">fn</span> query_scene() -&gt; Option&lt;String&gt; {
    <span class="k">let</span> decoded = query(&quot;<span class="s">scene</span>&quot;)?;

    <span class="k">for</span> segment <span class="k">in</span> decoded.split('<span class="s">/</span>') {
        <span class="k">if</span> segment == &quot;<span class="s">..</span>&quot; {
            <span class="k">return</span> None;
        }
    }

    <span class="k">let</span> safe = !decoded.is_empty()
        &amp;&amp; !decoded.starts_with('<span class="s">/</span>')
        &amp;&amp; !decoded.contains(&quot;<span class="s">//</span>&quot;)
        &amp;&amp; !decoded.contains('<span class="s">:</span>');
    safe.then_some(decoded)
}

<span class="c">/// The data URL prefix: \`?data=\`, \`off\` for this origin, else the bucket.</span>
<span class="k">pub</span> <span class="k">fn</span> data_base() -&gt; String {
    <span class="k">let</span> base = <span class="k">match</span> query(&quot;<span class="s">data</span>&quot;) {
        None =&gt; DATA_BASE.to_string(),
        Some(v) <span class="k">if</span> v == &quot;<span class="s">off</span>&quot; || v.is_empty() =&gt; <span class="k">return</span> String::new(),
        Some(v) <span class="k">if</span> v.starts_with(&quot;<span class="s">https://</span>&quot;) || is_local_url(&amp;v) =&gt; v,
        Some(other) =&gt; {
            log::warn!(&quot;<span class="s">data: ignoring \`?data=</span>{<span class="s">other</span>}<span class="s">\`; using </span>{<span class="s">DATA_BASE</span>}&quot;);
            DATA_BASE.to_string()
        }
    };

    <span class="k">if</span> base.ends_with('<span class="s">/</span>') {
        base
    } <span class="k">else</span> {
        base + &quot;<span class="s">/</span>&quot;
    }
}

<span class="c">/// \`base\` + \`file\`, unless \`file\` is already a full URL.</span>
<span class="k">pub</span> <span class="k">fn</span> join(base: &amp;str, file: &amp;str) -&gt; String {
    <span class="k">if</span> file.starts_with(&quot;<span class="s">https://</span>&quot;) || file.starts_with(&quot;<span class="s">http://</span>&quot;) {
        <span class="k">return</span> file.to_string();
    }

    format!(&quot;{}{}&quot;, base, file.trim_start_matches(&quot;<span class="s">./</span>&quot;))
}

<span class="c">/// The route of a scene name, \`.yaml\` and \`scenes/\` implied.</span>
<span class="k">pub</span> <span class="k">fn</span> named_scene(path: &amp;str) -&gt; SceneRoute {
    <span class="k">let</span> path = <span class="k">if</span> path.contains('<span class="s">.</span>') {
        path.to_string()
    } <span class="k">else</span> {
        format!(&quot;{<span class="s">path</span>}<span class="s">.yaml</span>&quot;)
    };
    <span class="k">let</span> path = <span class="k">if</span> path.contains('<span class="s">/</span>') {
        path
    } <span class="k">else</span> {
        format!(&quot;<span class="s">scenes/</span>{<span class="s">path</span>}&quot;)
    };
    <span class="k">let</span> base = data_base();
    SceneRoute {
        manifest: join(&amp;base, &amp;path),
        base,
    }
}

<span class="c">/// The scene this page asks for; None means the live source.</span>
<span class="k">pub</span> <span class="k">fn</span> scene_route() -&gt; Option&lt;SceneRoute&gt; {
    <span class="k">if</span> <span class="k">let</span> Some(path) = query_scene().or_else(path_scene) {
        <span class="k">return</span> Some(named_scene(&amp;path));
    }

    <span class="k">if</span> page_is_local() {
        Some(SceneRoute {
            manifest: LOCAL_SCENE.to_string(),
            base: String::new(),
        })
    } <span class="k">else</span> {
        None
    }
}</code></pre></div>
<h2 id="step-5-srcapplivers">Step 5 · src/app/live.rs<a class="anchor" href="#/course/14-loading#step-5-srcapplivers" aria-label="Link to this section">#</a></h2>
<p>Live notifications request a conditional reload instead of replacing geometry directly.</p>
<p><code>lessons/14/src/app/live.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Reads the scene from a URL, so the published page shows current data without rebuilding the viewer.</span>
<span class="k">use</span> super::decode::session_from_bytes;
<span class="k">use</span> super::fetch::{GetOpts, get};
<span class="k">use</span> super::manifest::Manifest;
<span class="k">use</span> super::route::{
    AUTO_GRID, data_base, is_local_url, join, page_is_local, path_scene, query, query_scene,
};
<span class="k">use</span> super::scene::FileDoc;
<span class="k">use</span> session_rust::Session;
<span class="k">use</span> std::cell::RefCell;
<span class="k">use</span> std::collections::HashMap;
<span class="k">use</span> std::hash::{Hash, Hasher};
<span class="k">use</span> std::rc::Rc;
<span class="k">use</span> wasm_bindgen::JsCast;
<span class="k">use</span> wasm_bindgen::closure::Closure;

<span class="c">/// The scene watched by default.</span>
<span class="k">pub</span> <span class="k">const</span> DEFAULT_SOURCE: &amp;str =
    &quot;<span class="s">https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml</span>&quot;;

<span class="c">/// The event stream a publisher announces uploads on.</span>
<span class="k">const</span> DEFAULT_NOTIFY: &amp;str = &quot;<span class="s">https://ntfy.sh/wood-live-84eaac4a04729911/sse</span>&quot;;

<span class="k">const</span> DEFAULT_POLL_SECONDS: f64 = <span class="s">5</span>.<span class="s">0</span>; <span class="c">// network check interval</span>

<span class="c">/// How often the relay flag is looked at, ms.</span>
<span class="k">const</span> NOTIFY_TICK_MS: i32 = <span class="s">100</span>;

<span class="c">/// An open relay connection.</span>
<span class="k">struct</span> Notify {
    _source: web_sys::EventSource,
    flag: Rc&lt;RefCell&lt;bool&gt;&gt;, <span class="c">// a message arrived</span>
    _on_message: Closure&lt;<span class="k">dyn</span> FnMut(web_sys::MessageEvent)&gt;, <span class="c">// the JS callback</span>
}

<span class="k">impl</span> Notify {
    <span class="c">/// Open the stream; None when the browser refuses.</span>
    <span class="k">fn</span> open(url: &amp;str) -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> source = <span class="k">match</span> web_sys::EventSource::new(url) {
            Ok(s) =&gt; s,
            Err(e) =&gt; {
                log::warn!(&quot;<span class="s">live: relay </span>{<span class="s">url</span>}<span class="s"> could not be opened (</span>{<span class="s">e:?</span>}<span class="s">); polling only</span>&quot;);
                <span class="k">return</span> None;
            }
        };
        <span class="k">let</span> flag = Rc::new(RefCell::new(<span class="s">false</span>));
        <span class="k">let</span> sink = flag.clone();
        <span class="k">let</span> on_message =
            Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::MessageEvent)&gt;::new(<span class="k">move</span> |e: web_sys::MessageEvent| {
                on_relay_message(&amp;sink, &amp;e)
            });
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        log::info!(&quot;<span class="s">live: notified by </span>{<span class="s">url</span>}&quot;);
        Some(Notify {
            _source: source,
            flag,
            _on_message: on_message,
        })
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Take the flag: true once per announcement.</span>
    <span class="k">fn</span> take(&amp;<span class="k">self</span>) -&gt; bool {
        std::mem::replace(&amp;<span class="k">mut</span> <span class="k">self</span>.flag.borrow_mut(), <span class="s">false</span>)
    }
}

<span class="k">impl</span> Drop <span class="k">for</span> Notify {
    <span class="c">/// Close the stream.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>._source.set_onmessage(None);
        <span class="k">self</span>._source.close();
    }
}

<span class="c">/// What one read found.</span>
<span class="k">enum</span> Read {
    Changed(Vec&lt;u8&gt;), <span class="c">// new bytes</span>
    Same, <span class="c">// unchanged since last time</span>
    Failed(String), <span class="c">// the error</span>
}</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The watched scene and what was last seen of it.</span>
<span class="k">pub</span> <span class="k">struct</span> LiveSource {
    <span class="k">pub</span> url: String,
    <span class="k">pub</span> tick_ms: i32, <span class="c">// how often \`check\` runs</span>
    <span class="k">pub</span> poll_ms: f64, <span class="c">// how often the network is read</span>
    last_read_ms: f64,
    base: String, <span class="c">// prefix for the manifest's files</span>
    manifest: Option&lt;Manifest&gt;, <span class="c">// last good manifest</span>
    etags: HashMap&lt;String, String&gt;, <span class="c">// last ETag per URL</span>
    hashes: HashMap&lt;String, u64&gt;, <span class="c">// last content hash per URL without ETag</span>
    sessions: HashMap&lt;String, Rc&lt;Session&gt;&gt;, <span class="c">// decoded file per URL</span>
    last_warning: Option&lt;String&gt;, <span class="c">// last message logged</span>
    pending: bool, <span class="c">// a change waits to be shown</span>
    notify: Option&lt;Notify&gt;, <span class="c">// relay connection</span>
}

<span class="k">impl</span> LiveSource {
    <span class="c">/// The manifest's texts.</span>
    <span class="k">pub</span> <span class="k">fn</span> texts(&amp;<span class="k">self</span>) -&gt; Vec&lt;super::manifest::TextItem&gt; {
        <span class="k">match</span> &amp;<span class="k">self</span>.manifest {
            Some(manifest) =&gt; manifest.texts.clone(),
            None =&gt; Vec::new(),
        }
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The live source from the page URL, None when off or a named scene.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_query() -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> live = query(&quot;<span class="s">live</span>&quot;);

        <span class="k">if</span> live.as_deref() == Some(&quot;<span class="s">off</span>&quot;) || live.as_deref() == Some(&quot;<span class="s">0</span>&quot;) {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> named = query_scene().or_else(path_scene);

        <span class="c">// a named scene or a dev server turns live off, except \`view_live\`</span>
        <span class="k">if</span> live.is_none()
            &amp;&amp; named.as_deref() != Some(&quot;<span class="s">view_live</span>&quot;)
            &amp;&amp; (named.is_some() || page_is_local())
        {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> url = <span class="k">match</span> live {
            Some(u) <span class="k">if</span> u.starts_with(&quot;<span class="s">https://</span>&quot;) || is_local_url(&amp;u) =&gt; u,
            Some(other) =&gt; {
                log::warn!(&quot;<span class="s">live: ignoring \`?live=</span>{<span class="s">other</span>}<span class="s">\`; watching the default</span>&quot;);
                DEFAULT_SOURCE.to_string()
            }
            None =&gt; DEFAULT_SOURCE.to_string(),
        };
        <span class="k">let</span> seconds = <span class="k">match</span> query(&quot;<span class="s">poll</span>&quot;) {
            Some(value) =&gt; <span class="k">match</span> value.parse::&lt;f64&gt;() {
                Ok(seconds) <span class="k">if</span> seconds &gt;= <span class="s">1</span>.<span class="s">0</span> =&gt; seconds,
                _ =&gt; DEFAULT_POLL_SECONDS,
            },
            None =&gt; DEFAULT_POLL_SECONDS,
        };
        <span class="k">let</span> notify = <span class="k">match</span> (is_local_url(&amp;url), query(&quot;<span class="s">notify</span>&quot;).as_deref()) {
            (_, Some(&quot;<span class="s">off</span>&quot;)) | (_, Some(&quot;<span class="s">0</span>&quot;)) | (<span class="s">true</span>, _) =&gt; None,
            (<span class="s">false</span>, Some(u)) <span class="k">if</span> u.starts_with(&quot;<span class="s">https://</span>&quot;) =&gt; Notify::open(u),
            (<span class="s">false</span>, _) =&gt; Notify::open(DEFAULT_NOTIFY),
        };
        <span class="k">let</span> poll_ms = seconds * <span class="s">1000</span>.<span class="s">0</span>;
        <span class="c">// with a relay, look at its flag often</span>
        <span class="k">let</span> tick_ms = <span class="k">if</span> notify.is_some() {
            NOTIFY_TICK_MS.min(poll_ms <span class="k">as</span> i32)
        } <span class="k">else</span> {
            poll_ms <span class="k">as</span> i32
        };
        Some(<span class="k">Self</span> {
            url,
            tick_ms,
            poll_ms,
            last_read_ms: f64::NEG_INFINITY,
            base: String::new(),
            manifest: None,
            etags: HashMap::new(),
            hashes: HashMap::new(),
            sessions: HashMap::new(),
            last_warning: None,
            pending: <span class="s">false</span>,
            notify,
        })
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Log a message once until it changes.</span>
    <span class="k">fn</span> warn(&amp;<span class="k">mut</span> <span class="k">self</span>, message: String) {
        <span class="k">if</span> <span class="k">self</span>.last_warning.as_deref() != Some(message.as_str()) {
            log::warn!(&quot;<span class="s">live: </span>{<span class="s">message</span>}&quot;);
            <span class="k">self</span>.last_warning = Some(message);
        }
    }

    <span class="c">/// Log and forget \`url\` so the next poll reads it again.</span>
    <span class="k">fn</span> forget(&amp;<span class="k">mut</span> <span class="k">self</span>, url: &amp;str, message: String) {
        <span class="k">self</span>.etags.remove(url);
        <span class="k">self</span>.hashes.remove(url);
        <span class="k">self</span>.sessions.remove(url);
        <span class="k">self</span>.warn(message);
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Read \`url\`, reporting Same when it did not change.</span>
    <span class="k">async</span> <span class="k">fn</span> read(&amp;<span class="k">mut</span> <span class="k">self</span>, url: &amp;str) -&gt; Read {
        <span class="k">let</span> known = <span class="k">self</span>.etags.get(url).cloned();
        <span class="k">let</span> opts = GetOpts {
            no_store: <span class="s">false</span>,
            revalidate: <span class="s">true</span>,
            if_none_match: known.clone(),
            range: None,
        };

        <span class="k">match</span> get(url, &amp;opts).<span class="k">await</span> {
            Err(e) =&gt; Read::Failed(e),
            Ok(r) <span class="k">if</span> r.status == <span class="s">304</span> =&gt; Read::Same,
            Ok(r) <span class="k">if</span> !(<span class="s">200</span>..<span class="s">300</span>).contains(&amp;r.status) =&gt; Read::Failed(format!(&quot;<span class="s">HTTP </span>{}&quot;, r.status)),
            Ok(r) =&gt; {
                <span class="k">if</span> <span class="k">let</span> Some(tag) = r.etag {
                    <span class="k">let</span> same = known.as_deref() == Some(tag.as_str());
                    <span class="k">self</span>.etags.insert(url.to_string(), tag);
                    <span class="k">return</span> <span class="k">if</span> same {
                        Read::Same
                    } <span class="k">else</span> {
                        Read::Changed(r.bytes)
                    };
                }

                <span class="c">// no ETag: compare a hash of the bytes</span>
                <span class="k">let</span> <span class="k">mut</span> hasher = std::collections::hash_map::DefaultHasher::new();
                r.bytes.hash(&amp;<span class="k">mut</span> hasher);
                <span class="k">let</span> hash = hasher.finish();
                <span class="k">let</span> same = <span class="k">self</span>.hashes.insert(url.to_string(), hash) == Some(hash);

                <span class="k">if</span> same {
                    Read::Same
                } <span class="k">else</span> {
                    Read::Changed(r.bytes)
                }
            }
        }
    }

    <span class="c">/// Parse and keep a manifest; false when invalid.</span>
    <span class="k">fn</span> adopt(&amp;<span class="k">mut</span> <span class="k">self</span>, bytes: &amp;[u8]) -&gt; bool {
        <span class="k">match</span> Manifest::parse(bytes) {
            Ok(m) =&gt; {
                <span class="c">// bucket manifests name files from the bucket root</span>
                <span class="k">let</span> bucket = data_base();
                <span class="k">self</span>.base = <span class="k">if</span> !bucket.is_empty() &amp;&amp; <span class="k">self</span>.url.starts_with(&amp;bucket) {
                    bucket
                } <span class="k">else</span> {
                    dir_of(&amp;<span class="k">self</span>.url)
                };
                log::info!(&quot;<span class="s">live: manifest '</span>{}<span class="s">' has </span>{}<span class="s"> items</span>&quot;, m.name, m.items.len());
                <span class="k">self</span>.manifest = Some(m);
                <span class="k">self</span>.last_warning = None;
                <span class="s">true</span>
            }
            Err(e) =&gt; {
                <span class="k">self</span>.warn(format!(
                    &quot;<span class="s">the manifest at </span>{}<span class="s"> is not valid TOML/YAML/JSON: </span>{<span class="s">e</span>}&quot;,
                    <span class="k">self</span>.url
                ));
                <span class="s">false</span>
            }
        }
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One tick; Some(docs) when the scene changed.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> check(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;Vec&lt;FileDoc&gt;&gt; {
        <span class="k">let</span> announced = <span class="k">self</span>.notify.as_ref().is_some_and(Notify::take);
        <span class="k">let</span> now = <span class="k">crate</span>::engine::performance::now_ms();

        <span class="k">if</span> !announced &amp;&amp; now - <span class="k">self</span>.last_read_ms &lt; <span class="k">self</span>.poll_ms {
            <span class="k">return</span> None; <span class="c">// not yet time</span>
        }

        <span class="k">self</span>.last_read_ms = now;
        <span class="k">let</span> url = <span class="k">self</span>.url.clone();
        <span class="k">let</span> changed = <span class="k">match</span> <span class="k">self</span>.read(&amp;url).<span class="k">await</span> {
            Read::Failed(e) =&gt; {
                <span class="k">self</span>.warn(format!(&quot;<span class="s">manifest </span>{<span class="s">url</span>}<span class="s"> unreachable (</span>{<span class="s">e</span>}<span class="s">)</span>&quot;));
                <span class="k">return</span> None;
            }
            Read::Changed(bytes) =&gt; {
                <span class="k">if</span> !<span class="k">self</span>.adopt(&amp;bytes) {
                    <span class="k">self</span>.etags.remove(&amp;url);
                    <span class="k">self</span>.hashes.remove(&amp;url);
                    <span class="k">return</span> None;
                }

                <span class="s">true</span>
            }
            Read::Same =&gt; <span class="s">false</span>,
        };
        <span class="k">self</span>.pending |= changed;
        <span class="k">let</span> files = <span class="k">self</span>.file_urls();
        <span class="k">let</span> <span class="k">mut</span> failed = <span class="s">false</span>;

        <span class="k">for</span> (_, file) <span class="k">in</span> &amp;files {
            <span class="k">if</span> file.contains(&quot;<span class="s">/pb/revisions/</span>&quot;) &amp;&amp; <span class="k">self</span>.sessions.contains_key(file) {
                <span class="k">continue</span>; <span class="c">// a revision never changes</span>
            }

            <span class="k">match</span> <span class="k">self</span>.read(file).<span class="k">await</span> {
                Read::Changed(bytes) =&gt; {
                    <span class="k">self</span>.pending = <span class="s">true</span>;
                    <span class="k">self</span>.decode(file, bytes).<span class="k">await</span>;
                }
                Read::Same =&gt; {}
                Read::Failed(e) =&gt; {
                    failed = <span class="s">true</span>;
                    <span class="k">self</span>.warn(format!(
                        &quot;{<span class="s">file</span>}<span class="s"> could not be read (</span>{<span class="s">e</span>}<span class="s">); retrying next poll</span>&quot;
                    ));
                }
            }
        }

        <span class="k">if</span> failed || !<span class="k">self</span>.pending {
            <span class="k">return</span> None;
        }

        log::info!(
            &quot;<span class="s">live: source changed</span>{}<span class="s">; reloading the scene</span>&quot;,
            <span class="k">if</span> announced { &quot;<span class="s"> (announced)</span>&quot; } <span class="k">else</span> { &quot;&quot; }
        );
        <span class="k">let</span> docs = <span class="k">self</span>.load_all(&amp;files).<span class="k">await</span>;
        <span class="c">// drop files the manifest no longer lists</span>
        <span class="k">let</span> <span class="k">mut</span> removed = Vec::new();

        <span class="k">for</span> url <span class="k">in</span> <span class="k">self</span>.sessions.keys() {
            <span class="k">let</span> <span class="k">mut</span> listed = <span class="s">false</span>;

            <span class="k">for</span> (_, file) <span class="k">in</span> &amp;files {
                <span class="k">if</span> file == url {
                    listed = <span class="s">true</span>;
                    <span class="k">break</span>;
                }
            }

            <span class="k">if</span> !listed {
                removed.push(url.clone());
            }
        }

        <span class="k">for</span> url <span class="k">in</span> removed {
            <span class="k">self</span>.sessions.remove(&amp;url);
        }

        <span class="k">if</span> docs.len() != files.len() {
            <span class="k">self</span>.warn(&quot;<span class="s">replacement is incomplete; retaining the last valid scene</span>&quot;.to_string());
            <span class="k">return</span> None;
        }

        <span class="k">self</span>.pending = <span class="s">false</span>;
        Some(docs)
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Decode one file and keep it; an empty file is dropped.</span>
    <span class="k">async</span> <span class="k">fn</span> decode(&amp;<span class="k">mut</span> <span class="k">self</span>, url: &amp;str, bytes: Vec&lt;u8&gt;) {
        <span class="k">let</span> n = bytes.len();
        <span class="k">let</span> session = <span class="k">match</span> session_from_bytes(url, bytes).<span class="k">await</span> {
            Ok(session) =&gt; session,
            Err(error) =&gt; {
                <span class="k">self</span>.forget(url, format!(&quot;<span class="s">cannot decode </span>{<span class="s">url</span>}<span class="s">: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };

        <span class="k">if</span> session.lookup.is_empty() {
            <span class="k">self</span>.forget(url, format!(&quot;{<span class="s">url</span>}<span class="s"> holds no geometry (</span>{<span class="s">n</span>}<span class="s"> bytes); skipped</span>&quot;));
            <span class="k">return</span>;
        }

        log::info!(
            &quot;<span class="s">live: decoded </span>{<span class="s">url</span>}<span class="s">: </span>{}<span class="s"> objects, </span>{<span class="s">n</span>}<span class="s"> bytes</span>&quot;,
            session.lookup.len()
        );
        <span class="k">self</span>.sessions.insert(url.to_string(), Rc::new(session));
    }</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// (item index, URL) of every file the manifest lists.</span>
    <span class="k">fn</span> file_urls(&amp;<span class="k">self</span>) -&gt; Vec&lt;(usize, String)&gt; {
        <span class="k">let</span> Some(m) = &amp;<span class="k">self</span>.manifest <span class="k">else</span> {
            <span class="k">return</span> Vec::new();
        };
        <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(m.items.len());

        <span class="k">for</span> (i, item) <span class="k">in</span> m.items.iter().enumerate() {
            <span class="k">if</span> !item.file.trim().is_empty() {
                out.push((i, join(&amp;<span class="k">self</span>.base, &amp;item.file)));
            }
        }

        out
    }

    <span class="c">/// One document per listed file, fetching any not yet decoded.</span>
    <span class="k">async</span> <span class="k">fn</span> load_all(&amp;<span class="k">mut</span> <span class="k">self</span>, files: &amp;[(usize, String)]) -&gt; Vec&lt;FileDoc&gt; {
        <span class="k">let</span> Some(m) = <span class="k">self</span>.manifest.take() <span class="k">else</span> {
            <span class="k">return</span> Vec::new();
        };
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">for</span> (i, url) <span class="k">in</span> files {
            <span class="k">let</span> (i, url) = (*i, url.as_str());

            <span class="k">if</span> !<span class="k">self</span>.sessions.contains_key(url) {
                <span class="k">match</span> get(
                    url,
                    &amp;GetOpts {
                        revalidate: <span class="s">true</span>,
                        ..GetOpts::default()
                    },
                )
                .<span class="k">await</span>
                {
                    Ok(r) <span class="k">if</span> (<span class="s">200</span>..<span class="s">300</span>).contains(&amp;r.status) =&gt; <span class="k">self</span>.decode(url, r.bytes).<span class="k">await</span>,
                    Ok(r) =&gt; <span class="k">self</span>.forget(url, format!(&quot;{<span class="s">url</span>}<span class="s"> answered HTTP </span>{}<span class="s">; skipped</span>&quot;, r.status)),
                    Err(e) =&gt; {
                        <span class="k">self</span>.forget(url, format!(&quot;{<span class="s">url</span>}<span class="s"> could not be fetched (</span>{<span class="s">e</span>}<span class="s">); skipped</span>&quot;))
                    }
                }
            }

            <span class="k">let</span> Some(session) = <span class="k">self</span>.sessions.get(url).cloned() <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> name = m.name_of(i, &amp;session.name);
            out.push(FileDoc {
                name,
                session,
                place: m.place(i, AUTO_GRID),
                point_px: m.items[i].point_size <span class="k">as</span> f32,
                display_only: m.items[i].display_only,
            });
        }

        <span class="k">self</span>.manifest = Some(m);
        out
    }
}</code></pre></div>
<p><code>lessons/14/src/app/live.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// \`url\` up to and including its last \`/\`.</span>
<span class="k">fn</span> dir_of(url: &amp;str) -&gt; String {
    <span class="k">match</span> url.rfind('<span class="s">/</span>') {
        Some(i) =&gt; url[..=i].to_string(),
        None =&gt; url.to_string(),
    }
}

<span class="c">/// Raise the flag on a publish message.</span>
<span class="k">fn</span> on_relay_message(flag: &amp;Rc&lt;RefCell&lt;bool&gt;&gt;, e: &amp;web_sys::MessageEvent) {
    <span class="k">let</span> Some(text) = e.data().as_string() <span class="k">else</span> {
        <span class="k">return</span>;
    };

    <span class="k">if</span> is_change_notification(&amp;text) {
        *flag.borrow_mut() = <span class="s">true</span>;
    }
}

<span class="c">/// True for a publish message, false for relay housekeeping.</span>
<span class="k">fn</span> is_change_notification(text: &amp;str) -&gt; bool {
    #[derive(serde::Deserialize)]
    <span class="k">struct</span> Envelope {
        event: Option&lt;String&gt;,
    }

    <span class="k">match</span> serde_json::from_str::&lt;Envelope&gt;(text) {
        Ok(env) =&gt; matches!(env.event.as_deref(), None | Some(&quot;<span class="s">message</span>&quot;)),
        Err(_) =&gt; !text.trim().is_empty(),
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/14/</code>.</p>
<h2 id="step-6-srcapploaderrs">Step 6 · src/app/loader.rs<a class="anchor" href="#/course/14-loading#step-6-srcapploaderrs" aria-label="Link to this section">#</a></h2>
<p>The loader stages manifest and geometry work before publishing it.</p>
<p><code>lessons/14/src/app/loader.rs</code> · edit · type this</p>
<p>Replaces the 116 lines from <code>use super::scene::{FileDoc, Scene, StreamedIn…</code> of <code>lessons/13/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Loads a scene file and turns it into GPU rows a chunk at a time, so the page never freezes.</span>
<span class="k">use</span> super::decode::session_from_bytes;
<span class="k">use</span> super::fetch::{fetch_bytes, sleep_ms};
<span class="k">use</span> super::live::LiveSource;
<span class="k">use</span> super::manifest::Manifest;
<span class="k">use</span> super::route::AUTO_GRID;
<span class="k">use</span> super::route::{SceneRoute, join, knob_u32, named_scene, scene_route};
<span class="k">use</span> super::scene::{FileDoc, Scene, StreamedInit};
<span class="k">use</span> super::stream::{CloudFields, cloud_fields, cloud_lod, fetch_colors, fetch_positions};
<span class="k">use</span> super::walk::cloud::StreamRows;
<span class="k">use</span> <span class="k">crate</span>::engine::performance::now_ms;
<span class="k">use</span> <span class="k">crate</span>::{CloudChunk, Msg, State};
<span class="k">use</span> session_rust::Xform;
<span class="k">use</span> std::cell::{Cell, RefCell};
<span class="k">use</span> std::rc::Rc;
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> wasm_bindgen::prelude::*;
<span class="k">use</span> winit::event_loop::EventLoopProxy;
<span class="k">use</span> winit::window::Window;

<span class="c">/// Points a streamed cloud reads before its first frame.</span>
<span class="k">const</span> STREAM_PREFIX_POINTS: u32 = <span class="s">2_000_000</span>;

<span class="c">/// Points per follow-up slice.</span>
<span class="k">const</span> STREAM_CHUNK_POINTS: u32 = <span class="s">2_000_000</span>;

<span class="c">/// Most streamed points on the page, \`?points=\` overrides.</span>
<span class="k">const</span> STREAM_MAX_POINTS: u32 = <span class="s">6_000_000</span>;

<span class="c">/// Files this large are always streamed.</span>
<span class="k">const</span> STREAM_MIN_BYTES: u64 = <span class="s">64</span> * <span class="s">1024</span> * <span class="s">1024</span>;

<span class="c">/// Fewest points a streamed cloud gets, even over budget.</span>
<span class="k">const</span> STREAM_MIN_PREFIX: u32 = <span class="s">250_000</span>;

thread_local! {
    <span class="c">/// Sends messages into the event loop.</span>
    <span class="k">static</span> PROXY: RefCell&lt;Option&lt;EventLoopProxy&lt;Msg&gt;&gt;&gt; = <span class="k">const</span> { RefCell::new(None) };

    <span class="c">/// Streamed points loaded so far.</span>
    <span class="k">static</span> RESIDENT: Cell&lt;u32&gt; = <span class="k">const</span> { Cell::new(<span class="s">0</span>) };

    <span class="c">/// Bumped when the scene is cleared; old stream tasks stop.</span>
    <span class="k">static</span> GENERATION: Cell&lt;u32&gt; = <span class="k">const</span> { Cell::new(<span class="s">0</span>) };

    <span class="c">/// Bumped on every reload request; older loads give up.</span>
    <span class="k">static</span> LOAD_GENERATION: Cell&lt;u64&gt; = <span class="k">const</span> { Cell::new(<span class="s">0</span>) };
}

<span class="c">/// Clear the scene and stop every stream.</span>
<span class="k">fn</span> clear_scene() {
    GENERATION.set(GENERATION.get().wrapping_add(<span class="s">1</span>));
    RESIDENT.set(<span class="s">0</span>);
    post(Msg::Clear);
}

<span class="c">/// Send one message to the event loop; false when it is gone.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> post(msg: Msg) -&gt; bool {
    PROXY.with_borrow(|proxy| {
        proxy
            .as_ref()
            .is_some_and(|proxy| proxy.send_event(msg).is_ok())
    })
}

<span class="c">/// The point ceiling, from \`?points=\` or the default.</span>
<span class="k">fn</span> max_points() -&gt; u32 {
    knob_u32(&quot;<span class="s">points</span>&quot;).unwrap_or(STREAM_MAX_POINTS)
}

<span class="c">/// Points still allowed.</span>
<span class="k">fn</span> budget_left() -&gt; u32 {
    max_points().saturating_sub(RESIDENT.get())
}

<span class="c">/// Count \`n\` points as loaded.</span>
<span class="k">fn</span> budget_spend(n: u32) {
    RESIDENT.set(RESIDENT.get().saturating_add(n));
}

<span class="c">/// Start the viewer, load the first scene, then keep polling.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> boot(window: Arc&lt;Window&gt;, proxy: EventLoopProxy&lt;Msg&gt;) {
    PROXY.with_borrow_mut(|slot| *slot = Some(proxy.clone()));
    <span class="k">let</span> state = <span class="k">match</span> State::new(window, Scene::new()).<span class="k">await</span> {
        Ok(state) =&gt; state,
        Err(error) =&gt; {
            super::feedback::error(&amp;format!(
                &quot;<span class="s">Unable to start WebGPU: </span>{<span class="s">error</span>}<span class="s">. Use a browser with an available WebGPU adapter, then reload.</span>&quot;
            ));
            <span class="k">return</span>;
        }
    };
    <span class="k">let</span> _ = proxy.send_event(Msg::Ready(Box::new(state)));

    <span class="k">let</span> <span class="k">mut</span> live = LiveSource::from_query();
    <span class="k">let</span> <span class="k">mut</span> loaded = <span class="s">false</span>;

    <span class="k">if</span> <span class="k">let</span> Some(src) = live.as_mut() {
        log::info!(&quot;<span class="s">live: watching </span>{}<span class="s"> every </span>{<span class="s">:.0</span>}<span class="s"> ms</span>&quot;, src.url, src.poll_ms);
        loaded = post_live(src).<span class="k">await</span>;
    }

    <span class="k">if</span> !loaded &amp;&amp; <span class="k">let</span> Some(route) = scene_route() {
        load_route(&amp;route, None).<span class="k">await</span>;
    }

    <span class="k">let</span> Some(<span class="k">mut</span> src) = live <span class="k">else</span> { <span class="k">return</span> };

    <span class="k">loop</span> {
        sleep_ms(src.tick_ms).<span class="k">await</span>;
        post_live(&amp;<span class="k">mut</span> src).<span class="k">await</span>;
    }
}

<span class="c">/// One live poll; true when the scene was replaced.</span>
<span class="k">async</span> <span class="k">fn</span> post_live(src: &amp;<span class="k">mut</span> LiveSource) -&gt; bool {
    <span class="k">let</span> generation = LOAD_GENERATION.get();
    <span class="k">let</span> Some(docs) = src.check().<span class="k">await</span> <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };

    <span class="k">if</span> stale_load(generation) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> texts = src.texts();

    <span class="k">if</span> docs.is_empty() &amp;&amp; texts.is_empty() {
        <span class="k">return</span> <span class="s">false</span>;
    }

    clear_scene();

    <span class="k">for</span> doc <span class="k">in</span> docs {
        post(Msg::File(doc));
    }

    post(Msg::Texts(texts));
    post(Msg::Fit);
    super::feedback::status(&quot;&quot;);
    <span class="s">true</span>
}

<span class="c">/// Load a named scene, or reload the page's own, keeping the camera.</span>
#[wasm_bindgen]
<span class="k">pub</span> <span class="k">fn</span> reload_scene(url: Option&lt;String&gt;) {
    <span class="k">let</span> route = <span class="k">match</span> url {
        Some(path) =&gt; Some(named_scene(&amp;path)),
        None =&gt; scene_route(),
    };
    <span class="k">let</span> Some(route) = route <span class="k">else</span> {
        log::warn!(&quot;<span class="s">reload_scene: this page has no scene route - nothing to reload</span>&quot;);
        <span class="k">return</span>;
    };
    <span class="k">let</span> generation = LOAD_GENERATION.get().wrapping_add(<span class="s">1</span>);
    LOAD_GENERATION.set(generation);
    wasm_bindgen_futures::spawn_local(load_replacement(route, generation));
}

<span class="c">/// Load a route as a replacement.</span>
<span class="k">async</span> <span class="k">fn</span> load_replacement(route: SceneRoute, generation: u64) {
    load_route(&amp;route, Some(generation)).<span class="k">await</span>;
}

<span class="c">/// True when a newer load has started since.</span>
<span class="k">fn</span> stale_load(generation: u64) -&gt; bool {
    LOAD_GENERATION.get() != generation
}

<span class="c">/// Fetch the manifest; a missing \`.toml\` falls back to \`.yaml\`.</span>
<span class="k">async</span> <span class="k">fn</span> fetch_manifest(route: &amp;SceneRoute) -&gt; Result&lt;Vec&lt;u8&gt;, String&gt; {
    <span class="k">match</span> fetch_bytes(&amp;route.manifest).<span class="k">await</span> {
        Err(error) <span class="k">if</span> error.starts_with(&quot;<span class="s">HTTP 404</span>&quot;) &amp;&amp; route.manifest.ends_with(&quot;<span class="s">.toml</span>&quot;) =&gt; {
            <span class="k">let</span> yaml = format!(&quot;{}<span class="s">.yaml</span>&quot;, route.manifest.trim_end_matches(&quot;<span class="s">.toml</span>&quot;));
            super::feedback::status(&quot;<span class="s">Opening the current YAML scene for this TOML bookmark</span>&quot;);
            fetch_bytes(&amp;yaml).<span class="k">await</span>
        }
        result =&gt; result,
    }
}

<span class="c">/// Load every item of a manifest; a reload swaps the scene only once complete.</span>
<span class="k">async</span> <span class="k">fn</span> load_route(route: &amp;SceneRoute, replacement: Option&lt;u64&gt;) {
    <span class="k">let</span> generation = <span class="k">match</span> replacement {
        Some(generation) =&gt; generation,
        None =&gt; LOAD_GENERATION.get(),
    };
    <span class="k">let</span> <span class="k">mut</span> pending = Vec::new(); <span class="c">// staged items of a reload</span>
    <span class="k">let</span> <span class="k">mut</span> failed = <span class="s">false</span>;
    <span class="k">let</span> budget = scene_budget_bytes(); <span class="c">// whole-file bytes allowed</span>
    <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0u64</span>;
    <span class="k">let</span> <span class="k">mut</span> skipped: Vec&lt;String&gt; = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> staged_points = <span class="s">0u32</span>;
    <span class="k">let</span> t0 = now_ms();
    <span class="k">let</span> bytes = <span class="k">match</span> fetch_manifest(route).<span class="k">await</span> {
        Ok(b) =&gt; b,
        Err(e) =&gt; {
            super::feedback::status(&amp;format!(&quot;<span class="s">Cannot fetch the scene manifest: </span>{<span class="s">e</span>}&quot;));
            <span class="k">return</span>;
        }
    };

    <span class="k">if</span> stale_load(generation) {
        <span class="k">return</span>;
    }

    <span class="k">let</span> manifest = <span class="k">match</span> Manifest::parse(&amp;bytes) {
        Ok(m) =&gt; m,
        Err(e) =&gt; {
            super::feedback::status(&amp;format!(&quot;<span class="s">Cannot read the scene manifest: </span>{<span class="s">e</span>}&quot;));
            <span class="k">return</span>;
        }
    };
    log::info!(&quot;<span class="s">scene '</span>{}<span class="s">': </span>{}<span class="s"> items</span>&quot;, manifest.name, manifest.items.len());

    <span class="k">let</span> <span class="k">mut</span> files = <span class="s">0u32</span>;

    <span class="k">for</span> item <span class="k">in</span> &amp;manifest.items {
        <span class="k">if</span> item.file.ends_with(&quot;<span class="s">.pb</span>&quot;) {
            files += <span class="s">1</span>;
        }
    }

    <span class="k">let</span> files = files.max(<span class="s">1</span>);
    <span class="k">let</span> share = (max_points() / files).max(STREAM_MIN_PREFIX);

    <span class="k">for</span> (i, item) <span class="k">in</span> manifest.items.iter().enumerate() {
        <span class="k">let</span> url = join(&amp;route.base, &amp;item.file);
        <span class="k">let</span> place = manifest.place(i, AUTO_GRID);
        <span class="k">let</span> point_px = item.point_size <span class="k">as</span> f32;

        <span class="k">if</span> url.ends_with(&quot;<span class="s">.pb</span>&quot;) {
            <span class="k">let</span> slot = Placement {
                name: manifest.name_of(i, &amp;item.file),
                place: place.clone(),
                point_px,
            };
            <span class="k">let</span> remaining = <span class="k">if</span> replacement.is_some() {
                max_points().saturating_sub(staged_points)
            } <span class="k">else</span> {
                budget_left()
            };

            <span class="k">if</span> <span class="k">let</span> Some(init) =
                stream_prefix(&amp;url, &amp;slot, share.min(remaining.max(STREAM_MIN_PREFIX))).<span class="k">await</span>
            {
                <span class="k">if</span> stale_load(generation) {
                    <span class="k">return</span>;
                }

                <span class="k">if</span> init.resident == <span class="s">0</span> {
                    failed = <span class="s">true</span>;
                    <span class="k">continue</span>;
                }

                <span class="k">if</span> replacement.is_some() {
                    staged_points = staged_points.saturating_add(init.resident);
                    pending.push(PendingDocument::Streamed(Box::new(init)));
                } <span class="k">else</span> {
                    budget_spend(init.resident);
                    post(Msg::StreamedCloud(Box::new(init)));
                }

                <span class="k">continue</span>;
            }
        }

        <span class="c">// skip a file the device cannot hold</span>
        <span class="k">let</span> length = super::fetch::content_length(&amp;url).<span class="k">await</span>.unwrap_or(<span class="s">0</span>);

        <span class="k">if</span> spent + length &gt; budget {
            log::warn!(
                &quot;<span class="s">skipped '</span>{}<span class="s">': </span>{}<span class="s"> MB over the </span>{}<span class="s"> MB scene budget</span>&quot;,
                item.file,
                length &gt;&gt; <span class="s">20</span>,
                budget &gt;&gt; <span class="s">20</span>
            );
            skipped.push(format!(&quot;{}<span class="s"> (</span>{}<span class="s"> MB)</span>&quot;, item.file, length &gt;&gt; <span class="s">20</span>));
            <span class="k">continue</span>;
        }

        spent += length;
        <span class="k">let</span> f0 = now_ms();
        <span class="k">let</span> bytes = <span class="k">match</span> fetch_bytes(&amp;url).<span class="k">await</span> {
            Ok(b) =&gt; b,
            Err(e) =&gt; {
                super::feedback::status(&amp;format!(&quot;<span class="s">Unable to load </span>{}<span class="s">: </span>{<span class="s">e</span>}&quot;, item.file));
                failed = <span class="s">true</span>;
                <span class="k">continue</span>;
            }
        };
        <span class="k">let</span> n = bytes.len();
        <span class="k">let</span> f1 = now_ms();
        <span class="k">let</span> session = <span class="k">match</span> session_from_bytes(&amp;url, bytes).<span class="k">await</span> {
            Ok(session) =&gt; session,
            Err(error) =&gt; {
                super::feedback::status(&amp;format!(&quot;<span class="s">Cannot decode </span>{}<span class="s">: </span>{<span class="s">error</span>}&quot;, item.file));
                failed = <span class="s">true</span>;
                <span class="k">continue</span>;
            }
        };

        <span class="k">if</span> stale_load(generation) {
            <span class="k">return</span>;
        }

        <span class="k">if</span> session.lookup.is_empty() {
            log::warn!(&quot;<span class="s">'</span>{}<span class="s">' holds no geometry (</span>{<span class="s">n</span>}<span class="s"> bytes); skipped</span>&quot;, item.file);
            failed = <span class="s">true</span>;
            <span class="k">continue</span>;
        }

        <span class="k">let</span> name = manifest.name_of(i, &amp;session.name);
        log::info!(
            &quot;<span class="s">loaded '</span>{<span class="s">name</span>}<span class="s">': </span>{}<span class="s"> objects, </span>{<span class="s">n</span>}<span class="s"> bytes | fetch </span>{<span class="s">:.0</span>}<span class="s"> ms, parse </span>{<span class="s">:.0</span>}<span class="s"> ms</span>&quot;,
            session.lookup.len(),
            f1 - f0,
            now_ms() - f1
        );
        <span class="k">let</span> doc = FileDoc {
            name,
            session: Rc::new(session),
            place,
            point_px,
            display_only: item.display_only,
        };

        <span class="k">if</span> replacement.is_some() {
            pending.push(PendingDocument::Whole(doc));
        } <span class="k">else</span> {
            post(Msg::File(doc));
        }
    }

    <span class="k">if</span> stale_load(generation) {
        <span class="k">return</span>;
    }

    <span class="k">if</span> replacement.is_some() {
        <span class="k">if</span> failed {
            super::feedback::status(
                &quot;<span class="s">Scene replacement failed; the last valid scene is still visible</span>&quot;,
            );
            <span class="k">return</span>;
        }

        clear_scene();
        budget_spend(staged_points);

        <span class="k">for</span> document <span class="k">in</span> pending {
            <span class="k">match</span> document {
                PendingDocument::Whole(doc) =&gt; {
                    post(Msg::File(doc));
                }
                PendingDocument::Streamed(stream) =&gt; {
                    post(Msg::StreamedCloud(stream));
                }
            }
        }
    }

    post(Msg::Texts(manifest.texts));
    post(Msg::Fit);

    <span class="k">if</span> !failed {
        super::feedback::status(&amp;skipped_notice(&amp;skipped, budget));
    }

    log::info!(
        &quot;<span class="s">scene posted </span>{<span class="s">:.0</span>}<span class="s"> ms after the manifest fetch</span>&quot;,
        now_ms() - t0
    );
}

<span class="c">/// One staged item of a reload.</span>
<span class="k">enum</span> PendingDocument {
    Whole(FileDoc),
    Streamed(Box&lt;StreamedInit&gt;), <span class="c">// a cloud's first slice</span>
}

<span class="c">/// Name and placement of a streamed document.</span>
<span class="k">struct</span> Placement {
    name: String,
    place: Xform,
    point_px: f32, <span class="c">// point size, clouds only</span>
}

<span class="c">/// Read a cloud's first \`share\` points by range; None when it should load whole.</span>
<span class="k">async</span> <span class="k">fn</span> stream_prefix(url: &amp;str, slot: &amp;Placement, share: u32) -&gt; Option&lt;StreamedInit&gt; {
    <span class="k">let</span> (name, place, point_px) = (slot.name.as_str(), slot.place.clone(), slot.point_px);
    <span class="k">let</span> <span class="k">mut</span> fields = cloud_fields(url).<span class="k">await</span>?;

    <span class="k">if</span> fields.count &lt;= STREAM_PREFIX_POINTS &amp;&amp; fields.coords_len &lt; STREAM_MIN_BYTES {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> lod = cloud_lod(url, &amp;<span class="k">mut</span> fields).<span class="k">await</span>?;
    <span class="k">let</span> resident = STREAM_PREFIX_POINTS.min(share).min(fields.count);
    <span class="k">let</span> Some(positions) = fetch_positions(url, &amp;fields, <span class="s">0</span>, resident).<span class="k">await</span> <span class="k">else</span> {
        log::warn!(
            &quot;<span class="s">'</span>{<span class="s">name</span>}<span class="s">': the prefix range read failed - the cloud stays off screen (a whole decode would take </span>{<span class="s">:.0</span>}<span class="s"> MB)</span>&quot;,
            fields.coords_len <span class="k">as</span> f64 / <span class="s">1</span>.<span class="s">048576</span>e<span class="s">6</span>
        );
        <span class="k">return</span> Some(StreamedInit {
            name: name.to_string(),
            url: url.to_string(),
            place,
            rows: StreamRows {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            lod,
            col_at: fields.colors_at,
            fields,
            resident: <span class="s">0</span>,
            point_px,
        });
    };
    <span class="k">let</span> (colors, col_at) = fetch_colors(url, &amp;fields, fields.colors_at, resident)
        .<span class="k">await</span>
        .unwrap_or((Vec::new(), fields.colors_at));
    log::info!(
        &quot;<span class="s">streamed '</span>{<span class="s">name</span>}<span class="s">': </span>{<span class="s">resident</span>}<span class="s"> of </span>{}<span class="s"> points on screen, </span>{}<span class="s"> nodes</span>&quot;,
        fields.count,
        lod.len()
    );
    Some(StreamedInit {
        name: name.to_string(),
        url: url.to_string(),
        place,
        rows: StreamRows { positions, colors },
        lod,
        fields,
        resident,
        point_px,
        col_at,
    })
}

<span class="c">/// Where a cloud's streaming continues.</span></code></pre></div>
<p>Replaces <code>fn spawn_stream_rest</code> in <code>lessons/13/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Keep reading a cloud's slices in the background.</span>
<span class="k">pub</span> <span class="k">fn</span> spawn_stream_rest(cursor: StreamCursor) {
    wasm_bindgen_futures::spawn_local(stream_rest(cursor));
}

<span class="c">/// The slice loop behind \`spawn_stream_rest\`.</span>
<span class="k">async</span> <span class="k">fn</span> stream_rest(c: StreamCursor) {
    <span class="k">let</span> (url, idx, fields) = (c.url, c.idx, c.fields);
    <span class="k">let</span> generation = GENERATION.get();
    <span class="k">let</span> <span class="k">mut</span> col_at = c.col_at;
    <span class="k">let</span> <span class="k">mut</span> at = c.from;

    <span class="k">while</span> at &lt; fields.count {
        <span class="k">if</span> GENERATION.get() != generation {
            <span class="k">return</span>;
        }

        <span class="k">let</span> left = budget_left();

        <span class="k">if</span> left == <span class="s">0</span> {
            log::info!(
                &quot;<span class="s">'</span>{<span class="s">url</span>}<span class="s">': </span>{<span class="s">at</span>}<span class="s"> of </span>{}<span class="s"> points resident - at the page's point ceiling (?points= to raise it)</span>&quot;,
                fields.count
            );
            <span class="k">return</span>;
        }

        <span class="k">let</span> to = (at + STREAM_CHUNK_POINTS.min(left)).min(fields.count);
        budget_spend(to - at);
        <span class="k">let</span> Some(positions) = fetch_positions(&amp;url, &amp;fields, at, to).<span class="k">await</span> <span class="k">else</span> {
            <span class="k">if</span> GENERATION.get() == generation {
                RESIDENT.set(RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status(&quot;<span class="s">A point-cloud range failed; reload to retry the missing data</span>&quot;);
            <span class="k">return</span>;
        };
        <span class="k">let</span> (colors, next) = fetch_colors(&amp;url, &amp;fields, col_at, to - at)
            .<span class="k">await</span>
            .unwrap_or((Vec::new(), col_at));
        col_at = next;

        <span class="k">if</span> GENERATION.get() != generation {
            <span class="k">return</span>;
        }

        <span class="k">if</span> !post(Msg::CloudChunk(CloudChunk {
            idx,
            rows: StreamRows { positions, colors },
            to,
        })) {
            <span class="k">return</span>;
        }
        at = to;
    }
}

<span class="c">/// File bytes a scene may load.</span>
<span class="k">fn</span> scene_budget_bytes() -&gt; u64 {
    <span class="k">if</span> <span class="k">let</span> Some(mb) = <span class="k">crate</span>::engine::gpu::view::knob(&quot;<span class="s">VIEWER_BUDGET</span>&quot;, &quot;<span class="s">budget</span>&quot;)
        &amp;&amp; <span class="k">let</span> Ok(mb) = mb.parse::&lt;u64&gt;()
    {
        <span class="k">return</span> mb &lt;&lt; <span class="s">20</span>;
    }

    <span class="k">let</span> gigabytes = web_sys::window()
        .map(|window| window.navigator())
        .and_then(|navigator| js_sys::Reflect::get(&amp;navigator, &amp;&quot;<span class="s">deviceMemory</span>&quot;.into()).ok())
        .and_then(|value| value.as_f64())
        .unwrap_or(<span class="s">4</span>.<span class="s">0</span>);
    ((gigabytes * <span class="s">16</span>.<span class="s">0</span>) <span class="k">as</span> u64) &lt;&lt; <span class="s">20</span>
}

<span class="c">/// The message naming skipped files, if any.</span>
<span class="k">fn</span> skipped_notice(skipped: &amp;[String], budget: u64) -&gt; String {
    <span class="k">if</span> skipped.is_empty() {
        <span class="k">return</span> String::new();
    }

    format!(
        &quot;<span class="s">Skipped over the </span>{}<span class="s"> MB scene budget: </span>{}<span class="s">. Add ?budget=&lt;MB&gt; to raise it.</span>&quot;,
        budget &gt;&gt; <span class="s">20</span>,
        skipped.join(&quot;<span class="s">, </span>&quot;)
    )
}</code></pre></div>
<h2 id="step-7-srcappmodrs">Step 7 · src/app/mod.rs<a class="anchor" href="#/course/14-loading#step-7-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The application module connects source loading and interaction helpers.</p>
<p><code>lessons/14/src/app/mod.rs</code> · edit · type this</p>
<p>Replaces <code>mod scene</code> in <code>lessons/13/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> manifest;
<span class="k">pub</span> <span class="k">mod</span> scene;
<span class="k">pub</span> <span class="k">mod</span> selection;
<span class="k">pub</span> <span class="k">mod</span> stream;
<span class="k">pub</span> <span class="k">mod</span> touch;
<span class="k">pub</span> <span class="k">mod</span> validate;
<span class="k">pub</span> <span class="k">mod</span> walk;

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> decode;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> fetch;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> live;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]</code></pre></div>
<h2 id="step-8-srcappsceners">Step 8 · src/app/scene.rs<a class="anchor" href="#/course/14-loading#step-8-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene owns source documents and maps their identities to GPU rows.</p>
<p><code>lessons/14/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the <code>pub docs: Vec&lt;FileDoc&gt;,</code> line in <code>struct Scene</code> of <code>lessons/13/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> texts: Vec&lt;super::manifest::TextItem&gt;, <span class="c">// Authored text on a world plane, from the manifest.</span></code></pre></div>
<p>Added after the <code>docs: Vec::new(),</code> line in <code>fn new</code> of <code>lessons/13/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            texts: Vec::new(),</code></pre></div>
<p>Added after the <code>self.docs.clear();</code> line in <code>fn clear</code> of <code>lessons/13/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.texts.clear();</code></pre></div>
<h2 id="step-9-srclibrs">Step 9 · src/lib.rs<a class="anchor" href="#/course/14-loading#step-9-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/14/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>mod engine;</code> line of <code>lessons/13/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">mod</span> selftest;</code></pre></div>
<p>Added after the <code>File(FileDoc),</code> line in <code>enum Msg</code> of <code>lessons/13/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Texts(Vec&lt;app::manifest::TextItem&gt;), <span class="c">// text labels to place</span></code></pre></div>
<p>Added after the <code>Msg::File(doc) =&gt; state.append(doc),</code> line in <code>fn user_event</code> of <code>lessons/13/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::Texts(texts) =&gt; state.set_texts(texts),</code></pre></div>
<h2 id="step-10-srcstaters">Step 10 · src/state.rs<a class="anchor" href="#/course/14-loading#step-10-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/14/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>);</code> line in <code>fn append</code> of <code>lessons/13/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Replace the scene's text labels.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_texts(&amp;<span class="k">mut</span> <span class="k">self</span>, texts: Vec&lt;<span class="k">crate</span>::app::manifest::TextItem&gt;) {
        <span class="k">self</span>.scene.texts = texts;
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.include_text_bounds();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Start a streamed point cloud; returns its slot.</span></code></pre></div>
<p>Added after the <code>let mut labels = self.scene_labels.clone();</code> line in <code>fn update_label</code> of <code>lessons/13/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">for</span> (index, text) <span class="k">in</span> <span class="k">self</span>.scene.texts.iter().enumerate() {
            labels.push(TextLabel {
                id: (<span class="k">self</span>.scene.docs.len() + index + <span class="s">1</span>) <span class="k">as</span> u32, <span class="c">// after the document ids</span>
                text: text.text.clone(), <span class="c">// the label's words</span>
                font_size: <span class="s">18</span>.<span class="s">0</span>,
                line_height: <span class="s">26</span>.<span class="s">0</span>,
                color: [<span class="s">255</span>; <span class="s">4</span>],
                placement: TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                },
                clip: None,
            });
        }</code></pre></div>
<p>Added after the <code>self.status(&amp;format!(&quot;Text: {error}&quot;));</code> line in <code>fn update_label</code> of <code>lessons/13/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Authored text planes count in camera fitting.</span>
    <span class="k">fn</span> include_text_bounds(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">self</span>.gpu.text.document.runs {
            <span class="k">let</span> TextPlacement::WorldPlane {
                world,
                right,
                up,
                world_height,
            } = run.label.placement
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> unit = world_height / f64::from(run.label.font_size);
            <span class="k">let</span> <span class="k">mut</span> width = <span class="s">0</span>.<span class="s">0f64</span>;
            <span class="k">let</span> <span class="k">mut</span> height = <span class="s">0</span>.<span class="s">0f64</span>;

            <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
                width = width.max(f64::from(line.line_w) * unit);
                height = height.max(f64::from(line.line_top + line.line_height) * unit);
            }

            <span class="k">let</span> padding = world_height * <span class="s">0</span>.<span class="s">125</span>;

            <span class="k">for</span> x <span class="k">in</span> [-padding, width + padding] {
                <span class="k">for</span> y <span class="k">in</span> [-padding, height + padding] {
                    <span class="k">let</span> <span class="k">mut</span> point = [<span class="s">0</span>.<span class="s">0f32</span>; <span class="s">3</span>];

                    <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                        point[axis] = (world[axis] + right[axis] * x - up[axis] * y) <span class="k">as</span> f32;
                    }

                    <span class="k">if</span> point.into_iter().all(f32::is_finite) {
                        <span class="k">self</span>.gpu.bounds.union_with_point(
                            point[<span class="s">0</span>] <span class="k">as</span> f64,
                            point[<span class="s">1</span>] <span class="k">as</span> f64,
                            point[<span class="s">2</span>] <span class="k">as</span> f64,
                        );
                    }
                }
            }
        }
    }

    <span class="c">/// Show a message in the status line.</span></code></pre></div>
<h2 id="step-11-trunktoml">Step 11 · Trunk.toml<a class="anchor" href="#/course/14-loading#step-11-trunktoml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/14/Trunk.toml</code> · 17 lines · copy the file, replace the whole file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>[build]
dist = &quot;<span class="s">../../../target/lessons/14/dist</span>&quot;
<span class="c"># Benchmarks and tutorial checkpoints use this same optimized browser build.</span>
release = <span class="s">true</span>
no_sri = <span class="s">true</span> <span class="c"># SRI = a hash on each script tag that the browser checks; not used here</span>
public_url = &quot;<span class="s">./</span>&quot; <span class="c"># links relative to the page, so the site works from any folder</span>


[watch]
<span class="c"># This replaces Trunk's default watch set, so the shared path dependency is explicit.</span>
watch = [&quot;<span class="s">src</span>&quot;, &quot;<span class="s">Cargo.toml</span>&quot;, &quot;<span class="s">index.html</span>&quot;, &quot;<span class="s">assets/view_local.yaml</span>&quot;, &quot;<span class="s">assets/text</span>&quot;, &quot;<span class="s">assets/text-quality.html</span>&quot;, &quot;<span class="s">../../../../session_rust/src</span>&quot;]
enable_cooldown = <span class="s">true</span>

[serve]
<span class="c"># The mutable development index must always name the current hashed WASM bundle.</span>
headers = { &quot;Cache-Control&quot; = &quot;<span class="s">no-store</span>&quot; }
addresses = [&quot;<span class="s">127.0.0.1</span>&quot;]
port = <span class="s">8770</span></code></pre></div>
<p>The maintained viewer adds one more table here, a <code>[[hooks]]</code> pre-build step that runs <code>docs/build_site.sh</code> so the course site is served next to the viewer.</p>
<h2 id="step-12-docsbuild_sitesh">Step 12 · docs/build_site.sh<a class="anchor" href="#/course/14-loading#step-12-docsbuild_sitesh" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/14/docs/build_site.sh</code> · 23 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">#!/usr/bin/env bash</span>
<span class="c"># Trunk pre-build hook: keep target/docs/site current so the viewer can copy it into dist/docs.</span>
<span class="c"># Rebuilds only when a documentation source is newer than the built site; quiet otherwise.</span>
set <span class="s">-euo</span> <span class="s">pipefail</span>
root=$(cd &quot;$(<span class="s">dirname </span>&quot;\${<span class="s">BASH_SOURCE</span>[<span class="s">0</span>]}&quot;)<span class="s">/..</span>&quot; &amp;&amp; pwd)
site=&quot;$<span class="s">root/target/docs/site</span>&quot;
stamp=&quot;$<span class="s">site/index.html</span>&quot;
newer=$(find &quot;$<span class="s">root/docs</span>&quot; &quot;$<span class="s">root/ARCHITECTURE.md</span>&quot; &quot;$<span class="s">root/mkdocs.yml</span>&quot; <span class="s">-newer</span> &quot;$<span class="s">stamp</span>&quot; <span class="s">-type</span> <span class="s">f</span> <span class="s">-not</span> <span class="s">-path</span> '<span class="s">*/__pycache__/*</span>' 2&gt;<span class="s">/dev/null</span> | head <span class="s">-1</span> || true)
<span class="k">if</span> [ -f &quot;$<span class="s">stamp</span>&quot; ] &amp;&amp; [ -z &quot;$<span class="s">newer</span>&quot; ]; <span class="k">then</span>
    exit <span class="s">0</span>
<span class="k">fi</span>
<span class="k">if</span> command <span class="s">-v</span> <span class="s">uvx</span> &gt;<span class="s">/dev/null</span> 2&gt;&amp;1 &amp;&amp; [ -x &quot;$<span class="s">root/docs/serve.sh</span>&quot; ] &amp;&amp; [ -f &quot;$<span class="s">root/mkdocs.yml</span>&quot; ]; <span class="k">then</span>
    &quot;$<span class="s">root/docs/serve.sh</span>&quot; <span class="s">build</span> <span class="s">--quiet</span>
<span class="k">else</span>
    mkdir <span class="s">-p</span> &quot;$<span class="s">site</span>&quot;
    cat &gt; &quot;$<span class="s">stamp</span>&quot; &lt;&lt;'HTML'
<span class="s">&lt;!doctype html&gt;&lt;html lang=&quot;en&quot;&gt;&lt;meta charset=&quot;utf-8&quot;&gt;&lt;title&gt;Session Viewer course&lt;/title&gt;</span>
<span class="s">&lt;body style=&quot;font:16px system-ui;padding:3rem&quot;&gt;&lt;h1&gt;Documentation not built&lt;/h1&gt;</span>
<span class="s">&lt;p&gt;The course sources are not in this checkout, or &lt;a href=&quot;https://docs.astral.sh/uv/&quot;&gt;uv&lt;/a&gt; is missing.</span>
<span class="s">Run &lt;code&gt;docs/serve.sh build&lt;/code&gt; in the maintained viewer checkout, then rebuild.&lt;/p&gt;&lt;/body&gt;&lt;/html&gt;</span>
HTML
    echo &quot;<span class="s">docs/build_site.sh: course sources or uvx not found; wrote a placeholder page</span>&quot; &gt;&amp;2
<span class="k">fi</span></code></pre></div>
<h2 id="step-13-indexhtml">Step 13 · index.html<a class="anchor" href="#/course/14-loading#step-13-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Add the <code>copy-dir</code> link only now: Trunk refuses to build when its source directory is missing.</p>
<p><code>lessons/14/index.html</code> · edit · type this</p>
<p>Added after the <code>&lt;link data-trunk rel=&quot;copy-file&quot; href=&quot;assets…</code> line of <code>lessons/13/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">&lt;!-- The built course site (docs/build_site.sh keeps it current before each build). --&gt;</span>
    &lt;link data-trunk rel=&quot;<span class="s">copy-dir</span>&quot; href=&quot;<span class="s">target/docs/site</span>&quot; data-target-path=&quot;<span class="s">docs</span>&quot;/&gt;</code></pre></div>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/14/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/14/src/selftest.rs</code></li>
<li><code>lessons/14/src/selftest/lifecycle.rs</code></li>
<li><code>lessons/14/tests/lifecycle.cjs</code></li>
<li><code>lessons/14/tests/loading.cjs</code></li>
</ul>
<h2 id="check">Check<a class="anchor" href="#/course/14-loading#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/14/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The local fixture loads from a manifest and still supports selection and control points; status: <strong>the status clears when loading finishes</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/14.png" alt="Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Nothing loads: the status names a failed fetch, parse or decode stage.</li>
<li>A replaced scene reappears: an old loader generation is allowed to publish.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/14-loading#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/14/src/app/
├── walk/
│   ├── bounds.rs
│   ├── brep.rs
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── cloud.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── frames.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   ├── mod.rs
│   └── points.rs
├── cloud_query.rs
├── decode.rs  +
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs
├── knobs.rs
├── live.rs  +
├── loader.rs  ~
├── manifest.rs  +
├── mod.rs  ~
├── route.rs  ~
├── scene.rs  ~
├── selection.rs
├── stream.rs
├── touch.rs
└── validate.rs  +</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/14/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/14-loading#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/15-publication">15 · Publication and streamed reads</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/14-loading#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.</p>
<p><a href="/session/docs/course/docs/screenshots/14.png"><img src="/session/docs/course/docs/screenshots/14.png" alt="Full viewer result for 14 loading" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappmanifestrs",text:"Step 1 · src/app/manifest.rs"},{level:2,id:"step-2-srcappvalidaters",text:"Step 2 · src/app/validate.rs"},{level:2,id:"step-3-srcappdecoders",text:"Step 3 · src/app/decode.rs"},{level:2,id:"step-4-srcapprouters",text:"Step 4 · src/app/route.rs"},{level:2,id:"step-5-srcapplivers",text:"Step 5 · src/app/live.rs"},{level:2,id:"step-6-srcapploaderrs",text:"Step 6 · src/app/loader.rs"},{level:2,id:"step-7-srcappmodrs",text:"Step 7 · src/app/mod.rs"},{level:2,id:"step-8-srcappsceners",text:"Step 8 · src/app/scene.rs"},{level:2,id:"step-9-srclibrs",text:"Step 9 · src/lib.rs"},{level:2,id:"step-10-srcstaters",text:"Step 10 · src/state.rs"},{level:2,id:"step-11-trunktoml",text:"Step 11 · Trunk.toml"},{level:2,id:"step-12-docsbuild_sitesh",text:"Step 12 · docs/build_site.sh"},{level:2,id:"step-13-indexhtml",text:"Step 13 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
