fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "grpc-api")]
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["proto/yacen/v1_0/api.proto"], &["proto/yacen/v1_0"])?;
    Ok(())
}
