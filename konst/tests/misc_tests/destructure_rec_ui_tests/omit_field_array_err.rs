type Array = [String; 4];

const fn to_bar(foo: Array) -> String {
    konst::destructure_rec!{[bar] = foo}
    bar
}

fn main(){}