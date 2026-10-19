use std::{io, path::{Path, PathBuf}};

pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let Some(root) = std::env::var_os("VIEWER_SAMPLE_PROOF_DIR") else { return Ok(()); };
    let path = PathBuf::from(root).join(path);
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    std::fs::write(path, bytes)
}
