use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FileSpan {
    pub file: &'static str,
    pub line: u32,
}

pub type ThreadError = Box<dyn Any + Send + 'static>;

#[derive(Debug, Clone)]
pub struct ShouldHavePanickedAt {
    pub span: FileSpan,
}

#[track_caller]
pub fn assert_type<T, Expected>(_: &T) {
    assert_eq!(
        std::any::type_name::<T>(),
        std::any::type_name::<Expected>()
    );
}

#[macro_export]
macro_rules! file_span {
    () => {{
        use $crate::misc_tests::test_utils::FileSpan;
        FileSpan {
            file: file!(),
            line: line!(),
        }
    }};
}

pub fn must_panic<F, R>(span: FileSpan, f: F) -> Result<ThreadError, ShouldHavePanickedAt>
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => Err(ShouldHavePanickedAt { span }),
        Err(e) => Ok(e),
    }
}

#[test]
fn test_must_panic() {
    assert!(must_panic(file_span!(), || panic!()).is_ok());
    assert!(must_panic(file_span!(), || ()).is_err());
}
