use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=REGEN_PROTO");

    let wood_dir = PathBuf::from("../src/proto");
    let session_dir = PathBuf::from("../../session/session_proto");
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/proto");

    if std::env::var("REGEN_PROTO").as_deref() == Ok("0")
        || !wood_dir.exists()
        || !session_dir.exists()
    {
        return;
    }

    let mut protos: Vec<PathBuf> = std::fs::read_dir(&wood_dir)
        .expect("read wood protos")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("proto"))
        .collect();
    // read_dir order differs between file systems; sorted keeps the output identical everywhere
    protos.sort();

    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    if std::env::var_os("PROTOC").is_none_or(|path| path.is_empty()) {
        std::env::set_var(
            "PROTOC",
            protoc_bin_vendored::protoc_bin_path().expect("bundled protoc"),
        );
    }

    let staging = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("proto_staging");
    std::fs::create_dir_all(&staging).expect("create staging");
    std::fs::create_dir_all(&out_dir).expect("create src/proto");

    let mut config = prost_build::Config::new();
    config.out_dir(&staging);
    config.extern_path(".session_proto", "::session_rust::proto");
    config
        .compile_protos(&protos, &[&wood_dir, &session_dir])
        .expect("prost-build: compile wood protos");

    // copy only what changed, so cargo does not rebuild the crate on every run
    for entry in std::fs::read_dir(&staging).expect("read staging") {
        let source = entry.expect("entry").path();
        let target = out_dir.join(source.file_name().unwrap());
        let bytes = std::fs::read(&source).expect("read generated");

        if std::fs::read(&target).ok().as_deref() != Some(bytes.as_slice()) {
            std::fs::write(&target, &bytes).expect("write generated");
        }
    }
}
