#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod osom_x64_code_generator;
mod osom_x64_instr_code_generator;

pub use osom_x64_code_generator::*;
pub use osom_x64_instr_code_generator::*;
