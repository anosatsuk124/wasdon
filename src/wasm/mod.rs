use crate::udon::uasm::Uasm;
use ::alloc::boxed::Box;

#[derive(Debug, Default)]
pub struct WasmEntry<'a> {
    data: &'a [u8],
    offset: u64,
}

#[derive(Debug)]
pub struct WasmTranslator<'a> {
    wasm_entry: Box<WasmEntry<'a>>,
    parser: wasmparser::Parser,
    head_position: usize,
    eof: bool,
}

pub struct ParsedData<'a> {
    payload: wasmparser::Payload<'a>,
    next: Option<Box<ParsedData<'a>>>,
}

impl WasmEntry<'_> {
    pub fn new(data: &[u8], offset: u64) -> WasmEntry {
        WasmEntry { data, offset }
    }
}

impl<'a> From<WasmEntry<'a>> for WasmTranslator<'a> {
    fn from(wasm_entry: WasmEntry<'a>) -> Self {
        let wasmparser = wasmparser::Parser::new(wasm_entry.offset);
        let eof = false;

        WasmTranslator {
            wasm_entry: Box::new(wasm_entry),
            parser: wasmparser,
            eof,
            head_position: 0,
        }
    }
}

impl ParsedData<'_> {
    pub fn new(payload: wasmparser::Payload<'_>) -> ParsedData {
        ParsedData {
            payload,
            next: None,
        }
    }

    pub fn update(&mut self, new: Self) {
        self.next = Some(Box::new(new));
    }

    pub fn get_payload(&self) -> &wasmparser::Payload<'_> {
        &self.payload
    }

    pub fn get_next(&self) -> Option<&ParsedData<'_>> {
        Some(self.next.as_ref()?)
    }
}

impl TryInto<Uasm> for ParsedData<'_> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Uasm, Self::Error> {
        unimplemented!()
    }
}

impl<'a> Iterator for ParsedData<'a> {
    type Item = ParsedData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;

        Some(*next)
    }
}

pub trait Interpretable
where
    Self: Iterator<Item = Self> + Sized,
{
    fn interpret(&self) -> anyhow::Result<()>;
    fn interpret_all(&mut self) -> anyhow::Result<()> {
        let current = self;

        for item in current {
            item.interpret()?;
        }

        Ok(())
    }
}

impl Interpretable for ParsedData<'_> {
    fn interpret(&self) -> anyhow::Result<()> {
        // TODO: Implement this function
        unimplemented!()
    }
}

impl<'a, 'b> WasmTranslator<'a>
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
                        return Err(anyhow::anyhow!("Failed to parse wasm file: {}", hint));
                    }
                },
                Err(err) => {
                    return Err(anyhow::anyhow!("Failed to parse wasm file: {}", err));
                }
            }
        };

        self.head_position += consumed;

        Ok(payload)
    }

    pub fn parse_all(&mut self) -> anyhow::Result<ParsedData<'a>> {
        let mut current = ParsedData::new(self.parse()?);
        let mut next = self.parse()?;

        while let wasmparser::Payload::End(_) = next {
            current.update(ParsedData::new(next));
            next = self.parse()?;
        }

        Ok(current)
    }
}
