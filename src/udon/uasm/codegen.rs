use ::alloc::string::String;

use crate::core::{Unit, Units};

use super::Uasm;

impl Units<Uasm> {
    pub fn codegen(&self) -> anyhow::Result<String> {
        let mut code = String::new();

        for unit in self.iter() {
            code.push_str(&unit.codegen()?);
        }

        Ok(code)
    }
}

impl Unit<Uasm> {
    pub fn codegen(&self) -> anyhow::Result<String> {
        let mut code = String::new();

        // let uasm = &self.data;

        todo!()
    }
}
