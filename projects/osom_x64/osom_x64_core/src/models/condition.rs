/// Represents conditions used in `CMOVcc` and `Jcc` instructions.
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Condition {
    /// In unsigned sense.
    Above,

    /// In unsigned sense.
    AboveOrEqual,

    /// In unsigned sense.
    Below,

    /// In unsigned sense.
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
    const _CHECK: () = {
        assert!(size_of::<Self>() == 1);
    };
}
