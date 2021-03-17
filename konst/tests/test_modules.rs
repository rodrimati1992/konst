mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    #[cfg(feature = "slice")]
    mod slice_const_method_tests;

    #[cfg(feature = "other")]
    mod other_types_tests;

    #[cfg(feature = "parsing")]
    mod parser;

    #[cfg(any(feature = "range", feature = "nonzero"))]
    mod range_and_nonzero_tests;

    #[cfg(feature = "primitives")]
    mod primitive_tests;
}
