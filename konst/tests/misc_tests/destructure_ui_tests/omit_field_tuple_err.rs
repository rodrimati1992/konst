type Tuple = (String, String, String);

const fn to_bar(foo: Tuple) -> String {
    konst::destructure!{(bar) = foo}
    bar
}

const fn to_bar_2(foo: Tuple) -> String {
    konst::destructure!{(bar, ..) = foo}
    bar
}

fn main(){}