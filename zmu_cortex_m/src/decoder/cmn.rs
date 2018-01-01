use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMN_reg_t1(command: u16) -> Instruction {
    Instruction::CMN_reg {
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}