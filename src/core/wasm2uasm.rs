use crate::core::InterpretableAs;
use crate::core::ParsedData;
use crate::udon::uasm::data::UasmInstruction;
use crate::udon::uasm::data::{
    UasmCode, UasmCodeLabel, UasmData, UasmDataAttribute, UasmDataSection, UasmType, UasmVarName,
    UasmVariable,
};
use crate::udon::uasm::Uasm;
use ::alloc::format;

use ::alloc::string::String;

impl InterpretableAs<UasmInstruction> for ParsedData<wasmparser::Operator<'_>> {
    fn interpret(&self) -> anyhow::Result<UasmInstruction> {
        use wasmparser::Operator;

        match self.get_data() {
            // TODO: implement all instructions
            // Operator::Nop => Ok(UasmInstruction::new(UasmOpcode::Nop)),
            _ => {
                todo!()
            }
        }
    }
}

impl TryInto<Uasm> for ParsedData<wasmparser::Payload<'_>> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Uasm, Self::Error> {
        unimplemented!()
    }
}

#[doc = include_str!("../../docs/variable.md")]
pub enum VarInfo {
    Local { local_index: usize, fn_name: String },
    Global { global_index: usize },
}

#[doc = include_str!("../../docs/variable.md")]
pub fn generate_variable_name(info: VarInfo) -> String {
    let var = match info {
        VarInfo::Local {
            local_index,
            fn_name,
        } => {
            format!("{fn_name}_L{local_index}")
        }
        VarInfo::Global { global_index } => {
            format!("G__{global_index}")
        }
    };

    format!("__{var}")
}

fn interpret_global_section(
    global_section: &wasmparser::SectionLimited<'_, wasmparser::Global>,
) -> anyhow::Result<Uasm> {
    let mut data_section = UasmDataSection::new();

    let mut code = UasmCode::new();

    for (index, global) in global_section.clone().into_iter().enumerate() {
        if global.is_err() {
            anyhow::bail!(
                "Failed to parse global section: {:?}",
                global.err().unwrap()
            )
        }

        let global = global.unwrap();

        let var_info = VarInfo::Global {
            global_index: index,
        };

        let var_name = generate_variable_name(var_info).into();

        let global_type = global.ty;

        let global_init = global.init_expr; // TODO: implement global initializer
        let mut operators = ParsedData::from(global_init.get_operators_reader().into_iter());

        let initializer = operators.interpret_all()?;

        code.set_block_with_label(
            UasmCodeLabel::new(format!("__INIT_{var_name}").into()),
            initializer.into(),
        );

        let var_name = UasmVarName::new(var_name);

        let var_type = UasmType::try_from(global_type.content_type)?;

        let uasm_data = UasmData {
            attribute: UasmDataAttribute::None,
            variable: UasmVariable::new(var_name, var_type),
        };

        data_section.push_data(&uasm_data);
    }

    Ok(Uasm::new(Some(data_section), None))
}

impl<'a> From<wasmparser::OperatorsIterator<'a>> for ParsedData<wasmparser::Operator<'a>> {
    fn from(value: wasmparser::OperatorsIterator<'a>) -> Self {
        let mut value_iter = value.into_iter();

        // FIXME: handle error
        let mut parsed_data = ParsedData::new(value_iter.next().unwrap().unwrap());

        for value in value_iter {
            let value = value.unwrap();

            parsed_data = parsed_data.update(ParsedData::new(value));
        }

        parsed_data
    }
}

impl InterpretableAs<Uasm> for ParsedData<wasmparser::Payload<'_>> {
    fn interpret(&self) -> anyhow::Result<Uasm> {
        use wasmparser::Payload;

        match self.get_data() {
            Payload::GlobalSection(global_section) => interpret_global_section(global_section),
            x => {
                unimplemented!("Unknown payload: {:?}", x)
            }
        }
    }
}
