use std::fmt::Write;

use super::{Instruction, Label};

/// Represents preassembled module, which is the final variant of the assembled module,
/// with the additional metadata.
pub trait PreassembledModule {
    type AssembleError;

    /// Retrevies position of given symbol. Return [`None`] if symbol is not present.
    fn symbol_position<T: AsRef<str>>(&self, name: &T) -> Option<usize>;

    /// Return iterator over all symbols.
    fn symbols(&self) -> impl Iterator<Item = (&impl AsRef<str>, usize)>;

    /// Returns the size of the underlying buffer. When calling [`PreassembledModule::assemble()`],
    /// passed `output` must be able to handle that many bytes.
    fn content_length(&self) -> usize;

    /// Writes the underlying buffer into the output. Returns the number of bytes written.
    ///
    /// # Errors
    /// For concrete errors see [`Self::AssembleError`].
    fn assemble<T: Write>(&self, output: &mut T) -> Result<usize, Self::AssembleError>;
}

/// Represents main just-in-time compiler API.
pub trait Assembler {
    type EmitError;
    type SetLabelError;
    type PreassembleError;
    type PreassembledModule: PreassembledModule;

    /// Creates a new label associated with the passed string.
    fn get_or_create_label<T: AsRef<str>>(&mut self, name: &T) -> Label;

    /// Adds instruction to the [`Assembler`].
    ///
    /// # Errors
    /// For concrete errors see [`Self::EmitError`].
    fn add_instruction(&mut self, instr: &Instruction)
        -> Result<usize, Self::EmitError>;

    /// Adds an arbitrary sequence of bytes to the [`Assembler`].
    ///
    /// # Errors
    /// For concrete errors see [`Self::EmitError`].
    fn add_bytes<T: AsRef<[u8]>>(
        &mut self,
        buffer: &T,
    ) -> Result<usize, Self::EmitError>;

    /// Sets `label` at the current position.
    ///
    /// # Errors
    /// For concrete errors see [`Self::SetLabelError`].
    fn set_label(&mut self, label: &Label) -> Result<(), Self::SetLabelError>;

    /// Creates a [`PreassembledModule`] and disposes current [`Assembler`]. No
    /// updates are possible after this point.
    fn preassemble(self) -> Result<Self::PreassembledModule, Self::PreassembleError>;
}
