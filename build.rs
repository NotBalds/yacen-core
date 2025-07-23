fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["proto/yacen/api_v2.proto"], &["proto/yacen"])?;
    Ok(())
}
