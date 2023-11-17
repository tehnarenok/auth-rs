use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("descriptor.bin");

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir)
        .compile(&["./proto/v1.proto"], &["./proto"])?;
    Ok(())
}
