#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod allocator;
pub mod default_impl;
