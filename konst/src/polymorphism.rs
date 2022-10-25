//! Miscelaneous items used for emulating polymorphism without trait methods.

/// Markers used to classify types, used as `Kind` associated types.
pub mod kinds {
    #[doc(inline)]
    pub use konst_kernel::polymorphism::kinds::*;
}

include! {"polymorphism/type_eq_.rs"}
