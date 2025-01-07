#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(not(feature = "std"), no_std)]

mod allocator;
pub use allocator::*;

#[cfg(feature = "alloc")]
mod std_allocator;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
pub use std_allocator::*;
