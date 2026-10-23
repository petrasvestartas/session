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
        let text=std::str::from_utf8(&bytes).ok().map(|text|text.replace("\r\n","\n"));
        let bytes=text.as_ref().map_or(bytes.as_slice(),|text|text.as_bytes());
        if crate::origin::FileVersion::of(bytes).hex()!=row.sha256{return Err(format!("Current source changed: {}/{}",row.root,row.path));}
    }
    if actual!=recorded{return Err("Current source file inventory changed".into());}
    Ok(())
}
