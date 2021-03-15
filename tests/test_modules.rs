mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    #[cfg(feature = "other")]
    mod other_types_tests;

    #[cfg(any(feature = "range", feature = "nonzero"))]
    mod range_and_nonzero_tests;

    #[cfg(feature = "primitives")]
    mod primitive_tests;
}
