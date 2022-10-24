use konst::string;

// finish early
const _: &str = string::from_iter!(["foo"],map(|_| break));
const _: &str =
    string::from_iter!(["foo"; 5],
        enumerate(),
        map(|(i, s)| if i == 4 { break } else { s })
    );

// non-str item
const _: &str = string::from_iter!(["foo"; 5],enumerate(),map(|(i, _)| i));

fn main(){}