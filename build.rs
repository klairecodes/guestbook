//use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("guestbook.bin"))
        .compile(&["proto/guestbook.proto"], &["proto"])
        .unwrap();

    tonic_build::compile_protos("proto/guestbook.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
