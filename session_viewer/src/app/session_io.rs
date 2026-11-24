pub use viewer_core::session_io::*;

#[cfg(target_arch = "wasm32")]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(inline_js = r#"
export function downloadSession(bytes) {
    const url = URL.createObjectURL(new Blob([bytes], {type:'application/octet-stream'}));

    const a = document.createElement('a'); a.href=url; a.download='session.session';
    document.body.append(a); a.click(); a.remove(); setTimeout(()=>URL.revokeObjectURL(url),10000);
}
export function downloadTextFile(name, text) {
    const url = URL.createObjectURL(new Blob([text], {type:'text/plain;charset=utf-8'}));
    const a = document.createElement('a'); a.href=url; a.download=name || 'export.obj';
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
export function chooseImportFile() {
    return new Promise((resolve,reject)=> {
        const input=document.createElement('input'); input.type='file'; input.accept='.pb,.obj,.session';
        input.oncancel=()=>resolve(null);
        input.onchange=async()=>{try {const file=input.files[0]; if(!file){resolve(null);return;}
            if(file.size>512*1024*1024)throw Error('Import exceeds the 512 MiB file limit');
            resolve([file.name, new Uint8Array(await file.arrayBuffer())]);}catch(e){reject(e);}};
        input.click();
    });
}
"#)]
    extern "C" {
        #[wasm_bindgen(catch, js_name=downloadSession)]
        pub fn download(bytes: &[u8]) -> Result<(), JsValue>;

        #[wasm_bindgen(catch, js_name=downloadTextFile)]
        pub fn download_text(name: &str, text: &str) -> Result<(), JsValue>;

        #[wasm_bindgen(js_name=chooseSession)]
        fn choose() -> js_sys::Promise;

        #[wasm_bindgen(js_name=chooseImportFile)]
        fn choose_import() -> js_sys::Promise;
    }

    /// Open a file picker and install the chosen session.
    pub fn pick() {
        let promise = choose();
        wasm_bindgen_futures::spawn_local(async move {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await;

            match result {
                Ok(value) if value.is_null() => {}
                Ok(value) => {
                    match super::open_with_state(&js_sys::Uint8Array::new(&value).to_vec()) {
                        Ok(opened) => super::super::loader::install_saved_scene(opened),
                        Err(error) => super::super::feedback::status(&error),
                    }
                }
                Err(error) => {
                    super::super::feedback::status(&format!("Cannot open session: {error:?}"))
                }
            }
        });
    }

    /// Open a file picker and import `.pb`, `.obj` or `.session` into the viewer.
    pub fn pick_import() {
        let promise = choose_import();
        wasm_bindgen_futures::spawn_local(async move {
            let result = wasm_bindgen_futures::JsFuture::from(promise).await;
            match result {
                Ok(value) if value.is_null() => {}
                Ok(value) => {
                    let pair = js_sys::Array::from(&value);
                    let name = pair
                        .get(0)
                        .as_string()
                        .unwrap_or_else(|| "import.pb".into());
                    let bytes = js_sys::Uint8Array::new(&pair.get(1)).to_vec();
                    if name.to_ascii_lowercase().ends_with(".session") {
                        match super::open_with_state(&bytes) {
                            Ok(opened) => super::super::loader::install_saved_scene(opened),
                            Err(error) => super::super::feedback::status(&error),
                        }
                    } else {
                        super::super::loader::install_imported_file(name, bytes);
                    }
                }
                Err(error) => {
                    super::super::feedback::status(&format!("Cannot import file: {error:?}"))
                }
            }
        });
    }
}
#[cfg(target_arch = "wasm32")]
pub use browser::{download, download_text, pick, pick_import};
