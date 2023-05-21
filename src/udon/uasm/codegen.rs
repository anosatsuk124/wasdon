use ::alloc::string::String;

use crate::core::{InterpretableAs, Unit, Units};

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

        for unit in self.into_iter() {
            match unit {
                Unit::GlobalUnit { data } => {
                    let data_section = if let Some(data) = data.data_section {
                        data
                    } else {
                        continue;
                    };
                    if uasm.data_section.is_some() {
                        // TODO:
                        data_section.get_data().iter().for_each(|data| {
                            uasm.data_section.unwrap().push_data(&data);
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
                Unit::NonGlobalUnit { data } => {
                    todo!()
                }
            }
        }

        uasm
    }
}
