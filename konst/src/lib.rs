//! Const equivalents of std functions and const parsing.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Const fn equivalents of standard library functions, methods, and operators.
//!
//! - [`destructure`]/[`if_let_Some`]/[`while_let_Some`]
//! macros to allow destructuring types in const without getting "cannot drop in const" errors.
//!
//! - Compile-time parsing through the [`Parser`] type, and [`parser_method`] macro.
//!
//! # Examples
//!
//! ### Parsing an enum
//!
//! This example demonstrates how you can parse a simple enum from an environment variable,
//! at compile-time.
//!
//! ```rust
//! use konst::{eq_str, option, result};
//! use konst::const_panic::{self, PanicFmt};
//!
//! use std::fmt::{self, Display};
//!
//!
//! const CHOICE: &str = option::unwrap_or!(option_env!("chosen-direction"), "forward");
//!
//! const DIRECTION: Direction = result::unwrap!(Direction::try_parse(CHOICE));
//!
//! fn main() {
//!     match DIRECTION {
//!         Direction::Forward => assert_eq!(CHOICE, "forward"),
//!         Direction::Backward => assert_eq!(CHOICE, "backward"),
//!         Direction::Left => assert_eq!(CHOICE, "left"),
//!         Direction::Right => assert_eq!(CHOICE, "right"),
//!     }
//! }
//!
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
//!         // As of Rust 1.89.0, string patterns don't work in const contexts
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
//! // `PanicFmt` derives the `PanicFmt` trait for debug-printing in `result::unwrap`.
//! // To use the `PanicFmt` derive you need to enable the "const_panic_derive" feature.
//! #[derive(Debug, PartialEq, PanicFmt)]
//! pub struct ParseDirectionError;
//!
//! impl Display for ParseDirectionError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         f.write_str("Failed to parse a Direction")
//!     }
//! }
//! ```
//!
//! ### Parsing CSV
//!
//! This example demonstrates how CSV can be parsed into integers.
//!
//! This example requires the `"iter"` feature (enabled by default).
//!
#![cfg_attr(feature = "iter", doc = "```rust")]
#![cfg_attr(not(feature = "iter"), doc = "```ignore")]
//! use konst::{iter, result, string};
//!
//!
//! assert_eq!(PARSED, [3, 8, 13, 21, 34]);
//!
//!
//! const CSV: &str = "3, 8, 13, 21, 34";
//!
//! static PARSED: [u64; 5] = iter::collect_const!(u64 =>
//!     string::split(CSV, ","),
//!         map(str::trim_ascii),
//!         map(|s| result::unwrap!(u64::from_str_radix(s, 10))),
//! );
//! ```
//!
//! ### Parsing a struct
//!
//! This example demonstrates how a key-value pair format can be parsed into a struct.
//!
//! This requires the `"iter"` and `"parsing_proc"` features (enabled by default).
//!
#![cfg_attr(all(feature = "iter", feature = "parsing_proc"), doc = "```rust")]
#![cfg_attr(
    not(all(feature = "iter", feature = "parsing_proc")),
    doc = "```ignore"
)]
//! use konst::{result, try_};
//! use konst::parsing::{Parser, ParseError, parser_method};
//!
//!
//! fn main(){
//!     assert_eq!(
//!         PARSED,
//!         Struct{
//!             name: "bob smith",
//!             amount: 1000,
//!             repeating: Shape::Circle,
//!             colors: [Color::Red, Color::Blue, Color::Green, Color::Blue],
//!         }
//!     );
//! }
//!
//!
//! const PARSED: Struct = {
//!     // You can also parse strings from environment variables, or from an `include_str!(....)`
//!     let input = "\
//!         colors = red, blue, green, blue
//!         amount = 1000
//!         repeating = circle
//!         name = bob smith
//!     ";
//!     
//!     result::unwrap!(parse_struct(&mut Parser::new(input)))
//! };
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub struct Struct<'a> {
//!     pub name: &'a str,
//!     pub amount: usize,
//!     pub repeating: Shape,
//!     pub colors: [Color; 4],
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub enum Shape {
//!     Circle,
//!     Square,
//!     Line,
//! }
//!
//! #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! pub enum Color {
//!     Red,
//!     Blue,
//!     Green,
//! }
//!
//! pub const fn parse_struct<'p>(parser: &mut Parser<'p>) -> Result<Struct<'p>, ParseError<'p>> {
//!     let mut name = "<none>";
//!     let mut amount = 0;
//!     let mut repeating = Shape::Circle;
//!     let mut colors = [Color::Red; 4];
//!     
//!     parser.trim_end();
//!     if !parser.is_empty() {
//!         loop {
//!             // keeps a copy of the parser at this position to parse the field name
//!             let mut prev_parser = parser.trim_start().copy();
//!
//!             // skip past the `=` to parse the field value
//!             try_!(parser.find_skip('=')).trim_start();
//!
//!             parser_method!{prev_parser, strip_prefix;
//!                 "name" => name = try_!(parser.split_keep('\n')),
//!                 "amount" => amount = try_!(parser.parse_usize()),
//!                 "repeating" => repeating = try_!(parse_shape(parser)),
//!                 "colors" => colors = try_!(parse_colors(parser)),
//!                 _ => {
//!                     let err = &"could not parse Struct field name";
//!                     return Err(prev_parser.to_other_error(err));
//!                 }
//!             }
//!
//!             if parser.is_empty() {
//!                 break
//!             }
//!             try_!(parser.strip_prefix("\n"));
//!         }
//!     }
//!
//!     Ok(Struct{name, amount, repeating, colors})
//! }
//!
//! pub const fn parse_shape<'p>(parser: &mut Parser<'p>) -> Result<Shape, ParseError<'p>> {
//!     let shape = parser_method!{parser, strip_prefix;
//!         "circle" => Shape::Circle,
//!         "square" => Shape::Square,
//!         "line" => Shape::Line,
//!         _ => return Err(parser.to_other_error(&"could not parse Shape"))
//!     };
//!     Ok(shape)
//! }
//!
//! pub const fn parse_colors<'p, const LEN: usize>(
//!     parser: &mut Parser<'p>,
//! ) -> Result<[Color; LEN], ParseError<'p>> {
//!     let mut colors = konst::array::ArrayBuilder::of_copy();
//!
//!     while !colors.is_full() {
//!         colors.push(try_!(parse_color(parser.trim_start())));
//!         
//!         // returns an error if there aren't enough comma-separated colors
//!         if let Err(e) = parser.strip_prefix(",") && !colors.is_full() {
//!             return Err(e);
//!         }
//!     }
//!
//!     Ok(colors.build())
//! }
//!
//! pub const fn parse_color<'p>(parser: &mut Parser<'p>) -> Result<Color, ParseError<'p>> {
//!     let color = parser_method!{parser, strip_prefix;
//!         "red" => Color::Red,
//!         "blue" => Color::Blue,
//!         "green" => Color::Green,
//!         _ => return Err(parser.to_other_error(&"could not parse Color"))
//!     };
//!     Ok(color)
//! }
//!
//! ```
//!
//! # Cargo features
//!
//! These are the features of this crate:
//!
//! - `"iter"`(enabled by default):
//! Enables all iteration-related items that take/return iterators,
//!
//! - `"cmp"`(enabled by default):
//! Enables all comparison-related items,
//! the string equality and ordering comparison functions don't require this feature.
//!
//! - `"parsing_proc"`(enabled by default):
//! Enables the `"parsing"` feature, compiles the `konst_proc_macros` dependency,
//! and enables the [`parser_method`] macro.
//! You can use this feature instead of `"parsing"` if the slightly longer
//! compile times aren't a problem.
//!
//! - `"parsing"`(enabled by default):
//! Enables the [`parsing`] module for parsing from strings,
//! and the `primitive::parse_*` functions.
//!
//! - `"const_panic_derive"`(disabled by default):
//! Enables the "derive" feature of the `const_panic` public dependency.
//!
//! - `"alloc"`(disabled by default):
//! Enables items that use types from the [`alloc`] crate.
//!
//! ### Rust release related
//!
//! None of thse features are enabled by default.
//!
//! - `"rust_latest_stable"`: enables the latest `"rust_1_*"` feature.
//! Only recommendable if you can update the Rust compiler every stable release.
//!
//! # No-std support
//!
//! `konst` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `konst` requires Rust 1.89.0.
//!
//! Features that require newer versions of Rust, or the nightly compiler,
//! need to be explicitly enabled with crate features.
//!
//!
//!
//! [`alloc`]: https://doc.rust-lang.org/alloc/
//! [`const_eq`]: crate::cmp::const_eq
//! [`const_eq_for`]: crate::cmp::const_eq_for
//! [`const_cmp`]: crate::cmp::const_cmp
//! [`const_cmp_for`]: crate::cmp::const_cmp_for
//! [`parsing`]: crate::parsing
//! [`primitive`]: crate::primitive
//! [`parser_method`]: crate::parsing::parser_method
//! [`Parser`]: crate::parsing::Parser
//! [`Parser::parse_u128`]: crate::parsing::Parser#method.parse_u128
//! [`destructure`]: crate::destructure
//! [`if_let_Some`]: crate::if_let_Some
//! [`while_let_Some`]: crate::while_let_Some
//!
#![deny(missing_docs)]
#![deny(unused_results)]
#![allow(rustdoc::redundant_explicit_links)]
// clippy's opinionated BS
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::init_numbered_fields)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::unused_unit)]
////////////
#![forbid(clippy::missing_const_for_fn)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
#[doc(hidden)]
pub mod macros;

mod docs;

#[doc(hidden)]
pub mod __for_cmp_impls;

#[doc(hidden)]
pub mod __utils;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub mod alloc_type;

pub mod array;

pub mod chr;

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

#[cfg(feature = "iter")]
#[macro_use]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub mod iter;

pub mod drop_flavor;

pub mod ffi;

pub mod polymorphism;

pub mod primitive;

pub mod option;

pub mod result;

pub mod range;

pub mod maybe_uninit;

pub mod manually_drop;

pub mod nonzero;

pub mod other;

#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub mod parsing;

pub mod ptr;

mod utils;

#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub use crate::parsing::Parser;

#[cfg(feature = "parsing_proc")]
#[doc(hidden)]
pub use konst_proc_macros::{__priv_bstr_end, __priv_bstr_start};

pub mod slice;

pub mod string;

pub use ::const_panic;

pub use crate::string::{cmp_str, eq_str};

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::string::{cmp_option_str, eq_option_str};

#[cfg(all(doctest, feature = "iter", feature = "parsing_proc"))]
#[doc = include_str!("../../README.md")]
pub struct ReadmeTest;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        assert,
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error, concat,
        marker::PhantomData,
        matches,
        mem::{self, ManuallyDrop, MaybeUninit, replace},
        ops::Range,
        option::Option::{self, None, Some},
        panic,
        primitive::{bool, str, u8, usize},
        ptr,
        result::Result::{self, Err, Ok},
        stringify,
    };

    pub const fn forget<T>(x: T) {
        core::mem::forget(x)
    }

    pub mod v {
        pub use core::{
            option::Option::Some,
            result::Result::{Err, Ok},
        };
    }

    pub use crate::__for_cmp_impls::U8Ordering;

    #[cfg(feature = "cmp")]
    pub use crate::cmp::{CmpWrapper, ConstCmp, IsAConstCmp, IsNotStdKind, IsStdKind};

    #[cfg(feature = "iter")]
    pub use crate::iter::collect_const::CollectorCmd;

    pub use crate::array::__array_macros_2::unit_array;

    pub use const_panic::concat_panic;

    pub use typewit::MakeTypeWitness;
}
