pub mod mangle;

use ::core::ops::Deref;

use ::alloc::vec::Vec;

pub trait InterpretableAs<T>
where
    Self: Iterator<Item = Self> + Sized,
{
    fn interpret(&self) -> anyhow::Result<Unit<T>>;
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
pub enum Unit<T> {
    GlobalUnit { data: T },
    NonGlobalUnit { data: T },
}

impl<T> Unit<T> {
    pub fn get_data(&self) -> &T {
        match self {
            Self::GlobalUnit { data } => data,
            Self::NonGlobalUnit { data } => data,
        }
    }
}

#[derive(Debug)]
pub struct Units<T>(pub Vec<Unit<T>>);

impl<T> Units<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, unit: Unit<T>) {
        self.0.push(unit);
    }
}

impl<T> Deref for Units<T> {
    type Target = Vec<Unit<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
