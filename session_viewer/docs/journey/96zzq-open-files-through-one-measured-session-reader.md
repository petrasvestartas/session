# 20.6.2 · Open files through one measured Session reader

**Chapter 20 · Shared GPU rendering · Section 20.6 · Current file and command behavior** · [Contents](contents.md)

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Use one full Session reader for browser files. Reuse the phase timer for decode, validation, kernel reconstruction and the actual display walk. Retain the source reload location, and adopt the candidate only after preparation succeeds. Keep archive opening on its existing path.

## Type

Continue from [Classify contact fill shading](96zzp-classify-contact-fill-shading.md). [Save or recover your work](recovery.md).

### 1. `src/load_measure.rs`

Let the existing timer observe String errors without duplicating its clock and completion algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn run<T>(name: &str, bytes: u64, clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>), operation: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-1.rs" skip=200
```

### 2. `src/document_source.rs`

Expose observations on the existing full reader and share its browser callback. Record the actual decode and a stable malformed-payload error. Start validation around the existing complete schema, identities, tree, placements and prepared-sheet checks. Finish validation and retain the original reload location in the one source origin. Measure the actual kernel reconstruction once.

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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn observe(phase:crate::load_measure::Completed<'_>){
 #[cfg(target_arch="wasm32")] crate::browser_phase::completed(phase,"document");
 #[cfg(not(target_arch="wasm32"))] let _=phase;
}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String>{prepare_at(bytes,place,point_px,None)}
pub(crate) fn prepare_at(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<Source,String>{
 prepare_observed(bytes,place,point_px,location,&mut crate::load_measure::now,&mut observe)
}
pub(crate) fn prepare_observed(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>,
 clock:&mut impl FnMut()->f64,observe:&mut impl FnMut(crate::load_measure::Completed<'_>))->Result<Source,String>{
 use crate::load_measure::run_result;
 let size=bytes.len()as u64;
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=run_result("decode",size,clock,observe,||proto::Session::decode(bytes).map_err(|_|"Invalid session protobuf".to_owned()))?;
    let (tree,sheets)=run_result("validation",size,clock,observe,||->Result<_,String>{
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
    Ok((tree,sheets))})?;
    let mut origin=crate::origin::Origin::new(&message,bytes);origin.location=location;let origin=Rc::new(origin);
    let document=run_result("kernel",size,clock,observe,||Session::from_proto(message).map(Rc::new).map_err(|e|e.to_string()))?;
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
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-2.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-3.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-4.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-5.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-6.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/source_phase_tests.rs`

Supply fixtures against actual original Element/interaction bytes, exact phase order and failure boundaries, and retained source locations.

Copy this supplied file:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-7.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional full-source observation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
ce_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-8.rs" skip=180
```

</details>

### 5. `src/scene_document.rs`

Reuse original-document import while allowing the browser file origin to retain its reload location. Use the measured full reader once. Time the existing original geometry, hierarchy and feature preparation as the actual display walk. Adopt the complete candidate only after a successful measured walk.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}
```

</details>

Replace it with:

```rust
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    Ok(key)})?;
    scene.restore(candidate);Ok(key)
}
```

<details>
<summary>Locate this block</summary>

```rust
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String> {
    let source=crate::document_source::prepare(bytes,place,point_px)?;
    let mut candidate=scene.clone();let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
```

</details>

Replace it with:

```rust
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String>{import_at(scene,bytes,name,place,point_px,None)}
pub fn import_at(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<u32,String>{
    let source=crate::document_source::prepare_at(bytes,place,point_px,location)?;
    let mut candidate=scene.clone();
    let key=crate::load_measure::run_result("display walk",bytes.len()as u64,&mut crate::load_measure::now,&mut crate::document_source::observe,||->Result<u32,String>{
    let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;
#[cfg(test)]
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String>{import_at(scene,bytes,name,place,point_px,None)}
pub fn import_at(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<u32,String>{
    let source=crate::document_source::prepare_at(bytes,place,point_px,location)?;
    let mut candidate=scene.clone();
    let key=crate::load_measure::run_result("display walk",bytes.len()as u64,&mut crate::load_measure::now,&mut crate::document_source::observe,||->Result<u32,String>{
    let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
    candidate.forest.add(key,if name.is_empty() {source.document.name.clone()} else {name.into()},source.tree,&owners);
    candidate.forest.original_placements(key,&source.document,place,&source.world);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let elements = owners.iter().filter_map(|(guid, id, _)|
        (matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_)))
            || candidate.instances.contains_key(id)).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner).map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    Ok(key)})?;
    scene.restore(candidate);Ok(key)
}

pub fn open(scene:&mut Scene,bytes:&[u8])->Result<(),String> {
    let bundle=crate::archive::Bundle::decode(bytes)?;
    let mut candidate=scene.clone();candidate.clear();let mut keys=Vec::new();
    for (source,meta) in bundle.documents.iter().zip(&bundle.metadata.documents) {keys.push(import(&mut candidate,source,&meta.name,&Xform::from_matrix(meta.place),meta.point_px)?);}
    crate::archive_restore_texts::restore(&mut candidate,&bundle.metadata)?;
    crate::archive_restore_flags::restore(&mut candidate,&bundle.metadata,&keys)?;
    scene.restore(candidate);Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_save_open_restores_original_sources_placements_layers_and_presentation() {
        let mut scene=Scene::demo();scene.clear();
        let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();
        let surface=scene.insert_surface(Rc::new(crate::surface::example())).unwrap();
        let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();
        let copy=scene.copy_row(cloud,false).unwrap();scene.place_cloud(copy,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        scene.cloud_point_size(cloud,5.).unwrap();scene.cloud_point_size(copy,5.).unwrap();
        let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.123456789012345,0.,0.))).unwrap()).unwrap();
        let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-1.,0.,0.,1.,0.,0.))).unwrap()).unwrap();
        let label=crate::scene_label::example(crate::text::TextPlacement::WorldBillboard {world:[0.,0.,0.],world_height:0.1});
        let text=scene.insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap()).unwrap();scene.place_text(text,Xform::translation(0.123456789012345,0.,0.)).unwrap();
        let root=&scene.forest.documents[0];let current=format!("{}/{}",root.key,root.nodes[0].source.guid);
        let view=Rc::make_mut(&mut scene.view);view.current=Some(current);view.hide(&[cad,text],true);view.lock(&[surface,line,text],true);view.faces.insert(cad,[2,5,7]);view.edges.insert(cad,[0;3]);
        let mut restored=Scene::demo();let old_ids=restored.objects().iter().map(|row|row.id.value()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&scene).unwrap();open(&mut restored,&bytes).unwrap();
        assert_eq!(restored.objects().len(),2);assert_eq!(restored.objects().iter().filter(|row|row.cad.is_some()).count(),1);assert_eq!(restored.objects().iter().filter(|row|row.surface.is_some()).count(),1);
        assert_eq!(restored.clouds().len(),2);assert_eq!(restored.clouds()[1].model.m[12],1_000_000_000.125);assert_eq!(restored.clouds()[0].prepared.source.to_proto().coords,scene.clouds()[0].prepared.source.to_proto().coords);assert!(restored.clouds().iter().all(|row|row.point_px==5.&&row.prepared.source.point_size==3.));
        assert_eq!(restored.points()[0].prepared.source[0],scene.points()[0].prepared.source[0]);assert_eq!(restored.points()[0].prepared.source[0],0.123456789012345);let _=point;
        assert_eq!(restored.texts().len(),1);assert_eq!(restored.texts()[0].label(false,false).unwrap().placement,scene.texts()[0].label(false,false).unwrap().placement);assert!(!restored.visible(restored.texts()[0].id));assert!(restored.view.locked.contains(&restored.texts()[0].id));
        assert!(restored.view.current.is_some());assert!(restored.created.is_some());assert_eq!(restored.view.hidden.len(),2);assert_eq!(restored.view.locked.len(),3);assert_eq!(restored.view.faces[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[2,5,7]);assert_eq!(restored.view.edges[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[0;3]);assert!(restored.objects().iter().all(|row|!old_ids.contains(&row.id.value())));
        let again=crate::archive::Bundle::decode(&crate::scene_archive::save(&restored).unwrap()).unwrap();assert_eq!(again.documents.len(),2);assert_eq!(again.metadata.hidden.len(),2);assert_eq!(again.metadata.locked.len(),3);
    }
    #[test]
    fn repeated_guid_documents_reopen_with_independent_flags_and_original_meshes() {
        let mut scene=Scene::demo();scene.clear();let raw=crate::specimen::bytes();scene.import(crate::document::load(&raw).unwrap()).unwrap();let n=scene.objects().len();scene.import(crate::document::load(&raw).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);
        let saved=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::demo();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);let a=&restored.objects()[0];let b=&restored.objects()[n];assert_ne!(a.guid,b.guid);assert_eq!(a.geometry().unwrap().guid(),b.geometry().unwrap().guid());assert!(!restored.visible(a.id));assert!(restored.visible(b.id));assert!(!restored.view.locked.contains(&a.id));assert!(restored.view.locked.contains(&b.id));
        let document=Rc::downgrade(&a.source().unwrap().document);let ids=restored.objects().iter().take(n).map(|row|row.id).collect::<Vec<_>>();for id in ids {assert!(restored.remove(id));}assert!(document.upgrade().is_none());
    }
    #[test]
    fn rejected_files_and_late_display_preparation_errors_leave_the_scene_unchanged() {
        let mut scene=Scene::demo();let ids=scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();assert!(open(&mut scene,b"broken").is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.tree=None;source.objects.as_mut().unwrap().points.push(session_rust::Point::new(1e100,0.,0.).to_proto());let bytes=source.encode_to_vec();assert!(import(&mut scene,&bytes,"too large",&Xform::identity(),0.).is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);assert!(scene.points().is_empty());assert!(scene.forest.documents.iter().all(|doc|doc.label!="too large"));
        source.objects.as_mut().unwrap().points.clear();source.definitions=Some(proto::Objects {instances:vec![Default::default()],..Default::default()});assert!(import(&mut scene,&source.encode_to_vec(),"unsupported",&Xform::identity(),0.).unwrap_err().contains("definition"));assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
    }
    #[test]
    fn document_placement_is_composed_without_mutating_original_source_coordinates() {
        let mut scene=Scene::demo();scene.clear();let cloud=crate::cloud::example();let _=cloud.guid();let original=cloud.to_proto();let source=proto::Session {name:"precise cloud".into(),objects:Some(proto::Objects {pointclouds:vec![original.clone()],..Default::default()}),..Default::default()};let raw=source.encode_to_vec();let place=Xform::translation(1_000_000_000.125,0.,0.);import(&mut scene,&raw,"placed",&place,7.).unwrap();assert_eq!(scene.clouds()[0].model.m[12],place.m[12]);assert_eq!(scene.clouds()[0].prepared.source.to_proto(),original);assert_eq!(scene.clouds()[0].point_px,7.);
    }
    #[test]
    fn repeated_roundtrips_keep_tree_depth_current_layer_source_header_and_precise_pose() {
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.guid="00000000-0000-0000-0000-000000000123".into();let raw=source.encode_to_vec();let mut scene=Scene::empty();import(&mut scene,&raw,"placed",&Xform::translation(0.123456789012345,0.,0.),0.).unwrap();let doc=&scene.forest.documents[0];let current=format!("{}/{}",doc.key,doc.nodes[0].source.guid);Rc::make_mut(&mut scene.view).current=Some(current);let nodes=doc.nodes.len();let depth=doc.nodes.iter().map(|node|node.depth).max().unwrap();
        for _ in 0..6 {let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();assert_eq!(proto::Session::decode(bundle.documents[0].as_slice()).unwrap().guid,source.guid);open(&mut scene,&saved).unwrap();assert_eq!(scene.forest.documents[0].nodes.len(),nodes);assert_eq!(scene.forest.documents[0].nodes.iter().map(|node|node.depth).max().unwrap(),depth);assert_eq!(scene.objects()[0].model.m[12],0.123456789012345);assert!(scene.view.current.is_some());}
    }
    #[test]
    fn moving_repeated_source_objects_into_one_layer_saves_distinct_editable_records() {
        let raw=crate::specimen::bytes();let mut scene=Scene::empty();import(&mut scene,&raw,"first",&Xform::identity(),0.).unwrap();let n=scene.objects().len();import(&mut scene,&raw,"second",&Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;let doc=&scene.forest.documents[0];let key=format!("{}/{}",doc.key,doc.nodes[0].source.guid);crate::layer_edit::apply(&mut scene,&key,crate::layer_edit::Edit::Move(vec![b])).unwrap();
        let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();let source=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();let meshes=source.objects.unwrap().meshes;assert_eq!(meshes.len(),n+1);let guids=meshes.iter().map(|mesh|&mesh.guid).collect::<HashSet<_>>();assert_eq!(guids.len(),n+1);assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().guid(),scene.objects().iter().find(|row|row.id==b).unwrap().geometry().unwrap().guid());let mut restored=Scene::empty();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);
    }

}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-9.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-10.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-11.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-12.rs"
```

</details>

### 6. `src/publication.rs`

Remove the older mesh reader and unmeasured fallback; preserve the existing archive path.

<details>
<summary>Locate the existing block</summary>

```rust
    let url=location.as_ref().map(|location|location.value().to_owned());
    match crate::document::load_at(bytes,location) {
        Ok(loaded)=>candidate.import(loaded).map_err(str::to_owned)?,
        Err(_)=>{let previous=candidate.all_ids();crate::scene_document::import(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.)?;if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}}
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-13.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/publication.rs`

Supply full original Element/interaction preparation and atomic malformed-file checks with retained reload origin.

<details>
<summary>Locate the existing block</summary>

```text
t(),1);assert_eq!(editor.camera.uniform(),camera);assert_eq!(editor.scene.clouds().len(),1);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),1);
    }

}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-14.rs" skip=180
```

</details>

### 8. `src/publication.rs`

Reuse the existing sheet URL initializer for newly imported originals; preserve earlier sheets’ metadata bases.

<details>
<summary>Locate the existing block</summary>

```rust
    crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let previous=candidate.all_ids();let url=location.as_ref().map(|location|location.value().to_owned());
    crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?;
    if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-15.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.6.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for open files through one measured session reader.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



file bytes → decode → full validation → kernel → display walk → atomic history adoption.



Why must timing and file adoption use the same full reader?

Measurements must describe the work that prepares the accepted document. A decode, validation or display error must leave the visible document and its undo/redo history intact.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Open an Element Session with interactions, download Report, then try malformed bytes. Inspect the four successful phases, the failed decode, and unchanged accepted drawing. Undo and Redo must preserve original owners.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.6.2
npm --prefix ../session_tests run course -- save 20.6.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The full Session file boundary now has one source preparation path and measured phases. Verify actual browser import, history, source reload and release before claiming acceptance.

Actual imported original Elements after complete measured file/document phases, atomic failed import, child editing, interaction visibility and two Save/reopen roundtrips. These checks preserve the existing assertions; contact depth and broader rendering parity remain separate.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.6.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
