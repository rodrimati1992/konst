mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    mod slice_const_method_tests;

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
}
