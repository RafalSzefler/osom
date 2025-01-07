//! REL constants are here so that assemblers can use them. Since JMP/JCC instructions
//! refor to themselves (as in: own instruction buffer) then assembling final offsets may
//! require change of instruction size, which then recursively affects other JMP/JCC instructions.

pub const JMP_REL8_SIZE: usize = 2;
pub const JMP_REL32_SIZE: usize = 5;
pub const JCC_REL8_SIZE: usize = 2;
pub const JCC_REL32_SIZE: usize = 6;

pub const MAX_INSTRUCTION_SIZE: usize = 15;
