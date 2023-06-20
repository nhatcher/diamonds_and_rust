// sections
pub(crate) const SECTION_TYPE: u8 = 0x01;
pub(crate) const SECTION_IMPORTS: u8 = 0x02;
pub(crate) const SECTION_FUNCTION: u8 = 0x03;
pub(crate) const SECTION_MEMORY: u8 = 0x05;
pub(crate) const SECTION_GLOBAL: u8 = 0x06;
pub(crate) const SECTION_EXPORT: u8 = 0x07;
pub(crate) const SECTION_CODE: u8 = 0x0a;

// types
pub(crate) const F64_TYPE: u8 = 0x7c;
pub(crate) const I32_TYPE: u8 = 0x7f;

// expressions
pub(crate) const EXPRESSION_END: u8 = 0x0b;

// constants
pub(crate) const CONSTANT_IMMUTABLE: u8 = 0x00;
pub(crate) const CONSTANT_MUTABLE: u8 = 0x01;

// misc
pub(crate) const FUNCTION_TYPE_MARKER: u8 = 0x60;

// instructions
pub(crate) const INSTR_F64_CONST: u8 = 0x44;
pub(crate) const INSTR_I32_CONST: u8 = 0x41;
pub(crate) const INSTR_LOCAL_SET: u8 = 0x21;
pub(crate) const INSTR_LOCAL_GET: u8 = 0x20;
pub(crate) const INSTR_GLOBAL_SET: u8 = 0x22;
pub(crate) const INSTR_GLOBAL_GET: u8 = 0x23;
pub(crate) const INSTR_I32_ADD: u8 = 0x6a;
pub(crate) const INSTR_I32_SUB: u8 = 0x6b;
pub(crate) const INSTR_F64_ADD: u8 = 0xa0;
pub(crate) const INSTR_F64_SUB: u8 = 0xa1;
pub(crate) const INSTR_F64_MUL: u8 = 0xa2;
pub(crate) const INSTR_F64_DIV: u8 = 0xa3;
pub(crate) const INSTR_FUNCTION_CALL: u8 = 0x10;
pub(crate) const INSTR_F64_CEIL: u8 = 0x8d;

// memory
pub(crate) const MEMORY_I32_LOAD: u8 = 0x28;
pub(crate) const MEMORY_I32_STORE: u8 = 0x36;
pub(crate) const MEMORY_F64_LOAD: u8 = 0x2b;
pub(crate) const MEMORY_F64_STORE: u8 = 0x39;

// converts a signed i32 into an f64
pub(crate) const INSTR_F64_CONVERT_I32_S: u8 = 0xb7;
// truncates a F64 into a signed i32
pub(crate) const INSTR_I32_TRUNC_F64_S: u8 = 0xaa;

pub(crate) const INSTR_BLOCK_IF: u8 = 0x04;
pub(crate) const INSTR_BLOCK_ELSE: u8 = 0x05;
pub(crate) const INSTR_BLOCK_LOOP: u8 = 0x03;
pub(crate) const INSTR_VOID: u8 = 0x40;
pub(crate) const INSTR_BR_IF: u8 = 0x0d;

pub(crate) const INSTR_BR: u8 = 0x0c;

pub(crate) const INSTR_F64_EQ: u8 = 0x61;
pub(crate) const INSTR_F64_NE: u8 = 0x62;
pub(crate) const INSTR_F64_LT: u8 = 0x63;
pub(crate) const INSTR_F64_GT: u8 = 0x64;
pub(crate) const INSTR_F64_LE: u8 = 0x65;
pub(crate) const INSTR_F64_GE: u8 = 0x66;

// descriptors
pub(crate) const FUNCTION_DESCRIPTOR: u8 = 0x01;
pub(crate) const CONSTANT_DESCRIPTOR: u8 = 0x03;

// limits
pub(crate) const LIMITS_FLAG_NO_MAX: u8 = 0x00;

// export types
pub(crate) const FUNCTION_EXPORT_KIND: u8 = 0x00;
pub(crate) const MEMORY_EXPORT_KIND: u8 = 0x02;
