mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    mod slice_const_method_tests;

    #[cfg(feature = "cmp")]
    mod other_types_tests;

    #[cfg(feature = "parsing")]
    mod parser;

    #[cfg(feature = "cmp")]
    mod range_and_nonzero_tests;

    #[cfg(feature = "cmp")]
    mod primitive_tests;
}
