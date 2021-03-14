mod misc_tests {
    #[macro_use]
    mod test_macros;

    mod slice_tests;

    #[cfg(feature = "primitives")]
    mod primitive_tests;
}
