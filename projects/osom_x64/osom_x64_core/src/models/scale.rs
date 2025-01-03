/// Represents scale used in SIB fields.
///
/// # Notest
/// Fits in 8-bit integer.
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Scale {
    Scale1,
    Scale2,
    Scale4,
    Scale8,
}

impl Scale {
    const _CHECK: () = {
        assert!(size_of::<Self>() == 1);
    };

    #[must_use]
    #[inline(always)]
    pub(crate) const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn factor(self) -> u8 {
        match self {
            Scale::Scale1 => 1,
            Scale::Scale2 => 2,
            Scale::Scale4 => 4,
            Scale::Scale8 => 8,
        }
    }
}
