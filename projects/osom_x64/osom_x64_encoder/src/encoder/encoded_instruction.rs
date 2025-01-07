#![allow(clippy::len_without_is_empty)]

use crate::constants::MAX_INSTRUCTION_SIZE;

/// Represents an encoded x64 instruction. It is an array of 15 bytes plus 1 byte length, that
/// is capable of holding any x64 instruction.
pub struct EncodedInstruction {
    len: u8,
    buffer: [u8; MAX_INSTRUCTION_SIZE],
}

impl EncodedInstruction {
    const _CHECK: () = const {
        assert!(size_of::<EncodedInstruction>() <= 16);
    };

    /// Creates a new instance of [`EncodedInstruction`].
    ///
    /// # Safety
    /// `len` has to be at most 15 and first `len` bytes in `buffer` has to contain
    /// a valid x64 encoded instruction.
    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(
        len: u8,
        buffer: [u8; MAX_INSTRUCTION_SIZE],
    ) -> Self {
        core::hint::assert_unchecked(len as usize <= MAX_INSTRUCTION_SIZE);
        Self { len, buffer }
    }

    /// Creates a new instance of [`EncodedInstruction`].
    ///
    /// # Safety
    /// `array` has to fully contain a valid x64 encoded instruction.
    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_array_unchecked<const T: usize>(array: [u8; T]) -> Self {
        const { assert!(T <= MAX_INSTRUCTION_SIZE, "Max instruction size is 15.") };
        let mut buffer = [0u8; MAX_INSTRUCTION_SIZE];

        let mut idx = 0;
        while idx < T {
            buffer[idx] = array[idx];
            idx += 1;
        }

        #[allow(clippy::cast_possible_truncation)]
        Self::new_unchecked(T as u8, buffer)
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer.as_ptr(), self.len() as usize) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        unsafe {
            core::hint::assert_unchecked(self.len as usize <= MAX_INSTRUCTION_SIZE);
        };
        self.len
    }
}
