type Tuple = (String, String, String);

const fn to_bar(foo: Tuple) -> impl Sized {
    konst::destructure_rec!{(bar, baz, qux) = foo}

    std::mem::forget(foo);

    (bar, baz, qux)
}


fn main(){}