//! Miscelaneous items used for emulating polymorphism without trait methods.
//!
//! # `typewit`
//!
//! This crate uses [`typewit`] for emulating trait based polymorphism in const fns,
//! because trait methods cannot be called in `const fn`s on stable(as of Rust 1.69.0)
//!

/// Markers used to classify types, used as `Kind` associated types.
pub mod kinds {
    #[doc(inline)]
    pub use konst_kernel::polymorphism::kinds::*;
}

#[doc(no_inline)]
pub use typewit::{
    self, simple_type_witness, type_fn, CallFn, HasTypeWitness, MakeTypeWitness, TypeEq, TypeFn,
    TypeWitnessTypeArg,
};

include! {"polymorphism/type_eq_projection_fn.rs"}
