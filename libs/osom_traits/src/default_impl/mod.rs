#[cfg(any(feature = "std", feature = "alloc"))]
mod std_allocator;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use std_allocator::*;
