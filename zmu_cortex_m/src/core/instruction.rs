//!
//! Representation of Cortex M Instruction set
//!

use crate::core::condition::Condition;
use crate::core::register::{ExtensionReg, Reg};
use crate::core::thumb::ThumbCode;
use enum_set::EnumSet;

#[derive(Debug, PartialEq, Copy, Clone)]
///
/// Types of shift operations supported
pub enum SRType {
    /// logical shift left
    LSL,
    /// logical shift right
    LSR,
    /// arithmetic shift right
    ASR,
    /// rotate right
    RRX,
    /// rotate right
    ROR,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// IT instruction conditions
pub enum ITCondition {
    /// condition normal operation
    Then,
    /// condition inverted operation
    Else,
}

#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Coding of imm32+carry variants for more efficient run time behaviour
///
pub enum Imm32Carry {
    /// Precalculated value carry value was not relevant
    /// for the decoding.
    NoCarry {
        /// imm32 original value
        imm32: u32,
    },
    /// Precalculated values for cases ASPR.C == 0 and ASPR.C ==1
    /// If carry was relevant for the decoding
    /// tuple of (immediate value, carry).
    Carry {
        /// Values of imm32 and carry, when carry was originally 0.
        imm32_c0: (u32, bool),
        /// Values of imm32 and carry, when carry was originally 1.
        imm32_c1: (u32, bool),
    },
}

#[derive(PartialEq, Debug, Copy, Clone)]
/// Instruction flags setting variants
pub enum SetFlags {
    /// Set Always
    True,
    /// Set Never
    False,
    /// Set when not in "IT" block
    NotInITBlock,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3ShiftParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3Params {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub setflags: SetFlags,
}
#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3NoSetFlagsParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2UsizeParams {
    pub rd: Reg,
    pub rm: Reg,
    pub rotation: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3UsizeParams {
    pub rd: Reg,
    pub rm: Reg,
    pub rn: Reg,
    pub rotation: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg4NoSetFlagsParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub ra: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftParams {
    pub rd: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftNParams {
    pub rd: Reg,
    pub rm: Reg,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2Params {
    pub rd: Reg,
    pub rm: Reg,
    pub setflags: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ImmParams {
    pub rd: Reg,
    pub rn: Reg,
    pub imm32: u32,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ImmCarryParams {
    pub rd: Reg,
    pub rn: Reg,
    pub imm32: Imm32Carry,
    pub setflags: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmCarryParams {
    pub rd: Reg,
    pub imm32: Imm32Carry,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmCarryNoSetFlagsParams {
    pub rn: Reg,
    pub imm32: Imm32Carry,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftNoSetFlagsParams {
    pub rn: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmParams {
    pub r: Reg,
    pub imm32: u32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg643232Params {
    pub rdlo: Reg,
    pub rdhi: Reg,
    pub rm: Reg,
    pub rn: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3HighParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub n_high: bool,
    pub m_high: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg4HighParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub ra: Reg,
    pub n_high: bool,
    pub m_high: bool,
}

#[allow(non_camel_case_types, missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Instruction set
/// These "micro instructions" are produced by the decoder
/// and operated on by the executor.
/// Note that the instruction list is not 1:1 to
/// the mnemonics listed in the ref manual, instead
/// the exact variant is decoded to allow faster runtime.
pub enum Instruction {
    // --------------------------------------------
    //
    // Group: Branch instructions
    //
    // --------------------------------------------
    /// Branch to target address (on condition)
    B_t13 {
        cond: Condition,
        imm32: i32,
        thumb32: bool,
    },
    /// Branch to target address
    B_t24 {
        imm32: i32,
        thumb32: bool,
    },
    /// Call a subroutine
    BL {
        imm32: i32,
    },
    /// Call a subroutine, optionally change instruction set
    BLX {
        rm: Reg,
    },
    /// Change to target address, change instruction set
    BX {
        rm: Reg,
    },
    /// Compare and branch on Nonzero / Zero
    CBZ {
        rn: Reg,
        nonzero: bool,
        imm32: u32,
    },
    /// Table branch, byte offsets
    TBB {
        rn: Reg,
        rm: Reg,
    },
    /// Table branch, halfword offsets
    TBH {
        rn: Reg,
        rm: Reg,
    },

    // --------------------------------------------
    //
    // Group: Standard data-processing instructions
    //
    // --------------------------------------------
    /// Add (immediate)
    ADD_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Add (register)
    ADD_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Add (register, sp)
    ADD_sp_reg {
        params: Reg2ShiftParams,
        thumb32: bool,
    },
    /// Add with Carry
    ADC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Add with Carry
    ADC_imm {
        params: Reg2ImmParams,
    },

    /// Form PC-relative Address
    ADR {
        params: RegImmParams,
        thumb32: bool,
    },

    /// Bitwise AND
    AND_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise AND
    AND_imm {
        params: Reg2ImmCarryParams,
    },

    /// Bitwise Bit Clear
    BIC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise Bit Clear
    BIC_imm {
        params: Reg2ImmCarryParams,
    },

    /// Compare Negative
    CMN_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },
    /// Compare Negative
    CMN_imm {
        params: RegImmParams,
    },

    /// Compare
    CMP_imm {
        params: RegImmParams,
        thumb32: bool,
    },
    /// Compare
    CMP_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },

    /// Bitwise Exclusive OR
    EOR_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise Exclusive OR
    EOR_imm {
        params: Reg2ImmCarryParams,
    },

    /// Copies operand to destination
    MOV_imm {
        params: RegImmCarryParams,
        thumb32: bool,
    },
    /// Copies operand to destination
    MOV_reg {
        params: Reg2Params,
        thumb32: bool,
    },

    /// Bitwise NOT
    MVN_reg {
        params: Reg2ShiftParams,
        thumb32: bool,
    },
    /// Bitwise NOT
    MVN_imm {
        params: RegImmCarryParams,
    },
    /// Bitwise OR NOT
    ORN_reg {
        params: Reg3ShiftParams,
    },

    /// Bitwise OR
    ORR_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise OR
    ORR_imm {
        params: Reg2ImmCarryParams,
    },

    /// Reverse subtract
    RSB_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Reverse subtract
    RSB_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },

    /// Subtract with Carry
    SBC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Subtract with Carry
    SBC_imm {
        params: Reg2ImmParams,
    },

    /// Subtract
    SUB_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Subtract
    SUB_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },

    /// Test equivalence
    TEQ_reg {
        params: Reg2ShiftNoSetFlagsParams,
    },
    /// Test equivalence
    TEQ_imm {
        params: RegImmCarryNoSetFlagsParams,
    },

    /// Test
    TST_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },
    /// Test
    TST_imm {
        params: RegImmCarryNoSetFlagsParams,
    },

    // --------------------------------------------
    //
    // Group: Shift instructions
    //
    // --------------------------------------------
    /// Arithmetic shift right
    ASR_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Arithmetic shift right
    ASR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Logical Shift Left (immediate)
    LSL_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Logical Shift Left (register)
    LSL_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Logical Shift Right (immediate)
    LSR_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Logical Shift Right (register)
    LSR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Rotate Right (immediate)
    ROR_imm {
        params: Reg2ShiftNParams,
    },
    /// Rotate Right (register)
    ROR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Rotate Right with Extend
    RRX {
        params: Reg2Params,
    },

    // --------------------------------------------
    //
    // Group: Multiply instructions
    //
    // --------------------------------------------
    /// Multipy and Accumulate
    MLA {
        params: Reg4NoSetFlagsParams,
    },
    /// Multipy and Subtract
    MLS {
        params: Reg4NoSetFlagsParams,
    },
    /// Multipy
    MUL {
        params: Reg3Params,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Signed multiply instructions (ArmV7-m)
    //
    // --------------------------------------------
    /// Signed Multiply and Accumulate (Long)
    SMLAL {
        params: Reg643232Params,
    },
    /// Signed Multiply (Long)
    SMULL {
        params: Reg643232Params,
    },

    // --------------------------------------------
    //
    // Group: Unsigned Multiply instructions (ARMv7-M base architecture)
    //
    // --------------------------------------------
    UMLAL {
        params: Reg643232Params,
    },
    UMULL {
        params: Reg643232Params,
    },
    // --------------------------------------------
    //
    // Group: Signed Multiply instructions (ARMv7-M DSP extension)
    //
    // --------------------------------------------
    /// Signed multiply: halfwords
    /// variants: SMULTT, SMULBB, SMULTB, SMULBT
    SMUL {
        params: Reg3HighParams,
    },
    /// Signed multiply and Accumulate, halfwords
    /// variants: SMLATT, SMLABB, SMLATB, SMLABT
    SMLA {
        params: Reg4HighParams,
    },

    //SMLAL second variant?
    //SMLALD
    //SMLAW
    //SMLSD
    //SMLSLD
    //SMMLA
    //SMMLS
    //SMMUL
    //SMUAD

    // --------------------------------------------
    //
    // Group: Saturating instructions (ARMv7-M base arch)
    //
    // --------------------------------------------

    //SSAT
    //USAT

    // --------------------------------------------
    //
    // Group: Unsigned Saturating instructions (ARMv7-M DSP extensions)
    //
    // --------------------------------------------
    //USAT16
    //SSAT16

    // --------------------------------------------
    //
    // Group: Saturating add/sub (ARMv7-M DSP extensions)
    //
    // --------------------------------------------
    //QADD
    //QSUB
    //QDADD
    //QDSUB

    // --------------------------------------------
    //
    // Group: Packing and unpacking instructions
    //
    // --------------------------------------------
    /// Signed Extend Byte
    SXTB {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Signed Extend Halfword
    SXTH {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Unsigned Extend Byte
    UXTB {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Unsigned Extend Halfword
    UXTH {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Packing and unpacking instructions (DSP extensions)
    //
    // --------------------------------------------
    //PKHBT, PKHTB
    //SXTAB
    //SXTAB16
    //SXTAH
    //SXTB16
    UXTAB {
        params: Reg3UsizeParams,
    },
    //UXTAB16
    //UXTAH
    //UXTB16

    // --------------------------------------------
    //
    // Group: Divide instructions
    //
    // --------------------------------------------
    /// signed divide
    SDIV {
        params: Reg3NoSetFlagsParams,
    },
    /// Unsigned divide
    UDIV {
        params: Reg3NoSetFlagsParams,
    },

    // --------------------------------------------
    //
    // Group: Parallel add / sub (DSP extension)
    //
    // --------------------------------------------
    //SADD16, QADD16, SHADD16, UADD16, UQADD16, UHADD16
    //SASX, QASX, SHASX, UASX, UQASX, UHSX
    //SSAX, QSAX, SHSAX, USAX, UQSAX, UHSAX
    //SSUB16, QSUB16, SHSUB16, USUB16, UQSUB16, UHSUB16
    //SADD8, QADD8, SHADD8, UADD8, UQADD8, UHADD8
    //SSUB8, QSUB8, SHSUB8, USUB8, UQSUB8, UHSUB8
    UADD8 {
        rd: Reg,
        rn: Reg,
        rm: Reg,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous data-processing instructions
    //
    // --------------------------------------------
    // Bit Field Clear
    BFC {
        rd: Reg,
        lsbit: usize,
        msbit: usize,
    },
    /// Bit Field Insert
    BFI {
        rd: Reg,
        rn: Reg,
        lsbit: usize,
        width: usize,
    },
    /// Count Leading Zeros
    CLZ {
        rd: Reg,
        rm: Reg,
    },
    /// Move Top
    MOVT {
        rd: Reg,
        imm16: u16,
    },
    // RBIT
    /// Byte-reverse word
    REV {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },

    /// Byte-reverse packed half-word
    REV16 {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },

    /// Byte-reverse signed half-word
    REVSH {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },

    //SBFX - signed bit field extract
    /// Unsigned bit field extract
    UBFX {
        rd: Reg,
        rn: Reg,
        lsb: usize,
        widthminus1: usize,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous data-processing instructions (DSP extensions)
    //
    // --------------------------------------------
    /// Select bytes using GE flags
    SEL {
        rd: Reg,
        rn: Reg,
        rm: Reg,
    },
    //USAD8
    //USADA8

    // --------------------------------------------
    //
    // Group: Status register access instructions
    //
    // --------------------------------------------
    /// Move to Register from Special Register
    MRS {
        rd: Reg,
        sysm: u8,
    },
    /// Move to Special Register from ARM Register
    MSR_reg {
        rn: Reg,
        sysm: u8,
        mask: u8,
    },
    /// Change Processor State
    CPS {
        im: bool,
        #[cfg(any(armv7m, armv7em))]
        affect_pri: bool,
        #[cfg(any(armv7m, armv7em))]
        affect_fault: bool,
    },

    // --------------------------------------------
    //
    // Group:  Load and Store instructions
    //
    // --------------------------------------------
    LDR_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDR_lit {
        rt: Reg,
        imm32: u32,
        add: bool,
        thumb32: bool,
    },
    LDR_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRSB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRSB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    LDRSH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    LDRSH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STR_imm {
        rn: Reg,
        rt: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRD_imm {
        rn: Reg,
        rt: Reg,
        rt2: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
    },
    STRB_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRH_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDREX {
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },

    LDREXB {
        rt: Reg,
        rn: Reg,
    },

    LDREXH {
        rt: Reg,
        rn: Reg,
    },
    LDRD_imm {
        rn: Reg,
        rt: Reg,
        rt2: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
    },
    STR_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    STREX {
        rd: Reg,
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },

    STREXB {
        rd: Reg,
        rt: Reg,
        rn: Reg,
    },

    STREXH {
        rd: Reg,
        rt: Reg,
        rn: Reg,
    },

    // --------------------------------------------
    //
    // Group:  Load and Store Multiple instructions
    //
    // --------------------------------------------
    LDM {
        rn: Reg,
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    POP {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    PUSH {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    STM {
        rn: Reg,
        registers: EnumSet<Reg>,
        wback: bool,
        thumb32: bool,
    },
    STMDB {
        rn: Reg,
        registers: EnumSet<Reg>,
        wback: bool,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous
    //
    // --------------------------------------------
    //CLREX
    //DBG
    /// Data Memory Barrier
    DMB,
    /// Data Synchronization Barrier
    DSB,
    /// Instruction Synchronization Barrier
    ISB,

    /// If-then
    IT {
        x: Option<ITCondition>,
        y: Option<ITCondition>,
        z: Option<ITCondition>,
        firstcond: Condition,
        mask: u8,
    },
    /// No operation
    NOP {
        thumb32: bool,
    },

    /// Preload data (immediate)
    PLD_imm {
        rn: Reg,
        imm32: u32,
        add: bool,
    },
    /// Preload data (literal)
    PLD_lit {
        imm32: u32,
        add: bool,
    },
    /// Preload data (register)
    PLD_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
    },
    /// Send event
    SEV {
        thumb32: bool,
    },
    /// Wait for Event
    WFE {
        thumb32: bool,
    },
    /// Wait for interrupt
    WFI {
        thumb32: bool,
    },
    /// Yield
    YIELD {
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Exception generating instructions
    //
    // --------------------------------------------
    /// supervisor call
    SVC {
        imm32: u32,
    },
    /// Breakpoint
    BKPT {
        imm32: u32,
    },
    // --------------------------------------------
    //
    // Group: Coprocessor instructions
    //
    // --------------------------------------------
    //CDP, CDP2
    MCR {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },
    MCR2 {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },
    //MCRR, MCRR2
    //MRC, MRC2
    //MRRC, MRRC2
    LDC_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    LDC2_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    //STC, STC2
    UDF {
        imm32: u32,
        opcode: ThumbCode,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Floating-point load and store instructions
    //
    // --------------------------------------------
    /// FP Load register
    VLDR {
        dd: ExtensionReg,
        rn: Reg,
        add: bool,
        imm32: u32,
        single_reg: bool,
    },
    /// FP Store register
    VSTR {
        dd: ExtensionReg,
        rn: Reg,
        add: bool,
        imm32: u32,
        single_reg: bool,
    },
    // VLDM
    // VPOP
    // VPUSH
    // VSTM

    // --------------------------------------------
    //
    // Group: Floating-point register transfer instructions
    //
    // --------------------------------------------

    // VMOV
    //VMRS
    //VMRS

    // --------------------------------------------
    //
    // Group: Floating-point data-processing instructions
    //
    // --------------------------------------------
    // VABS
    //VADD
    //VCMP
    //VCVT
    //VDIV
    //VFMA
    //VFNMA
    //VMAXNM
    //VMLA
    //VMOV
    //VMOV
    //VMUL
    //VNEG
    //VNMLA
    //VRINTA
    //VRINTZ
    //VSEL
    //VSQRT
    //VSUB
}

use std::fmt;

#[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
fn format_adressing_mode(
    name: &str,
    f: &mut fmt::Formatter,
    rn: Reg,
    rt: Reg,
    imm32: u32,
    index: bool,
    add: bool,
    wback: bool,
    thumb32: bool,
) -> fmt::Result {
    if index {
        if wback {
            // Pre-indexed
            write!(
                f,
                "{}{} {}, [{} , #{}{}]!",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        } else {
            // Offset
            write!(
                f,
                "{}{} {}, [{} {{, #{}{}}}]",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        }
    } else {
        // Post-indexed
        write!(
            f,
            "{}{} {}, [{}], #{}{}",
            name,
            if thumb32 { ".W" } else { "" },
            rt,
            rn,
            if add { "+" } else { "-" },
            imm32
        )
    }
}

#[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
fn format_adressing_mode2(
    name: &str,
    f: &mut fmt::Formatter,
    rn: Reg,
    rt: Reg,
    rt2: Reg,
    imm32: u32,
    index: bool,
    add: bool,
    wback: bool,
    thumb32: bool,
) -> fmt::Result {
    if index {
        if wback {
            // Pre-indexed
            write!(
                f,
                "{}{} {}, {}, [{} , #{}{}]!",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rt2,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        } else {
            // Offset
            write!(
                f,
                "{}{} {}, {}, [{} {{, #{}{}}}]",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rt2,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        }
    } else {
        // Post-indexed
        write!(
            f,
            "{}{} {}, {},  [{}], #{}{}",
            name,
            if thumb32 { ".W" } else { "" },
            rt,
            rt2,
            rn,
            if add { "+" } else { "-" },
            imm32
        )
    }
}

fn setflags_to_str(setflags: SetFlags) -> &'static str {
    match setflags {
        SetFlags::True => "s",
        SetFlags::False | SetFlags::NotInITBlock => "",
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(unused_variables)]
#[allow(clippy::too_many_lines)]
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: shift_t, shift_n formattings missing.
        // TODO: some of the wide instruction formattings missing.
        match *self {
            Self::ADD_imm { params, thumb32 } => {
                if params.rn == params.rd {
                    write!(
                        f,
                        "add{}{} {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(params.setflags),
                        params.rd,
                        params.imm32
                    )
                } else {
                    write!(
                        f,
                        "add{}{} {}, {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(params.setflags),
                        params.rd,
                        params.rn,
                        params.imm32
                    )
                }
            }
            Self::ADC_imm { params } => write!(
                f,
                "adc{}.W {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::ADD_reg { params, thumb32 } => write!(
                f,
                "add{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADD_sp_reg { params, thumb32 } => write!(
                f,
                "add{}{} {}, SP, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADC_reg { params, thumb32 } => write!(
                f,
                "adc{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADR { params, thumb32 } => write!(
                f,
                "adr{} {}, pc, 0x#{:x}",
                if thumb32 { ".W" } else { "" },
                params.r,
                params.imm32
            ),
            Self::AND_reg { params, thumb32 } => write!(
                f,
                "and{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::AND_imm { params } => write!(
                f,
                "and{}.W {},{}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),

            Self::ASR_imm { params, thumb32 } => write!(
                f,
                "asr{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::ROR_imm { params } => write!(
                f,
                "ror{}.w {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::ASR_reg { params, thumb32 } => write!(
                f,
                "asr{}{} {}, {}, {}",
                if thumb32 { ".W" } else { "" },
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::BIC_reg { params, thumb32 } => write!(
                f,
                "bic{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::BIC_imm { params } => write!(
                f,
                "bic{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::TEQ_imm { params } => write!(
                f,
                "teq.w {}, #{}",
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::B_t13 {
                ref cond,
                imm32,
                thumb32,
            } => write!(f, "b{}{} {}", cond, if thumb32 { ".W" } else { "" }, imm32),
            Self::B_t24 { imm32, thumb32 } => {
                write!(f, "b{} {}", if thumb32 { ".W" } else { "" }, imm32)
            }
            Self::BL { imm32 } => write!(f, "bl 0x#{:x}", imm32),
            Self::BX { rm } => write!(f, "bx {}", rm),
            Self::BLX { rm } => write!(f, "blx {}", rm),
            Self::BKPT { imm32 } => write!(f, "bkpt #{}", imm32),

            Self::BFI {
                ref rd,
                ref rn,
                ref lsbit,
                ref width,
            } => write!(f, "bfi {}, {}, #{}, #{}", rd, rn, lsbit, width),

            Self::BFC {
                ref rd,
                ref lsbit,
                ref msbit,
            } => write!(f, "bfc {}, #{}, #{}", rd, lsbit, msbit - lsbit + 1),

            Self::CMN_reg { params, thumb32 } => write!(
                f,
                "cmn{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::CMN_imm { params } => write!(f, "cmn.W {}, #{}", params.r, params.imm32),
            Self::CBZ { rn, nonzero, imm32 } => write!(
                f,
                "cb{}z {}, #{}",
                if nonzero { "n" } else { "" },
                rn,
                imm32,
            ),
            Self::CLZ { rd, rm } => write!(f, "clz {},{}", rd, rm),
            Self::CMP_imm { params, thumb32 } => write!(
                f,
                "cmp{} {}, #{}",
                if thumb32 { ".W" } else { "" },
                params.r,
                params.imm32
            ),
            Self::CMP_reg { params, thumb32 } => write!(
                f,
                "cmp{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),

            #[cfg(any(armv6m))]
            Self::CPS { im } => write!(f, "cps{} i", if im { "ID" } else { "IE" }),
            #[cfg(any(armv7m, armv7em))]
            Self::CPS {
                im,
                affect_pri,
                affect_fault,
            } => write!(
                f,
                "cps{} {}{}",
                if im { "ID" } else { "IE" },
                if affect_pri { "i" } else { "" },
                if affect_fault { "f" } else { "" }
            ),
            Self::DMB => write!(f, "dmb"),
            Self::DSB => write!(f, "dsb"),
            Self::EOR_reg { params, thumb32 } => write!(
                f,
                "eor{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ISB => write!(f, "isb"),
            Self::IT {
                ref x,
                ref y,
                ref z,
                ref firstcond,
                ref mask,
            } => {
                let x_str = match x {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                let y_str = match y {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                let z_str = match z {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                write!(f, "it{}{}{} {}", x_str, y_str, z_str, firstcond)
            }

            Self::LDM {
                rn,
                registers,
                thumb32,
            } => write!(
                f,
                "ldm{} {}, {{{:?}}}",
                if thumb32 { ".W" } else { "" },
                rn,
                registers
            ),
            Self::LDR_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldr{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Self::LDR_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldr", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::LDR_lit {
                rt,
                imm32,
                thumb32,
                add,
            } => {
                if imm32 == 0 {
                    write!(f, "ldr{} {}, [pc]", if thumb32 { ".W" } else { "" }, rt)
                } else {
                    write!(
                        f,
                        "ldr{} {}, [pc, #{}{}]",
                        if thumb32 { ".W" } else { "" },
                        rt,
                        if add { "+" } else { "-" },
                        imm32
                    )
                }
            }
            Self::LDREX { rt, rn, imm32 } => write!(f, "ldrex {}, {}, #{}", rt, rn, imm32),
            Self::LDREXB { rt, rn } => write!(f, "ldrexb {}, {}", rt, rn),
            Self::LDREXH { rt, rn } => write!(f, "ldrexh {}, {}", rt, rn),

            Self::LDRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrb", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::LDRB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrb{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Self::LDRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrh", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::LDRH_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Self::LDRSB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                wback,
                add,
                thumb32,
            } => write!(f, "ldrsb {}, [{}, {}]", rt, rn, rm),
            Self::LDRSH_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrsh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Self::LSL_imm { params, thumb32 } => write!(
                f,
                "lsl{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::LSL_reg { params, thumb32 } => write!(
                f,
                "lsl{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::LSR_reg { params, thumb32 } => write!(
                f,
                "lsr{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::LSR_imm { params, thumb32 } => write!(
                f,
                "lsr{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::MSR_reg { sysm, rn, mask } => write!(f, "msr {}, {}", sysm, rn),
            Self::MRS { rd, sysm } => write!(f, "mrs {}, {}", rd, sysm),
            Self::MUL { params, thumb32 } => write!(
                f,
                "mul{} {}, {}, {}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::SMUL { params } => write!(
                f,
                "smul{}{} {}, {}, {}",
                if params.n_high { "T" } else { "B" },
                if params.m_high { "T" } else { "B" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::SMLA { params } => write!(
                f,
                "smla{}{} {}, {}, {}, {}",
                if params.n_high { "T" } else { "B" },
                if params.m_high { "T" } else { "B" },
                params.rd,
                params.rn,
                params.rm,
                params.ra
            ),
            Self::MOV_reg { params, thumb32 } => write!(
                f,
                "mov{}{} {}, {}",
                if params.setflags { "s" } else { "" },
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm
            ),
            Self::MOV_imm { params, thumb32 } => write!(
                f,
                "mov{}{} {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::MOVT { rd, imm16 } => write!(f, "movt {}, #{}", rd, imm16),
            Self::LDRSH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrsh", f, rn, rt, imm32, index, add, wback, thumb32),

            Self::LDRSB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrsb", f, rn, rt, imm32, index, add, wback, thumb32),

            Self::MVN_reg { params, thumb32 } => write!(
                f,
                "mvn{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::MVN_imm { params } => write!(
                f,
                "mvn{} {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::NOP { .. } => write!(f, "nop"),
            Self::ORR_reg { params, thumb32 } => write!(
                f,
                "orr{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ORR_imm { params } => write!(
                f,
                "orr{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::ORN_reg { params } => write!(
                f,
                "orn{}.w {}, {}, {}{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::EOR_imm { params } => write!(
                f,
                "eor{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::POP { registers, thumb32 } => {
                write!(f, "pop{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Self::PUSH { thumb32, registers } => {
                write!(f, "push{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Self::PLD_imm { rn, imm32, add } => {
                write!(f, "pld [{}, {}{}]", rn, if add { "+" } else { "-" }, imm32)
            }
            Self::PLD_lit { imm32, add } => {
                write!(f, "pld [PC, {}{}]", if add { "+" } else { "-" }, imm32)
            }
            Self::PLD_reg {
                rn,
                rm,
                shift_t,
                shift_n,
            } => write!(
                f,
                "pld [{}, {}, {}]",
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),

            Self::REV { rd, rm, .. } => write!(f, "rev {}, {}", rd, rm),
            Self::REV16 { rd, rm, .. } => write!(f, "rev16 {}, {}", rd, rm),
            Self::REVSH { rd, rm, .. } => write!(f, "revsh {}, {}", rd, rm),
            Self::ROR_reg { params, .. } => write!(
                f,
                "ror{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::RSB_imm { params, thumb32 } => write!(
                f,
                "rsb{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::RRX { params } => write!(
                f,
                "mov.w{} {}, {}, rrx",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rm,
            ),

            Self::SBC_imm { params } => write!(
                f,
                "sbc{}.W {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::RSB_reg { params, thumb32 } => write!(
                f,
                "rsb{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),

            Self::SEV { .. } => write!(f, "sev"),
            Self::SBC_reg { params, thumb32 } => write!(
                f,
                "sbc{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::STM {
                rn,
                wback,
                registers,
                thumb32,
            } => write!(
                f,
                "stm{} {}{}, {{{:?}}}",
                if thumb32 { ".W" } else { "" },
                rn,
                if wback { "!" } else { "" },
                registers
            ),
            Self::STMDB {
                rn,
                wback,
                registers,
            } => write!(
                f,
                "stmdb {}{}, {{{:?}}}",
                rn,
                if wback { "!" } else { "" },
                registers
            ),
            Self::STR_imm {
                rn,
                rt,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("str", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::STREX { rd, rt, rn, imm32 } => {
                write!(f, "strex {}, {}, {}, #{}", rd, rt, rn, imm32)
            }
            Self::STREXB { rd, rt, rn } => write!(f, "strexb {}, {}, {}", rd, rt, rn),
            Self::STREXH { rd, rt, rn } => write!(f, "strexh {}, {}, {} ", rd, rt, rn),

            Self::STRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("strd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Self::LDRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("ldrd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Self::STR_reg {
                rn,
                rm,
                rt,
                index,
                add,
                wback,
                thumb32,
                ref shift_t,
                shift_n,
            } => write!(f, "str {}, [{}, {}]", rt, rn, rm),
            Self::STRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("strb", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::STRB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(f, "strb {}, [{}, {}]", rt, rn, rm),
            Self::STRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("strh", f, rn, rt, imm32, index, add, wback, thumb32),
            Self::STRH_reg {
                rn,
                rm,
                rt,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(f, "strh {}, [{}, {}]", rt, rn, rm),
            Self::SUB_imm { params, thumb32 } => {
                if params.rd == params.rn {
                    write!(
                        f,
                        "sub{}{} {}, #{}",
                        setflags_to_str(params.setflags),
                        if thumb32 { ".W" } else { "" },
                        params.rd,
                        params.imm32
                    )
                } else {
                    write!(
                        f,
                        "sub{}{} {}, {}, #{}",
                        setflags_to_str(params.setflags),
                        if thumb32 { ".W" } else { "" },
                        params.rd,
                        params.rn,
                        params.imm32
                    )
                }
            }
            Self::SUB_reg { params, thumb32 } => write!(
                f,
                "sub{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::TEQ_reg { params } => write!(
                f,
                "teq.W {}, {}, {}",
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::SVC { imm32 } => write!(f, "svc #{}", imm32),
            Self::SXTH { params, thumb32 } => write!(
                f,
                "sxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),

            Self::SXTB { params, thumb32 } => write!(
                f,
                "sxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::TBB { rn, rm } => write!(f, "tbb [{}, {}]", rn, rm),
            Self::TBH { rn, rm } => write!(f, "tbh [{}, {}, lsl #1]", rn, rm),
            Self::TST_reg { params, thumb32 } => write!(
                f,
                "tst{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::TST_imm { params } => write!(
                f,
                "tst {}, #{}",
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::UDF {
                imm32, ref opcode, ..
            } => write!(f, "udf {} (opcode = {})", imm32, opcode),

            Self::UADD8 { rd, rn, rm } => write!(f, "uadd8 {}, {}, {}", rd, rn, rm),
            Self::SEL { rd, rn, rm } => write!(f, "sel {}, {}, {}", rd, rn, rm),
            // ARMv7-M
            Self::UDIV { params } => write!(f, "udiv {}, {}, {}", params.rd, params.rn, params.rm),
            Self::SDIV { params } => write!(f, "sdiv {}, {}, {}", params.rd, params.rn, params.rm),
            // ARMv7-M
            Self::UMLAL { params } => write!(
                f,
                "umlal {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            // ARMv7-M
            Self::UMULL { params } => write!(
                f,
                "umull {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            Self::SMULL { params } => write!(
                f,
                "smull {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            // ARMv7-M
            Self::MLA { params } => write!(
                f,
                "mla {}, {}, {}, {}",
                params.rd, params.rn, params.rm, params.ra
            ),
            // ARMv7-M
            Self::MLS { params } => write!(
                f,
                "mls {}, {}, {}, {}",
                params.rd, params.rn, params.rm, params.ra
            ),
            // ARMv7-M
            Self::SMLAL { params } => write!(
                f,
                "smlal {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            Self::UXTB { params, thumb32 } => write!(
                f,
                "uxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UXTAB { params } => write!(
                f,
                "uxtb.w {},{},{} {}",
                params.rd,
                params.rn,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UXTH { params, thumb32 } => write!(
                f,
                "uxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UBFX {
                rd,
                rn,
                lsb,
                widthminus1,
            } => write!(f, "ubfx {}, {}, #{}, #{}", rd, rn, lsb, widthminus1 + 1),
            Self::VLDR {
                dd,
                rn,
                add,
                imm32,
                single_reg,
            } => write!(f, "vldr {}, {}", dd, rn),
            Self::VSTR {
                dd,
                rn,
                add,
                imm32,
                single_reg,
            } => write!(f, "vstr {}, {}", dd, rn),

            Self::WFE { .. } => write!(f, "wfe"),
            Self::WFI { .. } => write!(f, "wfi"),
            Self::YIELD { .. } => write!(f, "yield"),
            // ARMv7-M
            Self::MCR {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr"),

            // ARMv7-M
            Self::MCR2 {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr2"),

            // ARMv7-M
            Self::LDC_imm {
                ref coproc,
                ref imm32,
                ref crd,
                ref rn,
            } => write!(f, "ldc"),

            // ARMv7-M
            Self::LDC2_imm {
                ref coproc,
                ref imm32,
                ref crd,
                ref rn,
            } => write!(f, "ldc2"),
        }
    }
}

#[allow(unused_variables)]
impl fmt::Display for ITCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Then => write!(f, "t"),
            Self::Else => write!(f, "e"),
        }
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(unused_variables)]
#[allow(clippy::too_many_lines)]
/// Get the size of an instruction in bytes
pub fn instruction_size(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::ADC_imm { .. } => 4,
        Instruction::ADC_reg { params, thumb32 } => isize_t(*thumb32),
        Instruction::ADD_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADD_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADD_sp_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADR { thumb32, .. } => isize_t(*thumb32),
        Instruction::AND_imm { .. } => 4,
        Instruction::AND_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ASR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::ASR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::B_t13 { thumb32, .. } => isize_t(*thumb32),
        Instruction::B_t24 { thumb32, .. } => isize_t(*thumb32),
        Instruction::BFI { .. } => 4,
        Instruction::BFC { .. } => 4,
        Instruction::BIC_imm { .. } => 4,
        Instruction::BIC_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::BKPT { .. } => 2,
        Instruction::BL { .. } => 4,
        Instruction::BLX { .. } => 2,
        Instruction::BX { .. } => 2,

        Instruction::CBZ { .. } => 2,
        //CDP
        //CLREX
        Instruction::CLZ { .. } => 4,
        Instruction::CMN_imm { .. } => 4,
        Instruction::CMN_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::CMP_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::CMP_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::CPS { .. } => 2,

        Instruction::DMB { .. } => 4,
        Instruction::DSB { .. } => 4,

        Instruction::EOR_imm { .. } => 4,
        Instruction::EOR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::ISB { .. } => 4,
        Instruction::IT { .. } => 2,

        Instruction::LDC_imm { .. } => 4,
        Instruction::LDC2_imm { .. } => 4,
        Instruction::LDM { thumb32, .. } => isize_t(*thumb32),
        //LDMDB
        Instruction::LDR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDR_lit { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDRB_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRB_lit
        Instruction::LDRB_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRBT
        Instruction::LDRD_imm { .. } => 4,
        //LDRD_lit
        Instruction::LDREX { .. } => 4,
        Instruction::LDREXB { .. } => 4,
        Instruction::LDREXH { .. } => 4,
        Instruction::LDRH_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRH_lit
        Instruction::LDRH_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRHT
        Instruction::LDRSB_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRSB_lit
        Instruction::LDRSB_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRSBT
        Instruction::LDRSH_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRSH_lit
        Instruction::LDRSH_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSL_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSL_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::MCR { .. } => 4,
        Instruction::MCR2 { .. } => 4,
        Instruction::MLA { .. } => 4,
        Instruction::MLS { .. } => 4,
        Instruction::MOV_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::MOV_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::MOVT { .. } => 4,
        //MRC, MRC2
        //MRRC, MRRC2
        Instruction::MRS { .. } => 4,
        Instruction::MSR_reg { .. } => 4,
        Instruction::MUL { thumb32, .. } => isize_t(*thumb32),
        Instruction::MVN_imm { .. } => 4,
        Instruction::MVN_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::NOP { thumb32, .. } => isize_t(*thumb32),

        //ORN_imm
        Instruction::ORN_reg { .. } => 4,
        Instruction::ORR_imm { .. } => 4,
        Instruction::ORR_reg { thumb32, .. } => isize_t(*thumb32),

        //PKHBT, PKHTB
        Instruction::PLD_imm { .. } => 4,
        Instruction::PLD_lit { .. } => 4,
        Instruction::PLD_reg { .. } => 4,
        //PLI_imm,
        //PLI_reg
        Instruction::POP { thumb32, .. } => isize_t(*thumb32),
        Instruction::PUSH { thumb32, .. } => isize_t(*thumb32),

        //QADD16
        //QADD8
        //QASX
        //QSAX
        //QADD
        //QSUB
        //QDADD
        //QDSUB
        //QSUB16
        //QSUB8

        //RBIT
        Instruction::REV { thumb32, .. } => isize_t(*thumb32),
        Instruction::REV16 { thumb32, .. } => isize_t(*thumb32),
        Instruction::REVSH { thumb32, .. } => isize_t(*thumb32),
        Instruction::ROR_imm { .. } => 4,
        Instruction::ROR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::RRX { .. } => 4,
        Instruction::RSB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::RSB_reg { thumb32, .. } => 4,
        //SADD16
        //SADD8
        //SASX
        Instruction::SBC_imm { .. } => 4,
        Instruction::SBC_reg { thumb32, .. } => isize_t(*thumb32),
        //SBFX
        Instruction::SDIV { .. } => 4,
        Instruction::SEL { .. } => 4,
        Instruction::SEV { thumb32, .. } => isize_t(*thumb32),
        //SHADD16
        //SHADD8
        //SHASX
        //SHSAX
        //SHSUB16
        //SHSUB8
        Instruction::SMLA { .. } => 4,
        //SMLAD
        Instruction::SMLAL { .. } => 4,
        //SMLAL second variant?
        //SMLALD
        //SMLAW
        //SMLSD
        //SMLSLD
        //SMMLA
        //SMMLS
        //SMMUL
        //SMUAD
        Instruction::SMUL { .. } => 4,
        Instruction::SMULL { .. } => 4,
        //SMULW
        //SMUSD
        //SSAT
        //SSAT16
        //SSAX
        //SSUB16
        //SSUB8
        //STC, STC2
        Instruction::STM { thumb32, .. } => isize_t(*thumb32),
        Instruction::STMDB { .. } => 4,
        Instruction::STR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRB_reg { thumb32, .. } => isize_t(*thumb32),
        //STRBT
        Instruction::STRD_imm { .. } => 4,
        Instruction::STREX { .. } => 4,
        Instruction::STREXB { .. } => 4,
        Instruction::STREXH { .. } => 4,
        Instruction::STRH_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRH_reg { thumb32, .. } => isize_t(*thumb32),
        //STRHT
        //STRT
        Instruction::SUB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::SUB_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::SVC { .. } => 2,
        //SXTAB
        //SXTAB16
        //SXTAH
        Instruction::SXTB { thumb32, .. } => isize_t(*thumb32),
        //SXTB16
        Instruction::SXTH { thumb32, .. } => isize_t(*thumb32),

        Instruction::TBB { .. } => 4,
        Instruction::TBH { .. } => 4,
        Instruction::TEQ_imm { .. } => 4,
        Instruction::TEQ_reg { .. } => 4,
        Instruction::TST_imm { .. } => 4,
        Instruction::TST_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::UADD8 { .. } => 4,
        //UADD16
        //UASX
        Instruction::UBFX { .. } => 4,
        Instruction::UDF { thumb32, .. } => isize_t(*thumb32),
        Instruction::UDIV { .. } => 4,
        //UHADD16
        //UHADD8
        //UHASX
        //UHSAX
        //UHSUB16
        //UHSUB8
        //UMAAL
        Instruction::UMLAL { .. } => 4,
        Instruction::UMULL { .. } => 4,
        //UQADD16
        //UQADD8
        //UQASX
        //UQSAX
        //UQSUB16
        //UQSUB8
        //USAD8
        //USADA8
        //USAT
        //USAT16
        //USAX
        //USUB16
        //USUB8
        Instruction::UXTAB { .. } => 4,
        //UXTAB16
        //UXTAH
        Instruction::UXTB { thumb32, .. } => isize_t(*thumb32),
        Instruction::UXTH { thumb32, .. } => isize_t(*thumb32),

        //VABS
        //VADD
        //VCMP
        //VCVTX
        //VCVT
        //VCVTB
        //VCVTT
        //VDIV
        //VFMA
        //VFMS
        //VFNMA
        //VFNMS
        //VLDM
        //VMAXNM
        //VMINNM
        //VMLA
        //VMLS
        //VMOV_imm
        //VMON_reg
        //VMOVX
        //VMRS
        //VMSR
        //VMUL
        //VNEG
        //VNMLA,VNMLS, VNMUL
        //VPOP
        //VPUSH
        //VRINTA, VRINTN, VRINTP, VRiNTM
        //VRINTX,
        //VRINTZ, VRINTR
        //VSEL
        //VSQRT
        //VSTM
        //VSTR
        //VSUB
        Instruction::WFE { thumb32, .. } => isize_t(*thumb32),
        Instruction::WFI { thumb32, .. } => isize_t(*thumb32),
        Instruction::YIELD { thumb32, .. } => isize_t(*thumb32),
        Instruction::VLDR { .. } => 4,
        Instruction::VSTR { .. } => 4,
    }
}

#[inline(always)]
fn isize_t(thumb32: bool) -> usize {
    if thumb32 {
        4
    } else {
        2
    }
}
