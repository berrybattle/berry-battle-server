use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("game_update_descriptor.bin"))
        .compile(&["proto/game_update.proto"], &["proto"])
        .unwrap();
}
