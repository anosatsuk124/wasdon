use super::interpret::ParsedData;
use ::alloc::boxed::Box;

#[derive(Debug, Default)]
pub struct WasmEntry<'a> {
    data: &'a [u8],
    offset: u64,
}

impl WasmEntry<'_> {
    pub fn new(data: &[u8], offset: u64) -> WasmEntry {
        WasmEntry { data, offset }
    }
}

impl<'a> From<WasmEntry<'a>> for WasmParser<'a> {
    fn from(wasm_entry: WasmEntry<'a>) -> Self {
        let wasmparser = wasmparser::Parser::new(wasm_entry.offset);
        let eof = true;

        Self {
            wasm_entry: Box::new(wasm_entry),
            parser: wasmparser,
            eof,
            head_position: 0,
        }
    }
}

#[derive(Debug)]
pub struct WasmParser<'a> {
    wasm_entry: Box<WasmEntry<'a>>,
    parser: wasmparser::Parser,
    head_position: usize,
    eof: bool,
}

impl<'a, 'b> WasmParser<'a>
where
    'a: 'b,
{
    pub fn parse(&'b mut self) -> anyhow::Result<wasmparser::Payload<'a>> {
        let (consumed, payload) = {
            let parser = &mut self.parser;
            match parser.parse(&self.wasm_entry.data[self.head_position..], self.eof) {
                Ok(parsed) => match parsed {
                    wasmparser::Chunk::Parsed { consumed, payload } => (consumed, payload),
                    wasmparser::Chunk::NeedMoreData(hint) => {
                        unreachable!("Unexpected Error: need more data. Hint: {}", hint)
                    }
                },
                Err(err) => {
                    return Err(anyhow::anyhow!("Failed to parse wasm file: {}", err));
                }
            }
        };

        log::info!("Consumed {} bytes", consumed);
        self.head_position += consumed;

        log::info!("Payload: {:?}", payload);

        Ok(payload)
    }

    pub fn parse_all(&mut self) -> anyhow::Result<ParsedData<wasmparser::Payload>> {
        let mut current = ParsedData::new(self.parse()?);
        let mut next = self.parse()?;

        loop {
            if let wasmparser::Payload::End(_) = next {
                log::info!("End of payload");
                break;
            }
            log::info!("Update with: {:?}", &next);
            current = current.update(ParsedData::new(next));
            next = self.parse()?;
        }

        Ok(current)
    }
}
