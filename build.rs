fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "grpc-api")]
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["proto/yacen/v1_1/api.proto"], &["proto/yacen/v1_1"])?;
    Ok(())
}
