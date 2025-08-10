//! Const equivalents of `core::ffi` functions
//!
//! # Removed in 0.4.0
//!
//! All items relating to [CStr](core::ffi::c_str::CStr) stabilized by Rust 1.72.0.
//!
//!
//! ### CStr constructors
//!
//! The return value of fallible CStr constructors can be unwrapped with
//! [`konst::result::unwrap`]:
//!
//! ```rust
//! use std::ffi::CStr;
//! use konst::result;
//!
//! const AA: &CStr = result::unwrap!(CStr::from_bytes_with_nul(b"hello\0"));
//! assert_eq!(AA.to_bytes(), "hello".as_bytes());
//!
//! const BB: &CStr = result::unwrap!(CStr::from_bytes_until_nul(b"foo\0bar"));
//! assert_eq!(BB.to_bytes(), "foo".as_bytes());
//!
//! ```
//!
//!
//!

mod cstr;
