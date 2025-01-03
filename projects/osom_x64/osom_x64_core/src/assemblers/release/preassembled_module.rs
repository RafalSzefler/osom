use std::{collections::HashMap, fmt::Write};

use crate::models::PreassembledModule;

use super::{
    fragment::{Fragment, FragmentIterator},
    AssembleError,
};

pub struct X64ReleasePreassembledModule {
    symbols: HashMap<String, u32>,
    buffer: Vec<u8>,
}

impl X64ReleasePreassembledModule {
    pub fn new(symbols: HashMap<String, u32>, buffer: Vec<u8>) -> Self {
        Self { symbols, buffer }
    }

    #[inline(always)]
    fn iter_fragments(&self) -> impl Iterator<Item = &Fragment> {
        FragmentIterator::new(&self.buffer)
    }
}

impl PreassembledModule for X64ReleasePreassembledModule {
    type AssembleError = AssembleError;

    #[inline(always)]
    fn symbol_position<T: AsRef<str>>(&self, name: &T) -> Option<usize> {
        self.symbols.get(name.as_ref()).map(|x| *x as usize)
    }

    #[inline(always)]
    fn symbols(&self) -> impl Iterator<Item = (&impl AsRef<str>, usize)> {
        self.symbols.iter().map(|x| (x.0, *x.1 as usize))
    }

    #[inline(always)]
    fn content_length(&self) -> usize {
        self.iter_fragments().map(|f| f.buffer_length).sum::<u32>() as usize
    }

    fn assemble<T: Write>(&self, output: &mut T) -> Result<usize, Self::AssembleError> {
        todo!()
    }
}
