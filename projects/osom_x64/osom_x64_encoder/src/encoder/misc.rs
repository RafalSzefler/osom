use super::{errors::EncodingError, EncodedInstruction};

#[must_use]
#[inline(always)]
pub const fn encode_ret() -> EncodedInstruction {
    unsafe { EncodedInstruction::from_array_unchecked([0xC3]) }
}

/// Encodes NOP operation. Return [`EncodedInstruction`] of size `size` if
/// `size` is in `1..=9`.
///
/// # Errors
/// [`EncodingError::ArgumentOutOfRange`] if `size` is not inside `1..=9` range.
#[inline(always)]
pub const fn encode_nop(size: u8) -> Result<EncodedInstruction, EncodingError> {
    macro_rules! encode {
        ( $array: expr ) => {
            Ok(unsafe { EncodedInstruction::from_array_unchecked($array) })
        };
    }

    match size {
        1 => encode!([0x90]),
        2 => encode!([0x66, 0x90]),
        3 => encode!([0x0F, 0x1F, 0x00]),
        4 => encode!([0x0F, 0x1F, 0x40, 0x00]),
        5 => encode!([0x0F, 0x1F, 0x44, 0x00, 0x00]),
        6 => encode!([0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00]),
        7 => encode!([0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00]),
        8 => encode!([0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
        9 => encode!([0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
        _ => Err(EncodingError::ArgumentOutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ret() {
        let encoded_ret = encode_ret();
        assert_eq!(encoded_ret.as_slice(), &[0xC3]);
    }

    #[test]
    fn test_nop_size() {
        for idx in 1..=9 {
            let encoded_nop = encode_nop(idx).unwrap();
            assert_eq!(encoded_nop.len(), idx);
        }
    }
}
