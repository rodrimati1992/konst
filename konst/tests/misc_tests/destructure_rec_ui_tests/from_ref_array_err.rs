type Array = [u8; 4];

const fn to_bar(foo: &Array) {
    konst::destructure_rec!{[_bar, _baz, _qux @ ..] = foo}
}

type Nested<'a> = [&'a Array; 2];

const fn to_bar_nested(foo: Nested<'_>) {
    konst::destructure_rec!{[[_bar, _baz, _qux @ ..], _] = foo}
}

fn main(){}