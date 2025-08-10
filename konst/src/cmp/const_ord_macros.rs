/// Compares two values for ordering.
///
/// The arguments must implement the [`ConstCmp`] trait,
/// non-std types must have this method:
/// ```rust
/// # struct Foo;
/// # struct T;
/// # impl Foo {
/// const fn const_cmp(&self, _: &T) -> std::cmp::Ordering
/// # { std::cmp::Ordering::Equal }
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
/// use konst::cmp::{const_cmp, impl_cmp, try_equal};
///
/// use std::cmp::Ordering;
///
/// struct Fields<'a> {
///     foo: u32,
///     bar: Option<bool>,
///     baz: Ordering,
///     qux: &'a str,
/// }
///
/// impl_cmp!{
///     impl['a] Fields<'a>;
///     pub const fn const_cmp(&self, other: &Self) -> Ordering {
///         try_equal!(const_cmp!(self.foo, other.foo));
///         try_equal!(const_cmp!(self.bar, other.bar));
///         try_equal!(const_cmp!(self.baz, other.baz));
///         try_equal!(const_cmp!(self.qux, other.qux))
///     }
/// }
///
/// const _: () = {
///     let foo = Fields {
///         foo: 10,
///         bar: None,
///         baz: Ordering::Less,
///         qux: "hello",
///     };
///     
///     let bar = Fields {
///         foo: 99,
///         bar: Some(true),
///         baz: Ordering::Greater,
///         qux: "world",
///     };
///     
///     assert!(matches!(const_cmp!(foo, foo), Ordering::Equal));
///     assert!(matches!(const_cmp!(foo, bar), Ordering::Less));
///     assert!(matches!(const_cmp!(bar, foo), Ordering::Greater));
///     assert!(matches!(const_cmp!(bar, bar), Ordering::Equal));
/// };
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_cmp as const_cmp;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_cmp {
    ($left:expr, $right:expr $(,)*) => {
        match (
            $crate::__assert_const_cmp!(&$left).reff,
            $crate::__assert_const_cmp!(&$right).reff,
        ) {
            (left, right) => {
                let (left, right) = $crate::__coerce_to_cmp2!(left, right);
                let ret: $crate::__::Ordering = left.const_cmp(right);
                ret
            }
        }
    };
}

/// Compares two standard library types for ordering,
/// that can't be compared with [`const_cmp`].
///
/// <span id = "types-section"></span>
/// # Types
///
/// This macro supports multiple types with different prefixes:
///
/// - `slice`: for comparing `&[T]`. [example](#compare_slices)
///
/// - `option`: for comparing `Option<T>`. [example](#compare_options)
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
/// const_cmp_for!(type; left_value, right_value <comparator> )
/// ```
///
/// ### Comparator argument
///
/// The `<comparator>` argument can be any of:
///
/// - ` `(passing nothing): Compares the item using the [`const_cmp`] macro.
///
/// - `, |item| <expression>`:
/// Converts the item with `<expression>` to a type that can be compared using the
/// [`const_cmp`] macro.
///
/// - `, |left_item, right_item| <expression>`:
/// Compares the items by using `<expression>`,
/// which must evaluate to an [`cmp::Ordering`].
///
/// - `, path::to::function`:
/// Compares the items by using the passed function,
/// which must have this signature: `const fn(&Item, &Item) -> std::cmp::Ordering`.
///
/// An *item* is whatever element the passed-in types contain
/// (`T` is the item type for `&[T]`, `Option<T>`, and `Range<T>`),
/// it's always passed by reference.
///
/// # Examples
///
/// <span id = "compare_slices"></span>
/// ### Slices
///
/// ```rust
/// use konst::cmp::{const_cmp, const_cmp_for, try_equal};
///
/// use std::cmp::Ordering;
///
/// const fn cmp_slice_pair(left: &[(u32, u32)], right: &[(u32, u32)]) -> Ordering {
///     const_cmp_for!(slice; left, right, |l, r|{
///         try_equal!(const_cmp!(l.0, r.0));
///         try_equal!(const_cmp!(l.1, r.1))
///     })
/// }
///
/// const _: () = {
///     let foo = &[(0, 1), (1, 2), (3, 4), (5, 6)];
///     let bar = &[(0, 1), (3, 4), (5, 6), (7, 8)];
///
///     assert!(matches!(cmp_slice_pair(foo, foo), Ordering::Equal));
///     assert!(matches!(cmp_slice_pair(foo, bar), Ordering::Less));
///     assert!(matches!(cmp_slice_pair(bar, foo), Ordering::Greater));
///     assert!(matches!(cmp_slice_pair(bar, bar), Ordering::Equal));
/// };
/// ```
///
///
/// <span id = "compare_options"></span>
/// ### Options
///
/// ```rust
/// use konst::cmp::{const_cmp, const_cmp_for, try_equal};
///
/// use std::cmp::Ordering;
///
/// #[derive(Copy, Clone)]
/// enum Shape {
///     Square,
///     Circle,
///     Line,
/// }
///
/// const fn cmp_opt_pair(left: Option<Shape>, right: Option<Shape>) -> Ordering {
///     const_cmp_for!(option; left, right, |x| *x as u8 )
/// }
///
/// const _: () = {
///     let foo = Some(Shape::Square);
///     let bar = Some(Shape::Circle);
///     let baz = Some(Shape::Line);
///
///     assert!(matches!(cmp_opt_pair(foo, foo), Ordering::Equal));
///     assert!(matches!(cmp_opt_pair(foo, bar), Ordering::Less));
///     assert!(matches!(cmp_opt_pair(foo, baz), Ordering::Less));
///
///     assert!(matches!(cmp_opt_pair(bar, foo), Ordering::Greater));
///     assert!(matches!(cmp_opt_pair(bar, bar), Ordering::Equal));
///     assert!(matches!(cmp_opt_pair(bar, baz), Ordering::Less));
///
///     assert!(matches!(cmp_opt_pair(baz, foo), Ordering::Greater));
///     assert!(matches!(cmp_opt_pair(baz, bar), Ordering::Greater));
///     assert!(matches!(cmp_opt_pair(baz, baz), Ordering::Equal));
/// };
///
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
/// [`const_cmp`]: crate::cmp::const_cmp
/// [`cmp::Ordering`]: core::cmp::Ordering
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_cmp_for as const_cmp_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_cmp_for {
    (slice $($rem_args:tt)* ) => {
        $crate::__const_cmp_for_slice!{$($rem_args)*}
    };
    (option $($rem_args:tt)* ) => {
        $crate::__const_cmp_for_option!{$($rem_args)*}
    };
    ($($type:tt $($rem_args:tt)*)?) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "expected type argument, passed `",
            $crate::__::stringify!($($type)?),
            "`; valid type arguments:",
            "\n- slice",
            "\n- option",
        )}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_cmp_for_slice {
    (
        ;$left_slice:expr,
        $right_slice:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_slice, &$right_slice) {(left_slice, right_slice) => {
            let mut left_slice: &[_] = left_slice;
            let mut right_slice: &[_] = right_slice;

            if left_slice.len() == right_slice.len() {
                loop{
                    if let ([l, l_rem@..], [r, r_rem@..]) = (left_slice, right_slice) {
                        left_slice = l_rem;
                        right_slice = r_rem;

                        let ord = $crate::__priv_const_cmp_for!{
                            *l,
                            *r,
                            $($($comparison)*)?
                        };
                        if !$crate::__::matches!(ord, $crate::__::Equal) {
                            break ord;
                        }
                    } else {
                        break $crate::__::Equal
                    }
                }
            } else if left_slice.len() < right_slice.len() {
                $crate::__::Less
            } else {
                $crate::__::Greater
            }
        }}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_cmp_for_option {
    (
        ;$left_opt:expr,
        $right_opt:expr
        $(, $($comparison:tt)* )?
    ) => {
        match ($crate::__optref!(&$left_opt).reff, $crate::__optref!(&$right_opt).reff) {
            (Some(l), Some(r)) =>
                $crate::__priv_const_cmp_for!(*l, *r, $( $($comparison)* )?),
            (Some(_), None) => $crate::__::Greater,
            (None, Some(_)) => $crate::__::Less,
            (None, None) => $crate::__::Equal,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_const_cmp_for {
    ($left:expr, $right:expr, ) => {{
        let ret: $crate::__::Ordering = $crate::cmp::coerce_to_cmp!(&$left).const_cmp(&$right);
        ret
    }};
    ($left:expr, $right:expr, |$l:pat_param| $key_expr:expr $(,)*) => {{
        let ret: $crate::__::Ordering = $crate::cmp::coerce_to_cmp!({
            let $l = &$left;
            $key_expr
        })
        .const_cmp(&{
            let $l = &$right;
            $key_expr
        });
        ret
    }};
    ($left:expr, $right:expr, |$l:pat_param, $r:pat_param| $eq_expr:expr $(,)*) => {{
        let $l = &$left;
        let $r = &$right;
        let ret: $crate::__::Ordering = $eq_expr;
        ret
    }};
    ($left:expr, $right:expr, $func:path $(,)*) => {{
        let func = $func;
        let _: fn(&_, &_) -> $crate::__::Ordering = func;
        let ret: $crate::__::Ordering = func(&$left, &$right);
        ret
    }};
}

/// Evaluates to `$ord` if it is `Ordering::Equal`,
/// otherwise returns it from the enclosing function.
///
/// # Example
///
/// ```rust
/// use konst::cmp::{const_cmp, impl_cmp, try_equal};
///
/// use std::cmp::Ordering;
///
/// struct Fields<'a> {
///     first: &'a [u8; 4],
///     second: bool,
///     third: Option<&'static str>,
/// }
///
/// impl_cmp!{
///     impl['a] Fields<'a>;
///     pub const fn const_cmp(&self, other: &Self) -> Ordering {
///         try_equal!(const_cmp!(self.first, other.first));
///         try_equal!(const_cmp!(self.second, other.second));
///         try_equal!(const_cmp!(self.third, other.third))
///     }
/// }
///
/// const _: () = {
///     let foo = Fields {
///         first: &[3, 5, 8, 13],
///         second: false,
///         third: None,
///     };
///     
///     let bar = Fields {
///         first: &[5, 8, 13, 14],
///         second: true,
///         third: Some("what!?"),
///     };
///     
///     assert!(matches!(const_cmp!(foo, foo), Ordering::Equal));
///     assert!(matches!(const_cmp!(foo, bar), Ordering::Less));
///     assert!(matches!(const_cmp!(bar, foo), Ordering::Greater));
///     assert!(matches!(const_cmp!(bar, bar), Ordering::Equal));
/// };
///
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__try_equal as try_equal;

#[doc(hidden)]
#[macro_export]
macro_rules! __try_equal {
    ($ord:expr $(,)*) => {
        match $ord {
            $crate::__::Ordering::Equal => $crate::__::Ordering::Equal,
            ord => return ord,
        }
    };
}

#[cfg(feature = "cmp")]
macro_rules! cmp_int {
    ($l:expr, $r:expr $(,)*) => {{
        if $l == $r {
            Ordering::Equal
        } else if $l < $r {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }};
}
pub(crate) use cmp_int;
