#![warn(clippy::all, clippy::pedantic)]

use osom_x64_cg::{OsomX64CodeGenerator, OsomX64InstrCodeGenerator};
use proc_macro::TokenStream;

#[proc_macro]
pub fn osom_x64(input: TokenStream) -> TokenStream {
    let cg = OsomX64CodeGenerator::new(input.into());
    cg.transform().into()
}

#[proc_macro]
pub fn osom_x64_instr(input: TokenStream) -> TokenStream {
    let cg = OsomX64InstrCodeGenerator::new(input.into());
    cg.transform().into()
}
