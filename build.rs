use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_dir = "./src/grpc_stub/";
//    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let proto_file = &["proto/edb.proto", "proto/raft.proto"];

    tonic_build::configure()
        .out_dir(out_dir)
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(original_out_dir.join("edb_descriptor.bin"))
        .compile(proto_file, &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
    Ok(())
}
