// top-level patterns

const _: () = { konst::destructure_rec!{ .. = 10 }; };

const _: () = { konst::destructure_rec!{ foo bar = 10 }; };

const _: () = { konst::destructure_rec!{ ref foo = 10 }; };

const _: () = { konst::destructure_rec!{ ref mut foo = [] }; };

const _: () = { konst::destructure_rec!{ mut foo = [] }; };

const _: () = { konst::destructure_rec!{ foo @ [] @ _ = [] }; };


fn main(){}