#![warn(clippy::all, clippy::pedantic)]

#[macro_export]
macro_rules! reexport_crate {
    ( $crate_name: ident ) => {
        #[doc(hidden)]
        extern crate $crate_name;

        #[allow(unused_imports)]
        pub use $crate_name::*;
    };

    ( $crate_name: ident, $feature: literal ) => {
        #[cfg(feature=$feature)]
        #[doc(hidden)]
        extern crate $crate_name;

        #[cfg_attr(docsrs, doc(cfg(feature=$feature)))]
        #[cfg(feature=$feature)]
        #[allow(unused_imports)]
        pub use $crate_name::*;
    };
}

#[macro_export]
macro_rules! export_mod {
    ( $crate_name: ident ) => {
        #[doc(hidden)]
        mod $crate_name;

        #[allow(unused_imports)]
        pub use $crate_name::*;
    };

    ( $crate_name: ident, $feature: literal ) => {
        #[cfg(feature=$feature)]
        #[doc(hidden)]
        mod $crate_name;

        #[cfg_attr(docsrs, doc(cfg(feature=$feature)))]
        #[cfg(feature=$feature)]
        #[allow(unused_imports)]
        pub use $crate_name::*;
    };
}
