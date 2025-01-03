/// Represents different variants of machine sizes.
///
/// # Notes
/// Fits in 8-bit integer.
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MachineSize {
    None = 0,
    Byte = 1,
    Word = 2,
    DWord = 3,
    QWord = 4,
    XMMWord = 5,
    YMMWord = 6,
}

impl MachineSize {
    const _CHECK: () = {
        assert!(size_of::<Self>() == 1);
    };

    #[inline(always)]
    pub(crate) const fn as_u8(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub(crate) const unsafe fn from_u8(val: u8) -> Self {
        core::mem::transmute(val)
    }
}
