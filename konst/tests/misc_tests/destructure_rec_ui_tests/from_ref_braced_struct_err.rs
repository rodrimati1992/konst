struct Braced {
    bar: u8,
    baz: u8,
    qux: u8,
}


const fn to_bar(foo: &Braced) -> impl Sized {
    konst::destructure_rec!{Braced{bar, baz, qux} = foo}

    (foo, bar, baz)
}

const fn to_bar2(foo: &Braced) -> impl Sized {
    konst::destructure_rec!{Braced {bar, baz, qux}: &Braced = foo}

    (foo, bar, baz)
}

const fn to_bar_type_as_pat(foo: &Braced) -> impl Sized {
    konst::destructure_rec!{Braced<> {bar, baz, qux}: &Braced = foo}

    (foo, bar, baz)
}


type Nested<'a> = [&'a Braced; 2];

const fn to_bar_nested(foo: Nested<'_>) -> impl Sized {
    konst::destructure_rec!{[Braced {bar, baz, qux}, _] = foo}

    (foo, bar, baz)
}



fn main(){}