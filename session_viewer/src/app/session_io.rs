use super::scene::{FileDoc, Scene};
use crate::engine::text::TextLabel;
use prost::Message;
use serde::{Deserialize, Serialize};
use session_rust::{Session, Xform};
use std::rc::Rc;

const MAGIC: &[u8] = b"SESSION-VIEWER\x01\n"; // file header

const LIMIT: usize = 512 * 1024 * 1024; // largest file, bytes

/// The whole file after the header.
#[derive(Clone, PartialEq, Message)]
struct Archive {
    #[prost(bytes = "vec", repeated, tag = "1")]
    documents: Vec<Vec<u8>>, // one protobuf session per document
    #[prost(bytes = "vec", tag = "2")]
    metadata: Vec<u8>, // `Metadata` as JSON
}

/// Everything about the scene that is not a document.
#[derive(Serialize, Deserialize)]
struct Metadata {
    #[serde(default)]
    created_doc: Option<usize>, // index of the `Created` document
    #[serde(default)]
    current_layer: Option<(usize, String)>, // (document, layer) new objects go to
    documents: Vec<Document>,     // one per session
    hidden: Vec<(usize, String)>, // (document, guid) hidden
    #[serde(default)]
    locked: Vec<(usize, String)>, // (document, guid) locked
    #[serde(default)]
    colors: Vec<(usize, String, [u8; 3])>, // face colour overrides
    #[serde(default)]
    edge_colors: Option<Vec<(usize, String, [u8; 3])>>, // edge colour overrides, None in old files
    texts: Vec<(String, TextLabel, bool)>, // (key, label, active)
    #[serde(default)]
    groups: Vec<(usize, String)>, // (document, tree node guid) of each group
}

/// One document's name and placement.
#[derive(Serialize, Deserialize)]
struct Document {
    name: String,     // file name
    place: [f64; 16], // placement matrix
    point_px: f32,    // point size override
}

/// The scene as `.session` file bytes.
pub fn save(scene: &Scene) -> Result<Vec<u8>, String> {
    if !scene.streamed.is_empty() || !scene.sheets.is_empty() {
        return Err("This scene contains streamed sources. Open complete source documents before saving an editable session.".into());
    }

    let mut documents = Vec::new();
    let mut size = 0usize;

    for file in &scene.docs {
        if file.display_only {
            return Err(
                "A source document is not retained; the complete session cannot be saved.".into(),
            );
        }

        let bytes = file.session.to_proto().encode_to_vec(); // live entries, history kept, nothing copied
        size = size.saturating_add(bytes.len());

        if size > LIMIT {
            return Err("Session exceeds the 512 MiB file limit".into());
        }

        documents.push(bytes);
    }

    let mut hidden: Vec<_> = scene
        .hidden
        .iter()
        .map(|(doc, id)| (*doc, id.to_string()))
        .collect();
    hidden.sort();
    let mut locked: Vec<_> = scene
        .locked
        .iter()
        .map(|(doc, id)| (*doc, id.to_string()))
        .collect();
    locked.sort();
    let mut colors: Vec<_> = scene
        .colors
        .iter()
        .map(|((doc, id), color)| (*doc, id.to_string(), *color))
        .collect();
    colors.sort();
    let mut groups: Vec<_> = scene
        .groups
        .iter()
        .map(|(doc, id)| (*doc, id.to_string()))
        .collect();
    groups.sort();
    let metadata = Metadata {
        created_doc: scene.created_doc,
        current_layer: scene.current_layer.clone(),
        documents: scene
            .docs
            .iter()
            .map(|f| Document {
                name: f.name.clone(),
                place: f.place.m,
                point_px: f.point_px,
            })
            .collect(),
        hidden,
        locked,
        colors,
        edge_colors: Some({
            let mut colors: Vec<_> = scene
                .edge_colors
                .iter()
                .map(|((doc, id), color)| (*doc, id.to_string(), *color))
                .collect();
            colors.sort();
            colors
        }),
        texts: scene
            .texts
            .iter()
            .map(|t| (t.key.clone(), t.label.clone(), t.active))
            .collect(),
        groups,
    };
    let archive = Archive {
        documents,
        metadata: serde_json::to_vec(&metadata).map_err(|e| e.to_string())?,
    };

    if archive.encoded_len() + MAGIC.len() > LIMIT {
        return Err("Session exceeds the 512 MiB file limit".into());
    }

    let mut bytes = MAGIC.to_vec();
    archive.encode(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

/// A scene from `.session` file bytes.
pub fn open(bytes: &[u8]) -> Result<Scene, String> {
    if bytes.len() > LIMIT {
        return Err("Session exceeds the 512 MiB file limit".into());
    }

    let payload = bytes
        .strip_prefix(MAGIC)
        .ok_or("Not a Session Viewer file")?;
    let archive = Archive::decode(payload).map_err(|e| e.to_string())?;
    let metadata: Metadata =
        serde_json::from_slice(&archive.metadata).map_err(|e| e.to_string())?;

    if metadata.documents.len() != archive.documents.len() {
        return Err("Document inventory does not match".into());
    }

    if metadata
        .created_doc
        .is_some_and(|index| index >= metadata.documents.len())
    {
        return Err("Created document index is outside the inventory".into());
    }

    let mut scene = Scene::new();
    scene.created_doc = metadata.created_doc;
    // identity state first, so each row is made hidden and colored
    scene.hidden = metadata
        .hidden
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
    scene.locked = metadata
        .locked
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
    scene.groups = metadata
        .groups
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
    // old files have one colour for faces and edges
    scene.edge_colors = metadata
        .edge_colors
        .unwrap_or_else(|| metadata.colors.clone())
        .into_iter()
        .map(|(doc, id, color)| ((doc, Rc::from(id)), color))
        .collect();
    scene.colors = metadata
        .colors
        .into_iter()
        .map(|(doc, id, color)| ((doc, Rc::from(id)), color))
        .collect();

    for (meta, bytes) in metadata.documents.into_iter().zip(archive.documents) {
        if !meta.place.into_iter().all(f64::is_finite) || !meta.point_px.is_finite() {
            return Err("Non-finite document placement".into());
        }

        let proto =
            session_rust::proto::Session::decode(bytes.as_slice()).map_err(|e| e.to_string())?;
        super::validate::session(&proto)?;
        drop(proto); // checked; the kernel decodes its own copy
        let session = Session::pb_loads(&bytes).map_err(|e| e.to_string())?;
        drop(bytes); // the document is converted; free its bytes before the walk
        super::validate::retained(&session)?;
        scene.add_file(FileDoc {
            name: meta.name,
            place: Xform::from_matrix(meta.place),
            point_px: meta.point_px,
            display_only: false,
            session: Rc::new(session),
        });
    }

    for (key, label, active) in metadata.texts {
        scene.register_text(key, label, active);
    }

    // a layer that is gone is not restored
    if let Some((doc, name)) = metadata.current_layer {
        let _ = scene.set_current_layer(doc, &name);
    }

    Ok(scene)
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(inline_js = r#"
export function downloadSession(bytes) {
    const url = URL.createObjectURL(new Blob([bytes], {type:'application/octet-stream'}));

    const a = document.createElement('a'); a.href=url; a.download='session.session';
    document.body.append(a); a.click(); a.remove(); setTimeout(()=>URL.revokeObjectURL(url),10000);
}
export function chooseSession() {
    return new Promise((resolve,reject)=> {
        const input=document.createElement('input'); input.type='file'; input.accept='.session';
        input.oncancel=()=>resolve(null);
        input.onchange=async()=>{try {const file=input.files[0]; if(!file){resolve(null);return;}
            if(file.size>512*1024*1024)throw Error('Session exceeds the 512 MiB file limit');
            resolve(new Uint8Array(await file.arrayBuffer()));}catch(e){reject(e);}};
        input.click();
    });
}
"#)]
    extern "C" {
        #[wasm_bindgen(catch, js_name=downloadSession)]
        pub fn download(bytes: &[u8]) -> Result<(), JsValue>;

        #[wasm_bindgen(js_name=chooseSession)]
        fn choose() -> js_sys::Promise;
    }

    /// Open a file picker and install the chosen session.
    pub fn pick() {
        let promise = choose();
        wasm_bindgen_futures::spawn_local(async move {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await;

            match result {
                Ok(value) if value.is_null() => {}
                Ok(value) => match super::open(&js_sys::Uint8Array::new(&value).to_vec()) {
                    Ok(scene) => super::super::loader::install_saved_scene(scene),
                    Err(error) => super::super::feedback::status(&error),
                },
                Err(error) => {
                    super::super::feedback::status(&format!("Cannot open session: {error:?}"))
                }
            }
        });
    }
}
#[cfg(target_arch = "wasm32")]
pub use browser::{download, pick};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::deform::Target;
    use session_rust::{Geometry, Mesh, Point};

    /// A created text keeps its key, placement and shown flag; an undone one reopens hidden.
    #[test]
    fn a_created_text_survives_save_and_open() {
        let mut scene = Scene::new();
        scene.add_text(crate::app::edit::tests::text(1.0));
        scene.add_text(crate::app::edit::tests::text(2.0));
        assert!(scene.undo());
        let restored = open(&save(&scene).unwrap()).unwrap();
        let texts: Vec<_> = restored
            .texts
            .iter()
            .map(|text| (text.key.as_str(), &text.label.placement, text.active))
            .collect();
        let shown: Vec<_> = scene
            .texts
            .iter()
            .map(|text| (text.key.as_str(), &text.label.placement, text.active))
            .collect();
        assert_eq!(texts, shown);
        assert_eq!(restored.visible_texts().len(), 1);
    }

    /// A created line keeps its screen pen after reopening.
    #[test]
    fn created_curves_keep_visible_screen_pens_after_open() {
        let mut scene = Scene::new();
        scene
            .model(&crate::app::modeling::Modeling::Line(
                [-3000., -5000., 200.],
                [-3000., -1000., 200.],
            ))
            .unwrap();
        Rc::make_mut(&mut scene.docs[0].session).add_group("roof");
        scene.set_current_layer(0, "roof").unwrap();
        let restored = open(&save(&scene).unwrap()).unwrap();
        assert_eq!(restored.created_doc, Some(0));
        assert_eq!(restored.current_layer(), Some((0, "roof".to_string())));
        assert_eq!(restored.tables.seg.ribbons.len(), 1);
        assert_eq!(restored.tables.seg.ribbons[0].radius, 0.);
        assert_eq!(
            restored.tables.obj.rows[0].flags & crate::engine::gpu::Instance::FLAG_SHEET,
            0
        );
    }

    /// Edits, placements, hidden and colour state survive a save.
    #[test]
    fn edited_documents_placements_hidden_state_and_history_survive_save() {
        let mut source = Session::new("source");
        let mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(0., 0., 0.),
                Point::new(10., 0., 0.),
                Point::new(0., 10., 0.),
            ],
            vec![vec![0, 1, 2]],
        );
        source.add_mesh(mesh, None);
        let shared = Rc::new(source);
        let mut scene = Scene::new();

        for x in [100., 200.] {
            scene.add_file(FileDoc {
                name: format!("placement {x}"),
                place: Xform::translation(x, 0., 0.),
                session: Rc::clone(&shared),
                point_px: 3.,
                display_only: false,
            });
        }

        scene
            .edit_subobject(
                0,
                Target::Face(0),
                &Xform::translation(0., 0., 7.),
                "move face",
            )
            .unwrap();
        scene.hidden.insert(scene.identity_of(1).unwrap());
        scene.locked.insert(scene.identity_of(0).unwrap());
        scene
            .colors
            .insert(scene.identity_of(1).unwrap(), [240, 80, 30]);
        scene
            .edge_colors
            .insert(scene.identity_of(1).unwrap(), [30, 80, 240]);
        let count = Rc::strong_count(&scene.docs[1].session);
        let bytes = save(&scene).unwrap();
        assert_eq!(Rc::strong_count(&scene.docs[1].session), count, "no copy");
        assert!(scene.undo(), "saving leaves live undo available");
        let restored = open(&bytes).unwrap();
        assert_eq!(restored.docs.len(), 2);
        assert_eq!(restored.hidden.len(), 1);
        assert_eq!(restored.locked, scene.locked);
        assert_eq!(restored.colors, scene.colors);
        assert_eq!(restored.edge_colors, scene.edge_colors);
        let mut archive = Archive::decode(&bytes[MAGIC.len()..]).unwrap();
        let mut old: serde_json::Value = serde_json::from_slice(&archive.metadata).unwrap();
        old.as_object_mut().unwrap().remove("edge_colors");
        archive.metadata = serde_json::to_vec(&old).unwrap();
        let mut legacy = MAGIC.to_vec();
        archive.encode(&mut legacy).unwrap();
        let legacy = open(&legacy).unwrap();
        assert_eq!(
            legacy.edge_colors, legacy.colors,
            "legacy overrides still color both channels"
        );
        assert!(!restored.selectable(0));
        assert!(restored.selectable(1));
        assert_eq!(restored.docs[0].place.m[12], 100.);
        assert_eq!(restored.docs[1].place.m[12], 200.);
        let Geometry::Mesh(first) = restored.geometry(0).unwrap() else {
            panic!()
        };
        let Geometry::Mesh(second) = restored.geometry(1).unwrap() else {
            panic!()
        };
        assert_eq!(first.vertex[&0].z, 7.);
        assert_eq!(second.vertex[&0].z, 0.);
        assert_eq!(first.face[&0], vec![0, 1, 2]);
    }

    /// Junk and truncated files are refused.
    #[test]
    fn incomplete_or_foreign_files_are_rejected() {
        assert!(open(b"not a session").is_err());
        let bytes = save(&Scene::new()).unwrap();
        assert!(open(&bytes[..bytes.len() - 1]).is_err());
    }

    /// Time and peak resident memory of opening the documents in VIEWER_SESSION_BENCH, saved.
    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "benchmark; VIEWER_SESSION_BENCH lists .pb documents"]
    fn bench_open() {
        let Some(paths) = std::env::var_os("VIEWER_SESSION_BENCH") else {
            return;
        };
        let mut scene = Scene::new();

        for path in std::env::split_paths(&paths) {
            let session = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
            scene.add_file(FileDoc {
                name: path.display().to_string(),
                place: Xform::identity(),
                session: Rc::new(session),
                point_px: 0.,
                display_only: false,
            });
        }

        let bytes = save(&scene).unwrap();
        drop(scene);
        // resident MiB, or its peak since the reset below
        let status = |key: &str| {
            let text = std::fs::read_to_string("/proc/self/status").unwrap();
            let line = text.lines().find(|line| line.starts_with(key)).unwrap();
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<f64>()
                .unwrap()
                / 1024.
        };
        std::fs::write("/proc/self/clear_refs", "5").unwrap();
        let start = status("VmRSS:");
        let clock = std::time::Instant::now();
        let restored = open(&bytes).unwrap();
        let ms = clock.elapsed().as_secs_f64() * 1000.;
        let peak = status("VmHWM:") - start;
        std::fs::create_dir_all("target/review").unwrap();
        std::fs::write(
            "target/review/bench-open.txt",
            format!(
                "{:.0} MiB file, {} rows: {ms:.0} ms, peak +{peak:.0} MiB\n",
                bytes.len() as f64 / 1_048_576.,
                restored.row_count()
            ),
        )
        .unwrap();
    }
}
