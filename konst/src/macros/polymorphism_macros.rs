#[doc(hidden)]
#[macro_export]
macro_rules!  __priv_delegate_const_inner_fn{
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
            $crate::__priv_get_pati_ident!($($idents)*): $l_ty,
            $rhs: $r_ty,
        ) -> $ret $block
    }
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules!  __priv_delegate_const_inner_cmpwrapper{
    (
        ($cw_method:ident, $returns:ty)

        $(skip_coerce $(@$_skip:tt@)?;)*

        $( for[$($implg:tt)*] )?
        $(#[$attr:meta])*
        pub const fn $func:ident $(<$($fnlt:lifetime),* $(,)?>)?(
            $($idents:ident)* : $l_ty:ty,
            $rhs:ident: $r_ty:ty $(,)*
        ) -> $ret:ty $block:block
    ) => {
        $crate::__priv_std_kind_impl!{
            $(skip_coerce $(@$_skip@)?;)*
            impl[$($($implg)*)?] $l_ty
        }

        impl<$($($implg)*)?> $crate::__::CmpWrapper<$l_ty> {
            #[inline]
            pub const fn $cw_method<$($($fnlt,)*)?>(
                &self,
                r: $crate::__priv_ref_if_nonref!(($($idents)*) $r_ty),
            ) -> $returns {
                $func(
                    $crate::__priv_copy_if_nonref!(($($idents)*) self.0),
                    $crate::__priv_deref_if_nonref!(($($idents)*) r)
                )
            }
        }
    }
}

/// `__delegate_const_eq` allows:
/// - defining a free function,
/// - defining an inherent `cosnt_eq` method on CmpWrapper that delegates to that free function.
/// - ConstCmpMarker impl for the first parameter type
/// - Add a coerce inhenrent method for IsAConstCmpMarker
///
#[cfg(not(feature = "polymorphism"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __delegate_const_eq{
    ( $($input:tt)* )=>{
        $crate::__priv_delegate_const_inner_fn!{ $($input)* }
    }
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules!  __delegate_const_eq{
    ( $($input:tt)* )=>{
        $crate::__priv_delegate_const_inner_fn!{ $($input)* }

        $crate::__priv_delegate_const_inner_cmpwrapper!{
            (const_eq, bool)

            $($input)*
        }
    };
}

#[cfg(not(feature = "polymorphism"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __delegate_const_ord{
    ($($input:tt)*)=>{
        $crate::__priv_delegate_const_inner_fn!{ $($input)* }
    }
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules! __delegate_const_ord{
    ( $($input:tt)* )=>{
        $crate::__priv_delegate_const_inner_fn!{ $($input)* }

        $crate::__priv_delegate_const_inner_cmpwrapper!{
            (const_cmp, $crate::__::Ordering)

            skip_coerce;

            $($input)*
        }
    };
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_copy_if_nonref {
    ((ref $ident:ident) $expr:expr) => {
        &$expr
    };
    ((copy $ident:ident) $expr:expr) => {
        $expr
    };
}
#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_deref_if_nonref {
    ((ref $ident:ident) $expr:expr) => {
        $expr
    };
    ((copy $ident:ident) $expr:expr) => {
        *$expr
    };
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_ref_if_nonref {
    ((ref $ident:ident) $ty:ty) => {
        $ty
    };
    ((copy $ident:ident) $ty:ty) => {
        &$ty
    };
}

#[cfg(feature = "polymorphism")]
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_get_pati_ident {
    (ref $ident:ident) => {
        $ident
    };
    (copy $ident:ident) => {
        $ident
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_std_kind_impl {
    (
        impl[$($impl:tt)*] $self:ty
        $(where[ $($where_:tt)* ])?
    )=>{
        impl<$($impl)*> crate::__::ConstCmpMarker for $self
        where
            $($($where_)*)?
        {
            type Kind = crate::__::IsStdKind;
            type This = Self;
        }

        impl<$($impl)* __T> crate::__::IsAConstCmpMarker<crate::__::IsStdKind, $self, __T>
        where
            $($($where_)*)?
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$self) -> crate::__::CmpWrapper<$self> {
                crate::__::CmpWrapper(*reference)
            }
        }
    };
    (skip_coerce $($anything:tt)*)=>{};
}

/// Coerces `reference` to a type that has a `cosnt_eq` method.
#[macro_export]
macro_rules! coerce_to_cmp {
    ($reference:expr) => {{
        match $reference {
            ref reference => {
                let marker = $crate::__::IsAConstCmpMarker::NEW;
                if false {
                    marker.infer_type(reference);
                }
                marker.coerce(marker.unreference(reference))
            }
        }
    }};
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left, right) => {
                let l_marker = $crate::__::IsAConstCmpMarker::NEW;
                let r_marker = $crate::__::IsAConstCmpMarker::NEW;
                if false {
                    l_marker.infer_type(left);
                    r_marker.infer_type(right);
                }
                (
                    l_marker.coerce(l_marker.unreference(left)),
                    r_marker.unreference(right),
                )
            }
        }
    }};
}
