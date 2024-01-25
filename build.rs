use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("bb_rpc_descriptor.bin"))
        .compile(&["proto/data.proto", "proto/services.proto"], &["proto"])
        .unwrap();
}
