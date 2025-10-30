struct Tuple(u32, u32, u32);

const _: () = { konst::destructure_rec!{ foo [] = 10..20 }; };

const _: () = { konst::destructure_rec!{ std::ops::Range{start, .., end} = 10..20 }; };

const _: () = { konst::destructure_rec!{ std::ops::Range{start,, end} = 10..20 }; };

const _: () = { konst::destructure_rec!{ Tuple(a, .., b) = Tuple(1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ Tuple(a ,, b) = Tuple(1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ _ {start, end} = 10..20 }; };


fn main(){}