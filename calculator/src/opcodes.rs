// sections
pub(crate) const SECTION_TYPE: u8 = 0x01;
pub(crate) const SECTION_IMPORTS: u8 = 0x02;
pub(crate) const SECTION_FUNCTION: u8 = 0x03;
pub(crate) const SECTION_EXPORT: u8 = 0x07;
pub(crate) const SECTION_CODE: u8 = 0x0a;

// types
pub(crate) const DOUBLE_TYPE: u8 = 0x7c;

// expressions
pub(crate) const EXPRESSION_END: u8 = 0x0b;

// constants
pub(crate) const CONSTANT_IMMUTABLE: u8 = 0x00;
pub(crate) const CONSTANT_MUTABLE: u8 = 0x01;

// misc
pub(crate) const FUNCTION_TYPE_MARKER: u8 = 0x60;

// instructions
pub(crate) const INSTR_F64_CONST: u8 = 0x44;
pub(crate) const INSTR_LOCAL_GET: u8 = 0x20;
pub(crate) const INSTR_GLOBAL_GET: u8 = 0x23;
pub(crate) const INSTR_F64_ADD: u8 = 0xa0;
pub(crate) const INSTR_F64_SUB: u8 = 0xa1;
pub(crate) const INSTR_F64_MUL: u8 = 0xa2;
pub(crate) const INSTR_F64_DIV: u8 = 0xa3;
pub(crate) const INSTR_FUNCTION_CALL: u8 = 0x10;

// descriptors
pub(crate) const FUNCTION_DESCRIPTOR: u8 = 0x01;
pub(crate) const CONSTANT_DESCRIPTOR: u8 = 0x03;