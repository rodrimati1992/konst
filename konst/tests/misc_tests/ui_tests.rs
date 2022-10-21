#[cfg(feature = "__ui")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in [
        "iter_eval_ui_tests",
        "iter_mod_ui_tests",
        "collect_const_ui_tests",
        "parser_ui_tests",
    ] {
        t.compile_fail(format!("tests/misc_tests/{}/*err.rs", dir));
        t.pass(format!("tests/misc_tests/{}/*fine.rs", dir));
    }
}
