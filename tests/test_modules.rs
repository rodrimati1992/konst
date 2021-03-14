mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    mod range_and_nonzero_tests;

    #[cfg(feature = "primitives")]
    mod primitive_tests;
}
