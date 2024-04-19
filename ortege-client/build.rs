fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["../proto/accounts.proto"], &["../proto"])?;
    Ok(())
}
