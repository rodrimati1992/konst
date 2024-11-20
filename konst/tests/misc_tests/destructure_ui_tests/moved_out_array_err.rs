type Array = [String; 4];

const fn to_bar(foo: Array) -> impl Sized {
    konst::destructure!{[bar, baz, qux @ ..] = foo}

    std::mem::forget(foo);

    (bar, baz, qux)
}

fn main(){}