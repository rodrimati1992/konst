use crate::{
    chr,
    polymorphism::{HasTypeWitness, MakeTypeWitness, TypeEq, TypeWitnessTypeArg},
};

/// A byte slice pattern.
///
/// Types that implement this trait can be used to search into a byte slice.
///
/// This trait can only be implemented in the `konst` crate.
///
pub trait BytesPattern<const N: usize>: HasTypeWitness<BytesPatternInput<N, Self>> {
    #[doc(hidden)]
    type __Normalized: ?Sized;
}

#[non_exhaustive]
pub enum BytesPatternInput<const N: usize, P: ?Sized + BytesPattern<N>> {
    Str(TypeEq<P, str>, TypeEq<*const P::__Normalized, *const [u8]>),
    Bytes(TypeEq<P, [u8]>, TypeEq<*const P::__Normalized, *const [u8]>),
    Array(
        TypeEq<P, [u8; N]>,
        TypeEq<*const P::__Normalized, *const [u8]>,
    ),
    Char(
        TypeEq<P, char>,
        TypeEq<*const P::__Normalized, *const chr::Utf8Encoded>,
    ),
}

impl<const N: usize, Arg> TypeWitnessTypeArg for BytesPatternInput<N, Arg>
where
    Arg: ?Sized + BytesPattern<N>,
{
    type Arg = Arg;
}

impl MakeTypeWitness for BytesPatternInput<0, str> {
    const MAKE: Self = BytesPatternInput::Str(TypeEq::NEW, TypeEq::NEW);
}
impl BytesPattern<0> for str {
    #[doc(hidden)]
    type __Normalized = [u8];
}

impl MakeTypeWitness for BytesPatternInput<0, [u8]> {
    const MAKE: Self = BytesPatternInput::Bytes(TypeEq::NEW, TypeEq::NEW);
}
impl BytesPattern<0> for [u8] {
    #[doc(hidden)]
    type __Normalized = [u8];
}

impl<const N: usize> MakeTypeWitness for BytesPatternInput<N, [u8; N]> {
    const MAKE: Self = BytesPatternInput::Array(TypeEq::NEW, TypeEq::NEW);
}
impl<const N: usize> BytesPattern<N> for [u8; N] {
    #[doc(hidden)]
    type __Normalized = [u8];
}

impl MakeTypeWitness for BytesPatternInput<0, char> {
    const MAKE: Self = BytesPatternInput::Char(TypeEq::NEW, TypeEq::NEW);
}
impl BytesPattern<0> for char {
    #[doc(hidden)]
    type __Normalized = chr::Utf8Encoded;
}

#[derive(Copy, Clone)]
pub(crate) enum PatternNorm<'a, const N: usize, P>
where
    P: ?Sized + BytesPattern<N>,
{
    Bytes {
        val: &'a [u8],
        ten: TypeEq<*const P::__Normalized, *const [u8]>,
    },
    Char {
        val: chr::Utf8Encoded,
        ten: TypeEq<*const P::__Normalized, *const chr::Utf8Encoded>,
    },
}

impl<'a, const N: usize, P> PatternNorm<'a, N, P>
where
    P: ?Sized + BytesPattern<N>,
{
    pub(crate) const fn new(pattern: &'a P) -> Self {
        match P::WITNESS {
            BytesPatternInput::Str(te, ten) => {
                let val: &'a [u8] = te.in_ref().to_right(pattern).as_bytes();
                PatternNorm::Bytes { val, ten }
            }
            BytesPatternInput::Bytes(te, ten) => {
                let val: &'a [u8] = te.in_ref().to_right(pattern);
                PatternNorm::Bytes { val, ten }
            }
            BytesPatternInput::Array(te, ten) => {
                let val: &'a [u8] = te.in_ref().to_right(pattern);
                PatternNorm::Bytes { val, ten }
            }
            BytesPatternInput::Char(te, ten) => {
                let val: &'a char = te.in_ref().to_right(pattern);
                let val = chr::encode_utf8(*val);
                PatternNorm::Char { val, ten }
            }
        }
    }

    pub(crate) const fn as_bytes(&self) -> &[u8] {
        match self {
            PatternNorm::Bytes { val, ten } => ten.reachability_hint(val),
            PatternNorm::Char { val, ten } => ten.reachability_hint(val.as_bytes()),
        }
    }
}
