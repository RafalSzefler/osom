use super::{Condition, Immediate32, Immediate64, Label, Scale, GPR};

/// Definition of instructions. Not all combinations of
/// parameters are valid. For example `MOV_RR` requires
/// both registers to be of the same size.
///
/// The enum tags correspond roughly to Intel's instructions.
/// Suffixes have the following meaning:
///
///   * R = register
///   * M = memory
///   * I = immediate
///   * L = label
///   * I64 = 64-bit immediate (used only with `MOV_RI64`)
///
/// So for example `MOV_RR(GPR, GPR)` corresponds to Intel's
/// `MOV reg, reg` instruction, while `MOV_RM(GPR, GPR, Scale, i32)`
/// to Intel's `MOV reg, [scale*reg + offset]`.
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    /// Unknown instruction. Should not be used.
    UNKNOWN = 0,

    // ********
    // * Movs *
    // ********
    /// `MOV reg, reg`
    MOV_RR(GPR, GPR),

    /// `MOV reg, [scale*reg + offset]`
    MOV_RM(GPR, GPR, Scale, i32),

    /// `MOV [scale*reg + offset], reg`
    MOV_MR(GPR, Scale, i32, GPR),

    /// `MOV reg, imm32`, sign extended in case of 64-bit register
    MOV_RI(GPR, Immediate32),

    /// `MOV reg, imm64`
    MOV_RI64(GPR, Immediate64),

    /// `MOV reg, label + offset`
    MOV_RL(GPR, Label, i32),

    /// `CMOVcc reg, reg`
    COND_MOV_RR(Condition, GPR, GPR),

    /// `CMOVcc reg, [scale*reg + offset]`
    COND_MOV_RM(Condition, GPR, GPR, Scale, i32),

    // **********************
    // * Arithmetic & logic *
    // **********************
    /// `ADD reg, reg`
    ADD_RR(GPR, GPR),

    /// `ADD reg, [scale*reg + offset]`
    ADD_RM(GPR, GPR, Scale, i32),

    /// `ADD [scale*reg + offset], reg`
    ADD_MR(GPR, Scale, i32, GPR),

    /// `ADD reg, imm32`, sign extended in case of 64-bit register
    ADD_RI(GPR, Immediate32),

    /// `SUB reg, reg`
    SUB_RR(GPR, GPR),

    /// `SUB reg, [scale*reg + offset]`
    SUB_RM(GPR, GPR, Scale, i32),

    /// `SUB [scale*reg + offset], reg`
    SUB_MR(GPR, Scale, i32, GPR),

    /// `SUB reg, imm32`, sign extended in case of 64-bit register
    SUB_RI(GPR, Immediate32),

    /// `AND reg, reg`
    AND_RR(GPR, GPR),

    /// `AND reg, [scale*reg + offset]`
    AND_RM(GPR, GPR, Scale, i32),

    /// `AND [scale*reg + offset], reg`
    AND_MR(GPR, Scale, i32, GPR),

    /// `AND reg, imm32`, sign extended in case of 64-bit register
    AND_RI(GPR, Immediate32),

    /// `OR reg, reg`
    OR_RR(GPR, GPR),

    /// `OR reg, [scale*reg + offset]`
    OR_RM(GPR, GPR, Scale, i32),

    /// `OR [scale*reg + offset], reg`
    OR_MR(GPR, Scale, i32, GPR),

    /// `OR reg, imm32`, sign extended in case of 64-bit register
    OR_RI(GPR, Immediate32),

    /// `XOR reg, reg`
    XOR_RR(GPR, GPR),

    /// `XOR reg, [scale*reg + offset]`
    XOR_RM(GPR, GPR, Scale, i32),

    /// `XOR [scale*reg + offset], reg`
    XOR_MR(GPR, Scale, i32, GPR),

    /// `XOR reg, imm32`, sign extended in case of 64-bit register
    XOR_RI(GPR, Immediate32),

    /// `CMP reg, reg`
    CMP_RR(GPR, GPR),

    /// `CMP reg, [scale*reg + offset]`
    CMP_RM(GPR, GPR, Scale, i32),

    /// `CMP [scale*reg + offset], reg`
    CMP_MR(GPR, Scale, i32, GPR),

    /// `CMP reg, imm32`, sign extended in case of 64-bit register
    CMP_RI(GPR, Immediate32),

    // ****************
    // * Control flow *
    // ****************
    /// `RET`
    RET,

    /// `JMP label + offset`
    JUMP_L(Label, i32),

    /// `JMP [reg]`
    JUMP_R(GPR),

    /// `Jcc label + offset`
    COND_JUMP_L(Condition, Label, i32),

    /// `Jcc [reg]`
    COND_JUMP_R(Condition, GPR),

    /// `CALL label + offset`
    CALL_L(Label, i32),

    /// `CALL [reg]`
    CALL_R(GPR),
}

impl Instruction {
    const _CHECK: () = {
        assert!(size_of::<Self>() <= 16);
    };
}
