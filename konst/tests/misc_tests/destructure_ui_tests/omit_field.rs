////////////////////////

struct Braced {
    bar: String,
    baz: String,
    qux: String,
}


const fn to_bar(foo: Braced) -> String {
    komst::destructure!{Braced{bar} = foo}
    bar
}

const fn to_bar_2(foo: Braced) -> String {
    komst::destructure!{Braced{bar, ..} = foo}
    bar
}

////////////////////////

const _: () = {
    struct Tupled(String, String, String)

    const fn to_bar(foo: Tupled) -> String {
        komst::destructure!{Tupled(bar) = foo}
        bar
    }

    const fn to_bar_2(foo: Tupled) -> String {
        komst::destructure!{Tupled(bar, ..) = foo}
        bar
    }
};

////////////////////////

const _: () = {
    type Tuple = (String, String, String);

    const fn to_bar(foo: Tuple) -> String {
        komst::destructure!{(bar) = foo}
        bar
    }

    const fn to_bar_2(foo: Tuple) -> String {
        komst::destructure!{(bar, ..) = foo}
        bar
    }
};

////////////////////////

const _: () = {
    type Array = [String; 4];

    const fn to_bar(foo: Array) -> String {
        komst::destructure!{[bar] = foo}
        bar
    }

    const fn to_bar_2(foo: Array) -> String {
        komst::destructure!{[bar, ..] = foo}
        bar
    }

    const fn to_bar_3(foo: Array) -> String {
        komst::destructure!{[bar, ..] = foo}
        bar
    }
};