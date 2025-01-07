#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum EncodingError {
    ArgumentOutOfRange,
    RegistersSizeMismatch,
}
