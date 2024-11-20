struct Braced {
    bar: String,
    baz: String,
    qux: String,
}

impl Drop for Braced {
    fn drop(&mut self) {}
}


const fn to_bar(foo: Braced) -> (String, String, String) {
    konst::destructure!{Braced{bar, baz, qux} = foo}

    (bar, baz, qux)
}


fn main(){}