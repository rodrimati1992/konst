/// Compares two slices for equality with the passed in predicate.
///
/// # Exmaples
///
/// ### Comparing slices of structs
///
/// ```
/// use const_cmp::str_eq;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// pub struct Location {
///     pub file: &'static str,
///     pub column: u32,
///     pub line: u32,
/// }
///
/// impl Location {
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         str_eq(self.file, other.file) &&
///         self.column == other.column &&
///         self.line == other.line
///     }
///     
///     pub const fn const_eq_slice(this: &[Self], other: &[Self]) -> bool {
///         const_cmp::slice_eq_by!(this, other, |l, r| l.const_eq(r) )
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
/// const CMP_HERE: bool = Location::const_eq_slice(HERE, HERE);
/// assert!( CMP_HERE );
///
/// const CMP_HERE_THERE: bool = Location::const_eq_slice(HERE, THERE);
/// assert!( !CMP_HERE_THERE );
///
/// const CMP_THERE_THERE: bool = Location::const_eq_slice(THERE, THERE);
/// assert!( CMP_THERE_THERE );
///
///
/// # }
///
/// ```
///
#[macro_export]
macro_rules! slice_eq_by {
    (
        $left_slice:expr,
        $right_slice:expr,
        |$l: pat, $r: pat| $eq_expr:expr $(,)*
    ) => {
        match ($left_slice, $right_slice) {
            (left_slice, right_slice) => {
                let mut returned = left_slice.len() == right_slice.len();
                if returned {
                    let mut i = 0;
                    while i != left_slice.len() {
                        let $l = &left_slice[i];
                        let $r = &right_slice[i];
                        if !$eq_expr {
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

/// Compares two slices for equality by converting the slice elements to
/// some primitive type comparable with `==`, usually an integer.
///
///
/// # Example
///
/// ### Comparing slices of enums
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
/// const fn slice_eq_direction(left: &[Direction], right: &[Direction]) -> bool {
///     const_cmp::slice_eq_by_key!(left, right, |&x| x as u8 )
/// }
///
/// const CHEAT_CODE: &[Direction] = &[Up, Up, Down, Down, Left, Right, Left, Right];
///
/// const CLOCKWISE: &[Direction] = &[Up, Right, Down, Left];
///
///
/// const CMP_CHEAT: bool = slice_eq_direction(CHEAT_CODE, CHEAT_CODE);
/// assert!( CMP_CHEAT );
///
/// const CMP_CHEAT_CLOCK: bool = slice_eq_direction(CHEAT_CODE, CLOCKWISE);
/// assert!( !CMP_CHEAT_CLOCK );
///
/// const CMP_CLOCK_CLOCK: bool = slice_eq_direction(CLOCKWISE, CLOCKWISE);
/// assert!( CMP_CLOCK_CLOCK );
///
///
/// ```
#[macro_export]
macro_rules! slice_eq_by_key {
    (
        $left_slice:expr,
        $right_slice:expr,
        |$v: pat| $to_key:expr $(,)*
    ) => {
        match ($left_slice, $right_slice) {
            (left_slice, right_slice) => {
                let mut returned = left_slice.len() == right_slice.len();
                if returned {
                    let mut i = 0;
                    while i != left_slice.len() {
                        let left_elem = {
                            let $v = &left_slice[i];
                            $to_key
                        };
                        let right_elem = {
                            let $v = &right_slice[i];
                            $to_key
                        };
                        if left_elem != right_elem {
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
