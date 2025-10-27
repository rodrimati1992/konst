type Tuple = (u8, u8, u8);

const fn to_bar(foo: &Tuple) -> impl Sized {
    konst::destructure_rec!{(foo, bar, baz) = foo}
    (foo, bar, baz)
}

const fn to_bar_2(foo: &Tuple) -> impl Sized {
    konst::destructure_rec!{(foo, bar, baz): &Tuple = foo}
    (foo, bar, baz)
}


fn main(){}