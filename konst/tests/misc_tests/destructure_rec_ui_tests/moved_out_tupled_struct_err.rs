struct Tuple(String, String, String);

const fn to_bar(foo: Tuple) -> (String, String, String) {
    konst::destructure_rec!{Tuple(bar, baz, qux) = foo}

    std::mem::forget(foo);

    (bar, baz, qux)
}

fn main(){}