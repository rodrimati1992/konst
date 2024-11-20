struct Braced {
    bar: String,
    baz: String,
    qux: String,
}

const fn to_bar(foo: Braced) -> (String, String, String) {
    konst::destructure!{Braced{bar, baz, qux} = foo}

    std::mem::forget(foo);

    (bar, baz, qux)
}


fn main(){}