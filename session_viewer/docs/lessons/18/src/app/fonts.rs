// --8<-- [start:fonts]
// The wasm carries small font subsets; the whole fonts, a few MB, download only when a name needs a character the subsets lack, e.g. Ω.
use crate::engine::text::covers;
use session_rust::Session;
use std::cell::Cell;

thread_local! {
    static ASKED: Cell<bool> = const { Cell::new(false) }; // the whole fonts were asked for
}

/// Fetch the whole fonts once, when `text` has a character the bundled subsets lack.
pub fn need(text: &str) {
    if covers(text) || ASKED.get() {
        return;
    }

    ASKED.set(true);
    log::info!(
        "fonts: fetching the whole fonts for {:?}",
        text.chars().take(40).collect::<String>()
    );
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(fetch()); // starts the download and returns at once; the fonts arrive later as a message
}

/// `need` for a document's name, tree node names and object names.
pub fn need_names(name: &str, session: &Session) {
    need(name);

    if ASKED.get() {
        return;
    }

    for node in session.tree.nodes() {
        need(&node.borrow().name);
    }

    for geometry in session.lookup.values() {
        need(geometry.name());
    }
}

/// The whole fonts from `text/`, posted as `Msg::Fonts`; a failure may be asked again.
#[cfg(target_arch = "wasm32")]
async fn fetch() {
    let mut faces = Vec::new();

    for name in crate::engine::text::FULL_FONTS {
        match super::fetch::fetch_buffer(&format!("text/{name}")).await {
            Ok(array) => faces.push(array.to_vec()),
            Err(error) => {
                log::warn!("fonts: {name}: {error}");
                ASKED.set(false);
                return;
            }
        }
    }

    super::loader::post(crate::Msg::Fonts(faces));
}
// --8<-- [end:fonts]
