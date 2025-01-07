/// Represents various machine sizes.
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
    const _CHECK: () = const {
        assert!(size_of::<Self>() == 1);
    };

    #[must_use]
    #[inline(always)]
    pub const fn as_bytes(self) -> u8 {
        let val = self as u8;
        (1 << val) >> 1
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::MachineSize;

    #[rstest]
    #[case(MachineSize::None, 0)]
    #[case(MachineSize::Byte, 1)]
    #[case(MachineSize::Word, 2)]
    #[case(MachineSize::DWord, 4)]
    #[case(MachineSize::QWord, 8)]
    #[case(MachineSize::XMMWord, 16)]
    #[case(MachineSize::YMMWord, 32)]
    fn test_machine_size_value(#[case] size: MachineSize, #[case] expected: u8) {
        assert_eq!(size.as_bytes(), expected);
    }
}
