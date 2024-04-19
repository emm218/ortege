fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .build_client(false)
        .compile(&["./proto/accounts.proto"], &["./proto"])?;
    Ok(())
}
