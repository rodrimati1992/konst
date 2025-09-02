use core::marker::PhantomData;

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map_by_val {
    ($array:expr, $($closure:tt)* ) => (
        $crate::__parse_closure_1!{
            ($crate::__array_map2__with_parsed_closure)
            ($array,)
            (array_map),
            $($closure)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map2__with_parsed_closure {
    (
        $array:expr,
        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => (match $crate::array::IntoIter::of_drop($array) {
        mut consumer => {

            let mut builder = $crate::array::ArrayBuilder::of_drop();

            builder.infer_length_from_consumer(&consumer);

            $crate::while_let_Some!{elem = consumer.next() =>
                let $($pattern)* = elem;
                let mapped $(: $ret)? = $mapper;
                builder.push(mapped);
            }
            $crate::__::mem::forget(consumer);

            builder.build()
        }
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn2 {
    ($($args:tt)*) => ({
        $crate::__split_array_type_and_closure!{
            (
                ($crate::__array_from_fn2__splitted_type_and_closure)
                (__array_map2__with_parsed_closure)
            )
            ()
            ($($args)*)
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn2__splitted_type_and_closure {
    ($array_map_macro:ident $type:tt $($closure_unparsed:tt)*) => {
        $crate::__parse_closure_1!{
            ($crate::__array_from_fn_with_parsed_closure)
            ($array_map_macro $type)
            (from_fn),

            $($closure_unparsed)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn_with_parsed_closure {
    (
        $array_map_macro:ident ($($($type:tt)+)?)

        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => ({
        let mut i = 0usize;

        let arr $(: $crate::__unparenthesize_ty!($($type)*))? =
            $crate::$array_map_macro!{
                $crate::__::unit_array(),
                (()) $(-> $ret)? {
                    let $($pattern)* = i;
                    i+=1;
                    $mapper
                }
            };

        arr
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __split_array_type_and_closure {
    ((($($callback:tt)*) ($($args:tt)*)) $before:tt (=> $($rem:tt)*)) => {
        $($callback)* ! {$($args)* $before $($rem)*}
    };
    ((($($callback:tt)*) ($($args:tt)*)) ($($before:tt)*) ($(| $($rem:tt)*)?)) => {
        $($callback)* ! {$($args)* () $($before)* $(| $($rem)*)?}
    };
    ($callback:tt ($($before:tt)*) ($token:tt $($rem:tt)*)) => {
        $crate::__split_array_type_and_closure! {$callback ($($before)* $token) ($($rem)*)}
    };
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map_by_val_nd {
    ($array:expr, $($closure:tt)* ) => (
        $crate::__parse_closure_1!{
            ($crate::__array_map2__with_parsed_closure_nd)
            ($array,)
            (array_map),
            $($closure)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map2__with_parsed_closure_nd {
    (
        $array:expr,
        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => (match $crate::array::IntoIter::of_assumed_nondrop($array) {
        mut consumer => {
            let (in_pd, out_pd) = const {
                let in_assert_nd = $crate::__::none();
                let out_assert_nd = $crate::__::none();

                (
                    $crate::__::infer_opt_type(&in_assert_nd),
                    $crate::__::infer_opt_type(&out_assert_nd),
                )
            };

            let mut builder = $crate::array::ArrayBuilder::of_assumed_nondrop();

            builder.infer_length_from_consumer(&consumer);

            $crate::while_let_Some!{elem = consumer.next() =>
                let $($pattern)* = elem;
                if false {
                    $crate::__::infer_pd_type(&in_pd, &elem);
                }

                let mapped $(: $ret)? = $mapper;

                if false {
                    $crate::__::infer_pd_type(&out_pd, &mapped);
                }

                builder.push(mapped);
            }
            $crate::__::mem::forget(consumer);

            builder.build()
        }
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn_nd {
    ($($args:tt)*) => ({
        $crate::__split_array_type_and_closure!{
            (
                ($crate::__array_from_fn2__splitted_type_and_closure)
                (__array_map2__with_parsed_closure_nd)
            )
            ()
            ($($args)*)
        }
    });
}

////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub const fn unit_array<const N: usize>() -> [(); N] {
    [(); N]
}

#[inline(always)]
pub const fn none<T>() -> Option<T> {
    None
}

#[inline(always)]
pub const fn infer_opt_type<T>(_: &Option<T>) -> PhantomData<T> {
    PhantomData
}

#[inline(always)]
pub const fn infer_pd_type<T>(_: &PhantomData<T>, _: &T) {}
