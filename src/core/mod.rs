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
