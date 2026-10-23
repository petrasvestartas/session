# 19.1 · Complete source metadata and the actual command contract

Retain original non-drawing data and exact source grammar instead of saving only what happens to render.

## Ownership and data flow

Source closure includes unused definitions, components, interactions and frame families beside displayed objects. Preserve those namespaces and metadata through archives without inventing visible rows for every record. Compare the actual maintained registry/parser with course commands and record explicit unsupported or staged differences. Diagnostics count retained original records separately from GPU geometry; exact round trips prove original metadata survived, not just a similar picture.

The October 23 inventory records 312 viewer, 109 kernel and 12 Wood source/build entries. Hash valid UTF-8 source text after normalizing CRLF to LF; keep binary bytes unchanged. The verifier still requires every declared path and exact digest. The course intentionally rejects extra Opacity arguments at this checkpoint; the maintained parser accepts them, and 96zzr later teaches that behavior.

## The maintained implementation

app/session_io.rs preserves original Session document protobufs plus viewer metadata. app/command/mod.rs and verbs define the maintained Spec/Action vocabulary; app/inspection.rs reports actual source owners. app/validate.rs rejects malformed original planes and data before display preparation. Course grammar is staging wherever it differs from these production registrations.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/session_io.rs](../../../src/app/session_io.rs), [app/command/mod.rs](../../../src/app/command/mod.rs), [app/hierarchy.rs](../../../src/app/hierarchy.rs), [app/scene.rs](../../../src/app/scene.rs), [app/selection.rs](../../../src/app/selection.rs), [app/deform.rs](../../../src/app/deform.rs), [app/walk/mod.rs](../../../src/app/walk/mod.rs), [build.rs](../../../build.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-96-retain-non-drawing-originals-and-validate-raw-planes"></a>

<details>
<summary>Implementation 96-retain-non-drawing-originals-and-validate-raw-planes: Retain non drawing originals and validate raw planes</summary>

Retain non drawing originals and validate raw planes.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/source_metadata.rs`

Own exact non drawing source metadata and pre-kernel plane validation.

Create the file and type:

```rust
--8<-- "journey/code/96-retain-non-drawing-originals-and-validate-raw-planes-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod bench_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96-retain-non-drawing-originals-and-validate-raw-planes-02.rs" skip=30
```

</details>

<a id="checkpoint-96a-merge-unused-definitions-with-displayed-instances"></a>

<details>
<summary>Implementation 96a-merge-unused-definitions-with-displayed-instances: Merge unused definitions with displayed instances</summary>

Merge unused definitions with displayed instances.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/definition_merge.rs`

Merge exact retained definitions without losing unused originals.

Create the file and type:

```rust
--8<-- "journey/code/96a-merge-unused-definitions-with-displayed-instances-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_metadata;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96a-merge-unused-definitions-with-displayed-instances-02.rs" skip=26
```

</details>

<a id="checkpoint-96b-preserve-component-and-interaction-metadata-in-archives"></a>

<details>
<summary>Implementation 96b-preserve-component-and-interaction-metadata-in-archives: Preserve component and interaction metadata in archives</summary>

Preserve component and interaction metadata in archives.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/origin.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
        let header = proto::Session { name: message.name.clone(), guid: message.guid.clone(),
            tree: message.tree.clone(), graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(), xforms: message.xforms.clone(), ..Default::default() };
        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes), location: None }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-01.rs"
```

### 2. `src/document_source.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {return Err("Interactions arrive with element features".into());}
    let objects=source.objects.as_ref().ok_or("Session has no objects")?;
    if !objects.components.is_empty() {return Err("This subject cannot open assemblies yet".into());}
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(components);identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-02.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-03.rs"
```

</details>

### 3. `src/source_validate.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {if !crate::plane_original::valid(&session_rust::Plane::from_proto(item.clone())) {return Err("Invalid original plane frame".into())}}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
```

</details>

Replace it with:

```rust
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len()+objects.components.len()+objects.sheets.len()+objects.instances.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {crate::source_metadata::raw_plane(item)?;}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use prost::Message;
use session_rust::proto;

/// Source objects a scene may hold.
pub const MAX_OBJECTS: usize = 2_000_000;

/// Why a scene with more objects is refused.
pub const TOO_MANY: &str = "scene exceeds two million source objects";

/// Deepest tree level a scene may have.
pub const MAX_DEPTH: usize = 64;

/// Why a scene with a deeper tree is refused.
pub const TOO_DEEP: &str = "scene hierarchy exceeds 64 levels";

/// Every value must be finite.
fn finite(values: &[f64], label: &str) -> Result<(), String> {
    for value in values {
        if !value.is_finite() {
            return Err(format!("{label}: non-finite value"));
        }
    }

    Ok(())
}

/// A finite array of whole xyz triples.
fn triples(values: &[f64], label: &str) -> Result<(), String> {
    if !values.len().is_multiple_of(3) {
        return Err(format!("{label}: incomplete coordinate triple"));
    }

    finite(values, label)
}

/// A protobuf decode error as a message.
fn decode_error(error: prost::DecodeError) -> String {
    format!("invalid element geometry: {error}")
}

/// The varint at `i` and its byte length.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;

    loop {
        let byte = *b.get(i)?;

        if shift == 63 && byte > 1 {
            return None;
        }

        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;

        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }

        shift += 7;

        if shift > 63 {
            return None;
        }
    }
}

/// The highest control index a surface reads, None on overflow.
fn addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -> Option<usize> {
    (rows - 1)
        .checked_mul(stride_u)?
        .checked_add((columns - 1).checked_mul(stride_v)?)?
        .checked_add(size)
}

/// Check order, count and knots; returns the knot count.
fn axis(order: i32, count: i32, knots: &[f64]) -> Result<usize, String> {
    if !(2..=64).contains(&order) || count < order || count > 1_000_000 {
        return Err("invalid or oversized NURBS order/control count".into());
    }

    if knots.len() != (order + count - 2) as usize {
        return Err("NURBS knot count does not match its order and controls".into());
    }

    finite(knots, "NURBS knots")?;

    for pair in knots.windows(2) {
        if pair[0] > pair[1] {
            return Err("NURBS knots are not monotonic".into());
        }
    }

    if knots[(order - 2) as usize] >= knots[(count - 1) as usize] {
        return Err("NURBS parameter domain is empty".into());
    }

    Ok(count as usize)
}

/// A JSON parse error as a message.
fn json_error(error: serde_json::Error) -> String {
    format!("invalid session JSON: {error}")
}

/// A count field, defaulted and capped.
fn json_count(
    source: &serde_json::Value,
    key: &str,
    default: u64,
    maximum: u64,
) -> Result<usize, String> {
    let count = match source.get(key) {
        Some(value) => match value.as_u64() {
            Some(value) => value,
            None => return Err(format!("invalid JSON {key}")),
        },
        None => default,
    };

    if count > maximum {
        return Err(format!("JSON {key} exceeds its supported count"));
    }

    Ok(count as usize)
}

/// Check one JSON knot vector.
fn json_axis(
    source: &serde_json::Value,
    order_key: &str,
    count: usize,
    knots_key: &str,
) -> Result<(), String> {
    let order = json_count(source, order_key, 4, 64)?;
    let values = source[knots_key]
        .as_array()
        .ok_or("missing JSON NURBS knots")?;
    let mut knots = Vec::with_capacity(values.len());

    for value in values {
        knots.push(value.as_f64().ok_or("invalid JSON NURBS knot")?);
    }

    axis(order as i32, count as i32, &knots)?;
    Ok(())
}

/// Check one JSON curve or surface's controls against its counts.
fn json_controls(source: &serde_json::Value) -> Result<(), String> {
    let controls = source["control_points"]
        .as_array()
        .ok_or("JSON controls must be an array")?;
    let dimension = json_count(source, "dimension", 3, 3)?;

    if dimension < 2 {
        return Err("invalid JSON control dimension".into());
    }

    let size = dimension + usize::from(source["is_rational"].as_bool().unwrap_or(false));

    if source.get("cv_count_u").is_some() {
        let rows = json_count(source, "cv_count_u", 0, 1_000_000)?;
        let columns = json_count(source, "cv_count_v", 0, 1_000_000)?;
        let total = rows as u64 * columns as u64;

        if total > 4_000_000 || controls.len() as u64 != total * size as u64 {
            return Err("JSON surface controls disagree with their declared counts".into());
        }

        json_axis(source, "order_u", rows, "nurbsknots_u")?;
        json_axis(source, "order_v", columns, "nurbsknots_v")?;
    } else {
        let count = json_count(source, "cv_count", 0, 1_000_000)?;
        let stride = json_count(source, "cv_stride", size as u64, 4)?;

        if count != controls.len() || stride != size {
            return Err("JSON curve controls disagree with their declared count or stride".into());
        }

        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }

        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}

/// Check every NURBS record in session JSON before it is parsed.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];

    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }

        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count") || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }

                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Check one protobuf point.
pub fn point(source: &proto::Point) -> Result<(), String> {
    finite(&[source.x, source.y, source.z, source.width], "point")
}

/// Check one protobuf line.
pub fn line(source: &proto::Line) -> Result<(), String> {
    if source.coords.len() != 6 {
        return Err("line needs exactly two endpoint triples".into());
    }

    finite(&source.coords, "line coordinates")?;
    finite(&source.dash, "line dashes")
}

/// Check one protobuf polyline.
pub fn polyline(source: &proto::Polyline) -> Result<(), String> {
    triples(&source.coords, "polyline coordinates")
}

/// Check one object placement.
pub fn xform(entry: &proto::XformEntry) -> Result<(), String> {
    let Some(transform) = &entry.xform else {
        return Err("missing object transform".into());
    };
    finite(&transform.matrix, "object transform")?;

    if transform.matrix.len() != 16
        || transform.matrix[3] != 0.0
        || transform.matrix[7] != 0.0
        || transform.matrix[11] != 0.0
        || transform.matrix[15] != 1.0
    {
        return Err("object placement must be a sixteen-value affine matrix".into());
    }

    Ok(())
}

/// Nothing to check.
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}

/// Every index must name a vertex.
fn mesh_indices(source: &proto::Mesh, indices: &[u64]) -> Result<(), String> {
    for index in indices {
        if !source.vertices.contains_key(index) {
            return Err(format!("mesh references missing vertex {index}"));
        }
    }

    Ok(())
}

/// Check one protobuf mesh.
pub fn mesh(source: &proto::Mesh) -> Result<(), String> {
    for (key, vertex) in &source.vertices {
        if *key > u32::MAX as u64 {
            return Err("mesh vertex key exceeds the browser index range".into());
        }

        finite(&[vertex.x, vertex.y, vertex.z], "mesh vertex")?;

        for value in vertex.attributes.values() {
            if !value.is_finite() {
                return Err("mesh vertex has a non-finite attribute".into());
            }
        }
    }

    for face in source.faces.values() {
        if face.vertices.len() < 3 {
            return Err("mesh face has fewer than three vertices".into());
        }

        mesh_indices(source, &face.vertices)?;

        for hole in &face.holes {
            mesh_indices(source, &hole.vertices)?;
        }
    }

    for (face, triangles) in &source.triangulation {
        if !source.faces.contains_key(face) || !triangles.vertices.len().is_multiple_of(3) {
            return Err("invalid mesh triangulation record".into());
        }

        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}

/// Check one protobuf curve.
pub fn curve(source: &proto::NurbsCurve) -> Result<(), String> {
    if !(2..=3).contains(&source.dimension) {
        return Err("NURBS curve dimension must be 2 or 3".into());
    }

    let count = axis(source.order, source.cv_count, &source.nurbsknots)?;
    let size = source.dimension as usize + usize::from(source.is_rational);

    if source.cvs.len() != count * size {
        return Err("NURBS curve control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS curve controls")
}

/// Check one protobuf surface.
pub fn surface(source: &proto::NurbsSurface) -> Result<(), String> {
    if source.dimension != 3 {
        return Err("NURBS surface dimension must be 3".into());
    }

    let rows = axis(source.order_u, source.cv_count_u, &source.nurbsknots_u)?;
    let columns = axis(source.order_v, source.cv_count_v, &source.nurbsknots_v)?;

    if rows.saturating_mul(columns) > 4_000_000 {
        return Err("surface exceeds four million controls".into());
    }

    let size = 3 + usize::from(source.is_rational);
    let stride_u = if source.cv_stride_u > 0 {
        source.cv_stride_u as usize
    } else {
        columns * size
    };
    let stride_v = if source.cv_stride_v > 0 {
        source.cv_stride_v as usize
    } else {
        size
    };
    let addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);

    if !matches!(addressed, Some(length) if length <= source.cvs.len()) {
        return Err("NURBS surface control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS surface controls")?;

    if let Some(cached) = &source.cached_mesh {
        mesh(cached)?;
    }

    Ok(())
}

/// Check one protobuf point cloud.
pub fn cloud(source: &proto::PointCloud) -> Result<(), String> {
    triples(&source.coords, "point cloud")?;
    finite(&source.normals, "point normals")?;
    let points = source.coords.len() / 3;

    if (!source.normals.is_empty() && source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() && source.colors.len() != points * 4)
        || (!source.point_ids.is_empty() && source.point_ids.len() != points)
    {
        return Err("point-cloud attribute counts disagree".into());
    }

    let nodes = source.lod_size.len();

    if nodes > 0 {
        if source.lod_min.len() != nodes * 3
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * 8
        {
            return Err("point-cloud octree arrays disagree".into());
        }

        finite(&source.lod_min, "octree bounds")?;
        finite(&source.lod_size, "octree sizes")?;

        for node in 0..nodes {
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64 > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }

        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }

    Ok(())
}

/// Check one protobuf BRep.
pub fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }

    for item in &source.curves_3d {
        curve(item)?;
    }

    for item in &source.surfaces {
        surface(item)?;
    }

    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }

    Ok(())
}

/// Check one protobuf element's mesh or BRep.
pub fn element(source: &proto::Element) -> Result<(), String> {
    match source.geometry_type.as_str() {
        "Mesh" => {
            mesh(&proto::Mesh::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        "BRep" => {
            brep(&proto::BRep::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        _ => Ok(()),
    }
}

/// Check every definition record, and the definitions with `instances` within the cap.
pub fn definitions(d: &proto::Objects, instances: usize) -> Result<(), String> {
    let count = d.points.len()
        + d.lines.len()
        + d.polylines.len()
        + d.meshes.len()
        + d.pointclouds.len()
        + d.nurbscurves.len()
        + d.nurbssurfaces.len()
        + d.breps.len()
        + d.elements.len();

    if count + instances > MAX_OBJECTS {
        return Err(TOO_MANY.into());
    }

    for item in &d.points {
        point(item)?;
    }

    for item in &d.lines {
        line(item)?;
    }

    for item in &d.polylines {
        polyline(item)?;
    }

    for item in &d.meshes {
        mesh(item)?;
    }

    for item in &d.pointclouds {
        cloud(item)?;
    }

    for item in &d.nurbscurves {
        curve(item)?;
    }

    for item in &d.nurbssurfaces {
        surface(item)?;
    }

    for item in &d.breps {
        brep(item)?;
    }

    for item in &d.elements {
        element(item)?;
    }

    Ok(())
}

/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.planes.len()+objects.bboxes.len()+objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len()+objects.components.len()+objects.sheets.len()+objects.instances.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {crate::source_metadata::raw_plane(item)?;}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
        }

        for item in &objects.lines {
            line(item)?;
        }

        for item in &objects.polylines {
            polyline(item)?;
        }

        for item in &objects.meshes {
            mesh(item)?;
        }

        for item in &objects.pointclouds {
            cloud(item)?;
        }

        for item in &objects.nurbscurves {
            curve(item)?;
        }

        for item in &objects.nurbssurfaces {
            surface(item)?;
        }

        for item in &objects.breps {
            brep(item)?;
        }

        for item in &objects.elements {
            element(item)?;
        }
    }

    for entry in &source.xforms {
        xform(entry)?;
    }

    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];

        while let Some((node, depth)) = pending.pop() {
            if depth > MAX_DEPTH {
                return Err(TOO_DEEP.into());
            }

            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}

/// Check every geometry of a loaded session.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Huge or wrong counts are refused.
    #[test]
    fn hostile_counts_do_not_reach_kernel_allocations() {
        let source = proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&source).is_err());
        let source = proto::NurbsSurface {
            dimension: 3,
            order_u: 2,
            order_v: 2,
            cv_count_u: -1,
            cv_count_v: 2,
            ..Default::default()
        };
        assert!(surface(&source).is_err());
    }

    /// Bad indices and short cloud arrays are refused.
    #[test]
    fn missing_indices_and_partial_cloud_attributes_are_errors() {
        let mut source = proto::Mesh::default();
        source.faces.insert(
            0,
            proto::FaceData {
                vertices: vec![0, 1, 2],
                ..Default::default()
            },
        );
        assert!(mesh(&source).is_err());
        let cloud_source = proto::PointCloud {
            coords: vec![0., 0., 0.],
            point_ids: vec![7, 8],
            ..Default::default()
        };
        assert!(cloud(&cloud_source).is_err());
    }

    /// JSON counts must match the controls given.
    #[test]
    fn json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(r#"{"control_points":[],"cv_count":18446744073709551615,"dimension":3}"#).is_err()
        );
        assert!(
            json(r#"{"control_points":[],"cv_count_u":1000000,"cv_count_v":1000000}"#).is_err()
        );
        assert!(json(r#"{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}"#).is_ok());
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-04.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-05.rs"
```

</details>

### 4. `src/archive_document.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn prepare(scene:&Scene,namespace:&Namespace)->Result<(proto::Session,HashMap<String,String>,f32),String> {
    let names=crate::archive_identity::aliases(scene,namespace)?;let mut objects=crate::geometry_snapshot::objects(scene,&namespace.ids)?;crate::archive_identity::objects(&mut objects,&names);
    let mut xforms=Vec::new();let (mut roots,placed)=if let Some(doc)=namespace.doc.as_ref() {let mut forest=crate::hierarchy::Forest::default();forest.documents.push(doc.clone());forest.snapshot(scene,&mut xforms).map_err(str::to_owned)?} else {(vec![],HashSet::new())};
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.into());
        if !placed.contains(&id) {roots.push(proto::TreeNode {name:guid.clone(),..Default::default()});let model=scene.placement(id).ok_or("Saved source placement is missing")?;if model.m!=session_rust::Xform::identity().m {xforms.push(proto::XformEntry {guid,xform:Some(model.to_proto())});}}
    }
    for root in &mut roots {crate::archive_identity::tree(root,&names);}for entry in &mut xforms {if let Some(guid)=names.get(&entry.guid) {entry.guid=guid.clone();}}
    let mut source=namespace.doc.as_ref().and_then(|doc|doc.header.as_ref()).map_or_else(proto::Session::default,|header|header.header.clone());if source.name.is_empty() {source.name=namespace.name.clone();}
    if let Some(doc)=namespace.doc.as_ref(){source.graph=crate::graph_edit::saved(scene,doc,&names);}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    source.definitions=crate::instance_snapshot::definitions(scene,&namespace.ids)?;source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn prepare(scene:&Scene,namespace:&Namespace)->Result<(proto::Session,HashMap<String,String>,f32),String> {
        let names=crate::archive_identity::aliases(scene,namespace)?;let mut objects=crate::geometry_snapshot::objects(scene,&namespace.ids)?;crate::archive_identity::objects(&mut objects,&names);
    let mut xforms=Vec::new();let (mut roots,placed)=if let Some(doc)=namespace.doc.as_ref() {let mut forest=crate::hierarchy::Forest::default();forest.documents.push(doc.clone());forest.snapshot(scene,&mut xforms).map_err(str::to_owned)?} else {(vec![],HashSet::new())};
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.into());
        if !placed.contains(&id) {roots.push(proto::TreeNode {name:guid.clone(),..Default::default()});let model=scene.placement(id).ok_or("Saved source placement is missing")?;if model.m!=session_rust::Xform::identity().m {xforms.push(proto::XformEntry {guid,xform:Some(model.to_proto())});}}
    }
    for root in &mut roots {crate::archive_identity::tree(root,&names);}for entry in &mut xforms {if let Some(guid)=names.get(&entry.guid) {entry.guid=guid.clone();}}
    let mut source=namespace.doc.as_ref().and_then(|doc|doc.header.as_ref()).map_or_else(proto::Session::default,|header|header.header.clone());if source.name.is_empty() {source.name=namespace.name.clone();}
    if let Some(doc)=namespace.doc.as_ref(){source.graph=crate::graph_edit::saved(scene,doc,&names);}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    if let Some(metadata)=source.objects.as_ref(){objects.components=metadata.components.clone();}
    source.definitions=crate::definition_merge::merge(source.definitions.as_ref(),crate::instance_snapshot::definitions(scene,&namespace.ids)?)?;source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-06.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-07.rs"
```

</details>

</details>

<a id="checkpoint-96c-keep-current-element-and-frame-definition-families"></a>

<details>
<summary>Implementation 96c-keep-current-element-and-frame-definition-families: Keep current Element and frame definition families</summary>

Keep current Element and frame definition families.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/instance_plan.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{proto,Geometry,InstanceRef,Session};
use std::{collections::HashSet,rc::Rc};

pub fn validate(message:&proto::Session)->Result<(),String> {
    let instances=message.objects.as_ref().map_or(&[][..],|objects|objects.instances.as_slice());
    let Some(definitions)=message.definitions.as_ref() else {return if instances.is_empty() {Ok(())} else {Err("Instances have no original definitions".into())};};
    crate::source_validate::definitions(definitions,instances.len())?;
    if !definitions.planes.is_empty()||!definitions.bboxes.is_empty()||!definitions.elements.is_empty()||!definitions.components.is_empty()||!definitions.sheets.is_empty()||!definitions.instances.is_empty() {return Err("Unsupported original definition family".into());}
    let mut guids=HashSet::new();
    macro_rules! identity {($field:ident)=>{for object in &definitions.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Definition GUIDs must be distinct".into());}}};}
    identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);
    let mut refs=HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()||!refs.insert(instance.guid.as_str())||!guids.contains(instance.definition_guid.as_str()) {return Err("Invalid original instance reference".into());}
        if instance.flags&InstanceRef::FLAG_COLOR!=0 {
            let color=instance.color.as_ref().ok_or("Instance colour flag has no original colour")?;
            if [color.r,color.g,color.b,color.a].iter().any(|value|!value.is_finite()||!(0.0..=1.0).contains(value)) {return Err("Invalid original instance colour".into());}
        }
        if !instance.features.is_empty() {return Err("Instance features connect with element features".into());}
    }
    Ok(())
}

#[derive(Clone)]
pub struct Owner {pub document:Rc<Session>,pub instance:Rc<InstanceRef>,pub definition:Geometry}
impl Owner {
    pub fn new(document:Rc<Session>,instance:Rc<InstanceRef>)->Result<Self,&'static str> {
        let definition=document.definition_lookup.get(&instance.definition_guid).ok_or("Missing original instance definition")?.clone();
        Ok(Self {document,instance,definition})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn message()->proto::Session {
        let mesh=crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.3,0.8]).unwrap().geometry;
        proto::Session {objects:Some(proto::Objects {instances:vec![proto::InstanceRef {guid:"first".into(),name:"First".into(),definition_guid:mesh.guid().into(),..Default::default()},proto::InstanceRef {guid:"second".into(),name:"Second".into(),definition_guid:mesh.guid().into(),..Default::default()}],..Default::default()}),definitions:Some(proto::Objects {meshes:vec![mesh.to_proto()],..Default::default()}),..Default::default()}
    }
    #[test]
    fn checked_original_references_share_definition_identity_without_copying_geometry() {
        let message=message();validate(&message).unwrap();let document=Rc::new(Session::from_proto(message).unwrap());let first=Owner::new(document.clone(),document.objects.instances[0].clone()).unwrap();let second=Owner::new(document.clone(),document.objects.instances[1].clone()).unwrap();let (Geometry::Mesh(a),Geometry::Mesh(b))=(&first.definition,&second.definition) else {panic!("Original mesh definition")};assert!(Rc::ptr_eq(a,b));assert_eq!(first.instance.guid(),"first");assert_eq!(second.instance.guid(),"second");let weak=Rc::downgrade(a);drop(document);drop(first);assert!(weak.upgrade().is_some());drop(second);assert!(weak.upgrade().is_none());
    }
    #[test]
    fn missing_duplicate_and_unsupported_original_definitions_are_refused() {
        let mut source=message();source.definitions=None;assert!(validate(&source).is_err());let mut source=message();source.objects.as_mut().unwrap().instances[0].definition_guid="missing".into();assert!(validate(&source).is_err());let mut source=message();let duplicate=source.definitions.as_ref().unwrap().meshes[0].clone();source.definitions.as_mut().unwrap().meshes.push(duplicate);assert!(validate(&source).is_err());let mut source=message();source.definitions.as_mut().unwrap().instances.push(Default::default());assert!(validate(&source).is_err());
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Geometry,InstanceRef,Session};
use std::{collections::HashSet,rc::Rc};

pub fn validate(message:&proto::Session)->Result<(),String> {
    let instances=message.objects.as_ref().map_or(&[][..],|objects|objects.instances.as_slice());
    let Some(definitions)=message.definitions.as_ref() else {return if instances.is_empty() {Ok(())} else {Err("Instances have no original definitions".into())};};
    crate::source_validate::definitions(definitions,instances.len())?;
    if !definitions.components.is_empty()||!definitions.sheets.is_empty()||!definitions.instances.is_empty() {return Err("Unsupported original definition family".into());}
    let mut guids=HashSet::new();
    macro_rules! identity {($field:ident)=>{for object in &definitions.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Definition GUIDs must be distinct".into());}}};}
    identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);
    let mut refs=HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()||!refs.insert(instance.guid.as_str())||!guids.contains(instance.definition_guid.as_str()) {return Err("Invalid original instance reference".into());}
        if instance.flags&InstanceRef::FLAG_COLOR!=0 {
            let color=instance.color.as_ref().ok_or("Instance colour flag has no original colour")?;
            if [color.r,color.g,color.b,color.a].iter().any(|value|!value.is_finite()||!(0.0..=1.0).contains(value)) {return Err("Invalid original instance colour".into());}
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct Owner {pub document:Rc<Session>,pub instance:Rc<InstanceRef>,pub definition:Geometry}
impl Owner {
    pub fn new(document:Rc<Session>,instance:Rc<InstanceRef>)->Result<Self,&'static str> {
        let definition=document.definition_lookup.get(&instance.definition_guid).ok_or("Missing original instance definition")?.clone();
        Ok(Self {document,instance,definition})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn message()->proto::Session {
        let mesh=crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.3,0.8]).unwrap().geometry;
        proto::Session {objects:Some(proto::Objects {instances:vec![proto::InstanceRef {guid:"first".into(),name:"First".into(),definition_guid:mesh.guid().into(),..Default::default()},proto::InstanceRef {guid:"second".into(),name:"Second".into(),definition_guid:mesh.guid().into(),..Default::default()}],..Default::default()}),definitions:Some(proto::Objects {meshes:vec![mesh.to_proto()],..Default::default()}),..Default::default()}
    }
    #[test]
    fn checked_original_references_share_definition_identity_without_copying_geometry() {
        let message=message();validate(&message).unwrap();let document=Rc::new(Session::from_proto(message).unwrap());let first=Owner::new(document.clone(),document.objects.instances[0].clone()).unwrap();let second=Owner::new(document.clone(),document.objects.instances[1].clone()).unwrap();let (Geometry::Mesh(a),Geometry::Mesh(b))=(&first.definition,&second.definition) else {panic!("Original mesh definition")};assert!(Rc::ptr_eq(a,b));assert_eq!(first.instance.guid(),"first");assert_eq!(second.instance.guid(),"second");let weak=Rc::downgrade(a);drop(document);drop(first);assert!(weak.upgrade().is_some());drop(second);assert!(weak.upgrade().is_none());
    }
    #[test]
    fn missing_duplicate_and_unsupported_original_definitions_are_refused() {
        let mut source=message();source.definitions=None;assert!(validate(&source).is_err());let mut source=message();source.objects.as_mut().unwrap().instances[0].definition_guid="missing".into();assert!(validate(&source).is_err());let mut source=message();let duplicate=source.definitions.as_ref().unwrap().meshes[0].clone();source.definitions.as_mut().unwrap().meshes.push(duplicate);assert!(validate(&source).is_err());let mut source=message();source.definitions.as_mut().unwrap().instances.push(Default::default());assert!(validate(&source).is_err());
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-01.rs"
```

```text
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-02.rs"
```

</details>

### 2. `src/instance_definition.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_prepared_cad(shapes.prepare(source.clone())?)?;},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-03.rs"
```

### 3. `src/instance_snapshot.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
    for (_,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        match &owner.definition {
            Geometry::Mesh(source)=>keep!(meshes,source),Geometry::BRep(source)=>keep!(breps,source),Geometry::NurbsSurface(source)=>keep!(nurbssurfaces,source),Geometry::NurbsCurve(source)=>keep!(nurbscurves,source),Geometry::Polyline(source)=>keep!(polylines,source),Geometry::Line(source)=>keep!(lines,source),Geometry::Point(source)=>keep!(points,source),Geometry::PointCloud(source)=>keep!(pointclouds,source),_=>return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {($field:ident)=>{out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));};}
    order!(meshes);order!(breps);order!(nurbssurfaces);order!(nurbscurves);order!(polylines);order!(lines);order!(points);order!(pointclouds);
    Ok((!records.is_empty()).then_some(out))
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-04.rs"
```

</details>

<a id="checkpoint-96d-observe-the-real-current-registry-and-parser"></a>

<details>
<summary>Implementation 96d-observe-the-real-current-registry-and-parser: Observe the real current registry and parser</summary>

Observe the real current registry and parser.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/registry_audit.rs`

Observe the compiled maintained command registry directly.

Create the file and type:

```rust
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod definition_merge;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-02.rs" skip=27
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `src/inventory_example.rs`

Copy the supplied native registry observation example.

Copy this supplied file:

```text
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-03.rs"
```

</details>

### 4. `Cargo.toml`

Expose the maintained registry observation as a native example.

<details>
<summary>Locate the existing block</summary>

```toml
name = "reference"
path = "src/reference_example.rs"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-04.rs" skip=53
```

</details>

<a id="checkpoint-96e-verify-the-complete-declared-current-source-closure"></a>

<details>
<summary>Implementation 96e-verify-the-complete-declared-current-source-closure: Verify the complete declared current source closure</summary>

Verify the complete declared current source closure.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/current_inventory.json`

Copy the supplied full current Rust source/build closure and ownership snapshot.

Copy this supplied file:

```text
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-01.rs"
```

</details>

### 2. `src/source_audit.rs`

Verify actual source bytes and the complete declared file set.

Create the file and type:

```rust
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-02.rs"
```

### 3. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod registry_audit;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-03.rs" skip=25
```

</details>

<a id="checkpoint-96f-report-retained-original-metadata-in-the-real-viewer"></a>

<details>
<summary>Implementation 96f-report-retained-original-metadata-in-the-real-viewer: Report retained original metadata in the real viewer</summary>

Report retained original metadata in the real viewer.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/browser.rs`

Expose truthful serialized original metadata accounting in the teaching inspector.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-groups",&serde_json::json!(editor.scene.groups.iter().collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-graph",&serde_json::json!(crate::graph_edit::rows(&editor.scene)).to_string())?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96f-report-retained-original-metadata-in-the-real-viewer-01.rs"
```

</details>

<a id="checkpoint-96g-prove-exact-metadata-and-definition-round-trips"></a>

<details>
<summary>Implementation 96g-prove-exact-metadata-and-definition-round-trips: Prove exact metadata and definition round trips</summary>

Prove exact metadata and definition round trips.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/metadata_tests.rs`

Copy supplied exact archive, original-family and malformed-input fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96g-prove-exact-metadata-and-definition-round-trips-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied metadata fixtures.

<details>
<summary>Locate the existing block</summary>

```text

pub mod source_audit;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96g-prove-exact-metadata-and-definition-round-trips-02.rs" skip=23
```

</details>

</details>

<a id="checkpoint-96h-prove-current-source-grammar-and-explicit-differences"></a>

<details>
<summary>Implementation 96h-prove-current-source-grammar-and-explicit-differences: Prove current source grammar and explicit differences</summary>

Continue from [Prove exact metadata and definition round trips](../96g-prove-exact-metadata-and-definition-round-trips.md). [Save or recover your work](../recovery.md).

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/current_registry.json`

Copy real observed current parser outcomes, prompts, aliases and action flags.

Copy this supplied file:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/inventory_tests.rs`

Copy supplied complete source/registry and explicit grammar/shortcut fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/lib.rs`

Register supplied complete source and registry fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)]
mod metadata_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-03.rs" skip=33
```

</details>

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 4. `src/current_inventory.json`

Copy the October 23 source inventory: 312 viewer, 109 kernel and 12 Wood source/build entries. Repository heads identify baselines; exact file hashes identify the recorded working tree.

<details>
<summary>Locate the existing block</summary>

```text
{
  "session_head": "2f96b35d225015cfcb9bfe5e8ec66565ef99aa28",
  "wood_head": "4916884a45e2558d57dd1a0175bca1bd8a627410",
  "kernel_head": "1c415309c46dd10f2a95a44eb980b067a5ef1651",
  "scope": "All files under the three Rust src trees and existing Cargo/build/web entries. Includes the preserved local rational-control change. Assets are separately hashed by the dependency evidence closure; C++ and generated models require their own provenance. Includes preserved October 10 hierarchy-path and parent Element-lamp changes; the original inventory remains historical.",
  "files": [
    {
      "root": "viewer",
      "path": "Cargo.lock",
      "sha256": "9b313409fa85751cba3224e7902022ee6a9420f7fdeaf69acf13c0ec62a6d9de",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Cargo.toml",
      "sha256": "f3c4a4e33279d03a069b041b21d5b25be8c5f2c1846adbf406c693fc40234002",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Trunk.toml",
      "sha256": "e36cf10ae1b9029bdcaa6b55468bef4dab8a3211db52763dcfc1e7521bebb9f8",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "build.rs",
      "sha256": "868449c0976bf081f2a3bdf270c1ca0842f923b33a9b9e34dfcb0804c7f5363f",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "index.html",
      "sha256": "fd308bc334190d76723135c1c82b4cd0d0343f3dff0dda573bf338aa5d3961b0",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/app/agent.rs",
      "sha256": "555a699f8286aaefa426d3a0f9704e1444f9a4af2ccb7105c41c40dadcb5455b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/clipping.rs",
      "sha256": "33047462339a5d4fe32742d38e1ca8e364aea126186fd107f1007fbf4158a8e1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cloud_query.rs",
      "sha256": "96e5ea670b083218e2105e0f2ffa0578cc5c25050b8ce3ee790d00dc3c401f0c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/mod.rs",
      "sha256": "4e7ae69c626379e2e2ca8383a0cdec283ff59c4376bf4b09d3e5b27bf7546f52",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tests.rs",
      "sha256": "7b075fc2845ccdb68ac48a148d3c98ac9f54403896e21bb0b4f616430ffa1bcc",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/choices.rs",
      "sha256": "2b128f729985b18c0901ebe6dd8780d54abf701394b020da3c3408f3b2b3841f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/cut.rs",
      "sha256": "d0d00438f5ec9230dd73cb19b14c3ad5cd65c043c294bc341c477e81f4ed5a5f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/elements.rs",
      "sha256": "ced86184ffcb868f8677c92445378647ffbccb1ee1c6efc0f626873f904b3eff",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/gather.rs",
      "sha256": "acb5cc3b8b424c5c0889fed4ccb994f3a13da0d4de10dcc0944f05fd932d913a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/options.rs",
      "sha256": "b86ccb79060e5159fbba2cd57d7b0fd22c14ca8472ab499f03bdbd5e1971e938",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/shape.rs",
      "sha256": "54c275cb3c8e5fbd4c0e3ea4a47910ca1ffcd5e92405d8055ff5088e05f049e7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/surfacing.rs",
      "sha256": "ef9ef71fcb065485333be2ad358e832de162c3acb30a754a73009b22fc8df7ce",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool.rs",
      "sha256": "c08608743e48879cab82f76157d5f028215af1848cd5b9eeefc343f039001b1b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_edge.rs",
      "sha256": "90e9e807c4a56b5b10bf5f1c3ad35326a5ffcda043043b62680068dc6b5585ed",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_group.rs",
      "sha256": "193f4e783390255a028dc24581a66659b2c82602e20a67f06d42d70540b8c10d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arctic.rs",
      "sha256": "04637eef3834bfb9c9d76d415d30f7803d857a267ad4472c91117dabbb374f39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/area.rs",
      "sha256": "9951cf47f2ba308fca8b14e79d0b21dd9de84ec71328f50c808bac970571b73e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrow.rs",
      "sha256": "cd80826f3f4170566ac9d72ee72fc24399c137a761282b1211392e1d2e187997",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrowhead.rs",
      "sha256": "33d52b895d61bd9754cfdbc590e867bb60ce7efa3dc8f8a208e99ff1b6114c87",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/attributes.rs",
      "sha256": "fb0300c25c2c5decebac7bfea8d7740e031fe58c16444c2bb36074f353bb7e19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/block_with_hole.rs",
      "sha256": "ef3b9abac04888315e6cb616f95cad7a296ffef58c5f3d417ed812379b2816e2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/bounding_box.rs",
      "sha256": "5baa3e329f91c9c6a2be572e4f8752e8f49effbbf564e3230bd00d8fbfb21479",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/box.rs",
      "sha256": "0f6b98541d82ea317acd703a136d8a5962bee64fe5f07150db36945c4443e986",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/capsule.rs",
      "sha256": "193edcbb4eff8c02c47ee461fdc0cee1ea7ad98777e882472dba3b5c1918e3f5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/clipping_plane.rs",
      "sha256": "400dc33be2b869ae93c0a981c17116fba56672dc6326ac50efb23ff71e7bc294",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/close.rs",
      "sha256": "6ecb9cf60556c359550c1c85dab54700a98b31f9883dd9317e6d1129241a24c5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cone.rs",
      "sha256": "c3021c10760195345f67555ed9db7920c58a0a7c19ad925f1a9bcb5d912e0992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/construction_plane.rs",
      "sha256": "078a4a7462a809f4b71978ac5579ecb8b9844eb41ccc9ad121a7efeab67aaa23",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/controls.rs",
      "sha256": "12bf2eb1ef670991804bbefcab39aeb3f450037d96981f973494f403b88fd909",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/copy.rs",
      "sha256": "eecc6b3b7ab0b379e8bfaa61e9be5210a13ab6e76917909ed6a0de6c34bcd257",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/curve.rs",
      "sha256": "00f85352d925c0196603c314a63090d359081f5b5aaafe635356d4ec1dc4ded1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cylinder.rs",
      "sha256": "8c69c4c8a45352d92b7963fa59a247594e67b9d5fee09b985f288fbdeb53744f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/delete.rs",
      "sha256": "c573604cbdc1487b2b94c8430ae04b4a2dae3421b11a6fdf02176de202d4ec19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/dodecahedron.rs",
      "sha256": "ce468793396a169f84b3c5454f931cfbeecfa6d07993604c1f4b7c192d0a431a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/edge.rs",
      "sha256": "fe7666c920dddffd7b9e195121966ea519aba55b7716f86b26ae3788b05e90c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam.rs",
      "sha256": "b99b68f91c511f4fd1d8443b72de9c81670ba48ff7bebb9fdf10366845057e00",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam_variable.rs",
      "sha256": "b0898d92d19ec629a2c673cb7a64a452e1bc182e09a98cf66ac7d9d299ce6bba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_block.rs",
      "sha256": "751684bcc9a7581e90fe2f2dc6f74b5f33fc0bda14a95ae9a4a10ee98d8ad601",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_column.rs",
      "sha256": "e3a32d979a0de748ea4440f488b0e9493cfb4238d6492eb25043fec32d2fefbb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_plate.rs",
      "sha256": "0d984d66a5a49c347e6801ca7d0d8dace30ddec622ca9d4a02bd8046ad12ab07",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_support.rs",
      "sha256": "4781d8a89d967bcc99dd142eee41066f85b6cf99fd46347f36ae340716379ea6",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/escape.rs",
      "sha256": "0e7a4072d6a64c4e1b5c5b76cae3d3da6a563461379be9222ea035ad132965d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/explode.rs",
      "sha256": "353e44e0cc9953f0e06aead9eeb42cfc9605bbcb8caa72941b945d16f732c7ac",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend/reach.rs",
      "sha256": "37f9ed15f0b4148bebc6788829a6fc8bca4a558201348c145cdbd670d5e18a34",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend.rs",
      "sha256": "c86295b7fd4314a6ba86581c4ea12d142d0f867ff7698c4b542ce3b0ffe33de3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extrude.rs",
      "sha256": "ebc559cd931a5803c1236d521509e929fa11f0e3b987e5eaa0167f2cd0ff2428",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/face.rs",
      "sha256": "c5dc6a0907e7ce3239eb3396036c96c1370f2ff4e6a8df355d453660f8e05bc7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/fit.rs",
      "sha256": "b1181698ef63b6facb5c50203928959d9b5f54087dea0e0e76cf065567ec719d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/geometry.rs",
      "sha256": "d1612ae09503003fca3792ad9e4fcac3666069f7f3781ca8b59de7d9653ae586",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/grid_snap.rs",
      "sha256": "ab63e5d6f45c37d9b8b15ea8535d8386da1cdcce725b2a3398ccdc41e01b06c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/hide.rs",
      "sha256": "c4a8c1c4fb3d20b2768e3903841ccfae51c8fd8fb7d6b30b1ec3ab61cbdca800",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/icosahedron.rs",
      "sha256": "02407c66c611d0b1961e5726196cfe944082ab19fd4e49a11d22c7e40c458765",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "1a4836207a54e368ad3d91d6e53df5c089e4584b8ce53dc5bd7b0bcfe3750d0e"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/layers.rs",
      "sha256": "a0168bfe8c941fee7fba32c9a259c9f847640f24baa958d38383948843f6db6f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length.rs",
      "sha256": "64a425e2e952b2daa0ec729235f2702934bb81f472b17f8c140b26415a618204",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length_bounding_box.rs",
      "sha256": "654498f77d15404be2ef63a4a6f96138186e2eb4d4799bae01310e8b89439fb3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/line.rs",
      "sha256": "cce63f69a0a8c3dd422e67bf1fec62075bdd6aef55b0a9a79a6f53191d2a278e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/loft.rs",
      "sha256": "0bd5c5e9169fc10b067629df1567ba2abfbae68aba97148b226e2ad16e96f818",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure.rs",
      "sha256": "004c72cbad5f8a24e7ae6141528997ce1e8deecb2e2caa22af68b34d5133c28d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure_distance.rs",
      "sha256": "2b3813e899954595d609de0357984c5b5f01a8f7f031e6320ccffde088001421",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/mod.rs",
      "sha256": "7feb8f196ccecf0bf11c3222eb200dd3d00b8a58d9058fd57d4aa48ad973ff5c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/move.rs",
      "sha256": "884f591bcde1cb18537f34b02a6a271e921fcd44aec30eb0f3da69017fd57b3a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_arc.rs",
      "sha256": "c6b50ac35e93520029884440498e0d17f9c2daf09e82c2b77ed3db66f372d7a1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_circle.rs",
      "sha256": "207decf5247f2fec9b84cf70ff0319e2f646b3af082459825628a098729718d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_ellipse.rs",
      "sha256": "5140c1af03b1e011512258bc60901b04cde23222a780829b0328d0e7518dcce0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_parabola.rs",
      "sha256": "47f3b6a61efdf230fb1d03658b44aacd5268e69d1a359d1cb40d8654d21542b7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_4_points.rs",
      "sha256": "20464c6908c865053e38d7a9eb953e05f00cb02112262296a1fb36817bdf8981",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_loft.rs",
      "sha256": "108be02d75b82e2f5ac6f34c13dd46b638ccee81314bca8ad6051970d16a588f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_network.rs",
      "sha256": "7f94d0f5074736b627f761b7b812ff4abc05592fe9ec4bf142410f2cc9643854",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_revolve.rs",
      "sha256": "b447738bee31531180d016dabf2e77d93e30583c85812485114f7bf34850276c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep1.rs",
      "sha256": "aa18f91aa0a35fb3b552f3bd41bad8e9a2ed565751254adc58b511904b137b84",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep2.rs",
      "sha256": "750b15d8349ce66719eb982dfd85abd590a9f122cbbbf19b676d2ae2b5e04992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/object.rs",
      "sha256": "fcce94137a4f68c8e9c54617caaca0f7ede2828780203cf8413ef5fff76813b8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/octahedron.rs",
      "sha256": "203d8f4f5afb243be5ffcce6abdf3bf379d5abc8519b1c5b12aafaf10cca7f86",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/opacity.rs",
      "sha256": "0db647dcb96fa436c851edb2ef728df150fc8ef0681aac483481f0ff1c286ab8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/open.rs",
      "sha256": "7b71238ac03238c84d483701c161649ecbabe1ed6681025c81408f8862f76b43",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/orient_3_points.rs",
      "sha256": "943bea38cb8607eef2153cbaa1e20260a2a09531bd5d469a2bc525ed35cc9f9d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/outline.rs",
      "sha256": "537e232c2f91846dec99eb3e6e30e016f482577b1ef82a62c4f4f8a115566053",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/point.rs",
      "sha256": "d77b9be0208a33c75ed0134e99ca50815f2613fbbeb728adb33c30ad14306b0a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/polyline.rs",
      "sha256": "15a7f496a783fe8864fbc1a090a1c3ea884a2cedff7320ecf0fb85136c59e93d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/project_to_plane.rs",
      "sha256": "11dc267a1a5d36b974f4723fca487c38c958cc3ac2a0f926e81f99507e6904bb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/pyramid.rs",
      "sha256": "a1de9a501bdaf8b12212a51da89160d5abb6d49f437529ed8582678ba31919cd",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/quad_sphere.rs",
      "sha256": "d801aa84e8d8c513d89483113c3076389b14c67eb613aae40f7b970af13d96ba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/redo.rs",
      "sha256": "cc5ded7fc9d8c23b2e2638d5c48910a2ac6ad83a2815aa2976907d53511eded0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/report.rs",
      "sha256": "8b8e73ef4da70c6221cbec41e75f54db70a5d39df05ee29c19b35efb8afe9a63",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/rotate.rs",
      "sha256": "814e405ba0eac925a1322fc88fa7918e0d436cc291f4ab322cb5ae6dca4911c8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/save.rs",
      "sha256": "d2ed39ab8cc2f6755f5155a75269a1a097988799f161fe174587927e90084ad5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/scale.rs",
      "sha256": "bc3f8280720976d74fd46bd4447da41ed63943c014dc62d2d75fbeada88ac5c2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_all.rs",
      "sha256": "e263949a3a7f78ae516f1e3e4e0f53815a96547f0ffc39c0248cc258fd72ab8a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_by_name.rs",
      "sha256": "90d070cb338b3a721a4501e777bf9ff048cfc368fe5d98fd1facbab18531001c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_lasso.rs",
      "sha256": "dcef1e302ab9bea2353e2f85fa391b35117f6f7f97c153beb7901744a0571a7b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_small.rs",
      "sha256": "d4591b944daf9036df7f6854083adf9fb1f593e410d61ee141f1cdd98ee4ab5e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/selecting.rs",
      "sha256": "49b3172f3487531566fae3e4c955d9a438a0e153cf37d7f29a622bc070a48d27",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/show.rs",
      "sha256": "b55d0061a288fb54d8de3faf65e56df41a9ea98b3f2b353acb1802b9d52c9803",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/snap.rs",
      "sha256": "e85c98cc314cab74c8b9bcb74cf1d32429e49b49c76cf2c83f8a2e929b35603d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/sphere.rs",
      "sha256": "75e2438143ed9c3f6875596c517211e14000f8525376bf166e52372e82f4ef91",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/split.rs",
      "sha256": "8c029c76b77546dd5fb8831693397b48ef16852ed1379253c2a028349f99b54a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/tetrahedron.rs",
      "sha256": "f09e06bacd0c09b5b2c7b8b569c93267a4df5cf0fefab653868d154a0b02bd15",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/text.rs",
      "sha256": "7e0f56386df9eff853a325856005ba2670af975edbd7203e60a3dbb70bd030a7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/torus.rs",
      "sha256": "c64075764f74caf608a647bcfe27aa1537c4b85cc091071cf3728c457f288c08",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim/parts.rs",
      "sha256": "d78d99b59d5da7cf85de9cff7d9b161ac01c3b107d6b50fd0a67966f4f4ba71b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim.rs",
      "sha256": "60c27bce49c24b3867c658171267e1c159c24e0c1293de34067711f2c7c4b72d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/undo.rs",
      "sha256": "6e82fd3e78bf7d00c006740fd49264bb5d83c18957758bec545fab9ca09210aa",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/view.rs",
      "sha256": "a025bf0d864aeb1bfc1c993df44e112118c86e290797a687f22935a442f30e39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/volume.rs",
      "sha256": "310b7d44e53155c80a823aa411e758d3171a1c8b3a3bdbf273113c36430c0ad9",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/coords.rs",
      "sha256": "3b19ba73fee97ed22f48b7d5fe04bfc1090f1c31ca21c44265e936893a0e447c",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cplane.rs",
      "sha256": "4cc6d3d6ddeed8baef51fd8e49ea8809156b9bdb8105b114ad1b906273e3ea26",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/decode.rs",
      "sha256": "b9156c5c746b33a6f4fd8a86a9897ef7c0f2ddadd252a3d2004eb21c404bf8fd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/deform.rs",
      "sha256": "ed009f15ba5ace800914b14e78252b5990d76f001049fa667397ba361633959a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/edit.rs",
      "sha256": "896328d97d69df9750d6f17c738deb0624ca3021e60fb8d468dc4e0fdb94e49e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/feedback.rs",
      "sha256": "951457335d199ede0babce3425fb62466a0e717d12cee30816fd13f749058e53",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fetch.rs",
      "sha256": "57d886ea78208f2bb19ffa13a1a252af18ccfddc7a17cd7c9432a981a1560fed",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fonts.rs",
      "sha256": "d51582cbb4566c5efb172401510ed2d33675a20a0b63f0a54d9218fa7ee14e6a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/box_select.rs",
      "sha256": "f52620890c23afa10a99d046a9985fccaa8b15639c972a46a0fae94a2bf88044",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/control.rs",
      "sha256": "e58ecb9d4da6bda5216ac2693a3d319be7f9fc386f80afb2d6e5f8df012b2f22",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/gizmo.rs",
      "sha256": "6ff5f1d9cbd6eb5830a0e80c46813699ec9a195bfc7f1476c342d064b6bd850c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/mod.rs",
      "sha256": "a592ddc3964fa3b7732c08d9ac9339491980235e3f8273f9bcf75d0c043218e3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/object.rs",
      "sha256": "009f27a8d77f8a767e90187456824324f27fb70d0256d30eb2af9503ad8806c5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gizmo.rs",
      "sha256": "397a00a8a1f4cda1f34ba244a40f5a842f27b1c83d7affc298d0f4276d396340",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/hierarchy.rs",
      "sha256": "2f3698997bebcf6ee66cb4e5a4290b796d309de877674b059b48ef92e2af7a03",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/input.rs",
      "sha256": "8dbcf36cc41b9ccb88a925c86c423396eaa572bd730e23d7b3f8df0e956392c1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection/source_memory.rs",
      "sha256": "0828d7ad8d7b3c0ec05422be79edec175739a05e7f6fe94341d77aa10e12541d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection.rs",
      "sha256": "157ba8e01eb1b68e7be7182fa92be0188ba17cae357ae3a922e43b8860aa0891",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "d0ff1f7f682dc03297e79054202c860598862d6e83db1e7033c858bfbcae5b9c"
    },
    {
      "root": "viewer",
      "path": "src/app/keys.rs",
      "sha256": "28abcbf79f108564e48b4c61ee96b8521b69062d467d2c5c2ad787e6c230289a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/knobs.rs",
      "sha256": "b9dba07b897e6498d0a4210dca1b909365a82f46df32eb1a2e8dfd712889d99a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/layers.rs",
      "sha256": "196e1a664f8d6db2611c1c0d581972741b864f8da80037b84a86c8f299cd4d3d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live.rs",
      "sha256": "c219787e600906040aedf875182a61635b03e64839c3797b6c9a4eddded7c9c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live_revision.rs",
      "sha256": "91680efc998ea6d91d59bf46380fa938b3f1b9b8a3e9d4be7835096a6bb13c60",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/loader.rs",
      "sha256": "61e21d567eba5f336a45f8cae5b6e3d7c51438315cfb447ec73c9ff5604f50f7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/manifest.rs",
      "sha256": "ab020e177195951fa7d07d8f132f93de2b41d23c51d6625d19486658baef8228",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mesh_preview.rs",
      "sha256": "ec41fae6255735322acdb9f9e9cdd3891ba36ed1251b3f438678cc64891c8c4e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mod.rs",
      "sha256": "b74ca6bc7b9a0fc8c0889435b79e9e77de6bab4b953934267587f1c2a4a227fc",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/modeling.rs",
      "sha256": "7aa4202bc1756cee495ce0f2daefe9c2c320e651294322898c03624b761f9e04",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/range_gate.rs",
      "sha256": "7191b2de72b0b5e97074c977abbb9ee6b4d62c383f225c55148913528cb9ce1a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/route.rs",
      "sha256": "009e6ae7e824edebe981be1d31c30dabafb89ae227aec188113f680bfa6dc2b9",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene.rs",
      "sha256": "22e53f5e48fdbfdffb296a266b2c29ae4dd25f309bb2d566a5041cd12bdeb52d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_instances.rs",
      "sha256": "fa216bc9ebadbc25596c0e76711d49b72a9544317b265146f63614362ca39039",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_release.rs",
      "sha256": "b886d822f51516d904726d7ab2380447da7b8dd99f112c87d406ef54732fd95e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_rows.rs",
      "sha256": "d21f7804c6a58979381774fff193155e16b9b96c43c5f2cd0022eb3e4f1c7fb6",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/allocation.rs",
      "sha256": "ecb2566170adffa7450aced660683e5bf6bb5e910f95a3d8161ee4359b29ad13",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/commands_tests.rs",
      "sha256": "ce45af59fc2375833c7292459056175568d73d1f678106f650fccf214ccaa65b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/compaction.rs",
      "sha256": "b7f65a6f4d78543ee866bed81f96ad807df7b154411cef59234c39367bd7d52e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/editing_tests.rs",
      "sha256": "0e8e6b90788ae1d45ddc2f03cbfaf5e807b1e7c9b47e66e97df70bbad8af8826",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/nodes.rs",
      "sha256": "9c70230b1bd744d5c476938d594ae2be256c3b39b815b147707cb05759ddfbd4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/notes.rs",
      "sha256": "fa02d60b45d4f74823d75f6786780f7b24448cdf17664b9fdfa4e7f66f21a3ba",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/panel_tests.rs",
      "sha256": "4beba4f873594d0aa0dc1836720ad18739c0124a13ad0c0884fff5e4095f17c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/preview.rs",
      "sha256": "9df6f4543433ac9da0cdd853ae93ead478a40404a4b824096d22235c23341e8e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/split_tests.rs",
      "sha256": "ef2a718cb61c31593c0d5d255604635ce33ad7a1a639f9e0a408ec21dd9c6b70",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/testing.rs",
      "sha256": "9716dce2aeebcced6e6aa72463950f0f4e1e17471942e23752b00ab52af943fa",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tests.rs",
      "sha256": "30b78b84c2cd80a13cdb33f4cc68bac04645a05596b74216999df4e98a9d22bf",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tombs.rs",
      "sha256": "a5b5d0082c95e04309c1c61f262a315c2eb610999bf471af8c80246a2a76c7d3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync.rs",
      "sha256": "7160ce61ef8ba840ef536bff9532ed315b7c2b613301dc7993b61105411d0126",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_text.rs",
      "sha256": "b519d58bf6b0829e157f338300cf704f7986377cb1091294beb7f4c5e8863c8c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/selection.rs",
      "sha256": "3d635d3895cb8b9446d1b0b6771e6377c625985a599574fef0eee8f56cbe83cb",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/session_io.rs",
      "sha256": "ca46739b5e8d4ff3fe5b5bd22b2d35ab76d0a55c0af3ec2a4ce38ec97c3b87be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/sheet_query.rs",
      "sha256": "9ff2b40923abe62148dbf2ac355ea7f6db00a3a0402a6f2cf1dbaacaa6916ab7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/snap.rs",
      "sha256": "174fe242e49e33d825acecf477d771a2ed4a7a0bdd7120c25938ee90241ec7cd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/splitting.rs",
      "sha256": "50840e7f5750aa42a0e9058427dbea29beac8247b700b69e1fae7885f540b513",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/stream.rs",
      "sha256": "17b125e85e513f0d16ee7bba163a178a41901a9e4205886b9227729ee5203299",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/surface_preview.rs",
      "sha256": "7971672cbd085559288dac7495c9d6b57fe0518fe052c8d95bfa8949d8abe8f5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/touch.rs",
      "sha256": "6e45e9c59952bcfb102bc98ac9cd0c243d042c52a623d330605ef6b1b3ac6460",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line/tests.rs",
      "sha256": "f42ad65f70e9889f3d09e62bab1a04043f7f1d611f49ba7adb835394a3ed52e4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line.rs",
      "sha256": "e1465627dfb1b6afa326bbed8ce025de3ec0daa4ec4f0d34504099475e3d1bc0",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/graph.rs",
      "sha256": "6ed37b504001f80ab29a864567ff78fa8125a997cf4b179d538e8b580c7f11b9",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/layers.rs",
      "sha256": "3ed1b04e06c3b56d6ff39c0880be39c0944dcb4fc5847bce2adb127050da96ba",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/mod.rs",
      "sha256": "e281d789a462716213d8ec44b3faec0f9cca7abd65cdb6c3b1306895f9d97c59",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/number_box.rs",
      "sha256": "d38b57f74e9bb3ea749111bff86d8ef67fab2bcdd1293664c487bab08619b7e7",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/overlay.rs",
      "sha256": "99e66230fe357a9922134146603afb7043628bff4a0a4e69b16fe52863812739",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/phone.rs",
      "sha256": "40633a8dc3394a6898d30d324b19912f7002505720777a94a5d3fd64a3d32182",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/pointer.rs",
      "sha256": "7172927a19316efaa813fff3b3963d68fb5228e968f2c6e9ee55bb26cd60323e",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/theme.rs",
      "sha256": "a45673d3ff48b2114846750a22e2ced36f22dbfd7b1e11752914ecbd54b71f3d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/validate.rs",
      "sha256": "c7e6e4fbe33ea7af6898a18b92758df01f46cb43ea2aacd09d5da346a57623be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/bounds.rs",
      "sha256": "257a8f622f4ba277e235ae73c69e59b14c8a6cc2ffc52b9ed07076e25868991b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep.rs",
      "sha256": "e49dcc0911686b0f2f4fbd7c9aacc411bb989e45b5c0c1444544f087d886c285",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_edges.rs",
      "sha256": "2a19dd35e93bf62103e9549fc16f9c9d0fe6859a68be424b970d60dea17b5d2d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_orient.rs",
      "sha256": "0778910bea71d83e47737d5d35100b38f18df8b9dca3d886f642b36125260638",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_shapes.rs",
      "sha256": "c1621011e4006473015572ec811300241204838db2b96d4372b84a740e6fae2c",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/cloud.rs",
      "sha256": "813e12a5731d575101f15a1f18c5d5db2759ab400ba0d8e1701b7047f37207ce",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/curves.rs",
      "sha256": "4da38304e6eb4dbed57d0406fe36a9dc4da823eb4f044d4434160983792756d7",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/encode.rs",
      "sha256": "a925576401ada960e52a28d669f7bee634c5c4213cc74207846c3a5a22af3a83",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/frames.rs",
      "sha256": "1bf83056cc5e36a94514c86ab43ef13d7a10ad9b9de62cfc8df847e3dbd33d8f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh.rs",
      "sha256": "a070c4a31a303339687adfffd579446e31f33d44c4a3a5c4ca49cd216265e060",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_ink.rs",
      "sha256": "349f85de9cb23dcb928a80b520468c58e84c4a03a6fd6d60280d2ec10a85cef4",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_topology.rs",
      "sha256": "9a207b8d0152661e846568bbf5971c3c24fdf5b33e3d5bf4acf6b5e925846ddc",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mod.rs",
      "sha256": "048bf6d1a2e1b67948b011d5e3ede466e32722991e8c765097308de070fea88b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/plane.rs",
      "sha256": "9be8bd22c56cb84393584e3fa969723563805be0350b49db1a9aea04ec58c30d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/points.rs",
      "sha256": "af7baa2ff846c8ede26274635f105bd34377cf2aa63576424ff53da843d38330",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/sheet.rs",
      "sha256": "c5148556dd5a0ff2688c63316b89c53ac79fbb747bd8d70de706ed2fbbff115f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/camera.rs",
      "sha256": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/mod.rs",
      "sha256": "c41184b767cb4947bf33bfe8ded268a6d3c7185870a6fa71054a588c002a90d4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/theme.rs",
      "sha256": "2064473cf413d0be10ca1f71440db34e11912c30893a360f11e446a5ffa2099d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/view.rs",
      "sha256": "2b6c7b55d1257a7be5736edd22e69194655b6e14d4dfbffe5c1c01e86456f204",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ambient_warm.rs",
      "sha256": "94bb4b260a7875b4c82be81acd8131e3cee556020f3cc5145608ec33d6f52585",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/arena.rs",
      "sha256": "7f0d4e53f55d3a1bf384da7ac7781766b8faf4e9f7038ab0dc2c29bb7f6e6441",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/backdrop.rs",
      "sha256": "afb51eaef8ce003e22ab9b4ce0e0af2c1ee786c98b98ceb22992f2a3edf4d6bc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/buffers.rs",
      "sha256": "f226f75be1899a1540a4f80d4491019aa873aeed811dc73d0573405927a13f2c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/editing_tests.rs",
      "sha256": "ad0a23dda8ba3b14a7f55db51001d3bd61d24ec51cbea0552babcce6e206044d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/pipelines.rs",
      "sha256": "483249eef637d0deff02d1e80d96ad5ab56247e18e76faeff2fdc26cd4fdab73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/tests.rs",
      "sha256": "9f7a926a7e0e57d623a226d9baf781287d0ad6da3c4b0c9108d2612fcdd8d691",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip.rs",
      "sha256": "e0b31a38015a8c5bc31c404261ae46757e0655774413de0fd5ab92bb2282d99a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/cloud.rs",
      "sha256": "ebf520f190ed3924efa465cdeac6d743ee9c2f1db337067955ba08d96959e3a5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/device.rs",
      "sha256": "6d8d81150175551c886ba7dc412b384c2ef9be30fbc9ffc3fff53cbff6f67a1c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/faces.rs",
      "sha256": "b48b373d830c28cb9e9456e421390d7f326b4910c9608f487dcf1834be78ff71",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/frame.rs",
      "sha256": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/glyphs.rs",
      "sha256": "f992838e16e6792174214bd4ab67040e33026b9d471fad5b988ebd0597716970",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/hull.rs",
      "sha256": "42571ee70a810306267f2f1574e593819231f5633e245dc64f5460d5ff8679ad",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instance.rs",
      "sha256": "89198da7abf06d41040aae716580d539eab2eed5b4e68763f348c5b7de31bd70",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instanced.rs",
      "sha256": "f6fe9bb471dcf30fcc81534c6b20ebfbc1755c9cb0cea0edadf6272ab9e673db",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lane.rs",
      "sha256": "2e16f454ed7cbc29a0422d180ec3eb8aa9adbe7503f7425eadf24765dcfea838",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lod.rs",
      "sha256": "75dc349e43f44ca3bbbcc26a88f7bc3d56189a4268b0a2e3523e7903e9148a08",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/mod.rs",
      "sha256": "994339d5f4eb05eaee30be2c0352c7033d4b686afd6cad0c1e2ea84204d06035",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/objects.rs",
      "sha256": "58abc890347f29a8d3deb9b7651e3a43b4444a1e1429bf3de830ceb54516e7d5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pacing.rs",
      "sha256": "9e5916950f6658ad02b28aae1e009cfdc4546c4a4c4ba24c59c9194a7675b427",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pass.rs",
      "sha256": "cce6cc842567b136d01ab84d8ef1e39a5b149063128a5df932babf6950c80dc6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/patch.rs",
      "sha256": "00d8fae175ca925c7e48b8304c702550b1633f56747b4130ef22d6218cd97100",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pick.rs",
      "sha256": "8dae0bfee0fd18e17ab90b1c0a61cb028af4d87bac29badb72a08c0325ea65f0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/planes.rs",
      "sha256": "f0e509a16d84aa675ffd8c6b335a2a4f6074f120690881e4b1eae202a22e1f61",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/present.rs",
      "sha256": "ff2820ea362b9fad33463c03007deb90a72f22bf84f68f9c53356f31b7e61f79",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/render.rs",
      "sha256": "48150fc2c3afa9a453af5fa0d0aa35768ed42ee57454a3c46ef0016a518dfabc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/segments.rs",
      "sha256": "e0d9532d31ce5ffd592eff3c0ca47300e81aca229408fd39a7d248176118ad2a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/slots.rs",
      "sha256": "ef303f3f3e534502a9d90561955d840c091dcf9b2f1f69fea4f5540fbbfc18fd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/splat.rs",
      "sha256": "13c99ea37920f0dd4c39750deba7e57d274268b366c3cd56d2a938dec113fa3e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/pipelines.rs",
      "sha256": "e1c9ee95daa6fa83ae924519bd7f07a759e3f8d00c939f3dd1bce749ded218f3",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/tests.rs",
      "sha256": "474fdf5ec95cab2ef4f6a6d2d19528c344e1f3abb4f94cec8ff7ce442fb2e5ae",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao.rs",
      "sha256": "932d34fde6d1d92a37c449e9f3b7c921e3aed63308c7427ac72e080c6c909830",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline/tests.rs",
      "sha256": "d669e0da39d775ce45b86a1b01b54f8969b288b92a7cdb76b2941575f8f8940d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline.rs",
      "sha256": "08e790732dcd0ff16abe734be9d13b63dc26e6825f9d1d7578645ee529e4db03",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/targets.rs",
      "sha256": "f16908f3fd299e876687d0894374a440820ed713d3adc57689765f9991d2f140",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text.rs",
      "sha256": "ac22022ad4c3820c2a834ce060286094f24d7a0a7a30633eac4a26231cfa054a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_outline.rs",
      "sha256": "8444903f4b6c8132375822bd8d95eb5e06b51e023cb3487a730e90c5b3b842a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plane.rs",
      "sha256": "641063fe094b466de058a601013f47e3227955eb63ccc0d42564f1b0b8a6063c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plate.rs",
      "sha256": "b7ec607e1759845577f9c10596ebfea2377bef4f3777ced791f0592f6a71a9c1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/timing.rs",
      "sha256": "9165356ffbe6773316ddcfa7bd4e68c7e7ecd36892eba36a7dcb9d8760ef8ccc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/triangle_tiles.rs",
      "sha256": "9e337004028bc94be8f79c5007ad7d04711967c1ea5d093fa6b3cb882123e2c9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ui.rs",
      "sha256": "a74d2783ebbb7404e496ceeb01d62671f48018e3173837c66e0f83720cadb263",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload.rs",
      "sha256": "d8a88e7f7fcd03c316981bdaddee78ec6f6986be87e69bcf0f298c9b172dab5f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload_padding.rs",
      "sha256": "f713558ae631388ee310c7d6e3e9178eba12f383111cf2fba5f9ccd184fa1f52",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/vectors.rs",
      "sha256": "539d407e8e529d9cd9d4c507f675221805506221a49177773571e51e02d7fa3d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/view.rs",
      "sha256": "dec8a12c4a27401682d4a506590d107ffe762f1253af19774409e8d32e3879f1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget.rs",
      "sha256": "7227605387d824dd13210d70101a818ed95f9525a000bca1dc796dd7d21d0104",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget_mesh.rs",
      "sha256": "89d73dd598c5a0f7639409e724f3df910c09f3ffcf21de16f0cbbd8e669c7306",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/mod.rs",
      "sha256": "f9d05dbaaa0d9b2f6289a368e9c06657159eb8d9a0880936acaa682f736d97d9",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/engine/performance.rs",
      "sha256": "de679efdb2e8adf6e257324ce37d6026649654d7d4c4a7ab38dcf1b46d5bd5d8",
      "topic": "Frame pacing and performance",
      "lessons": "95"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/bindings.rs",
      "sha256": "7799e0467c095eea09730a0fc7c34847dfd8409dc1280ddda0bf171d35a77509",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/layouts.rs",
      "sha256": "2152f1a6ab2fe4fd17e7d3a3eea01606dcab2a6b80ba69bf0760a216d42d3f45",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/mod.rs",
      "sha256": "50590524bc796a7839dbf7fc075ad278909b53050fdb47c6e721d5d9f6c1bee5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/text.rs",
      "sha256": "918ec5b3bcc2a775c81e348d25b943299bc19d2d6ffd9b7ca647c10c9299591c",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/lib.rs",
      "sha256": "12a3ae5e9c08583ed1088714889ba96bebd7fde315d0e4c0370692a2b12457ab",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/lifecycle.rs",
      "sha256": "01744a1500c73dbeb25e4caa46aec8c82ee240ba706b76895363169dd864991a",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/oracle.rs",
      "sha256": "65e4c530ed8d341c7f1a4bbc5f2691861945e4828bb7056dc74e7202177455f3",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/profile.rs",
      "sha256": "6c8cfa7be644e9e527c1b4a289ca8d2afb10075843207e7ef15415a56b1d6bb1",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest.rs",
      "sha256": "dec37fbc978002c1184181627769ac0c6daeec91da64b13373ef1d33bdaa08fc",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_composite.wgsl",
      "sha256": "15eab6b52a9dff22c38cb505ff7462b5b4cd6bdbe3e5223716902a231b9dd6d8",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_depth.wgsl",
      "sha256": "80258c80084418bc9211c6c31438cc52d0944509a23418f9e9af4a7765d84cac",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_geometry.wgsl",
      "sha256": "3168319a5398a270b4c89d13342e16bb2f4353f97fce0341a2d500728f9868ee",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/arrow.wgsl",
      "sha256": "73cd0ec57df2490b9b3099c56fbb9930c4667f8eb55080a400ca98b4e8c0b2a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/background.wgsl",
      "sha256": "ea62408626759db7cf4a3756d83dbbd5aead2621aeee028f4c54de4310ef2aa6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/cap.wgsl",
      "sha256": "1892c0314d7ed98949f6f56711e2cb002ec29b2c1c1c46ff9c4b31357ef7bde9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/clip.wgsl",
      "sha256": "56aae3cacb18a7c036e05833264577d8ee181f29c25af56d2f96ac2582df01d1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/face_coverage.wgsl",
      "sha256": "e4b37b7fa30f5cf6237a0aa7e20287ad36b3cdd07893e43899e2bc33fb1ddd16",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/glyph.wgsl",
      "sha256": "7a1ed8abcaa5ab684ebe31cea2fbece63df6248057aed2e84d0ce8e9cc5e4471",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/grid.wgsl",
      "sha256": "67daedfc89ed5b5553d2061429a4d725d1bc12b7f8e503d04bf1d709befc21fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ink_visibility.wgsl",
      "sha256": "3423dfccfef9d924b4329d0b973554dc47bb77b7f7725c9a045c5181398a055e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/normals.wgsl",
      "sha256": "1776ddecbef8a7ca5454216697fb4d4aff07e631ea3452a885b6a47df8056733",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/physical.wgsl",
      "sha256": "197265d033815c44654368907825505fd55b03ee29cbb3ffa7bf04d3a4dd6c35",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/plane.wgsl",
      "sha256": "e9934e566c2cdb31ce648cd0f221f1ba7728df130f15fe8e69b8eba44d452312",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/project_triangles.wgsl",
      "sha256": "60f8ca735f51e5b977b0d977f55cf7f71fe5b841409ea4765de8dc6687867224",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/projected_triangle.wgsl",
      "sha256": "3e569eabf25978668964bec693e0e250d8e634900fdd4d14808db6338a815b73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ribbon.wgsl",
      "sha256": "f19427f01f8eeda0bd8200457f326717d972eaa4c8bc349c339c45243300f617",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scan_triangle_tiles.wgsl",
      "sha256": "d04d5a98009c0d253b4e5ef5c5751b8ee48239878e800753eb99e5f856e02ed0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scene.wgsl",
      "sha256": "e17b1d992b7c986e48abececae70cb1be1edc356125a0ae1f2805cd6a6a73281",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/slot_table.wgsl",
      "sha256": "0837fcc793e41b6706c3e00b7dde58972fe4fa9364d3227c8f1f5a193a17fdcc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/sphere.wgsl",
      "sha256": "4a4bbdb2853a6525beab037611cb9b2135ce42ad881154b8ab1d46c8e2c24a4a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat.wgsl",
      "sha256": "38fa50b2d6aedcf8dd923f534a21b6f05fc4a94c32b6855e2d2ecd6b3559eda4",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat_resolve.wgsl",
      "sha256": "529a0f871e59615087bce401df552a09243406da12240c3c84c64663a9decc0a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ssao.wgsl",
      "sha256": "abc35b0b2547741b4d092f9778b74932a2e035b23125d7feddffe19a71bc765f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/surface_outline.wgsl",
      "sha256": "446689ea45e5cf0fd03011250271a296eddf4942ad9e0dfd4432c8b57b089885",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_outline.wgsl",
      "sha256": "c06b165b85ee2a0bd20d0348f1aea3d1ce811a7fd76d66911380ef62fa8be03e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plane.wgsl",
      "sha256": "56c0dcf2431ff25d546d3b46dd6038823035744ac939c3895d7fcbe372073248",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plate.wgsl",
      "sha256": "5721973099036446bcd3310cb4ea28eb522b109009236602f4c00d757900b4cd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle.wgsl",
      "sha256": "ea26805ce07269fb242562ea71dec4fee114794d5cf24d385c2f1c2e9516960b",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle_tiles.wgsl",
      "sha256": "21bf7777d18bf7e506f97c0a31d4ab5a64bf6e062ffe0698dd67402806876c0f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/vector.wgsl",
      "sha256": "b67a636f91ab079c58e80361c5ed8782764acd47e133045a94c50f412b72030a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/widget.wgsl",
      "sha256": "4337efb0b24e1fa1116db7dc9fb6690e3745433168a704ff7edd71597416f8d0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/state/box_select.rs",
      "sha256": "3b0fda742185f5e1fac8836aa094707ccf7a66cba115300b52e1b40520c5c07d",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/clipping.rs",
      "sha256": "0b49d749ae19ee39f8adf3a26607b3743b6e0ce7958eef7b86f56afba36d2148",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/cloud_query.rs",
      "sha256": "5aa517e86ec917b672f38e4feb4e698c19addf6dbcd498987e777eb9c8ada999",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drag.rs",
      "sha256": "e94d90344c4cc68710cf5abcb2cdffc31aca7d7a53d0f3f49e093b0380a2e061",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drawing.rs",
      "sha256": "002139c15068a0b99bf58f4e98c06ab76d639a9632e387e221b26ce6b403cebf",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/edit.rs",
      "sha256": "ae99d6455cb03aacc2402cb5d8a6c292894add723f88453d145cd4a27bf866bd",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/features.rs",
      "sha256": "346212c4f10751e092ab484d2e91988454c832fb39d1751d1cc365aa6ff86a53",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/hydrate.rs",
      "sha256": "5425a603e63bc9090e2f9e7cbc8e16afff75aa6a9382295fe1911b95a90e6b5f",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/number_box.rs",
      "sha256": "c32afcd959720c2010f60127f2be1315f468f502146935d39468e9fa5f88b9c1",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/panel.rs",
      "sha256": "3523ff5721284e190ebbe6b35c68851151a017e5a7af90ae76ac0ef467cf0bee",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/sheet_query.rs",
      "sha256": "fc2d5fff6c5e5d794ee31d4f438e71b87e1043f45b0c36439f837d7f0691d038",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/splitting.rs",
      "sha256": "a8d8586f1b203bc82040a37b1d52b314fd3f03a9f1a5abb766999ea76d6f4924",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/text.rs",
      "sha256": "48ea023bdaee730f0d55bff43ef28937d6370a973bbd0819b5e484c66f7ced68",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/tool.rs",
      "sha256": "044fd2c8f01f1ce7995d89f04022de261609f9e65e552b4d3485c74f190c2889",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state.rs",
      "sha256": "48b6e5f4c403d3b89a3f6ad691b28e815b8fce8c905f0e363aa1e3e346555cbb",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/text_quality.rs",
      "sha256": "7fef20819989ace4919f20705cdda904656fe960ea557028e9a5dc7b34b68edc",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.lock",
      "sha256": "19a5db84f182cf5eb18e4ebc991abee9cda016caa7cecee98e1d30b680d961c5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.toml",
      "sha256": "07eee3412699f3b83805b845b7e8f1bf09550db774a2de28df399176f2c9a46b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "build.rs",
      "sha256": "cd8d703eca80af4ef136c63956bd97202178a8a86910e5abdfbb0d5fe1f428ce",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb.rs",
      "sha256": "c23fa5f21df2587c9bec613810599bd4f6dd5992e206e79531f1ede5d08710bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb_test.rs",
      "sha256": "29307cbe6191102d62ec23f82c5b85e3c518b4c6ce98fa8cb1971d04cda473a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/minitest.rs",
      "sha256": "c262cab354a0c2150ebd78880a48f0fbf854556a04ffa8143046835ea303be80",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/pdf_import.rs",
      "sha256": "834253bd8462d1fed0d9f16a2ce1391f9c8422d5a469ef0583f7107e9e85fb9f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline.rs",
      "sha256": "bad8ebaefeecd46ebac35b33d54ecc9320191d5e6b03c4516ee316c61dca4f87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline_test.rs",
      "sha256": "df6abe260a9b1422b32e6c84e33c8f87276c6733b0932d1ece4f7b90fbfced8c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep.rs",
      "sha256": "2eb96d33e16a294c197a63e8ee5d65759d8320394232b5fcabd49d0b02c5b84a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep_test.rs",
      "sha256": "f074d549288699afb223a31ce63a4eee05f9270829e40cd860458ff2fd18f3a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest.rs",
      "sha256": "95f4e14a0421603076ce8f6bc4ba448a255a63ca46ca461ea6f1e69c52feba4d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest_test.rs",
      "sha256": "cfa7376f74f4024f298e1c179bda6d5f8bde1caaa43a6722c31ec6809d58256b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection.rs",
      "sha256": "b01397286ac7a827347580bba1ff884fd844d1c4cf502cc6c97e005ee8d7e617",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection_test.rs",
      "sha256": "88cb7eca30b51939868e842fe8ae000a547716fc2b503eb379d9f6d8006717f4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color.rs",
      "sha256": "cd560ba1d5b30fe4d2a5a89884117251e482fe08d61c7a63b6f135f270f8c3e8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color_test.rs",
      "sha256": "31ea2af6afd5d9590359c63a7498a624b6818b8d38c06506a74b41cce29bdd7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull.rs",
      "sha256": "489528349623b9d49242e632e537e6956341f91a065a6f7b3ccb66d9fa3de541",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull_test.rs",
      "sha256": "f97a384db885e6e12421fb349c6ee15f78a3b1eac13be6189d370327ccab0d01",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element.rs",
      "sha256": "09ebffbf70831adf2bab9e1a25c61856ae12c14029cae22c71845db7f6b197f0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element_test.rs",
      "sha256": "94eb3a53a8ee5fd5324026a3b0111871633235d053303313f2322bc6e650d0b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders.rs",
      "sha256": "d1ca6e177ced274a6ed795e61f8fdcab52c78b5143d7a4b3e88daa096cedcb7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders_test.rs",
      "sha256": "b6127b52a4dc1ef11b7b6b6d047e3a1d5ae63fc2b934577457deec450d697d2c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj.rs",
      "sha256": "ad31bfe1e318f33d1351a75862898b113f50609eac72801f5b8b6b2effe7f2b3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj_test.rs",
      "sha256": "7484d9dcebc1a708be7895b49bb5c8d0712ece848835e80b54665376cdac221d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step.rs",
      "sha256": "20148416d9815d0ce5170f842c6de4f2a81f9ea10002eed207a418cd39a7f5ed",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step_test.rs",
      "sha256": "72638e3a27de36f4b8d95eb44be8a5faa146575dd4c08ef513441fecfe3c4000",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz.rs",
      "sha256": "227068f507310e6ed8abe2c722073ce51155387f48c2c804c90f9bcacf65bcd4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz_test.rs",
      "sha256": "b1c09fde3b71627a80929ffdbd8c7ba55440c90be67c8a4d66d0ccc44d39c140",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph.rs",
      "sha256": "d3be20d9978ccc02b7789f94d33cc73eaf612c5b7a2897eb1f22be1259e315e2",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph_test.rs",
      "sha256": "04c8ed991cce02fc7962828b8b0d2f8f85ba0433da32fbbc9591e7240e871f83",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/guid_serde.rs",
      "sha256": "0dda61bc6a049dbbcd077e22e1b28ac0bea66ff9fa8f0f3359f64d055c071310",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history.rs",
      "sha256": "f2df00f9e8f1ae0c2e35fba779849f5d4cd2efdd9faae8f52a444943d6012ba3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history_test.rs",
      "sha256": "ebd6ba188af75ef60dfabc4b3da65a98a2cc69925736fca03c1c35e2d805dd5c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref.rs",
      "sha256": "1a7b50e2125cdeaa5fe2f93784b2b5e041a46601124f61ebe36c61e494d35d25",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref_test.rs",
      "sha256": "02c668632f02f2a711296ab67d67fd137100b5fa28996328fa93b642b014c831",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction.rs",
      "sha256": "e1747cd3b610bdeb8f3b4e258e31942b518ee1f78daa75d1c0c4b213b63fc048",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction_test.rs",
      "sha256": "af845f98e0d67ebb03239f9c8aa3812c1106905dc1cd3e177782ba7ed466351e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection.rs",
      "sha256": "4abfa2bc740b4dc2763fc3e40189a60c909bf16100df5d027d39464fd7beeb66",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection_test.rs",
      "sha256": "3bd6fb198b14c92b63dc611694f4e61d26791f7a266c1ad09dd703ff8b2a4ada",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/lib.rs",
      "sha256": "51f6a8c03945f43f639b4827a1cde307b929e2cc1f9dabc63fee45a9e327b1ec",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line.rs",
      "sha256": "1b052733b7544605fc92a4a4a3d12b539fed176b069c003b7c29137214d3119b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line_test.rs",
      "sha256": "610aed482852c277f906b85d4bf5e2969b1f8f281e8beb2da863a73eedec1185",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/main.rs",
      "sha256": "c97d546a386a35c2bc0858eb24661bf8897035894a1858fb187dc8fb71165e87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix.rs",
      "sha256": "7f4c6bf3570a3c88da6f0ac3a59325a0902e4d6d07dbd0742faf9739f396cf73",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix_test.rs",
      "sha256": "c74b06f07ac71dc0e9e5080eb6994dc2e6fc4bb91e4e93952cba304dd494bc5e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh.rs",
      "sha256": "9e58e7e17cb2429a4a4cdff237558356103c85954baf4764691c6096e91833bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh_test.rs",
      "sha256": "24750286db18ea92e5d0bb8cac3138c9ebb3c4655c2fd32c9cef77901fdaae50",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mini_test.rs",
      "sha256": "eea605bbd1ca0c869944aa86e9eaf1b8e8c9bb5a9bf58d8a8a58283242119e93",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve.rs",
      "sha256": "ae7e12cf67da47f8f674558ce2870868123d979e58b3fe099a8e2fc581b0df14",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve_test.rs",
      "sha256": "94c5ae70bbbe5922746b0ec4de1da3b9403a254638e9ddd0f3705a04e695d317",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot.rs",
      "sha256": "c24be2250ab44b64b4b9f37afb0f4650f2e911dc8b084b0dc67a720bbc45243d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot_test.rs",
      "sha256": "3d51a23e3e4144dd81e59228d30781c3d6be879627b4160e587f7d5e657fa150",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface.rs",
      "sha256": "24da855f532214fe06c7c8f48e411b00806a6d7c9e9392b256563cbdf8d0c19a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_test.rs",
      "sha256": "ca096628ab4ea15c65d82de436596220b24d4b8cdd4c92c74df1792ad3a7a622",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed.rs",
      "sha256": "a2a9ae122c047df3b2f926c1de9794b0e3a93bf83d316673ff8877d6e0e73661",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed_test.rs",
      "sha256": "6fbc0f6e39df5f283e21205bc6836e2981119fca00965503855d86c150ab2ad0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb.rs",
      "sha256": "dc230125ed4dfd0c273092d5ed701fbffd52e1290ac93e3c27539a5949ad23ca",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb_test.rs",
      "sha256": "5e568654f05fe24f8b789a69d57c283e6b0eeeecbe5c543479fc4fda61989a63",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects.rs",
      "sha256": "5d2bdaa6924e69c2723bfb811273e00002ae78c04fecd03d33abe1f66412918e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects_test.rs",
      "sha256": "43acf66ccad9c77eef360b04f499dd44f37d6f5fad06ab473a8ca2b98562de48",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf.rs",
      "sha256": "498e731dd7d1663688025254207c1d80df4b61694b9c276853816e2f79b26ec5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf_test.rs",
      "sha256": "e1a204d8687d1972c206037205350056ff2ad1f6ad0d101df79144adc218e716",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/picking_test.rs",
      "sha256": "5ce0445f4f40fb24779838a61f806ed2631f134a6c63beb0551b5981d27512b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane.rs",
      "sha256": "851c2a4041284507ea87e72005f6e1279070f2aada4870b09c1c770b957dcbd1",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane_test.rs",
      "sha256": "6d7ba45bd35be4b12f6ef2e838a685b37df302eb9816cc8e2b6cf9e3a5af10fb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point.rs",
      "sha256": "b34781c548eddbe08aaaf07c3bb90e5206edd8424cfed988ad7df02ec42bc04b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point_test.rs",
      "sha256": "8e6257e9d626fd71cebd96d29f89aff7864b828ca0defd050e70b37f086e3d60",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud.rs",
      "sha256": "0cb8b3ab79e7e9947a583ae47cd498a00a63d01ba605df5c8b1fc9532cc68b24",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud_test.rs",
      "sha256": "4bedbc144dc3a891e64f8a63b57fc658a497894ad66850781806cac839421689",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline.rs",
      "sha256": "7d60104324a69a636d6b06e5478fd24c92be8704f02792672a2cef795dafb245",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline_test.rs",
      "sha256": "262d7dfac7bc9ad57aef36572864fee97481bcbbbdde7a433ea799a001c713ce",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives.rs",
      "sha256": "cf102029fc6b3d6f5d05de9f495334e1ccd862b592bb2e70c7c1617103d8d871",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives_test.rs",
      "sha256": "fd13401c77292c0d9cc62b736e1f43dbf948f1e6d87b8c00d2f11c3f8402e8d5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/.gitattributes",
      "sha256": "a79691a93b46e49ce460c26ef22afcc03d6eca1e63bf2edbc20e96159510f6c9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/session_proto.rs",
      "sha256": "8d56e772d9d742137014857cfc8da51c47f63c86c1423b31f9022fd0e1a557ff",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion.rs",
      "sha256": "a27375664e0e418324308ccbaca754256d1f11346786b00fcfc74265e1b1c25d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion_test.rs",
      "sha256": "66a067fd4acf7179f10f9a3d9cacf8c7ff8765384537a46bac43ff2bf8c6d998",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt.rs",
      "sha256": "e9e366c91c1ed06d0382bd73b63f357df7e3d17f3e558c3167771f4358c59c47",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt_test.rs",
      "sha256": "dcaeae17bfea503bb88a203aadec5003350d18f7334ca77b35060aa75c880e27",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive.rs",
      "sha256": "7bff923021c72c65d4baa153952683bf8dca5a5f3d783838d0576d9dc1fe1084",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive_test.rs",
      "sha256": "721823cdb23029a96f3ca1a363f2d9677d2434d0b3878ccf684c2d7bc50a38d8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid.rs",
      "sha256": "5d059020f5b0c604c71c019bd07f09ecfe970d51d69054fb5ffb939b98fad453",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid_test.rs",
      "sha256": "0fb60e49e2fafbaebc6a1d6784958a260aceb3496617f762793709052209ef6f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/render_mesh.rs",
      "sha256": "0ff23f70c2d793f13a33bad1e87a090269db57c88d5bd17be35380bc642c9dab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session.rs",
      "sha256": "0e79f5a0cd79f5091c046c65bab1716a6b14aa17a6cbfcb5481798af00a2a6ff",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config.rs",
      "sha256": "68f3f4d11d562a81a66a38ba27e7cf620d1ae8793e8cd5d1cb91e2478fe0fd5f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config_test.rs",
      "sha256": "fca94b4145b8260f51250d78116eb8e3b91188b094c0553551858cd1fc78c004",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_test.rs",
      "sha256": "b9dd833467772fcdfdd352d4e60a880cd47727eee232cdd4b4d14d749ca05928",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree.rs",
      "sha256": "1abe0e715da8389a3463a8335f3b5a29b62b1105b3e42c3fd03e6017d24a8872",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree_test.rs",
      "sha256": "f70954ccac3ea600aead350f0733283421d21595a87bcbde19f96155a9a66506",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh.rs",
      "sha256": "d73869d86c4c339fdd36f58d4f92517609adfc0052102f53b09f49735d4d16d6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh_test.rs",
      "sha256": "a30af59a56e8adc06bff7fade212dd8e1c7bae6be7f52ddefe731c6dc301649a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree.rs",
      "sha256": "20184b911763cf1c531e7f4240f5dcaedcf2dc1c75efc183316c8385149f3b59",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree_test.rs",
      "sha256": "bfb31c7bab8a7d50ee05831078e76b9627fbde740b581d38ee68cbf91cc47924",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree.rs",
      "sha256": "99ad88d205dc2ba61d2e9ce71002d4ffc7ea4cff9d20b833137e94f35e678b00",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree_test.rs",
      "sha256": "68b23ec3f391ab3a30f44ec2eb98453ffba12dc72a2f066469bfa111fd6abf8e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree.rs",
      "sha256": "c5c3fff7576eb0df92d264f0f5258fe0781e031377649ed35f6f52fc5e852384",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree_test.rs",
      "sha256": "ed1cbf26a626b0780c3d65ed8d50197f170ad7b94dbdd7fde1f46895c0e7b762",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split.rs",
      "sha256": "9ab9e25fce719171842595c16ec27e992acc082da5a07103f3bf660fec93b94a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split_test.rs",
      "sha256": "22c07ef52e5323abe8bd51c80909d6f0c434ea3e3d97d25dd0498ff115fdd474",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance.rs",
      "sha256": "d5dab04656fbddb2a5dd47b670d172f53a316f9de3799cd214e7a2205bb407ab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance_test.rs",
      "sha256": "e8c87898cc2413066003ce96b12eb9fc4beb979e34b31bdc2a285b66a39330a6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree.rs",
      "sha256": "f353b0f1ede8a33828dc36c57441f0dc4f49d105c85312f509026c5ab2b28756",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree_test.rs",
      "sha256": "6896501557b233f6393a67a9f0a2b99b200ab9da7e51c63845270d4ff155731e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector.rs",
      "sha256": "972b1be360e8073f1da6b974b7e35d45a6a8d76afd579407d39682d6920ca43a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector_test.rs",
      "sha256": "57b7621e012b92917102553dd9db0571eeb1269c63379e70879001e968b33e16",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform.rs",
      "sha256": "fa68b59f28584e5266b61e0d547773487bfca509de67d3a3d1c76eb18d362b5b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform_test.rs",
      "sha256": "192c281556e2c44bbd1ec567e6c6477635ad6b22954952cf637405171d5389fa",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "wood",
      "path": "Cargo.lock",
      "sha256": "61e22d4c3026e1d3a7a65fb5a5ddacdaeaca36b76a6cdba5455c6018b5b75689",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "Cargo.toml",
      "sha256": "6319241ba295b453fa0812f3a4b123b62523a46f1dfcb9adcb0a3c408626320d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "build.rs",
      "sha256": "9e5a81d2d6f6df9b812b7372c6b8d2696b21c30de8439ea950e19b8a2d60def4",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam.rs",
      "sha256": "ba841fe3c55eae7cebf5a10dd50e0f55c77a9bc43947196422a6585e72afc296",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam_variable.rs",
      "sha256": "dc203e3737fea4969482bc183f71be77d7f924323b47c8e0c4e5087046c0831d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/block.rs",
      "sha256": "fbd810e4fe5a98d9fe706f22c27c8af2ee21072e1c965d97c8c92f1a09479e4d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/column.rs",
      "sha256": "e44ea0467821c7036eadd34c59d0f9a8a8a02b34d74d9f15cc996412a926927d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/element.rs",
      "sha256": "0fae5a4eddf930a86f1ea7ae21d96d04488b38bce0399ed3b9f41aca19a11a8c",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/geometry.rs",
      "sha256": "19dd88c0840ec311b8f4d7561697ecb5143d8992b6dad74766b8e38d6a6f8490",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/lib.rs",
      "sha256": "e0f0fd7d599eeb27168e25083946496a38e5057e96f3797b6e088dbfb0694cdd",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/plate.rs",
      "sha256": "59058e25eca287d11e42dc065a4f1830d32958c4dbd11de29c23fd4b28a3c03e",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/proto/wood_proto.rs",
      "sha256": "916d5994429e36d418e863b721f9c1bfb8929316ab17e3af945a504e40785661",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/support.rs",
      "sha256": "9af8c322a9c8b396105b2b43104c9634950279a96ad7dcb76b04a7b86650fb60",
      "topic": "Element builders and features",
      "lessons": "92,96"
    }
  ],
  "captured": "2026-10-11"
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-04-current_inventory.json"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/source_audit.rs`

Verify valid UTF-8 source text with CRLF normalized to LF. Hash invalid UTF-8 as raw bytes; preserve exact file-set and SHA comparisons.

<details>
<summary>Locate the existing block</summary>

```text
#[derive(serde::Deserialize)]
pub struct FileRecord{pub root:String,pub path:String,pub sha256:String}
#[derive(serde::Deserialize)]
pub struct Snapshot{pub session_head:String,pub wood_head:String,pub files:Vec<FileRecord>}

pub fn recorded()->Snapshot{serde_json::from_str(include_str!("current_inventory.json")).expect("Supplied current source inventory")}

#[cfg(not(target_arch="wasm32"))]
pub fn verify(viewer:&std::path::Path,snapshot:&Snapshot)->Result<(),String>{
    use std::{collections::BTreeSet,path::{Path,PathBuf}};
    fn files(directory:&Path,prefix:&str,out:&mut BTreeSet<(String,String)>,root:&Path)->Result<(),String>{
        for item in std::fs::read_dir(directory).map_err(|e|e.to_string())?{let path=item.map_err(|e|e.to_string())?.path();if path.is_dir(){files(&path,prefix,out,root)?;}else{out.insert((prefix.into(),path.strip_prefix(root).unwrap().to_string_lossy().replace('\\',"/")));}}
        Ok(())
    }
    let roots=[("viewer",viewer.to_path_buf()),("kernel",viewer.join("../session_rust")),("wood",viewer.join("../../wood/rust"))];let mut actual=BTreeSet::new();
    for(name,root)in &roots{files(&root.join("src"),name,&mut actual,root)?;for file in ["Cargo.toml","Cargo.lock","build.rs","Trunk.toml","index.html"]{if root.join(file).is_file(){actual.insert((name.to_string(),file.into()));}}}
    let mut recorded=BTreeSet::new();
    for row in &snapshot.files{
        if !recorded.insert((row.root.clone(),row.path.clone())){return Err("Duplicate current source record".into());}
        let root:&PathBuf=&roots.iter().find(|(name,_)|*name==row.root).ok_or("Unknown source root")?.1;
        let bytes=std::fs::read(root.join(&row.path)).map_err(|e|format!("{}: {e}",row.path))?;
        if crate::origin::FileVersion::of(&bytes).hex()!=row.sha256{return Err(format!("Current source changed: {}/{}",row.root,row.path));}
    }
    if actual!=recorded{return Err("Current source file inventory changed".into());}
    Ok(())
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-05-source-audit.rs"
```

</details>

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 96h-prove-current-source-grammar-and-explicit-differences
npm --prefix ../session_tests run course -- check 96h-prove-current-source-grammar-and-explicit-differences
npm --prefix ../session_tests run course -- save 96h-prove-current-source-grammar-and-explicit-differences
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

These are expected checkpoint behaviors, not new GPU/browser acceptance from this editorial batch. Current production parity and the remaining runtime fixes require the declared matched checks after integration.

The original comparison scene remains intact while source and registry fixtures verify the complete declared maintained Rust inventory.

Read the dated source inventory and grammar observations beside the explicit implementation differences. Choose one listed command and execute it on an original selection in the browser. Distinguish that observed action from the optional source-hash and parser fixtures; complete viewer parity remains open.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Original Element ancestry and derived feature trees](96i-separate-own-and-hosted-element-features.md).
