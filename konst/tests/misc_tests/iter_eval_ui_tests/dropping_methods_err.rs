// testing that methods which drop items cause iterators over drop types to error in const


const A: [String; 1] = [String::new()];

const _: () = _ = konst::iter::eval!(A, filter(|_| false),next());
const _: () = _ = konst::iter::eval!(A, map_while(|_| None::<u8>),next());
const _: () = _ = konst::iter::eval!(A, skip_while(|_| false),next());
const _: () = _ = konst::iter::eval!(A, skip(1),next());
const _: () = _ = konst::iter::eval!(A, step_by(1),next());
const _: () = _ = konst::iter::eval!(A, take_while(|_| false),next());
const _: () = _ = konst::iter::eval!(A, take(1),next());
const _: () = _ = konst::iter::eval!(0.., zip(A),next());


fn main() {}