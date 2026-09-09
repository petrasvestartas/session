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

![The file is small fields between huge arrays; checkpoint 14 fetched each small field separately, the window fetches them once and skips the arrays by length.](illustrations/metadata-window.svg)

- Skipped geometry fields never decide the window's size: `read_length` reads at least 64 KiB inside the file, larger only for an array that is itself larger, and never past `end`.
- `slice` borrows an exact cached range, including a valid empty range at the window's end.

```mermaid
flowchart LR
    A["cloud .pb bytes"] -- "read_length ≥ 64 KiB" --> W["MetadataWindow<br/>at · bytes"]
    W -- "slice(at, length)" --> S["exact borrowed range"]
    style W fill:#1a1eb2,color:#fff
```

<!-- file: 15 session_viewer/src/app/stream.rs type hunks=1,2 -->

## Step 2 · Refill only on a jump

- `read` reuses the window when the requested range is inside it and replaces it under the same exposed revision otherwise; a changed ETag fails the read instead of mixing two revisions.

```mermaid
flowchart LR
    R["read(at, length)"] -- "inside window" --> H["reuse cached bytes"]
    R -- "outside window" --> F["refill · same ETag"]
    F -- "ETag changed" --> E["fail the read"]
    style R fill:#1a1eb2,color:#fff
```

<!-- file: 15 session_viewer/src/app/stream.rs type hunks=3 -->

## Step 3 · Route the LOD walk through the window

The loop is unchanged: headers, skips and array bodies now borrow from `window` instead of issuing their own requests.

```mermaid
flowchart LR
    L["LOD walk loop"] -- "headers · skips · arrays" --> W["window.read"]
    W --> B["borrowed bytes"]
    B --> P["parsed LOD fields"]
    style L fill:#1a1eb2,color:#fff
```

<!-- file: 15 session_viewer/src/app/stream.rs type hunks=4,5,6 -->

A unit test of the range rules, part of the file:

<!-- file: 15 session_viewer/src/app/stream.rs copy hunks=7 -->

## Step 4 · Publication helpers

- Publishing writes the immutable geometry revision first, verifies it, then updates the alias and the mutable manifest, so a manifest never points at missing bytes.
- Credentials stay in the local shell helpers; nothing in the browser bundle can write to the bucket.

```mermaid
flowchart LR
    G["geometry bytes"] -- "put + verify" --> R["immutable revision"]
    R -- "copy" --> A["stable alias"]
    A -- "then" --> M["mutable manifest"]
    style R fill:#1a1eb2,color:#fff
```

<!-- supplied: 15 -->

## Check

<!-- checkpoint: 15 -->

Expected:

- The local scene loads exactly as at checkpoint 14.
- A streamed cloud (`?scene=stream-test.yaml` with a local `?data=` server) still shows its display prefix and F10 still reaches source points beyond it.

![Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request.](screenshots/15.png)

## What changed

<!-- tree: 15 session_viewer/src/app -->

- `MetadataWindow` sits between `cloud_lod` and `source_range`; header and small-array reads share one cached range.
- Publication scripts under `bash/` write geometry, verify, alias, then manifest.

**Production equivalent:** `src/app/stream.rs`; `bash/view_put.sh`, `bash/view_live.sh`, `bash/lib/`.

## Try

- Open the browser's network panel while a streamed cloud loads and count the `Range` requests against the same scene at checkpoint 14: the header and node-table reads collapse into one window read.
- Lower the 64 KiB minimum in `read_length` to 1 KiB: the walk still succeeds, but every small field past the first kilobyte refills the window and the request count climbs back.
- Change the served file while the viewer is open so its ETag changes: the next read outside the window fails instead of mixing two revisions, and the status says so.

## Next

[16 · Resource accounting](16-accounting.md): what the viewer can and cannot measure about its own memory.
