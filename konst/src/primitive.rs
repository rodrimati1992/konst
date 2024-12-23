//! `const fn` equivalents of primitive type methods.
//! 
//! # Removed in 0.4.0
//!
//! These items were removed in 0.4.0 because there is equivalent
//! way to write it in const:
//!
//! - `parse_i128`: [`i128::from_str_radix`]
//! - `parse_i16`: [`i16::from_str_radix`]
//! - `parse_i32`: [`i32::from_str_radix`]
//! - `parse_i64`: [`i64::from_str_radix`]
//! - `parse_i8`: [`i8::from_str_radix`]
//! - `parse_isize`: [`isize::from_str_radix`]
//! - `parse_u128`: [`u128::from_str_radix`]
//! - `parse_u16`: [`u16::from_str_radix`]
//! - `parse_u32`: [`u32::from_str_radix`]
//! - `parse_u64`: [`u64::from_str_radix`]
//! - `parse_u8`: [`u8::from_str_radix`]
//! - `parse_usize`: [`usize::from_str_radix`]

/// `const fn`s for comparing primitive types for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

#[cfg(feature = "parsing")]
mod parse;

#[cfg(feature = "parsing")]
pub use parse::*;
