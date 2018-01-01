use core::bits::*;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_AND_reg_t1(command: u16) -> Instruction {
    Instruction::AND_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}