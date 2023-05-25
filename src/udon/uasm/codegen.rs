use ::alloc::string::String;

use crate::core::{InterpretableAs, Units};

use super::Uasm;

impl Units<Uasm> {
    pub fn codegen(&self) -> anyhow::Result<String> {
        let mut code = String::new();

        for unit in self.iter() {
            // code.push_str(&unit.codegen()?);
        }

        Ok(code)
    }
}

impl Into<Uasm> for Units<Uasm> {
    fn into(self) -> Uasm {
        let mut uasm = Uasm::default();

        for data in self.into_iter() {
            let data_section = if let Some(data) = data.data_section {
                data
            } else {
                continue;
            };
            let uasm_data_section = &mut uasm.data_section;
            if uasm_data_section.is_some() {
                data_section.get_data().iter().for_each(|data| {
                    uasm_data_section.as_mut().unwrap().push_data(&data);
                });
            } else {
                uasm.set_data_section(data_section);
            }

            let code_section = if let Some(code) = data.code_section {
                code
            } else {
                continue;
            };

            uasm.set_code_section(code_section);
        }

        uasm
    }
}
