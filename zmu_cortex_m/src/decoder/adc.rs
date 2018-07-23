use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADC_reg_t1(opcode: u16) -> Instruction {
    Instruction::ADC_reg {
        rn: From::from(bits_0_3(opcode)),
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADC_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADC_imm_t1(_opcode: u32) -> Instruction {
    unimplemented!()
}
