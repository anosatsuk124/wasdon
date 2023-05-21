use std::{fs::File, io::Read};

use wasdon::{core::InterpretableAs, udon::uasm::Uasm};

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

    let wasm_entry = wasdon::wasm::parser::WasmEntry::new(wasm.as_slice(), 0);

    let mut wasm_parser = wasdon::wasm::parser::WasmParser::from(wasm_entry);

    log::info!("data: {:x?}", &wasm);
    log::info!("data size: {:?}", &wasm.len());

    let mut parsed_data = wasm_parser.parse_all()?;

    log::info!("{:?}", &parsed_data);

    let uasm_units = parsed_data.interpret_all()?;

    log::info!("Units<Uasm>: {:?}", &uasm_units);

    Ok(())
}
