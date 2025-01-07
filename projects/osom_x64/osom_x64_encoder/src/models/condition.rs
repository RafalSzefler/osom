/// Represents conditions used in `CMOVcc` and `Jcc` instructions.
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Condition {
    /// In signed sense.
    Above,

    /// In signed sense.
    AboveOrEqual,

    /// In signed sense.
    Below,

    /// In signed sense.
    BelowOrEqual,

    Carry,
    NotCarry,
    Equal,
    NotEqual,

    /// In unsigned sense.
    Greater,

    /// In unsigned sense.
    GreaterOrEqual,

    /// In unsigned sense.
    Less,

    /// In unsigned sense.
    LessOrEqual,
    Overflow,
    NotOverflow,
    Parity,
    NotParity,
    Sign,
    NotSign,
    Zero,
    NotZero,
}

impl Condition {
    const _CHECK: () = const {
        assert!(size_of::<Self>() == 1);
    };
}
