/// `__delegate_const_eq` allows:
/// - defining a free function,
/// - defining an inherent `cosnt_eq` method on CmpWrapper that delegates to that free function.
/// - ConstCmp impl for the first parameter type
/// - Add a coerce inhenrent method for IsAConstCmp
///
#[doc(hidden)]
macro_rules! __delegate_const_eq{
    (
        $(skip_coerce $(@$_skip:tt@)?;)?

        $(for[$($implg:tt)*])?
        $(#[$attr:meta])*
        pub const fn $func:ident $(<$($fnlt:lifetime),* $(,)?>)?(
            $($idents:ident)* : $l_ty:ty,
            $rhs:ident: $r_ty:ty $(,)*
        ) -> $ret:ty $block:block
    )=>{
        $(#[$attr])*
        pub const fn $func<$($($fnlt,)*)? $($($implg)*)?>(
            __priv_get_pati_ident!($($idents)*): __priv_ref_if_ref!(($($idents)*) $l_ty),
            $rhs: $r_ty,
        ) -> $ret $block

        #[cfg(feature = "cmp")]
        const _: () = {
            #[cfg(all($(false $(@$_skip@)?)?))]
            impl<$($($implg)*)?> crate::cmp::ConstCmp for $l_ty {
                type Kind = crate::cmp::IsStdKind;
                type This = Self;
            }

            impl<$($($implg)*)?> crate::cmp::CmpWrapper<$l_ty> {
                /// Compares `self` and `other` for equality.
                #[inline]
                pub const fn const_eq<$($($fnlt,)*)?>(
                    &self,
                    other: __priv_ref_if_nonref!(($($idents)*) $r_ty),
                ) -> bool {
                    $func(
                        __priv_copy_if_nonref!(($($idents)*) self.0),
                        __priv_deref_if_nonref!(($($idents)*) other)
                    )
                }
            }
        };
    };
}

macro_rules! __delegate_const_ord{
    (
        $(skip_coerce $(@$_skip:tt@)?;)?

        $(for[$($implg:tt)*])?
        $(#[$attr:meta])*
        pub const fn $func:ident $(<$($fnlt:lifetime),* $(,)?>)?(
            $($idents:ident)* : $l_ty:ty,
            $rhs:ident: $r_ty:ty $(,)*
        ) -> $ret:ty $block:block
    )=>{
        $(#[$attr])*
        pub const fn $func<$($($fnlt,)*)? $($($implg)*)?>(
            __priv_get_pati_ident!($($idents)*): __priv_ref_if_ref!(($($idents)*) $l_ty),
            $rhs: $r_ty,
        ) -> $ret $block

        #[cfg(feature = "cmp")]
        const _: () = {
            impl<$($($implg)*)?> crate::cmp::CmpWrapper<$l_ty> {
                /// Compares `self` and `other` for ordering.
                #[inline]
                pub const fn const_cmp<$($($fnlt,)*)?>(
                    &self,
                    other: __priv_ref_if_nonref!(($($idents)*) $r_ty),
                ) -> core::cmp::Ordering {
                    $func(
                        __priv_copy_if_nonref!(($($idents)*) self.0),
                        __priv_deref_if_nonref!(($($idents)*) other)
                    )
                }
            }
        };
    };
}

#[cfg(feature = "cmp")]
#[doc(hidden)]
macro_rules! __priv_copy_if_nonref {
    ((ref $ident:ident) $expr:expr) => {
        &$expr
    };
    ((copy $ident:ident) $expr:expr) => {
        $expr
    };
}
#[cfg(feature = "cmp")]
#[doc(hidden)]
macro_rules! __priv_deref_if_nonref {
    ((ref $ident:ident) $expr:expr) => {
        $expr
    };
    ((copy $ident:ident) $expr:expr) => {
        *$expr
    };
}

#[cfg(feature = "cmp")]
#[doc(hidden)]
macro_rules! __priv_ref_if_nonref {
    ((ref $ident:ident) $ty:ty) => {
        $ty
    };
    ((copy $ident:ident) $ty:ty) => {
        &$ty
    };
}

#[cfg(feature = "cmp")]
#[doc(hidden)]
macro_rules! __priv_ref_if_ref {
    ((ref $ident:ident) $ty:ty) => {
        &$ty
    };
    ((copy $ident:ident) $ty:ty) => {
        $ty
    };
}

#[doc(hidden)]
macro_rules! __priv_get_pati_ident {
    (ref $ident:ident) => {
        $ident
    };
    (copy $ident:ident) => {
        $ident
    };
}
