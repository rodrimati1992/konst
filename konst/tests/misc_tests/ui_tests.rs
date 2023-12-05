// HELP:
//
// use `cargo test --features "__ui debug" -- ui trybuild=filter_here`
// to only  run UI tests

#[cfg(feature = "__ui")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in [
        "collect_const_ui_tests",
        "iter_eval_ui_tests",
        "iter_mod_ui_tests",
        "parser_ui_tests",
        "string_ui_tests",
        "type_eq_ui_tests",
    ] {
        t.compile_fail(format!("tests/misc_tests/{}/*err.rs", dir));
        t.pass(format!("tests/misc_tests/{}/*fine.rs", dir));
    }
}
