/// Compares two values for equality.
///
/// The arguments must implement the [`ConstCmp`] trait,
/// non-std types must have this method:
/// ```rust
/// # struct Foo;
/// # struct T;
/// # impl Foo {
/// const fn const_eq(&self, _: &T) -> bool
/// # { false }
/// # }
/// ```
///
/// # Limitations
///
/// The arguments must be concrete types, and have a fully inferred type.
/// eg: if you pass an integer literal it must have a suffix to indicate its type.
///
/// # Example
///
/// ```rust
/// use konst::cmp::{const_eq, impl_cmp};
///
/// use std::ops::Range;
///
/// struct Fields<'a> {
///     foo: u32,
///     bar: Option<bool>,
///     baz: Range<usize>,
///     qux: &'a str,
/// }
///
/// impl_cmp!{
///     impl['a] Fields<'a>;
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         self.foo == other.foo &&
///         const_eq!(self.bar, other.bar) &&
///         const_eq!(self.baz, other.baz) &&
///         const_eq!(self.qux, other.qux)
///     }
/// }
///
/// const _: () = {
///     let foo = Fields {
///         foo: 10,
///         bar: None,
///         baz: 10..20,
///         qux: "hello",
///     };
///     
///     let bar = Fields {
///         foo: 99,
///         bar: Some(true),
///         baz: 0..5,
///         qux: "world",
///     };
///     
///     assert!( const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
///     assert!(!const_eq!(bar, foo));
///     assert!( const_eq!(bar, bar));
/// };
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_eq as const_eq;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq {
    ($left:expr, $right:expr $(,)*) => {
        match (
            $crate::__assert_const_cmp!(&$left).reff,
            $crate::__assert_const_cmp!(&$right).reff,
        ) {
            (left, right) => {
                let (left, right) = $crate::__coerce_to_cmp2!(left, right);
                let ret: $crate::__::bool = left.const_eq(right);
                ret
            }
        }
    };
}

/// Compares two standard library types for equality,
/// that can't be compared with [`const_eq`].
///
/// <span id = "types-section"></span>
/// # Types
///
/// This macro supports multiple types with different prefixes:
///
/// - `slice`: for comparing `&[T]`. [example](#compare_slices_structs)
///
/// - `option`: for comparing `Option<T>`. [example](#compare_options)
///
/// - `range`: for comparing `Range<T>`. [example](#compare_ranges)
///
/// - `range_inclusive`: for comparing `RangeInclusive<T>`.
/// [example](#compare_ranges_incluside)
///
/// <span id = "limitations-section"></span>
/// # Limitations
///
/// The arguments must be concrete types, and have a fully inferred type.
/// eg: if you pass an integer literal it must have a suffix to indicate its type.
///
/// <span id = "arguments-section"></span>
/// # Arguments
///
/// The arguments take this form
///
/// ```text
/// const_eq_for!(type; left_value, right_value <comparator> )
/// ```
///
/// ### Comparator argument
///
/// The `<comparator>` argument can be any of:
///
/// - ` `(passing nothing): Compares the item using the [`const_eq`] macro.
/// [example](#compare_slices_structs)
///
/// - `, |item| <expression>`:
/// Converts the item with `<expression>` to a type that can be compared using the
/// [`const_eq`] macro.
/// [example](#compare_slices_fieldless_enums)
///
/// - `, |left_item, right_item| <expression>`:
/// Compares the items by using `<expression>`, which must evaluate to a `bool`.
/// [example](#compare_options)
///
/// - `, path::to::function`:
/// Compares the items by using the passed function,
/// which must have this signature: `const fn(&Item, &Item) -> bool`.
/// [example](#compare_ranges_incluside)
///
/// An *item* is whatever element the passed-in types contain
/// (`T` is the item type for `&[T]`, `Option<T>`, and `Range<T>`),
/// it's always passed by reference.
///
/// # Examples
///
/// <span id = "compare_slices_structs"></span>
/// ### Comparing slices of structs
///
/// ```
/// use konst::{cmp::const_eq_for, eq_str};
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// pub struct Location {
///     pub file: &'static str,
///     pub column: u32,
///     pub line: u32,
/// }
///
/// konst::cmp::impl_cmp! {
///     impl Location;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         eq_str(self.file, other.file) &&
///         self.column == other.column &&
///         self.line == other.line
///     }
/// }
/// #
/// # macro_rules! here {
/// #   () => {
/// #       $crate::Location{file: file!(), column: column!(), line: line!()}
/// #   }
/// # }
/// #
///
/// # fn main () {
/// const HERE: &[Location] = &[here!(), here!(), here!(), here!()];
///
/// const THERE: &[Location] = &[here!(), here!(), here!(), here!()];
///
/// const _: () = {
///     assert!( const_eq_for!(slice; HERE, HERE));
///     assert!(!const_eq_for!(slice; HERE, THERE));
///     assert!( const_eq_for!(slice; THERE, THERE));
/// };
/// # }
///
/// ```
///
/// <span id = "compare_slices_fieldless_enums"></span>
/// ### Comparing slices of field-less enums
///
/// ```rust
/// #[derive(Copy, Clone)]
/// enum Direction {
///     Left,
///     Right,
///     Up,
///     Down,
/// }
///
/// use Direction::*;
///
/// const fn eq_slice_direction(left: &[Direction], right: &[Direction]) -> bool {
///     konst::cmp::const_eq_for!(slice; left, right, |&x| x as u8)
/// }
///
/// const CHEAT_CODE: &[Direction] = &[Up, Up, Down, Down, Left, Right, Left, Right];
///
/// const CLOCKWISE: &[Direction] = &[Up, Right, Down, Left];
///
/// const _: () = {
///     assert!( eq_slice_direction(CHEAT_CODE, CHEAT_CODE));
///     assert!(!eq_slice_direction(CHEAT_CODE, CLOCKWISE));
///     assert!( eq_slice_direction(CLOCKWISE, CLOCKWISE));
/// };
///
/// ```
///
/// <span id = "compare_options"></span>
/// ### Comparing `Option`s
///
/// ```rust
/// use konst::cmp::const_eq_for;
///
/// const SOME: Option<(u32, u32)> = Some((3, 5));
/// const NONE: Option<(u32, u32)> = None;
///
/// const fn eq_opt_tuple(left: &Option<(u32, u32)>, right: &Option<(u32, u32)>) -> bool {
///     const_eq_for!(option; left, right, |l, r| l.0 == r.0 && l.1 == r.1 )
/// }
///
/// const _: () = {
///     assert!( eq_opt_tuple(&SOME, &SOME));
///     assert!(!eq_opt_tuple(&SOME, &NONE));
///     assert!( eq_opt_tuple(&NONE, &NONE));
/// };
///
/// ```
///
///
/// <span id = "compare_ranges"></span>
/// ### Comparing `Range`s
///
/// ```rust
/// use konst::cmp::{const_eq_for, impl_cmp};
///
/// use std::ops::Range;
///
/// #[derive(Copy, Clone)]
/// pub enum Month {
///     January,
///     February,
///     March,
///     April,
///     May,
///     June,
///     July,
///     August,
///     September,
///     October,
///     November,
///     December,
/// }
///
/// use Month::*;
///
/// impl_cmp! {
///     impl Month;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         *self as u8 == *other as u8
///     }
/// }
///
/// const FOO: Range<Month> = January..April;
/// const BAR: Range<Month> = October..December;
///
/// const _: () = {
///     assert!( const_eq_for!(range; FOO, FOO));
///     assert!(!const_eq_for!(range; FOO, BAR));
///     assert!( const_eq_for!(range; BAR, BAR));
/// };
///
/// ```
///
/// <span id = "compare_ranges_incluside"></span>
/// ### Comparing `RangeInclusive`s
///
/// ```rust
/// use konst::cmp::{const_eq_for, impl_cmp};
///
/// use std::ops::RangeInclusive;
///
/// #[derive(Copy, Clone)]
/// pub enum WeekDay {
///     Monday,
///     Tuesday,
///     Wednesday,
///     Thursday,
///     Friday,
///     Saturday,
///     Sunday,
/// }
///
/// use WeekDay::*;
///
/// impl_cmp! {
///     impl WeekDay;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         *self as u8 == *other as u8
///     }
/// }
///
/// const FOO: RangeInclusive<WeekDay> = Monday..=Thursday;
/// const BAR: RangeInclusive<WeekDay> = Friday..=Sunday;
///
/// const _: () = {
///     assert!( const_eq_for!(range_inclusive; FOO, FOO));
///     assert!(!const_eq_for!(range_inclusive; FOO, BAR, WeekDay::const_eq));
///     assert!( const_eq_for!(range_inclusive; BAR, BAR, WeekDay::const_eq));
/// };
///
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
/// [`const_eq`]: crate::cmp::const_eq
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_eq_for as const_eq_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq_for {
    (slice $($rem_args:tt)* ) => {
        $crate::__const_eq_for_slice!{$($rem_args)*}
    };
    (option $($rem_args:tt)* ) => {
        $crate::__const_eq_for_option!{$($rem_args)*}
    };
    (range $($rem_args:tt)* ) => {
        $crate::__const_eq_for_range!{$($rem_args)*}
    };
    (range_inclusive $($rem_args:tt)* ) => {
        $crate::__const_eq_for_range_inc!{$($rem_args)*}
    };
    ($($type:tt $($rem_args:tt)*)?) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "expected type argument, passed `",
            $crate::__::stringify!($($type)?),
            "`; valid type arguments:",
            "\n- slice",
            "\n- option",
            "\n- range",
            "\n- range_inclusive",
        )}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq_for_slice {
    (
        ;$left_slice:expr,
        $right_slice:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_slice, &$right_slice) {
            (left_slice, right_slice) => {
                let left_slice: &[_] = left_slice;
                let right_slice: &[_] = right_slice;

                let mut returned = left_slice.len() == right_slice.len();
                if returned {
                    let mut i = 0;
                    while i != left_slice.len() {
                        let are_eq = $crate::__priv_const_eq_for!(
                            left_slice[i],
                            right_slice[i],
                            $( $($comparison)* )?
                        );
                        if !are_eq {
                            returned = false;
                            break;
                        }
                        i += 1;
                    }
                }
                returned
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq_for_option {
    (
        ;$left_opt:expr,
        $right_opt:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_opt, &$right_opt) {
            (Some(l), Some(r)) =>
                $crate::__priv_const_eq_for!(*l, *r, $( $($comparison)* )?),
            (None, None) => true,
            _ => false,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq_for_range {
    (
        ;$left_range:expr,
        $right_range:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_range, &$right_range) {
            (left_range, right_range) => {
                $crate::__priv_const_eq_for!(
                    left_range.start,
                    right_range.start,
                    $( $($comparison)* )?
                ) &&
                $crate::__priv_const_eq_for!(
                    left_range.end,
                    right_range.end,
                    $( $($comparison)* )?
                )
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_eq_for_range_inc {
    (
        ;$left_range:expr,
        $right_range:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_range, &$right_range) {
            (left_range, right_range) => {
                $crate::__priv_const_eq_for!(
                    left_range.start(),
                    right_range.start(),
                    $( $($comparison)* )?
                ) &&
                $crate::__priv_const_eq_for!(
                    left_range.end(),
                    right_range.end(),
                    $( $($comparison)* )?
                )
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_const_eq_for {
    ($left:expr, $right:expr, ) => {{
        let ret: $crate::__::bool = $crate::cmp::coerce_to_cmp!($left).const_eq(&$right);
        ret
    }};
    ($left:expr, $right:expr, |$l:pat_param| $key_expr:expr $(,)*) => {{
        let ret: $crate::__::bool = $crate::cmp::coerce_to_cmp!({
            let $l = &$left;
            $key_expr
        })
        .const_eq(&{
            let $l = &$right;
            $key_expr
        });
        ret
    }};
    ($left:expr, $right:expr, |$l:pat_param, $r:pat_param| $eq_expr:expr $(,)*) => {{
        let $l = &$left;
        let $r = &$right;
        let ret: $crate::__::bool = $eq_expr;
        ret
    }};
    ($left:expr, $right:expr, $func:path $(,)*) => {{
        let func = $func;
        let _: for<'a, 'b> fn(&'a _, &'b _) -> $crate::__::bool = func;
        let ret: $crate::__::bool = func(&$left, &$right);
        ret
    }};
}

/// Compares two values for inequality.
///
/// The arguments must implement the [`ConstCmp`] trait,
/// non-std types must have this method:
/// ```rust
/// # struct Foo;
/// # struct T;
/// # impl Foo {
/// const fn const_eq(&self, _: &T) -> bool
/// # { false }
/// # }
/// ```
///
/// # Limitations
///
/// The arguments must be concrete types, and have a fully inferred type.
/// eg: if you pass an integer literal it must have a suffix to indicate its type.
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_ne;
///
/// const _: () = {
///     assert!( const_ne!(0u8..10, 20..30));
///     assert!(!const_ne!(0u8..10, 0..10));
/// };
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_ne as const_ne;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_ne {
    ($($args:tt)*) => {
        !$crate::cmp::const_eq!($($args)*)
    }
}

/// Compares two standard library types for inequality,
/// that can't be compared with [`const_ne`].
///
/// This takes the same arguments as the [`const_eq_for`] macro,
/// and has the same limitations as it does.
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_ne_for;
///
/// use std::ops::Range;
///
/// #[derive(Copy, Clone)]
/// pub enum Day {
///     Monday,
///     Tuesday,
///     Wednesday,
///     Thursday,
///     Friday,
///     Saturday,
///     Sunday,
/// }
///
/// use Day::*;
///
/// konst::cmp::impl_cmp! {
///     impl Day;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         *self as u8 == *other as u8
///     }
/// }
///
/// const FOO: Range<Day> = Monday..Wednesday;
/// const BAR: Range<Day> = Friday..Sunday;
///
/// const _: () = {
///     assert!(!const_ne_for!(range; FOO, FOO));
///     assert!( const_ne_for!(range; FOO, BAR));
///     assert!(!const_ne_for!(range; BAR, BAR));
/// };
///
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_ne_for as const_ne_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_ne_for {
    ($($args:tt)*) => {
        !$crate::cmp::const_eq_for!($($args)*)
    }
}
