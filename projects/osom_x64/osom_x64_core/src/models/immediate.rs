#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::cast_sign_loss
)]

pub trait Immediate: Into<u64> {
    fn size() -> u8;
    fn as_u64(&self) -> u64;
}

/// Represents a 32-bit integer to be used as
/// constants in x64 assembly. A thin wrapper around
/// `i32` for type-safety reasons.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Immediate32 {
    internal_value: i32,
}

impl Immediate32 {
    const _CHECK: () = {
        assert!(size_of::<Self>() == 4);
    };

    pub const ZERO: Self = Self::new(0);

    /// Creates a new instance of [`Immediate32`].
    #[must_use]
    #[inline(always)]
    pub const fn new(val: i32) -> Self {
        Self {
            internal_value: val,
        }
    }
}

impl From<Immediate32> for i32 {
    #[inline(always)]
    fn from(value: Immediate32) -> Self {
        value.internal_value
    }
}

impl From<i32> for Immediate32 {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<u32> for Immediate32 {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::new(value as i32)
    }
}

impl From<Immediate32> for u64 {
    #[inline(always)]
    fn from(value: Immediate32) -> Self {
        value.internal_value as u64
    }
}

impl Immediate for Immediate32 {
    #[inline(always)]
    fn size() -> u8 {
        4
    }

    #[inline(always)]
    fn as_u64(&self) -> u64 {
        self.internal_value as u64
    }
}

/// Represents a 64-bit integer to be used as
/// constants in x64 assembly. A thin wrapper around
/// `i64` for type-safety reasons.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Immediate64 {
    internal_value: i64,
}

impl Immediate64 {
    const _CHECK: () = {
        assert!(size_of::<Self>() == 8);
    };

    pub const ZERO: Self = Self::new(0);

    /// Creates a new instance of [`Immediate64`].
    #[must_use]
    #[inline(always)]
    pub const fn new(val: i64) -> Self {
        Self {
            internal_value: val,
        }
    }
}

impl From<Immediate64> for i64 {
    #[inline(always)]
    fn from(value: Immediate64) -> Self {
        value.internal_value
    }
}

impl From<i64> for Immediate64 {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<u64> for Immediate64 {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self::new(value as i64)
    }
}

impl From<Immediate32> for Immediate64 {
    #[inline(always)]
    fn from(value: Immediate32) -> Self {
        let no: i32 = value.into();
        (no as i64).into()
    }
}

impl From<Immediate64> for u64 {
    #[inline(always)]
    fn from(value: Immediate64) -> Self {
        value.internal_value as u64
    }
}

impl Immediate for Immediate64 {
    #[inline(always)]
    fn size() -> u8 {
        8
    }

    #[inline(always)]
    fn as_u64(&self) -> u64 {
        self.internal_value as u64
    }
}
