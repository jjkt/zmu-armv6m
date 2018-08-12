use bit_field::*;
//use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_STR_imm_t1(command: u16) -> Instruction {
    Instruction::STR_imm {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        imm32: (command.get_bits(6..11) as u32) << 2,
        index: true,
        add: true,
        wback: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t2(command: u16) -> Instruction {
    Instruction::STR_imm {
        rt: From::from(command.get_bits(8..11) as u8),
        rn: Reg::SP,
        imm32: (command.get_bits(0..8) as u32) << 2,
        index: true,
        add: true,
        wback: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_reg_t1(command: u16) -> Instruction {
    Instruction::STR_reg {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        rm: Reg::from(command.get_bits(6..9) as u8),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t1(command: u16) -> Instruction {
    Instruction::STRB_imm {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        imm32: (command.get_bits(6..11) as u32),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_reg_t1(command: u16) -> Instruction {
    Instruction::STRB_reg {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        rm: Reg::from(command.get_bits(6..9) as u8),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t1(command: u16) -> Instruction {
    Instruction::STRH_imm {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        imm32: (command.get_bits(6..11) as u32) << 1,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_STRH_reg_t1(command: u16) -> Instruction {
    Instruction::STRH_reg {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        rm: Reg::from(command.get_bits(6..9) as u8),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t3(opcode: u32) -> Instruction {
    // ARMv7-M
    Instruction::STR_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..12),
        index: true,
        add: true,
        wback: false,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t4(opcode: u32) -> Instruction {
    // ARMv7-M
    Instruction::STR_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..8),
        index: opcode.get_bit(10),
        add: opcode.get_bit(9),
        wback: opcode.get_bit(8),
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
