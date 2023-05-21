use ::alloc::borrow::Cow;
use ::alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ::core::ops::Deref;
use alloc::boxed::Box;
use alloc::format;
use hashbrown::HashMap;

use crate::core::Units;

/// the whole data structure of Udon Assembly
#[derive(Debug, Default)]
pub struct Uasm {
    /// the data section of Udon Assembly
    pub data_section: Option<UasmDataSection>,
    /// the code section of Udon Assembly
    pub code_section: Option<UasmCodeSection>,
}

impl ToString for Uasm {
    fn to_string(&self) -> String {
        let mut code = String::new();
        let data_section = if let Some(data_section) = &self.data_section {
            format!(
                r#"
            .data_start
            {}
            .data_end
            "#,
                data_section.to_string()
            )
        } else {
            String::new()
        };

        let code_section = if let Some(code_section) = &self.code_section {
            format!(
                r#"
            .code_start
            {}
            .code_end
            "#,
                code_section.to_string()
            )
        } else {
            String::new()
        };

        code.push_str(data_section.as_str());
        code.push_str(code_section.as_str());

        code
    }
}

impl Uasm {
    pub fn new(
        data_section: Option<UasmDataSection>,
        code_section: Option<UasmCodeSection>,
    ) -> Uasm {
        Uasm {
            data_section,
            code_section,
        }
    }

    pub fn set_data_section(&mut self, data_section: UasmDataSection) {
        self.data_section = Some(data_section);
    }

    pub fn set_code_section(&mut self, code_section: UasmCodeSection) {
        self.code_section = Some(code_section);
    }
}

/// the code section of Udon Assembly
#[derive(Debug)]
pub enum UasmCodeSection {
    /// the code section with export
    Export(UasmCode),
    /// the code section without export
    NoExport(UasmCode),
}

impl ToString for UasmCodeSection {
    fn to_string(&self) -> String {
        match self {
            UasmCodeSection::Export(code_section) => {
                let mut code = String::new();
                for (label, block) in code_section.0.iter() {
                    let label = label.to_string();
                    code.push_str(
                        format!(
                            r#"
                        .export {}
                        {}:
                            {}
                        "#,
                            &label,
                            &label,
                            block.to_string()
                        )
                        .as_str(),
                    );
                }

                code
            }
            UasmCodeSection::NoExport(code_section) => {
                let mut code = String::new();
                for (label, block) in code_section.0.iter() {
                    code.push_str(
                        format!(
                            r#"
                        {}:
                            {}
                        "#,
                            label.to_string(),
                            block.to_string()
                        )
                        .as_str(),
                    );
                }

                code
            }
        }
    }
}

/// the map of code labels and their code blocks
#[derive(Debug)]
pub struct UasmCode(HashMap<UasmCodeLabel, UasmCodeBlock>);

impl UasmCode {
    pub fn new() -> UasmCode {
        UasmCode(HashMap::new())
    }

    /// insert a code block with a label
    pub fn set_block_with_label(&mut self, label: UasmCodeLabel, block: UasmCodeBlock) {
        self.0.insert(label, block);
    }

    pub fn get_block_with_label(&self, label: &UasmCodeLabel) -> Option<&UasmCodeBlock> {
        self.0.get(label)
    }
}

/// the label of a code block
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct UasmCodeLabel(String);

impl ToString for UasmCodeLabel {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl UasmCodeLabel {
    pub fn new(label: Cow<'_, str>) -> UasmCodeLabel {
        UasmCodeLabel(label.into_owned())
    }
}

/// the code block of a code section
#[derive(Debug)]
pub struct UasmCodeBlock {
    instructions: Vec<UasmInstruction>,
}

impl ToString for UasmCodeBlock {
    fn to_string(&self) -> String {
        let mut code = String::new();

        for instruction in self.instructions.iter() {
            code.push_str("\n");
            code.push_str(&instruction.to_string());
        }

        code
    }
}

impl UasmCodeBlock {
    pub fn new() -> UasmCodeBlock {
        UasmCodeBlock {
            instructions: Vec::new(),
        }
    }

    pub fn push_instruction(&mut self, instruction: &UasmInstruction) {
        self.instructions.push(instruction.clone());
    }

    pub fn get_instructions(&self) -> &Vec<UasmInstruction> {
        &self.instructions
    }
}

impl From<Units<UasmInstruction>> for UasmCodeBlock {
    fn from(units: Units<UasmInstruction>) -> UasmCodeBlock {
        let mut block = UasmCodeBlock::new();

        for unit in units.iter() {
            let instruction = unit.get_data();
            block.push_instruction(instruction);
        }

        block
    }
}

/// Udon Assembly instruction
#[derive(Debug, Clone)]
pub struct UasmInstruction {
    pub opcode: UasmOpcode,
}

impl ToString for UasmInstruction {
    fn to_string(&self) -> String {
        self.opcode.to_string()
    }
}

impl UasmInstruction {
    pub fn new(opcode: UasmOpcode) -> UasmInstruction {
        UasmInstruction { opcode }
    }
}

/// Udon Assembly opcode
#[derive(Debug, Clone)]
pub enum UasmOpcode {
    // TODO: Define all opcodes
    Nop,
    Push(UasmVarName),
    Pop,
    Jump(UasmCodeLabel),
    JumpIfFalse(UasmCodeLabel),
    JumpIndirect(UasmVarName),
    Copy,
    Extern(String),
    /// NOTE: I don't know what this really do.
    Annotation,
}

impl ToString for UasmOpcode {
    fn to_string(&self) -> String {
        match self {
            &UasmOpcode::Nop => "NOP".to_string(),
            &UasmOpcode::Push(ref var_name) => format!("PUSH,{}", var_name.to_string()),
            &UasmOpcode::Pop => "POP".to_string(),
            &UasmOpcode::Jump(ref label) => format!("JUMP,{}", label.to_string()),
            &UasmOpcode::JumpIfFalse(ref label) => format!("JUMP_IF_FALSE,{}", label.to_string()),
            &UasmOpcode::JumpIndirect(ref var_name) => {
                format!("JUMP_INDIRECT,{}", var_name.to_string())
            }
            &UasmOpcode::Copy => "COPY".to_string(),
            &UasmOpcode::Extern(ref name) => format!(r#"EXTERN,"{}""#, name),
            &UasmOpcode::Annotation => unreachable!(),
        }
    }
}

/// the data section of Udon Assembly
#[derive(Debug, Default)]
pub struct UasmDataSection {
    data: Vec<UasmData>,
}

impl ToString for UasmDataSection {
    fn to_string(&self) -> String {
        let mut code = String::new();
        for data in self.data.iter() {
            code.push_str(format!(r#"{}"#, data.attribute.to_string(),).as_str());
            code.push_str(
                format!(
                    r#"{}: {}"#,
                    data.variable.name.to_string(),
                    data.variable.ty.to_string()
                )
                .as_str(),
            );
        }

        code
    }
}

impl UasmDataSection {
    pub fn new() -> UasmDataSection {
        UasmDataSection { data: Vec::new() }
    }

    pub fn push_data(&mut self, data: UasmData) {
        self.data.push(data);
    }

    pub fn get_data(&self) -> &Vec<UasmData> {
        &self.data
    }
}

/// the data section of Udon Assembly
#[derive(Debug)]
pub struct UasmData {
    pub attribute: UasmDataAttribute,
    pub variable: UasmVariable,
}

impl UasmData {
    pub fn set_attribute(&mut self, attribute: UasmDataAttribute) {
        self.attribute = attribute;
    }

    pub fn set_variable(&mut self, variable: UasmVariable) {
        self.variable = variable;
    }
}

/// the variables of a data section
#[derive(Debug)]
pub struct UasmVariable {
    pub name: UasmVarName,
    pub ty: UasmType,
}

impl UasmVariable {
    pub fn new(name: UasmVarName, ty: UasmType) -> Self {
        UasmVariable { name, ty }
    }
}

/// the name of a variable
#[derive(Debug, Clone)]
pub struct UasmVarName(String);

impl ToString for UasmVarName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl UasmVarName {
    pub fn new(name: Cow<'_, str>) -> UasmVarName {
        UasmVarName(name.into_owned())
    }
}

/// the typped value of a variable
#[derive(Debug)]
pub enum UasmType {
    Int32,
    Int64,
    Single,
    Double,
    String,
}

impl ToString for UasmType {
    fn to_string(&self) -> String {
        match self {
            &UasmType::Int32 => "%SystemInt32",
            &UasmType::Int64 => "%SystemInt64",
            &UasmType::Single => "%SystemSingle",
            &UasmType::Double => "$SystemDouble",
            &UasmType::String => "%SystemString",
        }
        .to_string()
    }
}

impl TryFrom<wasmparser::ValType> for UasmType {
    type Error = anyhow::Error;
    fn try_from(value: wasmparser::ValType) -> Result<Self, Self::Error> {
        use wasmparser::ValType;

        let ty = match value {
            ValType::I32 => UasmType::Int32,
            ValType::I64 => UasmType::Int64,
            ValType::F32 => UasmType::Single,
            ValType::F64 => UasmType::Double,
            ValType::V128 => anyhow::bail!("Unsupported type: {:?}", value), // TODO: Support V128
            _ => anyhow::bail!("Unsupported type: {:?}", value),
        };

        Ok(ty)
    }
}

/// the attributes of a data section
#[derive(Debug, Default)]
pub enum UasmDataAttribute {
    /// the data section with no attribute
    #[default]
    None,
    /// the data section with export
    Export,
    /// the data section without export
    Sync(UasmDataAttributeSync),
}

impl ToString for UasmDataAttribute {
    fn to_string(&self) -> String {
        match self {
            &UasmDataAttribute::None => "",
            &UasmDataAttribute::Export => ".export",
            // TODO: Support Sync
            &UasmDataAttribute::Sync(ref sync) => todo!(),
        }
        .to_string()
    }
}

/// the variation of a sync attribute
#[derive(Debug, Default)]
pub enum UasmDataAttributeSync {
    #[default]
    None,
    Linear,
    Smooth,
}

impl ToString for UasmDataAttributeSync {
    fn to_string(&self) -> String {
        match self {
            &UasmDataAttributeSync::None => "none",
            &UasmDataAttributeSync::Linear => "linear",
            &UasmDataAttributeSync::Smooth => "smooth",
        }
        .to_string()
    }
}
