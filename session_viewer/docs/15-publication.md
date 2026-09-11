# 15 · Publication and streamed reads

## You are building

```mermaid
flowchart TB
    C["cloud .pb<br/>length-delimited fields"] -- "header read at `at`" --> W["MetadataWindow<br/>at · bytes"]
    W -- "slice hit" --> P["parse tag · length"]
    W -- "slice miss → source_range(at, ≥64 KiB)" --> W
    P -- "large geometry field" --> S["skip by length"]
    P -- "small LOD array" --> L["lod.set_field"]
    S --> P
```

## Starting point

- Checkpoint 14: a streamed cloud locates its arrays with one small range request per protobuf header and one per array body.
- This lesson keeps the same parse and adds a bounded read-ahead window, so adjacent small fields share one request while large arrays are still skipped by length.

## Step 1 · A bounded window over the metadata

![The file is small fields between huge arrays; the window fetches the small fields once and skips the arrays by length.](illustrations/metadata-window.svg)

- Skipped geometry fields never decide the window's size: `read_length` reads at least 64 KiB inside the file, larger only for an array that is itself larger, and never past `end`.
- `slice` borrows an exact cached range, including a valid empty range at the window's end.


<!-- file: 15 session_viewer/src/app/stream.rs type hunks=1-2 -->

## Step 2 · Refill only on a jump

- `read` reuses the window when the requested range is inside it and replaces it under the same exposed revision otherwise; a changed ETag fails the read instead of mixing two revisions.

```mermaid
flowchart TB
    R["read(at, length)"] -- "inside window" --> H["reuse cached bytes"]
    R -- "outside window" --> F["refill · same ETag"]
    F -- "ETag changed" --> E["fail the read"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 15 session_viewer/src/app/stream.rs type hunks=3-3 -->

## Step 3 · Route the LOD walk through the window

The loop is unchanged: headers, skips and array bodies now borrow from `window` instead of issuing their own requests.

```mermaid
flowchart TB
    L["LOD walk loop"] -- "headers · skips · arrays" --> W["window.read"]
    W --> B["borrowed bytes"]
    B --> P["parsed LOD fields"]
    style L fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 15 session_viewer/src/app/stream.rs type hunks=4-6 -->

A unit test of the range rules, part of the file:

<!-- file: 15 session_viewer/src/app/stream.rs copy hunks=7-7 -->

## Step 4 · Publication helpers

- Publishing writes the immutable geometry revision first, verifies it, then updates the alias and the mutable manifest, so a manifest never points at missing bytes.
- Credentials stay in the local shell helpers; nothing in the browser bundle can write to the bucket.

```mermaid
flowchart TB
    G["geometry bytes"] -- "put + verify" --> R["immutable revision"]
    R -- "copy" --> A["stable alias"]
    A -- "then" --> M["mutable manifest"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- supplied: 15 -->

## Check

<!-- checkpoint: 15 -->

Expected:

- The local scene loads unchanged.
- A streamed cloud (`?scene=stream-test.yaml` with a local `?data=` server) still shows its display prefix and F10 still reaches source points beyond it.

![Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request.](screenshots/15.png)

## What changed

<!-- tree: 15 session_viewer/src/app -->

- `MetadataWindow` sits between `cloud_lod` and `source_range`; header and small-array reads share one cached range.
- Publication scripts under `bash/` write geometry, verify, alias, then manifest.

**Production equivalent:** `src/app/stream.rs`; `bash/view_put.sh`, `bash/view_live.sh`, `bash/lib/`.

## Try

- Open the browser's network panel while a streamed cloud loads and count the `Range` requests with and without the window: the header and node-table reads collapse into one window read.
- Lower the 64 KiB minimum in `read_length` to 1 KiB: the walk still succeeds, but every small field past the first kilobyte refills the window and the request count climbs back.
- Change the served file while the viewer is open so its ETag changes: the next read outside the window fails instead of mixing two revisions, and the status says so.


## Recall

??? question "The read window has a 64 KiB minimum, yet a large array is still skipped by its length. Why both rules?"
    Because the file is small fields separated by huge arrays. The minimum makes adjacent small fields — headers, node tables — share a single request instead of one round-trip each. Skipping by length keeps the window from ever pulling a geometry array it does not need. Lower the minimum and the request count climbs back, as the lesson's own experiment shows; drop the skip and you download the file you were trying to avoid.

??? question "A changed ETag fails the read instead of refilling the window. Defend that."
    Because the two halves would come from different revisions of the file, and the result would be a plausible, silently wrong scene — offsets from one version indexing bytes of another. Failing is recoverable: reload and get a consistent revision. Mixing is not detectable after the fact.

??? question "Publication writes the immutable geometry revision first, verifies it, then updates the alias and the manifest. What invariant does that ordering protect?"
    That a manifest never points at bytes that do not exist yet. Any reader arriving mid-publish sees either the old, complete scene or the new, complete scene — never a manifest naming a file still uploading. Write the thing that is pointed *at* before the pointer; it is the same ordering discipline as any atomic swap.

??? question "Credentials live in the local shell helpers, never in the browser bundle. What follows from that?"
    Anything shipped to a browser is public — the bundle, its constants, its query parameters. Publication is a local operation with local credentials, and the deployed viewer can only read. This is worth stating because the temptation is always to add "just one" write endpoint.

**Rebuild from memory:** without looking, describe what the network panel shows for a streamed cloud with and without the window, and say which of the two numbers a user would actually notice. Then decide whether this optimisation would be worth it for a local file — and why the answer differs.

## Next

[16 · Resource accounting](16-accounting.md): what the viewer can and cannot measure about its own memory.
