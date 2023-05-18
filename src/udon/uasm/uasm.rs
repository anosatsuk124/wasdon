use ::alloc::{string::String, vec::Vec};
use hashbrown::HashMap;

/// the whole data structure of Udon Assembly
#[derive(Debug, Default)]
pub struct Uasm {
    data_section: UasmDataSection,
    code_section: Option<UasmCodeSection>,
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
#[derive(Debug, Default)]
pub struct UasmCode(HashMap<UasmCodeLabel, UasmCodeBlock>);

/// the label of a code block
#[derive(Debug, Default)]
pub struct UasmCodeLabel(String);

/// the code block of a code section
#[derive(Debug, Default)]
pub struct UasmCodeBlock {
    instructions: Vec<UasmInstruction>,
}

/// Udon Assembly instruction
#[derive(Debug)]
pub struct UasmInstruction {
    opcode: UasmOpcode,
    operands: Vec<UasmOperand>,
}

/// Udon Assembly opcode
#[derive(Debug)]
pub enum UasmOpcode {
    // TODO: Define all opcodes
}

/// Udon Assembly operand
#[derive(Debug)]
pub enum UasmOperand {
    // TODO: Define all operands
}

/// the data section of Udon Assembly
#[derive(Debug, Default)]
pub struct UasmDataSection {
    data: Vec<UasmData>,
}

/// the data section of Udon Assembly
#[derive(Debug, Default)]
pub struct UasmData {
    attributes: UasmDataAttribute,
    variables: UasmVariable,
}

/// the variables of a data section
#[derive(Debug, Default)]
pub struct UasmVariable(HashMap<UasmVarName, UasmTypedValue>);

/// the name of a variable
#[derive(Debug, Default)]
pub struct UasmVarName(String);

/// the typped value of a variable
#[derive(Debug)]
pub enum UasmTypedValue {
    // TODO: Define all available types
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
