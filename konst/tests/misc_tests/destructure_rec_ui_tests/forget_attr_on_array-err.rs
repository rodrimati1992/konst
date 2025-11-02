type Array = [String; 4];

const fn to_bar(foo: Array) -> String {
    konst::destructure_rec!{
        #[forget_ignored_fields]
        [bar, ..] = foo
    }
    bar
}

fn main(){}