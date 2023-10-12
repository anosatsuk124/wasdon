use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "std")]
    drop(env_logger::try_init());

    log::info!("Starting translator");

    let args = std::env::args().collect::<Vec<_>>();

    let input_wasm = args
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("No input file specified"))?;

    let mut wasm = Vec::new();
    File::open(input_wasm)
        .unwrap()
        .read_to_end(&mut wasm)
        .unwrap();

    Ok(())
}
