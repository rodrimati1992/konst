#[macro_use]
mod control_flow;

#[cfg(feature = "iter")]
#[macro_use]
mod into_iter_macros;

#[macro_use]
mod internal_macros;

pub(crate) mod array_macros;

mod option_macros_;

mod result_macros_;
