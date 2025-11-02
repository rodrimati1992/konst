type Tuple = (String, String, String);

const fn to_bar(foo: Tuple) -> String {
    konst::destructure_rec!{(bar) = foo}
    bar
}

const fn to_bar_2(foo: Tuple) -> String {
    konst::destructure_rec!{(bar,) = foo}
    bar
}

const fn to_bar_3(foo: Tuple) -> String {
    konst::destructure_rec!{(bar, ..) = foo}
    bar
}

const fn to_bar_trailing_field(foo: Tuple) -> String {
    konst::destructure_rec!{(bar, .., baz) = foo}
    bar
}

fn main(){}