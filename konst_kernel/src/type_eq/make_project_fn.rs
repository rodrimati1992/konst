#[macro_export]
macro_rules! type_eq_projection_fn {
    (
        $(#[$attr:meta])*
        $vis:vis
        $(const $(@$is_const:ident@)?)?
        fn $function:ident
        (
            $($type_param:ident)?
            $(, $param:ident $(@$L_R_from_ctx:ident@)?: TypeEq<$L:ident, $R:ident>)?
        )
        ->
        $(:: $(@$c2:ident@)?)? $($type_name:ident)::* <
        $($gen_params:tt)*
    ) => {
        $crate::__mpf_assert_type_param_is_T!{ $($type_param)? }

        $crate::__::__make_projection_parse_generics!{
            (
                ( $(($(@$L_R_from_ctx@)?))? (__L, __R,) )
                ( $(($param, $L, $R))? (_, __L, __R) )
                (
                    $(#[$attr])*
                    #[inline(always)]
                    $vis $(const $($is_const)?)? fn $function
                )
                ( $(:: $($c2)?)? $($type_name)::* )
            )

            () // generic parameter
            () // generic arguments

            ($($gen_params)*)
        }
    };
}

macro_rules! __meta__make_projection_parse_generics {
    (
        $_:tt

        repeat_punct( $(($punct:tt ($($prep:tt)*)))* )
    ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_projection_parse_generics {
            (
                $fixed:tt
                $gen_params:tt
                $gen_args:tt
                ( >  $_($rem:tt)*)
            ) => {
                $crate::__make_projection_fn!{
                    $fixed
                    $gen_params
                    $gen_args
                    ($_($rem)*)
                }
            };
            $((
                $fixed:tt

                ($_($prev_gen_params:tt)*)
                ($_($prev_gen_args:tt)*)

                (
                    $lt_param:lifetime $_(: $_( $lt_bound:lifetime $_(+)? )* )?
                    $punct $_($rem:tt)*
                )
            ) => {
                $crate::__::__make_projection_parse_generics!{
                    $fixed

                    (
                        $_($prev_gen_params)*
                        $lt_param $_(: $_( $lt_bound + )* )? ,
                    )
                    ($_($prev_gen_args)* $lt_param,)

                    ( $($prep)*  $_($rem)* )
                }
            };)*
            $((
                $fixed:tt

                ($_($prev_gen_params:tt)*)
                ($_($prev_gen_args:tt)*)

                (
                    const $const_param:ident: $const_ty:ty
                    $punct $_($rem:tt)*
                )
            ) => {
                $crate::__::__make_projection_parse_generics!{
                    $fixed

                    ($_($prev_gen_params)* const $const_param: $const_ty,)
                    ($_($prev_gen_args)* $const_param,)

                    ( $($prep)*  $_($rem)* )
                }
            };)*
            $((
                (
                    (($_($declared_replacement_ty_params:tt)*) $_($ignored0:tt)*)
                    $rep_ty_param:tt
                    $prefix:tt
                    $type_name:tt
                )

                ($_($prev_gen_params:tt)*)
                ($_($prev_gen_args:tt)*)

                (
                    T $_(:
                        $_( $lt_bound:lifetime $_(+)? )*
                        $_( ( $_($ty_bounds:tt)* ) )?
                    )?
                    $punct $_($rem:tt)*
                )
            )=>{
                $_($_($crate::__mpf_assert_bound!{ $_($ty_bounds)* })?)?

                $crate::__::__make_projection_parse_generics!{
                    (
                        (($_($declared_replacement_ty_params)*) $_($ignored0)*)
                        $rep_ty_param
                        $prefix
                        $type_name
                    )

                    (
                        $_($prev_gen_params)*
                        $_($declared_replacement_ty_params)*
                    )

                    (
                        @replaced( $_( $_( $lt_bound + )* $_($_( $ty_bounds )*)? )? );
                        $_($prev_gen_args)* ;
                    )

                    ( $($prep)*  $_($rem)* )
                }
            };)*
            $((
                $fixed:tt

                ($_($prev_gen_params:tt)*)
                ($_($prev_gen_args:tt)*)

                (
                    $ty_param:ident $_(:
                        $_( $lt_bound:lifetime $_(+)? )*
                        $_( ( $_($ty_bounds:tt)* ) )?
                    )?
                    $punct $_($rem:tt)*
                )
            )=>{
                $_($_($crate::__mpf_assert_bound!{ $_($ty_bounds)* })?)?

                $crate::__::__make_projection_parse_generics!{
                    $fixed

                    (
                        $_($prev_gen_params)*
                        $ty_param $_(: $_( $lt_bound + )* $_($_( $ty_bounds )*)? )?,
                    )
                    ($_($prev_gen_args)* $ty_param,)

                    ( $($prep)*  $_($rem)* )
                }
            };)*
            (
                $fixed:tt  $prev_gen_params:tt $prev_gen_args:tt

                (
                    $ty_param:ident:
                        $_( $lt_bound:lifetime $_(+)? )*
                        $_(::)? $ident:ident
                    $_($rem:tt)*
                )
            )=>{
                $crate::__::compile_error!{$crate::__::concat!(
                    "trait bounds in parameter list must be wrapped in parentheses, context: `",
                    stringify!($ty_param: $_( $lt_bound + )*  $ident),
                    "`",
                )}
            };
        }

        #[doc(hidden)]
        pub use __make_projection_parse_generics;
    };
}

__meta__make_projection_parse_generics! {
    $
    repeat_punct(
        (, ())
        (> (>))
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_projection_fn {
    (
        (
            $ignored0:tt
            (($param:tt, $L:ident, $R:ident) $($ignored1:tt)*)
            ($($prefix:tt)*)
            ($($type_name:tt)*)
        )

        ($($gen_params:tt)*)

        (
            $(@replaced($($rep_ty_bound:tt)*);)*
            $($lt_arg:lifetime,)*
            $($gen_arg_before:ident,)*
            $(;$($gen_arg_after:ident,)*)?
        )

        (
            $(where $($where_preds:tt)* )?
        )
    ) => {
        $crate::__assert_replaced_type_param_and_where_clause!{
            ($(@replaced($($rep_ty_bound)*);)*)
            ($( $($where_preds)* )?)

            $($prefix)*
            <$($gen_params)*> (
                param: $crate::__::TypeEq<$L, $R>
            ) -> $crate::__::TypeEq<
                $($type_name)* <$($lt_arg,)* $($gen_arg_before,)* $L, $($($gen_arg_after,)*)?>,
                $($type_name)* <$($lt_arg,)* $($gen_arg_before,)* $R, $($($gen_arg_after,)*)?>,
            >
            where
                $L: $($($rep_ty_bound)*)?,
                $R: $($($rep_ty_bound)*)?,
                $($($where_preds)*)?
            {
                struct __Projector<T: ?Sized>($crate::__::PhantomData<T>);

                impl<$($gen_params)*> $crate::__::TypeFn<$L>
                for __Projector<
                    $($type_name)* <
                        $($lt_arg,)*
                        $($gen_arg_before,)*
                        $R,
                        $($($gen_arg_after,)*)?
                    >,
                >
                where
                    $L: $($($rep_ty_bound)*)?,
                    $R: $($($rep_ty_bound)*)?,
                    $($($where_preds)*)?
                {
                    type Output = $($type_name)* <
                        $($lt_arg,)*
                        $($gen_arg_before,)*
                        $L,
                        $($($gen_arg_after,)*)?
                    >;
                }

                param.project::<
                    __Projector<
                        $($type_name)* <
                            $($lt_arg,)*
                            $($gen_arg_before,)*
                            $R,
                            $($($gen_arg_after,)*)?
                        >
                    >
                >()
            }

        }
    };
    ($($tt:tt)*) => {
        $crate::__::compile_error!{
            $crate::__::stringify!{$($tt)*}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_replaced_type_param_and_where_clause {
    (() $where:tt $($token:tt)*) => {
        $crate::__::compile_error!{
            "expected a `T` type parameter in the return type"
        }
    };
    (
        (@replaced($($rep_ty_bound:tt)*);)
        (
            $($wc_lt:lifetime: $( $wc_lt_bound:lifetime $(+)? )*),*
            $(,)?
            $($wc_ty:ty : $($wc_ty_lt:lifetime $(+)?)* $($wc_ty_bound:path)?),*
            $(,)?
        )
        $($token:tt)*
    ) => {
        $($token)*
    };
    (
        $replaced:tt
        $where:tt
        $($token:tt)*
    ) => {
        $crate::__::compile_error!{"invalid where clause syntax"}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __mpf_assert_bound {
    ($bound:ty) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __mpf_assert_type_param_is_T {
    (T) => {};
    ($($tt:tt)*) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "expected function parameter to be `T`, found: `",
            $crate::__::stringify!($($tt)*),
            "`",
        )}
    };
}
