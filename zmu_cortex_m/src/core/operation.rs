use core::condition::Condition;
use core::register::Apsr;
use core::PSR;
use bit_field::BitField;

pub fn sign_extend(word: u32, topbit: u8, size: u8) -> i32 {
    if word & (1 << topbit) == (1 << topbit) {
        return (word | (((1 << (size - topbit)) - 1) << topbit)) as i32;
    }
    word as i32
}

//
// Add two numbers and carry
//
// x + y + carry
//
// return tuple of (result, carry, overflow)
//
pub fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
    let unsigned_sum = u64::from(x) + u64::from(y) + (carry_in as u64);
    let signed_sum = (x as i32) + (y as i32) + (carry_in as i32);
    let result = (unsigned_sum & 0xffff_ffff) as u32; // same value as signed_sum<N-1:0>
    let carry_out = u64::from(result) != unsigned_sum;
    let overflow = (result as i32) != signed_sum;

    (result, carry_out, overflow)
}

#[test]
fn test_add_with_carry() {
    let (result, carry, overflow) = add_with_carry(0x410, 4, false);
    assert!(result == 0x414);
    assert!(carry == false);
    assert!(overflow == false);
}


//
// This function performs the condition test for an instruction, based on:
// • the two Thumb conditional branch encodings, encodings T1 and T3 of the B instruction
// • the current values of the xPSR.IT[7:0] bits for other Thumb instructions.
//
pub fn condition_passed(condition: Condition, psr: &PSR) -> bool {
    match condition {
        Condition::EQ => psr.get_z(),
        Condition::NE => !psr.get_z(),
        Condition::CS => psr.get_c(),
        Condition::CC => !psr.get_c(),
        Condition::MI => psr.get_n(),
        Condition::PL => !psr.get_n(),

        Condition::VS => psr.get_v(),
        Condition::VC => !psr.get_v(),

        Condition::HI => psr.get_c() && psr.get_z(),
        Condition::LS => !(psr.get_c() && psr.get_z()),

        Condition::GE => psr.get_n() == psr.get_v(),
        Condition::LT => !(psr.get_n() == psr.get_v()),

        Condition::GT => (psr.get_n() == psr.get_v()) && !psr.get_z(),
        Condition::LE => !((psr.get_n() == psr.get_v()) && !psr.get_z()),

        Condition::AL => true,
    }
}

#[derive(Debug, PartialEq)]
pub enum SRType {
    LSL,
    LSR,
    ASR,
    RRX,
    ROR,
}


// Decode immedate shift type
// input: bits[2], immedate
// output: (shitft type, immedate to use)
//
pub fn decode_imm_shift(typebits: u8, imm5: u8) -> (SRType, u8) {
    match typebits.get_bits(0..3) {
        0b00 => (SRType::LSL, imm5),
        0b01 => (SRType::LSR, if imm5 == 0 { 32 } else { imm5 }),
        0b10 => (SRType::ASR, if imm5 == 0 { 32 } else { imm5 }),
        0b11 => match imm5 {
            0 => (SRType::RRX, 1),
            _ => (SRType::ROR, imm5),
        },
        _ => panic!("invalid typebits"),
    }
}

fn lsl_c(value: u32, shift: u32) -> (u32, bool) {
    assert!(shift > 0);
    let extended = u64::from(value) << shift;

    (extended.get_bits(0..32) as u32, extended.get_bit(32))
}


pub fn shift_c(value: u32, shift_t: SRType, amount: u32, carry_in: bool) -> (u32, bool) {
    assert!(!((shift_t == SRType::RRX) && (amount != 1)));
    if amount == 0 {
        (value, carry_in)
    } else {
        match shift_t {
            SRType::LSL => lsl_c(value, amount),
            _ => panic!("not implemented"),
        }
    }
}
