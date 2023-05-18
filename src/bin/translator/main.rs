fn main() -> anyhow::Result<()> {
    #[cfg(feature = "std")]
    drop(env_logger::try_init());

    log::info!("Starting translator");
    Ok(())
}
