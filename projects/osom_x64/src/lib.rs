#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

use osom_macros::reexport_crate;

reexport_crate!(osom_x64_core);
reexport_crate!(osom_x64_macros);
reexport_crate!(osom_x64_cg, "osom_x64_cg");
