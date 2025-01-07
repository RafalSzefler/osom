use crate::{constants::JCC_REL32_SIZE, models::Condition};

const _CHECK: () = const {
    use crate::constants::JCC_REL8_SIZE;
    assert!(JCC_REL8_SIZE == 2);
};

use super::EncodedInstruction;

#[allow(clippy::match_same_arms)]
const fn map_cond_to_opcode(cond: Condition) -> u8 {
    match cond {
        Condition::Above => 0x77,
        Condition::AboveOrEqual => 0x73,
        Condition::Below => 0x72,
        Condition::BelowOrEqual => 0x76,
        Condition::Carry => 0x72,
        Condition::NotCarry => 0x73,
        Condition::Equal => 0x74,
        Condition::NotEqual => 0x75,
        Condition::Greater => 0x7F,
        Condition::GreaterOrEqual => 0x7D,
        Condition::Less => 0x7C,
        Condition::LessOrEqual => 0x7E,
        Condition::Overflow => 0x70,
        Condition::NotOverflow => 0x71,
        Condition::Parity => 0x7A,
        Condition::NotParity => 0x7B,
        Condition::Sign => 0x78,
        Condition::NotSign => 0x79,
        Condition::Zero => 0x74,
        Condition::NotZero => 0x75,
    }
}

/// Encodes `jcc rel8`. Returns [`EncodedInstruction`] of length 2 on success.
///
/// # Notes
/// It does no manipulation of `rel`. In particular `rel` is relative to RIP, meaning
/// next instruction after this one.
#[must_use]
#[inline]
pub fn encode_jcc_rel8(cond: Condition, rel: i8) -> EncodedInstruction {
    let val: u8 = unsafe { core::mem::transmute(rel) };
    let buffer = [map_cond_to_opcode(cond), val];
    unsafe { EncodedInstruction::from_array_unchecked(buffer) }
}

/// Encodes `jcc rel32`. Returns [`EncodedInstruction`] of length 6 on success.
///
/// # Notes
/// It does not manipulate `rel`. In particular `rel` is relative to RIP, meaning
/// next instruction after this one.
#[must_use]
pub fn encode_jcc_rel32(cond: Condition, rel: i32) -> EncodedInstruction {
    // Note: rel32 jcc differs from rel8 in two ways: (1) it has additional 0x0F byte in front,
    // and (2) opcode corresponds to rel8 opcode + 0x10. For some reason.
    let opcode = map_cond_to_opcode(cond) + 0x10;
    let buffer = {
        let mut buffer = [0u8; JCC_REL32_SIZE];
        buffer[0] = 0x0F;
        buffer[1] = opcode;
        let slice: &mut [u8] = &mut buffer;
        slice[2..6].copy_from_slice(&rel.to_le_bytes());
        buffer
    };
    unsafe { EncodedInstruction::from_array_unchecked(buffer) }
}
