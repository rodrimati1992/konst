#[macro_export]
macro_rules! impl_cmp {
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_cmp_recursive!{
            impls[
            ]
            tokens[$($rem)*]
        }
    );
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_cmp_recursive!{
            impls[
            ]
            tokens[$($rem)*]
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cmp_recursive{
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_cmp_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    impl[$($impl_)*] $type
                    where[ $($($where)*)? ];
                )
            ]
            tokens[
                $($rem)*
            ]
        }
    );
    // The same as the above macro branch, but it doesn't require the `[]` in `impl[]`
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_cmp_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    impl[] $type
                    where[ $($($where)*)? ];
                )
            ]
            tokens[
                $($rem)*
            ]
        }
    );
    (
        impls [ $( $an_impl:tt )+ ]
        tokens $stuff:tt
    ) => (
        $(
            $crate::__impl_cmp_impl!{
                $an_impl
                $stuff
            }
        )+
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cmp_impl {
    (
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        [ $($everything:tt)* ]
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__::ConstCmpMarker for $type
        where
            $($where)*
        {
            type Kind = $crate::__::IsNotStdKind;
            type This = Self;
        }

        $(#[$impl_attr])*
        impl<$($impl_)*> $type
        where
            $($where)*
        {
            $($everything)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cmp_self_ty {
    ($self:ty, /*is_std_type*/ true )=>{
        $crate::__::PWrapper<$self>
    };
    ($self:ty, /*is_std_type*/ false )=>{
        $self
    };

}
