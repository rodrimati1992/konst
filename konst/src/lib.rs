//! Const equivalents of std functions and const parsing.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Const fn equivalents of standard library functions and methods.
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
//! use konst::{
//!     eq_str,
//!     option,
//!     result::unwrap_ctx,
//! };
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
//!         // As of Rust 1.65.0, string patterns don't work in const contexts
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
//! const CHOICE: &str = option::unwrap_or!(option_env!("chosen-direction"), "forward");
//!
//! const DIRECTION: Direction = unwrap_ctx!(Direction::try_parse(CHOICE));
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
//! #[derive(Debug, PartialEq)]
//! pub struct ParseDirectionError;
//!
//! use std::fmt::{self, Display};
//!
//! impl Display for ParseDirectionError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         f.write_str("Failed to parse a Direction")
//!     }
//! }
//!
//! impl ParseDirectionError {
//!     const fn panic(&self) -> ! {
//!         panic!("failed to parse a Direction")
//!     }
//! }
//!
//! ```
//!
//! ### Parsing CSV
//!
//! This example demonstrates how CSV can be parsed into integers.
//!
//! This example requires the `"parsing"` and `"iter"` features
//! (both are enabled by default).
//!
#![cfg_attr(all(feature = "parsing", feature = "iter"), doc = "```rust")]
#![cfg_attr(not(all(feature = "parsing", feature = "iter")), doc = "```ignore")]
//! use konst::{
//!     primitive::parse_u64,
//!     result::unwrap_ctx,
//!     iter, string,
//! };
//!
//! const CSV: &str = "3, 8, 13, 21, 34";
//!
//! static PARSED: [u64; 5] = iter::collect_const!(u64 =>
//!     string::split(CSV, ","),
//!         map(string::trim),
//!         map(|s| unwrap_ctx!(parse_u64(s))),
//! );
//!
//! assert_eq!(PARSED, [3, 8, 13, 21, 34]);
//!
//! ```
//!
//! ### Parsing a struct
//!
//! This example demonstrates how a key-value pair format can be parsed into a struct.
//!
//! This requires the `"parsing_proc"` feature (enabled by default).
//!
#![cfg_attr(feature = "parsing_proc", doc = "```rust")]
#![cfg_attr(not(feature = "parsing_proc"), doc = "```ignore")]
//! use konst::{
//!     parsing::{Parser, ParseValueResult},
//!     eq_str,
//!     for_range, parser_method, try_, unwrap_ctx,
//! };
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
//!     unwrap_ctx!(parse_struct(Parser::new(input))).0
//! };
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
//! pub const fn parse_struct(mut parser: Parser<'_>) -> ParseValueResult<'_, Struct<'_>> {
//!     let mut name = "<none>";
//!     let mut amount = 0;
//!     let mut repeating = Shape::Circle;
//!     let mut colors = [Color::Red; 4];
//!     
//!     parser = parser.trim_end();
//!     if !parser.is_empty() {
//!         loop {
//!             let mut prev_parser = parser.trim_start();
//!
//!             parser = try_!(parser.find_skip('='));
//!
//!             parser_method!{prev_parser, strip_prefix;
//!                 "name" => (name, parser) = try_!(parser.trim_start().split_keep('\n')),
//!                 "amount" => (amount, parser) = try_!(parser.trim_start().parse_usize()),
//!                 "repeating" => (repeating, parser) = try_!(parse_shape(parser.trim_start())),
//!                 "colors" => (colors, parser) = try_!(parse_colors(parser.trim_start())),
//!                 _ => {
//!                     let err = &"could not parse Struct field name";
//!                     return Err(prev_parser.into_other_error(err));
//!                 }
//!             }
//!
//!             if parser.is_empty() {
//!                 break
//!             }
//!             parser = try_!(parser.strip_prefix("\n"));
//!         }
//!     }
//!
//!     Ok((Struct{name, amount, repeating, colors}, parser))
//! }
//!
//! pub const fn parse_shape(mut parser: Parser<'_>) -> ParseValueResult<'_, Shape> {
//!     let shape = parser_method!{parser, strip_prefix;
//!         "circle" => Shape::Circle,
//!         "square" => Shape::Square,
//!         "line" => Shape::Line,
//!         _ => return Err(parser.into_other_error(&"could not parse Shape"))
//!     };
//!     Ok((shape, parser))
//! }
//!
//! pub const fn parse_colors<const LEN: usize>(
//!     mut parser: Parser<'_>,
//! ) -> ParseValueResult<'_, [Color; LEN]> {
//!     let mut colors = [Color::Red; LEN];
//!
//!     for_range!{i in 0..LEN =>
//!         (colors[i], parser) = try_!(parse_color(parser.trim_start()));
//!         
//!         match parser.strip_prefix(",") {
//!             Ok(next) => parser = next,
//!             Err(_) if i == LEN - 1 => {}
//!             Err(e) => return Err(e),
//!         }
//!     }
//!
//!     Ok((colors, parser))
//! }
//!
//! pub const fn parse_color(mut parser: Parser<'_>) -> ParseValueResult<'_, Color> {
//!     let color = parser_method!{parser, strip_prefix;
//!         "red" => Color::Red,
//!         "blue" => Color::Blue,
//!         "green" => Color::Green,
//!         _ => return Err(parser.into_other_error(&"could not parse Color"))
//!     };
//!     Ok((color, parser))
//! }
//!
//!
//!
//! ```
//!
//! # Cargo features
//!
//! These are the features of these crates:
//!
//! - `"iter"`(enabled by default):
//! Enables all iteration items, including macros/functions that take/return iterators,
//!
//! - `"cmp"`(enabled by default):
//! Enables all comparison functions and macros,
//! the string equality and ordering comparison functions don't require this feature.
//!
//! - `"parsing_proc"`(enabled by default):
//! Enables the `"parsing"` feature, compiles the `konst_proc_macros` dependency,
//! and enables the [`parser_method`] macro.
//! You can use this feature instead of `"parsing"` if the slightly longer
//! compile times aren't a problem.
//!
//! - `"parsing"`(enabled by default):
//! Enables the [`parsing`] module (for parsing from `&str` and `&[u8]`),
//! the `primitive::parse_*` functions, `try_rebind`, and `rebind_if_ok` macros.
//!
//! - `"alloc"`:
//! Enables items that use types from the [`alloc`] crate, including `Vec` and `String`.
//!
//! ### Rust release related
//!
//! None of thse features are enabled by default.
//!
//! - `"rust_latest_stable"`: enables the latest `"rust_1_*"` feature(there's currently none).
//! Only recommendable if you can update the Rust compiler every stable release.
//!
//! - `"mut_refs"`(disabled by default):
//! Enables const functions that take mutable references.
//! Use this whenever mutable references in const contexts are stabilized.
//! Also enables the `"rust_latest_stable"` feature.
//!
//! - `"nightly_mut_refs"`(disabled by default):
//! Enables the `"mut_refs"` feature. Requires Rust nightly.
//!
//! # No-std support
//!
//! `konst` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `konst` requires Rust 1.65.0.
//!
//! Features that require newer versions of Rust, or the nightly compiler,
//! need to be explicitly enabled with crate features.
//!
//!
//!
//! [`alloc`]: https://doc.rust-lang.org/alloc/
//! [`const_eq`]: ./macro.const_eq.html
//! [`const_eq_for`]: ./macro.const_eq_for.html
//! [`const_cmp`]: ./macro.const_cmp.html
//! [`const_cmp_for`]: ./macro.const_cmp_for.html
//! [`parsing`]: ./parsing/index.html
//! [`primitive`]: ./primitive/index.html
//! [`parser_method`]: macro.parser_method.html
//! [`Parser`]: ./parsing/struct.Parser.html
//! [`Parser::parse_u128`]: ./parsing/struct.Parser.html#method.parse_u128
//!
#![deny(missing_docs)]
#![deny(unused_results)]
// clippy's opinionated BS
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::init_numbered_fields)]
////////////
#![forbid(clippy::missing_const_for_fn)]
#![cfg_attr(
    feature = "nightly_mut_refs",
    feature(const_mut_refs, const_slice_from_raw_parts_mut)
)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

include! {"./root_module_macros.rs"}

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod __for_cmp_impls;

// pub mod other;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub mod alloc_type;

pub mod array;

pub mod chr;

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub mod iter;

/// const equivalents of `core::ffi` functions
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

#[allow(unused_imports)]
mod utils_1_56 {
    pub(crate) use konst_kernel::__priv_transmute;
}

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

#[doc(no_inline)]
pub use crate::result::unwrap_ctx;

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::string::{cmp_option_str, eq_option_str};

#[cfg(all(doctest, feature = "iter", feature = "parsing_proc"))]
#[doc = include_str!("../../README.md")]
pub struct ReadmeTest;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error, matches,
        ops::Range,
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

    #[cfg(feature = "cmp")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
    pub use crate::cmp::{CmpWrapper, ConstCmp, IsAConstCmp, IsNotStdKind, IsStdKind};

    pub use const_panic::concat_panic;
}
