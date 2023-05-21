use ::alloc::borrow::Cow;
use ::alloc::{string::String, vec::Vec};
use ::core::ops::Deref;
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
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UasmCodeLabel(String);

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

impl UasmInstruction {
    pub fn new(opcode: UasmOpcode) -> UasmInstruction {
        UasmInstruction { opcode }
    }
}

/// Udon Assembly opcode
#[derive(Debug, Clone)]
pub enum UasmOpcode {
    // TODO: Define all opcodes
}

/// the data section of Udon Assembly
#[derive(Debug, Default)]
pub struct UasmDataSection {
    data: Vec<UasmData>,
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
#[derive(Debug)]
pub struct UasmVarName(String);

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
    Float,
    Double,
    String,
}

impl TryFrom<wasmparser::ValType> for UasmType {
    type Error = anyhow::Error;
    fn try_from(value: wasmparser::ValType) -> Result<Self, Self::Error> {
        use wasmparser::ValType;

        let ty = match value {
            ValType::I32 => UasmType::Int32,
            ValType::I64 => UasmType::Int64,
            ValType::F32 => UasmType::Float,
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

/// the variation of a sync attribute
#[derive(Debug, Default)]
pub enum UasmDataAttributeSync {
    #[default]
    None,
    Linear,
    Smooth,
}
