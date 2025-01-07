use crate::constants::JMP_REL32_SIZE;

const _CHECK: () = const {
    use crate::constants::JMP_REL8_SIZE;
    assert!(JMP_REL8_SIZE == 2);
};

use super::EncodedInstruction;

/// Encodes `jmp rel8`. Returns [`EncodedInstruction`] of length 2 on success.
///
/// # Notes
/// It does not manipulate of `rel`. In particular `rel` is relative to RIP, meaning
/// next instruction after this one.
#[must_use]
#[inline]
pub fn encode_jmp_rel8(rel: i8) -> EncodedInstruction {
    const OPCODE: u8 = 0xEB;
    let val: u8 = unsafe { core::mem::transmute(rel) };
    let buffer = [OPCODE, val];
    unsafe { EncodedInstruction::from_array_unchecked(buffer) }
}

/// Encodes `jmp rel32`. Returns [`EncodedInstruction`] of length 5 on success.
///
/// # Notes
/// It does not manipulate `rel`. In particular `rel` is relative to RIP, meaning
/// next instruction after this one.
#[must_use]
pub fn encode_jmp_rel32(rel: i32) -> EncodedInstruction {
    const OPCODE: u8 = 0xE9;
    let buffer = {
        let mut buffer = [0u8; JMP_REL32_SIZE];
        buffer[0] = OPCODE;
        let slice: &mut [u8] = &mut buffer;
        slice[1..5].copy_from_slice(&rel.to_le_bytes());
        buffer
    };
    unsafe { EncodedInstruction::from_array_unchecked(buffer) }
}
