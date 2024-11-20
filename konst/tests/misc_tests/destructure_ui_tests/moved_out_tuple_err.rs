type Tuple = (String, String, String);

const fn to_bar(foo: Tuple) -> impl Sized {
    konst::destructure!{(bar, baz, qux) = foo}

    std::mem::forget(foo);

    (bar, baz, qux)
}


fn main(){}