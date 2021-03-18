//! Compile-time comparison, parsing, and const equivalents of std methods.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Many functions for comparing standard library types.
//!
//! - The [`const_eq`]/[`const_eq_for`]/[`const_cmp`]/[`const_cmp_for`] macros
//! for comparisons, powered by the [`polymorphism`] module.
//!
//! - Compile-time parsing through the [`Parser`] type, and [`parse_any`] macro.
//!
//! # Examples
//!
//! ### Parsing an enum
//!
//! This example demonstrates how you can parse a simple enum from an environment variable,
//! at compile-time.
//!
//! ```rust
//! use konst::eq_str;
//! use konst::{unwrap_opt_or, unwrap_res};
//!
//! #[derive(Debug, PartialEq)]
//! enum Direction {
//!     Forward,
//!     Backward,
//!     Left,
//!     Right,
//! }
//!
//! impl Direction {
//!     const fn try_parse(input: &str) -> Result<Self, ParseDirectionError> {
//!         // As of Rust 1.51.0, string patterns don't work in const contexts
//!         match () {
//!             _ if eq_str(input, "forward") => Ok(Direction::Forward),
//!             _ if eq_str(input, "backward") => Ok(Direction::Backward),
//!             _ if eq_str(input, "left") => Ok(Direction::Left),
//!             _ if eq_str(input, "right") => Ok(Direction::Right),
//!             _ => Err(ParseDirectionError),
//!         }
//!     }
//! }
//!
//! const CHOICE: &str = unwrap_opt_or!(option_env!("chosen-direction"), "forward");
//!
//! const DIRECTION: Direction = unwrap_res!(Direction::try_parse(CHOICE));
//!
//! match DIRECTION {
//!     Direction::Forward => assert_eq!(CHOICE, "forward"),
//!     Direction::Backward => assert_eq!(CHOICE, "backward"),
//!     Direction::Left => assert_eq!(CHOICE, "left"),
//!     Direction::Right => assert_eq!(CHOICE, "right"),
//! }
//!
//! # #[derive(Debug, PartialEq)]
//! # pub struct ParseDirectionError;
//! #
//! # use std::fmt::{self, Display};
//! #
//! # impl Display for Direction {
//! #   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #       f.write_str("Failed to parse a Direction")
//! #   }
//! # }
//! #
//!
//! ```
//!
//! # No-std support
//!
//! `konst` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `konst` requires Rust 1.46.0, because it uses looping an branching in const contexts.
//!
//! Features that require newer versions of Rust, or the nightly compiler,
//! need to be explicitly enabled with cargo features.
//!
//!
//!
//! [`polymorphism`]: ./polymorphism/index.html
//! [`Parser`]: ./parsing/struct.Parser.html
//! [`parse_any`]: macro.parse_any.html
//!

#![cfg_attr(
    feature = "constant_time_slice",
    feature(const_slice_from_raw_parts, const_fn_union,)
)]
#![no_std]

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod __for_cmp_impls;

// pub mod other;

pub mod polymorphism;

#[cfg(feature = "primitives")]
pub mod primitive;

#[cfg(feature = "range")]
pub mod range;

#[cfg(feature = "nonzero")]
pub mod nonzero;

#[cfg(feature = "other")]
pub mod other;

#[cfg(feature = "parsing")]
pub mod parsing;

mod utils;

#[cfg(feature = "parsing")]
pub use crate::parsing::Parser;

#[cfg(feature = "parsing")]
pub use konst_proc_macros::{__priv_bstr_end, __priv_bstr_start};

#[cfg(feature = "str")]
__declare_string_cmp_fns! {
    import_path = "konst",
    equality_fn = eq_str,
    ordering_fn = cmp_str,
    ordering_fn_inner = cmp_str_inner,
}

#[cfg(all(feature = "str", feature = "option"))]
__declare_fns_with_docs! {
    (Option<&'a str>, (eq_option_str, cmp_option_str))

    docs(default)

    macro = __impl_option_cmp_fns!(
        for['a,]

        params(l, r)
        eq_comparison = crate::polymorphism::CmpWrapper(l).const_eq(r),
        cmp_comparison = crate::polymorphism::CmpWrapper(l).const_cmp(r),
        parameter_copyability = copy,
    ),
}

#[cfg(feature = "slice")]
pub mod slice;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        matches,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };

    pub mod v {
        pub use core::{
            option::Option::Some,
            result::Result::{Err, Ok},
        };
    }

    pub use crate::__for_cmp_impls::U8Ordering;

    pub use crate::polymorphism::{
        CmpWrapper, ConstCmpMarker, IsAConstCmpMarker, IsNotStdKind, IsStdKind,
    };
}
