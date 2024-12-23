//! This crate has a few items reexported by `konst`, and used by `const_panic`.
//!
//! None of the items in this crate are documented,
//! because they are documented in the 0.3 `konst` crate.
//!
//!
#![no_std]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![deny(unused_results)]
#![allow(clippy::type_complexity)]
#![forbid(clippy::missing_const_for_fn)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

pub mod chr;

pub mod type_eq;

#[doc(hidden)]
pub mod maybe_uninit;

pub mod slice;

pub mod string;

pub mod polymorphism;

#[doc(hidden)]
pub mod __unsafe_utils;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub mod __ {
    pub use typewit::{
        HasTypeWitness, Identity, MakeTypeWitness, TypeEq, TypeFn, TypeWitnessTypeArg,
    };

    pub use crate::{
        maybe_uninit::{array_assume_init, uninit_array},
        type_eq::make_project_fn::__make_projection_parse_generics,
    };

    pub use core::{
        assert, compile_error, concat,
        marker::PhantomData,
        mem::{forget, ManuallyDrop, MaybeUninit},
        ops::Range,
        option::Option::{self, None, Some},
        panic,
        primitive::{bool, str, u8, usize},
        result::Result::{self, Err, Ok},
        stringify,
    };
}
