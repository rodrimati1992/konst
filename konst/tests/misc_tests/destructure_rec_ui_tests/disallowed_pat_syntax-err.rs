// top-level patterns

const _: () = { konst::destructure_rec!{ .. = 10 }; };

const _: () = { konst::destructure_rec!{ foo bar = 10 }; };

const _: () = { konst::destructure_rec!{ ref foo = 10 }; };

const _: () = { konst::destructure_rec!{ ref mut foo = [] }; };

const _: () = { konst::destructure_rec!{ mut foo = [] }; };

const _: () = { konst::destructure_rec!{ foo @ [] @ _ = [] }; };


const A: u32 = 10;
const B: u32 = 20;

const _: () = { konst::destructure_rec!{ 10..20 = 10 }; };

const _: () = { konst::destructure_rec!{ A..B = 10 }; };

const _: () = { konst::destructure_rec!{ &foo = &10 }; };

const _: () = { konst::destructure_rec!{ &mut foo = &mut 10 }; };

macro_rules! foo {
    () => ()
}

const _: () = { konst::destructure_rec!{ foo!() = &mut 10 }; };

fn main(){}