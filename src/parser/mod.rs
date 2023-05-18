use crate::udon::uasm::Uasm;
use ::alloc::boxed::Box;

#[derive(Debug, Default)]
pub struct WasmEntry<'a> {
    data: &'a [u8],
    offset: u64,
}

#[derive(Debug, Default)]
pub struct WasmTranslator<'a> {
    wasm_entry: Box<WasmEntry<'a>>,
    parser: Box<wasmparser::Parser>,
    current_position: usize,
    eof: bool,
}

impl WasmEntry<'_> {
    pub fn new(data: &[u8], offset: u64) -> WasmEntry {
        WasmEntry { data, offset }
    }
}

impl<'a> From<WasmEntry<'a>> for WasmTranslator<'a> {
    fn from(wasm_entry: WasmEntry<'a>) -> Self {
        let wasmparser = Box::new(wasmparser::Parser::new(wasm_entry.offset));
        let eof = false;

        WasmTranslator {
            wasm_entry: Box::new(wasm_entry),
            parser: wasmparser,
            eof,
            ..Default::default()
        }
    }
}

impl WasmTranslator<'_> {
    pub fn to_uasm(&mut self) -> Uasm {
        let uasm_data = Uasm::default();

        // TODO: Implement this function
        todo!()
    }
}
