use crate::models::Label;

pub enum EmitError {
    OutOfMemory,
}

pub enum SetLabelError {
    LabelAlreadyExists,
}

pub enum PreassembleError {
    LabelNotSet(Label),
}

pub enum AssembleError {
    LabelNotSet(Label),
    IO(std::io::Error),
}

impl From<std::io::Error> for AssembleError {
    #[inline(always)]
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
