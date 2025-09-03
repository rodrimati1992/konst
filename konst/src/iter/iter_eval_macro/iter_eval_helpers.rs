#[doc(hidden)]
pub struct __StepByVars {
    pub left_to_skip: usize,
    pub step: usize,
}

impl __StepByVars {
    #[track_caller]
    #[inline]
    pub const fn new(step: usize) -> Self {
        assert!(step != 0, "`step_by` requires a non-zero step");

        Self {
            // assigned 1 because:
            // - the macro decrements before checking this field
            // - Iterator::step_by always returns the first item regardless of step
            left_to_skip: 1,
            step,
        }
    }
}
