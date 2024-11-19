type Array = [String; 4];

const fn to_bar(foo: Array) -> String {
    konst::destructure!{[bar] = foo}
    bar
}

const fn to_bar_2(foo: Array) -> String {
    konst::destructure!{[bar, ..] = foo}
    bar
}

const fn to_bar_3(foo: Array) -> String {
    konst::destructure!{[bar, ..] = foo}
    bar
}

fn main(){}