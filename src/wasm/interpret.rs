use crate::core::Interpretable;
use crate::udon::uasm::Uasm;
use ::alloc::boxed::Box;

pub struct ParsedData<'a> {
    payload: wasmparser::Payload<'a>,
    next: Option<Box<ParsedData<'a>>>,
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

impl Interpretable for ParsedData<'_> {
    fn interpret(&self) -> anyhow::Result<()> {
        // TODO: Implement this function
        unimplemented!()
    }
}
