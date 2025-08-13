//! Miscelaneous items used for emulating polymorphism without trait methods.
//!
//! # `typewit`
//!
//! This crate uses [`typewit`] for emulating trait based polymorphism in const fns,
//! because trait methods cannot be called in `const fn`s on stable(as of Rust 1.89.0)
//!

/// Markers used to classify types, used as `Kind` associated types.
pub mod kinds {

    /// Marker for user-defined types that can be converted into const iterators
    pub enum IsIntoIterKind {}

    /// Marker for const iterators
    pub enum IsIteratorKind {}

    /// Marker for references.
    pub enum IsRefKind {}

    /// Marker for non-standard library types.
    pub enum IsNotStdKind {}

    /// Marker for standard library types.
    pub enum IsStdKind {}
}

#[doc(no_inline)]
pub use typewit::{
    self, CallFn, HasTypeWitness, MakeTypeWitness, TypeEq, TypeFn, TypeWitnessTypeArg,
    simple_type_witness, type_fn,
};
