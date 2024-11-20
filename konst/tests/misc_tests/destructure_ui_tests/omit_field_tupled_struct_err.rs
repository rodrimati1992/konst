struct Tupled(String, String, String);

const fn to_bar(foo: Tupled) -> String {
    konst::destructure!{Tupled(bar) = foo}
    bar
}

const fn to_bar_2(foo: Tupled) -> String {
    konst::destructure!{Tupled(bar, ..) = foo}
    bar
}


fn main(){}