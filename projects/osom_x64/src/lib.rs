#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[doc(hidden)]
extern crate osom_x64_core;
pub use osom_x64_core::*;

#[doc(hidden)]
extern crate osom_x64_macros;
pub use osom_x64_macros::*;

#[cfg(feature = "osom_x64_cg")]
#[doc(hidden)]
extern crate osom_x64_cg;

#[cfg_attr(docsrs, doc(cfg(feature = "osom_x64_cg")))]
#[cfg(feature = "osom_x64_cg")]
#[allow(unused_imports)]
pub use osom_x64_cg::*;
