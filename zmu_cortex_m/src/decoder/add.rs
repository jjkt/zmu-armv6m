use core::bits::{
    bit_7, bits_0_3, bits_0_7, bits_0_8, bits_3_6, bits_3_7, bits_6_9, bits_8_11, Bits,
};
use core::instruction::Instruction;
use core::operation::thumb_expand_imm;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_reg_t1(command: u16) -> Instruction {
    Instruction::ADD_reg {
        rm: From::from(bits_6_9(command)),
        rn: From::from(bits_3_6(command)),
        rd: From::from(bits_0_3(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_reg_t2_ADD_SP_reg(command: u16) -> Instruction {
    let rdn = From::from((bit_7(command) << 3) + bits_0_3(command));

    Instruction::ADD_reg {
        rm: From::from(bits_3_7(command)),
        rd: rdn,
        rn: rdn,
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_reg_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: bits_6_9(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rn: From::from(bits_8_11(command)),
        rd: From::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: From::from(bits_8_11(command)),
        rn: Reg::SP,
        imm32: (bits_0_8(command) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::SP,
        rn: Reg::SP,
        imm32: (bits_0_7(command) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t3(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8, 11);
    let rn: u8 = opcode.get_bits(16, 19);
    let s: u8 = opcode.get_bits(20, 20);
    Instruction::ADD_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: thumb_expand_imm(),
        setflags: s == 1,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t4(_opcode: u32) -> Instruction {
    unimplemented!()
}
