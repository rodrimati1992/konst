mod misc_tests {
    #[macro_use]
    mod test_macros;

    #[macro_use]
    mod test_utils;

    mod array_tests;

    mod iter_mod;

    mod manually_drop_tests;

    mod maybe_uninit_tests;

    mod slice_tests;

    mod slice_const_method_tests;

    mod string_tests;

    mod ptr_tests;

    #[cfg(feature = "cmp")]
    mod other_types_tests;

    // Parser doesn't use unsafe code
    #[cfg(not(miri))]
    #[cfg(feature = "parsing_no_proc")]
    mod parser;

    #[cfg(feature = "cmp")]
    mod range_and_nonzero_tests;

    #[cfg(feature = "cmp")]
    mod primitive_tests;

    mod ui_tests;
}
