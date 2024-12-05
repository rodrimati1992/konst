
#[doc(hidden)]
#[macro_export]
macro_rules! __array_map_by_val {
    ($array:expr, $($closure:tt)* ) => (
        $crate::__::__parse_closure_1!{
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
    ) => (match $crate::array::ArrayConsumer::new($array) {
        mut consumer => {

            let mut builder = $crate::array::ArrayBuilder::new();

            builder.infer_length_from_consumer(&consumer);

            while let Some(elem) = consumer.next() {
                let elem = $crate::__::ManuallyDrop::into_inner(elem);
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
        $crate::__::__split_array_type_and_closure!{
            (($crate::__array_from_fn2__splitted_type_and_closure) ())
            ()
            ($($args)*)
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn2__splitted_type_and_closure {
    ($type:tt $($closure_unparsed:tt)*) => {
        $crate::__::__parse_closure_1!{
            ($crate::__array_from_fn_with_parsed_closure)
            ($type)
            (from_fn_),

            $($closure_unparsed)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_from_fn_with_parsed_closure {
    (
        ($($($type:tt)+)?)

        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => ({
        let mut i = 0usize;

        let arr $(: $crate::__::__unparenthesize_ty!($($type)*))? =
            $crate::__array_map2__with_parsed_closure!{
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

