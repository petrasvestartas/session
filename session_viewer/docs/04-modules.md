# 04 · Add independent drawing modules

**Start:** checkpoint 03. **Finish:** meshes, strokes, markers and a small cloud share one GPU context.

## Split by owned resources

A drawing module is a concrete owner, not a second renderer. It creates the buffers and pipelines for one representation, accepts changed rows, records draws and releases scene-sized resources. The Gpu coordinator decides when to call it.

```text
source geometry → producer → typed upload rows
                                  ├─ mesh vertices / indices → mesh owner
                                  ├─ segment endpoints       → stroke owner
                                  ├─ marker records          → glyph owner
                                  └─ cloud rows / hierarchy  → cloud owner
                         shared device, queue and frame inputs
```

Do not split a type into many files merely because it has many methods. Creation, upload, draw and release share the same resource contract. Conversely, cloud LOD and font shaping do not belong inside the triangle draw function just because they eventually draw pixels.

## Read creation before drawing

Start with the typed upload records. Then read the buffer owner and its capacity policy. Appending rows may grow a buffer; updating a few flags should not. Replacing a buffer also requires replacing any bind group that refers to the old buffer.

The mesh module stores indexed triangles. A segment stores endpoints and style; its shader expands a screen-space footprint instead of relying on platform line-width support. Markers similarly use an explicit footprint. The cloud path can choose visible/resident points while retaining their source mapping.

A **bind group** connects actual buffers/textures to the layout expected by the pipeline. A **draw range** says which rows to process. These objects have different lifetimes: a pipeline can survive many scene replacements while a scene-sized buffer can be released.

## Frame coordination stays explicit

The frame code calls the modules in a readable order. It does not decode protobuf, compute NURBS geometry or allocate another device during drawing. `Upload` carries prepared data; the source producer owns how that data was derived.

This makes removal testable. Removing the standalone point producer should leave meshes, curves and other families functioning. The maintained modularity check later performs that exercise on an isolated source copy.

## Write the files

Follow [Complete file changes for 04](../lessons/04/index.md). Read the contracts and shaders first, their GPU owners next, and the integration last. Every import and module declaration is included; the generated file list is the exact editing order.

The fixture is deliberately small. You can inspect each representation before larger datasets make an ownership mistake difficult to locate.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, expect four displayed objects covering the introduced families. Move the camera and resize. Each representation must keep its placement and style.

If one family disappears, follow its producer → upload → buffer → draw range. If all families fail after resizing, inspect shared targets and frame inputs before changing individual shaders.

**Before continuing:** name the owner that would release each large buffer on scene replacement. Continue to [physical visibility](05-visibility.md).
