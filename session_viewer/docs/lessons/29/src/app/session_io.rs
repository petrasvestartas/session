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
    documents: Vec<Document>,   // one per session
    hidden: Vec<(usize, String)>, // (document, guid) hidden
    texts: Vec<(String, TextLabel, bool)>, // (key, label, active)
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

        let bytes = (*file.session).clone().pb_dumps(); // a copy keeps the undo history
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
    let metadata = Metadata {
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
        texts: scene
            .texts
            .iter()
            .map(|t| (t.key.clone(), t.label.clone(), t.active))
            .collect(),
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

    let mut scene = Scene::new();

    for (meta, bytes) in metadata.documents.into_iter().zip(archive.documents) {
        if !meta.place.into_iter().all(f64::is_finite) || !meta.point_px.is_finite() {
            return Err("Non-finite document placement".into());
        }

        let proto =
            session_rust::proto::Session::decode(bytes.as_slice()).map_err(|e| e.to_string())?;
        super::validate::session(&proto)?;
        let session = Session::pb_loads(&bytes).map_err(|e| e.to_string())?;
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

    scene.hidden = metadata
        .hidden
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
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

    /// A created line keeps its screen pen after reopening.
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
        let bytes = save(&scene).unwrap();
        assert!(scene.undo(), "saving leaves live undo available");
        let restored = open(&bytes).unwrap();
        assert_eq!(restored.docs.len(), 2);
        assert_eq!(restored.hidden.len(), 1);
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
}
