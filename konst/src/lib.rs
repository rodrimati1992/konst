//! Compile-time comparison, parsing, and const equivalents of std methods.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Compile-time parsing through the [`Parser`] type, and [`parse_any`] macro.
//!
//! - Functions for comparing many standard library types,
//! with the [`const_eq`]/[`const_eq_for`]/[`const_cmp`]/[`const_cmp_for`] macros
//! for more conveniently calling them, powered by the [`polymorphism`] module.
//!
//! - Const fn equivalents of other standard library functions and methods.
//!
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
//! use konst::{unwrap_opt_or, unwrap_ctx};
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
//! # #[derive(Debug, PartialEq)]
//! # pub struct ParseDirectionError;
//! #
//! # use std::fmt::{self, Display};
//! #
//! # impl Display for ParseDirectionError {
//! #   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #       f.write_str("Failed to parse a Direction")
//! #   }
//! # }
//! #
//! # impl ParseDirectionError {
//! #    const fn panic(&self) -> ! {
//! #        [/*failed to parse a Direction*/][0]
//! #    }
//! # }
//! #
//! #
//!
//! ```
//!
//! ### Parsing a struct
//!
//! This example demonstrates how you can use [`Parser`] to parse a struct at compile-time.
//!
#![cfg_attr(feature = "parsing", doc = "```rust")]
#![cfg_attr(not(feature = "parsing"), doc = "```ignore")]
//! use konst::{
//!     parsing::{Parser, ParseValueResult},
//!     for_range, parse_any, try_rebind, unwrap_ctx,
//! };
//!
//! const PARSED: Struct = {
//!     // You can also parse strings from environment variables, or from an `include_str!(....)`
//!     let input = "\
//!         1000,
//!         circle,
//!         red, blue, green, blue,
//!     ";
//!     
//!     unwrap_ctx!(parse_struct(Parser::from_str(input))).0
//! };
//!
//! fn main(){
//!     assert_eq!(
//!         PARSED,
//!         Struct{
//!             amount: 1000,
//!             repeating: Shape::Circle,
//!             colors: [Color::Red, Color::Blue, Color::Green, Color::Blue],
//!         }
//!     );
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub struct Struct {
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
//! pub const fn parse_struct(mut parser: Parser<'_>) -> ParseValueResult<'_, Struct> {
//!     try_rebind!{(let amount, parser) = parser.trim_start().parse_usize()}
//!     try_rebind!{parser = parser.strip_prefix(",")}
//!
//!     try_rebind!{(let repeating, parser) = parse_shape(parser.trim_start())}
//!     try_rebind!{parser = parser.strip_prefix(",")}
//!
//!     try_rebind!{(let colors, parser) = parse_colors(parser.trim_start())}
//!
//!     Ok((Struct{amount, repeating, colors}, parser))
//! }
//!
//! pub const fn parse_shape(mut parser: Parser<'_>) -> ParseValueResult<'_, Shape> {
//!     let shape = parse_any!{parser, strip_prefix;
//!         "circle" => Shape::Circle,
//!         "square" => Shape::Square,
//!         "line" => Shape::Line,
//!         _ => return Err(parser.into_other_error())
//!     };
//!     Ok((shape, parser))
//! }
//!
//! pub const fn parse_colors(mut parser: Parser<'_>) -> ParseValueResult<'_, [Color; 4]> {
//!     let mut colors = [Color::Red; 4];
//!
//!     for_range!{i in 0..4 =>
//!         try_rebind!{(colors[i], parser) = parse_color(parser.trim_start())}
//!         try_rebind!{parser = parser.strip_prefix(",")}
//!     }
//!
//!     Ok((colors, parser))
//! }
//!
//! pub const fn parse_color(mut parser: Parser<'_>) -> ParseValueResult<'_, Color> {
//!     let color = parse_any!{parser, strip_prefix;
//!         "red" => Color::Red,
//!         "blue" => Color::Blue,
//!         "green" => Color::Green,
//!         _ => return Err(parser.into_other_error())
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
//! - `"cmp"`(enabled by default):
//! Enables all comparison functions and macros,
//! the string equality and ordering comparison functions don't require this feature.
//!
//! - `"parser"`(enabled by default):
//! Enables the [`parsing`] module, for parsing from `&str` and `&[u8]`.
//!
//! - `"constant_time_slice"`(disabled by default):<br>
//! Improves the performance of slice functions that split slices,
//! from taking linear time to taking constant time,
//! this requires using some nightly Rust features.
//! <br>Note that only functions which mention this feature in their documentation are affected.
//!
//! - `"const_generics"` (disabled by default):
//! Changes impls for arrays to use const generics instead of only supporting small arrays.
//! This feature requires Rust 1.51.0.
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
//! [`const_eq`]: ./macro.const_eq.html
//! [`const_eq_for`]: ./macro.const_eq_for.html
//! [`const_cmp`]: ./macro.const_cmp.html
//! [`const_cmp_for`]: ./macro.const_cmp_for.html
//! [`polymorphism`]: ./polymorphism/index.html
//! [`parsing`]: ./parsing/index.html
//! [`Parser`]: ./parsing/struct.Parser.html
//! [`parse_any`]: macro.parse_any.html
//!

#![cfg_attr(
    feature = "constant_time_slice",
    feature(const_slice_from_raw_parts, const_fn_union, const_ptr_offset)
)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![no_std]

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod __for_cmp_impls;

// pub mod other;

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod polymorphism;

pub mod primitive;

pub mod range;

pub mod nonzero;

pub mod other;

#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub mod parsing;

mod utils;

#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub use crate::parsing::Parser;

#[cfg(feature = "parsing")]
#[doc(hidden)]
pub use konst_proc_macros::{__priv_bstr_end, __priv_bstr_start};

pub mod slice;

pub mod string;

pub use crate::string::{cmp_str, eq_str};

#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::string::{cmp_option_str, eq_option_str};

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
    pub use crate::polymorphism::{
        CmpWrapper, ConstCmpMarker, IsAConstCmpMarker, IsNotStdKind, IsStdKind,
    };
}
