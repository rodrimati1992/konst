#![deny(unused_mut)]

mod misc_tests {
    #[macro_use]
    mod test_macros;

    #[macro_use]
    mod test_utils;

    mod array_tests;

    #[cfg(feature = "iter")]
    mod iter_mod;

    mod ffi_tests;

    mod manually_drop_tests;

    mod maybe_uninit_tests;

    mod macro_tests;
    
    mod option_tests;

    mod slice_tests;

    mod slice_const_method_tests;

    mod string_tests;

    mod ptr_tests;

    #[cfg(feature = "cmp")]
    mod other_types_tests;

    // Parser doesn't use unsafe code
    #[cfg(not(miri))]
    #[cfg(feature = "parsing")]
    mod parser;

    #[cfg(feature = "cmp")]
    mod range_and_nonzero_tests;

    #[cfg(feature = "cmp")]
    mod minmax_tests;

    #[cfg(feature = "cmp")]
    mod primitive_tests;

    mod ui_tests;
}
