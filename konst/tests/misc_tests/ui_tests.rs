#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in ["iter_eval_ui_tests"] {
        t.compile_fail(format!("tests/misc_tests/{}/*err.rs", dir));
        t.pass(format!("tests/misc_tests/{}/*fine.rs", dir));
    }
}
