const s={title:"19 · Sheets: batched drawings with lazy metadata",html:`<h1 id="19-sheets-batched-drawings-with-lazy-metadata">19 · Sheets: batched drawings with lazy metadata<a class="anchor" href="#/course/19-sheets#19-sheets-batched-drawings-with-lazy-metadata" aria-label="Link to this section">#</a></h1>
<p>Two vector sheets stream into top view and clicking a segment resolves its source entity.</p>
<p><img src="/session/docs/course/docs/illustrations/sheet-cost.svg" alt="As objects, every line pays for a GUID string, a name, a colour and four copies of itself; as one batch a line is a few numbers and a small source id, with guid, name and kind in a side table read only when something is selected." loading="lazy" decoding="async"></p>
<h2 id="step-1-session_protosheetproto">Step 1 · session_proto/sheet.proto<a class="anchor" href="#/course/19-sheets#step-1-session_protosheetproto" aria-label="Link to this section">#</a></h2>
<p>Read this source file from its link; the checkpoint already contains it.</p>
<details class="note"><summary><code>session_proto/sheet.proto</code> · read only</summary><p><a href="#/course/kernel/sheet_proto">Open the full listing</a></p>
</details>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/19/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/19/examples/mk_sheet.rs</code></li>
</ul>
<h2 id="step-2-srcappstreamrs">Step 2 · src/app/stream.rs<a class="anchor" href="#/course/19-sheets#step-2-srcappstreamrs" aria-label="Link to this section">#</a></h2>
<p>Streaming reads bounded chunks and keeps stable source addresses.</p>
<p><code>lessons/19/src/app/stream.rs</code> · edit · type this</p>
<p>Added after the <code>pub revision: Option&lt;String&gt;,</code> line in <code>struct CloudFields</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Byte positions of a sheet's arrays in its file; length 0 = absent.</span>
#[derive(Clone, Debug, Default)]
<span class="k">pub</span> <span class="k">struct</span> SheetFields {
    <span class="k">pub</span> end: u64, <span class="c">// end of the sheet message</span>
    <span class="k">pub</span> coords_at: u64,
    <span class="k">pub</span> coords_len: u64,
    <span class="k">pub</span> colors_at: u64,
    <span class="k">pub</span> colors_len: u64, <span class="c">// their length</span>
    <span class="k">pub</span> widths_at: u64, <span class="c">// start of the pen widths</span>
    <span class="k">pub</span> widths_len: u64, <span class="c">// their length</span>
    <span class="k">pub</span> ids_at: u64, <span class="c">// start of the entity ids</span>
    <span class="k">pub</span> ids_len: u64, <span class="c">// their length</span>
    <span class="k">pub</span> count: u32, <span class="c">// segments in the sheet</span>
    <span class="k">pub</span> entities: u32, <span class="c">// records in the side table</span>
    <span class="k">pub</span> meta: String, <span class="c">// side table file name, empty = none</span>
    <span class="k">pub</span> revision: Option&lt;String&gt;, <span class="c">// file ETag every read must match</span>
}

<span class="c">/// One protobuf field header.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> Field {
    <span class="k">pub</span> field: u32,
    <span class="k">pub</span> wire: u32,
    <span class="k">pub</span> value: u64, <span class="c">// the varint value, or the body length</span>
    <span class="k">pub</span> body: u64,
    <span class="k">pub</span> next: u64, <span class="c">// where the next field starts</span>
}

<span class="c">/// Parse the field header at \`at\`; None when it runs past \`end\`.</span>
<span class="k">pub</span> <span class="k">fn</span> field_at(header: &amp;[u8], at: u64, end: u64) -&gt; Option&lt;Field&gt; {
    <span class="k">let</span> (tag, used) = varint(header, <span class="s">0</span>)?;
    <span class="k">let</span> (field, wire) = (u32::try_from(tag &gt;&gt; <span class="s">3</span>).ok()?, (tag &amp; <span class="s">7</span>) <span class="k">as</span> u32);

    <span class="k">if</span> field == <span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> body = body_end(at, used <span class="k">as</span> u64, end)?;

    <span class="k">if</span> wire != <span class="s">2</span> {
        <span class="k">let</span> (value, _) = <span class="k">if</span> wire == <span class="s">0</span> {
            varint(header, used)?
        } <span class="k">else</span> {
            (<span class="s">0</span>, <span class="s">0</span>)
        };
        <span class="k">let</span> skip = skip_scalar(header, used, wire)?;
        <span class="k">return</span> Some(Field {
            field,
            wire,
            value,
            body,
            next: body_end(body, skip <span class="k">as</span> u64, end)?,
        });
    }

    <span class="k">let</span> (length, extra) = varint(header, used)?;
    <span class="k">let</span> body = body_end(body, extra <span class="k">as</span> u64, end)?;
    Some(Field {
        field,
        wire,
        value: length,
        body,
        next: body_end(body, length, end)?,
    })
}

<span class="k">impl</span> SheetFields {
    <span class="c">/// Bytes per segment: six doubles.</span>
    <span class="k">pub</span> <span class="k">const</span> SEGMENT_BYTES: u64 = <span class="s">48</span>;

    <span class="c">/// Record one field found after \`coords\`; false when it is wrong.</span>
    <span class="k">pub</span> <span class="k">fn</span> set(&amp;<span class="k">mut</span> <span class="k">self</span>, f: &amp;Field, body: &amp;[u8]) -&gt; bool {
        <span class="k">let</span> per_segment = u64::from(<span class="k">self</span>.count) * <span class="s">4</span>;

        <span class="k">match</span> (f.field, f.wire) {
            (<span class="s">4</span>, <span class="s">2</span>) <span class="k">if</span> <span class="k">self</span>.colors_len == <span class="s">0</span> &amp;&amp; f.value == per_segment =&gt; {
                (<span class="k">self</span>.colors_at, <span class="k">self</span>.colors_len) = (f.body, f.value)
            }
            (<span class="s">5</span>, <span class="s">2</span>) <span class="k">if</span> <span class="k">self</span>.widths_len == <span class="s">0</span> &amp;&amp; f.value == per_segment =&gt; {
                (<span class="k">self</span>.widths_at, <span class="k">self</span>.widths_len) = (f.body, f.value)
            }
            (<span class="s">15</span>, <span class="s">2</span>) <span class="k">if</span> <span class="k">self</span>.ids_len == <span class="s">0</span> &amp;&amp; f.value == per_segment =&gt; {
                (<span class="k">self</span>.ids_at, <span class="k">self</span>.ids_len) = (f.body, f.value)
            }
            (<span class="s">6</span>, <span class="s">0</span>) =&gt; <span class="k">return</span> f.value == u64::from(<span class="k">self</span>.count),
            (<span class="s">7</span>, <span class="s">0</span>) =&gt; <span class="k">match</span> u32::try_from(f.value) {
                Ok(entities) =&gt; <span class="k">self</span>.entities = entities,
                Err(_) =&gt; <span class="k">return</span> <span class="s">false</span>,
            },
            (<span class="s">8</span>, <span class="s">2</span>) <span class="k">if</span> f.value &lt;= NAME_BYTES =&gt; <span class="k">match</span> std::str::from_utf8(body) {
                Ok(meta) =&gt; <span class="k">self</span>.meta = meta.to_string(),
                Err(_) =&gt; <span class="k">return</span> <span class="s">false</span>,
            },
            (<span class="s">1</span>..=<span class="s">8</span>, _) | (<span class="s">15</span>, _) =&gt; <span class="k">return</span> <span class="s">false</span>,
            _ =&gt; {}
        }

        <span class="s">true</span>
    }
}

<span class="c">/// A cloud's octree node table.</span></code></pre></div>
<p>Replaces the \`\` line of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    first_array(head, at, end)
}

<span class="c">/// Start, length of \`coords\` and end of the sheet message.</span>
<span class="k">pub</span> <span class="k">fn</span> sheet_layout(head: &amp;[u8]) -&gt; Option&lt;(u64, u64, u64)&gt; {
    <span class="k">let</span> <span class="k">mut</span> at = <span class="s">0usize</span>;
    <span class="k">let</span> objects_end = descend_message(head, &amp;<span class="k">mut</span> at, None, <span class="s">3</span>)?;
    <span class="k">let</span> end = descend_message(head, &amp;<span class="k">mut</span> at, Some(objects_end), <span class="s">17</span>)?;
    first_array(head, at, end)
}

<span class="c">/// The \`coords\` field, which only small names may precede.</span>
<span class="k">fn</span> first_array(head: &amp;[u8], <span class="k">mut</span> at: usize, end: u64) -&gt; Option&lt;(u64, u64, u64)&gt; {</code></pre></div>
<p>Replaces the <code>if field == 4 || length &gt; NAME_BYTES {</code> line in <code>fn cloud_layout</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> field &gt; <span class="s">3</span> || length &gt; NAME_BYTES {</code></pre></div>
<p>Replaces the doc line of <code>fn descend_message</code> in <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Enter field \`want\` of the message at \`at\`; returns its end.</span></code></pre></div>
<p>Replaces the <code>if want == 8 &amp;&amp; parent_end != Some(next) {</code> line in <code>fn descend_message</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(end) = parent_end
                &amp;&amp; end != next
            {</code></pre></div>
<p>Replaces the <code>if raw.len() as u64 != u64::from(count).check…</code> line in <code>fn checked_positions</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    checked_doubles(raw, u64::from(count).checked_mul(<span class="s">3</span>)?)
}

<span class="c">/// Exactly \`n\` doubles as f32; None when short or not finite.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">fn</span> checked_doubles(raw: &amp;[u8], n: u64) -&gt; Option&lt;Vec&lt;f32&gt;&gt; {
    <span class="k">if</span> raw.len() <span class="k">as</span> u64 != n.checked_mul(<span class="s">8</span>)? {</code></pre></div>
<p>Delete the 2 lines from <code>#[cfg(any(target_arch = &quot;wasm32&quot;, test))]</code> of <code>lessons/18/src/app/stream.rs</code>.</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// How much to read at once: at least \`length\`, up to 64 KiB.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A packed \`fixed32\` array in full.</span>
<span class="k">pub</span> <span class="k">fn</span> packed_u32(raw: &amp;[u8]) -&gt; Vec&lt;u32&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(raw.len() / <span class="s">4</span>);

    <span class="k">for</span> c <span class="k">in</span> raw.chunks_exact(<span class="s">4</span>) {
        out.push(u32::from_le_bytes(c.try_into().unwrap()));
    }

    out
}

<span class="c">/// A packed \`float\` array in full; a non-finite value becomes 0.</span>
<span class="k">pub</span> <span class="k">fn</span> packed_f32(raw: &amp;[u8]) -&gt; Vec&lt;f32&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(raw.len() / <span class="s">4</span>);

    <span class="k">for</span> c <span class="k">in</span> raw.chunks_exact(<span class="s">4</span>) {
        <span class="k">let</span> v = f32::from_le_bytes(c.try_into().unwrap());
        out.push(<span class="k">if</span> v.is_finite() { v } <span class="k">else</span> { <span class="s">0</span>.<span class="s">0</span> });
    }

    out
}

<span class="c">/// An already-fetched coords slice as f32 triples.</span></code></pre></div>
<p>Added after the <code>use crate::app::fetch::{GetOpts, fetch_range,…</code> line in <code>mod web</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">use</span> <span class="k">crate</span>::app::walk::sheet::SheetRows;</code></pre></div>
<p>Replaces the 7 lines from <code>fields: &amp;CloudFields,</code> in <code>fn read</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            end: u64,
            revision: &amp;Option&lt;String&gt;,
        ) -&gt; Option&lt;&amp;[u8]&gt; {
            body_end(at, length, end)?;

            <span class="k">if</span> <span class="k">self</span>.slice(at, length).is_none() {
                <span class="k">let</span> read_length = <span class="k">Self</span>::read_length(at, length, end)?;
                <span class="k">self</span>.bytes = source_range(url, at, read_length, revision).<span class="k">await</span>?;</code></pre></div>
<p>Replaces the <code>.read(url, at, 64.min(fields.end - at), fields)</code> line in <code>fn cloud_lod</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                .read(
                    url,
                    at,
                    <span class="s">64</span>.min(fields.end - at),
                    fields.end,
                    &amp;fields.revision,
                )</code></pre></div>
<p>Replaces the <code>let raw = window.read(url, body, length, fiel…</code> line in <code>fn cloud_lod</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> raw = window
                    .read(url, body, length, fields.end, &amp;fields.revision)
                    .<span class="k">await</span>?;</code></pre></div>
<p>Added after the <code>Some((colors, body_end(at, used as u64, end)?))</code> line in <code>fn fetch_colors</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Find where a sheet's arrays are in the file.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> sheet_fields(url: &amp;str) -&gt; Option&lt;SheetFields&gt; {
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

        <span class="k">let</span> (coords_at, coords_len, end) = sheet_layout(&amp;reply.bytes)?;

        <span class="k">if</span> coords_len == <span class="s">0</span> || !coords_len.is_multiple_of(SheetFields::SEGMENT_BYTES) {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> <span class="k">mut</span> fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: u32::try_from(coords_len / SheetFields::SEGMENT_BYTES).ok()?,
            revision: reply.etag,
            ..Default::default()
        };
        <span class="k">let</span> <span class="k">mut</span> at = body_end(coords_at, coords_len, end)?;
        <span class="k">let</span> <span class="k">mut</span> window = MetadataWindow::default();

        <span class="k">while</span> at &lt; end {
            <span class="k">let</span> header = window
                .read(url, at, <span class="s">64</span>.min(end - at), end, &amp;fields.revision)
                .<span class="k">await</span>?;
            <span class="k">let</span> f = field_at(header, at, end)?;
            <span class="k">let</span> body = <span class="k">if</span> (f.field, f.wire) == (<span class="s">8</span>, <span class="s">2</span>) {
                <span class="k">if</span> f.value &gt; NAME_BYTES {
                    <span class="k">return</span> None;
                }

                window
                    .read(url, f.body, f.value, end, &amp;fields.revision)
                    .<span class="k">await</span>?
            } <span class="k">else</span> {
                &amp;[]
            };

            <span class="k">if</span> !fields.set(&amp;f, body) {
                <span class="k">return</span> None;
            }

            at = f.next;
        }

        Some(fields)
    }

    <span class="c">/// Entries \`[from, to)\` of one fixed-width array.</span>
    <span class="k">async</span> <span class="k">fn</span> sheet_array(
        url: &amp;str,
        fields: &amp;SheetFields,
        (at, len): (u64, u64),
        from: u32,
        to: u32,
        stride: u64,
    ) -&gt; Option&lt;Vec&lt;u8&gt;&gt; {
        <span class="k">if</span> len == <span class="s">0</span> {
            <span class="k">return</span> Some(Vec::new());
        }

        <span class="k">let</span> start = at.checked_add(u64::from(from).checked_mul(stride)?)?;
        <span class="k">let</span> length = u64::from(to - from).checked_mul(stride)?;
        body_end(start, length, body_end(at, len, fields.end)?)?;
        source_range(url, start, length, &amp;fields.revision).<span class="k">await</span>
    }

    <span class="c">/// Segments \`[from, to)\` of a sheet, every array.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> fetch_sheet_slice(
        url: &amp;str,
        fields: &amp;SheetFields,
        from: u32,
        to: u32,
    ) -&gt; Option&lt;SheetRows&gt; {
        <span class="k">if</span> from &gt; to || to &gt; fields.count {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> coords = (fields.coords_at, fields.coords_len);
        <span class="k">let</span> raw = sheet_array(url, fields, coords, from, to, SheetFields::SEGMENT_BYTES).<span class="k">await</span>?;
        <span class="k">let</span> positions = checked_doubles(&amp;raw, u64::from(to - from) * <span class="s">6</span>)?;
        <span class="c">// colour, width and id are 4 bytes a segment</span>
        <span class="k">let</span> colors = (fields.colors_at, fields.colors_len);
        <span class="k">let</span> colors = packed_u32(&amp;sheet_array(url, fields, colors, from, to, <span class="s">4</span>).<span class="k">await</span>?);
        <span class="k">let</span> widths = (fields.widths_at, fields.widths_len);
        <span class="k">let</span> widths = packed_f32(&amp;sheet_array(url, fields, widths, from, to, <span class="s">4</span>).<span class="k">await</span>?);
        <span class="k">let</span> ids = (fields.ids_at, fields.ids_len);
        <span class="k">let</span> ids = packed_u32(&amp;sheet_array(url, fields, ids, from, to, <span class="s">4</span>).<span class="k">await</span>?);
        Some(SheetRows {
            positions,
            colors,
            widths,
            ids,
        })
    }</code></pre></div>
<p>Added after the <code>}</code> line in <code>mod tests</code> of <code>lessons/18/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One protobuf varint.</span>
    <span class="k">fn</span> uvarint(<span class="k">mut</span> v: u64) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">loop</span> {
            <span class="k">let</span> byte = (v &amp; <span class="s">0x7f</span>) <span class="k">as</span> u8;
            v &gt;&gt;= <span class="s">7</span>;

            <span class="k">if</span> v == <span class="s">0</span> {
                out.push(byte);
                <span class="k">return</span> out;
            }

            out.push(byte | <span class="s">0x80</span>);
        }
    }

    <span class="c">/// One length-delimited field: tag, length, body.</span>
    <span class="k">fn</span> bytes_field(field: u32, body: &amp;[u8]) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = uvarint(u64::from(field &lt;&lt; <span class="s">3</span> | <span class="s">2</span>));
        out.extend(uvarint(body.len() <span class="k">as</span> u64));
        out.extend_from_slice(body);
        out
    }

    <span class="c">/// One varint field.</span>
    <span class="k">fn</span> uint_field(field: u32, value: u32) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = uvarint(u64::from(field &lt;&lt; <span class="s">3</span>));
        out.extend(uvarint(u64::from(value)));
        out
    }

    <span class="c">/// A one-segment sheet file.</span>
    <span class="k">fn</span> sheet_file() -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> coords: Vec&lt;u8&gt; = [<span class="s">0</span>.<span class="s">0f64</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        <span class="k">let</span> <span class="k">mut</span> sheet = bytes_field(<span class="s">1</span>, <span class="s">b</span>&quot;<span class="s">guid</span>&quot;);
        sheet.extend(bytes_field(<span class="s">2</span>, <span class="s">b</span>&quot;<span class="s">plan</span>&quot;));
        sheet.extend(bytes_field(<span class="s">3</span>, &amp;coords));
        sheet.extend(bytes_field(<span class="s">4</span>, &amp;<span class="s">0xff00_00ffu32</span>.to_le_bytes()));
        sheet.extend(bytes_field(<span class="s">5</span>, &amp;<span class="s">0</span>.<span class="s">35f32</span>.to_le_bytes()));
        sheet.extend(uint_field(<span class="s">6</span>, <span class="s">1</span>));
        sheet.extend(uint_field(<span class="s">7</span>, <span class="s">9</span>));
        sheet.extend(bytes_field(<span class="s">8</span>, <span class="s">b</span>&quot;<span class="s">plan.shm</span>&quot;));
        sheet.extend(bytes_field(<span class="s">15</span>, &amp;<span class="s">7u32</span>.to_le_bytes()));
        <span class="k">let</span> objects = bytes_field(<span class="s">17</span>, &amp;sheet);
        bytes_field(<span class="s">3</span>, &amp;objects)
    }

    <span class="c">/// Scan the fields after \`coords\` from memory.</span>
    <span class="k">fn</span> scan_tail(file: &amp;[u8], fields: &amp;<span class="k">mut</span> SheetFields, <span class="k">mut</span> at: u64) -&gt; bool {
        <span class="k">while</span> at &lt; fields.end {
            <span class="k">let</span> Some(f) = field_at(&amp;file[at <span class="k">as</span> usize..], at, fields.end) <span class="k">else</span> {
                <span class="k">return</span> <span class="s">false</span>;
            };
            <span class="k">let</span> body = <span class="k">if</span> f.wire == <span class="s">2</span> {
                &amp;file[f.body <span class="k">as</span> usize..(f.body + f.value.min(NAME_BYTES)) <span class="k">as</span> usize]
            } <span class="k">else</span> {
                &amp;[]
            };

            <span class="k">if</span> !fields.set(&amp;f, body) {
                <span class="k">return</span> <span class="s">false</span>;
            }

            at = f.next;
        }

        <span class="s">true</span>
    }

    <span class="c">/// The sheet walk finds every array.</span>
    #[test]
    <span class="k">fn</span> sheet_layout_walks_objects_field_17_and_locates_every_array() {
        <span class="k">let</span> file = sheet_file();
        <span class="k">let</span> (coords_at, coords_len, end) = sheet_layout(&amp;file).unwrap();
        assert_eq!(coords_len, <span class="s">48</span>);
        assert_eq!(end, file.len() <span class="k">as</span> u64);
        assert_eq!(
            packed_f64(&amp;file[coords_at <span class="k">as</span> usize..(coords_at + <span class="s">48</span>) <span class="k">as</span> usize])[<span class="s">5</span>],
            <span class="s">5</span>.<span class="s">0</span>
        );
        <span class="k">let</span> <span class="k">mut</span> fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: <span class="s">1</span>,
            ..Default::default()
        };
        assert!(scan_tail(&amp;file, &amp;<span class="k">mut</span> fields, coords_at + coords_len));
        assert_eq!(fields.colors_len, <span class="s">4</span>);
        assert_eq!(fields.widths_len, <span class="s">4</span>);
        assert_eq!(fields.ids_len, <span class="s">4</span>);
        assert_eq!(fields.entities, <span class="s">9</span>);
        assert_eq!(fields.meta, &quot;<span class="s">plan.shm</span>&quot;);
        assert_eq!(
            packed_u32(&amp;file[fields.colors_at <span class="k">as</span> usize..][..<span class="s">4</span>]),
            [<span class="s">0xff00_00ff</span>]
        );
        assert_eq!(packed_f32(&amp;file[fields.widths_at <span class="k">as</span> usize..][..<span class="s">4</span>]), [<span class="s">0</span>.<span class="s">35</span>]);
        assert_eq!(packed_u32(&amp;file[fields.ids_at <span class="k">as</span> usize..][..<span class="s">4</span>]), [<span class="s">7</span>]);
        assert_eq!(fields.ids_at + <span class="s">4</span>, end);
        <span class="c">// a sheet is not a cloud; two objects are not one file</span>
        assert!(walk_to_coords(&amp;file).is_none());
        <span class="k">let</span> <span class="k">mut</span> two = bytes_field(<span class="s">17</span>, &amp;file[<span class="s">4</span>..]);
        two.extend(bytes_field(<span class="s">17</span>, <span class="s">b</span>&quot;&quot;));
        assert!(sheet_layout(&amp;bytes_field(<span class="s">3</span>, &amp;two)).is_none());
        <span class="c">// a wrong segment count is refused</span>
        <span class="k">let</span> <span class="k">mut</span> wrong = fields.clone();
        wrong.count = <span class="s">2</span>;
        assert!(!scan_tail(&amp;file, &amp;<span class="k">mut</span> wrong, coords_at + coords_len));
    }

    <span class="c">/// Colours decode and report where they end.</span></code></pre></div>
<h2 id="step-3-srcapploaderrs">Step 3 · src/app/loader.rs<a class="anchor" href="#/course/19-sheets#step-3-srcapploaderrs" aria-label="Link to this section">#</a></h2>
<p>The loader stages manifest and geometry work before publishing it.</p>
<p><code>lessons/19/src/app/loader.rs</code> · edit · type this</p>
<p>Replaces the 5 lines from <code>use super::scene::{FileDoc, Scene, StreamedIn…</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::scene::{FileDoc, Scene, SheetInit, StreamedInit};
<span class="k">use</span> super::stream::{
    CloudFields, SheetFields, cloud_fields, cloud_lod, fetch_colors, fetch_positions,
    fetch_sheet_slice, sheet_fields,
};
<span class="k">use</span> super::walk::cloud::StreamRows;
<span class="k">use</span> super::walk::sheet::SheetRows;
<span class="k">use</span> <span class="k">crate</span>::engine::performance::now_ms;
<span class="k">use</span> <span class="k">crate</span>::{CloudChunk, Msg, SheetChunk, State};</code></pre></div>
<p>Added after the <code>const STREAM_MIN_PREFIX: u32 = 250_000;</code> line of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Segments a sheet reads before its first frame.</span>
<span class="k">const</span> SHEET_PREFIX_SEGMENTS: u32 = <span class="s">500_000</span>;

<span class="c">/// Segments per follow-up slice.</span>
<span class="k">const</span> SHEET_CHUNK_SEGMENTS: u32 = <span class="s">500_000</span>;

<span class="c">/// Most sheet segments on the page, \`?segments=\` overrides.</span>
<span class="k">const</span> SHEET_MAX_SEGMENTS: u32 = <span class="s">3_000_000</span>;</code></pre></div>
<p>Added after the <code>static RESIDENT: Cell&lt;u32&gt; = const { Cell::ne…</code> line in <code>const STREAM_MIN_PREFIX</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Sheet segments loaded so far.</span>
    <span class="k">static</span> SHEET_RESIDENT: Cell&lt;u32&gt; = <span class="k">const</span> { Cell::new(<span class="s">0</span>) };

    <span class="c">/// Bumped when the scene is cleared; old stream tasks stop.</span></code></pre></div>
<p>Added after the <code>RESIDENT.set(0);</code> line in <code>fn clear_scene</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    SHEET_RESIDENT.set(<span class="s">0</span>);</code></pre></div>
<p>Added after the <code>RESIDENT.set(RESIDENT.get().saturating_add(n));</code> line in <code>fn budget_spend</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The segment ceiling, from \`?segments=\` or the default.</span>
<span class="k">fn</span> max_segments() -&gt; u32 {
    knob_u32(&quot;<span class="s">segments</span>&quot;).unwrap_or(SHEET_MAX_SEGMENTS)
}

<span class="c">/// Segments still allowed.</span>
<span class="k">fn</span> sheet_budget_left() -&gt; u32 {
    max_segments().saturating_sub(SHEET_RESIDENT.get())
}

<span class="c">/// Count \`n\` segments as loaded.</span>
<span class="k">fn</span> sheet_budget_spend(n: u32) {
    SHEET_RESIDENT.set(SHEET_RESIDENT.get().saturating_add(n));
}

<span class="c">/// Start the viewer, load the first scene, then keep polling.</span></code></pre></div>
<p>Added after the <code>let mut staged_points = 0u32;</code> line in <code>fn load_route</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> staged_segments = <span class="s">0u32</span>;</code></pre></div>
<p>Added after the <code>post(Msg::StreamedCloud(Box::new(init)));</code> line in <code>fn load_route</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> remaining = <span class="k">if</span> replacement.is_some() {
                max_segments().saturating_sub(staged_segments)
            } <span class="k">else</span> {
                sheet_budget_left()
            };

            <span class="k">if</span> <span class="k">let</span> Some(init) = sheet_prefix(&amp;url, &amp;slot, remaining).<span class="k">await</span> {
                <span class="k">if</span> stale_load(generation) {
                    <span class="k">return</span>;
                }

                <span class="k">if</span> init.resident == <span class="s">0</span> {
                    failed = <span class="s">true</span>;
                    <span class="k">continue</span>;
                }

                <span class="k">if</span> replacement.is_some() {
                    staged_segments = staged_segments.saturating_add(init.resident);
                    pending.push(PendingDocument::Sheet(Box::new(init)));
                } <span class="k">else</span> {
                    sheet_budget_spend(init.resident);
                    post(Msg::Sheet(Box::new(init)));
                }

                <span class="k">continue</span>;
            }</code></pre></div>
<p>Added after the <code>budget_spend(staged_points);</code> line in <code>fn load_route</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        sheet_budget_spend(staged_segments);</code></pre></div>
<p>Added after the <code>post(Msg::StreamedCloud(stream));</code> line in <code>fn load_route</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                PendingDocument::Sheet(sheet) =&gt; {
                    post(Msg::Sheet(sheet));
                }</code></pre></div>
<p>Added after the <code>Streamed(Box&lt;StreamedInit&gt;),</code> line in <code>enum PendingDocument</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One staged item of a reload.</span>
<span class="k">enum</span> PendingDocument {
    Whole(FileDoc),
    Streamed(Box&lt;StreamedInit&gt;), <span class="c">// a cloud's first slice</span>
    Sheet(Box&lt;SheetInit&gt;), <span class="c">// a sheet's first slice</span>
}

<span class="c">/// Name and placement of a streamed document.</span></code></pre></div>
<p>Added after the <code>})</code> line in <code>fn stream_prefix</code> of <code>lessons/18/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// \`name\` in the same folder as \`url\`.</span>
<span class="k">fn</span> sibling(url: &amp;str, name: &amp;str) -&gt; String {
    <span class="k">let</span> dir = url.rfind('<span class="s">/</span>').map_or(<span class="s">0</span>, |at| at + <span class="s">1</span>);
    format!(&quot;{}{<span class="s">name</span>}&quot;, &amp;url[..dir])
}

<span class="c">/// Read a sheet's first \`share\` segments by range; None when not a sheet file.</span>
<span class="k">async</span> <span class="k">fn</span> sheet_prefix(url: &amp;str, slot: &amp;Placement, share: u32) -&gt; Option&lt;SheetInit&gt; {
    <span class="k">let</span> name = slot.name.as_str();
    <span class="k">let</span> fields = sheet_fields(url).<span class="k">await</span>?;
    <span class="k">let</span> meta_url = (!fields.meta.is_empty()).then(|| sibling(url, &amp;fields.meta));
    <span class="k">let</span> <span class="k">mut</span> resident = SHEET_PREFIX_SEGMENTS.min(share).min(fields.count);
    <span class="k">let</span> rows = <span class="k">match</span> fetch_sheet_slice(url, &amp;fields, <span class="s">0</span>, resident).<span class="k">await</span> {
        Some(rows) <span class="k">if</span> resident &gt; <span class="s">0</span> =&gt; rows,
        _ =&gt; {
            log::warn!(
                &quot;<span class="s">'</span>{<span class="s">name</span>}<span class="s">': no sheet prefix - </span>{<span class="s">resident</span>}<span class="s"> of </span>{}<span class="s"> segments allowed (?segments= to raise the ceiling) or the range read failed</span>&quot;,
                fields.count
            );
            resident = <span class="s">0</span>;
            SheetRows {
                positions: Vec::new(),
                colors: Vec::new(),
                widths: Vec::new(),
                ids: Vec::new(),
            }
        }
    };
    log::info!(
        &quot;<span class="s">sheet '</span>{<span class="s">name</span>}<span class="s">': </span>{<span class="s">resident</span>}<span class="s"> of </span>{}<span class="s"> segments on screen, </span>{}<span class="s"> entities</span>&quot;,
        fields.count,
        fields.entities
    );
    Some(SheetInit {
        name: name.to_string(),
        url: url.to_string(),
        meta_url,
        place: slot.place.clone(),
        rows,
        fields,
        resident,
    })
}

<span class="c">/// Where a sheet's streaming continues.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetCursor {
    <span class="k">pub</span> idx: usize, <span class="c">// the sheet's slot in the scene</span>
    <span class="k">pub</span> url: String, <span class="c">// the sheet file</span>
    <span class="k">pub</span> fields: SheetFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> from: u32, <span class="c">// next segment to read</span>
}

<span class="c">/// Keep reading a sheet's slices in the background.</span>
<span class="k">pub</span> <span class="k">fn</span> spawn_sheet_rest(cursor: SheetCursor) {
    wasm_bindgen_futures::spawn_local(sheet_rest(cursor));
}

<span class="c">/// The slice loop behind \`spawn_sheet_rest\`.</span>
<span class="k">async</span> <span class="k">fn</span> sheet_rest(c: SheetCursor) {
    <span class="k">let</span> (url, idx, fields) = (c.url, c.idx, c.fields);
    <span class="k">let</span> generation = GENERATION.get();
    <span class="k">let</span> <span class="k">mut</span> at = c.from;

    <span class="k">while</span> at &lt; fields.count {
        <span class="k">if</span> GENERATION.get() != generation {
            <span class="k">return</span>;
        }

        <span class="k">let</span> left = sheet_budget_left();

        <span class="k">if</span> left == <span class="s">0</span> {
            log::info!(
                &quot;<span class="s">'</span>{<span class="s">url</span>}<span class="s">': </span>{<span class="s">at</span>}<span class="s"> of </span>{}<span class="s"> segments resident - at the page's segment ceiling (?segments= to raise it)</span>&quot;,
                fields.count
            );
            <span class="k">return</span>;
        }

        <span class="k">let</span> to = (at + SHEET_CHUNK_SEGMENTS.min(left)).min(fields.count);
        sheet_budget_spend(to - at);
        <span class="k">let</span> Some(rows) = fetch_sheet_slice(&amp;url, &amp;fields, at, to).<span class="k">await</span> <span class="k">else</span> {
            <span class="k">if</span> GENERATION.get() == generation {
                SHEET_RESIDENT.set(SHEET_RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status(&quot;<span class="s">A sheet range failed; reload to retry the missing data</span>&quot;);
            <span class="k">return</span>;
        };

        <span class="k">if</span> GENERATION.get() != generation {
            <span class="k">return</span>;
        }

        <span class="k">if</span> !post(Msg::SheetChunk(SheetChunk { idx, rows, to })) {
            <span class="k">return</span>;
        }

        at = to;
    }
}

<span class="c">/// Where a cloud's streaming continues.</span></code></pre></div>
<h2 id="step-4-srclibrs">Step 4 · src/lib.rs<a class="anchor" href="#/course/19-sheets#step-4-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/19/src/lib.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>use crate::app::scene::{FileDoc, StreamedInit};</code> of <code>lessons/18/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, SheetInit, StreamedInit};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::StreamRows;
<span class="k">use</span> <span class="k">crate</span>::app::walk::sheet::SheetRows;</code></pre></div>
<p>Added after the <code>pub to: u32,</code> line in <code>struct CloudChunk</code> of <code>lessons/18/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One more slice of sheet \`idx\`.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetChunk {
    <span class="k">pub</span> idx: usize, <span class="c">// which sheet</span>
    <span class="k">pub</span> rows: SheetRows, <span class="c">// the new segments</span>
    <span class="k">pub</span> to: u32, <span class="c">// segments loaded so far</span>
}

<span class="c">/// Messages the async loader sends to the event loop.</span></code></pre></div>
<p>Added after the <code>CloudQueryResolved(app::cloud_query::Resolved),</code> line in <code>enum Msg</code> of <code>lessons/18/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Sheet(Box&lt;SheetInit&gt;), <span class="c">// a drawing sheet starts streaming</span>
    SheetChunk(SheetChunk), <span class="c">// more segments arrived</span>
    SheetEntity(app::sheet_query::Resolved),</code></pre></div>
<p>Added after the <code>Msg::CloudQueryResolved(resolved) =&gt; state.cl…</code> line in <code>fn user_event</code> of <code>lessons/18/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::Sheet(init) =&gt; {
                <span class="k">let</span> (url, fields, from) = (init.url.clone(), init.fields.clone(), init.resident);
                <span class="k">let</span> idx = state.add_sheet(*init);
                loader::spawn_sheet_rest(loader::SheetCursor {
                    idx,
                    url,
                    fields,
                    from,
                });
            }
            Msg::SheetChunk(c) =&gt; state.extend_sheet(c.idx, c.rows, c.to),
            Msg::SheetEntity(resolved) =&gt; state.sheet_entity(resolved),</code></pre></div>
<h2 id="step-5-srcappwalksheetrs">Step 5 · src/app/walk/sheet.rs<a class="anchor" href="#/course/19-sheets#step-5-srcappwalksheetrs" aria-label="Link to this section">#</a></h2>
<p>Sheet slices append compact segments without creating one object per line.</p>
<p><code>lessons/19/src/app/walk/sheet.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Walks a sheet into GPU rows: a sheet is a 2D page of drawings, not geometry in the scene.</span>
<span class="k">use</span> super::encode::{BLACK, FACING_UNKNOWN};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::{CylinderSegment, SegDraw, SegRows};
<span class="k">use</span> session_rust::AABB;

<span class="c">/// Raw segment columns of one streamed slice.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetRows {
    <span class="k">pub</span> positions: Vec&lt;f32&gt;, <span class="c">// six floats per segment</span>
    <span class="k">pub</span> colors: Vec&lt;u32&gt;, <span class="c">// packed RGBA per segment</span>
    <span class="k">pub</span> widths: Vec&lt;f32&gt;, <span class="c">// pen width in mm per segment</span>
    <span class="k">pub</span> ids: Vec&lt;u32&gt;, <span class="c">// entity id per segment</span>
}

<span class="c">/// One slice of a sheet and where it goes.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetSlice {
    <span class="k">pub</span> rows: SheetRows, <span class="c">// the segments</span>
    <span class="k">pub</span> from: u32, <span class="c">// first segment index in the sheet</span>
    <span class="k">pub</span> row: u32,
}

<span class="c">/// Pen width in mm to a half width; 0 = hairline.</span>
<span class="k">fn</span> sheet_radius(width: f32) -&gt; f32 {
    <span class="k">if</span> width.is_finite() &amp;&amp; width &gt; <span class="s">0</span>.<span class="s">0</span> {
        width * <span class="s">0</span>.<span class="s">5</span>
    } <span class="k">else</span> {
        <span class="s">0</span>.<span class="s">0</span>
    }
}

<span class="c">/// Append one slice as ribbons; return its box.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_sheet_slice(seg: &amp;<span class="k">mut</span> SegRows, s: &amp;SheetSlice) -&gt; AABB {
    seg.ribbon_ids.resize(seg.ribbons.len(), u32::MAX); <span class="c">// older ribbons have no id</span>
    <span class="k">let</span> first = seg.ribbons.len() <span class="k">as</span> u32;
    <span class="k">let</span> count = (s.rows.positions.len() / <span class="s">6</span>) <span class="k">as</span> u32; <span class="c">// two points per segment</span>
    seg.ribbons.reserve(count <span class="k">as</span> usize);
    seg.ribbon_ids.reserve(count <span class="k">as</span> usize);
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

    <span class="k">for</span> (i, p) <span class="k">in</span> s.rows.positions.chunks_exact(<span class="s">6</span>).enumerate() {
        <span class="k">let</span> (p0, p1) = ([p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]], [p[<span class="s">3</span>], p[<span class="s">4</span>], p[<span class="s">5</span>]]);
        bounds.union_with_point(p0[<span class="s">0</span>] <span class="k">as</span> f64, p0[<span class="s">1</span>] <span class="k">as</span> f64, p0[<span class="s">2</span>] <span class="k">as</span> f64);
        bounds.union_with_point(p1[<span class="s">0</span>] <span class="k">as</span> f64, p1[<span class="s">1</span>] <span class="k">as</span> f64, p1[<span class="s">2</span>] <span class="k">as</span> f64);
        seg.ribbons.push(CylinderSegment {
            p0,
            radius: sheet_radius(s.rows.widths.get(i).copied().unwrap_or(<span class="s">0</span>.<span class="s">0</span>)), <span class="c">// missing = hairline</span>
            p1,
            instance_id: s.row,
            color: s.rows.colors.get(i).copied().unwrap_or(BLACK), <span class="c">// missing = black</span>
            facing: FACING_UNKNOWN,
        });
        seg.ribbon_ids
            .push(s.rows.ids.get(i).copied().unwrap_or(u32::MAX));
    }

    <span class="c">// one draw call per slice</span>
    seg.sheets.push(SegDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
    });
    bounds
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/19/src/app/walk/sheet.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> session_rust::Point;

    <span class="c">/// Short columns get defaults; the box covers every segment end.</span>
    #[test]
    <span class="k">fn</span> sheet_slice_pads_short_columns_and_reports_its_box() {
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        seg.ribbons.push(CylinderSegment {
            p0: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            radius: <span class="s">0</span>.<span class="s">0</span>,
            p1: [<span class="s">1</span>.<span class="s">0</span>; <span class="s">3</span>],
            instance_id: <span class="s">2</span>,
            color: BLACK,
            facing: FACING_UNKNOWN,
        });
        <span class="k">let</span> slice = SheetSlice {
            rows: SheetRows {
                positions: vec![<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">5</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
                colors: vec![<span class="s">0xff00_00ff</span>],
                widths: vec![<span class="s">1</span>.<span class="s">0</span>, f32::NAN],
                ids: vec![<span class="s">4</span>],
            },
            from: <span class="s">3</span>,
            row: <span class="s">7</span>,
        };
        <span class="k">let</span> bounds = walk_sheet_slice(&amp;<span class="k">mut</span> seg, &amp;slice);
        assert_eq!(bounds.min_point(), Point::new(-<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        assert_eq!(bounds.max_point(), Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        assert_eq!(seg.ribbons.len(), <span class="s">3</span>);
        assert_eq!(seg.ribbon_ids, [u32::MAX, <span class="s">4</span>, u32::MAX]);
        assert_eq!(seg.ribbons[<span class="s">1</span>].radius, <span class="s">0</span>.<span class="s">5</span>);
        assert_eq!(seg.ribbons[<span class="s">2</span>].radius, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(seg.ribbons[<span class="s">1</span>].color, <span class="s">0xff00_00ff</span>);
        assert_eq!(seg.ribbons[<span class="s">2</span>].color, BLACK);
        assert_eq!(seg.ribbons[<span class="s">2</span>].instance_id, <span class="s">7</span>);
        assert_eq!(seg.ribbons[<span class="s">2</span>].p1, [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]);
        <span class="k">let</span> draw = &amp;seg.sheets[<span class="s">0</span>];
        assert_eq!(
            (draw.instance, draw.from, draw.count, draw.first),
            (<span class="s">7</span>, <span class="s">3</span>, <span class="s">2</span>, <span class="s">1</span>)
        );
    }
}</code></pre></div>
<h2 id="step-6-srcappwalkmodrs">Step 6 · src/app/walk/mod.rs<a class="anchor" href="#/course/19-sheets#step-6-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>The geometry walk dispatches source types into their render buffers.</p>
<p><code>lessons/19/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod points;</code> line of <code>lessons/18/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> sheet;</code></pre></div>
<h2 id="step-7-srcenginegpusegmentsrs">Step 7 · src/engine/gpu/segments.rs<a class="anchor" href="#/course/19-sheets#step-7-srcenginegpusegmentsrs" aria-label="Link to this section">#</a></h2>
<p>The segment buffers store strokes and the object rows they belong to.</p>
<p><code>lessons/19/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the <code>const _: () = assert!(std::mem::size_of::&lt;Cyl…</code> line of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One batch of segments added to a sheet.</span>
<span class="k">pub</span> <span class="k">struct</span> SegDraw {
    <span class="k">pub</span> instance: u32, <span class="c">// object row of the sheet</span>
    <span class="k">pub</span> from: u32, <span class="c">// first segment index within the sheet</span>
    <span class="k">pub</span> count: u32, <span class="c">// segments in this batch</span>
    <span class="k">pub</span> first: u32, <span class="c">// row of the first segment in the upload</span>
}

<span class="c">/// Segment rows of one upload.</span></code></pre></div>
<p>Added after the <code>pub ribbons: Vec&lt;CylinderSegment&gt;,</code> line in <code>struct SegRows</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ribbon_ids: Vec&lt;u32&gt;, <span class="c">// source entity per ribbon, or u32::MAX</span>
    <span class="k">pub</span> sheets: Vec&lt;SegDraw&gt;, <span class="c">// sheet batches among the ribbons</span></code></pre></div>
<p>Replaces the <code>}</code> line in <code>impl SegRows</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.ribbon_ids);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.sheets);
    }
}

<span class="c">/// A run of sheet segments stored on the GPU.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> SegChunk {
    <span class="k">pub</span> from: u32, <span class="c">// first segment index within the sheet</span>
    <span class="k">pub</span> to: u32, <span class="c">// one past the last segment index</span>
    <span class="k">pub</span> row: u32, <span class="c">// GPU row of the first segment</span>
}

<span class="c">/// One drawing sheet on the GPU.</span>
<span class="k">pub</span> <span class="k">struct</span> SegSheet {
    <span class="k">pub</span> instance: u32, <span class="c">// object row</span>
    <span class="k">pub</span> resident: u32, <span class="c">// segments uploaded so far</span>
    <span class="k">pub</span> chunks: Vec&lt;SegChunk&gt;, <span class="c">// where those segments live</span>
    <span class="k">pub</span> ids: Vec&lt;u32&gt;, <span class="c">// source entity per segment</span>
}

<span class="c">/// Open a sheet or add a batch to the one on \`instance\`.</span>
<span class="k">fn</span> push_chunk(sheets: &amp;<span class="k">mut</span> Vec&lt;SegSheet&gt;, instance: u32, chunk: SegChunk, ids: &amp;[u32]) {
    <span class="k">if</span> chunk.from == <span class="s">0</span> {
        sheets.push(SegSheet {
            instance,
            resident: chunk.to,
            chunks: vec![chunk],
            ids: ids.to_vec(),
        });
        <span class="k">return</span>;
    }

    <span class="k">for</span> sheet <span class="k">in</span> sheets.iter_mut() {
        <span class="k">if</span> sheet.instance != instance {
            <span class="k">continue</span>;
        }

        <span class="c">// batches must arrive in order</span>
        <span class="k">if</span> chunk.from != sheet.resident {
            log::warn!(
                &quot;<span class="s">sheet chunk [</span>{}<span class="s">, </span>{}<span class="s">) does not continue the </span>{}<span class="s"> resident segments; dropped</span>&quot;,
                chunk.from,
                chunk.to,
                sheet.resident
            );
            <span class="k">return</span>;
        }

        sheet.resident = chunk.to;
        sheet.chunks.push(chunk);
        sheet.ids.extend_from_slice(ids);
        <span class="k">return</span>;
    }

    log::warn!(&quot;<span class="s">sheet chunk for row </span>{<span class="s">instance</span>}<span class="s"> arrived before its sheet; dropped</span>&quot;);
}

<span class="c">/// Sheet of a GPU ribbon row: (sheet index, segment index).</span>
<span class="k">fn</span> sheet_of(sheets: &amp;[SegSheet], row: u32) -&gt; Option&lt;(usize, u32)&gt; {
    <span class="k">for</span> (index, sheet) <span class="k">in</span> sheets.iter().enumerate() {
        <span class="k">for</span> k <span class="k">in</span> &amp;sheet.chunks {
            <span class="k">if</span> row &gt;= k.row &amp;&amp; row &lt; k.row + (k.to - k.from) {
                <span class="k">return</span> Some((index, k.from + (row - k.row)));
            }
        }
    }

    None</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Lines on the GPU: edges as pipes, curves as ribbons.</span></code></pre></div>
<p>Added after the <code>selected_edge: bool,</code> line in <code>struct SegmentLane</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    sheets: Vec&lt;SegSheet&gt;,</code></pre></div>
<p>Added after the <code>selected_edge: false,</code> line in <code>fn new</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            sheets: Vec::new(),</code></pre></div>
<p>Replaces the 11 lines from <code>let ribbons = joined_rows(&amp;up.ribbons, &amp;up.ri…</code> in <code>fn append</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> ribbon_base = <span class="k">self</span>.ribbons.buf.len();
        <span class="k">let</span> <span class="k">mut</span> ribbon_ids = up.ribbon_ids.clone();
        ribbon_ids.resize(up.ribbons.len(), u32::MAX);
        <span class="k">let</span> ribbons = joined_rows(&amp;up.ribbons, &amp;up.ribbon_chains, ribbon_base);
        <span class="k">let</span> ribbons_changed = <span class="k">self</span>.ribbons.buf.append(ctx, &amp;ribbons);

        <span class="k">if</span> <span class="k">self</span>.ribbons.ids.append(ctx, &amp;ribbon_ids) || ribbons_changed {
            <span class="k">self</span>.ribbons.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
        }

        <span class="c">// register the sheet batches</span>
        <span class="k">for</span> d <span class="k">in</span> &amp;up.sheets {
            <span class="k">let</span> Some(ids) = ribbon_ids.get(d.first <span class="k">as</span> usize..(d.first + d.count) <span class="k">as</span> usize) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> chunk = SegChunk {
                from: d.from,
                to: d.from + d.count,
                row: ribbon_base + d.first,
            };
            push_chunk(&amp;<span class="k">mut</span> <span class="k">self</span>.sheets, d.instance, chunk, ids);
        }
    }

    <span class="c">/// Sheet of a GPU ribbon row: (object row, segment index).</span>
    <span class="k">pub</span> <span class="k">fn</span> row_of(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;(u32, u32)&gt; {
        <span class="k">let</span> (index, local) = sheet_of(&amp;<span class="k">self</span>.sheets, row)?;
        Some((<span class="k">self</span>.sheets[index].instance, local))
    }

    <span class="c">/// Source entity of a sheet segment, None off every sheet.</span>
    <span class="k">pub</span> <span class="k">fn</span> source_id(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;u32&gt; {
        <span class="k">let</span> (index, local) = sheet_of(&amp;<span class="k">self</span>.sheets, row)?;
        <span class="k">self</span>.sheets[index].ids.get(local <span class="k">as</span> usize).copied()</code></pre></div>
<p>Added after the <code>self.selected_edge = false;</code> line in <code>fn reset</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.sheets.clear();</code></pre></div>
<p>Added after the <code>self.selected_edge = false;</code> line in <code>fn release</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.sheets = Vec::new();</code></pre></div>
<p>Added after the <code>}</code> line in <code>mod tests</code> of <code>lessons/18/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Sheet batches map GPU rows to segments; out-of-order ones drop.</span>
    #[test]
    <span class="k">fn</span> sheet_chunks_map_global_ribbon_rows_to_segments_and_ids() {
        <span class="k">let</span> <span class="k">mut</span> sheets = Vec::new();
        <span class="k">let</span> chunk = |from, to, row| SegChunk { from, to, row };
        push_chunk(&amp;<span class="k">mut</span> sheets, <span class="s">7</span>, chunk(<span class="s">0</span>, <span class="s">3</span>, <span class="s">10</span>), &amp;[<span class="s">100</span>, <span class="s">101</span>, <span class="s">102</span>]);
        push_chunk(&amp;<span class="k">mut</span> sheets, <span class="s">9</span>, chunk(<span class="s">0</span>, <span class="s">1</span>, <span class="s">13</span>), &amp;[u32::MAX]);
        push_chunk(&amp;<span class="k">mut</span> sheets, <span class="s">7</span>, chunk(<span class="s">3</span>, <span class="s">5</span>, <span class="s">20</span>), &amp;[<span class="s">103</span>, <span class="s">104</span>]);
        push_chunk(&amp;<span class="k">mut</span> sheets, <span class="s">7</span>, chunk(<span class="s">6</span>, <span class="s">8</span>, <span class="s">30</span>), &amp;[<span class="s">9</span>, <span class="s">9</span>]);
        push_chunk(&amp;<span class="k">mut</span> sheets, <span class="s">8</span>, chunk(<span class="s">2</span>, <span class="s">4</span>, <span class="s">40</span>), &amp;[<span class="s">9</span>, <span class="s">9</span>]);
        assert_eq!(sheets.len(), <span class="s">2</span>);
        assert_eq!(sheets[<span class="s">0</span>].resident, <span class="s">5</span>);
        assert_eq!(sheet_of(&amp;sheets, <span class="s">12</span>), Some((<span class="s">0</span>, <span class="s">2</span>)));
        assert_eq!(sheet_of(&amp;sheets, <span class="s">13</span>), Some((<span class="s">1</span>, <span class="s">0</span>)));
        assert_eq!(sheet_of(&amp;sheets, <span class="s">21</span>), Some((<span class="s">0</span>, <span class="s">4</span>)));
        assert_eq!(sheet_of(&amp;sheets, <span class="s">9</span>), None);
        assert_eq!(sheet_of(&amp;sheets, <span class="s">22</span>), None);
        assert_eq!(sheet_of(&amp;sheets, <span class="s">30</span>), None);
        assert_eq!(sheets[<span class="s">0</span>].ids[<span class="s">4</span>], <span class="s">104</span>);
        assert_eq!(sheets[<span class="s">1</span>].ids[<span class="s">0</span>], u32::MAX);
    }</code></pre></div>
<h2 id="step-8-srcappsceners">Step 8 · src/app/scene.rs<a class="anchor" href="#/course/19-sheets#step-8-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene owns source documents and maps their identities to GPU rows.</p>
<p><code>lessons/19/src/app/scene.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from <code>use crate::app::stream::{CloudFields, CloudLod};</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::sheet_query::{EntityMeta, SheetTable};
<span class="k">use</span> <span class="k">crate</span>::app::stream::{CloudFields, CloudLod, SheetFields};
<span class="k">use</span> <span class="k">crate</span>::app::walk::bounds::{Baselines, file_extent, is_planar, mark_sheet};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::{StreamRows, StreamSlice, walk_stream_slice};
<span class="k">use</span> <span class="k">crate</span>::app::walk::mesh::Lap;
<span class="k">use</span> <span class="k">crate</span>::app::walk::sheet::{SheetRows, SheetSlice, walk_sheet_slice};</code></pre></div>
<p>Replaces the 7 lines from <code>#[derive(Clone, Debug)]</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A streamed sheet's first slice.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetInit {
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> url: String,
    <span class="k">pub</span> meta_url: Option&lt;String&gt;, <span class="c">// its entity side table</span>
    <span class="k">pub</span> place: Xform,
    <span class="k">pub</span> rows: SheetRows, <span class="c">// the first segments</span>
    <span class="k">pub</span> fields: SheetFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> resident: u32, <span class="c">// segments in this slice</span>
}

<span class="c">/// A sheet's slot in the scene.</span>
<span class="k">pub</span> <span class="k">struct</span> SheetBatch {
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> url: String,
    <span class="k">pub</span> meta_url: Option&lt;String&gt;, <span class="c">// its entity side table</span>
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> fields: SheetFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> place: Xform,
    <span class="k">pub</span> done_to: u32,
    <span class="k">pub</span> total: u32, <span class="c">// segments in the file</span>
    <span class="k">pub</span> resolved: Option&lt;(u32, EntityMeta)&gt;, <span class="c">// entity the last pick found</span>
    <span class="k">pub</span> table: Option&lt;SheetTable&gt;, <span class="c">// side table head, read once</span>
}

<span class="c">/// What a pick landed on.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Picked {
    <span class="k">pub</span> doc: String, <span class="c">// document name</span>
    <span class="k">pub</span> guid: String,
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> point: Option&lt;PickedPoint&gt;,
    <span class="k">pub</span> entity: Option&lt;u32&gt;, <span class="c">// the entity id, for a sheet</span></code></pre></div>
<p>Added after the <code>pub streamed: Vec&lt;StreamedCloud&gt;,</code> line in <code>struct Scene</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> sheets: Vec&lt;SheetBatch&gt;,</code></pre></div>
<p>Added after the <code>streamed: Vec::new(),</code> line in <code>fn new</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            sheets: Vec::new(),</code></pre></div>
<p>Replaces the 26 lines from <code>self.tables = Upload::default();</code> in <code>fn clear</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hidden.clear();
        <span class="k">self</span>.reset_rows();
        gpu.release();
    }

    <span class="c">/// Forget every row.</span>
    <span class="k">fn</span> reset_rows(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.tables = Upload::default();
        <span class="k">self</span>.streamed.clear();
        <span class="k">self</span>.sheets.clear();
        <span class="k">self</span>.order.clear();
        <span class="k">self</span>.owners.clear();
        <span class="k">self</span>.edge_sources.clear();
        <span class="k">self</span>.ribbon_ranges.clear();
        <span class="k">self</span>.guid_to_row.clear();
        <span class="k">self</span>.selected = None;
        <span class="k">self</span>.bases = Bases::default();
    }

    <span class="c">/// Walk every document again and upload from scratch.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">let</span> docs = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.docs);
        <span class="k">let</span> texts = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.texts);
        <span class="k">self</span>.reset_rows();</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl Scene</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Add a streamed sheet from its first slice; returns its slot.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_sheet(&amp;<span class="k">mut</span> <span class="k">self</span>, init: SheetInit, gpu: &amp;<span class="k">mut</span> Gpu) -&gt; usize {
        <span class="k">let</span> SheetInit {
            name,
            url,
            meta_url,
            place,
            rows,
            fields,
            resident,
        } = init;
        <span class="k">let</span> total = fields.count;
        <span class="k">let</span> row = <span class="k">self</span>.push_row(
            <span class="k">self</span>.docs.len(),
            &amp;format!(&quot;<span class="s">sheet:</span>{<span class="s">url</span>}&quot;),
            place.clone(),
            Instance::FLAG_SHEET,
        );
        <span class="k">let</span> slice = SheetSlice { rows, from: <span class="s">0</span>, row };
        <span class="k">let</span> bounds = walk_sheet_slice(&amp;<span class="k">mut</span> <span class="k">self</span>.tables.seg, &amp;slice);
        <span class="k">let</span> o = <span class="k">self</span>.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        <span class="k">self</span>.tables.bounds.union_with(&amp;bounds.transformed(&amp;place));
        <span class="k">self</span>.upload_to(gpu);

        <span class="k">let</span> model = place.clone();
        <span class="k">self</span>.docs.push(FileDoc {
            name: name.clone(),
            place,
            session: Rc::new(Session::new(&amp;name)),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">true</span>,
        });
        <span class="k">self</span>.sheets.push(SheetBatch {
            name,
            url,
            meta_url,
            row,
            fields,
            place: model,
            done_to: resident,
            total,
            resolved: None,
            table: None,
        });
        <span class="k">self</span>.sheets.len() - <span class="s">1</span>
    }

    <span class="c">/// Add the next slice of sheet \`idx\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> extend_sheet(&amp;<span class="k">mut</span> <span class="k">self</span>, idx: usize, rows: SheetRows, to: u32, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">let</span> Some(sheet) = <span class="k">self</span>.sheets.get(idx) <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> to &lt;= sheet.done_to {
            <span class="k">return</span>;
        }

        <span class="k">let</span> place = sheet.place.clone();
        <span class="k">let</span> slice = SheetSlice {
            rows,
            from: sheet.done_to,
            row: sheet.row,
        };
        <span class="k">let</span> bounds = walk_sheet_slice(&amp;<span class="k">mut</span> <span class="k">self</span>.tables.seg, &amp;slice);
        <span class="k">self</span>.tables.bounds.union_with(&amp;bounds.transformed(&amp;place));
        <span class="k">self</span>.sheets[idx].done_to = to;
        <span class="k">self</span>.upload_to(gpu);
    }

    <span class="c">/// The sheet slot on object row \`row\`, if that row is a sheet.</span>
    <span class="k">pub</span> <span class="k">fn</span> sheet_slot(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;usize&gt; {
        <span class="k">for</span> (slot, sheet) <span class="k">in</span> <span class="k">self</span>.sheets.iter().enumerate() {
            <span class="k">if</span> sheet.row == row {
                <span class="k">return</span> Some(slot);
            }
        }

        None
    }

    <span class="c">/// The sheet on object row \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> sheet_at(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;&amp;SheetBatch&gt; {
        <span class="k">self</span>.sheets.get(<span class="k">self</span>.sheet_slot(row)?)
    }

    <span class="c">/// What a GPU pick landed on.</span></code></pre></div>
<p>Added after the <code>}</code> line in <code>fn resolve</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> entity = None;
        <span class="c">// bit 31 set: the sub id is a ribbon row</span>
        <span class="k">let</span> ribbon = pick.sub &amp; <span class="s">0x7fff_ffff</span>;

        <span class="k">if</span> pick.sub &amp; <span class="s">0x8000_0000</span> != <span class="s">0</span>
            &amp;&amp; <span class="k">self</span>.sheet_at(pick.row).is_some()
            &amp;&amp; <span class="k">let</span> Some((parent, _)) = gpu.segments.row_of(ribbon)
            &amp;&amp; parent == pick.row
        {
            entity = gpu.segments.source_id(ribbon).filter(|id| *id != u32::MAX);
        }</code></pre></div>
<p>Added after the <code>point,</code> line in <code>fn resolve</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            entity,</code></pre></div>
<p>Added after the <code>return &amp;text.label.text;</code> line in <code>fn object_name</code> of <code>lessons/18/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The row's name, or its type when unnamed.</span>
    <span class="k">pub</span> <span class="k">fn</span> object_name(&amp;<span class="k">self</span>, row: u32) -&gt; &amp;str {
        <span class="k">if</span> <span class="k">let</span> Some(text) = <span class="k">self</span>.text_at(row) {
            <span class="k">return</span> &amp;text.label.text;
        }

        <span class="k">if</span> <span class="k">let</span> Some(sheet) = <span class="k">self</span>.sheet_at(row) {
            <span class="k">return</span> <span class="k">match</span> &amp;sheet.resolved {
                Some((_, meta)) <span class="k">if</span> !meta.name.trim().is_empty() =&gt; &amp;meta.name,
                Some((_, meta)) <span class="k">if</span> !meta.kind.trim().is_empty() =&gt; &amp;meta.kind,
                _ =&gt; &amp;sheet.name,
            };
        }</code></pre></div>
<h2 id="step-9-srcappsheet_queryrs">Step 9 · src/app/sheet_query.rs<a class="anchor" href="#/course/19-sheets#step-9-srcappsheet_queryrs" aria-label="Link to this section">#</a></h2>
<p>Sheet queries resolve a picked segment to its source entity.</p>
<p><code>lessons/19/src/app/sheet_query.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Answers what a click hit on a sheet, the flat drawing page laid over the model.</span>
<span class="k">use</span> serde::Deserialize;
<span class="k">use</span> std::cell::Cell;
<span class="k">use</span> std::rc::Rc;

<span class="c">/// Bytes of the table head: \`SHM1\` then a u32 record count.</span>
<span class="k">pub</span> <span class="k">const</span> HEAD_BYTES: u64 = <span class="s">8</span>;

<span class="c">/// Bytes of one record: u64 offset, u64 length.</span>
<span class="k">pub</span> <span class="k">const</span> RECORD_BYTES: u64 = <span class="s">16</span>;

<span class="c">/// Largest entity record accepted.</span>
<span class="k">pub</span> <span class="k">const</span> MAX_BLOB: u64 = <span class="s">64</span> * <span class="s">1024</span>;

<span class="c">/// One sheet entity's identity, from its JSON record.</span>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> EntityMeta {
    #[serde(default)]
    <span class="k">pub</span> guid: String, <span class="c">// source object id</span>
    #[serde(default)]
    <span class="k">pub</span> name: String,
    #[serde(default)]
    <span class="k">pub</span> kind: String, <span class="c">// wall, door, ...</span>
    #[serde(default)]
    <span class="k">pub</span> width: f32,
    #[serde(default)]
    <span class="k">pub</span> color: Vec&lt;f32&gt;,
}

<span class="c">/// The table head, cached per sheet.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> SheetTable {
    <span class="k">pub</span> count: u32, <span class="c">// records in the table</span>
    <span class="k">pub</span> revision: Option&lt;String&gt;, <span class="c">// ETag every read must match</span>
}

<span class="c">/// The record count from a table head.</span>
<span class="k">pub</span> <span class="k">fn</span> table_count(raw: &amp;[u8]) -&gt; Result&lt;u32, String&gt; {
    <span class="k">if</span> raw.len() != HEAD_BYTES <span class="k">as</span> usize || &amp;raw[..<span class="s">4</span>] != <span class="s">b</span>&quot;<span class="s">SHM1</span>&quot; {
        <span class="k">return</span> Err(&quot;<span class="s">Side table head is not SHM1</span>&quot;.to_string());
    }

    Ok(u32::from_le_bytes(
        raw[<span class="s">4</span>..<span class="s">8</span>].try_into().expect(&quot;<span class="s">exact head checked above</span>&quot;),
    ))
}

<span class="c">/// One record as (offset, length).</span>
<span class="k">pub</span> <span class="k">fn</span> record(raw: &amp;[u8]) -&gt; Result&lt;(u64, u64), String&gt; {
    <span class="k">if</span> raw.len() != RECORD_BYTES <span class="k">as</span> usize {
        <span class="k">return</span> Err(&quot;<span class="s">Side table record must contain exactly 16 bytes</span>&quot;.to_string());
    }

    Ok((
        u64::from_le_bytes(raw[..<span class="s">8</span>].try_into().expect(&quot;<span class="s">exact record checked above</span>&quot;)),
        u64::from_le_bytes(raw[<span class="s">8</span>..].try_into().expect(&quot;<span class="s">exact record checked above</span>&quot;)),
    ))
}</code></pre></div>
<p><code>lessons/19/src/app/sheet_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Byte position of record \`id\`; \`record_at(count)\` starts the blobs.</span>
<span class="k">pub</span> <span class="k">fn</span> record_at(id: u32) -&gt; u64 {
    HEAD_BYTES + RECORD_BYTES * u64::from(id)
}

<span class="c">/// An entity from its JSON; missing keys default.</span>
<span class="k">pub</span> <span class="k">fn</span> entity_from(raw: &amp;[u8]) -&gt; Result&lt;EntityMeta, String&gt; {
    serde_json::from_slice(raw).map_err(|error| format!(&quot;<span class="s">Entity record is not valid JSON: </span>{<span class="s">error</span>}&quot;))
}

<span class="c">/// One entity lookup in flight.</span>
<span class="k">pub</span> <span class="k">struct</span> Query {
    <span class="k">pub</span> id: u64, <span class="c">// lookup number</span>
    <span class="k">pub</span> row: u32, <span class="c">// the sheet's object row</span>
    <span class="k">pub</span> entity: u32, <span class="c">// entity index in the sheet</span>
    <span class="k">pub</span> cancelled: Rc&lt;Cell&lt;bool&gt;&gt;, <span class="c">// set when a newer lookup replaces this</span>
}

<span class="k">impl</span> Query {
    <span class="c">/// A new lookup.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(id: u64, row: u32, entity: u32) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            id,
            row,
            entity,
            cancelled: Rc::new(Cell::new(<span class="s">false</span>)),
        }
    }
}

<span class="k">impl</span> Drop <span class="k">for</span> Query {
    <span class="c">/// Cancel the lookup.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.cancelled.set(<span class="s">true</span>);
    }
}

<span class="c">/// The answer to one lookup.</span>
<span class="k">pub</span> <span class="k">struct</span> Resolved {
    <span class="k">pub</span> query: u64, <span class="c">// which lookup</span>
    <span class="k">pub</span> result: Result&lt;(EntityMeta, SheetTable), String&gt;,
}</code></pre></div>
<p><code>lessons/19/src/app/sheet_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">mod</span> web {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::loader::post;

    <span class="k">use</span> <span class="k">crate</span>::app::fetch::fetch_range <span class="k">as</span> range;

    <span class="c">/// Start reading one entity; the answer arrives as a message.</span>
    <span class="k">pub</span> <span class="k">fn</span> fetch_entity(query: &amp;Query, url: String, table: Option&lt;SheetTable&gt;, entities: u32) {
        wasm_bindgen_futures::spawn_local(post_entity(
            query.id,
            query.entity,
            query.cancelled.clone(),
            url,
            table,
            entities,
        ));
    }

    <span class="c">/// Read the entity and post the answer unless cancelled.</span>
    <span class="k">async</span> <span class="k">fn</span> post_entity(
        query: u64,
        entity: u32,
        cancelled: Rc&lt;Cell&lt;bool&gt;&gt;,
        url: String,
        table: Option&lt;SheetTable&gt;,
        entities: u32,</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/19/src/app/sheet_query.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    ) {
        <span class="k">let</span> result = read_entity(&amp;url, entity, table, entities, &amp;cancelled).<span class="k">await</span>;

        <span class="k">if</span> !cancelled.get() {
            post(<span class="k">crate</span>::Msg::SheetEntity(Resolved { query, result }));
        }
    }

    <span class="c">/// Read the head if unknown, then the record, then the blob.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> read_entity(
        url: &amp;str,
        id: u32,
        table: Option&lt;SheetTable&gt;,
        entities: u32,
        cancelled: &amp;Cell&lt;bool&gt;,
    ) -&gt; Result&lt;(EntityMeta, SheetTable), String&gt; {
        <span class="k">let</span> table = <span class="k">match</span> table {
            Some(table) =&gt; table,
            None =&gt; {
                <span class="k">let</span> (raw, revision) = range(url, <span class="s">0</span>, HEAD_BYTES, &amp;None).<span class="k">await</span>?;
                <span class="k">let</span> count = table_count(&amp;raw)?;

                <span class="k">if</span> count != entities {
                    <span class="k">return</span> Err(format!(
                        &quot;<span class="s">Side table holds </span>{<span class="s">count</span>}<span class="s"> records; the sheet says </span>{<span class="s">entities</span>}&quot;
                    ));
                }

                SheetTable { count, revision }
            }
        };

        <span class="k">if</span> cancelled.get() {
            <span class="k">return</span> Err(&quot;<span class="s">Entity lookup cancelled</span>&quot;.to_string());
        }

        <span class="k">if</span> id &gt;= table.count {
            <span class="k">return</span> Err(format!(
                &quot;<span class="s">Entity </span>{<span class="s">id</span>}<span class="s"> is outside the </span>{}<span class="s">-record side table</span>&quot;,
                table.count
            ));
        }

        <span class="k">let</span> (raw, _) = range(url, record_at(id), RECORD_BYTES, &amp;table.revision).<span class="k">await</span>?;

        <span class="k">if</span> cancelled.get() {
            <span class="k">return</span> Err(&quot;<span class="s">Entity lookup cancelled</span>&quot;.to_string());
        }

        <span class="k">let</span> (offset, length) = record(&amp;raw)?;

        <span class="k">if</span> length &gt; MAX_BLOB {
            <span class="k">return</span> Err(format!(
                &quot;<span class="s">Entity </span>{<span class="s">id</span>}<span class="s"> record is </span>{<span class="s">length</span>}<span class="s"> bytes, over the </span>{<span class="s">MAX_BLOB</span>}<span class="s"> limit</span>&quot;
            ));
        }

        <span class="k">let</span> at = record_at(table.count)
            .checked_add(offset)
            .ok_or(&quot;<span class="s">Entity record offset overflows</span>&quot;)?;
        <span class="k">let</span> (blob, _) = range(url, at, length, &amp;table.revision).<span class="k">await</span>?;
        Ok((entity_from(&amp;blob)?, table))
    }
}
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">use</span> web::fetch_entity;

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Build a side table from JSON blobs.</span>
    <span class="k">fn</span> table(blobs: &amp;[&amp;str]) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = <span class="s">b</span>&quot;<span class="s">SHM1</span>&quot;.to_vec();
        out.extend((blobs.len() <span class="k">as</span> u32).to_le_bytes());
        <span class="k">let</span> <span class="k">mut</span> offset = <span class="s">0u64</span>;

        <span class="k">for</span> blob <span class="k">in</span> blobs {
            out.extend(offset.to_le_bytes());
            out.extend((blob.len() <span class="k">as</span> u64).to_le_bytes());
            offset += blob.len() <span class="k">as</span> u64;
        }

        <span class="k">for</span> blob <span class="k">in</span> blobs {
            out.extend(blob.as_bytes());
        }

        out
    }

    <span class="c">/// Head, records and blobs parse; junk is refused.</span>
    #[test]
    <span class="k">fn</span> side_table_head_records_and_blobs_parse_from_a_hand_built_buffer() {
        <span class="k">let</span> raw = table(&amp;[
            <span class="s">r</span>#&quot;<span class="s">{&quot;guid&quot;:&quot;g0&quot;,&quot;name&quot;:&quot;Wall A&quot;,&quot;kind&quot;:&quot;wall&quot;,&quot;width&quot;:0.35,&quot;color&quot;:[0,0,0,255]}</span>&quot;#,
            <span class="s">r</span>#&quot;<span class="s">{&quot;name&quot;:&quot;Door&quot;,&quot;kind&quot;:&quot;door&quot;,&quot;extra&quot;:true}</span>&quot;#,
        ]);
        <span class="k">let</span> count = table_count(&amp;raw[..HEAD_BYTES <span class="k">as</span> usize]).unwrap();
        assert_eq!(count, <span class="s">2</span>);
        <span class="k">let</span> blobs = record_at(count) <span class="k">as</span> usize;
        <span class="k">let</span> at = record_at(<span class="s">1</span>) <span class="k">as</span> usize;
        <span class="k">let</span> (offset, length) = record(&amp;raw[at..at + RECORD_BYTES <span class="k">as</span> usize]).unwrap();
        <span class="k">let</span> door = entity_from(&amp;raw[blobs + offset <span class="k">as</span> usize..][..length <span class="k">as</span> usize]).unwrap();
        assert_eq!(door.name, &quot;<span class="s">Door</span>&quot;);
        assert_eq!(door.kind, &quot;<span class="s">door</span>&quot;);
        assert_eq!(door.guid, &quot;&quot;);
        <span class="k">let</span> at = record_at(<span class="s">0</span>) <span class="k">as</span> usize;
        <span class="k">let</span> (offset, length) = record(&amp;raw[at..at + RECORD_BYTES <span class="k">as</span> usize]).unwrap();
        <span class="k">let</span> wall = entity_from(&amp;raw[blobs + offset <span class="k">as</span> usize..][..length <span class="k">as</span> usize]).unwrap();
        assert_eq!(wall.guid, &quot;<span class="s">g0</span>&quot;);
        assert_eq!(wall.width, <span class="s">0</span>.<span class="s">35</span>);
        assert_eq!(wall.color, [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">255</span>.<span class="s">0</span>]);
        assert!(table_count(<span class="s">b</span>&quot;<span class="s">SHM2\\0\\0\\0\\0</span>&quot;).is_err());
        assert!(table_count(&amp;raw[..<span class="s">7</span>]).is_err());
        assert!(record(&amp;raw[..<span class="s">15</span>]).is_err());
        assert!(entity_from(<span class="s">b</span>&quot;<span class="s">{</span>&quot;).is_err());
    }

    <span class="c">/// Dropping a query retires its token, so a late answer is ignored.</span>
    #[test]
    <span class="k">fn</span> dropping_a_query_cancels_its_token() {
        <span class="k">let</span> query = Query::new(<span class="s">3</span>, <span class="s">1</span>, <span class="s">9</span>);
        <span class="k">let</span> token = query.cancelled.clone();
        drop(query);
        assert!(token.get());
    }
}</code></pre></div>
<h2 id="step-10-srcappmodrs">Step 10 · src/app/mod.rs<a class="anchor" href="#/course/19-sheets#step-10-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The application module connects source loading and interaction helpers.</p>
<p><code>lessons/19/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod selection;</code> line of <code>lessons/18/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> sheet_query;</code></pre></div>
<h2 id="step-11-srcstaters">Step 11 · src/state.rs<a class="anchor" href="#/course/19-sheets#step-11-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/19/src/state.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from <code>use crate::app::scene::{FileDoc, Scene, Strea…</code> of <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! One place for what the viewer knows between frames, so no pass has to reach into another pass's data.</span>
<span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, Scene, SheetInit, StreamedInit};
<span class="k">use</span> <span class="k">crate</span>::app::selection::{ControlId, Controls, SelectionMode};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::StreamRows;
<span class="k">use</span> <span class="k">crate</span>::app::walk::encode::FACING_UNKNOWN;
<span class="k">use</span> <span class="k">crate</span>::app::walk::sheet::SheetRows;</code></pre></div>
<p>Added after the <code>mod cloud_query;</code> line of <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> sheet_query;</code></pre></div>
<p>Added after the <code>query_generation: u64,</code> line in <code>struct State</code> of <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    sheet_query: Option&lt;<span class="k">crate</span>::app::sheet_query::Query&gt;, <span class="c">// a sheet pick in flight</span>
    sheet_generation: u64, <span class="c">// counts sheet queries, old answers dropped</span></code></pre></div>
<p>Added after the <code>query_generation: 0,</code> line in <code>fn new</code> of <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            sheet_query: None,
            sheet_generation: <span class="s">0</span>,</code></pre></div>
<p>Replaces <code>fn clear</code> in <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Start a streamed sheet; returns its slot.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_sheet(&amp;<span class="k">mut</span> <span class="k">self</span>, init: SheetInit) -&gt; usize {
        <span class="k">let</span> idx = <span class="k">self</span>.scene.add_sheet(init, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        <span class="k">self</span>.touch();
        idx
    }

    <span class="c">/// Add more segments to sheet \`idx\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> extend_sheet(&amp;<span class="k">mut</span> <span class="k">self</span>, idx: usize, rows: SheetRows, to: u32) {
        <span class="k">self</span>.scene.extend_sheet(idx, rows, to, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        log::info!(
            &quot;<span class="s">sheet slice: </span>{<span class="s">to</span>}<span class="s"> segments resident | heap </span>{<span class="s">:.0</span>}<span class="s"> MB</span>&quot;,
            heap_mb()
        );
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Remove every document; camera and GPU stay.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.sheet_query = None;</code></pre></div>
<p>Replaces this block of <code>lessons/18/src/state.rs</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.gpu.arena.source_faces.select(&amp;<span class="k">self</span>.gpu.ctx, None);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.sheet_query = None;</code></pre></div>
<p>Added after the <code>None =&gt; log::info!(&quot;pick: &#39;{}&#39; {} row {}&quot;, hi…</code> line in <code>fn apply_pick</code> of <code>lessons/18/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="c">// a sheet entity, not an object</span>
                <span class="k">if</span> <span class="k">let</span> Some(entity) = hit.entity {
                    <span class="k">self</span>.apply_sheet_pick(hit.row, entity);
                    <span class="k">return</span>;
                }</code></pre></div>
<h2 id="step-12-srcstatesheet_queryrs">Step 12 · src/state/sheet_query.rs<a class="anchor" href="#/course/19-sheets#step-12-srcstatesheet_queryrs" aria-label="Link to this section">#</a></h2>
<p>State associates sheet replies with the current selection.</p>
<p><code>lessons/19/src/state/sheet_query.rs</code> · 83 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
<span class="k">use</span> <span class="k">crate</span>::app::selection::SelectionMode;
<span class="k">use</span> <span class="k">crate</span>::app::sheet_query::{Query, Resolved};

<span class="k">impl</span> State {
    <span class="c">/// Select one sheet entity; the same one again clears it.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> apply_sheet_pick(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, entity: u32) {
        <span class="k">let</span> Some(slot) = <span class="k">self</span>.scene.sheet_slot(row) <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="c">// already selected: clear</span>
        <span class="k">if</span> <span class="k">self</span>.selection
            == (SelectionMode::Edge {
                parent: row,
                edge: entity,
            })
        {
            <span class="k">self</span>.select(None);
            <span class="k">self</span>.status(&quot;&quot;);
            <span class="k">return</span>;
        }

        <span class="k">self</span>.select(Some(row));
        <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
        <span class="k">self</span>.selection.select_edge(row, entity);
        <span class="k">self</span>.gpu
            .segments
            .set_edge(&amp;<span class="k">self</span>.gpu.ctx, Some((row, entity)));
        <span class="k">self</span>.scene.sheets[slot].resolved = None;
        <span class="k">self</span>.sheet_generation = <span class="k">self</span>.sheet_generation.wrapping_add(<span class="s">1</span>); <span class="c">// new query id</span>
        <span class="k">let</span> query = Query::new(<span class="k">self</span>.sheet_generation, row, entity);

        <span class="c">// fetch name and kind from the side table, if any</span>
        <span class="k">match</span> <span class="k">self</span>.scene.sheets[slot].meta_url.clone() {
            None =&gt; <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Selected entity </span>{<span class="s">entity</span>}&quot;)),
            Some(url) =&gt; {
                <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Selected entity </span>{<span class="s">entity</span>}<span class="s">, fetching…</span>&quot;));
                #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
                <span class="k">crate</span>::app::sheet_query::fetch_entity(
                    &amp;query,
                    url,
                    <span class="k">self</span>.scene.sheets[slot].table.clone(),
                    <span class="k">self</span>.scene.sheets[slot].fields.entities,
                );
                #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
                <span class="k">let</span> _ = url;
            }
        }

        <span class="k">self</span>.sheet_query = Some(query);
        <span class="k">self</span>.touch();
    }

    <span class="c">/// The entity's name and kind arrived: show them.</span>
    <span class="k">pub</span> <span class="k">fn</span> sheet_entity(&amp;<span class="k">mut</span> <span class="k">self</span>, resolved: Resolved) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.sheet_query.as_ref() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="c">// an old query's answer</span>
        <span class="k">if</span> query.id != resolved.query || query.cancelled.get() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> query = <span class="k">self</span>.sheet_query.take().unwrap();
        <span class="k">let</span> Some(slot) = <span class="k">self</span>.scene.sheet_slot(query.row) <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">match</span> resolved.result {
            Ok((meta, table)) =&gt; {
                <span class="k">self</span>.status(&amp;format!(
                    &quot;<span class="s">Selected entity </span>{}<span class="s">: </span>{}<span class="s"> (</span>{}<span class="s">)</span>&quot;,
                    query.entity, meta.name, meta.kind
                ));
                <span class="k">let</span> sheet = &amp;<span class="k">mut</span> <span class="k">self</span>.scene.sheets[slot];
                sheet.table = Some(table);
                sheet.resolved = Some((query.entity, meta));
                <span class="k">self</span>.update_label();
                <span class="k">self</span>.touch();
            }
            Err(error) =&gt; <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Entity </span>{}<span class="s">: </span>{<span class="s">error</span>}&quot;, query.entity)),
        }
    }
}</code></pre></div>
<h2 id="step-13-srcappinspectionrs">Step 13 · src/app/inspection.rs<a class="anchor" href="#/course/19-sheets#step-13-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Inspection reports retained resources and source information.</p>
<p><code>lessons/19/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the <code>&quot;controls&quot;: state.inspected_controls(),</code> line in <code>fn publish</code> of <code>lessons/18/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">sheet_entity</span>&quot;: sheet_entity(state),</code></pre></div>
<p>Added after the <code>Some((document, guid.to_string()))</code> line in <code>fn selected_identity</code> of <code>lessons/18/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The picked entity of the selected sheet, if known.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> sheet_entity(state: &amp;State) -&gt; Option&lt;serde_json::Value&gt; {
    <span class="k">let</span> (id, meta) = state
        .scene
        .sheet_at(state.scene.selected?)?
        .resolved
        .as_ref()?;
    Some(serde_json::json!({&quot;<span class="s">id</span>&quot;: id, &quot;<span class="s">guid</span>&quot;: meta.guid, &quot;<span class="s">name</span>&quot;: meta.name, &quot;<span class="s">kind</span>&quot;: meta.kind}))
}

<span class="c">/// Every drawn text label, as JSON.</span></code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/19/</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/19-sheets#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/19/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: Two vector sheets stream into top view and clicking a segment resolves its source entity; status: <strong>Selected entity … fetching…</strong>.</p>
<p><a href="/session/docs/course/docs/screenshots/19-sheets-overview.png"><img src="/session/docs/course/docs/screenshots/19-sheets-overview.png" alt="Full viewer result for 19 sheets" loading="lazy" decoding="async"></a></p>
<p>If it fails:</p>
<ul>
<li>No sheets arrive: the manifest or data server does not provide the sheet stream.</li>
<li>A picked entity has the wrong name: a display segment index replaces its source entity ID.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/19-sheets#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/19/src/
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
│   │   ├── mod.rs  ~
│   │   ├── points.rs
│   │   └── sheet.rs  +
│   ├── cloud_query.rs
│   ├── decode.rs
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── input.rs
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── live.rs
│   ├── loader.rs  ~
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs  +
│   ├── stream.rs  ~
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
│   │   ├── segments.rs  ~
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
│   ├── sheet_query.rs  +
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/19/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/19-sheets#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/20-history">20 · The document: undo, redo and save</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/19-sheets#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Open <code>?scene=view_sheets&amp;inspect=1</code>, press <strong>5</strong> then <strong>F</strong>: both drawings fit the view.</p>
<p><a href="/session/docs/course/docs/screenshots/19-sheets-overview.png"><img src="/session/docs/course/docs/screenshots/19-sheets-overview.png" alt="Full viewer result for 19 sheets" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-session_protosheetproto",text:"Step 1 · session_proto/sheet.proto"},{level:2,id:"step-2-srcappstreamrs",text:"Step 2 · src/app/stream.rs"},{level:2,id:"step-3-srcapploaderrs",text:"Step 3 · src/app/loader.rs"},{level:2,id:"step-4-srclibrs",text:"Step 4 · src/lib.rs"},{level:2,id:"step-5-srcappwalksheetrs",text:"Step 5 · src/app/walk/sheet.rs"},{level:2,id:"step-6-srcappwalkmodrs",text:"Step 6 · src/app/walk/mod.rs"},{level:2,id:"step-7-srcenginegpusegmentsrs",text:"Step 7 · src/engine/gpu/segments.rs"},{level:2,id:"step-8-srcappsceners",text:"Step 8 · src/app/scene.rs"},{level:2,id:"step-9-srcappsheet_queryrs",text:"Step 9 · src/app/sheet_query.rs"},{level:2,id:"step-10-srcappmodrs",text:"Step 10 · src/app/mod.rs"},{level:2,id:"step-11-srcstaters",text:"Step 11 · src/state.rs"},{level:2,id:"step-12-srcstatesheet_queryrs",text:"Step 12 · src/state/sheet_query.rs"},{level:2,id:"step-13-srcappinspectionrs",text:"Step 13 · src/app/inspection.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
