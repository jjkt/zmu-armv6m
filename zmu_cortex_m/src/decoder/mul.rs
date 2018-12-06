use crate::core::instruction::Instruction;
use crate::core::register::Reg;
use bit_field::BitField;

#[allow(non_snake_case)]
#[inline]
pub fn decode_MUL_t1(opcode: u16) -> Instruction {
    Instruction::MUL {
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(0..3) as u8),
        setflags: true,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_MUL_t2(opcode: u32) -> Instruction {
    Instruction::MUL {
        rn: Reg::from(opcode.get_bits(16..20) as u8),
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        setflags: false,
        thumb32: true,
    }
}
