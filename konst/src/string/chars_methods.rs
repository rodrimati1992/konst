use crate::{
    iter::{ConstIntoIter, IsIteratorKind},
    string,
};

use konst_kernel::{
    iterator_shared,
    string::{__find_next_char_boundary, __find_prev_char_boundary},
};

/// Converts a string spanning one character to its char value
/// (as a u32)
pub(super) const fn string_to_usv(s: &str) -> u32 {
    match *s.as_bytes() {
        [a] => a as _,
        [a, b] => ((a as u32 & 0x1F) << 6) | (b as u32 & 0x7F),
        [a, b, c] => ((a as u32 & 0xF) << 12) | ((b as u32 & 0x3F) << 6) | (c as u32 & 0x3F),
        [a, b, c, d] => {
            ((a as u32 & 0x7) << 18)
                | ((b as u32 & 0x3F) << 12)
                | ((c as u32 & 0x3F) << 6)
                | (d as u32 & 0x3F)
        }
        _ => {
            #[cfg(feature = "debug")]
            {
                panic!("string must be a single char long")
            }
            #[cfg(not(feature = "debug"))]
            {
                0
            }
        }
    }
}

pub(super) const fn string_to_char(s: &str) -> char {
    let c: u32 = string_to_usv(s);

    unsafe { crate::chr::from_u32_unchecked(c) }
}

/// Cosnt equivalent of [`str::chars`].
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const CHARS: &[char] = &collect_const!(char => string::chars("bar"));
/// const REV: &[char] = &collect_const!(char => string::chars("bar").rev());
///
/// assert_eq!(CHARS, &['b', 'a', 'r']);
/// assert_eq!(REV, &['r', 'a', 'b']);
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn chars(string: &str) -> Chars<'_> {
    Chars { this: string }
}

/// Const equivalent of [`core::str::Chars`]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct Chars<'a> {
    this: &'a str,
}

/// Const equivalent of `core::iter::Rev<core::str::Chars<'_>>`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RChars<'a> {
    this: &'a str,
}

impl ConstIntoIter for Chars<'_> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = char;
}
impl ConstIntoIter for RChars<'_> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = char;
}

macro_rules! chars_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = char,
            iter_forward = Chars<'a>,
            iter_reversed = RChars<'a>,
            next(self){
                if self.this.is_empty() {
                    return None
                }

                let split_at = __find_next_char_boundary(self.this.as_bytes(), 0);
                let (prev, next) = string::split_at(self.this, split_at);

                Some((string_to_char(prev), Self{this: next}))
            },
            next_back{
                if self.this.is_empty() {
                    return None
                }

                let split_at = __find_prev_char_boundary(self.this.as_bytes(), self.this.len());
                let (prev, next) = string::split_at(self.this, split_at);

                Some((string_to_char(next), Self{this: prev}))
            },
            fields = {this},
        }
    };
}

impl<'a> Chars<'a> {
    chars_shared! {is_forward = true}

    /// Gets a string slice to the yet-to-be-iterated characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{for_range, option, string};
    ///
    /// const S: &str = {
    ///     let mut iter = string::chars("hello world");
    ///     for_range!{_ in 0..6 =>
    ///         (_, iter) = option::unwrap!(iter.next());
    ///     }
    ///     iter.as_str()
    /// };
    ///
    /// assert_eq!(S, "world")
    ///
    /// ```
    ///
    pub const fn as_str(&self) -> &'a str {
        self.this
    }
}

impl<'a> RChars<'a> {
    chars_shared! {is_forward = false}
}

////////////////////////////////////////////////////////////////////////////////

/// Cosnt equivalent of [`str::char_indices`].
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const CHARS: &[(usize, char)] =
///     &collect_const!((usize, char) => string::char_indices("个bar人"));
///
/// const REV: &[(usize, char)] =
///     &collect_const!((usize, char) => string::char_indices("个bar人").rev());
///
/// assert_eq!(CHARS, &[(0, '个'), (3, 'b'), (4, 'a'), (5, 'r'), (6, '人')]);
/// assert_eq!(REV, &[(6, '人'), (5, 'r'), (4, 'a'), (3, 'b'), (0, '个')]);
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn char_indices(string: &str) -> CharIndices<'_> {
    CharIndices {
        this: string,
        start_offset: 0,
    }
}

/// Const equivalent of [`core::str::CharIndices`]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct CharIndices<'a> {
    this: &'a str,
    start_offset: usize,
}

/// Const equivalent of `core::iter::Rev<core::str::CharIndices<'_>>`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RCharIndices<'a> {
    this: &'a str,
    start_offset: usize,
}

impl ConstIntoIter for CharIndices<'_> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = (usize, char);
}
impl ConstIntoIter for RCharIndices<'_> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = (usize, char);
}

macro_rules! chars_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = (usize, char),
            iter_forward = CharIndices<'a>,
            iter_reversed = RCharIndices<'a>,
            next(self){
                if self.this.is_empty() {
                    return None
                }

                let split_at = __find_next_char_boundary(self.this.as_bytes(), 0);
                let (prev, next) = string::split_at(self.this, split_at);
                let ret = Self {
                    this: next,
                    start_offset: self.start_offset + split_at,
                };

                Some(((self.start_offset, string_to_char(prev)), ret))
            },
            next_back{
                if self.this.is_empty() {
                    return None
                }

                let split_at = __find_prev_char_boundary(self.this.as_bytes(), self.this.len());
                let (prev, next) = string::split_at(self.this, split_at);
                let ret = Self {
                    this: prev,
                    start_offset: self.start_offset,
                };

                Some(((self.start_offset + split_at, string_to_char(next)), ret))
            },
            fields = {this, start_offset},
        }
    };
}

impl<'a> CharIndices<'a> {
    chars_shared! {is_forward = true}

    /// Gets a string slice to the yet-to-be-iterated characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{for_range, option, string};
    ///
    /// const S: &str = {
    ///     let mut iter = string::char_indices("this is fine");
    ///     for_range!{_ in 0..8 =>
    ///         (_, iter) = option::unwrap!(iter.next());
    ///     }
    ///     iter.as_str()
    /// };
    ///
    /// assert_eq!(S, "fine")
    ///
    /// ```
    ///
    pub const fn as_str(&self) -> &'a str {
        self.this
    }
}

impl<'a> RCharIndices<'a> {
    chars_shared! {is_forward = false}
}
