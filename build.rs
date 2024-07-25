fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let proto_file = &["proto/edb.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile(proto_file, &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
    Ok(())
}
