/// Represents scale used in SIB fields.
///
/// # Notes
/// Fits in 8-bit integer.
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Scale {
    Scale1 = 0,
    Scale2 = 1,
    Scale4 = 2,
    Scale8 = 3,
}

impl Scale {
    const _CHECK: () = const {
        assert!(size_of::<Self>() == 1);
    };

    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
