# 14 · Loading scenes

## You are building

```mermaid
flowchart TB
    U["URL<br/>?scene= · path · localhost"] -- "route.rs" --> R["SceneRoute<br/>manifest + base"]
    R -- "fetch_bytes" --> M["Manifest::parse<br/>items · texts"]
    M -- "per item" --> F["fetch_bytes(.pb)"]
    F -- "validate → decode" --> S["Rc&lt;Session&gt;"]
    S -- "generation still current?" --> P["PendingDocument<br/>staged in manifest order"]
    P -- "Msg::File / StreamedCloud / Texts / Fit" --> A["App → State → Scene"]
    L["LiveSource<br/>ETag polls"] -. "changed files" .-> P
```

## Starting point

- Checkpoint 13: the shell loads one bundled fixture from `loader.rs`; there is no manifest, no network and no replacement.
- This lesson installs the production path: route → manifest → validate → decode → staged replacement, plus the live source that watches a published manifest.

## Step 1 · The manifest is placement, not geometry

- A manifest lists files and where each sits (`at`, `xform`, or the auto-grid); the geometry stays in `.pb` files, so a placement edit never re-uploads geometry.
- `parse` accepts YAML, JSON and TOML with one set of semantics and rejects non-finite or non-affine transforms before anything is fetched.
- `TextItem` is a fixed world-plane label authored in the manifest; its frame must be unit and orthogonal, checked here rather than in a renderer.

```mermaid
flowchart LR
    F["yaml · json · toml"] -- "Manifest::parse" --> M["Manifest"] --> I["Item · at · xform"]
    M --> T["TextItem"]
    style M fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/manifest.rs type lines=1-57 -->

<!-- file: 14 session_viewer/src/app/manifest.rs type lines=58-146 -->

<!-- file: 14 session_viewer/src/app/manifest.rs type lines=147-217 -->

Parser unit tests, part of the file:

<!-- file: 14 session_viewer/src/app/manifest.rs copy lines=218-347 -->

## Step 2 · Validate serialized counts before the kernel allocates

- A hostile `cv_count` would make a kernel constructor allocate from a declared number; every count is checked against the actual storage length first.
- `session` walks a decoded protobuf; `retained` covers the JSON path, which has no protobuf constructors; `json` checks declared NURBS counts before serde builds objects.

```mermaid
flowchart LR
    P["decoded protobuf"] -- "validate::session" --> O["counts ≤ storage"]
    J["JSON document"] -- "validate::json" --> O
    style O fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/validate.rs copy lines=1-147 -->

<!-- file: 14 session_viewer/src/app/validate.rs type lines=148-232 -->

<!-- file: 14 session_viewer/src/app/validate.rs copy lines=233-473 -->

## Step 3 · Decode without freezing the page

- prost decodes the whole message in one call; converting objects into kernel types is the slow part, so `Pacer` yields to the browser every `CHUNK` objects through `next_tick`.
- The bytes are taken by value and dropped right after prost is done, before the conversion loop starts.

```mermaid
flowchart LR
    B["bytes"] -- "prost" --> M["message"] -- "Pacer::tick" --> K["kernel objects"]
    style M fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/decode.rs type lines=1-60 -->

<!-- file: 14 session_viewer/src/app/decode.rs type lines=61-149 -->

## Step 4 · The URL decides the route

- Three routes: a named scene (`?scene=` or the last path segment) from the bucket, the local manifest on a dev server, and no route at all on a deployed page, which hands over to the live source.
- `?data=` overrides where `.pb` files come from; `query_scene` refuses `..`, absolute paths and schemes so a manifest name stays inside one tree.

```mermaid
flowchart LR
    U["?scene= · path"] -- "scene_route" --> R["SceneRoute"] --> S["bucket · local · live"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/route.rs type -->

## Step 5 · A live source polls with ETags

- An idle poll must stay cheap: every file is re-read with `If-None-Match`, so an unchanged file answers `304` and is never downloaded or decoded again.
- A relay message (`EventSource`) only raises a flag that says "look now"; the conditional reads still decide what changed.
- `Notify` owns its closure handle and detaches it in `Drop`; nothing is leaked with `forget()`.

```mermaid
flowchart TB
    E["EventSource"] --> N["Notify flag"]
    N --> C["LiveSource::check"]
    C -- "If-None-Match" --> R["read: Changed · Same"]
    style C fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/live.rs type lines=1-90 -->

- `from_query` turns the page off, on, or onto a custom manifest; a named scene or a local dev page never watches the bucket.

<!-- file: 14 session_viewer/src/app/live.rs type lines=91-190 -->

- `read` returns `Changed`, `Same` or `Failed`; a server without ETags falls back to hashing the body.
- A manifest inside the bucket names its files from the bucket root; any other manifest names them from its own folder.

<!-- file: 14 session_viewer/src/app/live.rs type lines=191-256 -->

- `check` is one tick: nothing happens unless the relay flagged or the poll interval is due; a replacement with any unreadable file returns `None` and the last valid scene stays.

<!-- file: 14 session_viewer/src/app/live.rs type lines=257-354 -->

<!-- file: 14 session_viewer/src/app/live.rs type lines=355-441 -->

<!-- check: 14 -->

## Step 6 · Stage a replacement, then swap it in whole

![Two request generations in flight: the older one is dropped, the newer one is staged in manifest order and swapped in whole while the previous scene stays on screen.](illustrations/loading.svg)

- A slow old response arriving last must not replace a newer scene, so every load carries a generation and `stale_load` is checked after each await.
- Network responses finish in any order; `PendingDocument` keeps manifest order, and `clear_scene` runs only after every item succeeded.
- Streaming clouds keep their budget: `stream_prefix` opens a large file by range and `stream_rest` continues a slice at a time until its scene is cleared.
- Whole files have a budget too: `scene_budget_bytes` is `?budget=<MB>` or 16 MB per GB of `navigator.deviceMemory`, 64 MB when the browser says nothing, because a decoded file costs the wasm heap about five times its size. Each file's size is asked by HEAD first; one that would put the scene over the budget is skipped, and the status line names it and the knob instead of the page dying without a word.


<!-- file: 14 session_viewer/src/app/loader.rs type hunks=1 -->

<!-- file: 14 session_viewer/src/app/loader.rs type hunks=2 -->

<!-- file: 14 session_viewer/src/app/loader.rs type hunks=3 -->

## Step 7 · Wire the modules and the text message

- `decode`, `fetch` and `live` are browser-only; `manifest` and `validate` compile natively too.
- Manifest text reaches State through one `Msg::Texts`; `set_texts` builds fixed-plane labels and grows the fit bounds by the shaped text extents.

```mermaid
flowchart LR
    M["app::mod"] --> D["decode · fetch · live"]
    T["Msg::Texts"] --> S["set_texts"]
    style M fill:#f0bcdb,stroke:#ce4095,color:#111
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/src/app/mod.rs type -->

<!-- file: 14 session_viewer/src/app/scene.rs type -->

<!-- file: 14 session_viewer/src/lib.rs type -->

<!-- file: 14 session_viewer/src/state.rs type -->

- Trunk watches the kernel next door and serves the index uncached, so a rebuilt bundle is never hidden behind a stale page.
- A `pre_build` hook runs `docs/build_site.sh` before every bundle: it builds the documentation site into `target/docs/site`, which the page's `copy-dir` link publishes as `dist/docs`, so the black corner opens the course from the same `dist/` the viewer is served from.
- The hook rebuilds only when a documentation source is newer than the built `index.html`; a checkout without the course sources or without `uvx` gets a placeholder page instead of a failed build.

```mermaid
flowchart TB
    H["pre_build hook"] -- "docs/build_site.sh" --> S["target/docs/site"]
    S -- "copy-dir" --> D["dist/docs"]
    C["#viewer-docs corner"] -- "docs/" --> D
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 14 session_viewer/Trunk.toml copy -->

<!-- file: 14 session_viewer/docs/build_site.sh copy -->

<!-- supplied: 14 -->

## Check

<!-- checkpoint: 14 -->

Expected:

- The local fixture loads through the manifest and protobuf path.
- Click an object: selection still highlights; F10 still shows controls.
- The status line clears once the last item is posted.

If nothing loads, read the status text: it names the failing stage (manifest fetch, manifest parse, file fetch, decode).

![Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.](screenshots/14.png)

![Left: a manifest that lists the same file twice, the second with `at: [0, 12, 0]`, plus a fixed-plane text item; one download, two placements. Right: a manifest naming a missing file, and the status names the stage that failed.](screenshots/14-manifest.png)

## What changed

<!-- tree: 14 session_viewer/src/app -->

- `SceneRoute` → `Manifest` → validated `Session` → staged `PendingDocument` → `Msg::File`/`Msg::StreamedCloud`/`Msg::Texts`/`Msg::Fit`.
- A `LiveSource` re-reads a published manifest with ETags and replaces the scene only when every file is readable.
- `docs/build_site.sh` runs as Trunk's pre-build hook and keeps `dist/docs` current, so the page's documentation corner works in a served build.

**Production equivalent:** `src/app/manifest.rs`, `validate.rs`, `decode.rs`, `route.rs`, `live.rs`, `loader.rs` are the production files.

## Try

- Write `dist/scenes/two.yaml` listing `pb/interaction.pb` twice, the second entry with `at: [0, 12, 0]`, and open `?scene=two.yaml&data=off`: the file is fetched once and placed twice.
- Add a `texts` entry with `at`, `right`, `up` and `height`: the label sits in that world plane and foreshortens with the view.
- Point an item at a file that does not exist: the status reads which stage failed and the previous scene stays on screen.
- Give an item a non-orthogonal `xform`: `Manifest::parse` rejects it before any file is fetched.
- Touch a file under `docs/` and run `trunk serve` again: the hook rebuilds the site, and the black corner opens the fresh page from `dist/docs`.

## Next

[15 · Publication and streamed reads](15-publication.md): bounded metadata windows for streamed clouds, and the publication helpers.
