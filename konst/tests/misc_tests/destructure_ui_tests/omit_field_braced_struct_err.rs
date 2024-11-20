struct Braced {
    bar: String,
    baz: String,
    qux: String,
}


const fn to_bar(foo: Braced) -> String {
    konst::destructure!{Braced{bar} = foo}
    bar
}

const fn to_bar_2(foo: Braced) -> String {
    konst::destructure!{Braced{bar, ..}: Braced = foo}
    bar
}


fn main(){}