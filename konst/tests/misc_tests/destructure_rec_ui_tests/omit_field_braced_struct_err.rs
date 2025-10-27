struct Braced {
    bar: String,
    baz: String,
    qux: String,
}


const fn to_bar(foo: Braced) -> String {
    konst::destructure_rec!{Braced{bar} = foo}
    bar
}

const fn to_bar_2(foo: Braced) -> String {
    konst::destructure_rec!{Braced{bar, ..}: Braced = foo}
    bar
}

const fn to_bar_trailing_fields(foo: Braced) -> String {
    konst::destructure_rec!{Braced{bar, .., baz}: Braced = foo}
    bar
}


fn main(){}