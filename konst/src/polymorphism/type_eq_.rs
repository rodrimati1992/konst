
/// Gets a constant of `Te`, 
/// which is (by convention) an enum that allows type-based dispatch 
/// through the use of [`TypeEq`].
/// 
/// # Example 
/// 
/// This example shows how you can make a `const fn` that converts both 
/// `&str` and `&[u8]` to `&str`
/// 
/// ```rust
/// use konst::polymorphism::{InTypeEqEnum, TypeEq};
/// 
/// const fn str_try_from<'a, T, const L: usize>(input: T) -> Result<&'a str, std::str::Utf8Error>
/// where
///     T: Copy + InTypeEqEnum<StrTryFrom<'a, T, L>>
/// {
///     match T::TEQ_ENUM {
///         StrTryFrom::Str(teq) => Ok(teq.to_right(input)),
///         StrTryFrom::Bytes(teq) => {
///             let bytes = teq.to_right(input);
///             std::str::from_utf8(bytes)
///         }
///         StrTryFrom::Array(teq) => {
///             // this requires care not to infinitely recurse
///             let slice: &[_] = teq.to_right(input);
///             str_try_from(slice)
///         }
///     }
/// }
/// 
/// assert_eq!(str_try_from("hello"), Ok("hello"));
/// 
/// assert_eq!(str_try_from(&[b'w', b'o', b'r', b'l', b'd']), Ok("world"));
/// 
/// assert_eq!(str_try_from(b"foo bar" as &[_]), Ok("foo bar"));
/// 
/// 
/// impl<'a> InTypeEqEnum<StrTryFrom<'a, Self, 0>> for &'a str {
///     const TEQ_ENUM: StrTryFrom<'a, Self, 0> = StrTryFrom::Str(TypeEq::NEW);
/// }
/// 
/// impl<'a> InTypeEqEnum<StrTryFrom<'a, Self, 0>> for &'a [u8] {
///     const TEQ_ENUM: StrTryFrom<'a, Self, 0> = StrTryFrom::Bytes(TypeEq::NEW);
/// }
/// 
/// impl<'a, const L: usize> InTypeEqEnum<StrTryFrom<'a, Self, L>> for &'a [u8; L] {
///     const TEQ_ENUM: StrTryFrom<'a, Self, L> = StrTryFrom::Array(TypeEq::NEW);
/// }
/// 
/// // `#[non_exhausitve]` allows adding more supported types to the set.
/// #[non_exhaustive]
/// pub enum StrTryFrom<'a, T, const L: usize> {
///     Str(TypeEq<T, &'a str>),
///     Bytes(TypeEq<T, &'a [u8]>),
///     Array(TypeEq<T, &'a [u8; L]>),
/// }
/// 
/// ```
pub use konst_kernel::type_eq::InTypeEqEnum;


/// Value-level proof that `L` is the same type as `R`
///
/// This type can be used to prove that `L` and `R` are the same type,
/// because it can only be constructed with `TypeEq::<L, L>::NEW`,
/// where both type arguments are the same type.
#[doc(inline)]
pub use konst_kernel::type_eq::type_eq::TypeEq;