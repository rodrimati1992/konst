// HELP:
//
// This is the command I use:
// clear;clear; env TRYBUILD=overwrite \
// cargo test --no-default-features \
// --features "alloc parsing_proc iter cmp rust_latest_stable __ui"
//
// You'll need to run it with cmp enabled and disabled to run all the tests.
//
// use `cargo test --features "__ui debug <other_features>" -- ui trybuild=filter_here`
// to only  run UI tests

#[cfg(feature = "__ui")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in [
        "array_macros2_ui_tests",
        "destructure_ui_tests",
        #[cfg(feature = "konst_proc_macros")]
        "destructure_rec_ui_tests",
        #[cfg(feature = "cmp")]
        "cmp_macros_ui_tests",
        "collect_const_ui_tests",
        "iter_eval_ui_tests",
        "iter_mod_ui_tests",
        "misc_nonmacro_ui_tests",
        "misc_macro_ui_tests",
        "parser_ui_tests",
        "string_ui_tests",
        "slice_ui_tests",
        #[cfg(feature = "cmp")]
        "slice_ui_cmp_tests",
        #[cfg(feature = "cmp")]
        "iter_eval_cmp_feature_ui_tests",
        #[cfg(not(feature = "cmp"))]
        "iter_eval_no_cmp_feature_ui_tests",
    ] {
        t.compile_fail(format!("tests/misc_tests/{}/*err.rs", dir));
        t.pass(format!("tests/misc_tests/{}/*fine.rs", dir));
    }
}
