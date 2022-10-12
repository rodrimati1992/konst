use crate::{
    chr,
    polymorphism::{InTypeSet, TypeEq},
};

/// A string pattern.
///
/// Types that implement this trait can be used to search into a string.
///
pub trait Pattern<'a>: InTypeSet<PatternInput<'a, Self>> + Sized {}

macro_rules! declare_patterns {
    ($((
        $variant:ident, $ty:ty, $normalized:ty, $index:expr,
        |$param:tt| $normalizer:expr
    ),)*) => (
        pub struct PatternInput<'a, T: Pattern<'a>>(PatternInputInner<'a, T>);

        enum PatternInputInner<'a, T: Pattern<'a>> {
            $(
                $variant {te: TypeEq<T, $ty>},
            )*
        }

        $(
            impl<'a> InTypeSet<PatternInput<'a, Self>> for $ty {
                const SET: PatternInput<'a, Self> =
                    PatternInput(PatternInputInner::$variant{
                        te: TypeEq::NEW,
                    });
            }

            impl<'a> Pattern<'a> for $ty {}
        )*

        pub(crate) enum PatternNorm<'a, T> {
            $(
                $variant{
                    val: $normalized,
                    te: TypeEq<T, $ty>,
                },
            )*
        }

        impl<'a, T> PatternNorm<'a, T> {
            pub(crate) const fn new(pattern: T) -> Self
            where
                T: Pattern<'a>
            {
                match T::SET.0 {
                    $(
                        PatternInputInner::$variant{te} => {
                            let $param = te.to_right(pattern);
                            PatternNorm::$variant{val: $normalizer, te}
                        }
                    )*
                }
            }

            pub(crate) const fn as_str(&self) -> &str {
                match self {
                    PatternNorm::Str{val, te} => te.reachability_hint(*val),
                    PatternNorm::Char{val, te} => te.reachability_hint(val.as_str()),
                }
            }

            pub(crate) const fn as_bytes(&self) -> &[u8] {
                match self {
                    PatternNorm::Str{val, te} => te.reachability_hint(val.as_bytes()),
                    PatternNorm::Char{val, te} => te.reachability_hint(val.as_bytes()),
                }
            }
        }
    )
}

declare_patterns! {
    (Str, &'a str, &'a str, 0, |x| x),
    (Char, char, chr::Utf8Encoded, 1, |char| chr::encode_utf8(char)),
}
