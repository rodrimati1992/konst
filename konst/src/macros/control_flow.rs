/// For loop over a range
///
/// # Example
///     
/// ```rust
/// use konst::for_range;
///
/// const LEN: usize = 10;
/// const ARR: [u32; LEN] = {
///     let mut ret = [1; LEN];
///     for_range!{i in 2..LEN =>
///         ret[i] = ret[i - 1] + ret[i - 2];
///     }
///     ret
/// };
///
/// assert_eq!(ARR, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
/// ```
///
#[macro_export]
macro_rules! for_range {
    ($pat:pat_param in $range:expr => $($code:tt)*) => {
        let $crate::__::Range{mut start, end} = $range;

        while start < end {
            let $pat = start;

            start+=1;

            $($code)*
        }
    };
}

/// Emulates by-value destructuring of a [`Some`] variant that contains a Drop type in const.
///
/// # Motivation
///
/// This macro works around the fact that this code
///
/// ```rust,compile_fail
/// const fn foo<T>(opt: Option<T>) -> Result<T, ()> {
///     match opt {
///         Some(x) => Ok(x),
///         None => Err(())
///     }
/// }
/// ```
/// causes this error as of Rust 1.89:
/// ```text
/// error[E0493]: destructor of `Option<T>` cannot be evaluated at compile-time
///  --> konst/src/macros/control_flow.rs:42:17
///   |
/// 3 | const fn foo<T>(opt: Option<T>) -> Result<T, ()> {
///   |                 ^^^ the destructor for this type cannot be evaluated in constant functions
/// ...
/// 8 | }
///   | - value is dropped here
/// ```
///
/// # Example
///
/// ```rust
/// assert_eq!(ok_or_none_error(Some(10)), Ok(10));
/// assert_eq!(ok_or_none_error(None::<String>), Err(ItWasNoneError));
///
///
/// const fn ok_or_none_error<T>(opt: Option<T>) -> Result<T, ItWasNoneError> {
///     konst::if_let_Some!{x = opt => {
///         Ok(x)
///     } else {
///         Err(ItWasNoneError)
///     }}
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct ItWasNoneError;
/// ```
///
#[macro_export]
macro_rules! if_let_Some {
    ($some:pat = $e:expr => $then:block $(else $else:block)?) => {
        match $crate::option::__opt($e) {opt =>
            if $crate::__::Option::is_some(&opt) {
                let $some = $crate::__::Option::unwrap(opt);
                $then
            } else {
                $crate::__::forget(opt);
                $($else)?
            }
        }
    }
}

/// Emulates a by-value `while let Some` loop over
/// `Drop`-type-containing `Option`s in const.
///
/// # Motivation
///
/// This macro works around the fact that this code
///
/// ```rust,compile_fail
/// use konst::array::ArrayBuilder;
///
/// const fn foo<T: SomeTrait>() -> [T; 3] {
///     let mut builder = ArrayBuilder::of_drop();
///     while let Some(x) = produce_option(&builder) {
///         builder.push(x);
///     }
///     builder.build()
/// }
///
/// # trait SomeTrait {}
/// # const fn produce_option<T: SomeTrait, U>(_: &U) -> Option<T> {
/// #   None
/// # }
/// ```
/// causes this error as of Rust 1.89:
/// ```text
/// error[E0493]: destructor of `Option<T>` cannot be evaluated at compile-time
///   --> konst/src/macros/control_flow.rs:108:25
///    |
/// 9  |     while let Some(x) = produce_option(&builder) {
///    |                         ^^^^^^^^^^^^^^^^^^^^^^^^ the destructor for this type cannot be evaluated in constant functions
/// 10 |         builder.push(x);
/// 11 |     }
///    |     - value is dropped here
/// ```
///
/// # Example
///
/// This example requires the `"iter"` feature (enabled by default),
/// because it uses [`ArrayBuilder`](crate::array::ArrayBuilder).
///
#[cfg_attr(feature = "iter", doc = "```rust")]
#[cfg_attr(not(feature = "iter"), doc = "```ignore")]
/// use konst::array::ArrayBuilder;
/// use konst::drop_flavor::MayDrop;
///
/// assert_eq!(make_strings::<1>(), [String::new()]);
/// assert_eq!(make_strings::<2>(), [String::new(), String::new()]);
/// assert_eq!(make_strings::<3>(), [String::new(), String::new(), String::new()]);
///
/// const fn make_strings<const N: usize>() -> [String; N] {
///     let mut builder = ArrayBuilder::of_drop();
///     konst::while_let_Some!{x = produce_option(&builder) =>
///         builder.push(x);
///     }
///     builder.build()
/// }
///
/// const fn produce_option<const N: usize>(
///     ab: &ArrayBuilder<String, N, MayDrop>
/// ) -> Option<String> {
///     if ab.is_full() {
///         None
///     } else {
///         Some(String::new())
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! while_let_Some {
    ($some:pat = $e:expr => $($then:tt)*) => {
        loop {
            match $crate::option::__opt($e) {opt =>
                if $crate::__::Option::is_some(&opt) {
                    let $some = $crate::__::Option::unwrap(opt);
                    $($then)*
                } else {
                    $crate::__::forget(opt);
                    break
                }
            }
        }
    }
}
