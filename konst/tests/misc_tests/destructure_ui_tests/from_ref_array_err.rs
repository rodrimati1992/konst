type Array = [u8; 4];

const fn to_bar(foo: &Array) {
    konst::destructure!{[_bar, _baz, _qux @ ..] = foo}
}

fn main(){}