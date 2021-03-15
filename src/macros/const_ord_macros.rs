#[macro_export]
macro_rules! konst {
    ($left:expr, $right:expr) => {
        match $crate::coerce_to_cmp!($left, $right) {
            (left, right) => left.konst(right),
        }
    };
}

/// Compares two slices for ordering, evaluating to a [`cmp::Ordering`]
///
/// # Examples
///
///
///
///
///
///
/// [`cmp::Ordering`]: https://doc.rust-lang.org/core/cmp/enum.Ordering.html
///
#[macro_export]
macro_rules! konst_for {
    (
        slice;
        $left_slice:expr,
        $right_slice:expr
        $(, $($comparison:tt)* )?
    ) => {
        match ($left_slice, $right_slice) {(mut left_slice, mut right_slice) => {
            use $crate::__::Ordering as CmpOrdering;
            if left_slice.len() == right_slice.len() {
                loop{
                    if let ([l, l_rem@..], [r, r_rem@..]) = (left_slice, right_slice) {
                        left_slice = l_rem;
                        right_slice = r_rem;

                        let ord = $crate::__priv_konst_for!{
                            *l,
                            *r,
                            $($($comparison)*)?
                        };
                        if !$crate::__::matches!(ord, $crate::__::Ordering::Equal) {
                            break ord;
                        }
                    } else {
                        break $crate::__::Ordering::Equal
                    }
                }
            } else if left_slice.len() < right_slice.len() {
                CmpOrdering::Less
            } else {
                CmpOrdering::Greater
            }
        }}
    };
    (
        option;
        $left_opt:expr,
        $right_opt:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_opt, &$right_opt) {
            (Some(l), Some(r)) =>
                $crate::__priv_konst_for!(*l, *r, $( $($comparison)* )?),
            (Some(_), None) => $crate::__::Greater,
            (None, Some(_)) => $crate::__::Less,
            (None, None) => $crate::__::Equal,
        }
    };
    (
        range;
        $left_range:expr,
        $right_range:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_range, &$right_range) {
            (left_range, right_range) => {
                use $crate::__::Ordering as CmpOrdering;

                let start = $crate::__priv_konst_for!(
                    left_range.start,
                    right_range.start,
                    $( $($comparison)* )?
                );

                if let CmpOrdering::Equal = starts {
                    $crate::__priv_konst_for!(
                        left_range.end,
                        right_range.end,
                        $( $($comparison)* )?
                    )
                } else {
                    start
                }
            }
        }
    };
    (
        range_inclusive;
        $left_range:expr,
        $right_range:expr
        $(, $($comparison:tt)* )?
    ) => {
        match (&$left_range, &$right_range) {
            (left_range, right_range) => {
                use $crate::__::Ordering as CmpOrdering;

                let start = $crate::__priv_konst_for!(
                    left_range.start(),
                    right_range.start(),
                    $( $($comparison)* )?
                );

                if let CmpOrdering::Equal = start {
                    $crate::__priv_konst_for!(
                        left_range.end(),
                        right_range.end(),
                        $( $($comparison)* )?
                    )
                } else {
                    start
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_konst_for {
    ($left:expr, $right:expr, ) => {
        $crate::coerce_to_cmp!(&$left).konst(&$right)
    };
    ($left:expr, $right:expr, |$l: pat| $key_expr:expr $(,)*) => {
        $crate::coerce_to_cmp!({
            let $l = &$left;
            $key_expr
        })
        .konst(&{
            let $l = &$right;
            $key_expr
        })
    };
    ($left:expr, $right:expr, |$l: pat, $r: pat| $eq_expr:expr $(,)*) => {{
        let $l = &$left;
        let $r = &$right;
        $eq_expr
    }};
    ($left:expr, $right:expr, $func:path $(,)*) => {{
        $func(&$left, &$right)
    }};
}

/// Evaluates to `$ord` if it is `Ordering::Equal`,
/// otherwise returns it from the enclosing function.
#[macro_export]
macro_rules! try_equal {
    (break $ord:expr) => {
        match $ord {
            $crate::__::Ordering::Equal => $crate::__::Ordering::Equal,
            ord => return ord,
        }
    };
    ($ord:expr) => {
        match $ord {
            $crate::__::Ordering::Equal => $crate::__::Ordering::Equal,
            ord => return ord,
        }
    };
    (break; $ord:expr) => {
        match $ord {
            $crate::__::Ordering::Equal => $crate::__::Ordering::Equal,
            ord => return ord,
        }
    };
}

macro_rules! cmp_int {
    ($l:expr, $r:expr) => {{
        if $l == $r {
            Ordering::Equal
        } else if $l < $r {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }};
}
