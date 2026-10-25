# 3.7 · Restore exact sources and replay the captured edit

**Steps:** 26 · **Source:** `src/app/decode.rs` · `src/app/loader.rs` · `src/state.rs`

## <a id="checkpoint-32g-keys"></a>1. Identify the source release a reload belongs to

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-d1b49447" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-d1b49447" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-d1b49447" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-d1b49447" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-d1b49447" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">31..32G · RESIDENCY &amp; RELOAD</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Shared Arc&lt;GpuGeometry&gt; Cache, Source Unloading &amp; On-Demand Async Restoration</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Loaded(Arc&lt;Source&gt;) &lt;-&gt; Released(Origin, Key)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-d1b49447)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Immutable Arc&lt;GpuGeometry&gt; + Settings Slot</text>
  <rect x="44" y="76" width="284" height="48" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Arc&lt;GpuGeometry&gt; (Shared Across Undo)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">Zero vertex re-upload on selection change</text>
  <rect x="44" y="134" width="284" height="48" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="154" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Unload Sources: Drop f64 Kernel RAM</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="172" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">f32 GPU display mesh stays interactive</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-d1b49447)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-d1b49447)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Captured Edit Intent -&gt; Async HTTP Reload</text>
  <rect x="414" y="76" width="292" height="106" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Move / Delete / Save on Unloaded Row:</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="120" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">1. Capture PendingIntent + ReleaseKey</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="142" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">2. Bounded Fetch + AbortController</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="164" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">3. Atomic Adopt + Replay Exact f64 Edit</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-d1b49447)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-d1b49447)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Lossless f64 Precision Restored</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="830" y="84" width="208" height="42" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="109" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Sources Restored ✓</text>
  <rect x="830" y="136" width="208" height="40" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="160" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Move / Save Committed</text>
</svg>
</div>

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod prepared;
pub mod reload_url;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod prepared;
pub mod rehydrate;
pub mod reload_url;
```

### 2. `src/rehydrate.rs`

Create the file and type:

```rust
use std::rc::Rc;

#[derive(Clone)]
pub struct ReloadKey {
    pub origin: Rc<crate::origin::Origin>,
    pub epoch: u64,
}

impl ReloadKey {
    pub fn of(row: &crate::scene::Object) -> Option<Self> {
        Some(Self {
            origin: Rc::clone(row.origin()?),
            epoch: row.release_epoch()?,
        })
    }

    pub fn matches(&self, row: &crate::scene::Object) -> bool {
        row.release_epoch() == Some(self.epoch)
            && row
                .origin()
                .is_some_and(|origin| Rc::ptr_eq(origin, &self.origin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::{Action, Editor};

    #[test]
    fn a_reload_key_matches_only_its_current_import_release() {
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:key".into(),
        ));
        editor
            .apply(Action::ReplaceAt(
                bytes.clone(),
                Rc::clone(&url),
            ))
            .unwrap();
        assert!(ReloadKey::of(&editor.scene.objects()[0]).is_none());
        editor.apply(Action::UnloadSources).unwrap();
        let key = ReloadKey::of(&editor.scene.objects()[0]).unwrap();
        assert!(
            editor
                .scene
                .objects()
                .iter()
                .all(|row| key.matches(row))
        );
        let later = ReloadKey {
            origin: Rc::clone(&key.origin),
            epoch: key.epoch + 1,
        };
        assert!(!later.matches(&editor.scene.objects()[0]));
        editor.apply(Action::Close).unwrap();
        editor
            .apply(Action::ImportAt(bytes, url))
            .unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert!(!key.matches(&editor.scene.objects()[0]));
    }
}
```

### 3. `src/editor.rs`

**Locate this existing block:**

```rust
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
```

## <a id="checkpoint-32ga-prepare"></a>2. Prepare kernel data without rebuilding its display

### 1. `src/document.rs`

**Locate this existing block:**

```rust
) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document =
        Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document),
            guid: mesh.guid().to_owned(),
            origin: Rc::clone(&origin),
        });
        meshes.push(prepared);
    }
    Ok(Loaded { document, meshes })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Result<Loaded, &'static str> {
    let message = decode(bytes)?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document =
        Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document),
            guid: mesh.guid().to_owned(),
            origin: Rc::clone(&origin),
        });
        meshes.push(prepared);
    }
    Ok(Loaded { document, meshes })
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    Ok(message)
}

pub(crate) fn restore(
    bytes: &[u8],
    origin: &crate::origin::Origin,
) -> Result<Rc<Session>, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    if crate::origin::FileVersion::of(bytes) != origin.version {
        return Err("Reloaded file version changed");
    }
    Session::from_proto(decode(bytes)?)
        .map(Rc::new)
        .map_err(|_| "Cannot restore source session")
}
```

### 2. `src/rehydrate.rs`

**Locate this existing block:**

```rust
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub struct PreparedReload {
    pub key: ReloadKey,
    pub document: Rc<session_rust::Session>,
}

impl PreparedReload {
    pub fn new(key: ReloadKey, bytes: &[u8]) -> Result<Self, &'static str> {
        let document = crate::document::restore(bytes, &key.origin)?;
        Ok(Self { key, document })
    }

    pub fn geometry(&self, guid: &str) -> Option<&Rc<session_rust::Mesh>> {
        self.document
            .objects
            .meshes
            .iter()
            .find(|mesh| mesh.guid() == guid)
    }

    pub fn validates(&self, row: &crate::scene::Object) -> bool {
        let Some(mesh) = self.geometry(&row.metadata.source_guid) else {
            return false;
        };
        let metadata = &row.metadata;
        metadata.name == mesh.name
            && metadata.is_visible == mesh.is_visible
            && metadata.is_locked == mesh.is_locked
    }
}

#[cfg(test)]
```

## <a id="checkpoint-32gb-adopt"></a>3. Adopt restored source owners as one residency change

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) {
                continue;
            }
            let Some(geometry) = prepared
                .geometry(&row.metadata.source_guid)
                .cloned()
            else {
                continue;
            };
            let source = crate::document::Source {
                document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(),
                origin: Rc::clone(&prepared.key.origin),
            };
            row.editable = crate::edit_source::EditSource::Loaded {
                geometry,
                source: Some(source),
            };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
```

### 2. `src/editor.rs`

**Locate this existing block:**

```rust
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
```

## <a id="checkpoint-32gc-roundtrip"></a>4. Prove source restoration preserves display and history

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod rehydrate;
pub mod reload_url;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod rehydrate;
#[cfg(test)]
mod rehydrate_tests;
pub mod reload_url;
```

### 2. `src/rehydrate_tests.rs`

Create the file and type:

```rust
use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn hydration_restores_original_doubles_without_rebuilding_rows_or_history() {
    use prost::Message;
    let mut message =
        session_rust::proto::Session::decode(crate::specimen::bytes().as_slice())
            .unwrap();
    let value = 0.123456789012345;
    let source = &mut message.objects.as_mut().unwrap().meshes[0];
    source.vertices.get_mut(&0).unwrap().x = value;
    source.is_locked = Some(true);
    let bytes = message.encode_to_vec();
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:restore".into(),
    ));
    editor
        .apply(Action::ReplaceAt(bytes.clone(), url))
        .unwrap();
    editor.apply(Action::SelectNext).unwrap();
    editor
        .apply(Action::Translate([0.25, 0.0, 0.0]))
        .unwrap();
    let row = &editor.scene.objects()[0];
    let id = row.id;
    let guid = row.guid.clone();
    let model = row.model.m;
    let display = Rc::clone(&row.mesh);
    let metadata = Rc::clone(&row.metadata);
    let old = Rc::downgrade(row.geometry().unwrap());
    let roots = editor.scenes().count();
    let view = editor.camera.uniform();
    editor.apply(Action::UnloadSources).unwrap();
    let key = editor.reload_keys().pop().unwrap();
    assert!(editor.hydrate(vec![(key, bytes)]).unwrap());
    let row = &editor.scene.objects()[0];
    assert_eq!(row.id, id);
    assert_eq!(row.guid, guid);
    assert_eq!(row.model.m, model);
    assert!(Rc::ptr_eq(&row.mesh, &display) && Rc::ptr_eq(&row.metadata, &metadata));
    assert_eq!(
        row.geometry()
            .unwrap()
            .vertex_point(0)
            .unwrap()[0],
        value
    );
    assert!(old.upgrade().is_none());
    assert_eq!(editor.scenes().count(), roots);
    assert_eq!(editor.camera.uniform(), view);
    let saved =
        crate::document::load(&crate::document::snapshot(&editor.scene).unwrap())
            .unwrap();
    assert_eq!(
        saved.meshes[0]
            .geometry
            .vertex_point(0)
            .unwrap()[0],
        value
    );
    editor.apply(Action::Undo).unwrap();
    assert_ne!(editor.scene.objects()[0].model.m, model);
    assert!(
        editor.scene.objects()[0]
            .geometry()
            .is_some()
    );
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, model);
}
```

## <a id="checkpoint-32gd-rejections"></a>5. Reject stale or inconsistent source batches atomically

### 1. `src/rehydrate_tests.rs`

**Locate this existing block:**

```rust
use crate::editor::{Action, Editor};
use std::rc::Rc;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::editor::{Action, Editor};

#[test]
fn a_bad_second_version_rejects_the_entire_source_batch() {
    let mut editor = Editor::default();
    let bytes = crate::specimen::bytes();
    for mode in 0..2 {
        let url = Rc::new(crate::reload_url::ReloadUrl::new(format!(
            "test:batch-{mode}"
        )));
        let action = if mode == 0 {
            Action::ReplaceAt(bytes.clone(), url)
        } else {
            Action::ImportAt(bytes.clone(), url)
        };
        editor.apply(action).unwrap();
    }
    editor.apply(Action::UnloadSources).unwrap();
    let keys = editor.reload_keys();
    assert_eq!(keys.len(), 2);
    let displays: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|row| Rc::clone(&row.mesh))
        .collect();
    let ids: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect();
    let roots = editor.scenes().count();
    let mut changed = bytes.clone();
    changed.push(0);
    let result = editor.hydrate(vec![
        (keys[0].clone(), bytes.clone()),
        (keys[1].clone(), changed),
    ]);
    assert_eq!(result, Err("Reloaded file version changed"));
    assert_eq!(editor.scenes().count(), roots);
    for (index, row) in editor.scene.objects().iter().enumerate() {
        assert!(row.geometry().is_none());
        assert_eq!(row.id, ids[index]);
        assert!(Rc::ptr_eq(&row.mesh, &displays[index]));
    }
    assert!(
        editor
            .hydrate(
                keys.into_iter()
                    .map(|key| (key, bytes.clone()))
                    .collect()
            )
            .unwrap()
    );
    assert!(
        editor
            .scenes()
            .flat_map(|scene| scene.objects())
            .all(|row| row.origin().is_none() || row.geometry().is_some())
    );
}

#[test]
fn missing_source_identity_or_changed_metadata_keeps_rows_cold() {
    for change_name in [false, true] {
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:metadata".into(),
        ));
        editor
            .apply(Action::ReplaceAt(bytes.clone(), url))
            .unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        let key = editor.reload_keys().pop().unwrap();
        let display = Rc::clone(&editor.scene.objects()[0].mesh);
        let metadata = Rc::make_mut(&mut editor.scene.objects_mut()[0].metadata);
        if change_name {
            metadata.name.push_str(" changed");
        } else {
            metadata.source_guid = "missing".into();
        }
        assert_eq!(
            editor.hydrate(vec![(key, bytes)]),
            Err("Reloaded source GUID or metadata does not match")
        );
        assert!(
            editor
                .scene
                .objects()
                .iter()
                .all(|row| row.geometry().is_none())
        );
        assert!(Rc::ptr_eq(
            &editor.scene.objects()[0].mesh,
            &display
        ));
    }
}
use std::rc::Rc;
```

## <a id="checkpoint-32ge-stale"></a>6. Ignore old reload results before decoding them

### 1. `src/rehydrate_tests.rs`

**Locate this existing block:**

```rust
#[test]
fn a_bad_second_version_rejects_the_entire_source_batch() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn obsolete_and_duplicate_keys_are_ignored_before_payload_decoding() {
    let mut editor = Editor::default();
    let bytes = crate::specimen::bytes();
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:stale".into(),
    ));
    editor
        .apply(Action::ReplaceAt(
            bytes.clone(),
            Rc::clone(&url),
        ))
        .unwrap();
    editor.apply(Action::UnloadSources).unwrap();
    let old = editor.reload_keys().pop().unwrap();
    editor.apply(Action::Close).unwrap();
    assert!(
        !editor
            .hydrate(vec![(old.clone(), vec![255])])
            .unwrap()
    );
    editor
        .apply(Action::ImportAt(bytes.clone(), url))
        .unwrap();
    editor.apply(Action::UnloadSources).unwrap();
    let current = editor.reload_keys().pop().unwrap();
    assert!(
        !editor
            .hydrate(vec![(old, vec![255])])
            .unwrap()
    );
    assert!(
        !editor
            .hydrate(vec![
                (current.clone(), bytes.clone()),
                (current.clone(), vec![255])
            ])
            .unwrap()
    );
    assert!(
        editor
            .scene
            .objects()
            .iter()
            .all(|row| row.geometry().is_none())
    );
    assert!(!editor.hydrate(Vec::new()).unwrap());
    assert!(
        editor
            .hydrate(vec![(current.clone(), bytes.clone())])
            .unwrap()
    );
    assert!(
        !editor
            .hydrate(vec![(current.clone(), vec![255])])
            .unwrap()
    );
    editor.apply(Action::UnloadSources).unwrap();
    let next = editor.reload_keys().pop().unwrap();
    assert!(next.epoch > current.epoch);
    assert!(
        !editor
            .hydrate(vec![(current, vec![255])])
            .unwrap()
    );
    assert!(
        editor
            .scene
            .objects()
            .iter()
            .all(|row| row.release_epoch() == Some(next.epoch))
    );
    assert!(editor.hydrate(vec![(next, bytes)]).unwrap());
}

#[test]
fn a_bad_second_version_rejects_the_entire_source_batch() {
```

## <a id="checkpoint-32gf-request"></a>7. Give each source request its own owner

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
mod rehydrate_tests;
pub mod reload_url;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod rehydrate_tests;
pub mod reload_job;
pub mod reload_url;
```

### 2. `src/reload_job.rs`

Create the file and type:

```rust
use crate::rehydrate::ReloadKey;

pub struct Request {
    pub ticket: u64,
    pub urls: Vec<String>,
}

struct Pending {
    ticket: u64,
    keys: Vec<ReloadKey>,
}

#[derive(Default)]
pub struct ReloadJob {
    issued: u64,
    pending: Option<Pending>,
}

impl ReloadJob {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() {
            return Err("No released sources to reload");
        }
        let mut origins = std::collections::HashSet::new();
        let mut urls = Vec::new();
        for key in &keys {
            if !origins.insert(key.origin.id) {
                return Err("Duplicate reload origin");
            }
            let location = key
                .origin
                .location
                .as_ref()
                .ok_or("Source has no reload location")?;
            urls.push(location.value().to_owned());
        }
        let ticket = self
            .issued
            .checked_add(1)
            .ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
            .as_ref()
            .map(|pending| pending.ticket)
    }

    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        if self.pending() != Some(ticket) {
            return None;
        }
        self.pending
            .take()
            .map(|pending| pending.keys)
    }

    pub fn cancel(&mut self) -> bool {
        self.pending.take().is_some()
    }
}
```

## <a id="checkpoint-32gfa-ownership"></a>8. Prove cancelled work releases its source owners

### 1. `src/reload_job.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::{Action, Editor},
        reload_url::ReloadUrl,
    };
    use std::{cell::RefCell, rc::Rc};

    fn cold() -> (Editor, Vec<ReloadKey>) {
        let mut editor = Editor::default();
        let location = Rc::new(ReloadUrl::new("test:request".into()));
        editor
            .apply(Action::ReplaceAt(
                crate::specimen::bytes(),
                location,
            ))
            .unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        let keys = editor.reload_keys();
        (editor, keys)
    }

    #[test]
    fn only_current_work_can_finish_and_it_finishes_once() {
        let (_, keys) = cold();
        let mut job = ReloadJob::default();
        let first = job.begin(keys.clone()).unwrap();
        let second = job.begin(keys.clone()).unwrap();
        assert!(second.ticket > first.ticket);
        assert!(job.finish(first.ticket).is_none());
        assert_eq!(job.pending(), Some(second.ticket));
        let accepted = job.finish(second.ticket).unwrap();
        assert_eq!(accepted.len(), 1);
        assert!(Rc::ptr_eq(
            &accepted[0].origin,
            &keys[0].origin
        ));
        assert!(job.finish(second.ticket).is_none());
        let third = job.begin(keys.clone()).unwrap();
        assert!(job.cancel());
        assert!(!job.cancel());
        assert!(job.finish(third.ticket).is_none());
        assert!(
            job.begin(vec![keys[0].clone(), keys[0].clone()])
                .is_err()
        );
        assert!(job.begin(Vec::new()).is_err());
        assert_eq!(job.pending(), None);
    }

    #[test]
    fn cancellation_drops_keys_even_when_abandoned_work_holds_the_job() {
        let (mut editor, keys) = cold();
        let origin = Rc::downgrade(&keys[0].origin);
        let location = Rc::downgrade(keys[0].origin.location.as_ref().unwrap());
        let job = Rc::new(RefCell::new(ReloadJob::default()));
        let request = job.borrow_mut().begin(keys).unwrap();
        let abandoned = Rc::clone(&job);
        editor.apply(Action::Close).unwrap();
        assert!(origin.upgrade().is_some());
        assert!(job.borrow_mut().cancel());
        assert!(origin.upgrade().is_none() && location.upgrade().is_none());
        assert_eq!(request.urls, vec!["test:request".to_owned()]);
        assert!(
            abandoned
                .borrow_mut()
                .finish(request.ticket)
                .is_none()
        );
    }

    #[test]
    fn exhaustion_cancels_old_keys_and_never_wraps() {
        let (_, keys) = cold();
        let mut job = ReloadJob::default();
        job.begin(keys.clone()).unwrap();
        job.issued = u64::MAX;
        assert!(job.begin(keys).is_err());
        assert_eq!(job.pending(), None);
        assert_eq!(job.issued, u64::MAX);
    }
}
```

## <a id="checkpoint-32gg-fetch"></a>9. Read a source response within its byte limit

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod snapshot_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod snapshot_tests;
#[cfg(target_arch = "wasm32")]
mod source_fetch;
#[cfg(test)]
```

### 3. `src/source_fetch.rs`

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub async fn fetch(
    url: &str,
    signal: &web_sys::AbortSignal,
) -> Result<Vec<u8>, String> {
    let window = web_sys::window().ok_or("No browser window")?;
    let init = web_sys::RequestInit::new();
    init.set_signal(Some(signal));
    let value = JsFuture::from(window.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|error| format!("Cannot fetch source: {error:?}"))?;
    let response: web_sys::Response = value
        .dyn_into()
        .map_err(|_| "Invalid fetch response")?;
    if !response.ok() {
        return Err(format!("Reload HTTP {}", response.status()));
    }
    if let Some(length) = response
        .headers()
        .get("Content-Length")
        .map_err(|_| "Cannot read source length")?
    {
        let length: u64 = length
            .parse()
            .map_err(|_| "Invalid source length")?;
        if length > crate::document::MAX_BYTES as u64 {
            return Err("Reload source exceeds 4 MiB".into());
        }
    }
    let body = response
        .body()
        .ok_or("Source response has no body")?;
    let reader: web_sys::ReadableStreamDefaultReader = body
        .get_reader()
        .dyn_into()
        .map_err(|_| "Cannot read source stream")?;
    let result = read(&reader).await;
    if result.is_err() {
        let _ = JsFuture::from(reader.cancel()).await;
    }
    reader.release_lock();
    result
}

async fn read(
    reader: &web_sys::ReadableStreamDefaultReader,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    loop {
        let part = JsFuture::from(reader.read())
            .await
            .map_err(|error| format!("Cannot read source: {error:?}"))?;
        let done = js_sys::Reflect::get(&part, &JsValue::from_str("done"))
            .map_err(|_| "Invalid stream reply")?;
        if done.as_bool() == Some(true) {
            break;
        }
        let value = js_sys::Reflect::get(&part, &JsValue::from_str("value"))
            .map_err(|_| "Missing source chunk")?;
        let chunk: js_sys::Uint8Array = value
            .dyn_into()
            .map_err(|_| "Invalid source chunk")?;
        if chunk.length() as usize > crate::document::MAX_BYTES - bytes.len() {
            return Err("Reload source exceeds 4 MiB".into());
        }
        bytes.extend_from_slice(&chunk.to_vec());
    }
    Ok(bytes)
}
```

## <a id="checkpoint-32gga-flight"></a>10. Pair reload ownership with a browser abort controller

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
mod browser;
pub mod document;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod browser;
#[cfg(target_arch = "wasm32")]
mod browser_reload;
pub mod document;
```

### 2. `src/browser_reload.rs`

Create the file and type:

```rust
use crate::{
    rehydrate::ReloadKey,
    reload_job::{ReloadJob, Request},
};
use std::{cell::RefCell, rc::Rc};

pub type Shared = Rc<RefCell<Flight>>;

#[derive(Default)]
pub struct Flight {
    job: ReloadJob,
    controller: Option<web_sys::AbortController>,
}

impl Flight {
    pub fn cancel(&mut self) -> bool {
        let cancelled = self.job.cancel();
        if let Some(controller) = self.controller.take() {
            controller.abort();
        }
        cancelled
    }

    fn begin(
        &mut self,
        keys: Vec<ReloadKey>,
    ) -> Result<(Request, web_sys::AbortSignal), String> {
        self.cancel();
        let controller = web_sys::AbortController::new()
            .map_err(|error| format!("Cannot start reload: {error:?}"))?;
        let request = self.job.begin(keys)?;
        let signal = controller.signal();
        self.controller = Some(controller);
        Ok((request, signal))
    }

    fn finish(&mut self, ticket: u64, failed: bool) -> Option<Vec<ReloadKey>> {
        let keys = self.job.finish(ticket)?;
        if let Some(controller) = self.controller.take() {
            if failed {
                controller.abort();
            }
        }
        Some(keys)
    }
}
```

## <a id="checkpoint-32ggb-delivery"></a>11. Deliver only the current completed reload batch

### 1. `src/browser_reload.rs`

**Locate this existing block:**

```rust
pub type Shared = Rc<RefCell<Flight>>;

#[derive(Default)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub type Shared = Rc<RefCell<Flight>>;

pub type Reply = Result<Vec<(ReloadKey, Vec<u8>)>, String>;
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(
    shared: Shared,
    keys: Vec<ReloadKey>,
    delivery: Delivery,
) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys)?;
    crate::browser::report("Reloading editable sources…");
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                values.push(crate::source_fetch::fetch(&url, &signal).await?);
            }
            Ok(values)
        }
        .await;
        let Some(keys) = shared
            .borrow_mut()
            .finish(request.ticket, result.is_err())
        else {
            return;
        };
        let result = result.map(|values| keys.into_iter().zip(values).collect());
        if let Err(error) = deliver(result, &delivery) {
            crate::browser::report(&format!("Cannot deliver reload: {error:?}"));
        }
    });
    Ok(())
}

fn deliver(result: Reply, delivery: &Delivery) -> Result<(), wasm_bindgen::JsValue> {
    let event = web_sys::Event::new("viewer-reload")?;
    *delivery.borrow_mut() = Some(result);
    let result = web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

#[derive(Default)]
```

## <a id="checkpoint-32gh-command"></a>12. Restore editable sources through the command line

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    window.add_event_listener_with_callback(
        "viewer-file-error",
        update.as_ref().unchecked_ref(),
```

Replace it with:

```rust
    window.add_event_listener_with_callback(
        "viewer-file-error",
        update.as_ref().unchecked_ref(),
    )?;
    window.add_event_listener_with_callback(
        "viewer-reload",
        update.as_ref().unchecked_ref(),
```

**Locate this block:**

```rust
                    }
                }
            } else if let Some(line) = line {
                if line == "unload sources" {
                    Some(Action::UnloadSources)
                } else if line == "close" {
                    request.borrow_mut().cancel();
```

Replace it with:

```rust
                    }
                }
            } else if let Some(line) = line {
                if line == "reload sources" {
                    let result = crate::browser_reload::start(
                        std::rc::Rc::clone(&reload),
                        editor.reload_keys(),
                        std::rc::Rc::clone(&reload_delivery),
                    );
                    match result {
                        Ok(()) => panel.result("Reloading editable sources…"),
                        Err(error) => {
                            report(&error);
                            panel.result(&error);
                        }
                    }
                    None
                } else if line == "cancel reload" {
                    reload.borrow_mut().cancel();
                    report("Reload cancelled.");
                    panel.result("Reload cancelled.");
                    None
                } else if line == "unload sources" {
                    Some(Action::UnloadSources)
                } else if line == "close" {
                    request.borrow_mut().cancel();
```

**Locate this block:**

```rust
                    panel.answer("Open", &message);
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
```

Replace it with:

```rust
                    panel.answer("Open", &message);
                }
                None
            } else if event.type_() == "viewer-reload" {
                if let Some(result) = reload_delivery.borrow_mut().take() {
                    match result.and_then(|values| {
                        editor.hydrate(values).map_err(str::to_owned)
                    }) {
                        Ok(true) => {
                            renderer.set_scene(&editor.scene, editor.selected);
                            let message =
                                "Editable sources restored; display retained.";
                            report(message);
                            panel.answer("Reload Sources", message);
                        }
                        Ok(false) => {}
                        Err(error) => {
                            report(&error);
                            panel.answer("Reload Sources", &error);
                        }
                    }
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
```

**Locate this block:**

```rust
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
```

Replace it with:

```rust
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared =
        std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
```

**Locate this block:**

```rust
            "Unload Sources",
            "Save",
            "Example Box",
            "Example Triangle",
```

Replace it with:

```rust
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
```

## <a id="checkpoint-32gha-cancel"></a>13. Cancel reloads when their document context changes

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                None
            };
            if let Some(action) = action {
                let replacement = matches!(
```

Replace it with:

```rust
                None
            };
            if let Some(action) = action {
                if matches!(
                    &action,
                    Action::Close
                        | Action::UnloadSources
                        | Action::Undo
                        | Action::Redo
                        | Action::Replace(_)
                        | Action::ReplaceAt(_, _)
                ) {
                    reload.borrow_mut().cancel();
                }
                let replacement = matches!(
```

**Locate this block:**

```rust
                    }
                    read_mode.set(if line == "open replace" {
                        crate::file_input::Mode::Replace
                    } else {
```

Replace it with:

```rust
                    }
                    if line == "open replace" {
                        reload.borrow_mut().cancel();
                    }
                    read_mode.set(if line == "open replace" {
                        crate::file_input::Mode::Replace
                    } else {
```

### 2. `examples/reload_check.rs`

Create the file and type:

```rust
use prost::Message;
use session_rust::proto::Session;

fn read(path: &str) -> Session {
    let bytes = std::fs::read(path).expect("Read the browser file");
    Session::decode(bytes.as_slice()).expect("Valid session protobuf")
}

fn main() {
    let paths: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(
        paths.len(),
        2,
        "Give original and browser-saved files"
    );
    let original = read(&paths[0]);
    let saved = read(&paths[1]);
    let original = original.objects.unwrap().meshes;
    let restored = saved.objects.unwrap().meshes;
    assert_eq!(original.len(), restored.len());
    for source in original {
        let row = restored
            .iter()
            .find(|row| row.guid == source.guid)
            .expect("Saved original GUID");
        assert_eq!(
            row.vertices, source.vertices,
            "Exact original double coordinates"
        );
        assert_eq!(row.faces, source.faces, "Original topology");
        assert_eq!(row.name, source.name);
        assert_eq!(
            row.is_visible.unwrap_or(true),
            source.is_visible.unwrap_or(true)
        );
        assert_eq!(
            row.is_locked.unwrap_or(false),
            source.is_locked.unwrap_or(false)
        );
    }
    assert!(
        !saved.xforms.is_empty(),
        "Current placements are saved separately"
    );
    println!(
        "Browser reload/save preserves original geometry, flags, names and saved GUIDs."
    );
}
```

## <a id="checkpoint-32gi-capture"></a>14. Capture the requested edit before waiting

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-eb361fc9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-eb361fc9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-eb361fc9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-eb361fc9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-eb361fc9" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">32A..32F · SOURCE UNLOAD</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Releasing Heavy f64 Kernel Sources While Keeping f32 GPU Display Geometry Interactive</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Loaded(Arc&lt;Source&gt;) -&gt; Released { origin, key, version }</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-eb361fc9)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Before Unload: CPU f64 + GPU f32</text>
  <rect x="44" y="76" width="284" height="48" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Heavy f64 Kernel Source in WASM RAM</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Released by "Unload Sources" command</text>
  <rect x="44" y="132" width="284" height="48" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="152" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Compact Provenance Header Retained</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="170" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">{ OriginUrl, ReleaseKey, SourceVersion }</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-eb361fc9)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-eb361fc9)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Allowed vs Guarded Operations While Unloaded</text>
  <rect x="414" y="76" width="292" height="104" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">✓ Orbit / Pan / Zoom / Pick / Color / Hide</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="118" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">(Operates purely on retained GPU buffers &amp; settings)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">⚠ Move / Save / Exact Export</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="164" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">(Requires reloading exact f64 source first)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-eb361fc9)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-eb361fc9)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Low-RAM Interactive Viewport</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,96 944,74 1004,96 944,118" fill="#dbeafe" stroke="#111111" stroke-width="1.5"/>
  <polygon points="884,96 944,118 944,174 884,152" fill="#60a5fa" stroke="#111111" stroke-width="1.5"/>
  <polygon points="944,118 1004,96 1004,152 944,174" fill="#2563eb" stroke="#111111" stroke-width="1.5"/>
</svg>
</div>

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod camera;
mod edit_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
pub mod edit_intent;
mod edit_source;
```

### 2. `src/edit_intent.rs`

Create the file and type:

```rust
use crate::{editor::Action, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Move {
        id: ObjectId,
        offset: [f64; 3],
    },
    Delete {
        id: ObjectId,
    },
    Save,
}

impl Intent {
    pub fn capture(
        action: &Action,
        selected: Option<ObjectId>,
    ) -> Result<Option<Self>, &'static str> {
        Ok(match action {
            Action::Translate(offset) => {
                let id = selected.ok_or("Select an object before Move")?;
                if !offset.iter().all(|value| value.is_finite()) {
                    return Err("Move needs finite coordinates");
                }
                if offset.iter().all(|&value| value == 0.0) {
                    None
                } else {
                    Some(Self::Move {
                        id,
                        offset: *offset,
                    })
                }
            }
            Action::Delete => selected.map(|id| Self::Delete { id }),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Editor, reload_url::ReloadUrl};
    use std::rc::Rc;

    fn cold() -> Editor {
        let mut editor = Editor::default();
        editor
            .apply(Action::ReplaceAt(
                crate::specimen::bytes(),
                Rc::new(ReloadUrl::new("test:intent".into())),
            ))
            .unwrap();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        editor
    }

    #[test]
    fn move_keeps_original_target_and_arguments_after_selection_changes() {
        let mut editor = cold();
        let id = editor.selected.unwrap();
        let origin = Rc::downgrade(editor.scene.objects()[0].origin().unwrap());
        let intent = Intent::capture(
            &Action::Translate([0.125, -0.25, 0.5]),
            editor.selected,
        )
        .unwrap()
        .unwrap();
        editor.apply(Action::SelectNext).unwrap();
        editor
            .apply(Action::Orbit(0.1, 0.2))
            .unwrap();
        assert_ne!(editor.selected, Some(id));
        assert_eq!(
            intent,
            Intent::Move {
                id,
                offset: [0.125, -0.25, 0.5]
            }
        );
        editor.apply(Action::Close).unwrap();
        assert!(
            origin.upgrade().is_none(),
            "intent owns no source or URL"
        );
    }

    #[test]
    fn delete_keeps_its_original_target() {
        let mut editor = cold();
        let id = editor.selected.unwrap();
        let intent = Intent::capture(&Action::Delete, editor.selected)
            .unwrap()
            .unwrap();
        editor.apply(Action::SelectNext).unwrap();
        assert_eq!(intent, Intent::Delete { id });
        assert_ne!(editor.selected, Some(id));
    }

    #[test]
    fn empty_edits_do_not_request_io_and_invalid_moves_are_rejected() {
        assert_eq!(
            Intent::capture(&Action::Delete, None).unwrap(),
            None
        );
        assert!(Intent::capture(&Action::Translate([0.0; 3]), None).is_err());
        let editor = Editor::default();
        let id = editor.scene.objects()[0].id;
        assert_eq!(
            Intent::capture(&Action::Translate([0.0; 3]), Some(id)).unwrap(),
            None
        );
        assert!(
            Intent::capture(
                &Action::Translate([f64::NAN, 0.0, 0.0]),
                Some(id)
            )
            .is_err()
        );
        assert_eq!(
            Intent::capture(&Action::Fit, Some(id)).unwrap(),
            None
        );
    }
}
```

## <a id="checkpoint-32gia-scope"></a>15. Load only the sources the requested edit needs

### 1. `src/edit_intent.rs`

**Locate this existing block:**

```rust
use crate::{editor::Action, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Move {
        id: ObjectId,
        offset: [f64; 3],
    },
    Delete {
        id: ObjectId,
    },
    Save,
}

impl Intent {
    pub fn capture(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{
    editor::{Action, Editor},
    rehydrate::ReloadKey,
    scene::ObjectId,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Move {
        id: ObjectId,
        offset: [f64; 3],
    },
    Delete {
        id: ObjectId,
    },
    Save,
}

impl Intent {
    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
        match self {
            Self::Save => Ok(editor.reload_keys()),
            Self::Move { id, .. } | Self::Delete { id } => {
                let row = editor
                    .scene
                    .objects()
                    .iter()
                    .find(|row| row.id == *id)
                    .ok_or("Object not found")?;
                Ok(ReloadKey::of(row).into_iter().collect())
            }
        }
    }

    pub fn capture(
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod edit_intent;
mod edit_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod edit_intent;
#[cfg(test)]
mod edit_scope_tests;
mod edit_source;
```

### 3. `src/edit_scope_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Editor},
    reload_url::ReloadUrl,
};
use std::rc::Rc;

fn file(editor: &mut Editor, replace: bool, url: &str) {
    let bytes = crate::specimen::bytes();
    let location = Rc::new(ReloadUrl::new(url.into()));
    editor
        .apply(if replace {
            Action::ReplaceAt(bytes, location)
        } else {
            Action::ImportAt(bytes, location)
        })
        .unwrap();
}

#[test]
fn target_and_save_scope_differ() {
    let mut editor = Editor::default();
    file(&mut editor, true, "test:first");
    file(&mut editor, false, "test:second");
    editor.apply(Action::UnloadSources).unwrap();
    let row = &editor.scene.objects()[0];
    let id = row.id;
    let origin = row.origin().unwrap().id;
    editor.selected = Some(editor.scene.objects()[3].id);
    for intent in [
        Intent::Move {
            id,
            offset: [0.25, 0.0, 0.0],
        },
        Intent::Delete { id },
    ] {
        let keys = intent.keys(&editor).unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].origin.id, origin);
        assert_eq!(keys[0].epoch, 1);
    }
    let keys = Intent::Save.keys(&editor).unwrap();
    assert_eq!(editor.scene.objects().len(), 6);
    assert_eq!(keys.len(), 2);
    assert_ne!(keys[0].origin.id, keys[1].origin.id);
}

#[test]
fn save_ignores_history_only_sources() {
    let mut editor = Editor::default();
    file(&mut editor, true, "test:old");
    let old = editor.scene.objects()[0]
        .origin()
        .unwrap()
        .id;
    file(&mut editor, true, "test:current");
    let current = editor.scene.objects()[0]
        .origin()
        .unwrap()
        .id;
    editor.apply(Action::UnloadSources).unwrap();
    let keys = Intent::Save.keys(&editor).unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].origin.id, current);
    assert_ne!(keys[0].origin.id, old);
    assert!(
        editor
            .scenes()
            .skip(1)
            .flat_map(|scene| scene.objects())
            .any(|row| row
                .origin()
                .is_some_and(|origin| origin.id == old)
                && row.release_epoch().is_some())
    );
}

#[test]
fn loaded_or_missing_target() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    assert!(
        Intent::Delete { id }
            .keys(&editor)
            .unwrap()
            .is_empty()
    );
    assert!(
        Intent::Save
            .keys(&editor)
            .unwrap()
            .is_empty()
    );
    editor.scene.remove(id);
    assert_eq!(
        Intent::Delete { id }.keys(&editor).err(),
        Some("Object not found")
    );
    file(&mut editor, true, "test:loaded");
    let id = editor.scene.objects()[0].id;
    assert!(
        Intent::Move {
            id,
            offset: [1.0, 0.0, 0.0]
        }
        .keys(&editor)
        .unwrap()
        .is_empty()
    );
}
```

## <a id="checkpoint-32gib-move"></a>16. Move the target from its current placement

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                let id = self
                    .selected
                    .ok_or("Select an object before Move")?;
                if offset.iter().all(|&v| v == 0.0) {
                    return Ok(Change::View);
                }
                let object = self
                    .scene
                    .objects()
                    .iter()
                    .find(|o| o.id == id)
                    .ok_or("Object not found")?;
                let shift =
                    session_rust::Xform::translation(offset[0], offset[1], offset[2]);
                let model = &shift * &object.model;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.place(id, model)
                    })?;
            }
```

Replace it with:

```rust
                let id = self
                    .selected
                    .ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
```

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

Replace it with:

```rust
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod edit_scope_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod edit_move_tests;
#[cfg(test)]
mod edit_scope_tests;
```

### 3. `src/edit_move_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Change, Editor},
};
use session_rust::Xform;

#[test]
fn captured_move_uses_current_placement_and_preserves_later_selection() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    let intent = Intent::capture(
        &Action::Translate([0.125, -0.25, 0.5]),
        Some(id),
    )
    .unwrap()
    .unwrap();
    let before = Xform::translation(2.0, 3.0, 4.0);
    editor
        .scene
        .place(id, before.clone())
        .unwrap();
    let later = editor.scene.objects()[1].id;
    editor.selected = Some(later);
    editor
        .apply(Action::Orbit(0.2, 0.1))
        .unwrap();
    let camera = editor.camera.uniform();
    let Intent::Move { id, offset } = intent else {
        panic!("Move intent")
    };
    assert_eq!(
        editor.move_object(id, offset).unwrap(),
        Change::Scene
    );
    let expected = Xform::translation(2.125, 2.75, 4.5);
    assert_eq!(editor.scene.objects()[0].model.m, expected.m);
    assert_eq!(editor.selected, Some(later));
    assert_eq!(editor.camera.uniform(), camera);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, before.m);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, before.m);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, expected.m);
    assert_eq!(editor.selected, Some(later));
}

#[test]
fn no_op_and_refused_moves_keep_redo() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.move_object(id, [0.0; 3]).unwrap(),
        Change::View
    );
    assert!(
        editor
            .move_object(id, [f64::NAN, 0.0, 0.0])
            .is_err()
    );
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
    editor.scene.remove(id);
    assert_eq!(
        editor.move_object(id, [1.0, 0.0, 0.0]),
        Err("Object not found")
    );
}
```

## <a id="checkpoint-32giba-delete"></a>17. Delete the target while keeping later selection

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                if let Some(id) = self.selected {
                    let row = self
                        .scene
                        .objects()
                        .iter()
                        .find(|row| row.id == id)
                        .ok_or("Object not found")?;
                    if row.geometry().is_none() {
                        return Err("Reload editable sources before Delete");
                    }
                }
                if let Some(id) = self.selected.take() {
                    self.history.edit(&mut self.scene, |scene| {
                        scene.remove(id);
                    });
                }
            }
```

Replace it with:

```rust
                .edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                return match self.selected {
                    Some(id) => self.delete_object(id),
                    None => Ok(Change::View),
                };
            }
```

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

Replace it with:

```rust
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        let row = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if row.geometry().is_none() {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.selected == Some(id) {
            self.selected = None;
        }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod camera;
pub mod edit_intent;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
#[cfg(test)]
mod edit_delete_tests;
pub mod edit_intent;
```

### 3. `src/edit_delete_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Change, Editor},
    reload_url::ReloadUrl,
};
use std::rc::Rc;

#[test]
fn captured_delete_keeps_later_selection_and_one_undo() {
    let mut e = Editor::default();
    let id = e.scene.objects()[0].id;
    let intent = Intent::capture(&Action::Delete, Some(id))
        .unwrap()
        .unwrap();
    let later = e.scene.objects()[1].id;
    e.selected = Some(later);
    e.apply(Action::Orbit(0.2, 0.1)).unwrap();
    let camera = e.camera.uniform();
    let Intent::Delete { id } = intent else {
        panic!("Delete intent")
    };
    assert_eq!(e.delete_object(id).unwrap(), Change::Scene);
    assert!(!e.scene.contains(id));
    assert_eq!(e.selected, Some(later));
    assert_eq!(e.camera.uniform(), camera);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.objects().len(), 2);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.objects().len(), 2);
    e.apply(Action::Redo).unwrap();
    assert!(!e.scene.contains(id));
    assert_eq!(e.selected, Some(later));
}

#[test]
fn refused_delete_keeps_redo_and_selection() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(
        crate::specimen::bytes(),
        Rc::new(ReloadUrl::new("test:delete".into())),
    ))
    .unwrap();
    let cold = e.scene.objects()[0].id;
    e.selected = Some(cold);
    e.apply(Action::UnloadSources).unwrap();
    e.apply(Action::AddBox).unwrap();
    let added = e.scene.objects().last().unwrap().id;
    e.apply(Action::Undo).unwrap();
    assert_eq!(
        e.delete_object(cold),
        Err("Reload editable sources before Delete")
    );
    assert_eq!(e.selected, Some(cold));
    assert_eq!(
        e.delete_object(added),
        Err("Object not found")
    );
    e.selected = None;
    assert_eq!(
        e.apply(Action::Delete).unwrap(),
        Change::View
    );
    e.apply(Action::Redo).unwrap();
    assert!(e.scene.contains(added));
    e.selected = Some(added);
    e.delete_object(added).unwrap();
    assert_eq!(e.selected, None);
    e.apply(Action::Undo).unwrap();
    assert!(e.scene.contains(added));
}
```

## <a id="checkpoint-32gibb-reply"></a>18. Route captured Move, Delete and Save results

### 1. `src/edit_replay.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Change, Editor},
};

#[derive(Debug, PartialEq)]
pub enum Reply {
    Changed(Change),
    Saved(Vec<u8>),
}

impl Intent {
    pub fn replay(self, editor: &mut Editor) -> Result<Reply, &'static str> {
        match self {
            Self::Move { id, offset } => editor
                .move_object(id, offset)
                .map(Reply::Changed),
            Self::Delete { id } => editor.delete_object(id).map(Reply::Changed),
            Self::Save => crate::document::snapshot(&editor.scene).map(Reply::Saved),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Action, reload_url::ReloadUrl};
    use prost::Message;
    use std::rc::Rc;

    #[test]
    fn save_reply_keeps_precision_and_does_not_consume_move_undo() {
        let mut e = Editor::default();
        e.apply(Action::Replace(crate::specimen::bytes()))
            .unwrap();
        let id = e.scene.objects()[0].id;
        let original = e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto();
        let later = e.scene.objects()[1].id;
        e.selected = Some(later);
        let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        assert_eq!(
            Intent::Move {
                id,
                offset: [0.25, 0.0, 0.0]
            }
            .replay(&mut e)
            .unwrap(),
            Reply::Changed(Change::Scene)
        );
        let Reply::Saved(bytes) = Intent::Save.replay(&mut e).unwrap() else {
            panic!("Save reply")
        };
        let saved = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
        assert_eq!(
            saved.objects.unwrap().meshes[0].vertices,
            original.vertices
        );
        e.apply(Action::Undo).unwrap();
        assert_eq!(e.scene.objects()[0].model.m, model);
        assert_eq!(e.selected, Some(later));
        assert_eq!(e.camera.uniform(), camera);
        assert_eq!(
            Intent::Delete { id }.replay(&mut e).unwrap(),
            Reply::Changed(Change::Scene)
        );
        assert!(!e.scene.contains(id));
        assert_eq!(e.selected, Some(later));
        e.apply(Action::Undo).unwrap();
        assert!(e.scene.contains(id));
    }

    #[test]
    fn cold_save_reply_fails_without_losing_redo() {
        let mut e = Editor::default();
        e.apply(Action::ReplaceAt(
            crate::specimen::bytes(),
            Rc::new(ReloadUrl::new("test:reply".into())),
        ))
        .unwrap();
        e.apply(Action::UnloadSources).unwrap();
        e.apply(Action::AddBox).unwrap();
        let id = e.scene.objects().last().unwrap().id;
        e.apply(Action::Undo).unwrap();
        assert!(Intent::Save.replay(&mut e).is_err());
        e.apply(Action::Redo).unwrap();
        assert!(e.scene.contains(id));
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod edit_move_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod edit_move_tests;
pub mod edit_replay;
#[cfg(test)]
```

## <a id="checkpoint-32gic-owner"></a>19. Own the intent with its pending source ticket

### 1. `src/reload_job.rs`

**Locate this existing block:**

```rust
}

struct Pending {
    ticket: u64,
    keys: Vec<ReloadKey>,
}

#[derive(Default)]
pub struct ReloadJob {
    issued: u64,
    pending: Option<Pending>,
}

impl ReloadJob {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() {
            return Err("No released sources to reload");
        }
        let mut origins = std::collections::HashSet::new();
        let mut urls = Vec::new();
        for key in &keys {
            if !origins.insert(key.origin.id) {
                return Err("Duplicate reload origin");
            }
            let location = key
                .origin
                .location
                .as_ref()
                .ok_or("Source has no reload location")?;
            urls.push(location.value().to_owned());
        }
        let ticket = self
            .issued
            .checked_add(1)
            .ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
            .as_ref()
            .map(|pending| pending.ticket)
    }

    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        if self.pending() != Some(ticket) {
            return None;
        }
        self.pending
            .take()
            .map(|pending| pending.keys)
    }

    pub fn cancel(&mut self) -> bool {
        self.pending.take().is_some()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

struct Pending<P> {
    ticket: u64,
    keys: Vec<ReloadKey>,
    payload: P,
}

pub struct ReloadJob<P = ()> {
    issued: u64,
    pending: Option<Pending<P>>,
}

impl<P> Default for ReloadJob<P> {
    fn default() -> Self {
        Self {
            issued: 0,
            pending: None,
        }
    }
}

impl<P> ReloadJob<P> {
    pub fn begin_with(
        &mut self,
        keys: Vec<ReloadKey>,
        payload: P,
    ) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() {
            return Err("No released sources to reload");
        }
        let mut origins = std::collections::HashSet::new();
        let mut urls = Vec::new();
        for key in &keys {
            if !origins.insert(key.origin.id) {
                return Err("Duplicate reload origin");
            }
            let location = key
                .origin
                .location
                .as_ref()
                .ok_or("Source has no reload location")?;
            urls.push(location.value().to_owned());
        }
        let ticket = self
            .issued
            .checked_add(1)
            .ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending {
            ticket,
            keys,
            payload,
        });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
            .as_ref()
            .map(|pending| pending.ticket)
    }

    pub fn finish_with(&mut self, ticket: u64) -> Option<(Vec<ReloadKey>, P)> {
        if self.pending() != Some(ticket) {
            return None;
        }
        self.pending
            .take()
            .map(|pending| (pending.keys, pending.payload))
    }

    pub fn cancel(&mut self) -> bool {
        self.pending.take().is_some()
    }
}

impl ReloadJob<()> {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.begin_with(keys, ())
    }
    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        self.finish_with(ticket)
            .map(|(keys, ())| keys)
    }
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod edit_move_tests;
pub mod edit_replay;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod edit_move_tests;
#[cfg(test)]
mod edit_owner_tests;
pub mod edit_replay;
```

### 3. `src/edit_owner_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Editor},
    reload_job::ReloadJob,
    reload_url::ReloadUrl,
};
use std::rc::Rc;

#[test]
fn only_current_ticket_consumes_its_captured_intent_once() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(
        crate::specimen::bytes(),
        Rc::new(ReloadUrl::new("test:owner".into())),
    ))
    .unwrap();
    let id = e.scene.objects()[0].id;
    e.apply(Action::UnloadSources).unwrap();
    let mut job = ReloadJob::<Intent>::default();
    let first = job
        .begin_with(
            e.reload_keys(),
            Intent::Move {
                id,
                offset: [1.0, 0.0, 0.0],
            },
        )
        .unwrap();
    let second = job
        .begin_with(e.reload_keys(), Intent::Save)
        .unwrap();
    assert!(job.finish_with(first.ticket).is_none());
    assert_eq!(job.pending(), Some(second.ticket));
    let (keys, intent) = job.finish_with(second.ticket).unwrap();
    assert_eq!(intent, Intent::Save);
    assert_eq!(keys.len(), 1);
    assert!(job.finish_with(second.ticket).is_none());
    let third = job
        .begin_with(e.reload_keys(), Intent::Delete { id })
        .unwrap();
    assert!(job.cancel());
    assert!(job.finish_with(third.ticket).is_none());
}

#[test]
fn cancelled_payload_keeps_no_document_owner_alive() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(
        crate::specimen::bytes(),
        Rc::new(ReloadUrl::new("test:drop".into())),
    ))
    .unwrap();
    let source = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    e.apply(Action::UnloadSources).unwrap();
    assert!(source.upgrade().is_none());
    let keys = e.reload_keys();
    let origin = Rc::downgrade(&keys[0].origin);
    let mut job = ReloadJob::<Intent>::default();
    job.begin_with(keys, Intent::Save).unwrap();
    e.apply(Action::Close).unwrap();
    assert!(origin.upgrade().is_some());
    assert!(job.cancel());
    assert!(origin.upgrade().is_none());
}
```

## <a id="checkpoint-32gica-reply"></a>20. Pair captured intent with complete source bodies

### 1. `src/reload_reply.rs`

Create the file and type:

```rust
use crate::{edit_intent::Intent, rehydrate::ReloadKey};

pub struct Reply {
    pub intent: Option<Intent>,
    pub result: Result<Vec<(ReloadKey, Vec<u8>)>, String>,
}

impl Reply {
    pub fn new(
        keys: Vec<ReloadKey>,
        intent: Option<Intent>,
        values: Result<Vec<Vec<u8>>, String>,
    ) -> Self {
        let result = values.and_then(|values| {
            if values.len() != keys.len() {
                return Err("Reload body count does not match its keys".into());
            }
            Ok(keys.into_iter().zip(values).collect())
        });
        Self { intent, result }
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod reload_job;
pub mod reload_url;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod reload_job;
pub mod reload_reply;
#[cfg(test)]
mod reload_reply_tests;
pub mod reload_url;
```

### 3. `src/reload_reply_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Editor},
    reload_job::ReloadJob,
    reload_reply::Reply,
    reload_url::ReloadUrl,
};
use std::rc::Rc;

fn cold() -> (Editor, Vec<u8>) {
    let mut e = Editor::default();
    let bytes = crate::specimen::bytes();
    e.apply(Action::ReplaceAt(
        bytes.clone(),
        Rc::new(ReloadUrl::new("test:delivery".into())),
    ))
    .unwrap();
    e.apply(Action::UnloadSources).unwrap();
    (e, bytes)
}

#[test]
fn matched_reply_keeps_captured_operation_and_original_bytes() {
    for operation in 0..3 {
        let (mut e, bytes) = cold();
        let id = e.scene.objects()[0].id;
        let intent = match operation {
            0 => Intent::Move {
                id,
                offset: [0.25, 0.0, 0.0],
            },
            1 => Intent::Delete { id },
            _ => Intent::Save,
        };
        let mut job = ReloadJob::default();
        let request = job
            .begin_with(e.reload_keys(), Some(intent.clone()))
            .unwrap();
        let (keys, captured) = job.finish_with(request.ticket).unwrap();
        let reply = Reply::new(keys, captured, Ok(vec![bytes.clone()]));
        assert_eq!(reply.intent, Some(intent));
        let rows = reply.result.unwrap();
        assert_eq!(rows[0].1, bytes);
        let selected = e.selected;
        let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        assert!(e.hydrate(rows).unwrap());
        assert_eq!(e.selected, selected);
        assert_eq!(e.camera.uniform(), camera);
        assert_eq!(
            e.scene.objects()[0].model.m,
            model,
            "delivering bodies must not replay the operation yet"
        );
        assert!(job.finish_with(request.ticket).is_none());
    }
}

#[test]
fn failures_preserve_operation_and_refuse_partial_body_pairs() {
    let (e, bytes) = cold();
    let id = e.scene.objects()[0].id;
    let intent = Some(Intent::Delete { id });
    let failed = Reply::new(
        e.reload_keys(),
        intent.clone(),
        Err("Network failed".into()),
    );
    assert_eq!(failed.intent, intent);
    assert_eq!(
        failed.result.err().unwrap(),
        "Network failed"
    );
    for bodies in [vec![], vec![vec![1], vec![2]]] {
        let reply = Reply::new(
            e.reload_keys(),
            Some(Intent::Save),
            Ok(bodies),
        );
        assert_eq!(reply.intent, Some(Intent::Save));
        assert_eq!(
            reply.result.err().unwrap(),
            "Reload body count does not match its keys"
        );
    }
    let reply = Reply::new(e.reload_keys(), None, Ok(vec![bytes]));
    assert!(reply.intent.is_none());
    assert_eq!(reply.result.unwrap().len(), 1);
}
```

## <a id="checkpoint-32gicb-bridge"></a>21. Carry captured intent through browser completion

### 1. `src/browser_reload.rs`

**Locate this existing block:**

```rust
pub type Shared = Rc<RefCell<Flight>>;

pub type Reply = Result<Vec<(ReloadKey, Vec<u8>)>, String>;
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(
    shared: Shared,
    keys: Vec<ReloadKey>,
    delivery: Delivery,
) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys)?;
    crate::browser::report("Reloading editable sources…");
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                values.push(crate::source_fetch::fetch(&url, &signal).await?);
            }
            Ok(values)
        }
        .await;
        let Some(keys) = shared
            .borrow_mut()
            .finish(request.ticket, result.is_err())
        else {
            return;
        };
        let result = result.map(|values| keys.into_iter().zip(values).collect());
        if let Err(error) = deliver(result, &delivery) {
            crate::browser::report(&format!("Cannot deliver reload: {error:?}"));
        }
    });
    Ok(())
}

fn deliver(result: Reply, delivery: &Delivery) -> Result<(), wasm_bindgen::JsValue> {
    let event = web_sys::Event::new("viewer-reload")?;
    *delivery.borrow_mut() = Some(result);
    let result = web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

#[derive(Default)]
pub struct Flight {
    job: ReloadJob,
    controller: Option<web_sys::AbortController>,
}

impl Flight {
    pub fn cancel(&mut self) -> bool {
        let cancelled = self.job.cancel();
        if let Some(controller) = self.controller.take() {
            controller.abort();
        }
        cancelled
    }

    fn begin(
        &mut self,
        keys: Vec<ReloadKey>,
    ) -> Result<(Request, web_sys::AbortSignal), String> {
        self.cancel();
        let controller = web_sys::AbortController::new()
            .map_err(|error| format!("Cannot start reload: {error:?}"))?;
        let request = self.job.begin(keys)?;
        let signal = controller.signal();
        self.controller = Some(controller);
        Ok((request, signal))
    }

    fn finish(&mut self, ticket: u64, failed: bool) -> Option<Vec<ReloadKey>> {
        let keys = self.job.finish(ticket)?;
        if let Some(controller) = self.controller.take() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub type Shared = Rc<RefCell<Flight>>;

pub type Reply = crate::reload_reply::Reply;
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(
    shared: Shared,
    keys: Vec<ReloadKey>,
    delivery: Delivery,
) -> Result<(), String> {
    start_with(shared, keys, None, delivery)
}

pub fn start_with(
    shared: Shared,
    keys: Vec<ReloadKey>,
    intent: Option<crate::edit_intent::Intent>,
    delivery: Delivery,
) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys, intent)?;
    crate::browser::report("Reloading editable sources…");
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                values.push(crate::source_fetch::fetch(&url, &signal).await?);
            }
            Ok(values)
        }
        .await;
        let Some((keys, intent)) = shared
            .borrow_mut()
            .finish(request.ticket, result.is_err())
        else {
            return;
        };
        let reply = Reply::new(keys, intent, result);
        if let Err(error) = deliver(reply, &delivery) {
            crate::browser::report(&format!("Cannot deliver reload: {error:?}"));
        }
    });
    Ok(())
}

fn deliver(result: Reply, delivery: &Delivery) -> Result<(), wasm_bindgen::JsValue> {
    let event = web_sys::Event::new("viewer-reload")?;
    *delivery.borrow_mut() = Some(result);
    let result = web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

#[derive(Default)]
pub struct Flight {
    job: ReloadJob<Option<crate::edit_intent::Intent>>,
    controller: Option<web_sys::AbortController>,
}

impl Flight {
    pub fn cancel(&mut self) -> bool {
        let cancelled = self.job.cancel();
        if let Some(controller) = self.controller.take() {
            controller.abort();
        }
        cancelled
    }

    fn begin(
        &mut self,
        keys: Vec<ReloadKey>,
        intent: Option<crate::edit_intent::Intent>,
    ) -> Result<(Request, web_sys::AbortSignal), String> {
        self.cancel();
        let controller = web_sys::AbortController::new()
            .map_err(|error| format!("Cannot start reload: {error:?}"))?;
        let request = self.job.begin_with(keys, intent)?;
        let signal = controller.signal();
        self.controller = Some(controller);
        Ok((request, signal))
    }

    fn finish(
        &mut self,
        ticket: u64,
        failed: bool,
    ) -> Option<(
        Vec<ReloadKey>,
        Option<crate::edit_intent::Intent>,
    )> {
        let keys = self.job.finish_with(ticket)?;
        if let Some(controller) = self.controller.take() {
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
            } else if event.type_() == "viewer-reload" {
                if let Some(result) = reload_delivery.borrow_mut().take() {
                    match result.and_then(|values| {
                        editor.hydrate(values).map_err(str::to_owned)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            } else if event.type_() == "viewer-reload" {
                if let Some(reply) = reload_delivery.borrow_mut().take() {
                    match reply.result.and_then(|values| {
                        editor.hydrate(values).map_err(str::to_owned)
```

## <a id="checkpoint-32gid-complete"></a>22. Validate restoration before replaying the command

### 1. `src/reload_reply.rs`

**Locate this existing block:**

```rust
impl Reply {
    pub fn new(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Reply {
    pub fn complete(
        self,
        editor: &mut crate::editor::Editor,
    ) -> Result<Option<crate::edit_replay::Reply>, String> {
        if !editor
            .hydrate(self.result?)
            .map_err(str::to_owned)?
        {
            return Ok(None);
        }
        let reply = match self.intent {
            Some(intent) => intent
                .replay(editor)
                .map_err(str::to_owned)?,
            None => crate::edit_replay::Reply::Changed(crate::editor::Change::Scene),
        };
        Ok(Some(reply))
    }

    pub fn new(
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod rehydrate_tests;
pub mod reload_job;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod rehydrate_tests;
#[cfg(test)]
mod reload_complete_tests;
pub mod reload_job;
```

### 3. `src/reload_complete_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    edit_replay::Reply as Edit,
    editor::{Action, Editor},
    reload_reply::Reply,
    reload_url::ReloadUrl,
};
use std::rc::Rc;

fn cold() -> (Editor, Vec<u8>) {
    let mut e = Editor::default();
    let bytes = crate::specimen::bytes();
    e.apply(Action::ReplaceAt(
        bytes.clone(),
        Rc::new(ReloadUrl::new("test:complete".into())),
    ))
    .unwrap();
    e.apply(Action::UnloadSources).unwrap();
    (e, bytes)
}

#[test]
fn complete_replays_original_target_and_keeps_later_view() {
    for kind in 0..3 {
        let (mut e, bytes) = cold();
        let id = e.scene.objects()[0].id;
        let intent = match kind {
            0 => Intent::Move {
                id,
                offset: [0.25, 0.0, 0.0],
            },
            1 => Intent::Delete { id },
            _ => Intent::Save,
        };
        let keys = intent.keys(&e).unwrap();
        e.selected = Some(e.scene.objects()[1].id);
        e.apply(Action::Orbit(0.2, 0.1)).unwrap();
        let selected = e.selected;
        let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        let reply = Reply::new(keys, Some(intent), Ok(vec![bytes]));
        let result = reply.complete(&mut e).unwrap().unwrap();
        assert_eq!(e.selected, selected);
        assert_eq!(e.camera.uniform(), camera);
        match result {
            Edit::Changed(_) => {
                if kind == 0 {
                    assert_ne!(e.scene.objects()[0].model.m, model);
                } else {
                    assert!(!e.scene.contains(id));
                }
                e.apply(Action::Undo).unwrap();
                assert_eq!(e.scene.objects()[0].model.m, model);
                assert!(e.scene.contains(id));
            }
            Edit::Saved(bytes) => {
                let loaded = crate::document::load(&bytes).unwrap();
                assert_eq!(loaded.meshes.len(), e.scene.objects().len());
            }
        }
    }
}

#[test]
fn stale_context_and_failure_cannot_replay() {
    let (mut e, bytes) = cold();
    let keys = e.reload_keys();
    let reply = Reply::new(keys, Some(Intent::Save), Ok(vec![bytes]));
    e.apply(Action::Close).unwrap();
    assert!(reply.complete(&mut e).unwrap().is_none());
    let (mut e, _) = cold();
    let model = e.scene.objects()[0].model.m;
    let reply = Reply::new(
        e.reload_keys(),
        Some(Intent::Move {
            id: e.scene.objects()[0].id,
            offset: [1.0, 0.0, 0.0],
        }),
        Err("Network failed".into()),
    );
    assert_eq!(
        reply.complete(&mut e).unwrap_err(),
        "Network failed"
    );
    assert_eq!(e.scene.objects()[0].model.m, model);
    assert!(e.scene.objects()[0].geometry().is_none());
}
```

## <a id="checkpoint-32gie-response"></a>23. Deliver restored edit and Save results to the dock

### 1. `src/edit_intent.rs`

**Locate this existing block:**

```rust
impl Intent {
    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Intent {
    pub fn command(self) -> &'static str {
        match self {
            Self::Move { .. } => "Move",
            Self::Delete { .. } => "Delete",
            Self::Save => "Save",
        }
    }

    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                None
            } else if event.type_() == "viewer-reload" {
                if let Some(reply) = reload_delivery.borrow_mut().take() {
                    match reply.result.and_then(|values| {
                        editor.hydrate(values).map_err(str::to_owned)
                    }) {
                        Ok(true) => {
                            renderer.set_scene(&editor.scene, editor.selected);
                            let message =
                                "Editable sources restored; display retained.";
                            report(message);
                            panel.answer("Reload Sources", message);
                        }
                        Ok(false) => {}
                        Err(error) => {
                            report(&error);
                            panel.answer("Reload Sources", &error);
                        }
```

Replace it with:

```rust
                None
            } else if event.type_() == "viewer-reload" {
                if let Some(reply) = reload_delivery.borrow_mut().take() {
                    let command = reply
                        .intent
                        .map_or("Reload Sources", |intent| intent.command());
                    match reply.complete(&mut editor) {
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            renderer.set_scene(&editor.scene, editor.selected);
                            let message = if command == "Reload Sources" {
                                "Editable sources restored; display retained."
                                    .to_owned()
                            } else {
                                format!("{command} completed after source reload.")
                            };
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                            let message =
                                save_result(&bytes).unwrap_or_else(|error| error);
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            report(&error);
                            panel.answer(command, &error);
                        }
```

**Locate this block:**

```rust
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
```

Replace it with:

```rust
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| {
            format!(
                "Saved editable document ({} bytes)",
                bytes.len()
            )
        })
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
```

## <a id="checkpoint-32gif-auto"></a>24. Automatically restore sources for Move, Delete and Save

### 1. `src/browser_reload.rs`

**Locate this existing block:**

```rust
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn restore_for(
    intent: crate::edit_intent::Intent,
    editor: &crate::editor::Editor,
    shared: Shared,
    delivery: Delivery,
) -> Result<bool, String> {
    let keys = intent.keys(editor).map_err(str::to_owned)?;
    shared.borrow_mut().cancel();
    if keys.is_empty() {
        return Ok(false);
    }
    start_with(shared, keys, Some(intent), delivery)?;
    Ok(true)
}

pub fn start(
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute(
```

Replace it with:

```rust
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|row| (format!("{:?}", row.id), row.model.m))
        .collect();
    canvas.set_attribute(
        "data-row-placements",
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-selected-id",
        &editor
            .selected
            .map(|id| format!("{id:?}"))
            .unwrap_or_default(),
    )?;
    canvas.set_attribute(
        "data-camera-matrix",
        &serde_json::to_string(&editor.camera.view_projection().m)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
                            );
                        }
                    }
                    Ok(Change::View) => {}
                    Err(error) => {
                        report(error);
                        if event.type_() == "viewer-file" {
                            panel.answer("Open", error);
                        } else {
                            panel.result(error);
                        }
                    }
                }
```

Replace it with:

```rust
                            );
                        }
                    }
                    Ok(Some(Change::View)) | Ok(None) => {}
                    Err(error) => {
                        report(&error);
                        if event.type_() == "viewer-file" {
                            panel.answer("Open", &error);
                        } else {
                            panel.result(&error);
                        }
                    }
                }
```

**Locate this block:**

```rust
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if unloading {
                            report("Editable sources unloaded; display retained.");
```

Replace it with:

```rust
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                let waiting =
                    crate::edit_intent::Intent::capture(&action, editor.selected)
                        .map_err(str::to_owned)
                        .and_then(|intent| match intent {
                            Some(intent) => crate::browser_reload::restore_for(
                                intent,
                                &editor,
                                std::rc::Rc::clone(&reload),
                                std::rc::Rc::clone(&reload_delivery),
                            ),
                            None => Ok(false),
                        });
                let change = waiting.and_then(|waiting| {
                    if waiting {
                        panel.result("Reloading editable sources…");
                        Ok(None)
                    } else {
                        editor
                            .apply(action)
                            .map(Some)
                            .map_err(str::to_owned)
                    }
                });
                match change {
                    Ok(Some(Change::Scene)) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if unloading {
                            report("Editable sources unloaded; display retained.");
```

**Locate this block:**

```rust
                } else if line == "save" {
                    match crate::document::snapshot(&editor.scene) {
                        Ok(bytes) => match crate::file_output::download(&bytes) {
                            Ok(()) => panel.result(&format!(
                                "Saved editable document ({} bytes)",
                                bytes.len()
                            )),
                            Err(error) => {
                                panel.result(&format!("Save failed: {error:?}"))
                            }
                        },
                        Err(error) => panel.result(error),
                    }
                    None
                } else if line == "move" || line.starts_with("move ") {
                    match crate::offset::parse(&line) {
```

Replace it with:

```rust
                } else if line == "save" {
                    let result = crate::browser_reload::restore_for(
                        crate::edit_intent::Intent::Save,
                        &editor,
                        std::rc::Rc::clone(&reload),
                        std::rc::Rc::clone(&reload_delivery),
                    )
                    .and_then(|waiting| {
                        if waiting {
                            Ok("Reloading editable sources…".to_owned())
                        } else {
                            crate::document::snapshot(&editor.scene)
                                .map_err(str::to_owned)
                                .and_then(|bytes| save_result(&bytes))
                        }
                    });
                    panel.result(&result.unwrap_or_else(|error| error));
                    None
                } else if line == "move" || line.starts_with("move ") {
                    match crate::offset::parse(&line) {
```

## <a id="checkpoint-32gj-precision"></a>25. Prove restored Save keeps the source doubles

### 1. `src/specimen.rs`

**Locate this existing block:**

```rust
    session.pb_dumps()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    session.pb_dumps()
}

pub fn precise_bytes() -> Vec<u8> {
    use prost::Message;
    let mut message = session_rust::proto::Session::decode(bytes().as_slice()).unwrap();
    let mesh = &mut message.objects.as_mut().unwrap().meshes[0];
    mesh.vertices.get_mut(&0).unwrap().x = -0.775000000123456;
    message.encode_to_vec()
}
```

### 2. `examples/sample.rs`

**Locate this existing block:**

```rust
        "sample.pb",
        viewer_journey::specimen::bytes(),
    )
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "sample.pb",
        viewer_journey::specimen::precise_bytes(),
    )
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod reload_job;
pub mod reload_reply;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod reload_job;
#[cfg(test)]
mod reload_precision_tests;
pub mod reload_reply;
```

### 4. `src/reload_precision_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    edit_replay::Reply as Edit,
    editor::{Action, Editor},
    reload_reply::Reply,
    reload_url::ReloadUrl,
};
use prost::Message;
use std::rc::Rc;

#[test]
fn cold_save_preserves_source_double_and_move_history() {
    let mut e = Editor::default();
    let bytes = crate::specimen::precise_bytes();
    let source = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    let original = source.objects.unwrap().meshes[0]
        .vertices
        .clone();
    let x = original.get(&0).unwrap().x;
    assert_ne!(x, x as f32 as f64);
    e.apply(Action::ReplaceAt(
        bytes.clone(),
        Rc::new(ReloadUrl::new("test:precision".into())),
    ))
    .unwrap();
    e.apply(Action::SelectNext).unwrap();
    let id = e.selected.unwrap();
    let model = e.scene.objects()[0].model.m;
    Intent::Move {
        id,
        offset: [0.25, 0.0, 0.15],
    }
    .replay(&mut e)
    .unwrap();
    let moved = e.scene.objects()[0].model.m;
    e.apply(Action::UnloadSources).unwrap();
    let reply = Reply::new(
        e.reload_keys(),
        Some(Intent::Save),
        Ok(vec![bytes]),
    );
    let Some(Edit::Saved(saved)) = reply.complete(&mut e).unwrap() else {
        panic!("Saved reply")
    };
    let message = session_rust::proto::Session::decode(saved.as_slice()).unwrap();
    assert_eq!(
        message.objects.unwrap().meshes[0].vertices,
        original
    );
    assert_eq!(e.scene.objects()[0].model.m, moved);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.objects()[0].model.m, model);
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.objects()[0].model.m, moved);
}
```

## <a id="checkpoint-32gja-failures"></a>26. Prove failed restoration cannot partly commit

### 1. `src/lib.rs`

**Locate this existing block:**

```rust
mod reload_complete_tests;
pub mod reload_job;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod reload_complete_tests;
#[cfg(test)]
mod reload_failure_tests;
pub mod reload_job;
```

### 2. `src/reload_failure_tests.rs`

Create the file and type:

```rust
use crate::{
    edit_intent::Intent,
    editor::{Action, Editor},
    reload_reply::Reply,
    reload_url::ReloadUrl,
};
use std::rc::Rc;

fn cold_pair() -> (Editor, Vec<u8>) {
    let mut e = Editor::default();
    let bytes = crate::specimen::precise_bytes();
    e.apply(Action::ReplaceAt(
        bytes.clone(),
        Rc::new(ReloadUrl::new("test:first".into())),
    ))
    .unwrap();
    e.apply(Action::ImportAt(
        bytes.clone(),
        Rc::new(ReloadUrl::new("test:second".into())),
    ))
    .unwrap();
    e.apply(Action::UnloadSources).unwrap();
    (e, bytes)
}

#[test]
fn failed_multi_source_save_restores_nothing() {
    for kind in 0..3 {
        let (mut e, bytes) = cold_pair();
        let keys = e.reload_keys();
        assert_eq!(keys.len(), 2);
        let rows: Vec<_> = e
            .scene
            .objects()
            .iter()
            .map(|row| (row.id, row.model.m))
            .collect();
        let roots = e.scenes().count();
        let camera = e.camera.uniform();
        let bodies = match kind {
            0 => Err("Second source fetch failed".into()),
            1 => Ok(vec![bytes]),
            _ => Ok(vec![bytes, crate::specimen::bytes()]),
        };
        assert!(
            Reply::new(keys, Some(Intent::Save), bodies)
                .complete(&mut e)
                .is_err()
        );
        assert!(
            e.scene
                .objects()
                .iter()
                .all(|row| row.geometry().is_none())
        );
        assert_eq!(
            e.scene
                .objects()
                .iter()
                .map(|row| (row.id, row.model.m))
                .collect::<Vec<_>>(),
            rows
        );
        assert_eq!(e.scenes().count(), roots);
        assert_eq!(e.camera.uniform(), camera);
    }
}

#[test]
fn any_stale_or_duplicate_key_revokes_the_whole_reply() {
    for duplicate in [false, true] {
        let (mut e, bytes) = cold_pair();
        let mut keys = e.reload_keys();
        if duplicate {
            keys[1] = keys[0].clone();
        } else {
            keys[1].epoch += 1;
        }
        let reply = Reply::new(
            keys,
            Some(Intent::Save),
            Ok(vec![bytes.clone(), bytes]),
        );
        assert!(reply.complete(&mut e).unwrap().is_none());
        assert!(
            e.scene
                .objects()
                .iter()
                .all(|row| row.geometry().is_none())
        );
    }
}
```

## Build & Run

![Recorded visual output for Lesson 3.7 (32gja-failures)](../../../screenshots/journey/32gja-failures-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 3.7
npm --prefix ../session_tests run course -- save 3.7
```

Continue with [4.1 · Browser callbacks with a real lifetime](../../consolidation/33-owner.md).
