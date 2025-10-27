struct Tuple(String, String, String);

impl Drop for Tuple {
    fn drop(&mut self) {}
}

const fn to_bar(foo: Tuple) -> (String, String, String) {
    konst::destructure_rec!{Tuple(foo, bar, baz) = foo}

    (foo, bar, baz)
}

fn main(){}