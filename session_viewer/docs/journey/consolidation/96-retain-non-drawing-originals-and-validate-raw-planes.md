# 19.1 · Complete source metadata and the actual command contract

**Steps:** 9 · **Source:** [app/session_io.rs](../../../src/app/session_io.rs) · [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/scene.rs](../../../src/app/scene.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Retained Plane Triad: O + X_axis (R) + Y_axis (G) + Z_axis (B)</text>
  <polygon points="68,136 132,76 268,76 204,136" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <circle cx="168" cy="106" r="4.5" fill="#111111"/>
  <line x1="168" y1="106" x2="236" y2="106" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <line x1="168" y1="106" x2="202" y2="74" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="168" y1="106" x2="168" y2="48" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Family Bitmask: Mesh | BRep | Curve | Point | Plane | Cloud</text>
  <rect x="430" y="56" width="78" height="36" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="469" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Mesh ✓</text>
  <rect x="520" y="56" width="78" height="36" rx="4" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">BRep ✓</text>
  <rect x="610" y="56" width="78" height="36" rx="4" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Plane ✓</text>
  <rect x="475" y="104" width="168" height="34" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="125" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">w_pen = w_base * scale</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Filtered Geometry Families + Explicit Plane Frame</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 888,68 938,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="948,132 978,94 1036,94 1006,132" fill="#fef9c3" stroke="#111111" stroke-width="1.5"/>
  <line x1="992" y1="113" x2="1028" y2="113" stroke="#dc2626" stroke-width="2"/>
  <line x1="992" y1="113" x2="992" y2="74" stroke="#2563eb" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-96-retain-non-drawing-originals-and-validate-raw-planes"></a>1. Retain non drawing originals and validate raw planes

### 1. `src/source_metadata.rs`

Own exact non drawing source metadata and pre-kernel plane validation.

Create the file and type:

```rust
use session_rust::proto;
use std::collections::HashSet;

pub fn components(source: &proto::Session) -> Option<proto::Objects> {
    let objects = source.objects.as_ref()?;
    (!objects.components.is_empty()).then(|| proto::Objects {
        name: objects.name.clone(),
        guid: objects.guid.clone(),
        components: objects.components.clone(),
        ..Default::default()
    })
}

pub fn unused_definitions(source: &proto::Session) -> Option<proto::Objects> {
    let definitions = source.definitions.as_ref()?;
    let used = source
        .objects
        .as_ref()
        .map(|o| {
            o.instances
                .iter()
                .map(|i| i.definition_guid.as_str())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default();
    let mut out = proto::Objects {
        name: definitions.name.clone(),
        guid: definitions.guid.clone(),
        ..Default::default()
    };
    let mut count = 0;
    macro_rules! keep{($($field:ident),*)=>{$(out.$field=definitions.$field.iter().filter(|v|!used.contains(v.guid.as_str())).cloned().collect();count+=out.$field.len();)*};}
    keep!(
        points,
        lines,
        planes,
        bboxes,
        polylines,
        pointclouds,
        meshes,
        nurbscurves,
        nurbssurfaces,
        breps,
        elements,
        components,
        sheets,
        instances
    );
    (count > 0).then_some(out)
}

pub fn raw_plane(plane: &proto::Plane) -> Result<(), String> {
    if plane.frame.len() != 12
        || plane.frame.iter().any(|v| !v.is_finite())
        || !plane.width.is_finite()
        || plane.width < 0.
    {
        return Err("Invalid original plane frame".into());
    }
    if !crate::plane_original::valid(&session_rust::Plane::from_proto(
        plane.clone(),
    )) {
        return Err("Invalid original plane frame".into());
    }
    Ok(())
}

pub fn usage<'a>(
    scenes: impl IntoIterator<Item = &'a crate::scene::Scene>,
) -> [usize; 2] {
    use prost::Message;
    let mut origins = HashSet::new();
    let mut usage = [0, 0];
    for scene in scenes {
        for doc in &scene.forest.documents {
            if let Some(origin) = doc.header.as_ref() {
                if origins.insert(std::rc::Rc::as_ptr(origin)) {
                    usage[0] += 1;
                    usage[1] += origin.header.encoded_len();
                }
            }
        }
    }
    usage
}
```

### 2. `src/lib.rs`

Register the completed audit owner.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod bench_tests;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod bench_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
pub mod source_metadata;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-96a-merge-unused-definitions-with-displayed-instances"></a>2. Merge unused definitions with displayed instances

### 1. `src/definition_merge.rs`

Merge exact retained definitions without losing unused originals.

Create the file and type:

```rust
use session_rust::proto;

pub fn merge(
    unused: Option<&proto::Objects>,
    used: Option<proto::Objects>,
) -> Result<Option<proto::Objects>, String> {
    let Some(original) = unused else {
        return Ok(used);
    };
    let mut out = used.unwrap_or_default();
    out.name = original.name.clone();
    out.guid = original.guid.clone();
    macro_rules! append{($($field:ident),*)=>{$(for source in &original.$field{
        if let Some(existing)=out.$field.iter().find(|v|v.guid==source.guid){if existing!=source{return Err("Different original definitions reuse a GUID".into());}}
        else{out.$field.push(source.clone());}
    }out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));)*};}
    append!(
        points,
        lines,
        planes,
        bboxes,
        polylines,
        pointclouds,
        meshes,
        nurbscurves,
        nurbssurfaces,
        breps,
        elements,
        components,
        sheets,
        instances
    );
    Ok(Some(out))
}
```

### 2. `src/lib.rs`

Register the completed audit owner.

**Locate this existing block:**

```rust
mod bench_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
pub mod source_metadata;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod bench_tests;

pub mod source_metadata;

#[cfg(test)]
mod ambient_shader_contract;
pub mod definition_merge;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-96b-preserve-component-and-interaction-metadata-in-archives"></a>3. Preserve component and interaction metadata in archives

### 1. `src/origin.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

**Locate this existing block:**

```rust
            graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            graph: message.graph.clone(),
            objects: crate::source_metadata::components(message),
            interactions: message.interactions.clone(),
            definitions: crate::source_metadata::unused_definitions(message),
            bvh_boxes: message.bvh_boxes.clone(),
```

### 2. `src/document_source.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

**Locate this existing block:**

```rust
    }
    identity!(elements);
    identity!(planes);
    identity!(bboxes);
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    identity!(instances);
    identity!(sheets);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let sheets = objects
        .sheets
        .iter()
        .cloned()
        .map(|sheet| {
            crate::sheet_source::Prepared::new(Rc::new(sheet))
                .map(Rc::new)
                .map_err(str::to_owned)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    let world =
        crate::nested_placement::prepare(&document, place).map_err(str::to_owned)?;
    Ok(Source {
        document,
        origin,
        tree,
        world,
        sheets,
    })
}

fn supported(source: &proto::Session) -> Result<(), String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {
        return Err("Interactions arrive with element features".into());
    }
    let objects = source
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.components.is_empty() {
        return Err("This subject cannot open assemblies yet".into());
    }
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    identity!(components);
    identity!(elements);
    identity!(planes);
    identity!(bboxes);
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    identity!(instances);
    identity!(sheets);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let sheets = objects
        .sheets
        .iter()
        .cloned()
        .map(|sheet| {
            crate::sheet_source::Prepared::new(Rc::new(sheet))
                .map(Rc::new)
                .map_err(str::to_owned)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    let world =
        crate::nested_placement::prepare(&document, place).map_err(str::to_owned)?;
    Ok(Source {
        document,
        origin,
        tree,
        world,
        sheets,
    })
}

fn supported(source: &proto::Session) -> Result<(), String> {
    crate::instance_plan::validate(source)?;
    Ok(())
```

### 3. `src/source_validate.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

**Locate this existing block:**

```rust
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {
            if !crate::plane_original::valid(&session_rust::Plane::from_proto(
                item.clone(),
            )) {
                return Err("Invalid original plane frame".into());
            }
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            + objects.breps.len()
            + objects.elements.len()
            + objects.components.len()
            + objects.sheets.len()
            + objects.instances.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {
            crate::source_metadata::raw_plane(item)?;
        }
```

### 4. `src/archive_document.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

**Locate this existing block:**

```rust
    };
    source.definitions = crate::instance_snapshot::definitions(scene, &namespace.ids)?;
    source.objects = Some(objects);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    if let Some(metadata) = source.objects.as_ref() {
        objects.components = metadata.components.clone();
    }
    source.definitions = crate::definition_merge::merge(
        source.definitions.as_ref(),
        crate::instance_snapshot::definitions(scene, &namespace.ids)?,
    )?;
    source.objects = Some(objects);
```

## <a id="checkpoint-96c-keep-current-element-and-frame-definition-families"></a>4. Keep current Element and frame definition families

### 1. `src/instance_plan.rs`

Retain supported original definition families and per-instance feature records.

**Locate this existing block:**

```rust
    crate::source_validate::definitions(definitions, instances.len())?;
    if !definitions.planes.is_empty()
        || !definitions.bboxes.is_empty()
        || !definitions.elements.is_empty()
        || !definitions.components.is_empty()
        || !definitions.sheets.is_empty()
        || !definitions.instances.is_empty()
    {
        return Err("Unsupported original definition family".into());
    }
    let mut guids = HashSet::new();
    macro_rules! identity {
        ($field:ident) => {
            for object in &definitions.$field {
                if object.guid.is_empty() || !guids.insert(object.guid.as_str()) {
                    return Err("Definition GUIDs must be distinct".into());
                }
            }
        };
    }
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    let mut refs = HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()
            || !refs.insert(instance.guid.as_str())
            || !guids.contains(instance.definition_guid.as_str())
        {
            return Err("Invalid original instance reference".into());
        }
        if instance.flags & InstanceRef::FLAG_COLOR != 0 {
            let color = instance
                .color
                .as_ref()
                .ok_or("Instance colour flag has no original colour")?;
            if [color.r, color.g, color.b, color.a]
                .iter()
                .any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))
            {
                return Err("Invalid original instance colour".into());
            }
        }
        if !instance.features.is_empty() {
            return Err("Instance features connect with element features".into());
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    crate::source_validate::definitions(definitions, instances.len())?;
    if !definitions.components.is_empty()
        || !definitions.sheets.is_empty()
        || !definitions.instances.is_empty()
    {
        return Err("Unsupported original definition family".into());
    }
    let mut guids = HashSet::new();
    macro_rules! identity {
        ($field:ident) => {
            for object in &definitions.$field {
                if object.guid.is_empty() || !guids.insert(object.guid.as_str()) {
                    return Err("Definition GUIDs must be distinct".into());
                }
            }
        };
    }
    identity!(elements);
    identity!(planes);
    identity!(bboxes);
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    let mut refs = HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()
            || !refs.insert(instance.guid.as_str())
            || !guids.contains(instance.definition_guid.as_str())
        {
            return Err("Invalid original instance reference".into());
        }
        if instance.flags & InstanceRef::FLAG_COLOR != 0 {
            let color = instance
                .color
                .as_ref()
                .ok_or("Instance colour flag has no original colour")?;
            if [color.r, color.g, color.b, color.a]
                .iter()
                .any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))
            {
                return Err("Invalid original instance colour".into());
            }
        }
```

### 2. `src/instance_definition.rs`

Retain supported original definition families and per-instance feature records.

**Locate this existing block:**

```rust
    match &owner.definition {
        Geometry::Mesh(source) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    match &owner.definition {
        Geometry::Element(source) => {
            crate::element_original::insert(&mut scene, source.clone())?;
        }
        Geometry::Plane(source) => {
            crate::plane_original::insert(&mut scene, source.clone())?;
        }
        Geometry::OBB(source) => {
            crate::bounds_original::insert(&mut scene, source.clone())?;
        }
        Geometry::Mesh(source) => {
```

### 3. `src/instance_snapshot.rs`

Retain supported original definition families and per-instance feature records.

**Locate this existing block:**

```rust
        match &owner.definition {
            Geometry::Mesh(source) => keep!(meshes, source),
            Geometry::BRep(source) => keep!(breps, source),
            Geometry::NurbsSurface(source) => keep!(nurbssurfaces, source),
            Geometry::NurbsCurve(source) => keep!(nurbscurves, source),
            Geometry::Polyline(source) => keep!(polylines, source),
            Geometry::Line(source) => keep!(lines, source),
            Geometry::Point(source) => keep!(points, source),
            Geometry::PointCloud(source) => keep!(pointclouds, source),
            _ => return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {
        ($field:ident) => {
            out.$field
                .sort_by(|a, b| a.guid.cmp(&b.guid));
        };
    }
    order!(meshes);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match &owner.definition {
            Geometry::Element(source) => keep!(elements, source),
            Geometry::Plane(source) => keep!(planes, source),
            Geometry::OBB(source) => keep!(bboxes, source),
            Geometry::Mesh(source) => keep!(meshes, source),
            Geometry::BRep(source) => keep!(breps, source),
            Geometry::NurbsSurface(source) => keep!(nurbssurfaces, source),
            Geometry::NurbsCurve(source) => keep!(nurbscurves, source),
            Geometry::Polyline(source) => keep!(polylines, source),
            Geometry::Line(source) => keep!(lines, source),
            Geometry::Point(source) => keep!(points, source),
            Geometry::PointCloud(source) => keep!(pointclouds, source),
            _ => return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {
        ($field:ident) => {
            out.$field
                .sort_by(|a, b| a.guid.cmp(&b.guid));
        };
    }
    order!(elements);
    order!(planes);
    order!(bboxes);
    order!(meshes);
```

## <a id="checkpoint-96d-observe-the-real-current-registry-and-parser"></a>5. Observe the real current registry and parser

### 1. `src/registry_audit.rs`

Observe the compiled maintained command registry directly.

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
pub fn snapshot() -> serde_json::Value {
    use current_viewer::app::command;
    let outcome = |line: &str| match command::parse(line) {
        Ok(action) => {
            serde_json::json!({"accepted":true,"selection":action.needs_selection(),"keeps_draft":action.keeps_draft(),"keeps_split":action.keeps_split()})
        }
        Err(error) => serde_json::json!({"accepted":false,"error":error}),
    };
    serde_json::Value::Array(command::REGISTRY.iter().map(|entry|{
        let spec=entry.spec();
        serde_json::json!({"names":spec.names,"aliases":spec.aliases,"hint":spec.hint,"arity":spec.arity,"wait_for_option":spec.wait_for_option,"wait_after_option":spec.wait_after_option,
            "alias_results":spec.aliases.iter().map(|line|serde_json::json!({"line":line,"canonical":command::canonical(line),"result":outcome(line)})).collect::<Vec<_>>(),
            "bare":spec.names.iter().map(|line|serde_json::json!({"line":line,"result":outcome(line)})).collect::<Vec<_>>(),
            "options":spec.options.iter().map(|line|serde_json::json!({"line":line,"canonical":command::canonical(line),"result":outcome(line)})).collect::<Vec<_>>()})
    }).collect())
}
```

### 2. `src/lib.rs`

Register the completed audit owner.

**Locate this existing block:**

```rust
pub mod source_metadata;

#[cfg(test)]
mod ambient_shader_contract;
pub mod definition_merge;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_metadata;

pub mod definition_merge;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod registry_audit;
pub mod source_mesh_topology;
```

### 3. `src/inventory_example.rs`

Copy the supplied native registry observation example.

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&viewer_journey::registry_audit::snapshot())
            .unwrap()
    );
}
#[cfg(target_arch = "wasm32")]
fn main() {}
```

### 4. `Cargo.toml`

Expose the maintained registry observation as a native example.

**Locate this existing block:**

```toml
name = "reference"
path = "src/reference_example.rs"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
name = "reference"
path = "src/reference_example.rs"

[[example]]
name = "inventory"
path = "src/inventory_example.rs"
```

## <a id="checkpoint-96e-verify-the-complete-declared-current-source-closure"></a>6. Verify the complete declared current source closure

### 1. `src/current_inventory.json`

Copy the supplied full current Rust source/build closure and ownership snapshot.

Create the file and type:

```json
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

### 2. `src/source_audit.rs`

Verify actual source bytes and the complete declared file set.

Create the file and type:

```rust
#[derive(serde::Deserialize)]
pub struct FileRecord {
    pub root: String,
    pub path: String,
    pub sha256: String,
}
#[derive(serde::Deserialize)]
pub struct Snapshot {
    pub session_head: String,
    pub wood_head: String,
    pub files: Vec<FileRecord>,
}

pub fn recorded() -> Snapshot {
    serde_json::from_str(include_str!("current_inventory.json"))
        .expect("Supplied current source inventory")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn verify(viewer: &std::path::Path, snapshot: &Snapshot) -> Result<(), String> {
    use std::{
        collections::BTreeSet,
        path::{Path, PathBuf},
    };
    fn files(
        directory: &Path,
        prefix: &str,
        out: &mut BTreeSet<(String, String)>,
        root: &Path,
    ) -> Result<(), String> {
        for item in std::fs::read_dir(directory).map_err(|e| e.to_string())? {
            let path = item.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                files(&path, prefix, out, root)?;
            } else {
                out.insert((
                    prefix.into(),
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                ));
            }
        }
        Ok(())
    }
    let roots = [
        ("viewer", viewer.to_path_buf()),
        ("kernel", viewer.join("../session_rust")),
        ("wood", viewer.join("../../wood/rust")),
    ];
    let mut actual = BTreeSet::new();
    for (name, root) in &roots {
        files(&root.join("src"), name, &mut actual, root)?;
        for file in [
            "Cargo.toml",
            "Cargo.lock",
            "build.rs",
            "Trunk.toml",
            "index.html",
        ] {
            if root.join(file).is_file() {
                actual.insert((name.to_string(), file.into()));
            }
        }
    }
    let mut recorded = BTreeSet::new();
    for row in &snapshot.files {
        if !recorded.insert((row.root.clone(), row.path.clone())) {
            return Err("Duplicate current source record".into());
        }
        let root: &PathBuf = &roots
            .iter()
            .find(|(name, _)| *name == row.root)
            .ok_or("Unknown source root")?
            .1;
        let bytes = std::fs::read(root.join(&row.path))
            .map_err(|e| format!("{}: {e}", row.path))?;
        if crate::origin::FileVersion::of(&bytes).hex() != row.sha256 {
            return Err(format!(
                "Current source changed: {}/{}",
                row.root, row.path
            ));
        }
    }
    if actual != recorded {
        return Err("Current source file inventory changed".into());
    }
    Ok(())
}
```

### 3. `src/lib.rs`

Register the completed audit owner.

**Locate this existing block:**

```rust
pub mod definition_merge;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod registry_audit;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod definition_merge;

pub mod registry_audit;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_audit;
pub mod source_mesh_topology;
```

## <a id="checkpoint-96f-report-retained-original-metadata-in-the-real-viewer"></a>7. Report retained metadata in the real viewer

### 1. `src/browser.rs`

Expose truthful serialized original metadata accounting in the teaching inspector.

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-groups",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-source-metadata",
        &serde_json::to_string(&crate::source_metadata::usage(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-groups",
```

## <a id="checkpoint-96g-prove-exact-metadata-and-definition-round-trips"></a>8. Prove exact metadata and definition round trips

### 1. `src/metadata_tests.rs`

Copy supplied exact archive, original-family and malformed-input fixtures.

Create the file and type:

```rust
use prost::Message;
use session_rust::{Xform, proto};

#[test]
fn component_json_and_unknown_interaction_bytes_survive_archives() {
    let mut message =
        proto::Session::decode(crate::comparison::specimen().as_slice()).unwrap();
    let component = proto::Component {
        type_name: "FloorBuilder".into(),
        guid: "component-original".into(),
        name: "guide".into(),
        json_data: "{ \"origin\": [1000000000.125, 0, 0], \"custom\": true }".into(),
    };
    let interactions = vec![proto::InteractionEntry {
        guid: "original-edge".into(),
        interactions: vec![proto::Interaction {
            guid: "original-interaction".into(),
            name: "glue".into(),
            interaction_type: "custom-glue".into(),
            interaction_data: vec![0, 255, 1, 2, 3],
        }],
    }];
    message
        .objects
        .as_mut()
        .unwrap()
        .components
        .push(component.clone());
    message
        .tree
        .as_mut()
        .unwrap()
        .root
        .as_mut()
        .unwrap()
        .children
        .push(proto::TreeNode {
            name: component.guid.clone(),
            ..Default::default()
        });
    message.interactions = interactions.clone();
    if let Ok(path) = std::env::var("VIEWER_COURSE_METADATA_OUTPUT") {
        std::fs::write(path, message.encode_to_vec()).unwrap();
    }
    let mut scene = crate::scene::Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &message.encode_to_vec(),
        "original",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(scene.objects().len(), 4);
    for _ in 0..2 {
        let bytes = crate::scene_archive::save(&scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        let saved = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        assert_eq!(
            saved.objects.unwrap().components,
            vec![component.clone()]
        );
        assert_eq!(saved.interactions, interactions);
        crate::scene_document::open(&mut scene, &bytes).unwrap();
    }
}

#[test]
fn unused_original_definition_stays_original_without_a_display_row() {
    let mut message =
        proto::Session::decode(crate::comparison::specimen().as_slice()).unwrap();
    let mut original = session_rust::BRep::create_cylinder(25., 100.).to_proto();
    original.guid = "unused-original".into();
    message.definitions = Some(proto::Objects {
        breps: vec![original.clone()],
        ..Default::default()
    });
    let mut scene = crate::scene::Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &message.encode_to_vec(),
        "unused",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(scene.objects().len(), 4);
    let saved =
        crate::archive::Bundle::decode(&crate::scene_archive::save(&scene).unwrap())
            .unwrap();
    let source = proto::Session::decode(saved.documents[0].as_slice()).unwrap();
    assert_eq!(
        source.definitions.unwrap().breps,
        vec![original]
    );
}

#[test]
fn raw_plane_nonfinite_and_short_frames_are_rejected_before_kernel_defaults() {
    let original = session_rust::Plane::xy_plane().to_proto();
    for index in 0..12 {
        let mut p = original.clone();
        p.frame[index] = f64::NAN;
        let message = proto::Session {
            objects: Some(proto::Objects {
                planes: vec![p],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(
            crate::document_source::prepare(
                &message.encode_to_vec(),
                &Xform::identity(),
                0.
            )
            .is_err()
        );
    }
    let mut p = original.clone();
    p.frame.truncate(3);
    assert!(crate::source_metadata::raw_plane(&p).is_err());
    crate::source_metadata::raw_plane(&original).unwrap();
}
#[test]
fn element_instances_keep_typed_definitions_and_instance_features() {
    use wood::WoodElement;
    let axis = session_rust::Polyline::new(vec![
        session_rust::Point::new(0., 0., 0.),
        session_rust::Point::new(1000., 0., 0.),
    ]);
    let element = crate::element_beam::build(&[axis.clone()], 100., 200.).unwrap()[0]
        .to_element();
    let _ = element.guid();
    let source = element.to_proto();
    let feature = proto::ElementFeature {
        guid: "instance-feature".into(),
        name: "cut".into(),
        feature_type: "cut".into(),
        face_index: -1,
        outlines: vec![axis.to_proto()],
        ..Default::default()
    };
    let instance = proto::InstanceRef {
        guid: "element-instance".into(),
        name: "beam instance".into(),
        definition_guid: source.guid.clone(),
        features: vec![feature.clone()],
        ..Default::default()
    };
    let message = proto::Session {
        objects: Some(proto::Objects {
            instances: vec![instance],
            ..Default::default()
        }),
        definitions: Some(proto::Objects {
            elements: vec![source.clone()],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut scene = crate::scene::Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &message.encode_to_vec(),
        "element instance",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(scene.instances.len(), 1);
    let saved =
        crate::archive::Bundle::decode(&crate::scene_archive::save(&scene).unwrap())
            .unwrap();
    let source_again = proto::Session::decode(saved.documents[0].as_slice()).unwrap();
    assert_eq!(
        source_again.definitions.unwrap().elements,
        vec![source]
    );
    assert_eq!(
        source_again.objects.unwrap().instances[0].features,
        vec![feature]
    );
}
#[test]
fn merging_original_definitions_rejects_conflicting_payloads() {
    let mut mesh = session_rust::Mesh::from_vertices_and_faces(
        vec![session_rust::Point::new(0., 0., 0.)],
        vec![],
    )
    .to_proto();
    mesh.guid = "same".into();
    let unused = proto::Objects {
        meshes: vec![mesh.clone()],
        ..Default::default()
    };
    let mut other = mesh;
    other.name = "different source".into();
    assert!(
        crate::definition_merge::merge(
            Some(&unused),
            Some(proto::Objects {
                meshes: vec![other],
                ..Default::default()
            })
        )
        .is_err()
    );
}
```

### 2. `src/lib.rs`

Register supplied metadata fixtures.

**Locate this existing block:**

```rust
pub mod registry_audit;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_audit;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod registry_audit;

pub mod source_audit;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod metadata_tests;
pub mod source_mesh_topology;
```

## <a id="checkpoint-96h-prove-current-source-grammar-and-explicit-differences"></a>9. Prove current source grammar and explicit differences

### 1. `src/current_registry.json`

Copy real observed current parser outcomes, prompts, aliases and action flags.

Create the file and type:

```json
[
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "View",
        "result": {
          "accepted": false,
          "error": "Choose a View direction or display setting"
        }
      }
    ],
    "hint": "View: direction, projection, edges, outlines and display settings",
    "names": [
      "View"
    ],
    "options": [
      {
        "canonical": "View Top",
        "line": "View Top",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Side",
        "line": "View Side",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Front",
        "line": "View Front",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Back",
        "line": "View Back",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Left",
        "line": "View Left",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Right",
        "line": "View Right",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Bottom",
        "line": "View Bottom",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Isometric",
        "line": "View Isometric",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Isometric Back",
        "line": "View Isometric Back",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Reset",
        "line": "View Reset",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Perspective",
        "line": "View Perspective",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Orthographic",
        "line": "View Orthographic",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Show Edges",
        "line": "View Show Edges",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Hide Edges",
        "line": "View Hide Edges",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Show Lines",
        "line": "View Show Lines",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Hide Lines",
        "line": "View Hide Lines",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Show Points",
        "line": "View Show Points",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Hide Points",
        "line": "View Hide Points",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Outline",
        "line": "View Outline",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Outline On",
        "line": "View Outline On",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Outline Off",
        "line": "View Outline Off",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Lighting",
        "line": "View Lighting",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Backfaces",
        "line": "View Backfaces",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Xray",
        "line": "View Xray",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Point Size",
        "line": "View Point Size",
        "result": {
          "accepted": false,
          "error": "Choose a View direction or display setting"
        }
      },
      {
        "canonical": "View Plane Size",
        "line": "View Plane Size",
        "result": {
          "accepted": false,
          "error": "Choose a View direction or display setting"
        }
      },
      {
        "canonical": "View Names",
        "line": "View Names",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Hide Selected",
        "line": "View Hide Selected",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      },
      {
        "canonical": "View Show All",
        "line": "View Show All",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": true,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Report",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Report · download the viewer report: this run and the last failed or interrupted one",
    "names": [
      "Report"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Point",
        "result": {
          "accepted": false,
          "error": "Point needs one point"
        }
      }
    ],
    "hint": "Point · Enter then click or type x,y,z",
    "names": [
      "Point"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Line",
        "result": {
          "accepted": false,
          "error": "Line needs 2 points"
        }
      }
    ],
    "hint": "Line · Enter then click or type endpoints · Example: Line 0,0,0 100,0,0",
    "names": [
      "Line"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Arrow",
        "result": {
          "accepted": false,
          "error": "Arrow needs 2 points"
        }
      }
    ],
    "hint": "Arrow · Enter then click or type start and tip · Example: Arrow 0,0,0 100,0,0",
    "names": [
      "Arrow"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Polyline",
        "result": {
          "accepted": false,
          "error": "Polyline needs at least 2 points"
        }
      }
    ],
    "hint": "Polyline · click points, or choose Rectangle / Polygon · Enter finishes",
    "names": [
      "Polyline"
    ],
    "options": [
      {
        "canonical": "Polyline Points",
        "line": "Polyline Points",
        "result": {
          "accepted": false,
          "error": "use coordinates x,y,z separated by spaces"
        }
      },
      {
        "canonical": "Polyline Rectangle",
        "line": "Polyline Rectangle",
        "result": {
          "accepted": false,
          "error": "use coordinates x,y,z separated by spaces"
        }
      },
      {
        "canonical": "Polyline Polygon",
        "line": "Polyline Polygon",
        "result": {
          "accepted": false,
          "error": "use coordinates x,y,z separated by spaces"
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Curve",
        "result": {
          "accepted": false,
          "error": "Curve needs at least 2 points"
        }
      }
    ],
    "hint": "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
    "names": [
      "Curve"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Close",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Close · joins the polyline or curve being drawn back to its first point",
    "names": [
      "Close"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Trim",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Trim · pick objects, Enter, pick cutters, Enter, click the parts to remove · Esc cancels · Trim 0.2 0.8 keeps that part of a curve",
    "names": [
      "Trim"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Extend",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Extend · pick boundaries, Enter, click near curve ends · Extend Distance 5 grows ends by a length · Extend -0.2 1.2 extends a curve's domain",
    "names": [
      "Extend"
    ],
    "options": [
      {
        "canonical": "Extend Distance",
        "line": "Extend Distance",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": true,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Explode",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Explode · a polyline into lines, a BRep into faces, a mesh into faces, a point cloud into points",
    "names": [
      "Explode"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Move",
        "line": "m",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "aliases": [
      "m"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Move",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Move · pick a base point, then the target point · Move 10,0,0 moves by a typed offset",
    "names": [
      "Move"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Rotate",
        "line": "rot",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "aliases": [
      "rot"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Rotate",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Rotate · pick the center, then type an angle or pick two reference points · Rotate z 45 turns about a world axis",
    "names": [
      "Rotate"
    ],
    "options": [
      {
        "canonical": "Rotate x",
        "line": "Rotate x",
        "result": {
          "accepted": false,
          "error": "missing a number; try `Rotate x 90`"
        }
      },
      {
        "canonical": "Rotate y",
        "line": "Rotate y",
        "result": {
          "accepted": false,
          "error": "missing a number; try `Rotate x 90`"
        }
      },
      {
        "canonical": "Rotate z",
        "line": "Rotate z",
        "result": {
          "accepted": false,
          "error": "missing a number; try `Rotate x 90`"
        }
      }
    ],
    "wait_after_option": true,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Scale",
        "line": "s",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "aliases": [
      "s"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Scale",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Scale · pick the origin, then type a factor or pick two reference points · 1D, 2D or 3D · Scale 2 scales at once",
    "names": [
      "Scale"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Copy",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Copy · pick a base point, then as many target points as copies · Enter finishes · Copy 10,0,0 copies once",
    "names": [
      "Copy"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Orient 3 Points",
        "line": "orient3pt",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "aliases": [
      "orient3pt"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Orient 3 Points",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Orient 3 Points · pick three reference points, then three target points · the selection moves rigidly",
    "names": [
      "Orient 3 Points"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Split",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": true,
          "selection": false
        }
      }
    ],
    "hint": "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels",
    "names": [
      "Split"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Save",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Save downloads the complete editable scene as a .session file",
    "names": [
      "Save"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Open",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Open restores a saved .session file",
    "names": [
      "Open"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Delete",
        "line": "del",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "aliases": [
      "del"
    ],
    "arity": 0,
    "bare": [
      {
        "line": "Delete",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "",
    "names": [
      "Delete"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Undo",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Undo"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Redo",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Redo"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Hide",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Hide"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Show",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Show"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Fit",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Fit zooms to the selection, or the whole scene when nothing is selected",
    "names": [
      "Fit"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Escape",
        "line": "esc",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "esc"
    ],
    "arity": 0,
    "bare": [
      {
        "line": "Escape",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Escape"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Layers",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Layers (On Off All): show or hide the layer panel; All shows it with every row expanded",
    "names": [
      "Layers"
    ],
    "options": [
      {
        "canonical": "Layers On",
        "line": "Layers On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Layers Off",
        "line": "Layers Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Layers All",
        "line": "Layers All",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Opacity",
        "result": {
          "accepted": false,
          "error": "missing a number; try `Opacity 0.5`"
        }
      }
    ],
    "hint": "Opacity 0..1: how solid the faces are · 0 is x-ray, 1 solid · Example: Opacity 0.5",
    "names": [
      "Opacity"
    ],
    "options": [
      {
        "canonical": "Opacity 1",
        "line": "Opacity 1",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Opacity 0.7",
        "line": "Opacity 0.7",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Opacity 0.4",
        "line": "Opacity 0.4",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Opacity 0",
        "line": "Opacity 0",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 1,
    "bare": [
      {
        "line": "Arrowhead",
        "result": {
          "accepted": false,
          "error": "wrong number of arguments for `Arrowhead`"
        }
      }
    ],
    "hint": "Arrowhead: heads on the selected lines, polylines and curves · None, Start, End or Both · Example: Arrowhead End",
    "names": [
      "Arrowhead"
    ],
    "options": [
      {
        "canonical": "Arrowhead None",
        "line": "Arrowhead None",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Arrowhead Start",
        "line": "Arrowhead Start",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Arrowhead End",
        "line": "Arrowhead End",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Arrowhead Both",
        "line": "Arrowhead Both",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Element Attributes",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Attributes (On Off): show or hide the objects hung under each element, such as its base plane",
    "names": [
      "Element Attributes"
    ],
    "options": [
      {
        "canonical": "Element Attributes On",
        "line": "Element Attributes On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Element Attributes Off",
        "line": "Element Attributes Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Element Interactions",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Interactions (On Off): draw or remove what other elements do to each element, its contacts, joints and cuts",
    "names": [
      "Element Interactions"
    ],
    "options": [
      {
        "canonical": "Element Interactions On",
        "line": "Element Interactions On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Element Interactions Off",
        "line": "Element Interactions Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Element Plate",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Plate: pick closed polylines, the outlines · Thickness 60 or a number changes it · Enter creates a plate on each · Element Plate 40 on a selection creates at once",
    "names": [
      "Element Plate"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Element Beam",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Beam: pick lines or polylines, the axes · Width 150, Height 250 or two numbers change them · Enter creates a beam on each, mitred at its corners · Element Beam 120 200 on a selection creates at once",
    "names": [
      "Element Beam"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Element Column",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Column: pick lines, the axes · Width 200, Depth 300 or two numbers change them · Enter creates a column on each · Element Column 200 300 on a selection creates at once",
    "names": [
      "Element Column"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Element Block",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Block: pick closed polylines, bottom then top, then hole pairs in the same order · Enter creates the block · on a selection it creates at once",
    "names": [
      "Element Block"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Element Beam Variable",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Beam Variable: pick closed sections in station order · Enter creates the beam, its axis from the first section's centre to the last's · on a selection it creates at once",
    "names": [
      "Element Beam Variable"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Element Support",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Element Support: pick points or planes, where the supports stand · Enter creates a support on each · on a selection it creates at once",
    "names": [
      "Element Support"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Snap",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Snap (On Off): snapping · Snap End / Near / Mid / Center / Perp toggles one kind",
    "names": [
      "Snap"
    ],
    "options": [
      {
        "canonical": "Snap On",
        "line": "Snap On",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap Off",
        "line": "Snap Off",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap End",
        "line": "Snap End",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap Near",
        "line": "Snap Near",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap Mid",
        "line": "Snap Mid",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap Center",
        "line": "Snap Center",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Snap Perp",
        "line": "Snap Perp",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Grid Snap",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Grid Snap (On Off) or a spacing: a point off every object lands on the grid of the construction plane, 1000 apart at start like the ground grid's cells · Example: Grid Snap 50",
    "names": [
      "Grid Snap"
    ],
    "options": [
      {
        "canonical": "Grid Snap On",
        "line": "Grid Snap On",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Grid Snap Off",
        "line": "Grid Snap Off",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Construction Plane",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Construction Plane XY / XZ / YZ fixes the plane points land on and the grid lies on · 3 Point picks its origin, a point on its x axis and one on its y side · View follows the view again · Example: Construction Plane 3 Point 0,0,0 1000,0,1000 0,1000,0",
    "names": [
      "Construction Plane"
    ],
    "options": [
      {
        "canonical": "Construction Plane XY",
        "line": "Construction Plane XY",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Construction Plane XZ",
        "line": "Construction Plane XZ",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Construction Plane YZ",
        "line": "Construction Plane YZ",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Construction Plane 3 Point",
        "line": "Construction Plane 3 Point",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Construction Plane View",
        "line": "Construction Plane View",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Arctic",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Arctic (On Off): soft contact shading, studio lighting and outlines",
    "names": [
      "Arctic"
    ],
    "options": [
      {
        "canonical": "Arctic On",
        "line": "Arctic On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Arctic Off",
        "line": "Arctic Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Outline",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Outline (On Off): black surface outlines",
    "names": [
      "Outline"
    ],
    "options": [
      {
        "canonical": "Outline On",
        "line": "Outline On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Outline Off",
        "line": "Outline Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Object",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Object"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Edge",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Edge"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Face",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Face"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Controls",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "",
    "names": [
      "Controls"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Select Lasso",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Select Lasso · drag a loop around objects · Shift adds · Ctrl removes · Esc cancels",
    "names": [
      "Select Lasso"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Select By Name",
        "result": {
          "accepted": false,
          "error": "type a name after Select By Name; try `Select By Name beam`"
        }
      }
    ],
    "hint": "Select By Name text · selects visible objects whose name contains the text, any case · Example: Select By Name beam",
    "names": [
      "Select By Name"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Select All",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Select All · selects every visible object that is not locked; an element's attributes stay out · Ctrl+A",
    "names": [
      "Select All"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 1,
    "bare": [
      {
        "line": "Select Small",
        "result": {
          "accepted": false,
          "error": "wrong number of arguments for `Select Small`"
        }
      }
    ],
    "hint": "Select Small length · selects visible objects whose bounding-box diagonal is shorter, in scene units · Example: Select Small 10",
    "names": [
      "Select Small"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Clipping Plane",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Clipping Plane: an origin, then a point on the side to cut away · 3Point · XY / YZ / ZX through a point · On / Off every cut · Flip the selected plane · Fill Hatch / Solid (default) · Example: Clipping Plane XY 0,0,1200",
    "names": [
      "Clipping Plane"
    ],
    "options": [
      {
        "canonical": "Clipping Plane Normal",
        "line": "Clipping Plane Normal",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane 3Point",
        "line": "Clipping Plane 3Point",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane XY",
        "line": "Clipping Plane XY",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane YZ",
        "line": "Clipping Plane YZ",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane ZX",
        "line": "Clipping Plane ZX",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane Flip",
        "line": "Clipping Plane Flip",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Clipping Plane On",
        "line": "Clipping Plane On",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane Off",
        "line": "Clipping Plane Off",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane Fill",
        "line": "Clipping Plane Fill",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane Fill Hatch",
        "line": "Clipping Plane Fill Hatch",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Clipping Plane Fill Solid",
        "line": "Clipping Plane Fill Solid",
        "result": {
          "accepted": true,
          "keeps_draft": true,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Box",
        "line": "bo",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "bo"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Box",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Box (Brep Mesh): base center, corner or length and width, height · Example: Box 0,0,0 100 50 30",
    "names": [
      "Box"
    ],
    "options": [
      {
        "canonical": "Box Brep",
        "line": "Box Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Box Mesh",
        "line": "Box Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Sphere",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Sphere (Brep Mesh): center, radius · Example: Sphere 0,0,0 10",
    "names": [
      "Sphere"
    ],
    "options": [
      {
        "canonical": "Sphere Brep",
        "line": "Sphere Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Sphere Mesh",
        "line": "Sphere Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Cylinder",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Cylinder (Brep Mesh): base center, radius, height · Example: Cylinder 0,0,0 5 20",
    "names": [
      "Cylinder"
    ],
    "options": [
      {
        "canonical": "Cylinder Brep",
        "line": "Cylinder Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Cylinder Mesh",
        "line": "Cylinder Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Cone",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Cone (Brep Mesh): base center, radius, height to the apex · Example: Cone 0,0,0 5 20",
    "names": [
      "Cone"
    ],
    "options": [
      {
        "canonical": "Cone Brep",
        "line": "Cone Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Cone Mesh",
        "line": "Cone Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Pyramid",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Pyramid (Brep Mesh): base center, corner or edge length, height · Example: Pyramid 0,0,0 10 15",
    "names": [
      "Pyramid"
    ],
    "options": [
      {
        "canonical": "Pyramid Brep",
        "line": "Pyramid Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Pyramid Mesh",
        "line": "Pyramid Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Torus",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Torus (Brep Mesh): center, major radius, minor radius · Example: Torus 0,0,0 20 5",
    "names": [
      "Torus"
    ],
    "options": [
      {
        "canonical": "Torus Brep",
        "line": "Torus Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Torus Mesh",
        "line": "Torus Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Block With Hole",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Block With Hole (Brep Mesh): base center, corner or length and width, height, hole radius · Example: Block With Hole 0,0,0 40 30 20 5",
    "names": [
      "Block With Hole"
    ],
    "options": [
      {
        "canonical": "Block With Hole Brep",
        "line": "Block With Hole Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Block With Hole Mesh",
        "line": "Block With Hole Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Tetrahedron",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Tetrahedron (Brep Mesh): center, radius to the corners · Example: Tetrahedron 0,0,0 10",
    "names": [
      "Tetrahedron"
    ],
    "options": [
      {
        "canonical": "Tetrahedron Brep",
        "line": "Tetrahedron Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Tetrahedron Mesh",
        "line": "Tetrahedron Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Octahedron",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Octahedron (Brep Mesh): center, radius to the corners · Example: Octahedron 0,0,0 10",
    "names": [
      "Octahedron"
    ],
    "options": [
      {
        "canonical": "Octahedron Brep",
        "line": "Octahedron Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Octahedron Mesh",
        "line": "Octahedron Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Dodecahedron",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Dodecahedron (Brep Mesh): center, radius to the corners · Example: Dodecahedron 0,0,0 10",
    "names": [
      "Dodecahedron"
    ],
    "options": [
      {
        "canonical": "Dodecahedron Brep",
        "line": "Dodecahedron Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Dodecahedron Mesh",
        "line": "Dodecahedron Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Icosahedron",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Icosahedron (Brep Mesh): center, radius to the corners · Example: Icosahedron 0,0,0 10",
    "names": [
      "Icosahedron"
    ],
    "options": [
      {
        "canonical": "Icosahedron Brep",
        "line": "Icosahedron Brep",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Icosahedron Mesh",
        "line": "Icosahedron Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Quad Sphere",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Quad Sphere (Mesh): center, radius · six patches of 8 × 8 quads · Example: Quad Sphere 0,0,0 10",
    "names": [
      "Quad Sphere"
    ],
    "options": [
      {
        "canonical": "Quad Sphere Mesh",
        "line": "Quad Sphere Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Capsule",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Capsule (Mesh): base center, radius, total height · Example: Capsule 0,0,0 5 30",
    "names": [
      "Capsule"
    ],
    "options": [
      {
        "canonical": "Capsule Mesh",
        "line": "Capsule Mesh",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Curve Circle",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Curve Circle: center, radius · Example: Nurbs Curve Circle 0,0,0 10",
    "names": [
      "Nurbs Curve Circle"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Curve Ellipse",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Curve Ellipse: center, end of the first axis or its radius, second radius · Example: Nurbs Curve Ellipse 0,0,0 20 10",
    "names": [
      "Nurbs Curve Ellipse"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Curve Arc",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Curve Arc (Center 3 Points): center, start or radius, end or angle · Example: Nurbs Curve Arc 0,0,0 10 90",
    "names": [
      "Nurbs Curve Arc"
    ],
    "options": [
      {
        "canonical": "Nurbs Curve Arc Center",
        "line": "Nurbs Curve Arc Center",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Nurbs Curve Arc 3 Points",
        "line": "Nurbs Curve Arc 3 Points",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Curve Parabola",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Curve Parabola: start, end or length, apex or height · Example: Nurbs Curve Parabola 0,0,0 100 40",
    "names": [
      "Nurbs Curve Parabola"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Loft",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Loft (Open Closed): click curves in section order, Enter lofts · Example: select curves, then Loft Closed",
    "names": [
      "Loft"
    ],
    "options": [
      {
        "canonical": "Loft Open",
        "line": "Loft Open",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Loft Closed",
        "line": "Loft Closed",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Extrude",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Extrude (Cap On Cap Off): curves, Enter, then a distance or x,y,z · Example: select curves, then Extrude 25",
    "names": [
      "Extrude"
    ],
    "options": [
      {
        "canonical": "Extrude Cap On",
        "line": "Extrude Cap On",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Extrude Cap Off",
        "line": "Extrude Cap Off",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface Loft",
        "line": "nurbssurface_loft",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_loft"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface Loft",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface Loft (Smooth Straight): click curves in section order, Enter lofts",
    "names": [
      "Nurbs Surface Loft"
    ],
    "options": [
      {
        "canonical": "Nurbs Surface Loft Smooth",
        "line": "Nurbs Surface Loft Smooth",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Nurbs Surface Loft Straight",
        "line": "Nurbs Surface Loft Straight",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface Network",
        "line": "nurbssurface_network",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Nurbs Surface Network",
        "line": "nurbsnurbs_network",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      },
      {
        "canonical": "Nurbs Surface Network",
        "line": "Nurbs Nurbs Network",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_network",
      "nurbsnurbs_network",
      "Nurbs Nurbs Network"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface Network",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface Network: click curves of both directions in any order, Enter builds the surface through them",
    "names": [
      "Nurbs Surface Network"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface Revolve",
        "line": "nurbssurface_revolve",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_revolve"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface Revolve",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface Revolve: profile curves, Enter, axis start, axis end, angle · Example: Nurbs Surface Revolve 0,0,0 0,0,1 90",
    "names": [
      "Nurbs Surface Revolve"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface 4 Points",
        "line": "nurbssurface_4_points",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_4_points"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface 4 Points",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface 4 Points: four corners in order · Example: Nurbs Surface 4 Points 0,0,0 10,0,0 10,10,3 0,10,0",
    "names": [
      "Nurbs Surface 4 Points"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface Sweep1",
        "line": "nurbssurface_sweep1",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_sweep1"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface Sweep1",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface Sweep1: the rail, then profile curves, Enter sweeps",
    "names": [
      "Nurbs Surface Sweep1"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [
      {
        "canonical": "Nurbs Surface Sweep2",
        "line": "nurbssurface_sweep2",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "aliases": [
      "nurbssurface_sweep2"
    ],
    "arity": null,
    "bare": [
      {
        "line": "Nurbs Surface Sweep2",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Nurbs Surface Sweep2: first rail, second rail, shape curves, Enter sweeps",
    "names": [
      "Nurbs Surface Sweep2"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Text",
        "result": {
          "accepted": false,
          "error": "Text takes 1 to 80 characters · Example: Text Hello world"
        }
      }
    ],
    "hint": "Text Hello world · then click or type the lower-left point · Height N sets the letter height",
    "names": [
      "Text"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Project To Plane",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Project To Plane: flatten the selected points, lines, polylines, curves and meshes · CPlane is the plane the view faces · XY / YZ / ZX through the origin · 3Point picks the plane · Example: Project To Plane XY",
    "names": [
      "Project To Plane"
    ],
    "options": [
      {
        "canonical": "Project To Plane CPlane",
        "line": "Project To Plane CPlane",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Project To Plane XY",
        "line": "Project To Plane XY",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Project To Plane YZ",
        "line": "Project To Plane YZ",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Project To Plane ZX",
        "line": "Project To Plane ZX",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      },
      {
        "canonical": "Project To Plane 3Point",
        "line": "Project To Plane 3Point",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "wait_after_option": false,
    "wait_for_option": true
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Measure Distance",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": false
        }
      }
    ],
    "hint": "Measure Distance · pick two points, snaps on · or type Measure Distance 0,0,0 100,0,0",
    "names": [
      "Measure Distance"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Length",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Length · total length of the selected lines, polylines and NURBS curves",
    "names": [
      "Length"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "BoundingBox",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "BoundingBox · create a world axis aligned box around the selection",
    "names": [
      "BoundingBox"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Length BoundingBox",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Length BoundingBox · world X, Y and Z extents of the selection",
    "names": [
      "Length BoundingBox"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Area",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Area · surface area of the selected meshes, BReps, NURBS surfaces and elements, or of one selected face",
    "names": [
      "Area"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Volume",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Volume · volume of the selected closed meshes, solid BReps and elements; open ones are refused",
    "names": [
      "Volume"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": null,
    "bare": [
      {
        "line": "Add Group",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Add Group [name] · put the selection under a new group node; a click on one member then selects them all",
    "names": [
      "Add Group"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  },
  {
    "alias_results": [],
    "aliases": [],
    "arity": 0,
    "bare": [
      {
        "line": "Add Edge",
        "result": {
          "accepted": true,
          "keeps_draft": false,
          "keeps_split": false,
          "selection": true
        }
      }
    ],
    "hint": "Add Edge · connect the two selected objects in the session graph; the Graph table of the layers panel lists it",
    "names": [
      "Add Edge"
    ],
    "options": [],
    "wait_after_option": false,
    "wait_for_option": false
  }
]
```

### 2. `src/inventory_tests.rs`

Copy supplied complete source/registry and explicit grammar/shortcut fixtures.

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
fn maintained_root() -> std::path::PathBuf {
    let manifest = include_str!("../Cargo.toml");
    let line = manifest
        .lines()
        .find(|line| line.starts_with("current_viewer = "))
        .expect("Maintained native reference dependency");
    let path = line
        .split("path = \"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn all_declared_current_rust_sources_and_external_dependencies_match() {
    let snapshot = crate::source_audit::recorded();
    assert_eq!(snapshot.session_head.len(), 40);
    assert_eq!(snapshot.wood_head.len(), 40);
    crate::source_audit::verify(&maintained_root(), &snapshot).unwrap();
    assert_eq!(
        snapshot
            .files
            .iter()
            .filter(|v| v.root == "viewer")
            .count(),
        312
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn all_current_registered_names_options_and_aliases_match_the_observed_grammar() {
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("current_registry.json")).unwrap();
    assert_eq!(crate::registry_audit::snapshot(), expected);
    let rows = expected.as_array().unwrap();
    assert_eq!(rows.len(), 86);
    assert_eq!(
        rows.iter()
            .map(|v| v["options"].as_array().unwrap().len())
            .sum::<usize>(),
        117
    );
    assert_eq!(
        rows.iter()
            .map(|v| v["aliases"].as_array().unwrap().len())
            .sum::<usize>(),
        15
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn current_command_differences_and_canvas_letters_are_explicit() {
    use current_viewer::app::{command, keys};
    assert!(command::parse("Controls").is_ok());
    assert!(command::parse("Controls On").is_err());
    assert!(crate::commands::parse("Controls On").is_ok());
    assert!(command::parse("Opacity 0.4 extra").is_ok());
    assert!(crate::commands::parse("Opacity 0.4 extra").is_err());
    for line in [
        "Opacity NaN",
        "Opacity -1",
        "Opacity 2",
        "Length BoundingBox extra",
        "BoundingBox extra",
    ] {
        assert!(command::parse(line).is_err(), "{line}");
    }
    for (letter, line) in [("h", "Hide"), ("S", "Show"), ("f", "Fit")] {
        assert_eq!(keys::letter(letter), Some(line));
        assert!(command::parse(line).is_ok());
    }
    assert_eq!(keys::letter("q"), None);
}
```

### 3. `src/lib.rs`

Register supplied complete source and registry fixtures.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod metadata_tests;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod metadata_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod inventory_tests;
pub mod source_mesh_topology;
```

### 4. `src/current_inventory.json`

Copy the October 23 source inventory: 312 viewer, 109 kernel and 12 Wood source/build entries. Repository heads identify baselines; exact file hashes identify the recorded working tree.

Replace the entire file with:

```json
{
  "session_head": "a391b10473d580ef548d2d1ba3ea118986623b52",
  "wood_head": "07c57552c0d18c403bab0c16ba858d80561ffc47",
  "kernel_head": "d814b238cf42102622c34fdff5bced1814418b97",
  "scope": "All files under the three Rust src trees and existing Cargo/build/web entries. Includes the preserved local rational-control change. Assets are separately hashed by the dependency evidence closure; C++ and generated models require their own provenance. Includes preserved October 10 hierarchy-path and parent Element-lamp changes; the original inventory remains historical. October 14 refresh records seven changed kernel/wood inputs and current repository heads; all viewer file hashes and the historical inventories remain unchanged. Checkpoint 1276 separately records regenerated Session protocol documentation comments about LOCAL placement; protocol fields and executable code are unchanged. This final-checkpoint refresh hashes current working-tree bytes. Git HEADs are baseline identifiers, not proof of a clean tree; dirty and untracked paths are recorded separately. It verifies declared file hashes and inventory only, not compilation, runtime behavior or visual acceptance. This final-checkpoint refresh hashes current working-tree bytes. Git HEADs are baseline identifiers, not proof of a clean tree; dirty and untracked paths are recorded separately. It verifies declared file hashes and inventory only, not compilation, runtime behavior or visual acceptance. This final-checkpoint refresh hashes current working-tree bytes. Git HEADs are baseline identifiers, not proof of a clean tree; dirty and untracked paths are recorded separately. It verifies declared file hashes and inventory only, not compilation, runtime behavior or visual acceptance. This final-checkpoint refresh hashes current working-tree bytes. Git HEADs are baseline identifiers, not proof of a clean tree; dirty and untracked paths are recorded separately. It verifies declared file hashes and inventory only, not compilation, runtime behavior or visual acceptance.",
  "working_tree": {
    "repositories": [
      {
        "name": "session",
        "head": "a391b10473d580ef548d2d1ba3ea118986623b52",
        "dirty": true,
        "entries": [
          {
            "status": "M ",
            "path": "gitattributes"
          },
          {
            "status": " M",
            "path": ".gitignore"
          },
          {
            "status": " M",
            "path": "session_data"
          },
          {
            "status": " M",
            "path": "session_py"
          },
          {
            "status": " M",
            "path": "session_rust"
          },
          {
            "status": " M",
            "path": "session_tests/README.md"
          },
          {
            "status": " M",
            "path": "session_tests/plugins/course.ts"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/build-viewer-docs.sh"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/course.ts"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/course/checkpoints.test.ts"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/course/reference-checks.ts"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/course/reference.ts"
          },
          {
            "status": " M",
            "path": "session_tests/scripts/course/verify.ts"
          },
          {
            "status": " M",
            "path": "session_tests/src/codeTheme.ts"
          },
          {
            "status": " M",
            "path": "session_tests/src/views/CourseIndex.vue"
          },
          {
            "status": " M",
            "path": "session_viewer/Cargo.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/capture_current.cjs"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/capture_journey.cjs"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/96zzzzzzzq-draw-original-cad-wire-curves.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/book.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/checks/data/current-grid-physical-256-2026-10-15.ppm"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/checks/data/current-grid-physical-512-2026-10-15.ppm"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/96zzc-share-section-addresses-with-outlines-and-ambient.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/96zzzzzt-share-depth-endpoints-and-comparisons.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/96zzzzzza-match-physical-camera-defaults-and-limits.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/96zzzzzzn-respect-original-boundary-pens.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/96zzzzzzzm-retain-visible-instance-ranges.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/chapter-19.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/chapter-20.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/chapter-21.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/chapter-22.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/consolidation/current-state.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/contents.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/course.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/daily-plan.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/frame.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/production-course-map.json"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/production-course-map.md"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/15-perspective.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/23-import.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/29-identity.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/92-Cargo.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/94-Cargo.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/Cargo-publish.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/Cargo-text.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/docs/journey/release/source-residency-Cargo.lock"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/command/verbs/text.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/cplane.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/decode.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/edit.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/inspection.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/live.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/loader.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/manifest.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/scene.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/scene_text.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/walk/matrix.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/app/walk/mod.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/camera.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/backdrop.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/buffers.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/frame.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/lane.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/matrices.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/mod.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/segments.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/splat.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/gpu/text_plane.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/engine/text.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/lib.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/selftest.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/arrow.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/clip.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/grid.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/ink_visibility.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/matrix.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/ribbon.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/shaders/text_plane.wgsl"
          },
          {
            "status": " M",
            "path": "session_viewer/src/state.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/state/drawing.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/state/features.rs"
          },
          {
            "status": " M",
            "path": "session_viewer/src/state/text.rs"
          },
          {
            "status": "??",
            "path": "docs/plans/2026-10-18-viewer-course-progress.md"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/build-viewer-docs.ps1"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/check-consolidated-course.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/check-software-webgpu.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/cad-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/check-reload-grid.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/cloud-ink-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/cloud-lod-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/cloud-witness-course.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/cloud-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/glyph-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/grid-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/measure-runtime.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/plane-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/section-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/sheet-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/stroke-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/course/vector-witness.rs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/run-guarded.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/run-guarded.cmd"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/run-guarded.ps1"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/run-guarded.test.cjs"
          },
          {
            "status": "??",
            "path": "session_tests/scripts/warp-adapter.ps1"
          },
          {
            "status": "??",
            "path": "session_viewer/.gitattributes"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/checks/data/current-grid-physical-256-windows-2026-10-23.ppm"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/checks/data/current-grid-physical-512-windows-2026-10-23.ppm"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-01-bounded-source.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-02-generic-range-fixture.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-03-hidden-range-fixture.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-04-absolute-range-fixture.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-05-sphere-abi.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-06-dense-sphere-abi.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-07-mesh-ribbon-abi.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-08-mesh-family-round-caps.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-09-isolated-outline-composite.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-active-lane-10-remove-duplicate-mesh-ink.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-canonical-source-audit.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cap-test-warp-renderer.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cap-test-warp-wait.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-clipped-head-source-cap.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-12.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-13.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-14.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-15.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-16.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-17.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-18.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-19.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-20.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-21.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-adapter-22.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-control-owner-test.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-placement-register.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-placement-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-point-ids.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-production-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-record.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-resolve-source.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-cloud-source.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-dependency-cargo.toml"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-dependency-inventory-example.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-dependency-reference-example.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-dependency-reference-module.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-dependency-registry-module.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-finite-ink-lazy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-frame-pick-01-frame-ink-class.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-frame-pick-02-frame-colour-order.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-frame-pick-03-frame-query-order.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-frame-pick-04-analytic-filter-support.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-frame-pick-05-contact-diagnostic.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-bounds_original-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-browser-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-browser_defaults-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-cloud_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-commands-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-commands-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-display_settings.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-editor-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-editor-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-editor-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-editor-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-editor-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-highlight-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-lib-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-lod-policy-1.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-lod-policy-2.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-mesh_parts-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-mesh_parts-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-mesh_parts-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-plane_original-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-point-policy-1.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-point-policy-2.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_glyph.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_glyph.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_glyph_gpu.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_glyph_provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_glyph_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_joined-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_joined-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_joined-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_matrix_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_matrix_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_matrix_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_matrix_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_ribbon_gpu-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_sheet_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_sheet_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_sheet_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_sheet_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-production_sphere.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-renderer-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-glyph-scene-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-headed-terminal-coverage.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-hidden-ink-frame.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-hidden-ink-lane-state.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-hidden-ink-state.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-hidden-ink-test.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ink-plane-fallback.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-inventory-refresh-01-current_inventory.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-inventory-refresh-02-viewer-count.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-inventory-refresh-3-01-current_inventory.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-inventory-refresh-4-01-current_inventory.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-inventory-refresh-4-02-viewer-count.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-legacy-stroke-lazy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-line-placement-gpu-register.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-line-placement-gpu-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-12.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-13.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-14.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-15.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-16.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-17.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-18.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-19.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-20.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-21.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-22.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-23.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-24.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-25.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-26.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-27.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-28.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-29.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-30.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-31.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-32.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-33.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-34.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-35.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-36.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-37.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-38.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-39.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-40.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-41.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-42.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-43.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-44.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-45.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-46.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-47.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-48.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-49.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-50.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-51.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-adapter-52.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-matrix_compact_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix-01.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix-02.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_provenance-01.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_provenance-02.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-production_matrix_provenance-03.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-compact-scene-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-matrix_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-production_matrix.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-production_matrix.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-production_matrix_gpu.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-production_matrix_provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-production_matrix_walk.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-bounds_original-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-cad_display-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-cad_shape_cache-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-highlight-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-highlight-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-lib-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-mesh_parts-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-mesh_parts-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-mesh_parts-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-pick_sources-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-pick_sources-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-pick_sources-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-pick_sources-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-plane_original-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_brep_ink.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_joined-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_joined-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_joined-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_mesh_ink.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_mesh_ink_provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_mesh_ink_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-production_ribbon_gpu-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-renderer-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-renderer-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-renderer-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-scene-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-mesh-ink-surface-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-grid-reference-selection.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-reconciliation-01-pick-pass-order.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-reconciliation-02-outline-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-reconciliation-03-outline-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-reconciliation-04-outline-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-native-reconciliation-05-outline-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-check.cjs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-frame.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-install.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-owner.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-pagehide.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-present.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-request.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-frame-schedule-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-frame-schedule.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-lifecycle-change.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-lifecycle-install.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-lifecycle-stop.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-modules.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-runtime.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-pick-priority-production_glyph-01.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-pick-priority-production_glyph_provenance-01.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-pick-priority-production_glyph_provenance-02.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-pick-priority-production_ribbon_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-pick-priority-production_sphere-01.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-browser-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-commands-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-commands-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-editor-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-editor-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-editor-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-editor-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-editor-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-lib-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-owner-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-plane_original-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_joined-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_joined-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_joined-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_plane.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_plane.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_plane_provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_plane_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_ribbon_gpu-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_vector_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_vector_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-production_vector_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-renderer-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-plane-scene-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-12.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-13.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-14.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-15.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-16.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-17.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-18.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-19.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-20.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-21.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-22.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-23.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-24.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-25.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-26.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-27.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-28.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-29.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-30.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-31.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-32.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-alpha-layout.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-alpha-offset.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-alpha-record.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-alpha-shader.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-alpha-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-empty-sample-guard.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-lane.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-shader.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-print-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-production-colour-policy-01-unorm-grey.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-production-colour-policy-02-selected-source-edge.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-production-contact-boundary.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-production-outline-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-01-homogeneous-lens.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-02-cap-diagnostic.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-03-plane-fixture-size.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-04-printed-query-policy.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-05-precise-origin-owner.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-projection-adapter-06-surface-cache-fixture.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-protobuf-decode.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-protobuf-phase-test.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-protobuf-publication-test.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-reload-grid-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-12.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-13.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-14.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-15.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-16.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-17.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-18.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-19.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-20.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-21.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-22.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-adapter-23.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-gpu.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-rows.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-selection-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-ribbon-source.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-ambient-contact-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-cap-gpu-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-clipping-gpu-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-command-inventory-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-contact-boundary-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-finite-visibility-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-frame-target-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-grid-hardware-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-helper.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-pick-gpu-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-register.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-sample-composition-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-sample-positions-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-text-atlas-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-tile-gpu-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-adapter-widget-tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-software-wait-budget.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-source-address-inventory-refresh-01-pick-sources.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-source-address-inventory-refresh-02-scene-origin.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-highlight-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-highlight-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-highlight-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-lib-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-pick_encode-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-pick_encode-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-pick_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-pick_sources-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_curve_walk.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_joined.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_joined.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-10.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-11.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-12.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-13.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-14.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-15.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-16.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-17.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_ribbon_gpu-18.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_sheet.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_sheet_gpu.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_vector.wgsl"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_vector_gpu.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_vector_provenance.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-production_vector_tests.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-01.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-02.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-03.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-04.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-05.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-06.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-07.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-08.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-renderer-09.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-vector-scene-tests-register.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-whole-owner-fixture-1.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-whole-owner-fixture-2.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-windows-grid-reference.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/consolidation/history/2026-10-17-linux-state.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/current-viewer-2026-10-23.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/fixtures/Cargo.lock"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/fixtures/Cargo.toml"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/fixtures/README.md"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/fixtures/src/main.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/fixtures/src/ordered_mesh.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/redraft-10-sections/book-10-sections.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/redraft-10-sections/old-to-new-coverage-map.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/release/grid-native-review.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/release/grid-windows-pair-review.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/release/matrix-native-review.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/journey/release/mesh-native-review.json"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-grid-fractional-native.png"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-grid-zero-native.png"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-matrix-native-1x.png"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-matrix-selected-native-4x.png"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-mesh-selected-native-4x.png"
          },
          {
            "status": "??",
            "path": "session_viewer/docs/screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-mesh-unselected-native-4x.png"
          },
          {
            "status": "??",
            "path": "session_viewer/src/engine/gpu/splat/cloud_witness.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/src/engine/gpu/splat/cloud_witness/lod.rs"
          },
          {
            "status": "??",
            "path": "session_viewer/src/selftest/stroke_witness.rs"
          }
        ]
      },
      {
        "name": "wood",
        "head": "07c57552c0d18c403bab0c16ba858d80561ffc47",
        "dirty": false,
        "entries": []
      },
      {
        "name": "kernel",
        "head": "d814b238cf42102622c34fdff5bced1814418b97",
        "dirty": true,
        "entries": [
          {
            "status": "M ",
            "path": "ession_data"
          }
        ]
      }
    ]
  },
  "files": [
    {
      "root": "viewer",
      "path": "Cargo.lock",
      "sha256": "a84bbf92c6a14f11a5261587f6059387aca83e453e11e4d71c5c4d2bde70d437",
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
      "sha256": "b597b1d06c7d8bbfcdf80a16d71de27bc3aa2f20c10a57742ac58d3cb027742a",
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
      "path": "src/app/command/tool.rs",
      "sha256": "c08608743e48879cab82f76157d5f028215af1848cd5b9eeefc343f039001b1b",
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
      "path": "src/app/command/verbs/extend.rs",
      "sha256": "c86295b7fd4314a6ba86581c4ea12d142d0f867ff7698c4b542ce3b0ffe33de3",
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
      "sha256": "1a4836207a54e368ad3d91d6e53df5c089e4584b8ce53dc5bd7b0bcfe3750d0e",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97"
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
      "sha256": "e083f13a0cb4da2a4fd6c3bf13e733bf81fa6768a563c7c74959d65c8ea5797f",
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
      "path": "src/app/command/verbs/trim.rs",
      "sha256": "60c27bce49c24b3867c658171267e1c159c24e0c1293de34067711f2c7c4b72d",
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
      "sha256": "f2b685069fd3af7bf40147c7faa87aca9f066a3ecc99b9a0f935e8abcdd27aef",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/decode.rs",
      "sha256": "f517ebac6f210739f04da54a71b504471b78e82313d2160bb141ff9b9aa0caf6",
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
      "sha256": "5fc69027ff1534a13a784eac4643f5150a06f92ac9bafa4f6005eb50cc5fe81c",
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
      "path": "src/app/inspection.rs",
      "sha256": "b3ddba11292063a66e625d9d31c0576d3df341362fad1841266edc462f0bffd3",
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
      "path": "src/app/interactions.rs",
      "sha256": "d0ff1f7f682dc03297e79054202c860598862d6e83db1e7033c858bfbcae5b9c",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97"
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
      "sha256": "3ca1a9e32ef233b4adffe0d79583cc7cc18b9131f7537d206413b3b700047a5e",
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
      "sha256": "5ee1687239963ba6ce1403e2a1270ded6f6fbf3632d8b7ba85bea48055b4791b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/manifest.rs",
      "sha256": "c75d33f25078645cc91d71df05aa0c15e30c31f9bebc30522c67d875f5a91066",
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
      "sha256": "136710a388a2fdf301fa5e36e68c4b6e3a79f1dd903ef3f5e25e3ceb26f0e196",
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
      "path": "src/app/scene_sync.rs",
      "sha256": "7160ce61ef8ba840ef536bff9532ed315b7c2b613301dc7993b61105411d0126",
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
      "path": "src/app/scene_text.rs",
      "sha256": "7294f84b7b73e0db7adbeb62ebab58020032c3d92e0cb6d4763a48f70e66e8d5",
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
      "path": "src/app/ui/command_line.rs",
      "sha256": "e1465627dfb1b6afa326bbed8ce025de3ec0daa4ec4f0d34504099475e3d1bc0",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
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
      "path": "src/app/walk/matrix.rs",
      "sha256": "20bbee6ae44db1f8fee23ca919d3d535f224cf291f4c64d2258ede5f8bd7f62d",
      "topic": "Matrix rendering and exact source ownership",
      "lessons": "20,21,96"
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
      "sha256": "ac677c9a11ddbfa3e995443b0bf1548b105ced70320a6208b9956d7eba86bce1",
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
      "sha256": "be1baf4e1136824f315f4b61bd7db24641e582218771b1593c7c8690667c1aee",
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
      "sha256": "b7adaa194064221e4f89bf765f57899ed5a8d7ecd2ebb66b2709437206994e4d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/buffers.rs",
      "sha256": "2f1344481cb3527327e28c8a26731eafbf925b8a1fa81ff9cc89740be011aa6c",
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
      "sha256": "4e4d3a6251be85d0bc246ffbead385bd0361a11b4b73245f9980710fa0032228",
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
      "sha256": "e645f86ba853cf40c81dd35f34e7d59790ab818a241b9374951cb19ea1d10f11",
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
      "path": "src/engine/gpu/matrices.rs",
      "sha256": "00f606b5056037de1bf3923c7874202bd1877af7591759713e48ca46a3d5a1de",
      "topic": "Matrix rendering and exact source ownership",
      "lessons": "20,21,96"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/mod.rs",
      "sha256": "3c72e20c22dd889b45cb1f4b84c7f34aa1cf385f1099fbe437875814d652d9b0",
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
      "sha256": "a152ead3eb15a3e44ceab8f11cb3d804c549d879a4871d1897c9bdeaf760aeb6",
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
      "sha256": "d73fee1ea1d342651adc059ffd6c0c5faef1eb1046a055c6e722ba7299c843ea",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/splat/cloud_witness.rs",
      "sha256": "10b59c7dccdd2e4cec910cd79c5ed63a9afa8353e86b84657c647d840554d862",
      "topic": "Cloud rendering and current evidence",
      "lessons": "11,18,22,96"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/splat/cloud_witness/lod.rs",
      "sha256": "4356bca21aa40df54efb546d48f6d489177893ca7a0dc810a9158f4905f1e2e8",
      "topic": "Cloud rendering and current evidence",
      "lessons": "11,18,22,96"
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
      "path": "src/engine/gpu/surface_outline.rs",
      "sha256": "08e790732dcd0ff16abe734be9d13b63dc26e6825f9d1d7578645ee529e4db03",
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
      "sha256": "c359f7b06e9db8bdc940f880007cbee6e66f51feb8878e771454f9798cfdc524",
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
      "sha256": "7ad70d8aa3b0eabdbeed302b50af41174a767d85f46faabf8d592012b111edab",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/lib.rs",
      "sha256": "ae8178590bd125a75f973fb746afcca1d6c05b1a8cc49738c1ceec27ccfc5596",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/selftest.rs",
      "sha256": "5fde63ff1c457acaf1bf1f7d1a83b7bbd762cdbbef6b77b340fd411945f59ea1",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
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
      "path": "src/selftest/stroke_witness.rs",
      "sha256": "493e57a9a0a049e906738bfca8072947d022607c3458c4c0b53b2ed5d8b6ad4b",
      "topic": "Line rendering and current evidence",
      "lessons": "5,18,22,96"
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
      "sha256": "41166470f26b141dda7c03a753e14542dfdf3ed572f7f6920850c09afb03b80d",
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
      "sha256": "975782a36e64719e00eba623a42f28a39919c03035a71525e384d97ad3837544",
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
      "sha256": "094166f31a5f73bcf7a3957f077a2e299362b1e66520d3325b93a81e8bc922ea",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ink_visibility.wgsl",
      "sha256": "17111b9fc09de6d3f344f39f83f31d4dfbb8c57f92fe1763f635f73cb0a3a475",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/matrix.wgsl",
      "sha256": "0f319930e6a82b89909f4d71447b2d1b2d82cd11f2c4071f5f62e635e83ce620",
      "topic": "Matrix rendering and exact source ownership",
      "lessons": "20,21,96"
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
      "sha256": "950172d6d383b37009bd9c8d6e1edb63adf835e7b2926e91aba043f6a22f9f3f",
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
      "sha256": "8694a4d32e4676415b28fa4727bf4c1645862e80f4cc4e65e41c79ab1e5451c3",
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
      "path": "src/state.rs",
      "sha256": "8bd89dd783fc205afd1cd8abb4e0323a7f61a9569514227a315557cc0038558a",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
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
      "sha256": "bdd056fc264725d2283b9a55ba8ae5442d4f0e537808d537dfa00dadf539f1c5",
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
      "sha256": "3bf04533e9a97fa373f7d3c4a3cdeaf49bcb3535235e725f6012f461d5f14c98",
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
      "sha256": "fab03acef3a6796e15211bba69b00b60c9e512448d3ea34c7817d00c5763feb2",
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
      "path": "src/text_quality.rs",
      "sha256": "7fef20819989ace4919f20705cdda904656fe960ea557028e9a5dc7b34b68edc",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.lock",
      "sha256": "4778cc98a58deef1e035ad348ab4ec0b02fd732f08c9603f4d8057dd94a2a3d9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.toml",
      "sha256": "36bbbbb92d911ed1609118149d1a42bd5b8c74e7f246a09114a7eb8871a083c3",
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
      "sha256": "5f110dad59f0251883aca3b34f4f602ce88ac619a3afd95af75a68eb8a4c38fc",
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
      "sha256": "016bd1d44a6e900746789dde6ea158cc215255360472ab106b1efc005643dec5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix_test.rs",
      "sha256": "19869219588ff378223934cbd6c6551e1d34c5b3c7da35bf8ce581b10034d402",
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
      "sha256": "4464b762352c7040c47f33eab71627a930fd043921b6eebcced0320fc1d857d6",
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
      "sha256": "c466fd62352c9f4408fc722c060c993c79b5338ac1a4d6adcf4ce21954c64cb6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline_test.rs",
      "sha256": "9c77584b465a8edf79d6a40a1bf08fa9eb9843ff0c5e3b50cca98fa349a18050",
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
      "sha256": "7e54beecf7e3c5264f4165220cd05be8ef76c13627a4ddea6849124adf6fb4c2",
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
      "sha256": "3024289a03b2304f4c5547f0bb6f114f15de56633bc7cfe187ad2f42677ffb67",
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
      "sha256": "61c844737a6ec3074a0b02c0e4a6d9bae65ce2afafc7e2a71cd95263b01938b1",
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
      "sha256": "4e6ffd383e74febf21ee80a337db1c18651307c23f0a4ea2471a790c28769f07",
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
      "sha256": "ae7367198c7f8fe9cfc05e08a90818e18b6a2e340ad3daf90e279b0ca417d870",
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
  ]
}
```

### 5. `src/source_audit.rs`

Verify valid UTF-8 source text with CRLF normalized to LF. Hash invalid UTF-8 as raw bytes; preserve exact file-set and SHA comparisons.

**Locate this existing block:**

```rust
            .map_err(|e| format!("{}: {e}", row.path))?;
        if crate::origin::FileVersion::of(&bytes).hex() != row.sha256 {
            return Err(format!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .map_err(|e| format!("{}: {e}", row.path))?;
        let text = std::str::from_utf8(&bytes)
            .ok()
            .map(|text| text.replace("\r\n", "\n"));
        let bytes = text
            .as_ref()
            .map_or(bytes.as_slice(), |text| text.as_bytes());
        if crate::origin::FileVersion::of(bytes).hex() != row.sha256 {
            return Err(format!(
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 19.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 19.1
npm --prefix ../session_tests run course -- save 19.1
```

Continue with [19.2 · Original Element ancestry and derived feature trees](96i-separate-own-and-hosted-element-features.md).
