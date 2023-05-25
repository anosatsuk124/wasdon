pub mod mangle;
pub mod wasm2uasm;

use ::core::ops::Deref;

use ::alloc::{boxed::Box, vec::Vec};

#[derive(Debug)]
pub struct ParsedData<T> {
    data: T,
    next: Option<Box<ParsedData<T>>>,
}

impl<T> ParsedData<T> {
    pub fn new(data: T) -> ParsedData<T> {
        ParsedData { data, next: None }
    }

    pub fn update(self, mut new: Self) -> Self {
        new.next = Some(Box::new(self));

        new
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_next(&self) -> Option<&ParsedData<T>> {
        Some(self.next.as_ref()?)
    }
}

impl<T> Iterator for ParsedData<T> {
    type Item = ParsedData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;

        Some(*next)
    }
}

pub trait InterpretableAs<T>
where
    Self: Iterator<Item = Self> + Sized,
{
    fn interpret(&self) -> anyhow::Result<T>;
    fn interpret_all(&mut self) -> anyhow::Result<Units<T>> {
        let mut units = Units::new();

        for item in self.into_iter() {
            match item.interpret() {
                Ok(unit) => units.push(unit),
                Err(err) => return Err(err),
            }
        }

        Ok(units)
    }
}

#[derive(Debug)]
pub struct Units<T>(pub Vec<T>);

impl<T> Units<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, unit: T) {
        self.0.push(unit);
    }
}

impl<T> Deref for Units<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Iterator for Units<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
