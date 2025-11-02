struct Braced {
    bar: String,
    baz: String,
    qux: String,
}

impl Drop for Braced {
    fn drop(&mut self) {}
}


const fn to_bar(foo: Braced) -> (String, String, String) {
    konst::destructure_rec!{Braced{bar, baz, qux} = foo}

    (bar, baz, qux)
}

const fn to_bar_nested(foo: [Braced; 2]) -> (String, String, String) {
    konst::destructure_rec!{[Braced{bar, baz, qux}, forg] = foo}

    core::mem::forget(forg);

    (bar, baz, qux)
}


fn main(){}