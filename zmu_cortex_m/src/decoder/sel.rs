use crate::core::instruction::Instruction;
use bit_field::BitField;

#[allow(non_snake_case)]
pub fn decode_SEL_t1(opcode: u32) -> Instruction {
    Instruction::SEL {
        rd: opcode.get_bits(8..12).into(),
        rn: opcode.get_bits(16..20).into(),
        rm: opcode.get_bits(0..4).into(),
    }
}